use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[doc(hidden)]
pub use alloy_primitives::{Address, B256 as H256, U256};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockchainAddress(pub Address);
impl Debug for BlockchainAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[allow(non_snake_case)]
pub mod WithBlockchainAddress {
    use super::*;

    pub fn serialize<S>(this: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", this).serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = Address::from_str(&value)
            .ok()
            .ok_or_else(|| D::Error::invalid_value(serde::de::Unexpected::Str(&value), &"a valid address"))?;
        Ok(value)
    }
}
impl Serialize for BlockchainAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", self.0).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BlockchainAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = Address::from_str(&value)
            .ok()
            .ok_or_else(|| D::Error::invalid_value(serde::de::Unexpected::Str(&value), &"a valid address"))?;
        Ok(BlockchainAddress(value))
    }
}

impl From<Address> for BlockchainAddress {
    fn from(address: Address) -> Self {
        BlockchainAddress(address)
    }
}
impl From<BlockchainAddress> for Address {
    fn from(val: BlockchainAddress) -> Self {
        val.0
    }
}

impl Deref for BlockchainAddress {
    type Target = Address;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockchainTransactionHash(pub H256);
impl Debug for BlockchainTransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[allow(non_snake_case)]
pub mod WithBlockchainTransactionHash {
    use super::*;

    pub fn serialize<S>(this: &H256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", this).serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<H256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = H256::from_str(&value)
            .ok()
            .ok_or_else(|| D::Error::invalid_value(serde::de::Unexpected::Str(&value), &"a valid hash"))?;
        Ok(value)
    }
}
impl Serialize for BlockchainTransactionHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", self.0).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BlockchainTransactionHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = H256::from_str(&value)
            .ok()
            .ok_or_else(|| D::Error::invalid_value(serde::de::Unexpected::Str(&value), &"a valid hash"))?;
        Ok(BlockchainTransactionHash(value))
    }
}

impl From<H256> for BlockchainTransactionHash {
    fn from(hash: H256) -> Self {
        BlockchainTransactionHash(hash)
    }
}
impl From<BlockchainTransactionHash> for H256 {
    fn from(val: BlockchainTransactionHash) -> Self {
        val.0
    }
}

impl Deref for BlockchainTransactionHash {
    type Target = H256;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainTransactionHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
