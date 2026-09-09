/* SPDX-License-Identifier: GPL-2.0 */
/*
 * s390 (re)ipl support
 *
 * Copyright IBM Corp. 2007
 */

// Dependencies supplied by the corresponding low-level headers are intentionally
// left external to this translation.

#[repr(C)]
pub struct ipl_parameter_block {
    pub hdr: ipl_pl_hdr,
    pub data: ipl_parameter_block_data,
}

#[repr(C)]
pub union ipl_parameter_block_data {
    pub pb0_hdr: std::mem::ManuallyDrop<ipl_pb_hdr>,
    pub common: std::mem::ManuallyDrop<ipl_pb0_common>,
    pub fcp: std::mem::ManuallyDrop<ipl_pb0_fcp>,
    pub ccw: std::mem::ManuallyDrop<ipl_pb0_ccw>,
    pub eckd: std::mem::ManuallyDrop<ipl_pb0_eckd>,
    pub nvme: std::mem::ManuallyDrop<ipl_pb0_nvme>,
    pub raw: [std::ffi::c_char; PAGE_SIZE - std::mem::size_of::<ipl_pl_hdr>()],
}

// C: __packed __aligned(PAGE_SIZE)
// The containing declaration must be given the ABI alignment PAGE_SIZE by the
// target integration, while the fields above preserve the C representation.

pub const NSS_NAME_SIZE: usize = 8;

pub const IPL_BP_FCP_LEN: usize =
    std::mem::size_of::<ipl_pl_hdr>() + std::mem::size_of::<ipl_pb0_fcp>();
pub const IPL_BP0_FCP_LEN: usize = std::mem::size_of::<ipl_pb0_fcp>();

pub const IPL_BP_NVME_LEN: usize =
    std::mem::size_of::<ipl_pl_hdr>() + std::mem::size_of::<ipl_pb0_nvme>();
pub const IPL_BP0_NVME_LEN: usize = std::mem::size_of::<ipl_pb0_nvme>();

pub const IPL_BP_CCW_LEN: usize =
    std::mem::size_of::<ipl_pl_hdr>() + std::mem::size_of::<ipl_pb0_ccw>();
pub const IPL_BP0_CCW_LEN: usize = std::mem::size_of::<ipl_pb0_ccw>();

pub const IPL_BP_ECKD_LEN: usize =
    std::mem::size_of::<ipl_pl_hdr>() + std::mem::size_of::<ipl_pb0_eckd>();
pub const IPL_BP0_ECKD_LEN: usize = std::mem::size_of::<ipl_pb0_eckd>();

pub const IPL_MAX_SUPPORTED_VERSION: i32 = 0;
pub const IPL_RB_CERT_UNKNOWN: u16 = (-1i16) as u16;

pub const DIAG308_VMPARM_SIZE: usize = 64;
pub const DIAG308_SCPDATA_OFFSET: usize =
    std::mem::offset_of!(ipl_parameter_block, data);
pub const DIAG308_SCPDATA_SIZE: usize = PAGE_SIZE - DIAG308_SCPDATA_OFFSET;

pub struct save_area;

extern "C" {
    pub fn save_area_alloc(is_boot_cpu: bool) -> *mut save_area;
    pub fn save_area_boot_cpu() -> *mut save_area;
    pub fn save_area_add_regs(area: *mut save_area, regs: *mut std::ffi::c_void);
    pub fn save_area_add_vxrs(area: *mut save_area, vxrs: *mut __vector128);

    pub fn s390_reset_system();
    pub fn ipl_block_get_ascii_vmparm(
        dest: *mut std::ffi::c_char,
        size: usize,
        ipb: *const ipl_parameter_block,
    ) -> usize;
}

#[repr(i32)]
pub enum ipl_type {
    IPL_TYPE_UNKNOWN = 1,
    IPL_TYPE_CCW = 2,
    IPL_TYPE_FCP = 4,
    IPL_TYPE_FCP_DUMP = 8,
    IPL_TYPE_NSS = 16,
    IPL_TYPE_NVME = 32,
    IPL_TYPE_NVME_DUMP = 64,
    IPL_TYPE_ECKD = 128,
    IPL_TYPE_ECKD_DUMP = 256,
}

