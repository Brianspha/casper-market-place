#![no_main]
#![no_std]
#[macro_use]
extern crate alloc;
use alloc::{
    boxed::Box,
    collections::BTreeSet,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::contract_api::storage::new_uref;
use casper_contract::contract_api::{
    account::get_main_purse, system::transfer_from_purse_to_purse,
};
use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, contracts::NamedKeys, runtime_args, ApiError, CLType, CLTyped, CLValue,
    ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, RuntimeArgs, URef, U256, U512,
};
use cep47::{
    contract_utils::{AdminControl, ContractContext, OnChainContractStorage},
    Error, Meta, TokenId, CEP47,
};
use core::convert::TryFrom;
use core::convert::TryInto;
mod minters_control;
use minters_control::MinterControl;
const CASMARKET_TOKEN_KEY: &str = "casmarkettoken";

#[derive(Default)]
pub struct CasMarketToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for CasMarketToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP47<OnChainContractStorage> for CasMarketToken {}
impl AdminControl<OnChainContractStorage> for CasMarketToken {}
impl MinterControl<OnChainContractStorage> for CasMarketToken {}

impl CasMarketToken {
    fn constructor(&mut self, name: String, symbol: String, meta: Meta) {
        CEP47::init(self, name, symbol, meta);
        AdminControl::init(self);
        MinterControl::init(self);
    }
    #[allow(unused_variables)]
    #[allow(unused)]
    fn purchase_nft(&mut self, token_id: TokenId) -> Result<(), Error> {
        let caller = CasMarketToken::default().get_caller();
        match self.owner_of(token_id.clone()) {
            Some(owner) => {
                match runtime::get_key(CASMARKET_TOKEN_KEY) {
                    Some(contract_hash) => {
                        if self.owner_of(token_id.clone()) == core::prelude::v1::Some(contract_hash)
                        {
                            //@dev here we check if the contract is the owner of the token im a noob in rust dev this is probably not very efficient code
                            if owner != caller {
                                let amount: U512 = runtime::get_named_arg("amount");
                                let token_price =
                                    CasMarketToken::get_token_price::<U512>(&token_id);
                                let mut valid_token_price: bool = false;
                                match token_price {
                                    core::result::Result::Ok(price) => {
                                        if amount.is_zero() || price > amount {
                                            valid_token_price = true;
                                        }
                                    }
                                    core::result::Result::Err(price) => {
                                        revert(ApiError::User(20));
                                    }
                                }
                                if valid_token_price {
                                    //self.transfer(recipient,token);
                                    //self.transfer_tokens(,amount)
                                    Ok(())
                                } else {
                                    revert(ApiError::User(20));
                                }
                            } else {
                                revert(ApiError::User(20));
                            }
                        } else {
                            revert(ApiError::User(20));
                        }
                    }
                    None => {
                        revert(ApiError::User(20));
                    }
                }
            }
            None => Ok(()),
        }
    }
    fn mint(
        &mut self,
        recipient: Key,
        token_ids: Option<Vec<TokenId>>,
        token_metas: Vec<Meta>,
    ) -> Result<Vec<TokenId>, Error> {
        let caller = CasMarketToken::default().get_caller();
        if !CasMarketToken::default().is_minter() && !CasMarketToken::default().is_admin(caller) {
            revert(ApiError::User(20));
        }

        let confirmed_token_ids =
            CEP47::mint(self, recipient, token_ids, token_metas).unwrap_or_revert();
        Ok(confirmed_token_ids)
    }

    fn mint_copies(
        &mut self,
        recipient: Key,
        token_ids: Option<Vec<TokenId>>,
        token_meta: Meta,
        count: u32,
    ) -> Result<Vec<TokenId>, Error> {
        let caller = CasMarketToken::default().get_caller();
        if !CasMarketToken::default().is_minter() && !CasMarketToken::default().is_admin(caller) {
            revert(ApiError::User(20));
        }
        if let Some(token_ids) = &token_ids {
            if token_ids.len() != count as usize {
                return Err(Error::WrongArguments);
            }
        }
        let token_metas = vec![token_meta; count as usize];
        self.mint(recipient, token_ids, token_metas)
    }

    fn burn(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = CasMarketToken::default().get_caller();
        if !CasMarketToken::default().is_minter() && !CasMarketToken::default().is_admin(caller) {
            revert(ApiError::User(20));
        }
        CEP47::burn_internal(self, owner, token_ids).unwrap_or_revert();
        Ok(())
    }

    fn set_token_meta(&mut self, token_id: TokenId, token_meta: Meta) -> Result<(), Error> {
        let caller = CasMarketToken::default().get_caller();
        if !CasMarketToken::default().is_minter() && !CasMarketToken::default().is_admin(caller) {
            revert(ApiError::User(20));
        }
        CEP47::set_token_meta(self, token_id, token_meta).unwrap_or_revert();
        Ok(())
    }

