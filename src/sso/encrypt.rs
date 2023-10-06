use crate::utils::encrypt::*;

pub(super) fn encrypt_password(page_crypto: &str, password: &str) -> String {
    let crypto = BASE64PURPOSE.decode(page_crypto).unwrap();
    let mut crypto_block: GenericArray<u8, U8> = [0xff; 8].into();
    crypto_block[..crypto.len()].copy_from_slice(&crypto);
    let mut pad_password = pad8(password.as_bytes());

    let mut des_enc = DesEcbEnc::new_from_slice(&crypto_block).unwrap();
    des_enc.encrypt_blocks_mut(&mut pad_password);

    BASE64PURPOSE.encode(pad_password.iter().fold(Vec::new(), |mut result, x| {
        result.append(&mut x.to_vec());
        result
    }))
}
