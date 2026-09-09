/* SPDX-License-Identifier: GPL-2.0 */
/* Functions for assembling fcx enabled I/O control blocks. */

/* Dependencies supplied by the surrounding translation unit. */

pub const TCW_FORMAT_DEFAULT: u32 = 0;
pub const TCW_TIDAW_FORMAT_DEFAULT: u32 = 0;
pub const TCW_FLAGS_INPUT_TIDA: u32 = 1 << (23 - 5);
pub const TCW_FLAGS_TCCB_TIDA: u32 = 1 << (23 - 6);
pub const TCW_FLAGS_OUTPUT_TIDA: u32 = 1 << (23 - 7);
#[inline]
pub const fn TCW_FLAGS_TIDAW_FORMAT(x: u32) -> u32 { (x & 3) << (23 - 9) }
#[inline]
pub const fn TCW_FLAGS_GET_TIDAW_FORMAT(x: u32) -> u32 { (x >> (23 - 9)) & 3 }

#[repr(C, packed, align(64))]
pub struct tcw {
    /* format:2, reserved:6, flags:24 */
    pub format_flags: u32,
    /* reserved:8, tccbl:6, r:1, w:1, reserved:16 */
    pub control: u32,
    pub output: dma64_t,
    pub input: dma64_t,
    pub tsb: dma64_t,
    pub tccb: dma64_t,
    pub output_count: u32,
    pub input_count: u32,
    pub reserved: [u32; 3],
    pub intrg: dma32_t,
}

pub const TIDAW_FLAGS_LAST: u32 = 1 << (7 - 0);
pub const TIDAW_FLAGS_SKIP: u32 = 1 << (7 - 1);
pub const TIDAW_FLAGS_DATA_INT: u32 = 1 << (7 - 2);
pub const TIDAW_FLAGS_TTIC: u32 = 1 << (7 - 3);
pub const TIDAW_FLAGS_INSERT_CBC: u32 = 1 << (7 - 4);

#[repr(C, packed, align(16))]
pub struct tidaw {
    pub flags: u32,
    pub count: u32,
    pub addr: dma64_t,
}

#[repr(C, packed)]
pub struct tsa_iostat {
    pub dev_time: u32,
    pub def_time: u32,
    pub queue_time: u32,
    pub dev_busy_time: u32,
    pub dev_act_time: u32,
    pub sense: [u8; 32],
}

#[repr(C, packed)]
pub struct tsa_ddpc {
    pub reserved: [u8; 3],
    pub rc: u8,
    pub rcq: [u8; 16],
    pub sense: [u8; 32],
}

pub const TSA_INTRG_FLAGS_CU_STATE_VALID: u32 = 1 << (7 - 0);
pub const TSA_INTRG_FLAGS_DEV_STATE_VALID: u32 = 1 << (7 - 1);
pub const TSA_INTRG_FLAGS_OP_STATE_VALID: u32 = 1 << (7 - 2);

#[repr(C, packed)]
pub struct tsa_intrg {
    pub format: u32,
    pub flags: u32,
    pub cu_state: u32,
    pub dev_state: u32,
    pub op_state: u32,
    pub reserved: u32,
    pub sd_info: [u8; 12],
    pub dl_id: u32,
    pub dd_data: [u8; 28],
}

pub const TSB_FORMAT_NONE: u32 = 0;
pub const TSB_FORMAT_IOSTAT: u32 = 1;
pub const TSB_FORMAT_DDPC: u32 = 2;
pub const TSB_FORMAT_INTRG: u32 = 3;
pub const TSB_FLAGS_DCW_OFFSET_VALID: u32 = 1 << (7 - 0);
pub const TSB_FLAGS_COUNT_VALID: u32 = 1 << (7 - 1);
pub const TSB_FLAGS_CACHE_MISS: u32 = 1 << (7 - 2);
pub const TSB_FLAGS_TIME_VALID: u32 = 1 << (7 - 3);
#[inline]
pub const fn TSB_FLAGS_FORMAT(x: u32) -> u32 { x & 7 }
#[inline]
pub unsafe fn TSB_FORMAT(t: *const tsb) -> u32 { (*t).flags & 7 }

#[repr(C, packed, align(8))]
pub union tsb_tsa {
    pub iostat: tsa_iostat,
    pub ddpc: tsa_ddpc,
    pub intrg: tsa_intrg,
}

