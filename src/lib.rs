use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;

use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{near_bindgen, AccountId, env, Balance, log, PanicOnDefault, PromiseOrValue};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FTContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

// const TOTAL_SUPPLY: Balance = 10_000;

#[near_bindgen]
impl FTContract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Test NEAR Fungible Token".to_string(),
                symbol: "TEST".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 6,
            }
        )
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already Initilized");
        metadata.assert_valid();

        let mut this = Self {
            token: FungibleToken::new(b"FT".to_vec()),
            metadata: LazyOption::new(b"Test FT".to_vec(), Some(&metadata)),
        };

        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply minted")
        }
        .emit();

        this
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    fn on_account_closed(&mut self, account_id: AccountId, balacne: Balance) {
        log!("Closed @{} with {}", account_id, balacne);
    }

    pub fn give_random(&mut self, receipent: AccountId) -> u128 {
        let random_seed: Vec<u8> = env::random_seed();
        let random_amount = (random_seed[0] * 255 + random_seed[1]) as u128 % 10_000;
        // let random_amount: u128 = 100;
        self.token.internal_deposit(&receipent, random_amount.into());
        random_amount
    }
}

near_contract_standards::impl_fungible_token_core!(FTContract, token, on_tokens_burned);

near_contract_standards::impl_fungible_token_storage!(FTContract, token, on_account_closed);

