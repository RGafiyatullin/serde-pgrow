mod common;

#[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
struct NewType<T>(T);

#[tokio::test]
async fn t_real_as_f32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<NewType<f32>>("SELECT 4.2 :: REAL", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![NewType(4.2)])
}

#[tokio::test]
async fn t_double_precision_as_f64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<NewType<f64>>("SELECT 4.2 :: DOUBLE PRECISION", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![NewType(4.2)])
}

#[tokio::test]
async fn t_unannotated_as_i32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<NewType<i32>>("SELECT 1 UNION ALL SELECT 2", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![NewType(1), NewType(2)])
}