#[repr(C, packed, align(8))]
pub struct tsb {
    pub length: u32,
    pub flags: u32,
    pub dcw_offset: u32,
    pub count: u32,
    pub reserved: u32,
    pub tsa: tsb_tsa,
}

pub const DCW_INTRG_FORMAT_DEFAULT: u32 = 0;
pub const DCW_INTRG_RC_UNSPECIFIED: u32 = 0;
pub const DCW_INTRG_RC_TIMEOUT: u32 = 1;
pub const DCW_INTRG_RCQ_UNSPECIFIED: u32 = 0;
pub const DCW_INTRG_RCQ_PRIMARY: u32 = 1;
pub const DCW_INTRG_RCQ_SECONDARY: u32 = 2;
pub const DCW_INTRG_FLAGS_MPM: u32 = 1 << (7 - 0);
pub const DCW_INTRG_FLAGS_PPR: u32 = 1 << (7 - 1);
pub const DCW_INTRG_FLAGS_CRIT: u32 = 1 << (7 - 2);

#[repr(C, packed)]
pub struct dcw_intrg_data {
    pub format: u32,
    pub rc: u32,
    pub rcq: u32,
    pub lpm: u32,
    pub pam: u32,
    pub pim: u32,
    pub timeout: u32,
    pub flags: u32,
    pub reserved: [u32; 2],
    pub time: u64,
    pub prog_id: u64,
    pub prog_data: [u8; 0],
}

pub const DCW_FLAGS_CC: u32 = 1 << (7 - 1);
pub const DCW_CMD_WRITE: u8 = 0x01;
pub const DCW_CMD_READ: u8 = 0x02;
pub const DCW_CMD_CONTROL: u8 = 0x03;
pub const DCW_CMD_SENSE: u8 = 0x04;
pub const DCW_CMD_SENSE_ID: u8 = 0xe4;
pub const DCW_CMD_INTRG: u8 = 0x40;

#[repr(C, packed)]
pub struct dcw {
    pub cmd: u32,
    pub flags: u32,
    pub reserved: u32,
    pub cd_count: u32,
    pub count: u32,
    pub cd: [u8; 0],
}

pub const TCCB_FORMAT_DEFAULT: u32 = 0x7f;
pub const TCCB_MAX_DCW: usize = 30;
pub const TCCB_SAC_DEFAULT: u32 = 0x1ffe;
pub const TCCB_SAC_INTRG: u32 = 0x1fff;

#[repr(C, packed)]
pub struct tccb_tcah {
    pub format: u32,
    pub reserved0: u32,
    pub tcal: u32,
    pub sac: u32,
    pub reserved1: u32,
}

#[repr(C, packed)]
pub struct tccb_tcat {
    pub reserved: u32,
    pub count: u32,
}

#[repr(C, packed, align(8))]
pub struct tccb {
    pub tcah: tccb_tcah,
    pub tca: [u8; 0],
}

extern "C" {
    pub fn tcw_get_intrg(tcw: *mut tcw) -> *mut tcw;
    pub fn tcw_get_data(tcw: *mut tcw) -> *mut core::ffi::c_void;
    pub fn tcw_get_tccb(tcw: *mut tcw) -> *mut tccb;
    pub fn tcw_get_tsb(tcw: *mut tcw) -> *mut tsb;
    pub fn tcw_init(tcw: *mut tcw, r: i32, w: i32);
    pub fn tcw_finalize(tcw: *mut tcw, num_tidaws: i32);
    pub fn tcw_set_intrg(tcw: *mut tcw, intrg_tcw: *mut tcw);
    pub fn tcw_set_data(tcw: *mut tcw, data: *mut core::ffi::c_void, use_tidal: i32);
    pub fn tcw_set_tccb(tcw: *mut tcw, tccb: *mut tccb);
    pub fn tcw_set_tsb(tcw: *mut tcw, tsb: *mut tsb);
    pub fn tccb_init(tccb: *mut tccb, tccb_size: usize, sac: u32);
    pub fn tsb_init(tsb: *mut tsb);
    pub fn tccb_add_dcw(tccb: *mut tccb, tccb_size: usize, cmd: u8, flags: u8,
                        cd: *mut core::ffi::c_void, cd_count: u8, count: u32) -> *mut dcw;
    pub fn tcw_add_tidaw(tcw: *mut tcw, num_tidaws: i32, flags: u8,
                         addr: *mut core::ffi::c_void, count: u32) -> *mut tidaw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
