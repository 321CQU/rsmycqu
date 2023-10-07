use aes::Aes128;
use base64::engine::{general_purpose, GeneralPurpose};
use block_padding::{Padding, Pkcs7};
use des::Des;

pub(crate) use base64::Engine;
pub(crate) use block_padding::generic_array::{
    typenum::{U16, U8},
    GenericArray,
};
pub(crate) use crypto::cipher::*;

pub(crate) const BASE64PURPOSE: GeneralPurpose = general_purpose::STANDARD;

pub(crate) type DesEcbEnc = ecb::Encryptor<Des>;
#[allow(dead_code)]
pub(crate) type Aes128EcbEnc = ecb::Encryptor<Aes128>;
#[allow(dead_code)]
pub(crate) type Aes128CbcEnc = cbc::Encryptor<Aes128>;

enum DataPaddingBlockSize {
    U8,
    #[allow(dead_code)]
    U16,
}

fn pad(data_to_pad: &[u8], padding_block_size: DataPaddingBlockSize) -> Vec<u8> {
    let data_len = data_to_pad.len();
    let block_size = match padding_block_size {
        DataPaddingBlockSize::U8 => 8,
        DataPaddingBlockSize::U16 => 16,
    };

    let redundant_num = data_len % block_size;
    let padding_start_pos = data_len - redundant_num;
    let mut result = data_to_pad[..padding_start_pos].to_vec();

    match padding_block_size {
        DataPaddingBlockSize::U8 => {
            let mut block: GenericArray<u8, U8> = [0xff; 8].into();
            block[..redundant_num].copy_from_slice(&data_to_pad[padding_start_pos..]);
            Pkcs7::pad(&mut block, redundant_num);
            result.append(&mut block.to_vec())
        }
        DataPaddingBlockSize::U16 => {
            let mut block: GenericArray<u8, U16> = [0xff; 16].into();
            block[..redundant_num].copy_from_slice(&data_to_pad[padding_start_pos..]);
            Pkcs7::pad(&mut block, redundant_num);
            result.append(&mut block.to_vec())
        }
    }

    result
}

pub(crate) fn pad8(data_to_pad: &[u8]) -> Vec<GenericArray<u8, U8>> {
    let padded_data = pad(data_to_pad, DataPaddingBlockSize::U8);
    assert_eq!(padded_data.len() % 8, 0);
    let mut result = Vec::new();
    for i in 0..(padded_data.len() / 8) {
        let mut array: GenericArray<u8, U8> = [0; 8].into();
        array.copy_from_slice(&padded_data[(i * 8)..(i * 8 + 8)]);
        result.push(array);
    }

    result
}

#[allow(dead_code)]
pub(crate) fn pad16(data_to_pad: &[u8]) -> Vec<GenericArray<u8, U16>> {
    let padded_data = pad(data_to_pad, DataPaddingBlockSize::U16);
    assert_eq!(padded_data.len() % 16, 0);
    let mut result = Vec::new();
    for i in 0..(padded_data.len() / 16) {
        let mut array: GenericArray<u8, U16> = [0; 16].into();
        array.copy_from_slice(&padded_data[(i * 16)..(i * 16 + 16)]);
        result.push(array);
    }

    result
}
