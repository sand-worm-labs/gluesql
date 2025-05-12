## 🚴 ClickHouseStorage - ClickHouse Backend for GlueSQL

This crate provides **ClickHouse** storage support for [GlueSQL](https://github.com/gluesql/gluesql), allowing you to execute SQL queries directly against a ClickHouse database using the GlueSQL engine.

---

### ⚙️ Prerequisites

You need a running **ClickHouse** server.

#### 🔧 Option 1: Run ClickHouse via Docker (Recommended)

```bash
docker run --name clickhouse-glue \
  -d -p 8123:8123 -p 9000:9000 -p 9009:9009 \
  clickhouse/clickhouse-server
```

* `8123`: HTTP interface
* `9000`: Native TCP interface
* `9009`: Monitoring

> ClickHouse is now running on `localhost:9000` (TCP) or `localhost:8123` (HTTP)

#### 🖥 Option 2: Install Locally

Follow official instructions:
[https://clickhouse.com/docs/en/getting-started/install](https://clickhouse.com/docs/en/getting-started/install)

---

### 🔌 Usage

```rust
use gluesql_clickhouse_storage::ClickHouseStorage;
use gluesql_core::executor::Executor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let storage = ClickHouseStorage::new("tcp://localhost:9000", "default").await?;
    let mut glue = Executor::new(storage);

    glue.execute("CREATE TABLE users (id INT, name TEXT)").await?;
    glue.execute("INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob')").await?;
    let result = glue.execute("SELECT * FROM users").await?;

    println!("{:#?}", result);
    Ok(())
}
```

---

### 🧪 Run Tests

Enable the `test-clickhouse` feature to run tests against a real ClickHouse instance:

```bash
cargo test --features test-clickhouse
```

---

### 🧱 Supported Features

| Feature      | Status                                           |
| ------------ | ------------------------------------------------ |
| SELECT       | ✅ Works                                          |
| INSERT       | ✅ Works                                          |
| CREATE TABLE | ✅ Works                                          |
| DROP TABLE   | ✅ Works                                          |
| DELETE       | ⚠️ Limited (depends on MergeTree engines or TTL) |
| UPDATE       | ❌ Not supported in native ClickHouse             |

---

### 💬 Notes

* GlueSQL types are mapped to closest matching ClickHouse types.
* Deleting/updating rows is **not natively supported** in ClickHouse without workarounds.
* Use `MergeTree` tables with TTL or mutations for DELETE/UPDATE semantics.

---

### 📦 Features

In `Cargo.toml`:

```toml
[features]
default = []
test-clickhouse = []
```

