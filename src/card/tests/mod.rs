//! *Card所属feature并非默认启用，请在测试时启用`--all-features`标志以测试该模块*

use rstest::*;
use crate::card::{access_card, Card, EnergyFees};

use crate::errors::Error;
use crate::session::Session;
use crate::utils::test_fixture::{login_session, access_card_session};

#[rstest]
#[ignore]
#[tokio::test]
async fn test_access_card(#[future] login_session: Session) {
    {
        let mut session = Session::new();
        let res = access_card(&mut session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotLogin));
    }
    let mut session = login_session.await.clone();
    access_card(&mut session).await.unwrap();
    assert!(session.card_access_info.is_some());
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
#[ignore]
#[tokio::test]
async fn test_fetch_energy_fee(#[future] access_card_session: Session) {
    {
        let mut session = Session::new();
        let res = EnergyFees::fetch_self(&mut session, "b5321", true).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }

    let mut session = access_card_session.await.clone();
    EnergyFees::fetch_self(&mut session, "b5321", true).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_card(#[future] access_card_session: Session) {
    {
        let session = Session::new();
        let res = Card::fetch_self(&session).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }

    let session = access_card_session.await.clone();
    Card::fetch_self(&session).await.unwrap();
}

#[rstest]
#[ignore]
#[tokio::test]
async fn test_fetch_bills(#[future] access_card_session: Session) {
    let session = access_card_session.await.clone();
    let card = Card::fetch_self(&session).await.unwrap();
    {
        let session = Session::new();
        let res = card.fetch_bill(&session, "2023-11-10", "2023-12-12", 1, 100).await;
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::NotAccess));
    }

    card.fetch_bill(&session, "2023-11-10", "2023-12-12", 1, 100).await.unwrap();
}
