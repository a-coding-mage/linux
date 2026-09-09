/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* Translated from cc_hw_queue_defs.h. Linux/kernel dependencies are external. */

pub const HW_DESC_SIZE_WORDS: usize = 6;
pub const HW_QUEUE_SLOTS_MAX: usize = 15;

pub const HW_KEY_MASK_CIPHER_DO: u32 = 0x3;
pub const HW_KEY_SHIFT_CIPHER_CFG2: u32 = 2;

pub const CC_NUM_HW_KEY_SLOTS: u32 = 4;
pub const CC_FIRST_HW_KEY_SLOT: u32 = 0;
pub const CC_LAST_HW_KEY_SLOT: u32 = CC_FIRST_HW_KEY_SLOT + CC_NUM_HW_KEY_SLOTS - 1;
pub const CC_NUM_CPP_KEY_SLOTS: u32 = 8;
pub const CC_FIRST_CPP_KEY_SLOT: u32 = 16;
pub const CC_LAST_CPP_KEY_SLOT: u32 = CC_FIRST_CPP_KEY_SLOT + CC_NUM_CPP_KEY_SLOTS - 1;
pub const CC_CPP_DIN_ADDR: u32 = 0xFF00FF00;
pub const CC_CPP_DIN_SIZE: u32 = 0xFF00FF;

/* These constants depend on register definitions supplied by cc_kernel_regs.h. */
pub const WORD0_VALUE: u32 = CC_HWQ_GENMASK!(0, VALUE);
pub const WORD0_CPP_CIPHER_MODE: u32 = CC_HWQ_GENMASK!(0, CPP_CIPHER_MODE);
pub const WORD1_DIN_CONST_VALUE: u32 = CC_HWQ_GENMASK!(1, DIN_CONST_VALUE);
pub const WORD1_DIN_DMA_MODE: u32 = CC_HWQ_GENMASK!(1, DIN_DMA_MODE);
pub const WORD1_DIN_SIZE: u32 = CC_HWQ_GENMASK!(1, DIN_SIZE);
pub const WORD1_NOT_LAST: u32 = CC_HWQ_GENMASK!(1, NOT_LAST);
pub const WORD1_NS_BIT: u32 = CC_HWQ_GENMASK!(1, NS_BIT);
pub const WORD1_LOCK_QUEUE: u32 = CC_HWQ_GENMASK!(1, LOCK_QUEUE);
pub const WORD2_VALUE: u32 = CC_HWQ_GENMASK!(2, VALUE);
pub const WORD3_DOUT_DMA_MODE: u32 = CC_HWQ_GENMASK!(3, DOUT_DMA_MODE);
pub const WORD3_DOUT_LAST_IND: u32 = CC_HWQ_GENMASK!(3, DOUT_LAST_IND);
pub const WORD3_DOUT_SIZE: u32 = CC_HWQ_GENMASK!(3, DOUT_SIZE);
pub const WORD3_HASH_XOR_BIT: u32 = CC_HWQ_GENMASK!(3, HASH_XOR_BIT);
pub const WORD3_NS_BIT: u32 = CC_HWQ_GENMASK!(3, NS_BIT);
pub const WORD3_QUEUE_LAST_IND: u32 = CC_HWQ_GENMASK!(3, QUEUE_LAST_IND);
pub const WORD4_ACK_NEEDED: u32 = CC_HWQ_GENMASK!(4, ACK_NEEDED);
pub const WORD4_AES_SEL_N_HASH: u32 = CC_HWQ_GENMASK!(4, AES_SEL_N_HASH);
pub const WORD4_AES_XOR_CRYPTO_KEY: u32 = CC_HWQ_GENMASK!(4, AES_XOR_CRYPTO_KEY);
pub const WORD4_BYTES_SWAP: u32 = CC_HWQ_GENMASK!(4, BYTES_SWAP);
pub const WORD4_CIPHER_CONF0: u32 = CC_HWQ_GENMASK!(4, CIPHER_CONF0);
pub const WORD4_CIPHER_CONF1: u32 = CC_HWQ_GENMASK!(4, CIPHER_CONF1);
pub const WORD4_CIPHER_CONF2: u32 = CC_HWQ_GENMASK!(4, CIPHER_CONF2);
pub const WORD4_CIPHER_DO: u32 = CC_HWQ_GENMASK!(4, CIPHER_DO);
pub const WORD4_CIPHER_MODE: u32 = CC_HWQ_GENMASK!(4, CIPHER_MODE);
pub const WORD4_CMAC_SIZE0: u32 = CC_HWQ_GENMASK!(4, CMAC_SIZE0);
pub const WORD4_DATA_FLOW_MODE: u32 = CC_HWQ_GENMASK!(4, DATA_FLOW_MODE);
pub const WORD4_KEY_SIZE: u32 = CC_HWQ_GENMASK!(4, KEY_SIZE);
pub const WORD4_SETUP_OPERATION: u32 = CC_HWQ_GENMASK!(4, SETUP_OPERATION);
pub const WORD5_DIN_ADDR_HIGH: u32 = CC_HWQ_GENMASK!(5, DIN_ADDR_HIGH);
pub const WORD5_DOUT_ADDR_HIGH: u32 = CC_HWQ_GENMASK!(5, DOUT_ADDR_HIGH);

