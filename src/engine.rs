//! Module for transaction processing.

use std::ops::Not;

use csv::{Reader, Writer};
use rust_decimal::Decimal;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::{
    transaction::{
        Chargeback, Deposit, DisputableTransaction, Dispute, DisputeState, Operation, Resolve,
        TransactionId, TransactionRecord, Withdrawal,
    },
    Error,
};

/// Transaction engine responsible to store and process transactions.
#[derive(Default)]
pub struct Engine {
    disputable_transactions: FxHashMap<TransactionId, DisputableTransaction>,
    clients: FxHashMap<ClientId, ClientData>,
}

impl Engine {
    /// Loads transactions from a `csv::Reader`.
    pub fn load_from_reader<R: std::io::Read>(
        &mut self,
        mut reader: Reader<R>,
    ) -> Result<(), Error> {
        let iter = reader.deserialize();

        for result in iter {
            let record: TransactionRecord = result?;
            self.load_record(&record);
        }
        Ok(())
    }

    /// Loads one record (= one transaction) in the engine.
    fn load_record(&mut self, record: &TransactionRecord) {
        match record.r#type {
            Operation::Deposit => self.process_deposit(
                record.tx,
                Deposit::new(
                    record.client,
                    record.amount.expect("deposits must indicate the amount"),
                ),
            ),
            Operation::Withdrawal => self.process_withdrawal(Withdrawal::new(
                record.client,
                record.amount.expect("withdrawals must indicate the amount"),
            )),
            Operation::Dispute => self.process_dispute(Dispute::new(record.client, record.tx)),
            Operation::Resolve => self.process_resolve(Resolve::new(record.client, record.tx)),
            Operation::Chargeback => {
                self.process_chargeback(Chargeback::new(record.client, record.tx))
            }
        };
    }

    /// Writes the accounts state into a `csv::Writer`.
    pub fn dump_accounts<W: std::io::Write>(self, mut writer: Writer<W>) -> Result<(), Error> {
        for (id, data) in self.clients {
            writer.serialize(ClientRecord::from_id_and_data(id, data))?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Returns the set of all clients and their data.
    pub fn clients(&self) -> &FxHashMap<ClientId, ClientData> {
        &self.clients
    }

    /// Returns the set of all clients as a vector ordered by client id.
    pub fn clients_ordered(&self) -> Vec<ClientRecord> {
        let mut vec: Vec<_> = self
            .clients
            .iter()
            .map(|(id, data)| ClientRecord::from_id_and_data(*id, *data))
            .collect();
        vec.sort_by(|a, b| a.client.cmp(&b.client));
        vec
    }

    /// Processes a transaction of type: deposit.
    fn process_deposit(&mut self, tx: TransactionId, deposit: Deposit) {
        // Get or create the client.
        let client = self.clients.entry(deposit.client).or_default();
        // Client must not be locked.
        if client.locked {
            return;
        }
        // Increase available funds and save the transaction in memory.
        client.available += deposit.amount;
        self.disputable_transactions.insert(
            tx,
            DisputableTransaction::new(deposit, DisputeState::Undisputed),
        );
    }

    /// Processes a transaction of type: withdrawal.
    fn process_withdrawal(&mut self, withdrawal: Withdrawal) {
        // Client must exist.
        if let Some(client) = self.clients.get_mut(&withdrawal.client) {
            // Client must not be locked.
            if client.locked {
                return;
            }
            // Withdraw the money only if it's available.
            if client.available >= withdrawal.amount {
                client.available -= withdrawal.amount;
            }
        }
    }

    /// Processes a transaction of type: dispute.
    fn process_dispute(&mut self, dispute: Dispute) {
        // Client must exist.
        if let Some(client) = self.clients.get_mut(&dispute.client) {
            // The transaction to be disputed must exist.
            if let Some(disputable_tx) = self.disputable_transactions.get_mut(&dispute.tx) {
                // And be in the correct state.
                if matches!(disputable_tx.state, DisputeState::Undisputed).not() {
                    return;
                }
                // Client id must be the same.
                if disputable_tx.deposit.client != dispute.client {
                    return;
                }
                // Hold the money and change the transaction state.
                client.available -= disputable_tx.deposit.amount;
                client.held += disputable_tx.deposit.amount;
                disputable_tx.state = DisputeState::Disputed;
            }
        }
    }

    /// Processes a transaction of type: resolve.
    fn process_resolve(&mut self, resolve: Resolve) {
        // Client must exist.
        if let Some(client) = self.clients.get_mut(&resolve.client) {
            // The transaction to be resolved must exist.
            if let Some(disputable_tx) = self.disputable_transactions.get_mut(&resolve.tx) {
                // And be in the correct state.
                if matches!(disputable_tx.state, DisputeState::Disputed).not() {
                    return;
                }
                // Client id must be the same.
                if disputable_tx.deposit.client != resolve.client {
                    return;
                }
                // Unblock the money and change the transaction state.
                client.available += disputable_tx.deposit.amount;
                client.held -= disputable_tx.deposit.amount;
                disputable_tx.state = DisputeState::Undisputed;
            }
        }
    }

    /// Processes a transaction of type: chargeback.
    fn process_chargeback(&mut self, chargeback: Chargeback) {
        // Client must exist.
        if let Some(client) = self.clients.get_mut(&chargeback.client) {
            // The transaction for chargeback must exist.
            if let Some(disputable_tx) = self.disputable_transactions.get_mut(&chargeback.tx) {
                // And be in the correct state.
                if matches!(disputable_tx.state, DisputeState::Disputed).not() {
                    return;
                }
                // Client id must be the same.
                if disputable_tx.deposit.client != chargeback.client {
                    return;
                }
                // Return the money, lock the client and change the transaction state.
                client.held -= disputable_tx.deposit.amount;
                client.locked = true;
                disputable_tx.state = DisputeState::Chargedback;
            }
        }
    }
}

/// Id of a client.
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct ClientId(u16);

/// Data for a client.
#[derive(Serialize, Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct ClientData {
    available: Decimal,
    held: Decimal,
    locked: bool,
}

/// Record with all client information.
#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct ClientRecord {
    client: ClientId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

impl ClientRecord {
    fn from_id_and_data(client: ClientId, data: ClientData) -> Self {
        Self::new(client.0, data.available, data.held, data.locked)
    }

    /// Creates a new client record.
    pub fn new(client: u16, available: Decimal, held: Decimal, locked: bool) -> Self {
        Self {
            client: ClientId(client),
            available,
            held,
            total: available + held,
            locked,
        }
    }
}
