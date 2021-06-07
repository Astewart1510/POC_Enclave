
use sgx_types::sgx_enclave_id_t;
use sgx_types::sgx_status_t;

extern {

    pub fn seal_encrypted_data(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
         cipher_text: *mut u8, cipher_text_len: usize,
         sealed_cipher_text: *mut u8,sealed_cipher_text_len: usize, 
         sealed_length_for_trimming: *mut i64
    ) -> sgx_status_t;

     pub fn unseal_aes_encrypt(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
         sealed_data_received: *mut u8, sealed_data_received: usize, 
         aes_encrypted_placeholder: *mut u8,aes_encrypted_placeholder: usize, 
         aes_encrypted_length: *mut i64
    ) -> sgx_status_t;
}