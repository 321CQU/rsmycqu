pub(crate) use ::block_padding::array::{Array, typenum::U16};
use ::block_padding::{Padding, Pkcs7};
use aes::Aes128;
pub(crate) use base64::Engine;
use base64::engine::{GeneralPurpose, general_purpose};
pub(crate) use ecb::cipher::*;

#[allow(deprecated)]
use crate::utils::encrypt::array::Array as GenericArray;

pub(crate) const BASE64PURPOSE: GeneralPurpose = general_purpose::STANDARD;

pub(crate) type Aes128EcbEnc = ecb::Encryptor<Aes128>;

fn pad(data_to_pad: &[u8]) -> Vec<u8> {
    let data_len = data_to_pad.len();
    let block_size = 16;

    let redundant_num = data_len % block_size;
    let padding_start_pos = data_len - redundant_num;
    let mut result = data_to_pad[..padding_start_pos].to_vec();

    let mut block: Array<u8, U16> = [0xff; 16].into();
    block[..redundant_num].copy_from_slice(&data_to_pad[padding_start_pos..]);
    Pkcs7::pad(&mut block, redundant_num);
    result.append(&mut block.to_vec());

    result
}

#[allow(deprecated)]
pub(crate) fn pad16(data_to_pad: &[u8]) -> Vec<GenericArray<u8, U16>> {
    let padded_data = pad(data_to_pad);
    assert_eq!(padded_data.len() % 16, 0);
    let mut result = Vec::new();
    for i in 0..(padded_data.len() / 16) {
        let mut array: GenericArray<u8, U16> = [0; 16].into();
        array.copy_from_slice(&padded_data[(i * 16)..(i * 16 + 16)]);
        result.push(array);
    }

    result
}
