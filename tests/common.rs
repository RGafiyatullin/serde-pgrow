use ::futures::prelude::*;
use ::serde::Deserialize;
use ::tokio_postgres::types::ToSql;
use ::tokio_postgres::Row as PgRow;

use ::serde_pgrow::prelude::*;

pub use ::eyre::Report as AnyError;

#[allow(dead_code)]
pub fn init_logger() {
    let _ = ::pretty_env_logger::try_init_timed();
}

pub async fn pg_query(
    statement: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Vec<PgRow>, AnyError> {
    let tls = ::tokio_postgres::NoTls;

    let (client, connection) =
        ::tokio_postgres::connect("host=127.0.0.1 user=tests password=tests dbname=tests", tls)
            .await
            .unwrap();

    let rows = async move { client.query(statement, args).await.unwrap() };

    let (rows, _) = future::join(rows, connection).await;

    Ok(rows)
}

pub async fn pg_query_and_de<'de, T: Deserialize<'de>>(
    q: &str,
    args: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>, AnyError> {
    let rows = pg_query(q, args).await.unwrap();
    let records: Vec<T> = rows.cast().unwrap();

    Ok(records)
}
