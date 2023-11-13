//! 提供加密相关函数

use hex::encode_upper;

use crate::utils::encrypt::*;

/// [`Exam`] 中 `fetch_all` API需要加密学号
pub(in crate::mycqu) fn encrypt_student_id(student_id: impl AsRef<str>) -> String {
    let crypto_block: &GenericArray<u8, U16> = "cquisse123456789".as_bytes().into();

    let mut pad_student_id = pad16(student_id.as_ref().as_bytes());

    let mut encryptor = Aes128EcbEnc::new(crypto_block);
    encryptor.encrypt_blocks_mut(&mut pad_student_id);

    encode_upper(pad_student_id.iter().fold(Vec::new(), |mut result, x| {
        result.append(&mut x.to_vec());
        result
    }))
}
