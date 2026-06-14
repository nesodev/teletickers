use crate::error::{AppError, AppResult};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
}

impl Money {
    pub fn new(amount: Decimal) -> AppResult<Self> {
        if amount < Decimal::ZERO {
            return Err(AppError::ValidationError("El monto no puede ser negativo".to_string()));
        }

        Ok(Money { amount })
    }

    pub fn from_f64(amount: f64) -> AppResult<Self> {
        let decimal = Decimal::from_f64_retain(amount).ok_or_else(|| AppError::ValidationError("Monto inválido".to_string()))?;
        Self::new(decimal)
    }

    pub fn zero() -> Self {
        Money { amount: Decimal::ZERO }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn to_f64(&self) -> f64 {
        self.amount.to_f64().unwrap_or(0.0)
    }

    pub fn round_to_cents(&self) -> Self {
        Money { amount: self.amount.round_dp(2) }
    }

    pub fn calcular_igv(&self) -> Self {
        let igv_rate = Decimal::new(18, 2); // 18%
        let igv = self.amount * igv_rate / Decimal::new(100, 0);
        Money { amount: igv.round_dp(2) }
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money { amount: self.amount + other.amount }
    }
}

impl Sub for Money {
    type Output = AppResult<Self>;

    fn sub(self, other: Self) -> AppResult<Self> {
        if self.amount < other.amount {
            return Err(AppError::ValidationError("El resultado sería negativo".to_string()));
        }
        Ok(Money { amount: self.amount - other.amount })
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, quantity: u32) -> Self {
        Money {
            amount: self.amount * Decimal::from(quantity),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S/ {:.2}", self.amount)
    }
}
