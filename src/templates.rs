use askama::Template;

use crate::{
    models::{Entry, User},
    repository::EntryReserved,
};

#[derive(Template)]
#[template(path = "me.txt")]
pub struct Me<'user> {
    pub me: &'user User,
}

#[derive(Template)]
#[template(path = "search.txt")]
pub struct Search<'query, 'entries> {
    pub query: &'query str,
    pub limit: u32,
    pub entries: &'entries [Entry],
}

#[derive(Template)]
#[template(path = "entry.txt")]
pub struct SingleEntry<'entry> {
    pub entry: &'entry Entry,
    pub reserved: Option<EntryReserved>,
}

#[derive(Template)]
#[template(path = "cart.txt")]
pub struct Cart<'entries> {
    pub entries: &'entries [Entry],
}

#[derive(Template)]
#[template(path = "help.txt")]
pub struct Help;
