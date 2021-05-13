mod common;

#[tokio::test]
async fn t_two_struct_fields_struct() {
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
        left: Left,
        right: Right,
    }

    let out = common::pg_query_and_de::<OneFieldStruct>(
        "SELECT true left_first_field, 42 right_second_field UNION ALL SELECT false, 0",
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

#[tokio::test]
async fn t_deep_struct() {
    // common::init_logger();

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct A {
        the_field: bool,
    }
    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct B {
        a: A,
    }

    #[derive(Debug, ::serde::Deserialize, PartialEq, Eq)]
    struct C {
        b: B,
    }

    let out = common::pg_query_and_de::<C>("SELECT true b_a_the_field UNION ALL SELECT false", &[])
        .await
        .unwrap();
    assert_eq!(
        out,
        vec![
            C {
                b: B {
                    a: A { the_field: true },
                },
            },
            C {
                b: B {
                    a: A { the_field: false },
                },
            },
        ]
    )
}
