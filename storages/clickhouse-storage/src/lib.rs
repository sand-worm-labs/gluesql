pub mod error;
mod store;
mod store_mut;
pub mod utils;

use {
    error::ResultExt,
    gluesql_core::{
        error::Result,
        store::{
            AlterTable, CustomFunction, CustomFunctionMut, Index, IndexMut, Metadata, Transaction,
        },
    },
    clickhouse::Client
};

pub struct ClickHouseStorage {
    pub db: Client,
}

impl ClickHouseStorage {
    pub async fn new(conn_str: &str, db_name: &str) -> Result<Self> {
        let full_url = format!("{}/{}", conn_str.trim_end_matches('/'), db_name);
        let client = Client::default().with_url(&full_url);
        Ok(Self { db: client })
    }

    pub async fn drop_database(&mut self) -> Result<()> {
        self.db = Client::default(); 
        Ok(())
    }
}

impl Metadata for ClickHouseStorage {}
impl AlterTable for ClickHouseStorage {}
impl CustomFunction for ClickHouseStorage {}
impl CustomFunctionMut for ClickHouseStorage {}
impl Index for ClickHouseStorage {}
impl IndexMut for ClickHouseStorage {}
impl Transaction for ClickHouseStorage {}
