/* SPDX-License-Identifier: MIT */
/* Guest OS interface to Xen. Translated from xen.h. */

// Dependency supplied by asm/xen/interface.h:
// xen_pfn_t, xen_ulong_t, xen_long_t, arch_vcpu_info, pvclock_vcpu_time_info,
// pvclock_wall_clock, arch_shared_info, MAX_VIRT_CPUS, and guest-handle macros.

pub const __HYPERVISOR_set_trap_table: u32 = 0;
pub const __HYPERVISOR_mmu_update: u32 = 1;
pub const __HYPERVISOR_set_gdt: u32 = 2;
pub const __HYPERVISOR_stack_switch: u32 = 3;
pub const __HYPERVISOR_set_callbacks: u32 = 4;
pub const __HYPERVISOR_fpu_taskswitch: u32 = 5;
pub const __HYPERVISOR_sched_op_compat: u32 = 6;
pub const __HYPERVISOR_platform_op: u32 = 7;
pub const __HYPERVISOR_set_debugreg: u32 = 8;
pub const __HYPERVISOR_get_debugreg: u32 = 9;
pub const __HYPERVISOR_update_descriptor: u32 = 10;
pub const __HYPERVISOR_memory_op: u32 = 12;
pub const __HYPERVISOR_multicall: u32 = 13;
pub const __HYPERVISOR_update_va_mapping: u32 = 14;
pub const __HYPERVISOR_set_timer_op: u32 = 15;
pub const __HYPERVISOR_event_channel_op_compat: u32 = 16;
pub const __HYPERVISOR_xen_version: u32 = 17;
pub const __HYPERVISOR_console_io: u32 = 18;
pub const __HYPERVISOR_physdev_op_compat: u32 = 19;
pub const __HYPERVISOR_grant_table_op: u32 = 20;
pub const __HYPERVISOR_vm_assist: u32 = 21;
pub const __HYPERVISOR_update_va_mapping_otherdomain: u32 = 22;
pub const __HYPERVISOR_iret: u32 = 23;
pub const __HYPERVISOR_vcpu_op: u32 = 24;
pub const __HYPERVISOR_set_segment_base: u32 = 25;
pub const __HYPERVISOR_mmuext_op: u32 = 26;
pub const __HYPERVISOR_xsm_op: u32 = 27;
pub const __HYPERVISOR_nmi_op: u32 = 28;
pub const __HYPERVISOR_sched_op: u32 = 29;
pub const __HYPERVISOR_callback_op: u32 = 30;
pub const __HYPERVISOR_xenoprof_op: u32 = 31;
pub const __HYPERVISOR_event_channel_op: u32 = 32;
pub const __HYPERVISOR_physdev_op: u32 = 33;
pub const __HYPERVISOR_hvm_op: u32 = 34;
pub const __HYPERVISOR_sysctl: u32 = 35;
pub const __HYPERVISOR_domctl: u32 = 36;
pub const __HYPERVISOR_kexec_op: u32 = 37;
pub const __HYPERVISOR_tmem_op: u32 = 38;
pub const __HYPERVISOR_xc_reserved_op: u32 = 39;
pub const __HYPERVISOR_xenpmu_op: u32 = 40;
pub const __HYPERVISOR_dm_op: u32 = 41;
pub const __HYPERVISOR_arch_0: u32 = 48;
pub const __HYPERVISOR_arch_1: u32 = 49;
pub const __HYPERVISOR_arch_2: u32 = 50;
pub const __HYPERVISOR_arch_3: u32 = 51;
pub const __HYPERVISOR_arch_4: u32 = 52;
pub const __HYPERVISOR_arch_5: u32 = 53;
pub const __HYPERVISOR_arch_6: u32 = 54;
pub const __HYPERVISOR_arch_7: u32 = 55;

pub const VIRQ_TIMER: u32 = 0; pub const VIRQ_DEBUG: u32 = 1;
pub const VIRQ_CONSOLE: u32 = 2; pub const VIRQ_DOM_EXC: u32 = 3;
pub const VIRQ_TBUF: u32 = 4; pub const VIRQ_DEBUGGER: u32 = 6;
pub const VIRQ_XENOPROF: u32 = 7; pub const VIRQ_CON_RING: u32 = 8;
pub const VIRQ_PCPU_STATE: u32 = 9; pub const VIRQ_MEM_EVENT: u32 = 10;
pub const VIRQ_XC_RESERVED: u32 = 11; pub const VIRQ_ENOMEM: u32 = 12;
pub const VIRQ_XENPMU: u32 = 13;
pub const VIRQ_ARCH_0: u32 = 16; pub const VIRQ_ARCH_1: u32 = 17;
pub const VIRQ_ARCH_2: u32 = 18; pub const VIRQ_ARCH_3: u32 = 19;
pub const VIRQ_ARCH_4: u32 = 20; pub const VIRQ_ARCH_5: u32 = 21;
pub const VIRQ_ARCH_6: u32 = 22; pub const VIRQ_ARCH_7: u32 = 23;
pub const NR_VIRQS: u32 = 24;

