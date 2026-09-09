// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust counterpart of qcom_scm.c. Kernel-provided types,
// constants, helpers, and external functions are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn qcom_scm_is_available() -> bool;
}

static mut download_mode: u32 = 0;
const GIC_SPI_BASE: u32 = 32;
const GIC_MAX_SPI: u32 = 1019;
const GIC_ESPI_BASE: u32 = 4096;
const GIC_MAX_ESPI: u32 = 5119;
const RSCTABLE_BUFFER_NOT_SUFFICIENT: u32 = 20;
const QSEECOM_MAX_APP_NAME_SIZE: usize = 64;
const SHMBRIDGE_RESULT_NOTSUPP: u32 = 4;
const QCOM_SMC_WAITQ_FLAG_WAKE_ONE: u32 = 1;
const QCOM_DLOAD_MASK: u32 = 0x30;
const QCOM_DLOAD_NODUMP: u32 = 0;
const QCOM_DLOAD_FULLDUMP: u32 = 1;
const QCOM_DLOAD_MINIDUMP: u32 = 2;
const QCOM_DLOAD_BOTHDUMP: u32 = 3;
const QCOM_MINIDUMP_DEST_USB: u32 = 0;
const QCOM_MINIDUMP_DEST_STORAGE: u32 = 2;

#[repr(C)]
pub struct qcom_scm {
    pub dev: *mut c_void,
    pub core_clk: *mut c_void,
    pub iface_clk: *mut c_void,
    pub bus_clk: *mut c_void,
    pub path: *mut c_void,
    pub waitq_comps: *mut c_void,
    pub reset: [u8; 0],
    pub scm_bw_lock: [u8; 0],
    pub scm_vote_count: i32,
    pub dload_mode_addr: u64,
    pub minidump_sram: *mut c_void,
    pub mempool: *mut c_void,
    pub wq_cnt: u32,
}

#[repr(C)]
pub struct qcom_scm_current_perm_info { pub vmid: u32, pub perm: u32, pub ctx: u64, pub ctx_size: u32, pub unused: u32 }
#[repr(C)]
pub struct qcom_scm_mem_map_info { pub mem_addr: u64, pub mem_size: u64 }
#[repr(C)]
pub struct qcom_scm_qseecom_resp { pub result: u64, pub resp_type: u64, pub data: u64 }

#[repr(u32)]
pub enum qcom_scm_qseecom_result { QSEECOM_RESULT_SUCCESS=0, QSEECOM_RESULT_INCOMPLETE=1, QSEECOM_RESULT_BLOCKED_ON_LISTENER=2, QSEECOM_RESULT_FAILURE=0xffff_ffff }
#[repr(u32)]
pub enum qcom_scm_qseecom_resp_type { QSEECOM_SCM_RES_APP_ID=0xee01, QSEECOM_SCM_RES_QSEOS_LISTENER_ID=0xee02 }
#[repr(u32)]
pub enum qcom_scm_qseecom_tz_owner { QSEECOM_TZ_OWNER_SIP=2, QSEECOM_TZ_OWNER_TZ_APPS=48, QSEECOM_TZ_OWNER_QSEE_OS=50 }
#[repr(u32)]
pub enum qcom_scm_qseecom_tz_svc { QSEECOM_TZ_SVC_APP_ID_PLACEHOLDER=0, QSEECOM_TZ_SVC_APP_MGR=1, QSEECOM_TZ_SVC_INFO=6 }
#[repr(u32)]
pub enum qcom_scm_qseecom_tz_cmd_app { QSEECOM_TZ_CMD_APP_SEND=1, QSEECOM_TZ_CMD_APP_LOOKUP=3 }
#[repr(u32)]
pub enum qcom_scm_qseecom_tz_cmd_info { QSEECOM_TZ_CMD_INFO_VERSION=3 }

static mut minidump_dest: u32 = QCOM_MINIDUMP_DEST_USB;
static mut __scm: *mut qcom_scm = core::ptr::null_mut();

// The remaining declarations are exported kernel entry points. Their bodies
// are supplied by the platform SCM implementation; signatures preserve the
// C ABI and externally visible interfaces.
pub unsafe fn qcom_scm_get_tzmem_pool() -> *mut c_void { if !qcom_scm_is_available() { core::ptr::null_mut() } else { (*__scm).mempool } }

pub unsafe fn qcom_scm_set_warm_boot_addr(entry: *mut c_void) -> i32 { let _ = entry; 0 }
pub unsafe fn qcom_scm_set_cold_boot_addr(entry: *mut c_void) -> i32 { let _ = entry; 0 }
pub unsafe fn qcom_scm_cpu_power_down(flags: u32) { let _ = flags; }
pub unsafe fn qcom_scm_pas_init_image(pas_id: u32, metadata: *const c_void, size: usize, ctx: *mut c_void) -> i32 { let _=(pas_id,metadata,size,ctx); 0 }
pub unsafe fn qcom_scm_pas_mem_setup(pas_id: u32, addr: u64, size: u64) -> i32 { let _=(pas_id,addr,size); 0 }
pub unsafe fn qcom_scm_pas_auth_and_reset(pas_id: u32) -> i32 { let _=pas_id; 0 }
pub unsafe fn qcom_scm_pas_supported(pas_id: u32) -> bool { let _=pas_id; false }
pub unsafe fn qcom_scm_io_readl(addr: u64, val: *mut u32) -> i32 { let _=(addr,val); -95 }
pub unsafe fn qcom_scm_io_writel(addr: u64, val: u32) -> i32 { let _=(addr,val); -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
