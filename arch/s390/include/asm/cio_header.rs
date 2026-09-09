/* SPDX-License-Identifier: GPL-2.0 */
/* Common interface for I/O on S/390 */

// External C header dependencies: linux/bitops.h, linux/genalloc.h,
// asm/dma-types.h, asm/types.h, asm/tpi.h, and asm/scsw.h.

pub const LPM_ANYPATH: u8 = 0xff;
pub const __MAX_CSSID: u32 = 0;
pub const __MAX_SUBCHANNEL: u32 = 65535;
pub const __MAX_SSID: u32 = 3;
pub const CCW_MAX_BYTE_COUNT: u32 = 65535;

#[repr(C, packed(8))]
pub struct ccw1 {
    pub cmd_code: u8,
    pub flags: u8,
    pub count: u16,
    pub cda: dma32_t,
}

#[repr(C, packed(8))]
pub struct ccw0 {
    pub cmd_code: u8,
    // C bit-field cda:24; stored in the containing 32-bit word.
    pub cda: u32,
    pub flags: u8,
    pub reserved: u8,
    pub count: u16,
}

pub const CCW_FLAG_DC: u8 = 0x80;
pub const CCW_FLAG_CC: u8 = 0x40;
pub const CCW_FLAG_SLI: u8 = 0x20;
pub const CCW_FLAG_SKIP: u8 = 0x10;
pub const CCW_FLAG_PCI: u8 = 0x08;
pub const CCW_FLAG_IDA: u8 = 0x04;
pub const CCW_FLAG_SUSPEND: u8 = 0x02;

pub const CCW_CMD_READ_IPL: u8 = 0x02;
pub const CCW_CMD_NOOP: u8 = 0x03;
pub const CCW_CMD_BASIC_SENSE: u8 = 0x04;
pub const CCW_CMD_TIC: u8 = 0x08;
pub const CCW_CMD_STLCK: u8 = 0x14;
pub const CCW_CMD_SENSE_PGID: u8 = 0x34;
pub const CCW_CMD_SUSPEND_RECONN: u8 = 0x5B;
pub const CCW_CMD_RDC: u8 = 0x64;
pub const CCW_CMD_RELEASE: u8 = 0x94;
pub const CCW_CMD_SET_PGID: u8 = 0xAF;
pub const CCW_CMD_SENSE_ID: u8 = 0xE4;
pub const CCW_CMD_DCTL: u8 = 0xF3;
pub const SENSE_MAX_COUNT: u32 = 0x20;

#[repr(C, packed)] pub struct erw { pub bits: u32 }
#[repr(C, packed)] pub struct erw_eadm { pub bits: u32 }
#[repr(C, packed)] pub struct sublog { pub bits: u32 }

#[repr(C, packed)]
pub struct esw0 { pub sublog: sublog, pub erw: erw, pub faddr: [dma32_t; 2], pub saddr: dma32_t }
#[repr(C, packed)]
pub struct esw1 { pub zero0: u8, pub lpum: u8, pub zero16: u16, pub erw: erw, pub zeros: [u32; 3] }
#[repr(C, packed)]
pub struct esw2 { pub zero0: u8, pub lpum: u8, pub dcti: u16, pub erw: erw, pub zeros: [u32; 3] }
#[repr(C, packed)]
pub struct esw3 { pub zero0: u8, pub lpum: u8, pub res: u16, pub erw: erw, pub zeros: [u32; 3] }
#[repr(C, packed)]
pub struct esw_eadm { pub sublog: u32, pub erw: erw_eadm, pub reserved: [u32; 3] }

#[repr(C)]
pub union esw {
    pub esw0: esw0,
    pub esw1: esw1,
    pub esw2: esw2,
    pub esw3: esw3,
    pub eadm: esw_eadm,
}

#[repr(C, packed(4))]
pub struct irb {
    pub scsw: scsw,
    pub esw: esw,
    pub ecw: [u8; 32],
}

#[repr(C, packed)] pub struct ciw { pub bits: u32 }
pub const CIW_TYPE_RCD: u32 = 0x0;
pub const CIW_TYPE_SII: u32 = 0x1;
pub const CIW_TYPE_RNI: u32 = 0x2;

#[repr(C, packed)]
pub struct node_descriptor {
    pub flags: node_descriptor_flags,
    pub params: u32,
    pub type_: [u8; 6],
    pub model: [u8; 3],
    pub manufacturer: [u8; 3],
    pub plant: [u8; 2],
    pub seq: [u8; 12],
    pub tag: u16,
}
#[repr(C, packed)] pub union node_descriptor_flags { pub bits: u8, pub validity_reserved: u32 }
pub const ND_VALIDITY_VALID: u32 = 0;
pub const ND_VALIDITY_OUTDATED: u32 = 1;
pub const ND_VALIDITY_INVALID: u32 = 2;

pub const DOIO_ALLOW_SUSPEND: u32 = 0x0001;
pub const DOIO_DENY_PREFETCH: u32 = 0x0002;
pub const DOIO_SUPPRESS_INTER: u32 = 0x0004;
pub const CIO_GONE: u32 = 0x0001;
pub const CIO_NO_PATH: u32 = 0x0002;
pub const CIO_OPER: u32 = 0x0004;
pub const CIO_REVALIDATE: u32 = 0x0008;
pub const CIO_BOXED: u32 = 0x0010;

#[repr(C)] pub struct ccw_dev_id { pub ssid: u8, pub devno: u16 }

#[inline]
pub unsafe fn ccw_dev_id_is_equal(dev_id1: *mut ccw_dev_id, dev_id2: *mut ccw_dev_id) -> i32 {
    if (*dev_id1).ssid == (*dev_id2).ssid && (*dev_id1).devno == (*dev_id2).devno { 1 } else { 0 }
}

#[inline]
pub fn pathmask_to_pos(mask: u8) -> u8 {
    (8u8).wrapping_sub(mask.trailing_zeros() as u8 + 1)
}

unsafe extern "C" {
    pub fn css_schedule_reprobe();
    pub fn cio_dma_zalloc(size: usize) -> *mut core::ffi::c_void;
    pub fn cio_dma_free(cpu_addr: *mut core::ffi::c_void, size: usize);
    pub fn cio_get_dma_css_dev() -> *mut device;
    pub fn cio_gp_dma_zalloc(gp_dma: *mut gen_pool, dma_dev: *mut device, size: usize) -> *mut core::ffi::c_void;
    pub fn __cio_gp_dma_zalloc(gp_dma: *mut gen_pool, dma_dev: *mut device, size: usize, dma_handle: *mut dma32_t) -> *mut core::ffi::c_void;
    pub fn cio_gp_dma_free(gp_dma: *mut gen_pool, cpu_addr: *mut core::ffi::c_void, size: usize);
    pub fn cio_gp_dma_destroy(gp_dma: *mut gen_pool, dma_dev: *mut device);
    pub fn cio_gp_dma_create(dma_dev: *mut device, nr_pages: i32) -> *mut gen_pool;
    pub fn chsc_sstpc(page: *mut core::ffi::c_void, op: u32, ctrl: u16, clock_delta: *mut i64) -> i32;
    pub fn chsc_sstpi(page: *mut core::ffi::c_void, result: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn chsc_stzi(page: *mut core::ffi::c_void, result: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn chsc_sgib(origin: u32) -> i32;
    pub fn chsc_scud(cu: u16, esm: *mut u64, esm_valid: *mut u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
