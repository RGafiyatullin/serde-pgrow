mod common;

#[tokio::test]
async fn t_tuple_struct() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct TupleStruct(bool, i32);

    let out = common::pg_query_and_de::<TupleStruct>(
        "SELECT true _0, 42 _1 UNION ALL SELECT false, 0",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![TupleStruct(true, 42), TupleStruct(false, 0),])
}

#[tokio::test]
async fn t_tuple() {
    // common::init_logger();

    let out = common::pg_query_and_de::<(bool, i32)>(
        "SELECT true _0, 42 _1 UNION ALL SELECT false, 0",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![(true, 42), (false, 0),])
}

#[tokio::test]
async fn t_a_one_item_tuple() {
    // common::init_logger();

    let out = common::pg_query_and_de::<(bool,)>("SELECT true _0 UNION ALL SELECT false", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![(true,), (false,),])
}

#[tokio::test]
#[ignore]
async fn t_a_one_item_tuple_unannotated() {
    // common::init_logger();

    let out = common::pg_query_and_de::<(bool,)>("SELECT true UNION ALL SELECT false", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![(true,), (false,),])
}
