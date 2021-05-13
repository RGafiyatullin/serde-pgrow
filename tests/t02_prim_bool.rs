mod common;

#[tokio::test]
async fn t_unannotated_as_bool() {
    // common::init_logger();
    let out = common::pg_query_and_de::<bool>("SELECT true UNION ALL SELECT false", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![true, false])
}

#[tokio::test]
async fn t_unannotated_as_bool_opt() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Option<bool>>(
        "SELECT true UNION ALL SELECT false UNION ALL SELECT NULL",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![Some(true), Some(false), None])
}
