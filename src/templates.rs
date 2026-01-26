use askama::Template;

use crate::models::{Entry, User};

#[derive(Template)]
#[template(path = "me.txt", escape = "none")]
pub struct Me<'user> {
    pub me: &'user User,
}

#[derive(Template)]
#[template(path = "search.txt", escape = "none")]
pub struct Search<'query, 'entries> {
    pub query: &'query str,
    pub limit: u32,
    pub entries: &'entries [Entry],
}
