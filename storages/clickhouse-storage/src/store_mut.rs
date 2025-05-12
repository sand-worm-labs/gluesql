use {
    crate::{
        error::{ClickHouseStorageError, OptionExt, ResultExt}, utils::{get_primary_key}, ClickHouseStorage
    },
    async_trait::async_trait,
    gluesql_core::{
        ast::ColumnUniqueOption,
        data::{Key, Schema},
        error::{Error, Result},
        store::{DataRow, Store, StoreMut},
    },
    mongodb::{
        bson::{doc, Bson, Document},
        options::{IndexOptions, ReplaceOptions},
    },
};

struct IndexInfo {
    name: String,
    key: String,
    index_type: IndexType,
}

enum IndexType {
    Primary,
    Unique,
}

#[async_trait(?Send)]
impl StoreMut for ClickHouseStorage {
    async fn insert_schema(&mut self, schema: &Schema) -> Result<()> {
     todo!() 
    }

    async fn delete_schema(&mut self, table_name: &str) -> Result<()> {
        todo!()
    }

    async fn append_data(&mut self, table_name: &str, rows: Vec<DataRow>) -> Result<()> {
        todo!()
    }

    async fn insert_data(&mut self, table_name: &str, rows: Vec<(Key, DataRow)>) -> Result<()> {
        todo!()
    }

    async fn delete_data(&mut self, table_name: &str, keys: Vec<Key>) -> Result<()> {
        todo!()
    }
}
