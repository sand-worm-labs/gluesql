use {
    crate::{error::ResultExt},
    bson::{doc, Document},
    gluesql_core::{
        ast::{ColumnDef, ForeignKey},
        error::Result,
    },
    mongodb::options::CreateCollectionOptions,
    serde_json::to_string,
};

pub fn get_primary_key(column_defs: &[ColumnDef]) -> Option<&ColumnDef> {
    column_defs
        .iter()
        .find(|column_def| column_def.unique.map(|x| x.is_primary).unwrap_or(false))
}