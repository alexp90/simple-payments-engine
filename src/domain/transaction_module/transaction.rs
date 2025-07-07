use crate::domain::account_module::account::AccountId;
use crate::domain::Amount;

pub type TransactionId = u32;

#[derive(Clone)]
pub enum Transaction {
    Deposit(DepositTransaction),
    DisputedDeposit(DisputedDepositTransaction),
    ChargedBackDeposit(ChargedBackDepositTransaction),
    Withdrawal(WithdrawalTransaction)
}

#[derive(Clone)]
pub struct DepositTransaction {
    id: TransactionId,
    to_account_id: AccountId,
    amount: Amount
}

impl DepositTransaction {
    pub(in crate::domain) fn new(id: TransactionId, to_account_id: AccountId, amount: Amount) -> Self {
        Self {
            id,
            to_account_id,
            amount,
        }
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn to_account_id(&self) -> AccountId {
        self.to_account_id
    }

    pub fn open_dispute(&self) -> DisputedDepositTransaction {
        DisputedDepositTransaction {
            id: self.id,
            to_account_id: self.to_account_id,
            amount: self.amount,
        }
    }
}



#[derive(Clone)]
pub struct DisputedDepositTransaction {
    id: TransactionId,
    to_account_id: AccountId,
    amount: Amount
}

impl DisputedDepositTransaction {
    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn to_account_id(&self) -> AccountId {
        self.to_account_id
    }

    pub fn resolve_dispute(&self) -> DepositTransaction {
        DepositTransaction {
            id: self.id,
            to_account_id: self.to_account_id,
            amount: self.amount,
        }
    }

    pub fn charge_back(&self) -> ChargedBackDepositTransaction {
        ChargedBackDepositTransaction {
            id: self.id,
            to_account_id: self.to_account_id,
            amount: self.amount,
        }
    }
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct ChargedBackDepositTransaction {
    id: TransactionId,
    to_account_id: AccountId,
    amount: Amount
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct WithdrawalTransaction {
    id: TransactionId,
    from_account_id: AccountId,
    amount: Amount
}

impl WithdrawalTransaction {

    pub fn new(id: TransactionId, from_account_id: AccountId, amount: Amount) -> Self {
        Self {
            id,
            from_account_id,
            amount,
        }
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }
}

impl Transaction {
    pub fn id(&self) -> TransactionId {
        match self {
            Transaction::Deposit(deposit_transaction) => deposit_transaction.id,
            Transaction::Withdrawal(withdrawal_transaction) => withdrawal_transaction.id,
            Transaction::DisputedDeposit(disputed_deposit_transaction) => disputed_deposit_transaction.id,
            Transaction::ChargedBackDeposit(charged_back_transaction) => charged_back_transaction.id,
        }
    }
}