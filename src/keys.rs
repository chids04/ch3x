use std::error;

use openssl::bn::BigNumContext;
use openssl::error::ErrorStack;
use openssl::hash::hash;
use openssl::nid::Nid;
use openssl::ec::{EcGroup, EcKey, EcPoint, PointConversionForm};
use openssl::pkey::Private;
use openssl::sha::sha256;
use openssl::base64;
use crate::utils::Hash32;

pub struct Wallet {
    priv_key: EcKey<Private>,
    pub pub_key: EcPoint
}

impl Wallet {
    pub fn new() -> Result<Self, ErrorStack> {
        let nid = Nid::X9_62_PRIME256V1; 
        let group = EcGroup::from_curve_name(nid)?;
        let priv_key = EcKey::generate(&group)?;

        let pub_key = priv_key.public_key().to_owned(&group)?;

        return Ok(
            Wallet{
                pub_key,
                priv_key
            }
        )
    }
    
    pub fn get_addr(&self) -> Result<String, ErrorStack>{
        let nid = Nid::X9_62_PRIME256V1; 
        let group = EcGroup::from_curve_name(nid)?;
        let mut ctx = BigNumContext::new()?;

        //convert ec point to bytes
        let key_bytes = self.pub_key.to_bytes(
            &group, 
            PointConversionForm::COMPRESSED,
            &mut ctx)?;
        
        //hash the bytes
        let hash_pub = sha256(&key_bytes);

        //return hex encoded string
        Ok(hex::encode(&hash_pub))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test that a new wallet can be created without error
    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert!(wallet.is_ok());
    }

    // test that get_addr returns a valid base64 string
    #[test]
    fn test_wallet_address_generation() {
        let wallet = Wallet::new().unwrap();
        let addr = wallet.get_addr();
        assert!(addr.is_ok());
        let addr_str = addr.unwrap();


        //hex encoded pub key is 64 chars long (not including 0x)
        assert_eq!(addr_str.len(), 64);
    }

    // test that two wallets have different addresses
    #[test]
    fn test_wallets_have_unique_addresses() {
        let wallet1 = Wallet::new().unwrap();
        let wallet2 = Wallet::new().unwrap();
        let addr1 = wallet1.get_addr().unwrap();
        let addr2 = wallet2.get_addr().unwrap();
        assert_ne!(addr1, addr2);
    }

    // test that get_addr does not panic on repeated calls
    #[test]
    fn test_get_addr_multiple_calls() {
        let wallet = Wallet::new().unwrap();
        let addr1 = wallet.get_addr().unwrap();
        let addr2 = wallet.get_addr().unwrap();
        assert_eq!(addr1, addr2);
    }
}