use openssl::bn::BigNumContext;
use openssl::error::ErrorStack;
use openssl::ec::PointConversionForm;
use openssl::hash::MessageDigest;
use openssl::pkey::{Private, PKey, Public};
use openssl::sha::sha256;
use openssl::ecdsa::EcdsaSig;


pub struct Wallet {
    priv_key: PKey<Private>,
    pub pub_key: PKey<Public>,
    pub address: String,
}


impl Wallet {
    pub fn new() -> Result<Self, ErrorStack> {

        let priv_key: PKey<Private> = PKey::ec_gen("prime256v1")?;
        let pub_key = priv_key.public_key_to_pem()?;
        let pub_key = PKey::public_key_from_pem(&pub_key)?;

        let bytes = pub_key.public_key_to_der()?;

        return Ok(
            Wallet{
                pub_key,
                priv_key,
                address: hex::encode(bytes),
            }
        )
    }
    
    // pub fn get_addr(&self) -> Result<String, ErrorStack>{

    //     let ec_key = self.pub_key.ec_key()?;
    //     let point = ec_key.public_key();
    //     let group = ec_key.group();
    //     let mut ctx = BigNumContext::new()?;

    //     let uncompressed_pub_key_bytes = point.to_bytes(
    //         &group,
    //         PointConversionForm::COMPRESSED,
    //         &mut ctx,
    //     )?;

    //     let hash_pub = sha256(&uncompressed_pub_key_bytes);
    //     //return hex encoded string
    //     Ok(hex::encode(&hash_pub))
    // }

    pub fn sign_bytes(&self, data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let ec_key = self.priv_key.ec_key()?;
        let sig = EcdsaSig::sign(data, &ec_key)?;

        let r = sig.r().to_vec();
        let s = sig.s().to_vec();

        let mut res = Vec::new();
        res.extend_from_slice(&r);
        res.extend_from_slice(&s);

        Ok(res)
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

        // DER-encoded public keys for prime256v1 are typically 91 bytes, so hex encoding is 182 chars
        assert_eq!(wallet.address.len(), 182);
    }

    // test that two wallets have different addresses
    #[test]
    fn test_wallets_have_unique_addresses() {
        let wallet1 = Wallet::new().unwrap();
        let wallet2 = Wallet::new().unwrap();
        assert_ne!(wallet1.address, wallet2.address);
    }

    #[test]
    fn signature_creation(){
        let message = b"a random message";
        let wallet = Wallet::new().unwrap();

        let sig = wallet.sign_bytes(message).unwrap();

        //sig is 64 bytes long, 32 bytes for r and s component each
        assert_eq!(sig.len(), 64);
        
    }

}