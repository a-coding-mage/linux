/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* ARM CryptoCell Linux Crypto Driver */

// C header dependencies are supplied by the surrounding translation unit.

extern "C" {
    pub static mut cc_dump_desc: bool;
    pub static mut cc_dump_bytes: bool;
}

pub const DRV_MODULE_VERSION: &str = "5.0";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hw_rev {
    CC_HW_REV_630 = 630,
    CC_HW_REV_710 = 710,
    CC_HW_REV_712 = 712,
    CC_HW_REV_713 = 713,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_std_body {
    CC_STD_NIST = 0x1,
    CC_STD_OSCCA = 0x2,
    CC_STD_ALL = 0x3,
}

pub const CC_PINS_FULL: u32 = 0x0;
pub const CC_PINS_SLIM: u32 = 0x9F;
pub const DMA_BIT_MASK_LEN: u32 = 48;
pub const CC_AXI_IRQ_MASK: u32 = (1 << CC_AXIM_CFG_BRESPMASK_BIT_SHIFT) |
    (1 << CC_AXIM_CFG_RRESPMASK_BIT_SHIFT) |
    (1 << CC_AXIM_CFG_INFLTMASK_BIT_SHIFT) |
    (1 << CC_AXIM_CFG_COMPMASK_BIT_SHIFT);
pub const CC_AXI_ERR_IRQ_MASK: u32 = BIT(CC_HOST_IRR_AXI_ERR_INT_BIT_SHIFT);
pub const CC_COMP_IRQ_MASK: u32 = BIT(CC_HOST_IRR_AXIM_COMP_INT_BIT_SHIFT);
pub const CC_SECURITY_DISABLED_MASK: u32 = BIT(CC_SECURITY_DISABLED_VALUE_BIT_SHIFT);
pub const CC_NVM_IS_IDLE_MASK: u32 = BIT(CC_NVM_IS_IDLE_VALUE_BIT_SHIFT);
pub const AXIM_MON_COMP_VALUE: u32 = CC_GENMASK(CC_AXIM_MON_COMP_VALUE);
pub const CC_CPP_AES_ABORT_MASK: u32 =
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_0_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_1_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_2_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_3_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_4_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_5_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_6_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_AES_7_MASK_BIT_SHIFT);
pub const CC_CPP_SM4_ABORT_MASK: u32 =
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_0_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_1_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_2_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_3_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_4_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_5_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_6_MASK_BIT_SHIFT) |
    BIT(CC_HOST_IMR_REE_OP_ABORTED_SM_7_MASK_BIT_SHIFT);

#[macro_export]
macro_rules! CC_REG { ($reg_name:ident) => { concat_idents::concat_idents!(CC_, $reg_name, _REG_OFFSET) }; }

pub const CC_GPR0_IRQ_MASK: u32 = BIT(CC_HOST_IRR_GPR0_BIT_SHIFT);
pub const CC_CRA_PRIO: i32 = 400;
pub const MIN_HW_QUEUE_SIZE: usize = 50;
pub const MAX_REQUEST_QUEUE_SIZE: usize = 4096;
pub const MAX_MLLI_BUFF_SIZE: usize = 2080;
pub const NS_BIT: u32 = 1;
pub const AXI_ID: u32 = 0;

#[repr(C)]
pub struct cc_cpp_req { pub is_cpp: bool, pub alg: cc_cpp_alg, pub slot: u8 }

pub const CC_MAX_IVGEN_DMA_ADDRESSES: usize = 3;

#[repr(C)]
pub struct cc_crypto_req {
    pub user_cb: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void, i32)>,
    pub user_arg: *mut core::ffi::c_void,
    pub seq_compl: completion,
    pub cpp: cc_cpp_req,
}

#[repr(C)]
pub struct cc_drvdata {
    pub cc_base: *mut core::ffi::c_void, pub irq: i32, pub hw_queue_avail: completion,
    pub plat_dev: *mut platform_device, pub mlli_sram_addr: u32, pub mlli_buffs_pool: *mut dma_pool,
    pub alg_list: list_head, pub hash_handle: *mut core::ffi::c_void, pub aead_handle: *mut core::ffi::c_void,
    pub request_mgr_handle: *mut core::ffi::c_void, pub fips_handle: *mut core::ffi::c_void,
    pub sram_free_offset: u32, pub dir: *mut dentry, pub clk: *mut clk, pub coherent: bool,
    pub hw_rev_name: *mut i8, pub hw_rev: cc_hw_rev, pub axim_mon_offset: u32, pub sig_offset: u32,
    pub ver_offset: u32, pub std_bodies: i32, pub sec_disabled: bool, pub comp_mask: u32,
    pub cache_params: u32, pub ace_const: u32,
}

#[repr(C)]
pub struct cc_crypto_alg { pub entry: list_head, pub cipher_mode: i32, pub flow_mode: i32, pub auth_mode: i32, pub drvdata: *mut cc_drvdata, pub skcipher_alg: skcipher_alg, pub aead_alg: aead_alg }

#[repr(C)]
pub union cc_alg_template_u { pub skcipher: skcipher_alg, pub aead: aead_alg }
#[repr(C)]
pub struct cc_alg_template { pub name: [i8; CRYPTO_MAX_ALG_NAME], pub driver_name: [i8; CRYPTO_MAX_ALG_NAME], pub blocksize: u32, pub template_u: cc_alg_template_u, pub cipher_mode: i32, pub flow_mode: i32, pub auth_mode: i32, pub min_hw_rev: u32, pub std_body: cc_std_body, pub sec_func: bool, pub data_unit: u32, pub drvdata: *mut cc_drvdata }

#[repr(C)]
pub struct async_gen_req_ctx { pub iv_dma_addr: dma_addr_t, pub iv: *mut u8, pub op_type: drv_crypto_direction }

#[inline]
pub unsafe fn drvdata_to_dev(drvdata: *mut cc_drvdata) -> *mut device { &mut (*(*drvdata).plat_dev).dev }

extern "C" {
    pub fn __dump_byte_array(name: *const i8, buf: *const u8, len: usize);
    pub fn cc_wait_for_reset_completion(drvdata: *mut cc_drvdata) -> bool;
    pub fn init_cc_regs(drvdata: *mut cc_drvdata) -> i32;
    pub fn fini_cc_regs(drvdata: *mut cc_drvdata);
    pub fn cc_get_default_hash_len(drvdata: *mut cc_drvdata) -> u32;
    pub fn iowrite32(val: u32, addr: *mut core::ffi::c_void);
    pub fn ioread32(addr: *mut core::ffi::c_void) -> u32;
}

#[inline]
pub unsafe fn dump_byte_array(name: *const i8, the_array: *const u8, size: usize) { if cc_dump_bytes { __dump_byte_array(name, the_array, size); } }
#[inline]
pub unsafe fn cc_iowrite(drvdata: *mut cc_drvdata, reg: usize, val: u32) { iowrite32(val, ((*drvdata).cc_base as *mut u8).add(reg) as *mut _); }
#[inline]
pub unsafe fn cc_ioread(drvdata: *mut cc_drvdata, reg: usize) -> u32 { ioread32(((*drvdata).cc_base as *mut u8).add(reg) as *mut _) }
#[inline]
pub unsafe fn cc_gfp_flags(req: *mut crypto_async_request) -> gfp_t { if (*req).flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC } }
#[inline]
pub unsafe fn set_queue_last_ind(drvdata: *mut cc_drvdata, pdesc: *mut cc_hw_desc) { if (*drvdata).hw_rev as i32 >= CC_HW_REV_712 as i32 { set_queue_last_ind_bit(pdesc); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
