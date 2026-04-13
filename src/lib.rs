#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct NaijaRemitLock;

#[contractimpl]
impl NaijaRemitLock {
    pub fn create_lockbox(
        env: Env,
        sender: Address,
        beneficiary: Address,
        purpose: Symbol,
        release_time: u64,
    ) -> Symbol {
        sender.require_auth();
        symbol_short!("CREATED")
    }

    pub fn release(env: Env, lockbox_id: Symbol) -> Symbol {
        symbol_short!("RELEASED")
    }

    pub fn emergency_cancel(env: Env, lockbox_id: Symbol) -> Symbol {
        symbol_short!("CANCELED")
    }
}