#[repr(C)]
pub struct ipl_info {
    pub type_: ipl_type,
    pub data: ipl_info_data,
}

#[repr(C)]
pub union ipl_info_data {
    pub ccw: std::mem::ManuallyDrop<ipl_info_ccw>,
    pub eckd: std::mem::ManuallyDrop<ipl_info_eckd>,
    pub fcp: std::mem::ManuallyDrop<ipl_info_fcp>,
    pub nvme: std::mem::ManuallyDrop<ipl_info_nvme>,
    pub nss: std::mem::ManuallyDrop<ipl_info_nss>,
}

#[repr(C)]
pub struct ipl_info_ccw { pub dev_id: ccw_dev_id }
#[repr(C)]
pub struct ipl_info_eckd { pub dev_id: ccw_dev_id }
#[repr(C)]
pub struct ipl_info_fcp { pub dev_id: ccw_dev_id, pub wwpn: u64, pub lun: u64 }
#[repr(C)]
pub struct ipl_info_nvme { pub fid: u32, pub nsid: u32 }
#[repr(C)]
pub struct ipl_info_nss { pub name: [std::ffi::c_char; NSS_NAME_SIZE + 1] }

extern "C" {
    pub static mut ipl_info: ipl_info;
    pub fn setup_ipl();
    pub fn set_os_info_reipl_block();
}

#[inline]
pub unsafe fn is_ipl_type_dump() -> bool {
    ((*std::ptr::addr_of!(ipl_info)).type_ as i32 == 8)
        || ((*std::ptr::addr_of!(ipl_info)).type_ as i32 == 256)
        || ((*std::ptr::addr_of!(ipl_info)).type_ as i32 == 64)
}

#[repr(C)]
pub struct ipl_report {
    pub ipib: *mut ipl_parameter_block,
    pub components: list_head,
    pub certificates: list_head,
    pub size: usize,
}

#[repr(C)]
pub struct ipl_report_component {
    pub list: list_head,
    pub entry: ipl_rb_component_entry,
}

#[repr(C)]
pub struct ipl_report_certificate {
    pub list: list_head,
    pub entry: ipl_rb_certificate_entry,
    pub key: *mut std::ffi::c_void,
}

pub struct kexec_buf;

extern "C" {
    pub fn ipl_report_init(ipib: *mut ipl_parameter_block) -> *mut ipl_report;
    pub fn ipl_report_finish(report: *mut ipl_report) -> *mut std::ffi::c_void;
    pub fn ipl_report_free(report: *mut ipl_report) -> i32;
    pub fn ipl_report_add_component(
        report: *mut ipl_report,
        kbuf: *mut kexec_buf,
        flags: u8,
        cert: u16,
    ) -> i32;
    pub fn ipl_report_add_certificate(
        report: *mut ipl_report,
        key: *mut std::ffi::c_void,
        addr: c_ulong,
        len: c_ulong,
    ) -> i32;
}

/* DIAG 308 support */
#[repr(i32)]
pub enum diag308_subcode {
    DIAG308_CLEAR_RESET = 0,
    DIAG308_LOAD_NORMAL_RESET = 1,
    DIAG308_REL_HSA = 2,
    DIAG308_LOAD_CLEAR = 3,
    DIAG308_LOAD_NORMAL_DUMP = 4,
    DIAG308_SET = 5,
    DIAG308_STORE = 6,
    DIAG308_LOAD_NORMAL = 7,
}

pub const DIAG308_FLAG_EI: c_ulong = 1u64 << 16;

#[repr(i32)]
pub enum diag308_rc {
    DIAG308_RC_OK = 0x0001,
    DIAG308_RC_NOCONFIG = 0x0102,
}

extern "C" {
    pub fn diag308(subcode: c_ulong, addr: *mut std::ffi::c_void) -> i32;
    pub fn store_status(fn_: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>, data: *mut std::ffi::c_void);
    pub fn lgr_info_log();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
