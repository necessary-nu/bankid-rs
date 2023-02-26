#[cfg(feature = "sync")]
pub use test as maybe_async_test;

use maybe_async::maybe_async;
use rand::Rng;
#[cfg(feature = "async")]
pub use tokio::test as maybe_async_test;

use bankid::config::CA_TEST;
use bankid::{
    client::BankID,
    config::{ConfigBuilder, Pkcs12},
    model::{AuthenticatePayloadBuilder, CancelPayload, CollectPayload},
};

#[maybe_async]
#[maybe_async_test]
async fn test_authenticate() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .personal_number(personal_number())
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    bank_id.authenticate(payload).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_sign() {}

#[maybe_async]
#[maybe_async_test]
async fn test_collect() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .personal_number(personal_number())
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = bank_id.authenticate(payload).await.unwrap();

    bank_id
        .collect(CollectPayload {
            order_ref: authenticate.order_ref,
        })
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_cancel() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .personal_number(personal_number())
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = bank_id.authenticate(payload).await.unwrap();

    bank_id
        .cancel(CancelPayload {
            order_ref: authenticate.order_ref,
        })
        .await
        .unwrap();
}

fn client() -> BankID {
    let pkcs12 = Pkcs12::Der {
        der: P12_TEST.to_vec(),
        password: "qwerty123".to_string(),
    };

    let config = ConfigBuilder::default()
        .pkcs12(pkcs12)
        .url("https://appapi2.test.bankid.com/rp/v5.1".to_string())
        .ca(CA_TEST.to_string())
        .build()
        .unwrap();

    BankID::new(config)
}

fn personal_number() -> String {
    let mut rng = rand::thread_rng();

    format!(
        "19{:0>2}{:0>2}{:0>2}{}",
        rng.gen_range(1..100),
        rng.gen_range(1..12),
        rng.gen_range(1..28),
        rng.gen_range(1000..9999),
    )
}

const P12_TEST: &'static [u8] = include_bytes!("../resources/testcert.p12");
