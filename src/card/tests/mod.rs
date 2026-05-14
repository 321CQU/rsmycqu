//! *Card所属feature并非默认启用，请在测试时启用`--all-features`标志以测试该模块*

use rstest::*;
use serde_json::json;

use crate::{
    card::{Card, EnergyFees, access_card},
    errors::ApiError,
    session::Session,
    utils::test_fixture::{access_card_session, login_session, shared_client},
};

#[rstest]
#[ignore]
#[tokio::test]
async fn test_access_card(
    #[future] login_session: Session,
    shared_client: &'static crate::session::Client,
) {
    {
        let mut session = Session::new();
        let res = access_card(shared_client, &mut session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotLogin));
    }
    let mut session = login_session.await.clone();
    access_card(shared_client, &mut session).await.unwrap();
    assert!(session.access_infos().card_access_info.is_some());
}

#[rstest]
fn test_page_ticket_parse() {
    let text = include_str!("page_ticket_html.html");

    assert_eq!(
        regex!("ticket=(.*)\'")
            .captures(text)
            .and_then(|item| item.get(1))
            .unwrap()
            .as_str(),
        "4552BB7524AB492E83587A9E57E9E995"
    )
}

#[rstest]
fn test_parse_energy_fee_huxi_response_without_unit_suffix() {
    let json = json!({
        "剩余金额": "51.89",
        "电剩余补助": "0",
        "水剩余补助": "74.30",
        "电表标志": "e",
        "电表地址": "000000461390",
        "电表读数": "13729.93",
        "水表标志": "w",
        "水表地址": "00000170462452",
        "水表读数": "605.40"
    });

    let fees: EnergyFees = serde_json::from_value(json).unwrap();

    assert_eq!(
        fees,
        EnergyFees {
            balance: "51.89".to_string(),
            subsidies: crate::card::Subsidy::Huxi {
                electricity: "0".to_string(),
                water: "74.30".to_string(),
            },
        }
    );
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_energy_fee(
    #[future] access_card_session: Session,
    shared_client: &'static crate::session::Client,
) {
    {
        let mut session = Session::new();
        let res = EnergyFees::fetch_self(shared_client, &mut session, "b5321", true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    let mut session = access_card_session.await.clone();
    EnergyFees::fetch_self(shared_client, &mut session, "b5321", true)
        .await
        .unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_card(
    #[future] access_card_session: Session,
    shared_client: &'static crate::session::Client,
) {
    {
        let session = Session::new();
        let res = Card::fetch_self(shared_client, &session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    let session = access_card_session.await.clone();
    Card::fetch_self(shared_client, &session).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_bills(
    #[future] access_card_session: Session,
    shared_client: &'static crate::session::Client,
) {
    let session = access_card_session.await.clone();
    let card = Card::fetch_self(shared_client, &session).await.unwrap();
    {
        let session = Session::new();
        let res = card
            .fetch_bill(shared_client, &session, "2023-11-10", "2023-12-12", 1, 100)
            .await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ApiError::NotAccess));
    }

    card.fetch_bill(shared_client, &session, "2023-11-10", "2023-12-12", 1, 100)
        .await
        .unwrap();
}
