// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..
#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#[cfg(not(target_env = "sgx"))]

#[macro_use] extern crate sgx_tstd as std;

extern crate serde_derive;
extern crate sgx_types;
extern crate sgx_tseal;
extern crate serde_cbor;
extern crate sgx_crypto_helper;
extern crate sgx_tcrypto;
extern crate serde_json;

use sgx_types::*;
use sgx_tseal::SgxSealedData;
use sgx_types::marker::ContiguousMemory;

use std::vec::Vec;
use std::slice;
use std::ptr;
use std::{
    u32,
    mem::size_of,
};
use core::convert::TryInto;

pub const SCRATCH_PAD_SIZE: usize = 60 * 1024 * 1024;

pub type Bytes = Vec<u8>;

#[no_mangle]
pub extern "C" fn unseal_aes_encrypt(sealed_data_received: *mut u8, sealed_data_received_len: usize, aes_encrypted_placeholder: *mut u8, _aes_encrypted_placeholder_len: usize, aes_encrypted_length: &mut i64) -> sgx_status_t {
    println!("[+] Sealed data passsed into enclave");

    let sealed_data_received = unsafe { slice::from_raw_parts_mut(sealed_data_received, sealed_data_received_len) };
    let sealed_data_received_pointer: *mut u8 = &mut sealed_data_received[0];
 
    let maybe_sealed_data = from_sealed_log_for_slice::<u8>(
        sealed_data_received_pointer,
        sealed_data_received.len() as u32
    );
    println!("[+] Begin unsealing process..");
      let sealed_data = match maybe_sealed_data {
        Some(sealed_data) => sealed_data,
        None => return 
            sgx_status_t::SGX_ERROR_INVALID_PARAMETER
        };
    
    let unsealed_data = match sealed_data.unseal_data() {
        Ok(unsealed_data) => unsealed_data,
        Err(e) => return e
    };
    println!("[+] Successfuly unsealed");
    let cbor_encoded_slice = unsealed_data.get_decrypt_txt();
    let final_unsealed_data = &cbor_encoded_slice[..];
    println!("[+] Unsealed data parsed into bytes");
    unsafe{
            ptr::copy_nonoverlapping(final_unsealed_data.as_ptr(),
                                    aes_encrypted_placeholder,
                                    final_unsealed_data.len());
            }
    *aes_encrypted_length = final_unsealed_data.len().try_into().unwrap();
   
    drop(sealed_data_received);
    drop(cbor_encoded_slice);
    drop(final_unsealed_data);
    drop(unsealed_data);
    println!("[+] Sealed data sent to untrusted");

    sgx_status_t::SGX_SUCCESS
}



#[no_mangle]
pub extern "C" fn seal_encrypted_data(cipher_text: *const u8, cipher_text_len: usize, sealed_cipher_text: *mut u8, _sealed_cipher_text_len: usize, sealed_length_for_trimming: &mut i64) -> sgx_status_t {
    println!("[+] Encrypted data passsed into enclave");

    let cipher_text_slice = unsafe { slice::from_raw_parts(cipher_text, cipher_text_len) };
    // perform length check
    if cipher_text_slice.len() != cipher_text_len {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    let value = cipher_text_slice.to_vec();
    let extra_data: [u8; 0] = [0u8; 0]; 

    println!("[+] Begin sealing process..");
    let sealing_result = SgxSealedData::<[u8]>::seal_data(
        &extra_data,
        &value,
    );
    let sealed_data = match sealing_result {
        Ok(ref sealed_data) => sealed_data,
        Err(sgx_error) => return sgx_error
    };
    println!("[+] Successfuly sealed");

    let mut sealedcipher_vec: Vec<u8> = vec![0; SCRATCH_PAD_SIZE];
    let sealedcipher_vec_pointer: *mut u8 = &mut sealedcipher_vec[0];

    let sealed_log_size = size_of::<sgx_sealed_data_t>() + value.len();

    let _option = to_sealed_log_for_slice(&sealed_data, sealedcipher_vec_pointer,sealed_log_size as u32);

    let sealed_encrypted_data = unsafe {
        slice::from_raw_parts(sealedcipher_vec_pointer, sealed_log_size as usize)
        };
   println!("[+] Sealed data parsed into bytes");
    unsafe{
                ptr::copy_nonoverlapping(sealed_encrypted_data.as_ptr(),
                                         sealed_cipher_text,
                                         sealed_encrypted_data.len());
            }
    *sealed_length_for_trimming = sealed_encrypted_data.len().try_into().unwrap();
    
    drop(cipher_text_slice);
    drop(sealed_data);
    drop(sealing_result );
    drop(value);
    drop(sealedcipher_vec);
    drop(sealed_encrypted_data);
    println!("[+] Sealed data sent to untrusted");
    sgx_status_t::SGX_SUCCESS
}



fn to_sealed_log_for_slice<T: Copy + ContiguousMemory>(
    sealed_data: &SgxSealedData<[T]>,
    sealed_log: * mut u8,
    sealed_log_size: u32
) -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data
            .to_raw_sealed_data_t(
                sealed_log as * mut sgx_sealed_data_t,
                sealed_log_size
            )
    }
}

fn from_sealed_log_for_slice<'a, T: Copy + ContiguousMemory>(
    sealed_log: * mut u8,
    sealed_log_size: u32
) -> Option<SgxSealedData<'a, [T]>> {
    unsafe {
        SgxSealedData::<[T]>::from_raw_sealed_data_t(
            sealed_log as * mut sgx_sealed_data_t,
            sealed_log_size
        )
    }
}

