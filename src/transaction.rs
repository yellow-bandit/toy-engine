//! Object definitions for transactions.

use derive_more::Constructor;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::engine::ClientId;

#[derive(Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct TransactionId(u32);

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Operation {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

/// Struct representation of a transaction record from the input file.
#[derive(Deserialize)]
pub(crate) struct TransactionRecord {
    pub(crate) r#type: Operation,
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
    pub(crate) amount: Option<Decimal>,
}

#[derive(Constructor)]
pub(crate) struct Deposit {
    pub(crate) client: ClientId,
    pub(crate) amount: Decimal,
}

#[derive(Constructor)]
pub(crate) struct Withdrawal {
    pub(crate) client: ClientId,
    pub(crate) amount: Decimal,
}

#[derive(Constructor)]
pub(crate) struct Dispute {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Constructor)]
pub(crate) struct Resolve {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Constructor)]
pub(crate) struct Chargeback {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

pub(crate) enum DisputeState {
    Undisputed,
    Disputed,
    Chargedback,
}

#[derive(Constructor)]
pub(crate) struct DisputableTransaction {
    pub(crate) deposit: Deposit,
    pub(crate) state: DisputeState,
}
