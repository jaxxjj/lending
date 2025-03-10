use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Over borrowable amount")]
    OverBorrowableAmount,
    #[msg("Over repayment")]
    OverRepayment,
    #[msg("Health factor too high")]
    HealthFactorTooHigh,
}
