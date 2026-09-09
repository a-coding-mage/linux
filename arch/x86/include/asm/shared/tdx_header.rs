/* SPDX-License-Identifier: GPL-2.0 */

pub const TDX_HYPERCALL_STANDARD: u64 = 0;

pub const TDX_CPUID_LEAF_ID: u64 = 0x21;
pub const TDX_IDENT: &str = "IntelTDX    ";

/* TDX module Call Leaf IDs */
pub const TDG_VP_VMCALL: u64 = 0;
pub const TDG_VP_INFO: u64 = 1;
pub const TDG_MR_RTMR_EXTEND: u64 = 2;
pub const TDG_VP_VEINFO_GET: u64 = 3;
pub const TDG_MR_REPORT: u64 = 4;
pub const TDG_MEM_PAGE_ACCEPT: u64 = 6;
pub const TDG_VM_RD: u64 = 7;
pub const TDG_VM_WR: u64 = 8;

/* TDX TD attributes */
pub const TDX_TD_ATTR_DEBUG_BIT: u32 = 0;
pub const TDX_TD_ATTR_DEBUG: u64 = 1u64 << TDX_TD_ATTR_DEBUG_BIT;
pub const TDX_TD_ATTR_HGS_PLUS_PROF_BIT: u32 = 4;
pub const TDX_TD_ATTR_HGS_PLUS_PROF: u64 = 1u64 << TDX_TD_ATTR_HGS_PLUS_PROF_BIT;
pub const TDX_TD_ATTR_PERF_PROF_BIT: u32 = 5;
pub const TDX_TD_ATTR_PERF_PROF: u64 = 1u64 << TDX_TD_ATTR_PERF_PROF_BIT;
pub const TDX_TD_ATTR_PMT_PROF_BIT: u32 = 6;
pub const TDX_TD_ATTR_PMT_PROF: u64 = 1u64 << TDX_TD_ATTR_PMT_PROF_BIT;
pub const TDX_TD_ATTR_ICSSD_BIT: u32 = 16;
pub const TDX_TD_ATTR_ICSSD: u64 = 1u64 << TDX_TD_ATTR_ICSSD_BIT;
pub const TDX_TD_ATTR_LASS_BIT: u32 = 27;
pub const TDX_TD_ATTR_LASS: u64 = 1u64 << TDX_TD_ATTR_LASS_BIT;
pub const TDX_TD_ATTR_SEPT_VE_DISABLE_BIT: u32 = 28;
pub const TDX_TD_ATTR_SEPT_VE_DISABLE: u64 = 1u64 << TDX_TD_ATTR_SEPT_VE_DISABLE_BIT;
pub const TDX_TD_ATTR_MIGRATABLE_BIT: u32 = 29;
pub const TDX_TD_ATTR_MIGRATABLE: u64 = 1u64 << TDX_TD_ATTR_MIGRATABLE_BIT;
pub const TDX_TD_ATTR_PKS_BIT: u32 = 30;
pub const TDX_TD_ATTR_PKS: u64 = 1u64 << TDX_TD_ATTR_PKS_BIT;
pub const TDX_TD_ATTR_KL_BIT: u32 = 31;
pub const TDX_TD_ATTR_KL: u64 = 1u64 << TDX_TD_ATTR_KL_BIT;
pub const TDX_TD_ATTR_TPA_BIT: u32 = 62;
pub const TDX_TD_ATTR_TPA: u64 = 1u64 << TDX_TD_ATTR_TPA_BIT;
pub const TDX_TD_ATTR_PERFMON_BIT: u32 = 63;
pub const TDX_TD_ATTR_PERFMON: u64 = 1u64 << TDX_TD_ATTR_PERFMON_BIT;

/* TDX TD-Scope Metadata. To be used by TDG.VM.WR and TDG.VM.RD */
pub const TDCS_CONFIG_FLAGS: u64 = 0x1110000300000016;
pub const TDCS_TD_CTLS: u64 = 0x1110000300000017;
pub const TDCS_NOTIFY_ENABLES: u64 = 0x9100000000000010;
pub const TDCS_TOPOLOGY_ENUM_CONFIGURED: u64 = 0x9100000000000019;

/* TDCS_CONFIG_FLAGS bits */
pub const TDCS_CONFIG_FLEXIBLE_PENDING_VE: u64 = 1u64 << 1;

