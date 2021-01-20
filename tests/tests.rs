use ::futures::prelude::*;
use ::serde_pgrow::prelude::*;

#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct StructOne {
    string_one: String,
    string_two: String,
}
#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct StructTwo {
    string_one: String,
    string_two: String,
}

#[tokio::test]
#[ignore]
async fn test_a_struct() {
    let tls = ::tokio_postgres::NoTls;

    let (client, connection) =
        ::tokio_postgres::connect("host=127.0.0.1 user=tests password=tests dbname=tests", tls)
            .await
            .unwrap();
    let rows = async move {
        client
            .query(
                r"
            SELECT 
                'left_one' _0_string_one, 'left_two' _0_string_two,
                'right_one' _1_string_one, 'right_two' _1_string_two
        ",
                &[],
            )
            .await
            .unwrap()
            .cast::<(StructOne, StructTwo)>()
            .unwrap()
    };

    let (rows, _) = future::join(rows, connection).await;

    println!("rows: {:#?}", rows);
}