#[repr(C)]
pub union cc_hw_desc_data {
    pub word: [u32; HW_DESC_SIZE_WORDS],
    pub hword: [u16; HW_DESC_SIZE_WORDS * 2],
}
#[repr(C)]
pub struct cc_hw_desc { pub data: cc_hw_desc_data }

#[repr(i32)] pub enum cc_axi_sec { AXI_SECURE = 0, AXI_NOT_SECURE = 1 }
#[repr(i32)] pub enum cc_desc_direction { DESC_DIRECTION_ILLEGAL = -1, DESC_DIRECTION_ENCRYPT_ENCRYPT = 0, DESC_DIRECTION_DECRYPT_DECRYPT = 1, DESC_DIRECTION_DECRYPT_ENCRYPT = 3, DESC_DIRECTION_END = i32::MAX }
#[repr(i32)] pub enum cc_dma_mode { DMA_MODE_NULL = -1, NO_DMA = 0, DMA_SRAM = 1, DMA_DLLI = 2, DMA_MLLI = 3, DMA_MODE_END = i32::MAX }
#[repr(i32)] pub enum cc_flow_mode { FLOW_MODE_NULL = -1, BYPASS = 0, DIN_AES_DOUT = 1, AES_to_HASH = 2, AES_and_HASH = 3, DIN_DES_DOUT = 4, DES_to_HASH = 5, DES_and_HASH = 6, DIN_HASH = 7, DIN_HASH_and_BYPASS = 8, AESMAC_and_BYPASS = 9, AES_to_HASH_and_DOUT = 10, DIN_RC4_DOUT = 11, DES_to_HASH_and_DOUT = 12, AES_to_AES_to_HASH_and_DOUT = 13, AES_to_AES_to_HASH = 14, AES_to_HASH_and_AES = 15, DIN_SM4_DOUT = 16, DIN_AES_AESMAC = 17, HASH_to_DOUT = 18, S_DIN_to_AES = 32, S_DIN_to_AES2 = 33, S_DIN_to_DES = 34, S_DIN_to_RC4 = 35, S_DIN_to_SM4 = 36, S_DIN_to_HASH = 37, S_AES_to_DOUT = 38, S_AES2_to_DOUT = 39, S_SM4_to_DOUT = 40, S_RC4_to_DOUT = 41, S_DES_to_DOUT = 42, S_HASH_to_DOUT = 43, SET_FLOW_ID = 44, FLOW_MODE_END = i32::MAX }
#[repr(i32)] pub enum cc_setup_op { SETUP_LOAD_NOP = 0, SETUP_LOAD_STATE0 = 1, SETUP_LOAD_STATE1 = 2, SETUP_LOAD_STATE2 = 3, SETUP_LOAD_KEY0 = 4, SETUP_LOAD_XEX_KEY = 5, SETUP_WRITE_STATE0 = 8, SETUP_WRITE_STATE1 = 9, SETUP_WRITE_STATE2 = 10, SETUP_WRITE_STATE3 = 11, SETUP_OP_END = i32::MAX }
#[repr(i32)] pub enum cc_hash_conf_pad { HASH_PADDING_DISABLED = 0, HASH_PADDING_ENABLED = 1, HASH_DIGEST_RESULT_LITTLE_ENDIAN = 2, HASH_CONFIG1_PADDING_RESERVE32 = i32::MAX }
#[repr(i32)] pub enum cc_aes_mac_selector { AES_SK = 1, AES_CMAC_INIT = 2, AES_CMAC_SIZE0 = 3, AES_MAC_END = i32::MAX }
#[repr(i32)] pub enum cc_hw_crypto_key { USER_KEY = 0, ROOT_KEY = 1, PROVISIONING_KEY = 2, SESSION_KEY = 3, RESERVED_KEY = 4, PLATFORM_KEY = 5, CUSTOMER_KEY = 6, KFDE0_KEY = 7, KFDE1_KEY = 9, KFDE2_KEY = 10, KFDE3_KEY = 11, END_OF_KEYS = i32::MAX }
#[repr(i32)] pub enum cc_hw_aes_key_size { AES_128_KEY = 0, AES_192_KEY = 1, AES_256_KEY = 2, END_OF_AES_KEYS = i32::MAX }
#[repr(i32)] pub enum cc_hash_cipher_pad { DO_NOT_PAD = 0, DO_PAD = 1, HASH_CIPHER_DO_PADDING_RESERVE32 = i32::MAX }

