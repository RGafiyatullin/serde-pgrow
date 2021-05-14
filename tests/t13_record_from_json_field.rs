mod common;

use ::serde_json::Value as JsValue;

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
#[ignore]
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

#[tokio::test]
async fn t_a_record_with_vec_of_js_value_fields() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct ARecord {
        integers: Vec<i32>,
        jsons: Vec<JsValue>,
        jsonbs: Vec<JsValue>,
    }

    let out = common::pg_query_and_de::<ARecord>(
        r#"
            SELECT array_agg(R.i) integers, array_agg(R.j) jsons, array_agg(R.b) jsonbs FROM (
                SELECT 1 i, '1' :: JSON j, '1' :: JSONB b 
                UNION ALL
                SELECT 2, '2' :: JSON, '2' :: JSONB 
            ) R
        "#,
        &[],
    )
    .await
    .unwrap();

    assert_eq!(
        out,
        vec![ARecord {
            integers: vec![1, 2],
            jsons: vec![::serde_json::json!(1), ::serde_json::json!(2),],
            jsonbs: vec![::serde_json::json!(1), ::serde_json::json!(2),]
        },]
    );
}
