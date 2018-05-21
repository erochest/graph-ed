use chrono::prelude::*;
use context::Pool;
use diesel::prelude::*;
use diesel::*;
use schema::nodes;
use schema::trees;
use schema::user_trees;
use schema::users;

#[derive(Queryable, Associations, Identifiable)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub email: String,
    pub created: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, GraphQLInputObject, Insertable, Deserialize)]
#[table_name = "users"]
#[graphql(description = "A user in the system.")]
pub struct NewUser {
    pub nickname: Option<String>,
    pub email: String,
}

impl User {
    pub fn new(pool: &Pool, new_user: NewUser) -> Result<User, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        insert_into(users::table)
            .values(&new_user)
            .get_result(&*connection)
            .map_err(|err| format!("Error inserting {:?}: {}", &new_user, &err))
    }

    pub fn get(pool: &Pool, id: i32) -> Result<User, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;
        users::table
            .find(id)
            .first::<User>(&*connection)
            .map_err(|err| format!("User not found: {}", &err))
    }

    pub fn find_by_email(pool: &Pool, email_addr: &str) -> Result<User, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        users::table
            .filter(users::email.eq(email_addr.to_string()))
            .first::<User>(&*connection)
            .map_err(|err| format!("User not found: {}", &err))
    }

    pub fn find_trees(&self, pool: &Pool) -> Result<Vec<Tree>, String> {
        use diesel::pg::expression::dsl::any;

        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        let user_trees_ids = UserTree::belonging_to(self).select(user_trees::tree_id);

        trees::table
            .filter(trees::id.eq(any(user_trees_ids)))
            .load::<Tree>(&*connection)
            .map_err(|err| format!("Unable to load trees: {}", &err))
    }
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Node)]
pub struct Tree {
    pub id: i32,
    pub title: Option<String>,
    pub node_id: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "trees"]
struct NewTree {
    title: Option<String>,
    node_id: i32,
}

impl Tree {
    pub fn new(pool: &Pool, title: &str) -> Result<Tree, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;
        let root = Node::new(&pool, None, &title, "")?;
        let new_tree = NewTree {
            title: Some(String::from(title)),
            node_id: root.id,
        };
        insert_into(trees::table)
            .values(&new_tree)
            .get_result(&*connection)
            .map_err(|err| format!("Error inserting {:?}: {}", &new_tree, &err))
    }

    pub fn get(pool: &Pool, id: i32) -> Result<Tree, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;
        trees::table
            .find(id)
            .first::<Tree>(&*connection)
            .map_err(|err| format!("Tree not found: {}", &err))
    }

    pub fn root(&self, pool: &Pool) -> Result<Node, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;
        nodes::table
            .find(self.node_id)
            .first::<Node>(&*connection)
            .map_err(|err| format!("Tree root not found: {}", &err))
    }
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Tree)]
pub struct UserTree {
    pub id: i32,
    pub user_id: i32,
    pub tree_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "nodes"]
struct NewNode {
    pub title: Option<String>,
    pub content: Option<String>,
    pub node_id: Option<i32>,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Node)]
pub struct Node {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub node_id: Option<i32>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl Node {
    pub fn new(
        pool: &Pool,
        parent_id: Option<i32>,
        title: &str,
        content: &str,
    ) -> Result<Node, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;
        let new_node = NewNode {
            title: Some(String::from(title)),
            content: Some(String::from(content)),
            node_id: parent_id,
        };
        insert_into(nodes::table)
            .values(&new_node)
            .get_result(&*connection)
            .map_err(|err| format!("Error inserting {:?}: {}", &new_node, &err))
    }

    pub fn get(pool: &Pool, id: i32) -> Result<Node, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        nodes::table
            .find(id)
            .first::<Node>(&*connection)
            .map_err(|err| format!("Node node found: {}", &err))
    }

    pub fn tree(&self, pool: &Pool) -> Result<Option<Tree>, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        trees::table
            .filter(trees::node_id.eq(self.id))
            .first::<Tree>(&*connection)
            .optional()
            .map_err(|err| format!("Error getting tree: {}", &err))
    }

    pub fn parent(&self, pool: &Pool) -> Result<Option<Node>, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        match self.node_id {
            Some(node_id) => nodes::table
                .filter(nodes::node_id.eq(node_id))
                .first::<Node>(&*connection)
                .optional()
                .map_err(|err| format!("Error getting node: {}", &err)),
            None => Ok(None),
        }
    }

    pub fn children(&self, pool: &Pool) -> Result<Vec<Node>, String> {
        let connection = pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))?;

        nodes::table
            .filter(nodes::node_id.eq(self.id))
            .load::<Node>(&*connection)
            .map_err(|err| format!("Unable to load children: {}", &err))
    }
}
