use std::collections::HashMap;

use ::futures::prelude::*;
use ::serde_json::Value as JsValue;
use ::serde_pgrow::prelude::*;

#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct StructOne {
    string_one: String,
    string_two: String,
    int_one: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct StructTwo {
    string_one: String,
    string_two: String,
    int_two: i64,
}

#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct WithFlatten {
    _0_string_one: String,
    _0_int_one: i64,

    #[serde(flatten)]
    fields: HashMap<String, JsValue>,
}

#[tokio::test]
#[ignore]
async fn test_a_struct() {
    let tls = ::tokio_postgres::NoTls;

    let (client, connection) =
        ::tokio_postgres::connect("host=127.0.0.1 user=tests password=tests dbname=tests", tls)
            .await
            .unwrap();
    let rows = async move {
        client
            .query(
                r"
            SELECT 
                'left_one' _0_string_one, 'left_two' _0_string_two, 1::BIGINT _0_int_one,
                'right_one' _1_string_one, 'right_two' _1_string_two, 2::BIGINT _1_int_two
        ",
                &[],
            )
            .await
            .unwrap()
    };

    let (rows, _) = future::join(rows, connection).await;

    println!("rows: {:#?}", rows);
    let two_structs = rows.cast::<(StructOne, StructTwo)>().unwrap();
    println!("structs: {:#?}", two_structs);
    let hashmaps = rows.cast::<HashMap<String, JsValue>>().unwrap();
    println!("hashmaps: {:#?}", hashmaps);

    let flattens = rows.cast::<WithFlatten>().unwrap();
    println!("flattens: {:#?}", flattens);

    let jsons = rows.cast::<JsValue>().unwrap();
    println!("jsons: {:#?}", jsons);
}
