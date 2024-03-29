use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::U512;
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct CasMarketInstance(TestContract);

impl CasMarketInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        meta: Meta,
        ) -> CasMarketInstance {
        CasMarketInstance(TestContract::new(
            env,
            "casmarket_token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "meta" => meta,
            },
        ))
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, symbol: &str, meta: Meta) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "meta" => meta},
        );
    }
    pub fn grant_admin<T: Into<Key>>(&self, sender: AccountHash, admin: T) {
        self.0.call_contract(
            sender,
            "grant_admin",
            runtime_args! {
            "admin" => admin.into()},
        );
    }

    pub fn revoke_admin<T: Into<Key>>(&self, sender: AccountHash, admin: T) {
        self.0.call_contract(
            sender,
            "revoke_admin",
            runtime_args! {
            "admin" => admin.into()},
        );
    }

    pub fn grant_minter<T: Into<Key>>(&self, sender: AccountHash, minter: T) {
        self.0.call_contract(
            sender,
            "grant_minter",
            runtime_args! {
            "minter" => minter.into()},
        );
    }

    pub fn revoke_minter<T: Into<Key>>(&self, sender: AccountHash, minter: T) {
        self.0.call_contract(
            sender,
            "revoke_minter",
            runtime_args! {
            "minter" => minter.into()},
        );
    }


    pub fn is_admin<T: Into<Key>>(&self, account: T) -> bool {
        self.0
            .query_dictionary::<()>("admins", key_to_str(&account.into()))
            .is_some()
    }

    pub fn is_minter<T: Into<Key>>(&self, account: T) -> bool {
        self.0
            .query_dictionary::<()>("minters", key_to_str(&account.into()))
            .is_some()
    }

    pub fn mint<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
        token_metas: Vec<Meta>,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids,
                "token_metas" => token_metas,
            },
        )
    }
    pub fn mint_one<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_id: TokenId,
        token_meta: Meta,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => vec![token_id],
                "token_metas" => vec![token_meta]
            },
        )
    }
    pub fn purchase_nft(&self, sender: AccountHash, token_id: TokenId, amount:U512) {
        self.0.call_contract(
            Key::into_account(casper_types::Key::Account(sender)).unwrap(),
            "purchase_nft",
            runtime_args! {
                "token_id" => token_id,
                "amount"=>amount,
            },
        )
    }
 
    pub fn update_token_commission<T: Into<Key>>(
        &self,
        sender: AccountHash,
        token_id: TokenId,
        value: U512,
    ) {
        self.0.call_contract(
            sender,
            "update_token_commission",
            runtime_args! {
                "token_id" => token_id,
                "value" => value,
            },
        )
    }
    pub fn delagate_nft(&mut self, sender: AccountHash, token_id: TokenId,token_price: U512) {
        self.0.call_contract(
            Key::into_account(casper_types::Key::Account(sender)).unwrap(),
            "delagate_nft",
            runtime_args! {
                "token_id" => token_id,
                "token_price"=> token_price
            },
        )
    }
    pub fn token_delegated(&mut self,token_id: TokenId) -> bool{
        match self.0.query_dictionary("token_delegated", token_id.to_string()) {
            Some(delegated) =>delegated,
            _ => false
        }

    }
    pub fn revoke_delegation(&mut self, sender: AccountHash, token_id: TokenId, owner: Key) {
        self.0.call_contract(
            Key::into_account(casper_types::Key::Account(sender)).unwrap(),
            "revoke_delegation",
            runtime_args! {
                "token_id" => token_id,
                "owner" => owner,
            },
        )
    }
    pub fn mint_copies<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Option<Vec<TokenId>>,
        token_meta: Meta,
        count: u32,
    ) {
        self.0.call_contract(
            sender,
            "mint_copies",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids,
                "token_meta" => token_meta,
                "count" => count
            },
        )
    }

    pub fn mint_many<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
        token_metas: Vec<Meta>,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids,
                "token_metas" => token_metas
            },
        )
    }

    pub fn burn_one<T: Into<Key>>(&self, sender: AccountHash, owner: T, token_id: TokenId) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "owner" => owner.into(),
                "token_ids" => vec![token_id]
            },
        )
    }

    pub fn burn_many<T: Into<Key>>(&self, sender: AccountHash, owner: T, token_ids: Vec<TokenId>) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "owner" => owner.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn transfer<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
    ) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn transfer_from<T: Into<Key>>(
        &self,
        sender: AccountHash,
        owner: T,
        recipient: T,
        token_ids: Vec<TokenId>,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "sender" => owner.into(),
                "recipient" => recipient.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn approve<T: Into<Key>>(&self, sender: AccountHash, spender: T, token_ids: Vec<TokenId>) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {"spender" => spender.into(), "token_ids" => token_ids},
        )
    }

    pub fn get_approved<T: Into<Key>>(&self, owner: T, token_id: TokenId) -> Option<Key> {
        self.0.query_dictionary(
            "allowances",
            key_and_value_to_str::<String>(&owner.into(), &token_id.to_string()),
        )
    }

    pub fn update_token_meta(&self, sender: AccountHash, token_id: TokenId, token_meta: Meta) {
        self.0.call_contract(
            sender,
            "update_token_meta",
            runtime_args! {
                "token_id" => token_id,
                "token_meta" => token_meta
            },
        )
    }

    pub fn get_token_by_index<T: Into<Key>>(&self, account: T, index: U256) -> Option<TokenId> {
        self.0.query_dictionary(
            "owned_tokens_by_index",
            key_and_value_to_str(&account.into(), &index),
        )
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        self.0.query_dictionary("owners", token_id.to_string())
    }

    pub fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        self.0.query_dictionary("metadata", token_id.to_string())
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
    }

    pub fn meta(&self) -> Meta {
        self.0.query_named_key(String::from("meta"))
    }
    pub fn delegate_nft(&self) -> Meta {
        self.0.query_named_key(String::from("meta"))
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