pub const MMU_NORMAL_PT_UPDATE: u32 = 0;
pub const MMU_MACHPHYS_UPDATE: u32 = 1;
pub const MMU_PT_UPDATE_PRESERVE_AD: u32 = 2;
pub const MMU_PT_UPDATE_NO_TRANSLATE: u32 = 3;

pub const MMUEXT_PIN_L1_TABLE: u32 = 0; pub const MMUEXT_PIN_L2_TABLE: u32 = 1;
pub const MMUEXT_PIN_L3_TABLE: u32 = 2; pub const MMUEXT_PIN_L4_TABLE: u32 = 3;
pub const MMUEXT_UNPIN_TABLE: u32 = 4; pub const MMUEXT_NEW_BASEPTR: u32 = 5;
pub const MMUEXT_TLB_FLUSH_LOCAL: u32 = 6; pub const MMUEXT_INVLPG_LOCAL: u32 = 7;
pub const MMUEXT_TLB_FLUSH_MULTI: u32 = 8; pub const MMUEXT_INVLPG_MULTI: u32 = 9;
pub const MMUEXT_TLB_FLUSH_ALL: u32 = 10; pub const MMUEXT_INVLPG_ALL: u32 = 11;
pub const MMUEXT_FLUSH_CACHE: u32 = 12; pub const MMUEXT_SET_LDT: u32 = 13;
pub const MMUEXT_NEW_USER_BASEPTR: u32 = 15; pub const MMUEXT_CLEAR_PAGE: u32 = 16;
pub const MMUEXT_COPY_PAGE: u32 = 17; pub const MMUEXT_FLUSH_CACHE_GLOBAL: u32 = 18;
pub const MMUEXT_MARK_SUPER: u32 = 19; pub const MMUEXT_UNMARK_SUPER: u32 = 20;

#[repr(C)]
pub union MmuextArg1 { pub mfn: xen_pfn_t, pub linear_addr: usize }
#[repr(C)]
pub union MmuextArg2 { pub nr_ents: u32, pub vcpumask: *mut core::ffi::c_void, pub src_mfn: xen_pfn_t }
#[repr(C)]
pub struct mmuext_op { pub cmd: u32, pub arg1: MmuextArg1, pub arg2: MmuextArg2 }

pub const UVMF_NONE: usize = 0; pub const UVMF_TLB_FLUSH: usize = 1;
pub const UVMF_INVLPG: usize = 2; pub const UVMF_FLUSHTYPE_MASK: usize = 3;
pub const UVMF_MULTI: usize = 0; pub const UVMF_LOCAL: usize = 0; pub const UVMF_ALL: usize = 4;
pub const CONSOLEIO_write: u32 = 0; pub const CONSOLEIO_read: u32 = 1;
pub const VMASST_CMD_enable: u32 = 0; pub const VMASST_CMD_disable: u32 = 1;
pub const VMASST_TYPE_4gb_segments: u32 = 0; pub const VMASST_TYPE_4gb_segments_notify: u32 = 1;
pub const VMASST_TYPE_writable_pagetables: u32 = 2; pub const VMASST_TYPE_pae_extended_cr3: u32 = 3;
pub const VMASST_TYPE_architectural_iopl: u32 = 4; pub const VMASST_TYPE_runstate_update_flag: u32 = 5;
pub const MAX_VMASST_TYPE: u32 = 5;

pub type domid_t = u16;
pub const DOMID_FIRST_RESERVED: u16 = 0x7ff0; pub const DOMID_SELF: u16 = 0x7ff0;
pub const DOMID_IO: u16 = 0x7ff1; pub const DOMID_XEN: u16 = 0x7ff2;
pub const DOMID_COW: u16 = 0x7ff3; pub const DOMID_INVALID: u16 = 0x7ff4;
pub const DOMID_IDLE: u16 = 0x7fff;

#[repr(C)] pub struct mmu_update { pub ptr: u64, pub val: u64 }
#[repr(C)] pub struct multicall_entry { pub op: xen_ulong_t, pub result: xen_long_t, pub args: [xen_ulong_t; 6] }

