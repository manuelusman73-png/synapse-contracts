#[cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String as SorobanString, vec};
use crate::{SynapseContract, SynapseContractClient, types::TransactionStatus};

fn setup(env: &Env) -> (Address, SynapseContractClient) {
    env.mock_all_auths();
    let id = env.register_contract(None, SynapseContract);
    let client = SynapseContractClient::new(env, &id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (admin, client)
}

fn usd(env: &Env) -> SorobanString { SorobanString::from_str(env, "USD") }

// ---------------------------------------------------------------------------

