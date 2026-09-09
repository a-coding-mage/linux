/* SPDX-License-Identifier: GPL-2.0 */
/* Linux-specific, architecture-independent Hyper-V definitions. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

pub const VTPM_BASE_ADDRESS: u32 = 0xfed40000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HvPartitionType {
    Guest,
    Root,
    L1Vh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MsHypervInfo {
    pub features: u32,
    pub priv_high: u32,
    pub ext_features: u32,
    pub misc_features: u32,
    pub hints: u32,
    pub nested_features: u32,
    pub max_vp_index: u32,
    pub max_lp_index: u32,
    pub vtl: u8,
    pub isolation_config_a: u32,
    pub isolation_config_b: u32,
    pub shared_gpa_boundary: u64,
    pub msi_ext_dest_id: bool,
    pub confidential_vmbus_available: bool,
}

extern "C" {
    pub static mut ms_hyperv: MsHypervInfo;
    pub static mut hv_nested: bool;
    pub static mut hv_current_partition_id: u64;
    pub static mut hv_curr_partition_type: HvPartitionType;
    pub static mut hyperv_pcpu_input_arg: *mut *mut c_void;
    pub static mut hyperv_pcpu_output_arg: *mut *mut c_void;

    pub fn hv_do_hypercall(control: u64, inputaddr: *mut c_void, outputaddr: *mut c_void) -> u64;
    pub fn hv_do_fast_hypercall8(control: u16, input8: u64) -> u64;
    pub fn hv_do_fast_hypercall16(control: u16, input1: u64, input2: u64) -> u64;
    pub fn hv_isolation_type_snp() -> bool;
    pub fn hv_isolation_type_tdx() -> bool;
    pub fn node_to_pxm(node: c_int) -> c_int;
    pub fn touch_nmi_watchdog();
}

#[inline]
pub unsafe fn hv_recommend_using_aeoi() -> bool {
    /* HV_DEPRECATING_AEOI_RECOMMENDED is supplied by the build environment. */
    #[cfg(feature = "HV_DEPRECATING_AEOI_RECOMMENDED")]
    { return (ms_hyperv.hints & HV_DEPRECATING_AEOI_RECOMMENDED) == 0; }
    #[cfg(not(feature = "HV_DEPRECATING_AEOI_RECOMMENDED"))]
    { false }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct HvProximityDomainInfo {
    pub domain_id: c_int,
    pub flags: HvProximityDomainFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct HvProximityDomainFlags { pub proximity_info_valid: u8, pub proximity_preferred: u8 }

pub const NUMA_NO_NODE: c_int = -1;
pub unsafe fn hv_numa_node_to_pxm_info(node: c_int) -> HvProximityDomainInfo {
    let mut pxm_info = HvProximityDomainInfo::default();
    if node != NUMA_NO_NODE {
        pxm_info.domain_id = node_to_pxm(node);
        pxm_info.flags.proximity_info_valid = 1;
        pxm_info.flags.proximity_preferred = 1;
    }
    pxm_info
}

#[inline] pub const fn hv_result(status: u64) -> c_int { (status & HV_HYPERCALL_RESULT_MASK) as c_int }
#[inline] pub fn hv_result_success(status: u64) -> bool { hv_result(status) == HV_STATUS_SUCCESS as c_int }
#[inline] pub const fn hv_repcomp(status: u64) -> c_uint { ((status & HV_HYPERCALL_REP_COMP_MASK) >> HV_HYPERCALL_REP_COMP_OFFSET) as c_uint }

pub unsafe fn hv_do_rep_hypercall_ex(code: u16, rep_count: u16, varhead_size: u16, rep_start: u16, input: *mut c_void, output: *mut c_void) -> u64 {
    let mut control = code as u64;
    control |= (varhead_size as u64) << HV_HYPERCALL_VARHEAD_OFFSET;
    control |= (rep_count as u64) << HV_HYPERCALL_REP_COMP_OFFSET;
    control |= (rep_start as u64) << HV_HYPERCALL_REP_START_OFFSET;
    let mut status;
    let mut rep_comp;
    loop {
        status = hv_do_hypercall(control, input, output);
        if !hv_result_success(status) { return status; }
        rep_comp = hv_repcomp(status);
        control &= !HV_HYPERCALL_REP_START_MASK;
        control |= (rep_comp as u64) << HV_HYPERCALL_REP_START_OFFSET;
        touch_nmi_watchdog();
        if rep_comp >= rep_count as c_uint { break; }
    }
    status
}

pub unsafe fn hv_do_rep_hypercall(code: u16, rep_count: u16, varhead_size: u16, input: *mut c_void, output: *mut c_void) -> u64 {
    hv_do_rep_hypercall_ex(code, rep_count, varhead_size, 0, input, output)
}

#[inline] pub const fn hv_generate_guest_id(kernel_version: u64) -> u64 {
    ((HV_LINUX_VENDOR_ID as u64) << 48) | (kernel_version << 16)
}

