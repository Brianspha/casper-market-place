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
    const NAME: &str = "casmarkettoken";
    const SYMBOL: &str = "casmarkettoken";
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

    #[test]
    fn test_token_meta() {
        let (env, token, owner) = deploy();
        let user = env.next_user();
        let token_id = TokenId::zero();
        let token_meta = contract_meta();

        token.grant_admin(owner,owner);
        token.grant_admin(owner,user);
        token.grant_minter(owner,owner);
        token.grant_minter(user,user);
        token.mint_one(owner, user, token_id, token_meta.clone());
    
        let user_token_meta = token.token_meta(token_id);
        assert_eq!(user_token_meta.unwrap(), token_meta);
    
        let first_user_token = token.get_token_by_index(Key::Account(user), U256::zero());
        assert_eq!(first_user_token, Some(token_id));
    }
}

#[allow(unused)]

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