/* TDCS_TD_CTLS bits */
pub const TD_CTLS_PENDING_VE_DISABLE_BIT: u32 = 0;
pub const TD_CTLS_PENDING_VE_DISABLE: u64 = 1u64 << TD_CTLS_PENDING_VE_DISABLE_BIT;
pub const TD_CTLS_ENUM_TOPOLOGY_BIT: u32 = 1;
pub const TD_CTLS_ENUM_TOPOLOGY: u64 = 1u64 << TD_CTLS_ENUM_TOPOLOGY_BIT;
pub const TD_CTLS_VIRT_CPUID2_BIT: u32 = 2;
pub const TD_CTLS_VIRT_CPUID2: u64 = 1u64 << TD_CTLS_VIRT_CPUID2_BIT;
pub const TD_CTLS_REDUCE_VE_BIT: u32 = 3;
pub const TD_CTLS_REDUCE_VE: u64 = 1u64 << TD_CTLS_REDUCE_VE_BIT;
pub const TD_CTLS_LOCK_BIT: u32 = 63;
pub const TD_CTLS_LOCK: u64 = 1u64 << TD_CTLS_LOCK_BIT;

/* TDX hypercall Leaf IDs */
pub const TDVMCALL_GET_TD_VM_CALL_INFO: u64 = 0x10000;
pub const TDVMCALL_MAP_GPA: u64 = 0x10001;
pub const TDVMCALL_GET_QUOTE: u64 = 0x10002;
pub const TDVMCALL_REPORT_FATAL_ERROR: u64 = 0x10003;
pub const TDVMCALL_SETUP_EVENT_NOTIFY_INTERRUPT: u64 = 0x10004;

/* TDG.VP.VMCALL Status Codes (returned in R10) */
pub const TDVMCALL_STATUS_SUCCESS: u64 = 0x0000000000000000;
pub const TDVMCALL_STATUS_RETRY: u64 = 0x0000000000000001;
pub const TDVMCALL_STATUS_INVALID_OPERAND: u64 = 0x8000000000000000;
pub const TDVMCALL_STATUS_ALIGN_ERROR: u64 = 0x8000000000000002;
pub const TDVMCALL_STATUS_SUBFUNC_UNSUPPORTED: u64 = 0x8000000000000003;

/* Bitmasks of exposed registers (with VMM). */
pub const TDX_RDX: u64 = 1u64 << 2;
pub const TDX_RBX: u64 = 1u64 << 3;
pub const TDX_RSI: u64 = 1u64 << 6;
pub const TDX_RDI: u64 = 1u64 << 7;
pub const TDX_R8: u64 = 1u64 << 8;
pub const TDX_R9: u64 = 1u64 << 9;
pub const TDX_R10: u64 = 1u64 << 10;
pub const TDX_R11: u64 = 1u64 << 11;
pub const TDX_R12: u64 = 1u64 << 12;
pub const TDX_R13: u64 = 1u64 << 13;
pub const TDX_R14: u64 = 1u64 << 14;
pub const TDX_R15: u64 = 1u64 << 15;

/* These registers are clobbered to hold arguments for each TDVMCALL. */
pub const TDVMCALL_EXPOSE_REGS_MASK: u64 = TDX_RDX | TDX_RBX | TDX_RSI | TDX_RDI |
    TDX_R8 | TDX_R9 | TDX_R10 | TDX_R11 | TDX_R12 | TDX_R13 | TDX_R14 | TDX_R15;

/* TDX supported page sizes from the TDX module ABI. */
pub const TDX_PS_4K: u64 = 0;
pub const TDX_PS_2M: u64 = 1;
pub const TDX_PS_1G: u64 = 2;
pub const TDX_PS_NR: u64 = TDX_PS_1G + 1;

/*
 * Used in __tdcall*() to gather the input/output registers' values of the
 * TDCALL instruction when requesting services from the TDX module. This is a
 * software only structure and not part of the TDX module/VMM ABI
 */
#[repr(C)]
pub struct tdx_module_args {
    pub rcx: u64,
    pub rdx: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbx: u64,
    pub rdi: u64,
    pub rsi: u64,
}

extern "C" {
    pub fn __tdcall(fn_: u64, args: *mut tdx_module_args) -> u64;
    pub fn __tdcall_ret(fn_: u64, args: *mut tdx_module_args) -> u64;
    pub fn __tdcall_saved_ret(fn_: u64, args: *mut tdx_module_args) -> u64;
    pub fn __tdx_hypercall(args: *mut tdx_module_args) -> u64;
    pub fn __tdx_hypercall_failed() -> !;
    pub fn tdx_accept_memory(start: phys_addr_t, end: phys_addr_t) -> bool;
}

pub unsafe fn _tdx_hypercall(fn_: u64, r12: u64, r13: u64, r14: u64, r15: u64) -> u64 {
    let mut args = tdx_module_args {
        rcx: 0, rdx: 0, r8: 0, r9: 0,
        r10: TDX_HYPERCALL_STANDARD,
        r11: fn_, r12, r13, r14, r15,
        rbx: 0, rdi: 0, rsi: 0,
    };
    __tdx_hypercall(&mut args)
}

pub fn hcall_func(exit_reason: u64) -> u64 {
    exit_reason
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
