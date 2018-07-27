use std::os::raw::{c_int, c_char};

pub enum botan_cipher_struct {}
pub type botan_cipher_t = *mut botan_cipher_struct;

extern "C" {
    pub fn botan_cipher_init(
        cipher: *mut botan_cipher_t,
        name: *const c_char,
        flags: u32,
    ) -> c_int;
    pub fn botan_cipher_valid_nonce_length(
        cipher: botan_cipher_t,
        nl: usize,
    ) -> c_int;
    pub fn botan_cipher_get_tag_length(
        cipher: botan_cipher_t,
        tag_size: *mut usize,
    ) -> c_int;
    pub fn botan_cipher_get_default_nonce_length(
        cipher: botan_cipher_t,
        nl: *mut usize,
    ) -> c_int;
    pub fn botan_cipher_get_update_granularity(
        cipher: botan_cipher_t,
        ug: *mut usize,
    ) -> c_int;
    pub fn botan_cipher_query_keylen(
        arg1: botan_cipher_t,
        out_minimum_keylength: *mut usize,
        out_maximum_keylength: *mut usize,
    ) -> c_int;
    pub fn botan_cipher_set_key(
        cipher: botan_cipher_t,
        key: *const u8,
        key_len: usize,
    ) -> c_int;
    pub fn botan_cipher_set_associated_data(
        cipher: botan_cipher_t,
        ad: *const u8,
        ad_len: usize,
    ) -> c_int;
    pub fn botan_cipher_start(
        cipher: botan_cipher_t,
        nonce: *const u8,
        nonce_len: usize,
    ) -> c_int;
    pub fn botan_cipher_update(
        cipher: botan_cipher_t,
        flags: u32,
        output: *mut u8,
        output_size: usize,
        output_written: *mut usize,
        input_bytes: *const u8,
        input_size: usize,
        input_consumed: *mut usize,
    ) -> c_int;
    pub fn botan_cipher_clear(cipher: botan_cipher_t) -> c_int;
    pub fn botan_cipher_destroy(cipher: botan_cipher_t) -> c_int;

}