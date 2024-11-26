use base64::prelude::*;
use zuc::cipher::{BlockSizeUser, Unsigned};
use zuc::prelude::*;
use zuc::ZucCore;

pub fn encryption(input: String) -> String {
    let mut buf = input.into_bytes();
    const B: usize = <ZucCore as BlockSizeUser>::BlockSize::USIZE;
    pkcs7_padding::<B>(&mut buf);
    let mut zuc = Zuc::new(&[0; 16].into(), &[1; 16].into());
    zuc.apply_keystream(&mut buf);
    BASE64_STANDARD.encode(buf)
}

fn pkcs7_padding<const B: usize>(buf: &mut Vec<u8>) {
    let padding_len = B - buf.len() % B;
    buf.resize(buf.len() + padding_len, padding_len as u8);
}