    fn update_token_meta(
        &mut self,
        token_id: TokenId,
        token_meta_key: String,
        token_meta_value: String,
    ) -> Result<(), Error> {
        let caller = CasMarketToken::default().get_caller();
        if !CasMarketToken::default().is_minter() && !CasMarketToken::default().is_admin(caller) {
            revert(ApiError::User(20));
        }
        let mut token_meta = CasMarketToken::default()
            .token_meta(token_id.clone())
            .unwrap_or_revert();
        token_meta.insert(token_meta_key, token_meta_value);
        CEP47::set_token_meta(self, token_id, token_meta).unwrap_or_revert();
        Ok(())
    }

    fn update_token_commission(&mut self, token_id: TokenId, value: U512) -> Result<(), Error> {
        if self.owner_of(token_id.clone()).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };
        CasMarketToken::store_value(&token_id, value);
        Ok(())
    }
    fn store_value<T: ToBytes + CLTyped>(name: &str, value: T) {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                storage::write(key_ref, value);
            }
            None => {
                let key = storage::new_uref(value).into();
                runtime::put_key(name, key);
            }
        }
    }
    fn remove_value(name: &str) {
        runtime::remove_key(name)
    }
    fn get_token_price<T: ToBytes + CLTyped + casper_types::bytesrepr::FromBytes>(
        name: &str,
    ) -> Result<U512, Error> {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ = key.try_into().unwrap_or_revert();
                let value = storage::read(key_).unwrap_or_revert().unwrap_or_revert();
                Ok(value)
            }
            None => Err(Error::TokenIdDoesntExist),
        }
    }

    fn get_token_delegated<T: ToBytes + CLTyped + casper_types::bytesrepr::FromBytes>(
        name: &str,
    ) -> Result<bool, Error> {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ = key.try_into().unwrap_or_revert();
                let value = storage::read(key_).unwrap_or_revert().unwrap_or_revert();
                Ok(value)
            }
            None => Err(Error::TokenIdDoesntExist),
        }
    }
    fn delagate_nft(
        &mut self,
        token_id: TokenId,
        owner: Key,
        token_price: U512,
    ) -> Result<String, Error> {
        match CasMarketToken::get_token_delegated::<bool>(token_id.as_str()) {
            core::result::Result::Ok(delegated) => {
                if delegated {
                    //@dev already delegated
                    revert(ApiError::User(20));
                }
                match runtime::get_key(CASMARKET_TOKEN_KEY) {
                    //@dev here we fetch the contract address then transfer nft token ownership to it and mark the token as delegated
                    Some(contract_hash) => {
                        if self.owner_of(token_id.clone()) == core::prelude::v1::Some(owner) {
                            CasMarketToken::store_value(token_id.as_str(), true);
                            CasMarketToken::store_value(token_id.as_str(), token_price);
                            match self.transfer(contract_hash, [token_id].to_vec()) {
                                core::result::Result::Ok(_results) => {
                                    Ok("Succesfully delegated token".to_string())
                                }
                                _ => Ok("Unable to transfer token to contract".to_string()),
                            }
                        } else {
                            Ok("Internal contract error".to_string())
                        }
                    }
                    _ => Ok("Internal contract error".to_string()),
                }
            }
            _ => {
                revert(ApiError::User(20));
            }
        }
    }
    #[allow(unused_variables)]
    #[allow(unused)]
    fn revoke_delegation(&mut self, token_id: TokenId, owner: Key) -> Result<String, Error> {
        match CasMarketToken::get_token_delegated::<bool>(token_id.as_str()) {
            core::result::Result::Ok(delegated) => {
                if !delegated {
                    //@dev already delegated
                    revert(ApiError::User(20));
                }
                match runtime::get_key(CASMARKET_TOKEN_KEY) {
                    //@dev here we fetch the contract address then transfer nft token ownership to it and mark the token as delegated
                    Some(contract_hash) => {
                        if self.owner_of(token_id.clone()) == core::prelude::v1::Some(contract_hash) {
                            CasMarketToken::remove_value(token_id.as_str());
                            CasMarketToken::remove_value(token_id.as_str()); //@dev -1 represents 
                            match self.transfer(owner, [token_id].to_vec()) {
                                core::result::Result::Ok(_results) => {
                                    Ok("Succesfully revoked delegation to token".to_string())
                                }
                                _ => Ok("Unable to transfer token from contract to user".to_string()),
                            }
                        } else {
                            Ok("Internal contract error".to_string())
                        }
                    }
                    _ => Ok("Internal contract error".to_string()),
                }
            }
            _ => {
                revert(ApiError::User(20));
            }
        }
    }
    #[allow(unused_variables)]
    #[allow(unused)]
    fn transfer_tokens(&mut self, recipient: Key, amount: U512) -> Result<(), Error> {
        match URef::try_from(recipient) {
            core::prelude::v1::Ok(recipient) => {
                transfer_from_purse_to_purse(get_main_purse(), new_uref(recipient), amount, None)
                    .unwrap_or_revert();
                Ok(())
            }
            _ => {
                revert(ApiError::User(20));
            }
        }
    }
}

