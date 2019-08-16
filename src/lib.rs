use bincode;
use libhash::{errors::Error, Hashing};
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::transmute, Digest, Sha3_256};

// Hash type used in this implementation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hashing for Hash {
    type Hash = Hash;
    fn hash<D: Serialize>(d: &D) -> Result<Hash, Error> {
        let mut hasher = Sha3_256::new();
        let ser =
            bincode::serialize(d).map_err(|x| Error::ComputeHashSerialize(format!("{}", x)))?;
        hasher.input(ser);
        let r = hasher.result();
        Ok(Hash(unsafe { transmute(r) }))
    }
    fn new(digest: &[u8]) -> Hash {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
