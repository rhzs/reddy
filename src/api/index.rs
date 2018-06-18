use actix::*;
use actix_web::{fs::NamedFile, HttpRequest, Result};
use std::path::Path;
use model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<Syn, ConnDsl>
}
