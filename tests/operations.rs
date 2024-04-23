use csv::Reader;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use toy_engine::{engine::ClientRecord, Engine};

#[test]
fn multiple_clients_deposit() {
    let data = "\
type,client,tx,amount
deposit,1,1,1.0
deposit,2,2,2.0
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![
            ClientRecord::new(1, 1.into(), 0.into(), false),
            ClientRecord::new(2, 2.into(), 0.into(), false)
        ]
    );
}

#[test]
fn successful_withdrawal() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
withdrawal,1,2,1
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 2.into(), 0.into(), false),]
    );
}

#[test]
fn full_withdrawal() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
withdrawal,1,2,3
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 0.into(), false),]
    );
}

#[test]
fn unsuccessful_withdrawal() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
withdrawal,1,2,4
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 3.into(), 0.into(), false),]
    );
}

#[test]
fn successful_dispute() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 3.into(), false),]
    );
}

#[test]
fn successful_dispute_negative_balance() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
withdrawal,1,2,1
dispute,1,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(
            1,
            Decimal::from_i32(-1).unwrap(),
            3.into(),
            false
        ),]
    );
}

#[test]
fn dispute_wrong_tx() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,3,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 3.into(), 0.into(), false),]
    );
}

#[test]
fn dispute_wrong_client() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,2,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 3.into(), 0.into(), false),]
    );
}

#[test]
fn successful_resolve() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
resolve,1,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 3.into(), 0.into(), false),]
    );
}

#[test]
fn resolve_wrong_tx() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
resolve,1,2,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 3.into(), false),]
    );
}

#[test]
fn resolve_not_under_dispute() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
deposit,1,2,1
resolve,1,2,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 1.into(), 3.into(), false),]
    );
}

#[test]
fn resolve_wrong_client() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
resolve,2,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 3.into(), false),]
    );
}

#[test]
fn successful_chargeback() {
    let data = "\
type,client,tx,amount
deposit,1,1,1
deposit,1,2,3
dispute,1,1,
chargeback,1,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 3.into(), 0.into(), true),]
    );
}

#[test]
fn chargeback_wrong_tx() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
chargeback,1,2,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 3.into(), false),]
    );
}

#[test]
fn chargeback_not_under_dispute() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
deposit,1,2,1
chargeback,1,2,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 1.into(), 3.into(), false),]
    );
}

#[test]
fn chargeback_wrong_client() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
chargeback,2,1,
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 3.into(), false),]
    );
}

#[test]
fn chargeback_freeze() {
    let data = "\
type,client,tx,amount
deposit,1,1,3
dispute,1,1,
chargeback,1,1,
deposit,1,2,1
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![ClientRecord::new(1, 0.into(), 0.into(), true),]
    );
}

#[test]
fn interleaved_clients() {
    let data = "\
type,client,tx,amount
deposit,1,1,4
withdrawal,1,1,1
deposit,2,2,4
dispute,2,2,
resolve,2,2,
withdrawal,1,3,1
deposit,2,4,1
withdrawal,2,5,2
";
    let reader = Reader::from_reader(data.as_bytes());
    let mut engine = Engine::default();
    engine.load_from_reader(reader).unwrap();
    let clients = engine.clients_ordered();
    assert_eq!(
        clients,
        vec![
            ClientRecord::new(1, 2.into(), 0.into(), false),
            ClientRecord::new(2, 3.into(), 0.into(), false),
        ]
    );
}
