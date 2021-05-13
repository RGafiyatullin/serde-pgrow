use ::serde_json::Value as JsValue;

mod common {
    use ::futures::prelude::*;
    use ::tokio_postgres::types::ToSql;
    use ::tokio_postgres::Row as PgRow;

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
}

#[tokio::main]
async fn main() {
    let rows = common::pg_query(
        r#"
    SELECT 
        1 :: INT an_int,
        1 :: BIGINT a_big_int,
        'one' :: VARCHAR a_varchar,
        'one' :: CHAR a_char,
        'one' a_text_probably,
        NULL :: INT an_int_nullable
"#,
        &[],
    )
    .await
    .unwrap();

    for row in rows {
        println!("BEGIN ROW");
        let cols = row.columns();
        for col in cols {
            let name = col.name();
            let type_ = col.type_();
            let kind = type_.kind();
            let schema = type_.schema();

            println!(
                " COL: name = {:?}; type = {:?}; kind: {:?}; schema: {:?}",
                name, type_, kind, schema,
            );
            println!("    VALUE: {:?}", row.try_get::<_, JsValue>(col.name()))
        }

        println!("END ROW");
    }
}
