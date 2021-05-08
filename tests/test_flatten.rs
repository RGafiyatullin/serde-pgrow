use std::collections::HashMap;

use ::futures::prelude::*;
use ::serde_pgrow::prelude::*;

#[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum DataValue {
    String(String),
    Int(i64),
    Bool(bool),
}

#[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq)]
struct Flat {
    x: i64,

    #[serde(flatten)]
    props: HashMap<String, DataValue>,
}

#[tokio::test]
#[ignore]
async fn test_flatten() {
    let (client, connection) = ::tokio_postgres::connect(
        "host=127.0.0.1 user=tests password=tests dbname=tests",
        ::tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    let rows = async move {
        client
            .query(
                r#"
                SELECT
                    'test_str' p_1, 1::BIGINT p_2, 1::BIGINT x
                "#,
                &[],
            )
            .await
            .unwrap()
    };

    let (rows, _) = future::join(rows, connection).await;

    let val = rows.first().cast::<Flat>().unwrap();
    let expected = {
        let mut props = HashMap::new();
        props.insert("p_1".to_string(), DataValue::String("test_str".to_string()));
        props.insert("p_2".to_string(), DataValue::Int(1));

        Some(Flat { x: 1, props })
    };

    assert_eq!(val, expected);
}
