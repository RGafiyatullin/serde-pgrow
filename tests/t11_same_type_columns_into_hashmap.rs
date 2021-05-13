use std::collections::HashMap;

mod common;

#[tokio::test]
async fn t_text_columns_into_hashmap() {
    // common::init_logger();
    let out = common::pg_query_and_de::<HashMap<String, String>>(
        "SELECT '1' one, '2' two, '3' three",
        &[],
    )
    .await
    .unwrap();
    let expected_hashmap = vec![
        ("one".to_owned(), "1".to_owned()),
        ("two".to_owned(), "2".to_owned()),
        ("three".to_owned(), "3".to_owned()),
    ]
    .into_iter()
    .collect();
    assert_eq!(out, vec![expected_hashmap]);
}

#[tokio::test]
async fn t_int4_columns_into_hashmap() {
    // common::init_logger();
    let out = common::pg_query_and_de::<HashMap<String, i32>>("SELECT 1 one, 2 two, 3 three", &[])
        .await
        .unwrap();
    let expected_hashmap = vec![
        ("one".to_owned(), 1),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
    ]
    .into_iter()
    .collect();
    assert_eq!(out, vec![expected_hashmap]);
}

#[tokio::test]
async fn t_int4_columns_into_hashmap_tuple_idx_prefix() {
    // common::init_logger();
    let out = common::pg_query_and_de::<(HashMap<String, i32>, ())>(
        "SELECT 1 _0_one, 2 _0_two, 3 _0_three",
        &[],
    )
    .await
    .unwrap();
    let expected_hashmap = vec![
        ("one".to_owned(), 1),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
    ]
    .into_iter()
    .collect();
    assert_eq!(out, vec![(expected_hashmap, ())]);
}
