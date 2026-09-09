/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The included Linux and s390 types are supplied
// by the surrounding translation unit.

#[repr(C, packed)]
pub struct arqb {
    pub data: u64,
    // fmt:4, unnamed:12
    pub fmt_reserved: u16,
    pub cmd_code: u16,
    pub reserved_16: u16,
    pub msb_count: u16,
    pub reserved: [u32; 12],
}

pub const ARQB_CMD_MOVE: u16 = 1;

#[repr(C, packed)]
pub struct arsb {
    // fmt:4, unnamed:28
    pub fmt_reserved: u32,
    pub ef: u8,
    pub reserved_ef: u8,
    pub ecbi: u8,
    pub reserved_ecbi: u8,
    pub fvf: u8,
    pub reserved_fvf: u16,
    pub eqc: u8,
    pub reserved_eqc: u32,
    pub fail_msb: u64,
    pub fail_aidaw: u64,
    pub fail_ms: u64,
    pub fail_scm: u64,
    pub reserved: [u32; 4],
}

pub const EQC_WR_PROHIBIT: u8 = 22;

#[repr(C, packed)]
pub struct msb {
    // fmt:4, oc:4
    pub fmt_oc: u8,
    pub flags: u8,
    // unnamed:12, bs:4
    pub reserved_bs: u16,
    pub blk_count: u32,
    pub data_addr: dma64_t,
    pub scm_addr: u64,
    pub reserved: u64,
}

#[repr(C, packed)]
pub struct aidaw {
    pub flags: u8,
    pub reserved_24: u32,
    pub reserved_32: u32,
    pub data_addr: dma64_t,
}

pub const MSB_OC_CLEAR: u8 = 0;
pub const MSB_OC_READ: u8 = 1;
pub const MSB_OC_WRITE: u8 = 2;
pub const MSB_OC_RELEASE: u8 = 3;

pub const MSB_FLAG_BNM: u8 = 0x80;
pub const MSB_FLAG_IDA: u8 = 0x40;

pub const MSB_BS_4K: u16 = 0;
pub const MSB_BS_1M: u16 = 1;

pub const AOB_NR_MSB: usize = 124;

#[repr(C, packed, align(4096))]
pub struct aob {
    pub request: arqb,
    pub response: arsb,
    pub msb: [msb; AOB_NR_MSB],
}

#[repr(C)]
pub struct aob_rq_header {
    pub scmdev: *mut scm_device,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct scm_device {
    pub address: u64,
    pub size: u64,
    pub nr_max_block: ::core::ffi::c_uint,
    pub dev: device,
    pub attrs: scm_device_attrs,
}

#[repr(C, packed)]
pub struct scm_device_attrs {
    // persistence:4, oper_state:4, data_state:4, rank:4,
    // release:1, res_id:8 (remaining implementation-defined padding omitted)
    pub persistence_oper_state_data_state_rank: u16,
    pub release_res_id: u16,
}

pub const OP_STATE_GOOD: ::core::ffi::c_int = 1;
pub const OP_STATE_TEMP_ERR: ::core::ffi::c_int = 2;
pub const OP_STATE_PERM_ERR: ::core::ffi::c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum scm_event {
    SCM_CHANGE,
    SCM_AVAIL,
}

#[repr(C)]
pub struct scm_driver {
    pub drv: device_driver,
    pub probe: Option<unsafe extern "C" fn(scmdev: *mut scm_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(scmdev: *mut scm_device)>,
    pub notify: Option<unsafe extern "C" fn(scmdev: *mut scm_device, event: scm_event)>,
    pub handler: Option<unsafe extern "C" fn(
        scmdev: *mut scm_device,
        data: *mut ::core::ffi::c_void,
        error: blk_status_t,
    )>,
}

unsafe extern "C" {
    pub fn scm_driver_register(scmdrv: *mut scm_driver) -> ::core::ffi::c_int;
    pub fn scm_driver_unregister(scmdrv: *mut scm_driver);
    pub fn eadm_start_aob(aob: *mut aob) -> ::core::ffi::c_int;
    pub fn scm_irq_handler(aob: *mut aob, error: blk_status_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
