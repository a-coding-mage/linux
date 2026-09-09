/* BSD license and compatibility definitions translated from virtio_crypto.h. */

pub const VIRTIO_CRYPTO_SERVICE_CIPHER: u32 = 0;
pub const VIRTIO_CRYPTO_SERVICE_HASH: u32 = 1;
pub const VIRTIO_CRYPTO_SERVICE_MAC: u32 = 2;
pub const VIRTIO_CRYPTO_SERVICE_AEAD: u32 = 3;
pub const VIRTIO_CRYPTO_SERVICE_AKCIPHER: u32 = 4;

pub const fn VIRTIO_CRYPTO_OPCODE(service: u32, op: u32) -> u32 { (service << 8) | op }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_ctrl_header { pub opcode: __le32, pub algo: __le32, pub flag: __le32, pub queue_id: __le32 }
pub const VIRTIO_CRYPTO_CIPHER_CREATE_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_CIPHER, 0x02);
pub const VIRTIO_CRYPTO_CIPHER_DESTROY_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_CIPHER, 0x03);
pub const VIRTIO_CRYPTO_HASH_CREATE_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_HASH, 0x02);
pub const VIRTIO_CRYPTO_HASH_DESTROY_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_HASH, 0x03);
pub const VIRTIO_CRYPTO_MAC_CREATE_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_MAC, 0x02);
pub const VIRTIO_CRYPTO_MAC_DESTROY_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_MAC, 0x03);
pub const VIRTIO_CRYPTO_AEAD_CREATE_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AEAD, 0x02);
pub const VIRTIO_CRYPTO_AEAD_DESTROY_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AEAD, 0x03);
pub const VIRTIO_CRYPTO_AKCIPHER_CREATE_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 0x04);
pub const VIRTIO_CRYPTO_AKCIPHER_DESTROY_SESSION: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 0x05);

