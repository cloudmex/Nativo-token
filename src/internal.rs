use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{AccountId, Balance, PromiseResult};

use crate::*;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
pub const MIN_TRANSFER_UNIT: u128 = 1000; // to make sibyl attacks more expensive in terms of tokens
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/jpeg;base64,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";


pub fn default_ft_metadata() -> FungibleTokenMetadata {
    FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Nativo Token".to_string(),
        symbol: "$NTV".to_string(),
        icon: Some(String::from(DATA_IMAGE_SVG_NEAR_ICON.to_string(),
        )),
        reference: Some("https://nativonft.app".into()),
        reference_hash: None,
        decimals: 24,
    }
}

impl NativoToken {
    pub fn assert_owner_calling(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "can only be called by the owner"
        );
    }

    pub fn assert_minter(&self, account_id: String) {
        assert!(self.minters.contains(&account_id), "not a minter");
    }

    //get stored metadata or default
    pub fn internal_get_ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap_or(default_ft_metadata())
    }

    pub fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        self.accounts.get(&account_id).unwrap_or(0)
    }

    pub fn mint_into(&mut self, account_id: &AccountId, amount: Balance) {
        let balance = self.internal_unwrap_balance_of(account_id);
        self.internal_update_account(&account_id, balance + amount);
        self.total_supply += amount;
    }

    pub fn internal_burn(&mut self, account_id: &AccountId, amount: u128) {
        let balance = self.internal_unwrap_balance_of(account_id);
        assert!(balance >= amount);
        self.internal_update_account(&account_id, balance - amount);
        assert!(self.total_supply >= amount);
        self.total_supply -= amount;
    }

    pub fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        assert_ne!(
            sender_id, receiver_id,
            "Sender and receiver should be different"
        );

        if self.locked_until_nano > 0 && env::block_timestamp() < self.locked_until_nano {
            panic!(
                "transfers are locked until unix timestamp {}",
                self.locked_until_nano / NANOSECONDS
            );
        }

        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount == sender_balance || amount > ONE_NEAR / MIN_TRANSFER_UNIT,
            "The amount should be at least 1/{}",
            MIN_TRANSFER_UNIT
        );

        // remove from sender
        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount <= sender_balance,
            "The account doesn't have enough balance {}",
            sender_balance
        );
        let balance_left = sender_balance - amount;
        self.internal_update_account(&sender_id, balance_left);

        // check vesting
        if self.vested_count > 0 {
            match self.vested.get(&sender_id) {
                Some(vesting) => {
                    //compute locked
                    let locked = vesting.compute_amount_locked();
                    if locked == 0 {
                        //vesting is complete. remove vesting lock
                        self.vested.remove(&sender_id);
                        self.vested_count -= 1;
                    } else if balance_left < locked {
                        panic!("Vested account, balance can not go lower than {}", locked);
                    }
                }
                None => {}
            }
        }

        // add to receiver
        let receiver_balance = self.internal_unwrap_balance_of(receiver_id);
        self.internal_update_account(&receiver_id, receiver_balance + amount);

        log!("Transfer {} from {} to {}", amount, sender_id, receiver_id);
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }

    /// Inner method to save the given account for a given account ID.
    pub fn internal_update_account(&mut self, account_id: &AccountId, balance: u128) {
        self.accounts.insert(account_id, &balance); //insert_or_update
    }

    // TODO rename
    pub fn int_ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: ValidAccountId,
        amount: U128,
    ) -> (u128, u128) {
        let sender_id: AccountId = sender_id.into();
        let receiver_id: AccountId = receiver_id.into();
        let amount: Balance = amount.into();

        // Get the unused amount from the `ft_on_transfer` call result.
        let unused_amount = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(unused_amount) = near_sdk::serde_json::from_slice::<U128>(&value) {
                    std::cmp::min(amount, unused_amount.0)
                } else {
                    amount
                }
            }
            PromiseResult::Failed => amount,
        };

        if unused_amount > 0 {
            let receiver_balance = self.accounts.get(&receiver_id).unwrap_or(0);
            if receiver_balance > 0 {
                let refund_amount = std::cmp::min(receiver_balance, unused_amount);
                self.accounts
                    .insert(&receiver_id, &(receiver_balance - refund_amount));

                if let Some(sender_balance) = self.accounts.get(&sender_id) {
                    self.accounts
                        .insert(&sender_id, &(sender_balance + refund_amount));
                    log!(
                        "Refund {} from {} to {}",
                        refund_amount,
                        receiver_id,
                        sender_id
                    );
                    return (amount - refund_amount, 0);
                } else {
                    // Sender's account was deleted, so we need to burn tokens.
                    self.total_supply -= refund_amount;
                    log!("The account of the sender was deleted");
                    return (amount, refund_amount);
                }
            }
        }
        (amount, 0)
    }
}
