/*
 * Guest OS interface to x86 Xen.
 * Translated from the C header; architecture/build-condition branches are
 * retained as comments where they require external configuration.
 */

// __DEFINE_GUEST_HANDLE(name, type): under __XEN__ this is a repr(C) wrapper;
// otherwise it is a raw pointer alias. The declarations below use wrappers
// for the guest-handle ABI and preserve the pointed-to C types.

pub type xen_pfn_t = ::core::ffi::c_ulong;
pub const PRI_xen_pfn: &str = "lx";
pub type xen_ulong_t = ::core::ffi::c_ulong;
pub const PRI_xen_ulong: &str = "lx";
pub type xen_long_t = ::core::ffi::c_long;
pub const PRI_xen_long: &str = "lx";

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_uchar { pub p: *mut u8 }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_uint { pub p: *mut ::core::ffi::c_uint }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_char { pub p: *mut ::core::ffi::c_char }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_int { pub p: *mut ::core::ffi::c_int }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_void { pub p: *mut ::core::ffi::c_void }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_uint64_t { pub p: *mut u64 }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_uint32_t { pub p: *mut u32 }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_xen_pfn_t { pub p: *mut xen_pfn_t }
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct __guest_handle_xen_ulong_t { pub p: *mut xen_ulong_t }

pub const HYPERVISOR_VIRT_START: usize = __HYPERVISOR_VIRT_START as usize;
pub const MACH2PHYS_VIRT_START: usize = __MACH2PHYS_VIRT_START as usize;
pub const MACH2PHYS_VIRT_END: usize = __MACH2PHYS_VIRT_END as usize;
pub const MACH2PHYS_NR_ENTRIES: usize =
    (MACH2PHYS_VIRT_END.wrapping_sub(MACH2PHYS_VIRT_START)) >> __MACH2PHYS_SHIFT;

pub const MAX_VIRT_CPUS: u32 = 32;
pub const FIRST_RESERVED_GDT_PAGE: usize = 14;
pub const FIRST_RESERVED_GDT_BYTE: usize = FIRST_RESERVED_GDT_PAGE * 4096;
pub const FIRST_RESERVED_GDT_ENTRY: usize = FIRST_RESERVED_GDT_BYTE / 8;

