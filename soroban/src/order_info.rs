use soroban_sdk::{contracttype, symbol_short, unwrap::UnwrapOptimized, Env, String, Symbol};

const ORDERINFO_KEY: Symbol = symbol_short!("ORDERINFO");

#[contracttype]
#[derive(Clone)]
pub struct TokenOrderInfo{
    pub invoice_num: String,
    pub po_num: String,
    pub total_amount: u32,
    pub checksum: String,
    pub supplier_name: String,
    pub buyer_name: String,
}


pub struct OrderInfoUtils {
    env: Env,
}

impl OrderInfoUtils {
    pub fn new(env: &Env) -> OrderInfoUtils {
        OrderInfoUtils { env: env.clone() }
    }

    #[inline(always)]
    pub fn set_order_info(&self, order_info: &TokenOrderInfo) {
        self.env.storage().persistent().set(&ORDERINFO_KEY, order_info);
    }

    #[inline(always)]
    pub fn get_order_info(&self) -> TokenOrderInfo {
        self.env
            .storage()
            .persistent()
            .get(&ORDERINFO_KEY)
            .unwrap_optimized()
    }
}

pub fn write_order_info(env: &Env, order_info: TokenOrderInfo) {
    let util = OrderInfoUtils::new(env);
    util.set_order_info(&order_info);
}

pub fn read_invoice_num(env: &Env) -> String {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().invoice_num
}

pub fn read_total_amount(env: &Env) -> u32 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().total_amount
}