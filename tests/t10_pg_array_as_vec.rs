mod common;

use ::serde_json::Value as JsValue;

#[tokio::test]
// #[ignore]
async fn t_pg_arr_of_bool_as_vec_bool() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Vec<bool>>(
        "SELECT array_agg(R.a) FROM (SELECT true a UNION ALL SELECT false) R",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![vec![true, false]])
}

#[tokio::test]
// #[ignore]
async fn t_pg_arr_of_int4_as_vec_i32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Vec<i32>>(
        "SELECT array_agg(R.a) FROM (SELECT 42 a UNION ALL SELECT 0) R",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![vec![42, 0]])
}

#[tokio::test]
// #[ignore]
async fn t_pg_arr_of_text_as_vec_nullable_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Vec<Option<String>>>(
        "SELECT array_agg(R.a) FROM (SELECT 'a string' a UNION ALL SELECT NULL) R",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![vec![Some("a string".to_owned()), None]])
}

#[tokio::test]
// #[ignore]
async fn t_pg_arr_of_varchar_as_vec_nullable_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Vec<Option<String>>>(
        "SELECT array_agg(R.a) FROM (SELECT 'a string' :: VARCHAR a UNION ALL SELECT NULL) R",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![vec![Some("a string".to_owned()), None]])
}

#[tokio::test]
// #[ignore]
async fn t_pg_arr_of_jsonb_as_vec_of_jsvalue() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Vec<JsValue>>(
        "SELECT array_agg(R.a) FROM (SELECT '1' :: JSONB a UNION ALL SELECT '2' :: JSONB) R",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(
        out,
        vec![vec![::serde_json::json!(1), ::serde_json::json!(2),]]
    )
}
