#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, Env, Vec, TryFromVal, IntoVal, Val, Symbol, Error};
use soroban_sdk::token::TokenClient;

pub struct PaymentAndMessagingSystem;

#[derive(Clone)]
pub struct Transaction {
    recipient: Address,
    amount: i128,
    message: Bytes,
}

impl TryFromVal<Env, Val> for Transaction {
    type Error = Error;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (recipient, amount, message): (Address, i128, Bytes) = TryFromVal::try_from_val(env, val)?;
        Ok(Transaction { recipient, amount, message })
    }
}

impl IntoVal<Env, Val> for Transaction {
    fn into_val(&self, env: &Env) -> Val {
        (self.recipient.clone(), self.amount, self.message.clone()).into_val(env)
    }
}

#[contractimpl]
impl PaymentAndMessagingSystem {
    pub fn send_payment(env: Env, token_address: Address, sender: Address, recipient: Address, amount: i128, message: Bytes) {
        sender.require_auth();
        let client = TokenClient::new(&env, &token_address);
        client.transfer(&sender, &recipient, &amount);
        env.events().publish::<(Address, Symbol), Transaction>(
            (recipient.clone(), Symbol::new(&env, "payment_sent")),
            Transaction { recipient, amount, message },
        );
    }

    pub fn send_payment_multiple(env: Env, token_address: Address, sender: Address, recipients: Vec<Address>, amount: i128, message: Bytes) {
        sender.require_auth();
        let client = TokenClient::new(&env, &token_address);
        for recipient in recipients.iter() {
            client.transfer(&sender, &recipient, &amount);
        }
        env.events().publish::<(Symbol, Address), Bytes>(
            (Symbol::new(&env, "bulk_payment"), sender),
            message,
        );
    }

    pub fn get_balance(env: Env, token_address: Address, address: Address) -> i128 {
        let client = TokenClient::new(&env, &token_address);
        client.balance(&address)
    }
}
