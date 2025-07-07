use log::error;
use crate::domain::Amount;
use crate::domain::transaction_module::transaction::{DepositTransaction, WithdrawalTransaction};

pub type AccountId = u16;

#[derive(Clone)]
pub enum Account {
    Active ( ActiveAccount ),
    Frozen ( FrozenAccount )
}

impl Account {
    pub fn id(&self) -> AccountId {
        match self {
            Account::Active(active_account) => active_account.id,
            Account::Frozen(frozen_account) => frozen_account.id,
        }
    }

    pub fn available_amount(&self) -> Amount {
        match self {
            Account::Active(active_account) => active_account.available_amount,
            Account::Frozen(frozen_account) => frozen_account.available_amount,
        }
    }

    pub fn held_amount(&self) -> Amount {
        match self {
            Account::Active(active_account) => active_account.held_amount,
            Account::Frozen(frozen_account) => frozen_account.held_amount,
        }
    }

    pub fn total_amount(&self) -> Amount {
        match self {
            Account::Active(active_account) => active_account.total_amount(),
            Account::Frozen(frozen_account) => frozen_account.total_amount(),
        }
    }
}


#[derive(Clone)]
pub struct ActiveAccount {
    id: AccountId,
    available_amount: Amount,
    held_amount: Amount,
}
impl ActiveAccount {

    pub(in crate::domain) fn new(account_id: AccountId) -> Self {
        Self {
            id: account_id,
            available_amount: Amount::ZERO,
            held_amount: Amount::ZERO,
        }
    }

    pub fn id(&self) -> AccountId{
        self.id
    }

    pub fn total_amount(&self) -> Amount {
        self.available_amount + self.held_amount
    }

    pub fn deposit(&self, transaction: &DepositTransaction) -> ActiveAccount {
        ActiveAccount {
            id: self.id,
            available_amount: self.available_amount + transaction.amount(),
            held_amount: self.held_amount
        }
    }

    pub fn withdraw(&self, transaction: &WithdrawalTransaction) -> Result<ActiveAccount, ActiveAccount> {
        let amount_after_withdraw = self.available_amount - transaction.amount();

        if amount_after_withdraw.is_sign_negative() {
            error!("Impossible to withdraw amount {} from account_module {} - not enough balance: {}", transaction.amount(), self.id, self.available_amount);
            Err(self.clone())
        } else {
            Ok(ActiveAccount {
                id: self.id,
                available_amount: amount_after_withdraw,
                held_amount: self.held_amount
            })
        }
    }

    pub fn hold_amount(&self, amount: Amount) -> ActiveAccount {
        ActiveAccount {
            id: self.id,
            available_amount: self.available_amount - amount,
            held_amount: self.held_amount + amount
        }
    }

    pub fn release_held_amount(&self, amount: Amount) -> ActiveAccount {
        ActiveAccount {
            id: self.id,
            available_amount: self.available_amount + amount,
            held_amount: self.held_amount - amount
        }
    }

    pub fn charge_back_amount(&self, amount: Amount) -> FrozenAccount {
        FrozenAccount {
            id: self.id,
            available_amount: self.available_amount,
            held_amount: self.held_amount - amount
        }
    }
}

#[derive(Clone)]
pub struct FrozenAccount {
    id: AccountId,
    available_amount: Amount,
    held_amount: Amount,
}

impl FrozenAccount {
    pub fn total_amount(&self) -> Amount {
        self.available_amount + self.held_amount
    }
}