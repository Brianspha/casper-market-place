use crate::Users;
use cep47::contract_utils::Dict;

const MARKET_PLACE_USERS: &str = "marketplaceusers";

pub struct MarketPlaceUsers {
    dict: Dict,
}

impl MarketPlaceUsers {
    pub fn instance() -> MarketPlaceUsers {
        MarketPlaceUsers {
            dict: Dict::instance(MARKET_PLACE_USERS),
        }
    }

    pub fn init() {
        Dict::init(MARKET_PLACE_USERS)
    }

    pub fn get(&self, key: &str) -> Option<MarketPlaceUsers> {
        self.dict.get(key)
    }

    pub fn set(&self, key: &str, value: MarketPlaceUsers) {
        self.dict.set(key, value);
    }

    pub fn remove(&self, key: &str) {
        self.dict.remove::<MarketPlaceUsers>(key);
    }
}