extern "C" {
    pub fn hv_get_hypervisor_version(info: *mut HvHypervisorVersionInfo) -> c_int;
    pub fn hv_setup_vmbus_handler(handler: Option<unsafe extern "C" fn()>);
    pub fn hv_remove_vmbus_handler();
    pub fn hv_setup_stimer0_handler(handler: Option<unsafe extern "C" fn()>);
    pub fn hv_remove_stimer0_handler();
    pub fn hv_setup_kexec_handler(handler: Option<unsafe extern "C" fn()>);
    pub fn hv_remove_kexec_handler();
    pub fn hv_setup_crash_handler(handler: Option<unsafe extern "C" fn(*mut PtRegs)>);
    pub fn hv_remove_crash_handler();
    pub fn hv_setup_mshv_handler(handler: Option<unsafe extern "C" fn()>);
}

#[repr(C)] pub struct HvHypervisorVersionInfo { _private: [u8; 0] }
#[repr(C)] pub struct PtRegs { _private: [u8; 0] }

/* The following declarations are active when CONFIG_HYPERV is enabled. */
#[cfg(feature = "CONFIG_HYPERV")]
extern "C" {
    pub static mut hv_vp_index: *mut u32;
    pub static mut hv_max_vp_index: u32;
    pub static mut hv_read_reference_counter: Option<unsafe extern "C" fn() -> u64>;
    pub fn hv_common_init() -> c_int;
    pub fn hv_get_partition_id();
    pub fn hv_common_free();
    pub fn ms_hyperv_late_init();
    pub fn hv_common_cpu_init(cpu: c_uint) -> c_int;
    pub fn hv_common_cpu_die(cpu: c_uint) -> c_int;
    pub fn hv_identify_partition_type();
}
pub const VP_INVAL: u32 = u32::MAX;

#[cfg(feature = "CONFIG_HYPERV")]
pub unsafe fn hv_cpu_number_to_vp_number(cpu_number: c_int) -> c_int { *hv_vp_index.offset(cpu_number as isize) as c_int }

/* Configuration-dependent kernel declarations and logging macros remain external. */
#[cfg(feature = "CONFIG_HYPERV")]
extern "C" {
    pub fn hv_result_to_string(hv_status: u64) -> *const c_char;
    pub fn hv_result_to_errno(status: u64) -> c_int;
    pub fn hyperv_report_panic(regs: *mut PtRegs, err: c_long, in_die: bool);
    pub fn hv_is_hyperv_initialized() -> bool;
    pub fn hv_is_hibernation_supported() -> bool;
    pub fn hv_is_isolation_supported() -> bool;
    pub fn hv_ghcb_hypercall(control: u64, input: *mut c_void, output: *mut c_void, input_size: u32) -> u64;
    pub fn hv_tdx_hypercall(control: u64, param1: u64, param2: u64) -> u64;
    pub fn hv_enable_coco_interrupt(cpu: c_uint, vector: c_uint, set: bool);
    pub fn hv_para_set_sint_proxy(enable: bool);
    pub fn hv_para_get_synic_register(reg: c_uint) -> u64;
    pub fn hv_para_set_synic_register(reg: c_uint, val: u64);
    pub fn hyperv_cleanup();
    pub fn hv_query_ext_cap(cap_query: u64) -> bool;
}

#[cfg(not(feature = "CONFIG_HYPERV"))]
pub fn hv_is_hyperv_initialized() -> bool { false }
#[cfg(not(feature = "CONFIG_HYPERV"))]
pub fn hv_is_hibernation_supported() -> bool { false }
#[cfg(not(feature = "CONFIG_HYPERV"))]
pub fn hv_is_isolation_supported() -> bool { false }
#[cfg(not(feature = "CONFIG_HYPERV"))]
pub fn hyperv_cleanup() {}

/* CONFIG_MSHV_ROOT declarations/fallbacks are supplied by the kernel build. */
#[cfg(feature = "CONFIG_MSHV_ROOT")]
pub unsafe fn hv_root_partition() -> bool { hv_curr_partition_type as u32 == HvPartitionType::Root as u32 }
#[cfg(feature = "CONFIG_MSHV_ROOT")]
pub unsafe fn hv_l1vh_partition() -> bool { hv_curr_partition_type as u32 == HvPartitionType::L1Vh as u32 }
#[cfg(feature = "CONFIG_MSHV_ROOT")]
pub unsafe fn hv_parent_partition() -> bool { hv_root_partition() || hv_l1vh_partition() }

#[cfg(not(feature = "CONFIG_MSHV_ROOT"))]
pub fn hv_root_partition() -> bool { false }
#[cfg(not(feature = "CONFIG_MSHV_ROOT"))]
pub fn hv_l1vh_partition() -> bool { false }
#[cfg(not(feature = "CONFIG_MSHV_ROOT"))]
pub fn hv_parent_partition() -> bool { false }

#[cfg(feature = "CONFIG_HYPERV_VTL_MODE")]
extern "C" { pub fn get_vtl() -> u8; }
#[cfg(not(feature = "CONFIG_HYPERV_VTL_MODE"))]
pub fn get_vtl() -> u8 { 0 }

/* External Hyper-V constants/types referenced above are provided by hyperv/hvhdk.h. */
extern "C" {
    static _hv_header_external_symbols: c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
