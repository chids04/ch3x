use std::fmt;
use std::error::Error;
use openssl::ec::EcKey;
use openssl::ecdsa::EcdsaSig;
use openssl::bn::BigNum;

use crate::block::Block;
use crate::keys::Wallet;

#[derive(Debug)]
pub struct Blockchain{
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    //add pending transactions array
}


#[derive(Debug)] 
pub enum BcError {
    InvalidSignature, 
    HexDecodingError(hex::FromHexError), 
    OpenSslError(openssl::error::ErrorStack), 
    KeyDerivationError(String), 
    BlockNotFound, 
}


impl fmt::Display for BcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BcError::InvalidSignature => {
                write!(f, "signature verification failed")
            }
            BcError::HexDecodingError(err) => {
                write!(f, "hex decoding error: {}", err)
            }
            BcError::OpenSslError(err) => {
                write!(f, "openssl error: {}", err)
            }
            BcError::KeyDerivationError(msg) => {
                write!(f, "key derivation error: {}", msg)
            }
            BcError::BlockNotFound => {
                write!(f, "requested block not found")
            }
        }
    }
}

impl Error for BcError {}

impl From<hex::FromHexError> for BcError {
    fn from(err: hex::FromHexError) -> Self {
        BcError::HexDecodingError(err)
    }
}

impl From<openssl::error::ErrorStack> for BcError {
    fn from(err: openssl::error::ErrorStack) -> Self {
        BcError::OpenSslError(err)
    }
}



impl Blockchain {
    pub fn new() -> Self{
        Blockchain { blocks: vec![Block::mine_genesis()], difficulty: 1 }
    }

    pub fn prev_block(&self) -> &Block {
        if let Some(block) = self.blocks.last() {
            return block;
        }
        else{
            panic!("missing previous block");
        }
    }

    pub fn add_block(&mut self, block: Block){ 
        self.blocks.push(block)
    }

    pub fn genesis(&self) -> Option<&Block> {
        self.blocks.first()
    }

    pub fn head(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn verify_sig(&self, raw_sig: &[u8], data: &[u8], pub_key: &str) -> Result<bool, BcError>{
        //pub key is der encoded then hex encoded
        if raw_sig.len() < 64 {
            return Err(BcError::InvalidSignature);
        }

        let decoded_der = hex::decode(pub_key)?;
        let pub_key = EcKey::public_key_from_der(&decoded_der)?;

        let r = BigNum::from_slice(&raw_sig[0..32]).map_err(BcError::OpenSslError)?;
        let s = BigNum::from_slice(&raw_sig[32..64]).map_err(BcError::OpenSslError)?;
        
        let sig = EcdsaSig::from_private_components(r, s)?;
        
        let valid = sig.verify(data, &pub_key)?;

        Ok(valid)
        
    }
}

#[cfg(test)]
mod test{
    use super::*;

    // test that the blockchain is initialized with one block (the genesis block)
    #[test]
    fn genesis_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 1);
    }

    // test that the difficulty is set correctly on initialization
    #[test]
    fn difficulty_is_set() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.difficulty, 1);
    }

    // test that the genesis block is the default block
    #[test]
    fn genesis_block_is_default() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks[0], Block::mine_genesis());
    }
    
    #[test]
    fn verify_sig(){
        let bc = Blockchain::new();
        let msg = b"hello world";
        let wallet = Wallet::new().unwrap();

        let sig = wallet.sign_bytes(msg).unwrap();
        
        let verified = bc.verify_sig(&sig, msg, &wallet.address).unwrap();

        assert_eq!(true, verified);

        let bad_msg = b"not hello world";
        let not_verified = bc.verify_sig(&sig, bad_msg, &wallet.address).unwrap();

        assert_eq!(false, not_verified);
    }

}

