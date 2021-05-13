mod common;

#[tokio::test]
async fn t_unannotated_as_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<String>("SELECT 'a string'", &[])
        .await
        .unwrap();
    assert_eq!(out, vec!["a string".to_owned()])
}

#[tokio::test]
async fn t_text_as_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<String>("SELECT 'a string' :: TEXT", &[])
        .await
        .unwrap();
    assert_eq!(out, vec!["a string".to_owned()])
}

#[tokio::test]
async fn t_bpchar_as_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<String>("SELECT 'a string' :: BPCHAR", &[])
        .await
        .unwrap();
    assert_eq!(out, vec!["a string".to_owned()])
}

#[tokio::test]
async fn t_varchar_as_string() {
    // common::init_logger();
    let out = common::pg_query_and_de::<String>("SELECT 'a string' :: VARCHAR", &[])
        .await
        .unwrap();
    assert_eq!(out, vec!["a string".to_owned()])
}

#[tokio::test]
async fn t_unannotated_as_string_opt() {
    // common::init_logger();
    let out =
        common::pg_query_and_de::<Option<String>>("SELECT 'a string' UNION ALL SELECT NULL", &[])
            .await
            .unwrap();
    assert_eq!(out, vec![Some("a string".to_owned()), None])
}

#[tokio::test]
async fn t_text_as_string_opt() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Option<String>>(
        "SELECT 'a string' :: TEXT UNION ALL SELECT NULL",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![Some("a string".to_owned()), None])
}

#[tokio::test]
async fn t_bpchar_as_string_opt() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Option<String>>(
        "SELECT 'a string' :: BPCHAR UNION ALL SELECT NULL",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![Some("a string".to_owned()), None])
}

#[tokio::test]
async fn t_varchar_as_string_opt() {
    // common::init_logger();
    let out = common::pg_query_and_de::<Option<String>>(
        "SELECT 'a string' :: VARCHAR UNION ALL SELECT NULL",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(out, vec![Some("a string".to_owned()), None])
}
