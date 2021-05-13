mod common;

#[tokio::test]
// #[ignore]
async fn t_single_json_field_into_a_record_field() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        one: i32,
        two: i32,
        three: i32,
    }
    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARow {
        a_record: ARecord,
    }

    let out = common::pg_query_and_de::<ARow>(
        r#"
            SELECT '{"one": 1, "two": 2, "three": 3}' :: JSON a_record
        "#,
        &[],
    )
    .await
    .unwrap();

    assert_eq!(
        out,
        vec![ARow {
            a_record: ARecord {
                one: 1,
                two: 2,
                three: 3
            }
        }]
    );
}

#[tokio::test]
// #[ignore]
async fn t_single_json_field_into_a_record() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        one: i32,
        two: i32,
        three: i32,
    }

    let out = common::pg_query_and_de::<ARecord>(
        r#"
            SELECT '{"one": 1, "two": 2, "three": 3}' :: JSON
        "#,
        &[],
    )
    .await
    .unwrap();

    assert_eq!(
        out,
        vec![ARecord {
            one: 1,
            two: 2,
            three: 3
        }]
    );
}
