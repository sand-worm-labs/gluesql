use {
    crate::{
    error::{ClickHouseStorageError, OptionExt, ResultExt}, utils::get_primary_key, ClickHouseStorage, 
    },
    async_trait::async_trait,
    futures::{stream, Stream, StreamExt, TryStreamExt},
    gluesql_core::{
        ast::{ColumnDef, ColumnUniqueOption},
        data::{Key, Schema},
        error::Result,
        parse_sql::parse_data_type,
        prelude::{Error, Value},
        store::{DataRow, RowIter, Store},
        translate::translate_data_type,
    },
    serde_json::from_str,
    std::{collections::HashMap, future},
};

#[async_trait(?Send)]
impl Store for ClickHouseStorage {
    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
      todo!()
    }

    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>> {
        todo!() 
    }

    async fn fetch_data(&self, table_name: &str, target: &Key) -> Result<Option<DataRow>> {
        todo!()
    }

    async fn scan_data(&self, table_name: &str) -> Result<RowIter> {
       todo!()
    }
}
