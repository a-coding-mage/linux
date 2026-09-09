/* SPDX-License-Identifier: MIT */
/* Translated from xen-mca.h. */

pub const __HYPERVISOR_mca: u32 = __HYPERVISOR_arch_0;
pub const XEN_MCA_INTERFACE_VERSION: u32 = 0x01ecc003;
pub const XEN_MC_NONURGENT: u32 = 0x1;
pub const XEN_MC_URGENT: u32 = 0x2;
pub const XEN_MC_ACK: u32 = 0x4;
pub const XEN_MC_OK: u32 = 0x0;
pub const XEN_MC_FETCHFAILED: u32 = 0x1;
pub const XEN_MC_NODATA: u32 = 0x2;

pub const VIRQ_MCA: u32 = VIRQ_ARCH_0;
pub const MC_TYPE_GLOBAL: u16 = 0;
pub const MC_TYPE_BANK: u16 = 1;
pub const MC_TYPE_EXTENDED: u16 = 2;
pub const MC_TYPE_RECOVERY: u16 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_common { pub type_: u16, pub size: u16 }

pub const MC_FLAG_CORRECTABLE: u32 = 1 << 0;
pub const MC_FLAG_UNCORRECTABLE: u32 = 1 << 1;
pub const MC_FLAG_RECOVERABLE: u32 = 1 << 2;
pub const MC_FLAG_POLLED: u32 = 1 << 3;
pub const MC_FLAG_RESET: u32 = 1 << 4;
pub const MC_FLAG_CMCI: u32 = 1 << 5;
pub const MC_FLAG_MCE: u32 = 1 << 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_global { pub common: mcinfo_common, pub mc_domid: u16, pub mc_vcpuid: u16, pub mc_socketid: u32, pub mc_coreid: u16, pub mc_core_threadid: u16, pub mc_apicid: u32, pub mc_flags: u32, pub mc_gstatus: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_bank { pub common: mcinfo_common, pub mc_bank: u16, pub mc_domid: u16, pub mc_status: u64, pub mc_addr: u64, pub mc_misc: u64, pub mc_ctrl2: u64, pub mc_tsc: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_msr { pub reg: u64, pub value: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcinfo_extended { pub common: mcinfo_common, pub mc_msrs: u32, pub mc_msr: [mcinfo_msr; core::mem::size_of::<*const ()>() * 4] }

pub const REC_ACTION_RECOVERED: u32 = 0x1 << 0;
pub const REC_ACTION_NONE: u32 = 0x1 << 1;
pub const REC_ACTION_NEED_RESET: u32 = 0x1 << 2;
pub const MC_ACTION_PAGE_OFFLINE: u32 = 0x1 << 0;
pub const MC_ACTION_CPU_OFFLINE: u32 = 0x1 << 1;
pub const MC_ACTION_CACHE_SHRINK: u32 = 0x1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_offline_action { pub mfn: u64, pub status: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_offline_action { pub mc_socketid: u32, pub mc_coreid: u16, pub mc_core_threadid: u16 }

pub const MAX_UNION_SIZE: usize = 16;
#[repr(C)]
pub union mcinfo_recovery_action_info { pub page_retire: page_offline_action, pub cpu_offline: cpu_offline_action, pub pad: [u8; MAX_UNION_SIZE] }
#[repr(C)]
pub struct mcinfo_recovery { pub common: mcinfo_common, pub mc_bank: u16, pub action_flags: u8, pub action_types: u8, pub action_info: mcinfo_recovery_action_info }

pub const MCINFO_MAXSIZE: usize = 768;
#[repr(C)]
pub struct mc_info { pub mi_nentries: u32, pub flags: u32, pub mi_data: [u64; (MCINFO_MAXSIZE - 1) / 8] }

pub const __MC_MSR_ARRAYSIZE: usize = 8;
pub const __MC_NMSRS: usize = 1;
pub const MC_NCAPS: usize = 7;
#[repr(C)]
pub struct mcinfo_logical_cpu { pub mc_cpunr: u32, pub mc_chipid: u32, pub mc_coreid: u16, pub mc_threadid: u16, pub mc_apicid: u32, pub mc_clusterid: u32, pub mc_ncores: u32, pub mc_ncores_active: u32, pub mc_nthreads: u32, pub mc_cpuid_level: u32, pub mc_family: u32, pub mc_vendor: u32, pub mc_model: u32, pub mc_step: u32, pub mc_vendorid: [i8; 16], pub mc_brandid: [i8; 64], pub mc_cpu_caps: [u32; MC_NCAPS], pub mc_cache_size: u32, pub mc_cache_alignment: u32, pub mc_nmsrvals: u32, pub mc_msrvalues: [mcinfo_msr; __MC_MSR_ARRAYSIZE] }

#[inline]
pub unsafe fn x86_mcinfo_nentries(mi: *const mc_info) -> u32 { (*mi).mi_nentries }
#[inline]
pub unsafe fn x86_mcinfo_first(mi: *mut mc_info) -> *mut mcinfo_common { (*mi).mi_data.as_mut_ptr() as *mut mcinfo_common }
#[inline]
pub unsafe fn x86_mcinfo_next(mic: *mut mcinfo_common) -> *mut mcinfo_common { (mic as *mut u8).add((*mic).size as usize) as *mut mcinfo_common }
#[inline]
pub unsafe fn x86_mcinfo_lookup(ret: *mut *mut mcinfo_common, mi: *mut mc_info, type_: u16) {
    if ret.is_null() || mi.is_null() { return; }
    let mut mic = x86_mcinfo_first(mi);
    let mut found = false;
    for _ in 0..x86_mcinfo_nentries(mi) { if (*mic).type_ == type_ { found = true; break; } mic = x86_mcinfo_next(mic); }
    *ret = if found { mic } else { core::ptr::null_mut() };
}

pub const XEN_MC_fetch: u32 = 1;
#[repr(C)]
pub struct xen_mc_fetch { pub flags: u32, pub _pad0: u32, pub fetch_id: u64, pub data: GUEST_HANDLE_mc_info }
pub const XEN_MC_notifydomain: u32 = 2;
#[repr(C)]
pub struct xen_mc_notifydomain { pub mc_domid: u16, pub mc_vcpuid: u16, pub flags: u32 }
pub const XEN_MC_physcpuinfo: u32 = 3;
#[repr(C)]
pub struct xen_mc_physcpuinfo { pub ncpus: u32, pub _pad0: u32, pub info: GUEST_HANDLE_mcinfo_logical_cpu }
pub const XEN_MC_msrinject: u32 = 4;
pub const MC_MSRINJ_MAXMSRS: usize = 8;
#[repr(C)]
pub struct xen_mc_msrinject { pub mcinj_cpunr: u32, pub mcinj_flags: u32, pub mcinj_count: u32, pub _pad0: u32, pub mcinj_msr: [mcinfo_msr; MC_MSRINJ_MAXMSRS] }
pub const MC_MSRINJ_F_INTERPOSE: u32 = 0x1;
pub const XEN_MC_mceinject: u32 = 5;
#[repr(C)]
pub struct xen_mc_mceinject { pub mceinj_cpunr: core::ffi::c_uint }
#[repr(C)]
pub union xen_mc_u { pub mc_fetch: xen_mc_fetch, pub mc_notifydomain: xen_mc_notifydomain, pub mc_physcpuinfo: xen_mc_physcpuinfo, pub mc_msrinject: xen_mc_msrinject, pub mc_mceinject: xen_mc_mceinject }
#[repr(C)]
pub struct xen_mc { pub cmd: u32, pub interface_version: u32, pub u: xen_mc_u }

#[repr(C)]
pub struct xen_mce { pub status: u64, pub misc: u64, pub addr: u64, pub mcgstatus: u64, pub ip: u64, pub tsc: u64, pub time: u64, pub cpuvendor: u8, pub inject_flags: u8, pub pad: u16, pub cpuid: u32, pub cs: u8, pub bank: u8, pub cpu: u8, pub finished: u8, pub extcpu: u32, pub socketid: u32, pub apicid: u32, pub mcgcap: u64, pub synd: u64, pub ipid: u64, pub ppin: u64 }
pub const XEN_MCE_LOG_LEN: usize = 32;
#[repr(C)]
pub struct xen_mce_log { pub signature: [i8; 12], pub len: core::ffi::c_uint, pub next: core::ffi::c_uint, pub flags: core::ffi::c_uint, pub recordlen: core::ffi::c_uint, pub entry: [xen_mce; XEN_MCE_LOG_LEN] }
pub const XEN_MCE_OVERFLOW: u32 = 0;
pub const XEN_MCE_LOG_SIGNATURE: &str = "MACHINECHECK";
pub const MCE_GET_RECORD_LEN: u32 = _IOR('M' as u32, 1, core::mem::size_of::<i32>());
pub const MCE_GET_LOG_LEN: u32 = _IOR('M' as u32, 2, core::mem::size_of::<i32>());
pub const MCE_GETCLEAR_FLAGS: u32 = _IOR('M' as u32, 3, core::mem::size_of::<i32>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