#[repr(C)]
pub struct vcpu_time_info {
    pub version: u32, pub pad0: u32, pub tsc_timestamp: u64, pub system_time: u64,
    pub tsc_to_system_mul: u32, pub tsc_shift: i8, pub pad1: [i8; 3],
}
#[repr(C)] pub struct vcpu_info {
    pub evtchn_upcall_pending: u8, pub evtchn_upcall_mask: u8, pub evtchn_pending_sel: xen_ulong_t,
    pub arch: arch_vcpu_info, pub time: pvclock_vcpu_time_info,
}
#[repr(C)] pub struct shared_info {
    pub vcpu_info: [vcpu_info; MAX_VIRT_CPUS],
    pub evtchn_pending: [xen_ulong_t; core::mem::size_of::<xen_ulong_t>() * 8],
    pub evtchn_mask: [xen_ulong_t; core::mem::size_of::<xen_ulong_t>() * 8],
    pub wc: pvclock_wall_clock,
    #[cfg(not(CONFIG_X86_32))] pub wc_sec_hi: u32,
    pub arch: arch_shared_info,
}

pub const MAX_GUEST_CMDLINE: usize = 1024;
#[repr(C)] pub union StartConsole {
    pub domU: StartConsoleDomU, pub dom0: StartConsoleDom0,
}
#[repr(C)] pub struct StartConsoleDomU { pub mfn: xen_pfn_t, pub evtchn: u32 }
#[repr(C)] pub struct StartConsoleDom0 { pub info_off: u32, pub info_size: u32 }
#[repr(C)] pub struct start_info {
    pub magic: [core::ffi::c_char; 32], pub nr_pages: usize, pub shared_info: usize,
    pub flags: u32, pub store_mfn: xen_pfn_t, pub store_evtchn: u32, pub console: StartConsole,
    pub pt_base: usize, pub nr_pt_frames: usize, pub mfn_list: usize, pub mod_start: usize,
    pub mod_len: usize, pub cmd_line: [i8; MAX_GUEST_CMDLINE], pub first_p2m_pfn: usize,
    pub nr_p2m_frames: usize,
}
pub const SIF_PRIVILEGED: u32 = 1 << 0; pub const SIF_INITDOMAIN: u32 = 1 << 1;
pub const SIF_MULTIBOOT_MOD: u32 = 1 << 2; pub const SIF_MOD_START_PFN: u32 = 1 << 3;
pub const SIF_VIRT_P2M_4TOOLS: u32 = 1 << 4; pub const SIF_PM_MASK: u32 = 0xff << 8;

#[repr(C)] pub struct xen_multiboot_mod_list { pub mod_start: u32, pub mod_end: u32, pub cmdline: u32, pub pad: u32 }
pub const XEN_VGATYPE_TEXT_MODE_3: u8 = 0x03; pub const XEN_VGATYPE_VESA_LFB: u8 = 0x23; pub const XEN_VGATYPE_EFI_LFB: u8 = 0x70;
#[repr(C)] pub struct TextMode3 { pub font_height: u16, pub cursor_x: u16, pub cursor_y: u16, pub rows: u16, pub columns: u16 }
#[repr(C)] pub struct VesaLfb { pub width: u16, pub height: u16, pub bytes_per_line: u16, pub bits_per_pixel: u16, pub lfb_base: u32, pub lfb_size: u32, pub red_pos: u8, pub red_size: u8, pub green_pos: u8, pub green_size: u8, pub blue_pos: u8, pub blue_size: u8, pub rsvd_pos: u8, pub rsvd_size: u8, pub gbl_caps: u32, pub mode_attrs: u16, pub pad: u16, pub ext_lfb_base: u32 }
#[repr(C)] pub union Dom0VgaUnion { pub text_mode_3: TextMode3, pub vesa_lfb: VesaLfb }
#[repr(C)] pub struct dom0_vga_console_info { pub video_type: u8, pub u: Dom0VgaUnion }
pub type cpumap_t = u64; pub type xen_domain_handle_t = [u8; 16];
pub const TMEM_SPEC_VERSION: u32 = 1;
#[repr(C)] pub struct TmemNew { pub uuid: [u64; 2], pub flags: u32 }
#[repr(C)] pub struct TmemGen { pub oid: [u64; 3], pub index: u32, pub tmem_offset: u32, pub pfn_offset: u32, pub len: u32, pub gmfn: *mut core::ffi::c_void }
#[repr(C)] pub union TmemUnion { pub new: TmemNew, pub gen: TmemGen }
#[repr(C)] pub struct tmem_op { pub cmd: u32, pub pool_id: i32, pub u: TmemUnion }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