#[no_mangle]
fn update_token_commission() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let value = runtime::get_named_arg::<U512>("value");
    CasMarketToken::default()
        .update_token_commission(token_id, value)
        .unwrap_or_revert();
}
#[no_mangle]
fn delagate_nft() {
    let caller = CasMarketToken::default().get_caller();
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_price = runtime::get_named_arg::<U512>("token_price");

    CasMarketToken::default()
        .delagate_nft(token_id, caller, token_price)
        .unwrap_or_revert();
}
#[no_mangle]
fn revoke_delagation() {
    let caller = CasMarketToken::default().get_caller();
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    CasMarketToken::default()
        .revoke_delegation(token_id, caller)
        .unwrap_or_revert();
}
#[no_mangle]
fn purchase_nft() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    CasMarketToken::default()
        .purchase_nft(token_id)
        .unwrap_or_revert();
}
#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    let symbol = runtime::get_named_arg::<String>("symbol");
    let meta = runtime::get_named_arg::<Meta>("meta");
    let admin = runtime::get_named_arg::<Key>("admin");
    CasMarketToken::default().constructor(name, symbol, meta);
    CasMarketToken::default().add_admin_without_checked(admin);
}

#[no_mangle]
fn name() {
    let ret = CasMarketToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret = CasMarketToken::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn meta() {
    let ret = CasMarketToken::default().meta();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret = CasMarketToken::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = CasMarketToken::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn owner_of() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = CasMarketToken::default().owner_of(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_token_by_index() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let index = runtime::get_named_arg::<U256>("index");
    let ret = CasMarketToken::default().get_token_by_index(owner, index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = CasMarketToken::default().token_meta(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    CasMarketToken::default()
        .set_token_meta(token_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
fn update_token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_meta_key = runtime::get_named_arg::<String>("token_meta_key");
    let token_meta_value = runtime::get_named_arg::<String>("token_meta_value");
    CasMarketToken::default()
        .update_token_meta(token_id, token_meta_key, token_meta_value)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Option<Vec<TokenId>>>("token_ids");
    let token_metas = runtime::get_named_arg::<Vec<Meta>>("token_metas");
    CasMarketToken::default()
        .mint(recipient, token_ids, token_metas)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint_copies() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Option<Vec<TokenId>>>("token_ids");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    let count = runtime::get_named_arg::<u32>("count");
    CasMarketToken::default()
        .mint_copies(recipient, token_ids, token_meta, count)
        .unwrap_or_revert();
}

#[no_mangle]
fn burn() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    CasMarketToken::default()
        .burn(owner, token_ids)
        .unwrap_or_revert()
}

#[no_mangle]
fn transfer() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    CasMarketToken::default()
        .transfer(recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer_from() {
    let sender = runtime::get_named_arg::<Key>("sender");
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    let caller = CasMarketToken::default().get_caller();
    if !CasMarketToken::default().is_admin(caller) {
        revert(ApiError::User(20));
    }
    CasMarketToken::default()
        .transfer_from_internal(sender, recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn grant_minter() {
    let minter = runtime::get_named_arg::<Key>("minter");
    CasMarketToken::default().assert_caller_is_admin();
    CasMarketToken::default().add_minter(minter);
}

#[no_mangle]
fn revoke_minter() {
    let minter = runtime::get_named_arg::<Key>("minter");
    CasMarketToken::default().assert_caller_is_admin();
    CasMarketToken::default().revoke_minter(minter);
}

#[no_mangle]
fn grant_admin() {
    let admin = runtime::get_named_arg::<Key>("admin");
    CasMarketToken::default().add_admin(admin);
}

#[no_mangle]
fn revoke_admin() {
    let admin = runtime::get_named_arg::<Key>("admin");
    CasMarketToken::default().disable_admin(admin);
}

#[no_mangle]
fn call() {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let meta: Meta = runtime::get_named_arg("meta");
    let admin: Key = runtime::get_named_arg("admin");

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        Some(NamedKeys::new()),
        Some(String::from("contract_package_hash")),
        None,
    );

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "symbol" => symbol,
        "meta" => meta,
        "admin" => admin
    };

    let package_hash = ContractPackageHash::new(
        runtime::get_key("contract_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    runtime::put_key(CASMARKET_TOKEN_KEY, contract_hash.into());
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", CASMARKET_TOKEN_KEY),
        storage::new_uref(contract_hash).into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
            Parameter::new("admin", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "meta",
        vec![],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token_by_index",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("index", U256::cl_type()),
        ],
        CLType::Option(Box::new(TokenId::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_token_meta",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_token_meta",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("token_meta_key", String::cl_type()),
            Parameter::new("token_meta_value", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_metas", CLType::List(Box::new(Meta::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_copies",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("sender", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "grant_minter",
        vec![Parameter::new("minter", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "revoke_minter",
        vec![Parameter::new("minter", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "grant_admin",
        vec![Parameter::new("admin", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "revoke_admin",
        vec![Parameter::new("admin", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_token_commission",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("value", U512::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "purchase_nft",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("store_u512"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("revoke_delegation"),
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("owner", Key::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("delagate_nft"),
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_price", CLType::U512),

        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
