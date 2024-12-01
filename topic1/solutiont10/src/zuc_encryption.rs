use base64::prelude::*;
use zuc::cipher::{array::Array, typenum::Unsigned, BlockSizeUser, Iv, Key};
use zuc::prelude::*;
use zuc::ZucCore;

const B: usize = <ZucCore as BlockSizeUser>::BlockSize::USIZE;
const KEY: Key<Zuc> = Array([0; 16]);
const IV: Iv<Zuc> = Array([1; 16]);

pub fn encryption(input: String) -> String {
    let mut buf = input.into_bytes();
    pkcs7_padding::<B>(&mut buf);
    let mut zuc = Zuc::new(&KEY, &IV);
    zuc.apply_keystream(&mut buf);
    BASE64_STANDARD.encode(buf)
}

fn pkcs7_padding<const B: usize>(buf: &mut Vec<u8>) {
    let padding_len = B - buf.len() % B;
    buf.extend(std::iter::repeat(padding_len as u8).take(padding_len));
}
