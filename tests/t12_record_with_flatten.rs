use std::collections::HashMap;

mod common;

#[tokio::test]
// #[ignore]
async fn t_text_columns_into_hashmap() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        #[serde(flatten)]
        fields: HashMap<String, i32>,
    }

    let out = common::pg_query_and_de::<ARecord>("SELECT 1 one, 2 two, 3 three", &[])
        .await
        .unwrap();
    let fields = vec![
        ("one".to_owned(), 1),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
    ]
    .into_iter()
    .collect();
    assert_eq!(out, vec![ARecord { fields }]);
}

#[tokio::test]
// #[ignore]
async fn t_int4_columns_into_struct_with_flattenned_field() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        one: i32,
        #[serde(flatten)]
        the_rest: HashMap<String, i32>,
    }
    let out = common::pg_query_and_de::<ARecord>("SELECT 1 one, 2 two, 3 three", &[])
        .await
        .unwrap();
    let the_rest = vec![("two".to_owned(), 2), ("three".to_owned(), 3)]
        .into_iter()
        .collect();
    assert_eq!(out, vec![ARecord { one: 1, the_rest }]);
}

#[tokio::test]
// #[ignore]
async fn t_int4_columns_into_struct_with_flattenned_field_with_tuple_idx_prefix() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        one: i32,
        #[serde(flatten)]
        the_rest: HashMap<String, i32>,
    }
    let out = common::pg_query_and_de::<(ARecord, i32)>(
        "SELECT 1 _0_one, 2 _0_two, 3 _0_three, 4 _1",
        &[],
    )
    .await
    .unwrap();
    let the_rest = vec![("two".to_owned(), 2), ("three".to_owned(), 3)]
        .into_iter()
        .collect();
    assert_eq!(out, vec![(ARecord { one: 1, the_rest }, 4)]);
}
