mod common;

#[tokio::test]
async fn t_int2_as_u16() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u16>("SELECT 42 :: INT2", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_unannotated_as_u32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u32>("SELECT 42", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_unannotated_as_u32_opt() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Option<u32>>("SELECT 42 UNION ALL SELECT NULL", &[])
        .await
        .unwrap();
    assert_eq!(out, vec![Some(42), None])
}

#[tokio::test]
async fn t_int2_as_u32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u32>("SELECT 42 :: INT2", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_int4_as_u32() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u32>("SELECT 42 :: INT4", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_int2_as_u64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u64>("SELECT 42 :: INT2", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_int4_as_u64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u64>("SELECT 42 :: INT4", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_int8_as_u64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u64>("SELECT 42 :: INT8", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}

#[tokio::test]
async fn t_bigint_as_u64() {
    // common::init_logger();
    let out = common::pg_query_and_de::<u64>("SELECT 42 :: BIGINT", &[])
        .await
        .unwrap();

    assert_eq!(out, vec![42])
}
