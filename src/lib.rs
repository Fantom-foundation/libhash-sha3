#[macro_use]
extern crate failure;
use bincode;
use core::fmt::Display;
use core::fmt::Formatter;
use libhash::{
    errors::{Error, Result},
    Hash as LibHash,
};
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::transmute, Digest, Sha3_256};
use to_vec::ToVec;

const DISPLAY_PREFIX_LEN: usize = 3;
const DISPLAY_SUFFIX_LEN: usize = 3;

// Hash type used in this implementation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Copy)]
pub struct Hash(pub [u8; 32]);

impl LibHash for Hash {
    fn new<D: Serialize>(d: &D) -> Result<Hash> {
        let mut hasher = Sha3_256::new();
        let ser =
            bincode::serialize(d).map_err(|x| Error::ComputeHashSerialize(format!("{}", x)))?;
        hasher.input(ser);
        let r = hasher.result();
        Ok(Hash(unsafe { transmute(r) }))
    }
    fn new_from_digest(digest: &[u8]) -> Hash {
        let mut a: [u8; 32] = [0; 32];
        a.copy_from_slice(&digest[0..32]);
        Hash(a)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl ToVec<u8> for Hash {
    fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash { 0: [0; 32] }
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let mut formatted = String::new();
        for num in 1..DISPLAY_PREFIX_LEN {
            formatted.push_str(&format!("{:02X}", self.0[num - 1]));
        }
        formatted.push_str("::");
        for num in (1..DISPLAY_SUFFIX_LEN).rev() {
            formatted.push_str(&format!("{:02X}", self.0[32 - num]));
        }
        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
