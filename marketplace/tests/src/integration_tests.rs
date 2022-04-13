#[cfg(test)]
#[allow(unused)]
pub mod tests {
    use crate::casmarket_instance::{CasMarketInstance, TokenId};
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
        DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG,
        DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_PAYMENT,
    };
    use casper_execution_engine::core::engine_state::{
        run_genesis_request::RunGenesisRequest, GenesisAccount,
    };
    use casper_types::U256;
    use casper_types::{
        account::AccountHash, runtime_args, Key, Motes, PublicKey, RuntimeArgs, SecretKey, U512,
    };
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use test_env::TestEnv;
    pub type Meta = BTreeMap<String, String>;
    const NAME: &str = "CASTOKEN";
    const SYMBOL: &str = "CASTOKEN";
    pub fn contract_meta() -> Meta {
        let mut meta = Meta::new();
        meta.insert("origin".to_string(), "small".to_string());
        meta
    }


    fn deploy() -> (TestEnv, CasMarketInstance, AccountHash) {
        let env = TestEnv::new();
        let owner = env.next_user();
        let token = CasMarketInstance::new(&env, NAME, owner, NAME, SYMBOL, contract_meta());
        (env, token, owner)
    }
    #[test]
    fn test_deploy() {
        let (_, token, owner) = deploy();
        assert!(token.is_admin(owner));
        assert_eq!(token.name(), NAME);
        assert_eq!(token.symbol(), SYMBOL);
        assert_eq!(token.meta(), contract_meta());
        assert_eq!(token.total_supply(), U256::zero());
        
    }
}

#[allow(unused)]
fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
