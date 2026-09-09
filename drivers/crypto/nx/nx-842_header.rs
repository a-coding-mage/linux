/* SPDX-License-Identifier: GPL-2.0 */

/* Linux dependencies are supplied by the surrounding translation unit. */

pub const DDE_BUFFER_ALIGN: i32 = 128;
pub const DDE_BUFFER_SIZE_MULT: i32 = 32;
pub const DDE_BUFFER_LAST_MULT: i32 = 8;
pub const DDL_LEN_MAX: i32 = 17;

pub const CCW_CI_842: u32 = 0x00003ff8;
pub const CCW_FC_842: u32 = 0x00000007;

pub const CCW_FC_842_COMP_NOCRC: i32 = 0;
pub const CCW_FC_842_COMP_CRC: i32 = 1;
pub const CCW_FC_842_DECOMP_NOCRC: i32 = 2;
pub const CCW_FC_842_DECOMP_CRC: i32 = 3;
pub const CCW_FC_842_MOVE: i32 = 4;

pub const CSB_CC_TRANSLATION_DUP1: i32 = 80;
pub const CSB_CC_TRANSLATION_DUP2: i32 = 82;
pub const CSB_CC_TRANSLATION_DUP3: i32 = 84;
pub const CSB_CC_TRANSLATION_DUP4: i32 = 86;
pub const CSB_CC_TRANSLATION_DUP5: i32 = 92;
pub const CSB_CC_TRANSLATION_DUP6: i32 = 94;
pub const CSB_CC_PROTECTION_DUP1: i32 = 81;
pub const CSB_CC_PROTECTION_DUP2: i32 = 83;
pub const CSB_CC_PROTECTION_DUP3: i32 = 85;
pub const CSB_CC_PROTECTION_DUP4: i32 = 87;
pub const CSB_CC_PROTECTION_DUP5: i32 = 93;
pub const CSB_CC_PROTECTION_DUP6: i32 = 95;
pub const CSB_CC_RD_EXTERNAL_DUP1: i32 = 89;
pub const CSB_CC_RD_EXTERNAL_DUP2: i32 = 90;
pub const CSB_CC_RD_EXTERNAL_DUP3: i32 = 91;

pub const CSB_CC_TPBC_GT_SPBC: i32 = 64;
pub const CSB_CC_CRC_MISMATCH: i32 = 65;
pub const CSB_CC_TEMPL_INVALID: i32 = 66;
pub const CSB_CC_TEMPL_OVERFLOW: i32 = 67;
pub const CSB_CC_DECRYPT_OVERFLOW: i32 = 64;
pub const CSB_CC_MINV_OVERFLOW: i32 = 128;
pub const CSB_CC_HW_EXPIRED_TIMER: i32 = 224;
pub const CSB_CC_HYP_RESERVE_START: i32 = 240;
pub const CSB_CC_HYP_RESERVE_END: i32 = 253;
pub const CSB_CC_HYP_RESERVE_P9_END: i32 = 251;
pub const CSB_CC_HYP_RESERVE_NO_INTR_SERVER: i32 = 252;
pub const CSB_CC_HYP_NO_HW: i32 = 254;
pub const CSB_CC_HYP_HANG_ABORTED: i32 = 255;

pub const CCB_CM_EXTRA_WRITE: u32 = CCB_CM0_ALL_COMPLETIONS & CCB_CM12_STORE;
pub const CCB_CM_INTERRUPT: u32 = CCB_CM0_ALL_COMPLETIONS & CCB_CM12_INTERRUPT;

#[inline]
pub const fn len_on_size(pa: usize, size: usize) -> usize {
    size - (pa & (size - 1))
}

#[inline]
pub const fn len_on_page(pa: usize) -> usize {
    len_on_size(pa, PAGE_SIZE)
}

pub struct crypto_scomp;

pub unsafe fn nx842_get_pa(addr: *mut core::ffi::c_void) -> libc::c_ulong {
    if !is_vmalloc_addr(addr) {
        return __pa(addr);
    }
    page_to_phys(vmalloc_to_page(addr)) + offset_in_page(addr)
}

#[repr(C)]
pub struct nx842_constraints {
    pub alignment: i32,
    pub multiple: i32,
    pub minimum: i32,
    pub maximum: i32,
}

#[repr(C)]
pub struct nx842_driver {
    pub name: *mut u8,
    pub owner: *mut module,
    pub workmem_size: usize,
    pub constraints: *mut nx842_constraints,
    pub compress: Option<unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u32, *mut core::ffi::c_void) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u32, *mut core::ffi::c_void) -> i32>,
}

#[repr(C, packed)]
pub struct nx842_crypto_header_group {
    pub padding: __be16,
    pub compressed_length: __be32,
    pub uncompressed_length: __be32,
}

#[repr(C, packed)]
pub struct nx842_crypto_header_hdr {
    pub magic: __be16,
    pub ignore: __be16,
    pub groups: u8,
}

#[repr(C, packed)]
pub struct nx842_crypto_header {
    pub hdr: nx842_crypto_header_hdr,
    pub group: [nx842_crypto_header_group; 0],
}

pub const NX842_CRYPTO_GROUP_MAX: usize = 0x20;

#[repr(C)]
pub struct nx842_crypto_ctx {
    pub lock: spinlock_t,
    pub wmem: *mut u8,
    pub sbounce: *mut u8,
    pub dbounce: *mut u8,
    pub header: nx842_crypto_header_hdr,
    pub group: [nx842_crypto_header_group; NX842_CRYPTO_GROUP_MAX],
    pub driver: *mut nx842_driver,
}

extern "C" {
    pub fn nx842_crypto_alloc_ctx(driver: *mut nx842_driver) -> *mut core::ffi::c_void;
    pub fn nx842_crypto_free_ctx(ctx: *mut core::ffi::c_void);
    pub fn nx842_crypto_compress(tfm: *mut crypto_scomp, src: *const u8, slen: u32,
                                 dst: *mut u8, dlen: *mut u32, ctx: *mut core::ffi::c_void) -> i32;
    pub fn nx842_crypto_decompress(tfm: *mut crypto_scomp, src: *const u8, slen: u32,
                                   dst: *mut u8, dlen: *mut u32, ctx: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