#[inline] pub unsafe fn hw_desc_init(pdesc: *mut cc_hw_desc) { core::ptr::write_bytes(pdesc, 0, 1); }
#[inline] pub unsafe fn set_queue_last_ind_bit(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_QUEUE_LAST_IND, 1); }
#[inline] pub unsafe fn set_din_type(pdesc: *mut cc_hw_desc, dma_mode: cc_dma_mode, addr: dma_addr_t, size: u32, axi_sec: cc_axi_sec) { (*pdesc).data.word[0] = lower_32_bits(addr); #[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)] { (*pdesc).data.word[5] |= FIELD_PREP!(WORD5_DIN_ADDR_HIGH, upper_32_bits(addr)); } (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_DIN_DMA_MODE, dma_mode) | FIELD_PREP!(WORD1_DIN_SIZE, size) | FIELD_PREP!(WORD1_NS_BIT, axi_sec); }
#[inline] pub unsafe fn set_din_no_dma(pdesc: *mut cc_hw_desc, addr: u32, size: u32) { (*pdesc).data.word[0] = addr; (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_DIN_SIZE, size); }
#[inline] pub unsafe fn set_cpp_crypto_key(pdesc: *mut cc_hw_desc, slot: u8) { (*pdesc).data.word[0] |= CC_CPP_DIN_ADDR; (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_DIN_SIZE, CC_CPP_DIN_SIZE); (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_LOCK_QUEUE, 1); (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_SETUP_OPERATION, slot); }
#[inline] pub unsafe fn set_din_sram(pdesc: *mut cc_hw_desc, addr: u32, size: u32) { (*pdesc).data.word[0] = addr; (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_DIN_SIZE, size) | FIELD_PREP!(WORD1_DIN_DMA_MODE, DMA_SRAM); }
#[inline] pub unsafe fn set_din_const(pdesc: *mut cc_hw_desc, val: u32, size: u32) { (*pdesc).data.word[0] = val; (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_DIN_CONST_VALUE, 1) | FIELD_PREP!(WORD1_DIN_DMA_MODE, DMA_SRAM) | FIELD_PREP!(WORD1_DIN_SIZE, size); }
#[inline] pub unsafe fn set_din_not_last_indication(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[1] |= FIELD_PREP!(WORD1_NOT_LAST, 1); }
#[inline] pub unsafe fn set_dout_type(pdesc: *mut cc_hw_desc, dma_mode: cc_dma_mode, addr: dma_addr_t, size: u32, axi_sec: cc_axi_sec) { (*pdesc).data.word[2] = lower_32_bits(addr); #[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)] { (*pdesc).data.word[5] |= FIELD_PREP!(WORD5_DOUT_ADDR_HIGH, upper_32_bits(addr)); } (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_DOUT_DMA_MODE, dma_mode) | FIELD_PREP!(WORD3_DOUT_SIZE, size) | FIELD_PREP!(WORD3_NS_BIT, axi_sec); }
#[inline] pub unsafe fn set_dout_dlli(pdesc: *mut cc_hw_desc, addr: dma_addr_t, size: u32, axi_sec: cc_axi_sec, last_ind: u32) { set_dout_type(pdesc, cc_dma_mode::DMA_DLLI, addr, size, axi_sec); (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_DOUT_LAST_IND, last_ind); }
#[inline] pub unsafe fn set_dout_mlli(pdesc: *mut cc_hw_desc, addr: u32, size: u32, axi_sec: cc_axi_sec, last_ind: bool) { set_dout_type(pdesc, cc_dma_mode::DMA_MLLI, addr, size, axi_sec); (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_DOUT_LAST_IND, last_ind); }
#[inline] pub unsafe fn set_dout_no_dma(pdesc: *mut cc_hw_desc, addr: u32, size: u32, write_enable: bool) { (*pdesc).data.word[2] = addr; (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_DOUT_SIZE, size) | FIELD_PREP!(WORD3_DOUT_LAST_IND, write_enable); }
#[inline] pub unsafe fn set_xor_val(pdesc: *mut cc_hw_desc, val: u32) { (*pdesc).data.word[2] = val; }
#[inline] pub unsafe fn set_xor_active(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_HASH_XOR_BIT, 1); }
#[inline] pub unsafe fn set_aes_not_hash_mode(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_AES_SEL_N_HASH, 1); }
#[inline] pub unsafe fn set_aes_xor_crypto_key(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_AES_XOR_CRYPTO_KEY, 1); }
#[inline] pub unsafe fn set_dout_sram(pdesc: *mut cc_hw_desc, addr: u32, size: u32) { (*pdesc).data.word[2] = addr; (*pdesc).data.word[3] |= FIELD_PREP!(WORD3_DOUT_DMA_MODE, DMA_SRAM) | FIELD_PREP!(WORD3_DOUT_SIZE, size); }
#[inline] pub unsafe fn set_xex_data_unit_size(pdesc: *mut cc_hw_desc, size: u32) { (*pdesc).data.word[2] = size; }
#[inline] pub unsafe fn set_multi2_num_rounds(pdesc: *mut cc_hw_desc, num: u32) { (*pdesc).data.word[2] = num; }
#[inline] pub unsafe fn set_flow_mode(pdesc: *mut cc_hw_desc, mode: cc_flow_mode) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_DATA_FLOW_MODE, mode); }
#[inline] pub unsafe fn set_cipher_mode(pdesc: *mut cc_hw_desc, mode: i32) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CIPHER_MODE, mode); }
#[inline] pub unsafe fn set_hash_cipher_mode(pdesc: *mut cc_hw_desc, cipher_mode: drv_cipher_mode, hash_mode: drv_hash_mode) { set_cipher_mode(pdesc, cipher_mode); if hash_mode == DRV_HASH_SM3 { set_aes_xor_crypto_key(pdesc); } }
#[inline] pub unsafe fn set_cipher_config0(pdesc: *mut cc_hw_desc, mode: i32) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CIPHER_CONF0, mode); }
#[inline] pub unsafe fn set_cipher_config1(pdesc: *mut cc_hw_desc, config: cc_hash_conf_pad) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CIPHER_CONF1, config); }
#[inline] pub unsafe fn set_hw_crypto_key(pdesc: *mut cc_hw_desc, hw_key: cc_hw_crypto_key) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CIPHER_DO, (hw_key as i32 & HW_KEY_MASK_CIPHER_DO as i32)) | FIELD_PREP!(WORD4_CIPHER_CONF2, ((hw_key as i32) >> HW_KEY_SHIFT_CIPHER_CFG2)); }
#[inline] pub unsafe fn set_bytes_swap(pdesc: *mut cc_hw_desc, config: bool) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_BYTES_SWAP, config); }
#[inline] pub unsafe fn set_cmac_size0_mode(pdesc: *mut cc_hw_desc) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CMAC_SIZE0, 1); }
#[inline] pub unsafe fn set_key_size(pdesc: *mut cc_hw_desc, size: u32) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_KEY_SIZE, size); }
#[inline] pub unsafe fn set_key_size_aes(pdesc: *mut cc_hw_desc, size: u32) { set_key_size(pdesc, (size >> 3) - 2); }
#[inline] pub unsafe fn set_key_size_des(pdesc: *mut cc_hw_desc, size: u32) { set_key_size(pdesc, (size >> 3) - 1); }
#[inline] pub unsafe fn set_setup_mode(pdesc: *mut cc_hw_desc, mode: cc_setup_op) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_SETUP_OPERATION, mode); }
#[inline] pub unsafe fn set_cipher_do(pdesc: *mut cc_hw_desc, config: cc_hash_cipher_pad) { (*pdesc).data.word[4] |= FIELD_PREP!(WORD4_CIPHER_DO, (config as i32 & HW_KEY_MASK_CIPHER_DO as i32)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
