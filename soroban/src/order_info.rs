use soroban_sdk::{contracttype, symbol_short, unwrap::UnwrapOptimized, Env, String, Symbol};

const ORDERINFO_KEY: Symbol = symbol_short!("ORDERINFO");

#[contracttype]
#[derive(Clone)]
pub struct TokenOrderInfo{
    pub invoiceNum: String,
    pub poNum: String,
    pub totalAmount: u32,
    pub checkSum: String,
    pub suppierName: String,
    pub buyerName: String,
}


pub struct OrderInfo {
    env: Env,
}

impl OrderInfo {
    pub fn new(env: &Env) -> OrderInfo {
        OrderInfo { env: env.clone() }
    }

    #[inline(always)]
    pub fn set_order_info(&self, orderInfo: &TokenOrderInfo) {
        self.env.storage().instance().set(&ORDERINFO_KEY, metadata);
    }

    #[inline(always)]
    pub fn get_order_info(&self) -> TokenOrderInfo {
        self.env
            .storage()
            .instance()
            .get(&ORDERINFO_KEY)
            .unwrap_optimized()
    }
}