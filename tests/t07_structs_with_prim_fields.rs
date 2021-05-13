mod common;

#[tokio::test]
async fn t_one_prim_field_struct() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct OneFieldStruct {
        the_field: bool,
    }

    let out = common::pg_query_and_de::<OneFieldStruct>(
        "SELECT true the_field UNION ALL SELECT false",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(
        out,
        vec![
            OneFieldStruct { the_field: true },
            OneFieldStruct { the_field: false }
        ]
    )
}

#[tokio::test]
async fn t_two_prim_field_struct() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct TwoFieldStruct {
        first_field: bool,
        second_field: i32,
    }

    let out = common::pg_query_and_de::<TwoFieldStruct>(
        "SELECT true first_field, 42 second_field UNION ALL SELECT false, 0",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(
        out,
        vec![
            TwoFieldStruct {
                first_field: true,
                second_field: 42
            },
            TwoFieldStruct {
                first_field: false,
                second_field: 0
            },
        ]
    )
}

#[tokio::test]
#[ignore]
async fn t_two_indirect_prim_fields_struct() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct Left {
        first_field: bool,
    }
    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct Right {
        second_field: i32,
    }

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct OneFieldStruct {
        #[serde(flatten)]
        left: Left,
        #[serde(flatten)]
        right: Right,
    }

    let out = common::pg_query_and_de::<OneFieldStruct>(
        "SELECT true first_field, 42 second_field UNION ALL SELECT false, 0",
        &[],
    )
    .await
    .unwrap();
    assert_eq!(
        out,
        vec![
            OneFieldStruct {
                left: Left { first_field: true },
                right: Right { second_field: 42 },
            },
            OneFieldStruct {
                left: Left { first_field: false },
                right: Right { second_field: 0 },
            },
        ]
    )
}
