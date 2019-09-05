/// # Fantom libhash-sha3
///
/// This crate is an sha3 implementation of the Hashing trait, defined in the libHash repository:
/// (https://github.com/Fantom-foundation/libhash). This hash will be used for the consensus algorithms,
/// such as the Directed Acyclic Graph (DAG) algorithm.
///
/// This crate provides access to a single struct: Hash. Hash stores a reference vector of 32 8 bit
/// unsigned integers. These values are then converted to a hash using the sha3 hashing algorithm.
use bincode;
use libhash::{errors::Error, Hashing};
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::transmute, Digest, Sha3_256};

/// The Hash type used in this hash implementation. Takes in a Vector of 8 bit unsigned integers
/// with a capacity of 32.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

/// Implementation of the Hashing trait.
impl Hashing for Hash {
    type Hash = Hash;
    /// Creates a new hash from the inputted data.
    fn hash<D: Serialize>(d: &D) -> Result<Hash, Error> {
        // Instantiate a new sha3 hash.
        let mut hasher = Sha3_256::new();
        // Serialize the sha3 data and map the data to a Vec<u8>.
        let ser =
            bincode::serialize(d).map_err(|x| Error::ComputeHashSerialize(format!("{}", x)))?;
        // Processes the input data/
        hasher.input(ser);
        // Get the hash result.
        let r = hasher.result();
        // Return the hash.
        Ok(Hash(unsafe { transmute(r) }))
    }
    /// Creates a new instance of the Hash function.
    fn new(digest: &[u8]) -> Hash {
        // Create a Vec<u8> with a capacity of 32 with all values as 0.
        let mut a: [u8; 32] = [0; 32];
        // Copy values as pointers from the Vec
        a.copy_from_slice(&digest[0..32]);
        // Return the hash with the Vec<8> reference
        Hash(a)
    }
}

/// Allows the Hash struct to return contents as ref.
impl AsRef<[u8]> for Hash {
    /// Return Vec<8> as ref.
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
