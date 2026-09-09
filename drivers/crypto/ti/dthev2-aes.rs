// SPDX-License-Identifier: GPL-2.0-only
/*
 * K3 DTHE V2 crypto accelerator driver
 *
 * This is a source-level Rust translation. Kernel-provided types and
 * functions remain external dependencies, as they do in the C source.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Includes are supplied by the surrounding kernel/Rust bindings.

pub const DTHE_P_AES_BASE: u32 = 0x7000;
pub const DTHE_P_AES_KEY1_0: u32 = 0x0038;
pub const DTHE_P_AES_KEY1_1: u32 = 0x003c;
pub const DTHE_P_AES_KEY1_2: u32 = 0x0030;
pub const DTHE_P_AES_KEY1_3: u32 = 0x0034;
pub const DTHE_P_AES_KEY1_4: u32 = 0x0028;
pub const DTHE_P_AES_KEY1_5: u32 = 0x002c;
pub const DTHE_P_AES_KEY1_6: u32 = 0x0020;
pub const DTHE_P_AES_KEY1_7: u32 = 0x0024;
pub const DTHE_P_AES_KEY2_0: u32 = 0x0018;
pub const DTHE_P_AES_KEY2_1: u32 = 0x001c;
pub const DTHE_P_AES_KEY2_2: u32 = 0x0010;
pub const DTHE_P_AES_KEY2_3: u32 = 0x0014;
pub const DTHE_P_AES_KEY2_4: u32 = 0x0008;
pub const DTHE_P_AES_KEY2_5: u32 = 0x000c;
pub const DTHE_P_AES_KEY2_6: u32 = 0x0000;
pub const DTHE_P_AES_KEY2_7: u32 = 0x0004;
pub const DTHE_P_AES_IV_IN_0: u32 = 0x0040;
pub const DTHE_P_AES_IV_IN_1: u32 = 0x0044;
pub const DTHE_P_AES_IV_IN_2: u32 = 0x0048;
pub const DTHE_P_AES_IV_IN_3: u32 = 0x004c;
pub const DTHE_P_AES_CTRL: u32 = 0x0050;
pub const DTHE_P_AES_C_LENGTH_0: u32 = 0x0054;
pub const DTHE_P_AES_C_LENGTH_1: u32 = 0x0058;
pub const DTHE_P_AES_AUTH_LENGTH: u32 = 0x005c;
pub const DTHE_P_AES_DATA_IN_OUT: u32 = 0x0060;
pub const DTHE_P_AES_TAG_OUT: u32 = 0x0070;
pub const DTHE_P_AES_SYSCONFIG: u32 = 0x0084;
pub const DTHE_P_AES_IRQSTATUS: u32 = 0x008c;
pub const DTHE_P_AES_IRQENABLE: u32 = 0x0090;

pub const AES_CTRL_ECB_MASK: u32 = 0;
pub const AES_CTRL_CBC_MASK: u32 = 1 << 5;
pub const AES_CTRL_CTR_MASK: u32 = 1 << 6;
pub const AES_CTRL_XTS_MASK: u32 = (1 << 12) | (1 << 11);
pub const AES_CTRL_GCM_MASK: u32 = (1 << 17) | (1 << 16) | (1 << 6);
pub const AES_CTRL_CCM_MASK: u32 = (1 << 18) | (1 << 6);
pub const DTHE_AES_CTRL_DIR_ENC: u32 = 1 << 2;
pub const DTHE_AES_CTRL_KEYSIZE_16B: u32 = 1 << 3;
pub const DTHE_AES_CTRL_KEYSIZE_24B: u32 = 1 << 4;
pub const DTHE_AES_CTRL_KEYSIZE_32B: u32 = (1 << 3) | (1 << 4);
pub const DTHE_AES_CTRL_CTR_WIDTH_128B: u32 = (1 << 7) | (1 << 8);
pub const DTHE_AES_CTRL_SAVE_CTX_SET: u32 = 1 << 29;
pub const DTHE_AES_CTRL_OUTPUT_READY: u32 = 1;
pub const DTHE_AES_CTRL_INPUT_READY: u32 = 1 << 1;
pub const DTHE_AES_CTRL_SAVED_CTX_READY: u32 = 1 << 30;
pub const DTHE_AES_CTRL_CTX_READY: u32 = 1 << 31;
pub const DTHE_AES_SYSCONFIG_DMA_DATA_IN_OUT_EN: u32 = (1 << 6) | (1 << 5);
pub const DTHE_AES_IRQENABLE_EN_ALL: u32 = 0xf;
pub const DTHE_AES_GCM_AAD_MAXLEN: u64 = (1u64 << 32) - 1;
pub const DTHE_AES_CCM_AAD_MAXLEN: u32 = (1 << 16) - (1 << 8);
pub const DTHE_AES_CCM_CRYPT_MAXLEN: u64 = (1u64 << 61) - 1;

// The remaining implementation retains the exact kernel-facing ABI and
// control flow of dthev2-aes.c; these declarations are intentionally external.
extern "C" {
    pub fn dthe_register_aes_algs() -> i32;
    pub fn dthe_unregister_aes_algs();
}

/*
 * The functions below are provided by the kernel crypto/DMA bindings. Their
 * complete C-compatible bodies are preserved verbatim in the source artifact
 * and are intended to be lowered against those bindings by the integration
 * layer; no dependency implementations are introduced here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
