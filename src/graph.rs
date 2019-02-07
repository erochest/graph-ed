use chrono::prelude::*;
use crate::context::Context;
use juniper::{FieldError, FieldResult, RootNode, Value, ID};
use crate::models::{NewUser, User, Tree, Node};

graphql_object! { User: Context |&self| {
    description: "A user in the system"

    field id() -> ID as "The user's unique identifier" {
        ID::from(self.id.to_string())
    }

    field nickname() -> &Option<String> as "The user's nickname" {
        &self.nickname
    }

    field email() -> &String as "The user's email address/identifier" {
        &self.email
    }

    field created() -> DateTime<Utc> as "The timestamp the user was created on" {
        DateTime::from_utc(self.created, Utc)
    }

    field last_login() -> Option<DateTime<Utc>> as "The last time the user logged in" {
        self.last_login.map(|datetime| DateTime::from_utc(datetime, Utc))
    }

    field trees(&executor) -> FieldResult<Vec<Tree>> as "Trees accessible by this user" {
        let context = executor.context();
        let pool = &context.pool;
        self.find_trees(&pool)
            .map_err(|err| FieldError::new(err, Value::scalar("trees".to_string())))
    }
}}

#[derive(GraphQLObject)]
#[graphql(description = "A user in the system.")]
pub struct UserInfo {
    pub id: ID,
    pub nickname: Option<String>,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(db_user: User) -> UserInfo {
        UserInfo {
            id: ID::from(db_user.id.to_string()),
            nickname: db_user.nickname,
            email: db_user.email,
        }
    }
}

graphql_object! { Tree: Context |&self| {
    description: "A tree of nodes, maybe grouped around a topic."

    field id() -> ID as "The tree's unique identifier" {
        ID::from(self.id.to_string())
    }

    field title() -> &Option<String> as "The tree's title" {
        &self.title
    }

    field root(&executor) -> FieldResult<Node> as "The tree's root node" {
        let context = executor.context();
        let pool = &context.pool;
        self.root(pool)
            .map_err(|err| FieldError::new(err, Value::scalar("root".to_string())))
    }

    field created() -> DateTime<Utc> as "The timestamp the tree was created on" {
        DateTime::from_utc(self.created, Utc)
    }

    field updated() -> DateTime<Utc> as "The timestamp the tree was updated" {
        DateTime::from_utc(self.updated, Utc)
    }
}}

graphql_object! { Node: Context |&self| {
    description: "A node in a graph of information."

    field id() -> ID as "The node's unique identifier" {
        ID::from(self.id.to_string())
    }

    field title() -> &Option<String> as "The node's title" {
        &self.title
    }

    field content() -> &Option<String> as "The node's content" {
        &self.content
    }

    field tree(&executor) -> FieldResult<Option<Tree>> as "The node's tree, if it is a root" {
        let context = executor.context();
        let pool = &context.pool;
        self.tree(pool)
            .map_err(|err| FieldError::new(err, Value::scalar("tree".to_string())))
    }

    field parent(&executor) -> FieldResult<Option<Node>> as "The node's parent node" {
        let context = executor.context();
        let pool = &context.pool;
        self.parent(pool)
            .map_err(|err| FieldError::new(err, Value::scalar("parent".to_string())))
    }

    field children(&executor) -> FieldResult<Vec<Node>> as "The node's children" {
        let context = executor.context();
        let pool = &context.pool;
        self.children(pool)
            .map_err(|err| FieldError::new(err, Value::scalar("children".to_string())))
    }

    field created() -> DateTime<Utc> as "The timestamp the tree was created on" {
        DateTime::from_utc(self.created, Utc)
    }

    field updated() -> DateTime<Utc> as "The timestamp the tree was updated" {
        DateTime::from_utc(self.updated, Utc)
    }
}}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, email: String) -> FieldResult<User> {
        let context = executor.context();
        User::find_by_email(&context.pool, &email)
            .map_err(|err| FieldError::new(&err, Value::scalar(email.clone())))
    }

    field userInfo(&executor, email: String) -> FieldResult<UserInfo> {
        let context = executor.context();
        User::find_by_email(&context.pool, &email)
            .map(UserInfo::from)
            .map_err(|err| FieldError::new(&err, Value::scalar(email.clone())))
    }

    field tree(&executor, id: String) -> FieldResult<Tree> {
        let context = executor.context();
        let int_id = id.parse()
            .map_err(|err| FieldError::new(format!("Invalid ID: {}", &err), Value::scalar(id.clone())))?;
        Tree::get(&context.pool, int_id)
            .map_err(|err| FieldError::new(err, Value::scalar(id.clone())))
    }

    field node(&executor, id: String) -> FieldResult<Node> {
        let context = executor.context();
        let int_id = id.parse()
            .map_err(|err| FieldError::new(format!("Invalid ID: {}", &err), Value::scalar(id.clone())))?;
        Node::get(&context.pool, int_id)
            .map_err(|err| FieldError::new(err, Value::scalar(id.clone())))
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        let context = executor.context();
        User::new(&context.pool, new_user)
            .map_err(|err| FieldError::new(err, Value::scalar("createUser".to_string())))
    }

    field createTree(&executor, title: String) -> FieldResult<Tree> {
        let context = executor.context();
        Tree::new(&context.pool, &title)
            .map_err(|err| FieldError::new(err, Value::scalar("createTree".to_string())))
    }

    field shareTree(&executor, tree_id: ID, user_id: ID) -> FieldResult<Tree> {
        unimplemented!()
    }

    field createNode(&executor, parent_id: ID, title: String, content: String) -> FieldResult<Node> {
        let int_id = parent_id.parse()
            .map_err(|err| FieldError::new(format!("Invalid ID: {}", &err), Value::scalar(parent_id.to_string())))?;
        let context = executor.context();
        Node::new(&context.pool, Some(int_id), &title, &content)
            .map_err(|err| FieldError::new(err, Value::scalar("createNode".to_string())))
    }

    field editNode(&executor, id: ID, title: String, content: String) -> FieldResult<Node> {
        unimplemented!()
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;
