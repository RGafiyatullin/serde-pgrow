mod common;

#[tokio::test]
async fn t_real_as_f32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<f32>("SELECT 4.2 :: REAL", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![4.2])
}

#[tokio::test]
async fn t_double_precision_as_f64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<f64>("SELECT 4.2 :: DOUBLE PRECISION", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![4.2])
}
