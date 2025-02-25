use snafu::ResultExt;

use crate::{
    errors::sso::{PasswordEncryptSnafu, SSOResult},
    utils::encrypt::*,
};

pub(super) fn encrypt_password(
    page_crypto: impl AsRef<str>,
    password: impl AsRef<str>,
) -> SSOResult<String> {
    let crypto = BASE64PURPOSE
        .decode(page_crypto.as_ref())
        .context(PasswordEncryptSnafu {})?;
    let mut crypto_block: GenericArray<u8, U8> = [0xff; 8].into();
    crypto_block[..crypto.len()].copy_from_slice(&crypto);
    let mut pad_password = pad8(password.as_ref().as_bytes());

    let mut des_enc = DesEcbEnc::new(&crypto_block);
    des_enc.encrypt_blocks_mut(&mut pad_password);

    Ok(
        BASE64PURPOSE.encode(pad_password.iter().fold(Vec::new(), |mut result, x| {
            result.append(&mut x.to_vec());
            result
        })),
    )
}
