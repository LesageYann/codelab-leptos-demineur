/// A codec that relies on `bincode` to encode data.
///
/// This does not use serde but the bincode derive macros `bincode::Encode` and `bincode::Decode` instead.
/// bincode recommends not using `serde` because of [known issues](https://docs.rs/bincode/latest/bincode/serde/index.html#known-issues).
///
/// This is only available with the **`bincode` feature** feature enabled.
///
/// If you want to use `serde` with bincode, please use the `bincode_serde` feature instead.
pub struct BincodeCodec;

impl<T: bincode::Encode> crate::Encoder<T> for BincodeCodec {
    type Error = bincode::error::EncodeError;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        bincode::encode_to_vec(val, bincode::config::standard())
    }
}

impl<T: for<'a> bincode::de::BorrowDecode<'a, ()>> crate::Decoder<T> for BincodeCodec {
    type Error = bincode::error::DecodeError;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        let (data, _bytes_read) =
            bincode::borrow_decode_from_slice(val, bincode::config::standard())?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Decoder, Encoder};

    use super::*;

    #[test]
    fn test_bincode_codec() {
        #[derive(Clone, Debug, PartialEq, bincode::Encode, bincode::Decode)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = BincodeCodec::encode(&t).unwrap();
        let dec: Test = BincodeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