#[inline]
pub unsafe fn TI_GET_DPL(_ti: *const trap_info) -> u8 { (*_ti).flags & 3 }
#[inline]
pub unsafe fn TI_GET_IF(_ti: *const trap_info) -> u8 { (*_ti).flags & 4 }
#[inline]
pub unsafe fn TI_SET_DPL(_ti: *mut trap_info, _dpl: u8) { (*_ti).flags |= _dpl; }
#[inline]
pub unsafe fn TI_SET_IF(_ti: *mut trap_info, _if: u8) {
    (*_ti).flags |= ((!(_if == 0) as u8) << 2);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trap_info {
    pub vector: u8,
    pub flags: u8,
    pub cs: u16,
    pub address: ::core::ffi::c_ulong,
}
pub type __guest_handle_trap_info = *mut trap_info;

#[repr(C)]
pub struct arch_shared_info {
    pub max_pfn: ::core::ffi::c_ulong,
    pub pfn_to_mfn_frame_list_list: xen_pfn_t,
    pub nmi_reason: ::core::ffi::c_ulong,
    pub p2m_cr3: ::core::ffi::c_ulong,
    pub p2m_vaddr: ::core::ffi::c_ulong,
    pub p2m_generation: ::core::ffi::c_ulong,
    // Present only when CONFIG_X86_32 is defined.
    #[cfg(any())]
    pub wc_sec_hi: u32,
}

#[repr(C)]
pub struct vcpu_guest_context {
    pub fpu_ctxt: [u8; 512],
    pub flags: ::core::ffi::c_ulong,
    pub user_regs: cpu_user_regs,
    pub trap_ctxt: [trap_info; 256],
    pub ldt_base: ::core::ffi::c_ulong,
    pub ldt_ents: ::core::ffi::c_ulong,
    pub gdt_frames: [::core::ffi::c_ulong; 16],
    pub gdt_ents: ::core::ffi::c_ulong,
    pub kernel_ss: ::core::ffi::c_ulong,
    pub kernel_sp: ::core::ffi::c_ulong,
    pub ctrlreg: [::core::ffi::c_ulong; 8],
    pub debugreg: [::core::ffi::c_ulong; 8],
    // __i386__ and __x86_64__ layouts are selected by the external build.
    pub event_callback_eip: ::core::ffi::c_ulong,
    pub failsafe_callback_eip: ::core::ffi::c_ulong,
    #[cfg(any())]
    pub event_callback_cs: ::core::ffi::c_ulong,
    #[cfg(any())]
    pub failsafe_callback_cs: ::core::ffi::c_ulong,
    #[cfg(any())]
    pub syscall_callback_eip: ::core::ffi::c_ulong,
    pub vm_assist: ::core::ffi::c_ulong,
    #[cfg(any())]
    pub fs_base: u64,
    #[cfg(any())]
    pub gs_base_kernel: u64,
    #[cfg(any())]
    pub gs_base_user: u64,
}
pub type __guest_handle_vcpu_guest_context = *mut vcpu_guest_context;

#[repr(C)]
pub struct xen_pmu_amd_ctxt {
    pub counters: u32,
    pub ctrls: u32,
    pub regs: [u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_cntr_pair { pub counter: u64, pub control: u64 }

#[repr(C)]
pub struct xen_pmu_intel_ctxt {
    pub fixed_counters: u32,
    pub arch_counters: u32,
    pub global_ctrl: u64,
    pub global_ovf_ctrl: u64,
    pub global_status: u64,
    pub fixed_ctrl: u64,
    pub ds_area: u64,
    pub pebs_enable: u64,
    pub debugctl: u64,
    pub regs: [u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pmu_regs {
    pub ip: u64, pub sp: u64, pub flags: u64,
    pub cs: u16, pub ss: u16, pub cpl: u8, pub pad: [u8; 3],
}

pub const PMU_CACHED: u32 = 1 << 0;
pub const PMU_SAMPLE_USER: u32 = 1 << 1;
pub const PMU_SAMPLE_REAL: u32 = 1 << 2;
pub const PMU_SAMPLE_PV: u32 = 1 << 3;
pub const XENPMU_REGS_PAD_SZ: usize = 64;
pub const XENPMU_CTXT_PAD_SZ: usize = 128;

#[repr(C)]
pub union xen_pmu_arch_r { pub regs: xen_pmu_regs, pub pad: [u8; XENPMU_REGS_PAD_SZ] }
#[repr(C)]
pub union xen_pmu_arch_l { pub lapic_lvtpc: u32, pub pad: u64 }
#[repr(C)]
pub union xen_pmu_arch_c {
    pub amd: xen_pmu_amd_ctxt,
    pub intel: xen_pmu_intel_ctxt,
    pub pad: [u8; XENPMU_CTXT_PAD_SZ],
}
#[repr(C)]
pub struct xen_pmu_arch { pub r: xen_pmu_arch_r, pub pmu_flags: u64, pub l: xen_pmu_arch_l, pub c: xen_pmu_arch_c }

pub const VGCF_I387_VALID: u32 = 1 << 0;
pub const VGCF_IN_KERNEL: u32 = 1 << 2;
pub const _VGCF_i387_valid: u32 = 0;
pub const VGCF_i387_valid: u32 = 1 << _VGCF_i387_valid;
pub const _VGCF_in_kernel: u32 = 2;
pub const VGCF_in_kernel: u32 = 1 << _VGCF_in_kernel;
pub const _VGCF_failsafe_disables_events: u32 = 3;
pub const VGCF_failsafe_disables_events: u32 = 1 << _VGCF_failsafe_disables_events;
pub const _VGCF_syscall_disables_events: u32 = 4;
pub const VGCF_syscall_disables_events: u32 = 1 << _VGCF_syscall_disables_events;
pub const _VGCF_online: u32 = 5;
pub const VGCF_online: u32 = 1 << _VGCF_online;

// XEN_EMULATE_PREFIX and XEN_CPUID expand to architecture-specific assembly
// provided by asm/emulate_prefix.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
