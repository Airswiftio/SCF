use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String};

use crate::{
    admin::{has_admin, read_admin, write_admin},
    ext_token::write_ext_token,
    interface::TokenizedCertificateTrait,
    storage_types::{
        ExtTokenInfo, HashMetadata, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
    },
    token_data::{write_amount, write_metadata},
};

#[contract]
pub struct TokenizedCertificate;

#[contractimpl]
impl TokenizedCertificateTrait for TokenizedCertificate {
    fn initialize(e: Env, admin: Address, ext_token_address: Address, ext_token_decimals: u32) {
        if has_admin(&e) {
            panic!("already initialized")
        }
        write_admin(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }
        write_ext_token(
            &e,
            ExtTokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        )
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&e, &new_admin);
    }

    fn mint(e: Env, amount: u32, po_hash: String, invoice_hash: String, bol_hash: String) {
        let admin = read_admin(&e);
        admin.require_auth();

        write_amount(&e, amount);
        write_metadata(
            &e,
            HashMetadata {
                po_hash,
                invoice_hash,
                bol_hash,
            },
        );

        // TODO: emit 'minted' event
    }
}
