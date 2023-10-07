// use aes::cipher::BlockEncryptMut;
// use base64::Engine;
// use block_padding::generic_array::{GenericArray, typenum::U8};
// use des::cipher::KeyInit;
// use rsmycqu::encrypt::{BASE64PURPOSE, DesEcbEnc, pad8};
//
// #[test]
// fn test_login_page_encrypt() {
//     let crypto = BASE64PURPOSE.decode("IGEOE4OMIBo=").unwrap();
//     let mut crypto_block: GenericArray::<u8, U8> = [0xff; 8].into();
//     crypto_block[..crypto.len()].copy_from_slice(&crypto);
//     let password = "abc123456".as_bytes();
//     let mut pad_password = pad8(&password);
//
//     let mut des_enc = DesEcbEnc::new_from_slice(
//         &crypto_block
//     ).unwrap();
//     des_enc.encrypt_blocks_mut(&mut pad_password);
//     let result = BASE64PURPOSE.encode(
//         pad_password.iter().fold(
//             Vec::new(),
//             |mut result, x| {
//                 result.append(&mut x.to_vec());
//                 result
//             })
//     );
//
//     assert_eq!(result, "9p5YTOsEgya0j7w0dbg/CA==")
// }