pub const VIRTIO_CRYPTO_NO_CIPHER: u32 = 0; pub const VIRTIO_CRYPTO_CIPHER_ARC4: u32 = 1; pub const VIRTIO_CRYPTO_CIPHER_AES_ECB: u32 = 2; pub const VIRTIO_CRYPTO_CIPHER_AES_CBC: u32 = 3; pub const VIRTIO_CRYPTO_CIPHER_AES_CTR: u32 = 4; pub const VIRTIO_CRYPTO_CIPHER_DES_ECB: u32 = 5; pub const VIRTIO_CRYPTO_CIPHER_DES_CBC: u32 = 6; pub const VIRTIO_CRYPTO_CIPHER_3DES_ECB: u32 = 7; pub const VIRTIO_CRYPTO_CIPHER_3DES_CBC: u32 = 8; pub const VIRTIO_CRYPTO_CIPHER_3DES_CTR: u32 = 9; pub const VIRTIO_CRYPTO_CIPHER_KASUMI_F8: u32 = 10; pub const VIRTIO_CRYPTO_CIPHER_SNOW3G_UEA2: u32 = 11; pub const VIRTIO_CRYPTO_CIPHER_AES_F8: u32 = 12; pub const VIRTIO_CRYPTO_CIPHER_AES_XTS: u32 = 13; pub const VIRTIO_CRYPTO_CIPHER_ZUC_EEA3: u32 = 14;
pub const VIRTIO_CRYPTO_OP_ENCRYPT: u32 = 1; pub const VIRTIO_CRYPTO_OP_DECRYPT: u32 = 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_cipher_session_para { pub algo: __le32, pub keylen: __le32, pub op: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_session_input { pub session_id: __le64, pub status: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_cipher_session_req { pub para: virtio_crypto_cipher_session_para, pub padding: [__u8; 32] }

pub const VIRTIO_CRYPTO_NO_HASH: u32 = 0; pub const VIRTIO_CRYPTO_HASH_MD5: u32 = 1; pub const VIRTIO_CRYPTO_HASH_SHA1: u32 = 2; pub const VIRTIO_CRYPTO_HASH_SHA_224: u32 = 3; pub const VIRTIO_CRYPTO_HASH_SHA_256: u32 = 4; pub const VIRTIO_CRYPTO_HASH_SHA_384: u32 = 5; pub const VIRTIO_CRYPTO_HASH_SHA_512: u32 = 6; pub const VIRTIO_CRYPTO_HASH_SHA3_224: u32 = 7; pub const VIRTIO_CRYPTO_HASH_SHA3_256: u32 = 8; pub const VIRTIO_CRYPTO_HASH_SHA3_384: u32 = 9; pub const VIRTIO_CRYPTO_HASH_SHA3_512: u32 = 10; pub const VIRTIO_CRYPTO_HASH_SHA3_SHAKE128: u32 = 11; pub const VIRTIO_CRYPTO_HASH_SHA3_SHAKE256: u32 = 12;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_hash_session_para { pub algo: __le32, pub hash_result_len: __le32, pub padding: [__u8; 8] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_hash_create_session_req { pub para: virtio_crypto_hash_session_para, pub padding: [__u8; 40] }

pub const VIRTIO_CRYPTO_NO_MAC: u32 = 0; pub const VIRTIO_CRYPTO_MAC_HMAC_MD5: u32 = 1; pub const VIRTIO_CRYPTO_MAC_HMAC_SHA1: u32 = 2; pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_224: u32 = 3; pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_256: u32 = 4; pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_384: u32 = 5; pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_512: u32 = 6; pub const VIRTIO_CRYPTO_MAC_CMAC_3DES: u32 = 25; pub const VIRTIO_CRYPTO_MAC_CMAC_AES: u32 = 26; pub const VIRTIO_CRYPTO_MAC_KASUMI_F9: u32 = 27; pub const VIRTIO_CRYPTO_MAC_SNOW3G_UIA2: u32 = 28; pub const VIRTIO_CRYPTO_MAC_GMAC_AES: u32 = 41; pub const VIRTIO_CRYPTO_MAC_GMAC_TWOFISH: u32 = 42; pub const VIRTIO_CRYPTO_MAC_CBCMAC_AES: u32 = 49; pub const VIRTIO_CRYPTO_MAC_CBCMAC_KASUMI_F9: u32 = 50; pub const VIRTIO_CRYPTO_MAC_XCBC_AES: u32 = 53;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_mac_session_para { pub algo: __le32, pub hash_result_len: __le32, pub auth_key_len: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_mac_create_session_req { pub para: virtio_crypto_mac_session_para, pub padding: [__u8; 40] }

pub const VIRTIO_CRYPTO_NO_AEAD: u32 = 0; pub const VIRTIO_CRYPTO_AEAD_GCM: u32 = 1; pub const VIRTIO_CRYPTO_AEAD_CCM: u32 = 2; pub const VIRTIO_CRYPTO_AEAD_CHACHA20_POLY1305: u32 = 3;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_aead_session_para { pub algo: __le32, pub key_len: __le32, pub hash_result_len: __le32, pub aad_len: __le32, pub op: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_aead_create_session_req { pub para: virtio_crypto_aead_session_para, pub padding: [__u8; 32] }

pub const VIRTIO_CRYPTO_RSA_RAW_PADDING: u32 = 0; pub const VIRTIO_CRYPTO_RSA_PKCS1_PADDING: u32 = 1; pub const VIRTIO_CRYPTO_RSA_NO_HASH: u32 = 0; pub const VIRTIO_CRYPTO_RSA_MD2: u32 = 1; pub const VIRTIO_CRYPTO_RSA_MD3: u32 = 2; pub const VIRTIO_CRYPTO_RSA_MD4: u32 = 3; pub const VIRTIO_CRYPTO_RSA_MD5: u32 = 4; pub const VIRTIO_CRYPTO_RSA_SHA1: u32 = 5; pub const VIRTIO_CRYPTO_RSA_SHA256: u32 = 6; pub const VIRTIO_CRYPTO_RSA_SHA384: u32 = 7; pub const VIRTIO_CRYPTO_RSA_SHA512: u32 = 8; pub const VIRTIO_CRYPTO_RSA_SHA224: u32 = 9;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_rsa_session_para { pub padding_algo: __le32, pub hash_algo: __le32 }
pub const VIRTIO_CRYPTO_CURVE_UNKNOWN: u32 = 0; pub const VIRTIO_CRYPTO_CURVE_NIST_P192: u32 = 1; pub const VIRTIO_CRYPTO_CURVE_NIST_P224: u32 = 2; pub const VIRTIO_CRYPTO_CURVE_NIST_P256: u32 = 3; pub const VIRTIO_CRYPTO_CURVE_NIST_P384: u32 = 4; pub const VIRTIO_CRYPTO_CURVE_NIST_P521: u32 = 5;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_ecdsa_session_para { pub curve_id: __le32, pub padding: __le32 }
pub const VIRTIO_CRYPTO_NO_AKCIPHER: u32 = 0; pub const VIRTIO_CRYPTO_AKCIPHER_RSA: u32 = 1; pub const VIRTIO_CRYPTO_AKCIPHER_DSA: u32 = 2; pub const VIRTIO_CRYPTO_AKCIPHER_ECDSA: u32 = 3; pub const VIRTIO_CRYPTO_AKCIPHER_KEY_TYPE_PUBLIC: u32 = 1; pub const VIRTIO_CRYPTO_AKCIPHER_KEY_TYPE_PRIVATE: u32 = 2;
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_akcipher_session_para_u { pub rsa: virtio_crypto_rsa_session_para, pub ecdsa: virtio_crypto_ecdsa_session_para }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_akcipher_session_para { pub algo: __le32, pub keytype: __le32, pub keylen: __le32, pub u: virtio_crypto_akcipher_session_para_u }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_akcipher_create_session_req { pub para: virtio_crypto_akcipher_session_para, pub padding: [__u8; 36] }

pub const VIRTIO_CRYPTO_SYM_ALG_CHAIN_ORDER_HASH_THEN_CIPHER: u32 = 1; pub const VIRTIO_CRYPTO_SYM_ALG_CHAIN_ORDER_CIPHER_THEN_HASH: u32 = 2; pub const VIRTIO_CRYPTO_SYM_HASH_MODE_PLAIN: u32 = 1; pub const VIRTIO_CRYPTO_SYM_HASH_MODE_AUTH: u32 = 2; pub const VIRTIO_CRYPTO_SYM_HASH_MODE_NESTED: u32 = 3;
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_alg_chain_session_para_u { pub hash_param: virtio_crypto_hash_session_para, pub mac_param: virtio_crypto_mac_session_para, pub padding: [__u8; 16] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_alg_chain_session_para { pub alg_chain_order: __le32, pub hash_mode: __le32, pub cipher_param: virtio_crypto_cipher_session_para, pub u: virtio_crypto_alg_chain_session_para_u, pub aad_len: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_alg_chain_session_req { pub para: virtio_crypto_alg_chain_session_para }
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_sym_create_session_req_u { pub cipher: virtio_crypto_cipher_session_req, pub chain: virtio_crypto_alg_chain_session_req, pub padding: [__u8; 48] }
pub const VIRTIO_CRYPTO_SYM_OP_NONE: u32 = 0; pub const VIRTIO_CRYPTO_SYM_OP_CIPHER: u32 = 1; pub const VIRTIO_CRYPTO_SYM_OP_ALGORITHM_CHAINING: u32 = 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_sym_create_session_req { pub u: virtio_crypto_sym_create_session_req_u, pub op_type: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_destroy_session_req { pub session_id: __le64, pub padding: [__u8; 48] }
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_op_ctrl_req_u { pub sym_create_session: virtio_crypto_sym_create_session_req, pub hash_create_session: virtio_crypto_hash_create_session_req, pub mac_create_session: virtio_crypto_mac_create_session_req, pub aead_create_session: virtio_crypto_aead_create_session_req, pub akcipher_create_session: virtio_crypto_akcipher_create_session_req, pub destroy_session: virtio_crypto_destroy_session_req, pub padding: [__u8; 56] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_op_ctrl_req { pub header: virtio_crypto_ctrl_header, pub u: virtio_crypto_op_ctrl_req_u }

pub const VIRTIO_CRYPTO_CIPHER_ENCRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_CIPHER, 0); pub const VIRTIO_CRYPTO_CIPHER_DECRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_CIPHER, 1); pub const VIRTIO_CRYPTO_HASH: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_HASH, 0); pub const VIRTIO_CRYPTO_MAC: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_MAC, 0); pub const VIRTIO_CRYPTO_AEAD_ENCRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AEAD, 0); pub const VIRTIO_CRYPTO_AEAD_DECRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AEAD, 1); pub const VIRTIO_CRYPTO_AKCIPHER_ENCRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 0); pub const VIRTIO_CRYPTO_AKCIPHER_DECRYPT: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 1); pub const VIRTIO_CRYPTO_AKCIPHER_SIGN: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 2); pub const VIRTIO_CRYPTO_AKCIPHER_VERIFY: u32 = VIRTIO_CRYPTO_OPCODE(VIRTIO_CRYPTO_SERVICE_AKCIPHER, 3);
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_op_header { pub opcode: __le32, pub algo: __le32, pub session_id: __le64, pub flag: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_cipher_para { pub iv_len: __le32, pub src_data_len: __le32, pub dst_data_len: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_hash_para { pub src_data_len: __le32, pub hash_result_len: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_mac_para { pub hash: virtio_crypto_hash_para }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_aead_para { pub iv_len: __le32, pub aad_len: __le32, pub src_data_len: __le32, pub dst_data_len: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_cipher_data_req { pub para: virtio_crypto_cipher_para, pub padding: [__u8; 24] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_hash_data_req { pub para: virtio_crypto_hash_para, pub padding: [__u8; 40] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_mac_data_req { pub para: virtio_crypto_mac_para, pub padding: [__u8; 40] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_alg_chain_data_para { pub iv_len: __le32, pub src_data_len: __le32, pub dst_data_len: __le32, pub cipher_start_src_offset: __le32, pub len_to_cipher: __le32, pub hash_start_src_offset: __le32, pub len_to_hash: __le32, pub aad_len: __le32, pub hash_result_len: __le32, pub reserved: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_alg_chain_data_req { pub para: virtio_crypto_alg_chain_data_para }
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_sym_data_req_u { pub cipher: virtio_crypto_cipher_data_req, pub chain: virtio_crypto_alg_chain_data_req, pub padding: [__u8; 40] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_sym_data_req { pub u: virtio_crypto_sym_data_req_u, pub op_type: __le32, pub padding: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_aead_data_req { pub para: virtio_crypto_aead_para, pub padding: [__u8; 32] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_akcipher_para { pub src_data_len: __le32, pub dst_data_len: __le32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_akcipher_data_req { pub para: virtio_crypto_akcipher_para, pub padding: [__u8; 40] }
#[repr(C)] #[derive(Copy, Clone)] pub union virtio_crypto_op_data_req_u { pub sym_req: virtio_crypto_sym_data_req, pub hash_req: virtio_crypto_hash_data_req, pub mac_req: virtio_crypto_mac_data_req, pub aead_req: virtio_crypto_aead_data_req, pub akcipher_req: virtio_crypto_akcipher_data_req, pub padding: [__u8; 48] }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_op_data_req { pub header: virtio_crypto_op_header, pub u: virtio_crypto_op_data_req_u }

pub const VIRTIO_CRYPTO_OK: u32 = 0; pub const VIRTIO_CRYPTO_ERR: u32 = 1; pub const VIRTIO_CRYPTO_BADMSG: u32 = 2; pub const VIRTIO_CRYPTO_NOTSUPP: u32 = 3; pub const VIRTIO_CRYPTO_INVSESS: u32 = 4; pub const VIRTIO_CRYPTO_NOSPC: u32 = 5; pub const VIRTIO_CRYPTO_KEY_REJECTED: u32 = 6; pub const VIRTIO_CRYPTO_S_HW_READY: u32 = 1 << 0;
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_config { pub status: __le32, pub max_dataqueues: __le32, pub crypto_services: __le32, pub cipher_algo_l: __le32, pub cipher_algo_h: __le32, pub hash_algo: __le32, pub mac_algo_l: __le32, pub mac_algo_h: __le32, pub aead_algo: __le32, pub max_cipher_key_len: __le32, pub max_auth_key_len: __le32, pub akcipher_algo: __le32, pub max_size: __le64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct virtio_crypto_inhdr { pub status: __u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
