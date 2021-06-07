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
#![feature(unsafe_cell_get_mut)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate sgx_crypto_helper;

extern crate base64;
extern crate sgx_types;
extern crate sgx_urts;
extern crate hex;
extern crate bincode;
//extern crate sgx_tstd as std;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::json::Json;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::u8;
use std::mem::{drop};
use rocket::data::{Limits, ToByteUnit};

pub mod structs;
use structs::{SealedData, UnsealedData};

pub mod ecalls;
use ecalls::{seal_encrypted_data, unseal_aes_encrypt};

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

pub const SCRATCH_PAD_SIZE: usize = 60 * 1024 * 1024;// 10 * megabyte equals only 1 megabyte only.

// initilaiseing the enclave
fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

#[post("/seal_encrypted_data", format = "application/json", data = "<unsealed_data_received>")]
fn seal_encrypted_data_in_enclave(unsealed_data_received: Json<UnsealedData>, enclave: State<SgxEnclave>) -> JsonValue {
    println!("✔ Data received from NodeJS server for sealing");
    let mut cipher_text = unsealed_data_received.unsealed_data_received.to_vec();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut sealed_cipher_text: Vec<u8> = vec![0; SCRATCH_PAD_SIZE];
    
    let sealed_length_for_trimming: &mut i64 = &mut 0;
    println!("✔ Sending to Enclave..");
    unsafe {
        seal_encrypted_data(enclave.geteid(), &mut retval, cipher_text.as_mut_ptr(),
                        cipher_text.len(), sealed_cipher_text.as_mut_ptr(), sealed_cipher_text.len(), sealed_length_for_trimming)
    };
    let n_us = *sealed_length_for_trimming as usize;
    let sealed_data = &sealed_cipher_text[..n_us];

    drop(cipher_text);
    println!("✔ Received sealed data from enclave - sending to NodeJS sever..");
    json!(sealed_data)
}


#[post("/unseal_encrypted_data", format = "application/json", data = "<sealed_data_received>")]
fn unseal_and_encrypt(sealed_data_received: Json<SealedData>, enclave: State<SgxEnclave>) -> JsonValue {
    println!("✔ Sealed data received from NodeJS server");
    let mut sealed_data_received = sealed_data_received.sealed_data_received.to_vec();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut aes_encrypted_placeholder: Vec<u8> = vec![0; SCRATCH_PAD_SIZE];

    let aes_encrypted_length: &mut i64 = &mut 0;
    println!("✔ Sending to Enclave..");
    unsafe {
        unseal_aes_encrypt(enclave.geteid(), &mut retval,
                        sealed_data_received.as_mut_ptr(), sealed_data_received.len(), 
                        aes_encrypted_placeholder.as_mut_ptr(), aes_encrypted_placeholder.len(), 
                        aes_encrypted_length)
    };
    let n_us = *aes_encrypted_length as usize;
    let aes_encrypted_data = &aes_encrypted_placeholder[..n_us];

    drop(sealed_data_received);
    println!("✔ Received unsealed data - sending back to NodeJS server..");
    json!(aes_encrypted_data)
}


#[rocket::main]
async fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful - this is the enclaves ID {}!", r.geteid());
            println!("[+] The enclave is also listening for a GET request on local port 8000!");
            println!("[+] When a get request is pinged it will execute the sealing data process...");
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return
        },
    };
    println!("✔ Starting enclave server on port 9000 localhost...");

    let figment = rocket::Config::figment()
        .merge(("port", 9000))
        .merge(("limits", Limits::new().limit("json", 100.mebibytes())));

    rocket::custom(figment).mount("/", routes![seal_encrypted_data_in_enclave, unseal_and_encrypt])
    .manage(enclave)
    .launch()
    .await;

    println!("[✔ successfully closed enclave");
}
