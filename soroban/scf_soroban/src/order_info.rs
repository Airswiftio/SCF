use soroban_sdk::{contracttype, symbol_short, unwrap::UnwrapOptimized, Env, String, Symbol};

const ORDERINFO_KEY: Symbol = symbol_short!("ORDERINFO");

#[contracttype]
#[derive(Clone)]
pub struct TokenOrderInfo {
    pub invoice_num: i128,
    pub po_num: i128,
    pub total_amount: u32,
    pub checksum: String,
    pub supplier_name: String,
    pub buyer_name: String,
    pub start_time: u64,
    pub end_time: u64,
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
        self.env
            .storage()
            .instance()
            .set(&ORDERINFO_KEY, order_info);
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

pub fn write_order_info(
    env: &Env,
    invoice_num: i128,
    po_num: i128,
    total_amount: u32,
    checksum: String,
    supplier_name: String,
    buyer_name: String,
    start_time: u64,
    end_time: u64,
) {
    let util = OrderInfoUtils::new(env);
    let order_info = TokenOrderInfo {
        invoice_num,
        po_num,
        total_amount,
        checksum,
        supplier_name,
        buyer_name,
        start_time,
        end_time,
    };
    util.set_order_info(&order_info);
}

pub fn read_invoice_num(env: &Env) -> i128 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().invoice_num
}

pub fn read_total_amount(env: &Env) -> u32 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().total_amount
}

pub fn read_end_time(env: &Env) -> u64 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().end_time
}
