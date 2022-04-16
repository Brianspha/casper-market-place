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
        meta.insert("origin".to_string(), "{price:1000.1}".to_string());
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
    fn test_mint_copies() {
        let (env, token, owner) = deploy();
        let user = env.next_user();
        let token_id = TokenId::zero();
        let token_meta = contract_meta();
        token.mint_copies(owner, user, None, token_meta, 3);
        assert_eq!(token.total_supply(), U256::from("3"));
    }
    #[test]
    fn test_delegate_nft() {
        let (mut env, mut token, mut owner) = deploy();
        let user = env.next_user();
        let token_id = TokenId::zero();
        let token_meta = contract_meta();
        token.mint_copies(owner, user, None, token_meta, 3);
        if let Ok(number) = U512::from_dec_str("111.11") {
            token.delagate_nft(user, TokenId::zero(), number);

            assert!(token.token_delegated(TokenId::zero()), "{}", true);
        }
    }
    #[test]
    fn test_delegate_revoke_nft() {
        let (mut env, mut token, mut owner) = deploy();
        let user = env.next_user();
        let token_id = TokenId::zero();
        let token_meta = contract_meta();
        token.mint_copies(owner, user, None, token_meta, 3);
        if let Ok(number) = U512::from_dec_str("111.11") {
            token.delagate_nft(user, TokenId::zero(), number);
            assert!(token.token_delegated(TokenId::zero()), "{}", true);
            token.revoke_delegation(
                user,
                TokenId::zero(),
                token.owner_of(TokenId::zero()).unwrap(),
            );
            assert_eq!(
                token.owner_of(TokenId::zero()).unwrap(),
                casper_types::Key::Account(owner)
            );
        }
    }
    #[test]
    fn test_purchase_nft() {
        let (mut env, mut token, mut owner) = deploy();
        let user = env.next_user();
        let token_id = TokenId::zero();
        let token_meta = contract_meta();
        token.mint_copies(owner, user, None, token_meta, 3);
        if let Ok(number) = U512::from_dec_str("111.11") {
            token.delagate_nft(user, TokenId::zero(), number);
            assert!(token.token_delegated(TokenId::zero()), "{}", true);
            token.purchase_nft(owner, TokenId::zero(), number);
            assert_eq!(
                token.owner_of(TokenId::zero()).unwrap(),
                casper_types::Key::Account(owner)
            );
            assert_eq!(token.balance_of(Key::Account(owner)), U256::from(1));
        }
    }
}
#[allow(unused)]

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
