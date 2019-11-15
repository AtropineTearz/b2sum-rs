use blake2_rfc::blake2b::Blake2b;
use filebuffer::FileBuffer;
use std::path::Path;

/// ## Blake2b File Hash Constructor
/// 
/// This is the official constructor used to call the new() function which returns a 64 byte blake2b digest of a file.
/// 
/// ## Example
/// 
/// ```
/// // Imports The Crate
/// use b2sum::Blake2bSum;
/// 
/// // Returns as String
/// let hex_output = Blake2bSum::new("file.txt");
/// 
/// // Converts To Byte Vector From Hexadecimal
/// let bytes = Blake2bSum::to_bytes(hex_output);
/// 
/// // Prints Hexadecimal Output
/// println!("{}",hex_output);
/// 
/// ```
pub struct Blake2bSum;

impl Blake2bSum {
    /// ## Hash File
    /// This is a function that hashes a file using **Blake2b** and returns the **Hexadecimal Representation** of it as a String. It takes as input any reference to Path.
    /// 
    /// It should be noted that changes to the file during hashing, such as truncating the file can cause problems.
    pub fn new<T: AsRef<Path>>(path: T) -> String {

        // Opens File Using File Buffer
        let fbuffer = FileBuffer::open(path).expect("failed to open file");
        
        // Sets Blake2b Context at a digest size of 512 bits (64 bytes)
        let mut context = Blake2b::new(64);
        context.update(&fbuffer);
        let hash = context.finalize();
        
        // Return as Hexadecimal Encoded String
        return hex::encode_upper(hash.as_bytes());
    }
    pub fn to_bytes(s: String) -> Vec<u8> {
        return hex::decode(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
