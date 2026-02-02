use num_bigint::BigInt;
use num_traits::{Zero, One};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigIntAmount(BigInt);

impl BigIntAmount {
    pub fn zero() -> Self {
        Self(BigInt::zero())
    }

    pub fn one() -> Self {
        Self(BigInt::one())
    }

    pub fn from_str(v: &str) -> Self {
        Self(v.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero()))
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn is_negative(&self) -> bool {
        self.0.sign() == num_bigint::Sign::Minus
    }

    pub fn saturating_sub(&self, other: &Self) -> Self {
        if &self.0 < &other.0 {
            Self(BigInt::zero())
        } else {
            Self(&self.0 - &other.0)
        }
    }

    pub fn saturating_add(&self, other: &Self) -> Self {
        Self(&self.0 + &other.0)
    }

    pub fn inner(&self) -> &BigInt {
        &self.0
    }
}

impl Add for BigIntAmount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BigIntAmount {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for BigIntAmount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for BigIntAmount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Serialize for BigIntAmount {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for BigIntAmount {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(Self::from_str(&s))
    }
}

impl fmt::Debug for BigIntAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}
