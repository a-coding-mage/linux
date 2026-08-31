/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Userspace interface for /dev/kvm - kernel based virtual machine
 *
 * Note: you must update KVM_API_VERSION if you change this interface.
 *
 * C dependencies removed from executable Rust:
 * linux/const.h, linux/types.h, linux/compiler.h, linux/stddef.h,
 * linux/ioctl.h, asm/kvm.h, and linux/kvm_types.h when __KERNEL__.
 */

pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __u64 = u64;
pub type c_char = i8;
pub type c_int = i32;
pub type c_ulong = usize;
pub type c_void = core::ffi::c_void;

pub const KVM_API_VERSION: __u32 = 12;

/*
 * Backwards-compatible definitions.
 * #define __KVM_HAVE_GUEST_DEBUG
 */

/* for KVM_SET_USER_MEMORY_REGION */
#[repr(C)]
pub struct kvm_userspace_memory_region {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64, /* bytes */
    pub userspace_addr: __u64, /* start of the userspace allocated memory */
}

/* for KVM_SET_USER_MEMORY_REGION2 */
#[repr(C)]
pub struct kvm_userspace_memory_region2 {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
    pub guest_memfd_offset: __u64,
    pub guest_memfd: __u32,
    pub pad1: __u32,
    pub pad2: [__u64; 14],
}

/*
 * The bit 0 ~ bit 15 of kvm_userspace_memory_region::flags are visible for
 * userspace, other bits are reserved for kvm internal use which are defined
 * in include/linux/kvm_host.h.
 */
pub const KVM_MEM_LOG_DIRTY_PAGES: c_ulong = 1usize << 0;
pub const KVM_MEM_READONLY: c_ulong = 1usize << 1;
pub const KVM_MEM_GUEST_MEMFD: c_ulong = 1usize << 2;

/* for KVM_IRQ_LINE */
#[repr(C)]
pub union kvm_irq_level_unnamed {
    pub irq: __u32,
    pub status: __s32,
}

#[repr(C)]
pub struct kvm_irq_level {
    /*
     * ACPI gsi notion of irq.
     * For IA-64 (APIC model) IOAPIC0: irq 0-23; IOAPIC1: irq 24-47..
     * For X86 (standard AT mode) PIC0/1: irq 0-15. IOAPIC0: 0-23..
     * For ARM: See Documentation/virt/kvm/api.rst
     */
    pub u: kvm_irq_level_unnamed,
    pub level: __u32,
}

#[repr(C)]
pub union kvm_irqchip_chip {
    pub dummy: [c_char; 512], /* reserving space */
    /* Present in C when __KVM_HAVE_PIT: pub pic: kvm_pic_state, */
    /* Present in C when __KVM_HAVE_IOAPIC: pub ioapic: kvm_ioapic_state, */
}

#[repr(C)]
pub struct kvm_irqchip {
    pub chip_id: __u32,
    pub pad: __u32,
    pub chip: kvm_irqchip_chip,
}

/* for KVM_CREATE_PIT2 */
#[repr(C)]
pub struct kvm_pit_config {
    pub flags: __u32,
    pub pad: [__u32; 15],
}

pub const KVM_PIT_SPEAKER_DUMMY: __u32 = 1;

pub const KVM_EXIT_HYPERV_SYNIC: __u32 = 1;
pub const KVM_EXIT_HYPERV_HCALL: __u32 = 2;
pub const KVM_EXIT_HYPERV_SYNDBG: __u32 = 3;

#[repr(C)]
pub struct kvm_hyperv_exit_synic {
    pub msr: __u32,
    pub pad2: __u32,
    pub control: __u64,
    pub evt_page: __u64,
    pub msg_page: __u64,
}

#[repr(C)]
pub struct kvm_hyperv_exit_hcall {
    pub input: __u64,
    pub result: __u64,
    pub params: [__u64; 2],
}

#[repr(C)]
pub struct kvm_hyperv_exit_syndbg {
    pub msr: __u32,
    pub pad2: __u32,
    pub control: __u64,
    pub status: __u64,
    pub send_page: __u64,
    pub recv_page: __u64,
    pub pending_page: __u64,
}

#[repr(C)]
pub union kvm_hyperv_exit_u {
    pub synic: core::mem::ManuallyDrop<kvm_hyperv_exit_synic>,
    pub hcall: core::mem::ManuallyDrop<kvm_hyperv_exit_hcall>,
    pub syndbg: core::mem::ManuallyDrop<kvm_hyperv_exit_syndbg>,
}

#[repr(C)]
pub struct kvm_hyperv_exit {
    pub type_: __u32,
    pub pad1: __u32,
    pub u: kvm_hyperv_exit_u,
}

pub const KVM_EXIT_XEN_HCALL: __u32 = 1;

#[repr(C)]
pub struct kvm_xen_exit_hcall {
    pub longmode: __u32,
    pub cpl: __u32,
    pub input: __u64,
    pub result: __u64,
    pub params: [__u64; 6],
}

#[repr(C)]
pub union kvm_xen_exit_u {
    pub hcall: core::mem::ManuallyDrop<kvm_xen_exit_hcall>,
}

#[repr(C)]
pub struct kvm_xen_exit {
    pub type_: __u32,
    pub u: kvm_xen_exit_u,
}

#[repr(C)]
pub struct kvm_exit_snp_req_certs {
    pub gpa: __u64,
    pub npages: __u64,
    pub ret: __u64,
}

pub const KVM_S390_GET_SKEYS_NONE: __u32 = 1;
pub const KVM_S390_SKEYS_MAX: __u32 = 1048576;

pub const KVM_EXIT_UNKNOWN: __u32 = 0;
pub const KVM_EXIT_EXCEPTION: __u32 = 1;
pub const KVM_EXIT_IO: __u32 = 2;
pub const KVM_EXIT_HYPERCALL: __u32 = 3;
pub const KVM_EXIT_DEBUG: __u32 = 4;
pub const KVM_EXIT_HLT: __u32 = 5;
pub const KVM_EXIT_MMIO: __u32 = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: __u32 = 7;
pub const KVM_EXIT_SHUTDOWN: __u32 = 8;
pub const KVM_EXIT_FAIL_ENTRY: __u32 = 9;
pub const KVM_EXIT_INTR: __u32 = 10;
pub const KVM_EXIT_SET_TPR: __u32 = 11;
pub const KVM_EXIT_TPR_ACCESS: __u32 = 12;
pub const KVM_EXIT_S390_SIEIC: __u32 = 13;
pub const KVM_EXIT_S390_RESET: __u32 = 14;
pub const KVM_EXIT_DCR: __u32 = 15; /* deprecated */
pub const KVM_EXIT_NMI: __u32 = 16;
pub const KVM_EXIT_INTERNAL_ERROR: __u32 = 17;
pub const KVM_EXIT_OSI: __u32 = 18;
pub const KVM_EXIT_PAPR_HCALL: __u32 = 19;
pub const KVM_EXIT_S390_UCONTROL: __u32 = 20;
pub const KVM_EXIT_WATCHDOG: __u32 = 21;
pub const KVM_EXIT_S390_TSCH: __u32 = 22;
pub const KVM_EXIT_EPR: __u32 = 23;
pub const KVM_EXIT_SYSTEM_EVENT: __u32 = 24;
pub const KVM_EXIT_S390_STSI: __u32 = 25;
pub const KVM_EXIT_IOAPIC_EOI: __u32 = 26;
pub const KVM_EXIT_HYPERV: __u32 = 27;
pub const KVM_EXIT_ARM_NISV: __u32 = 28;
pub const KVM_EXIT_X86_RDMSR: __u32 = 29;
pub const KVM_EXIT_X86_WRMSR: __u32 = 30;
pub const KVM_EXIT_DIRTY_RING_FULL: __u32 = 31;
pub const KVM_EXIT_AP_RESET_HOLD: __u32 = 32;
pub const KVM_EXIT_X86_BUS_LOCK: __u32 = 33;
pub const KVM_EXIT_XEN: __u32 = 34;
pub const KVM_EXIT_RISCV_SBI: __u32 = 35;
pub const KVM_EXIT_RISCV_CSR: __u32 = 36;
pub const KVM_EXIT_NOTIFY: __u32 = 37;
pub const KVM_EXIT_LOONGARCH_IOCSR: __u32 = 38;
pub const KVM_EXIT_MEMORY_FAULT: __u32 = 39;
pub const KVM_EXIT_TDX: __u32 = 40;
pub const KVM_EXIT_ARM_SEA: __u32 = 41;
pub const KVM_EXIT_ARM_LDST64B: __u32 = 42;
pub const KVM_EXIT_SNP_REQ_CERTS: __u32 = 43;

/* For KVM_EXIT_INTERNAL_ERROR */
/* Emulate instruction failed. */
pub const KVM_INTERNAL_ERROR_EMULATION: __u32 = 1;
/* Encounter unexpected simultaneous exceptions. */
pub const KVM_INTERNAL_ERROR_SIMUL_EX: __u32 = 2;
/* Encounter unexpected vm-exit due to delivery event. */
pub const KVM_INTERNAL_ERROR_DELIVERY_EV: __u32 = 3;
/* Encounter unexpected vm-exit reason */
pub const KVM_INTERNAL_ERROR_UNEXPECTED_EXIT_REASON: __u32 = 4;

/* Flags that describe what fields in emulation_failure hold valid data. */
pub const KVM_INTERNAL_ERROR_EMULATION_FLAG_INSTRUCTION_BYTES: __u64 = 1u64 << 0;

/*
 * struct kvm_run can be modified by userspace at any time, so KVM must be
 * careful to avoid TOCTOU bugs. In order to protect KVM, HINT_UNSAFE_IN_KVM()
 * renames fields in struct kvm_run from <symbol> to <symbol>__unsafe when
 * compiled into the kernel, ensuring that any use within KVM is obvious and
 * gets extra scrutiny.
 */
/* Rust userspace translation uses immediate_exit, matching non-__KERNEL__. */

pub const KVM_EXIT_IO_IN: __u8 = 0;
pub const KVM_EXIT_IO_OUT: __u8 = 1;
pub const KVM_SYSTEM_EVENT_SHUTDOWN: __u32 = 1;
pub const KVM_SYSTEM_EVENT_RESET: __u32 = 2;
pub const KVM_SYSTEM_EVENT_CRASH: __u32 = 3;
pub const KVM_SYSTEM_EVENT_WAKEUP: __u32 = 4;
pub const KVM_SYSTEM_EVENT_SUSPEND: __u32 = 5;
pub const KVM_SYSTEM_EVENT_SEV_TERM: __u32 = 6;
pub const KVM_SYSTEM_EVENT_TDX_FATAL: __u32 = 7;
pub const KVM_MSR_EXIT_REASON_INVAL: __u32 = 1 << 0;
pub const KVM_MSR_EXIT_REASON_UNKNOWN: __u32 = 1 << 1;
pub const KVM_MSR_EXIT_REASON_FILTER: __u32 = 1 << 2;
pub const KVM_MSR_EXIT_REASON_VALID_MASK: __u32 =
    KVM_MSR_EXIT_REASON_INVAL | KVM_MSR_EXIT_REASON_UNKNOWN | KVM_MSR_EXIT_REASON_FILTER;
pub const KVM_NOTIFY_CONTEXT_INVALID: __u32 = 1 << 0;
pub const KVM_MEMORY_EXIT_FLAG_PRIVATE: __u64 = 1u64 << 3;
pub const KVM_EXIT_ARM_SEA_FLAG_GPA_VALID: __u64 = 1u64 << 0;
pub const SYNC_REGS_SIZE_BYTES: usize = 2048;

#[repr(C)]
pub struct kvm_run_hw {
    pub hardware_exit_reason: __u64,
}
#[repr(C)]
pub struct kvm_run_fail_entry {
    pub hardware_entry_failure_reason: __u64,
    pub cpu: __u32,
}
#[repr(C)]
pub struct kvm_run_ex {
    pub exception: __u32,
    pub error_code: __u32,
}
#[repr(C)]
pub struct kvm_run_io {
    pub direction: __u8,
    pub size: __u8, /* bytes */
    pub port: __u16,
    pub count: __u32,
    pub data_offset: __u64, /* relative to kvm_run start */
}
#[repr(C)]
pub struct kvm_run_debug {
    pub arch: kvm_debug_exit_arch,
}
#[repr(C)]
pub struct kvm_run_mmio {
    pub phys_addr: __u64,
    pub data: [__u8; 8],
    pub len: __u32,
    pub is_write: __u8,
}
#[repr(C)]
pub union kvm_run_hypercall_unnamed {
    pub longmode: __u32, /* non-__KERNEL__ */
    pub flags: __u64,
}
#[repr(C)]
pub struct kvm_run_hypercall {
    pub nr: __u64,
    pub args: [__u64; 6],
    pub ret: __u64,
    pub u: kvm_run_hypercall_unnamed,
}
#[repr(C)]
pub struct kvm_run_tpr_access {
    pub rip: __u64,
    pub is_write: __u32,
    pub pad: __u32,
}
#[repr(C)]
pub struct kvm_run_s390_sieic {
    pub icptcode: __u8,
    pub ipa: __u16,
    pub ipb: __u32,
}
#[repr(C)]
pub struct kvm_run_s390_ucontrol {
    pub trans_exc_code: __u64,
    pub pgm_code: __u32,
}
#[repr(C)]
pub struct kvm_run_dcr {
    pub dcrn: __u32,
    pub data: __u32,
    pub is_write: __u8,
}
#[repr(C)]
pub struct kvm_run_internal {
    pub suberror: __u32,
    /* Available with KVM_CAP_INTERNAL_ERROR_DATA: */
    pub ndata: __u32,
    pub data: [__u64; 16],
}
#[repr(C)]
pub struct kvm_run_emulation_failure_insn {
    pub insn_size: __u8,
    pub insn_bytes: [__u8; 15],
}
#[repr(C)]
pub union kvm_run_emulation_failure_u {
    pub insn: core::mem::ManuallyDrop<kvm_run_emulation_failure_insn>,
}
#[repr(C)]
pub struct kvm_run_emulation_failure {
    pub suberror: __u32,
    pub ndata: __u32,
    pub flags: __u64,
    pub u: kvm_run_emulation_failure_u,
    /* Arbitrary debug data may follow. */
}
#[repr(C)]
pub struct kvm_run_osi {
    pub gprs: [__u64; 32],
}
#[repr(C)]
pub struct kvm_run_papr_hcall {
    pub nr: __u64,
    pub ret: __u64,
    pub args: [__u64; 9],
}
#[repr(C)]
pub struct kvm_run_s390_tsch {
    pub subchannel_id: __u16,
    pub subchannel_nr: __u16,
    pub io_int_parm: __u32,
    pub io_int_word: __u32,
    pub ipb: __u32,
    pub dequeued: __u8,
}
#[repr(C)]
pub struct kvm_run_epr {
    pub epr: __u32,
}
#[repr(C)]
pub union kvm_run_system_event_unnamed {
    pub flags: __u64, /* non-__KERNEL__ */
    pub data: [__u64; 16],
}
#[repr(C)]
pub struct kvm_run_system_event {
    pub type_: __u32,
    pub ndata: __u32,
    pub u: kvm_run_system_event_unnamed,
}
#[repr(C)]
pub struct kvm_run_s390_stsi {
    pub addr: __u64,
    pub ar: __u8,
    pub reserved: __u8,
    pub fc: __u8,
    pub sel1: __u8,
    pub sel2: __u16,
}
#[repr(C)]
pub struct kvm_run_eoi {
    pub vector: __u8,
}
#[repr(C)]
pub struct kvm_run_arm_nisv {
    pub esr_iss: __u64,
    pub fault_ipa: __u64,
}
#[repr(C)]
pub struct kvm_run_msr {
    pub error: __u8, /* user -> kernel */
    pub pad: [__u8; 7],
    pub reason: __u32, /* kernel -> user */
    pub index: __u32, /* kernel -> user */
    pub data: __u64, /* kernel <-> user */
}
#[repr(C)]
pub struct kvm_run_riscv_sbi {
    pub extension_id: c_ulong,
    pub function_id: c_ulong,
    pub args: [c_ulong; 6],
    pub ret: [c_ulong; 2],
}
#[repr(C)]
pub struct kvm_run_riscv_csr {
    pub csr_num: c_ulong,
    pub new_value: c_ulong,
    pub write_mask: c_ulong,
    pub ret_value: c_ulong,
}
#[repr(C)]
pub struct kvm_run_notify {
    pub flags: __u32,
}
#[repr(C)]
pub struct kvm_run_memory_fault {
    pub flags: __u64,
    pub gpa: __u64,
    pub size: __u64,
}
#[repr(C)]
pub struct kvm_run_tdx_unknown {
    pub ret: __u64,
    pub data: [__u64; 5],
}
#[repr(C)]
pub struct kvm_run_tdx_get_quote {
    pub ret: __u64,
    pub gpa: __u64,
    pub size: __u64,
}
#[repr(C)]
pub struct kvm_run_tdx_get_tdvmcall_info {
    pub ret: __u64,
    pub leaf: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
}
#[repr(C)]
pub struct kvm_run_tdx_setup_event_notify {
    pub ret: __u64,
    pub vector: __u64,
}
#[repr(C)]
pub union kvm_run_tdx_u {
    pub unknown: core::mem::ManuallyDrop<kvm_run_tdx_unknown>,
    pub get_quote: core::mem::ManuallyDrop<kvm_run_tdx_get_quote>,
    pub get_tdvmcall_info: core::mem::ManuallyDrop<kvm_run_tdx_get_tdvmcall_info>,
    pub setup_event_notify: core::mem::ManuallyDrop<kvm_run_tdx_setup_event_notify>,
}
#[repr(C)]
pub struct kvm_run_tdx {
    pub flags: __u64,
    pub nr: __u64,
    pub u: kvm_run_tdx_u,
}
#[repr(C)]
pub struct kvm_run_arm_sea {
    pub flags: __u64,
    pub esr: __u64,
    pub gva: __u64,
    pub gpa: __u64,
}

#[repr(C)]
pub union kvm_run_exit {
    pub hw: core::mem::ManuallyDrop<kvm_run_hw>,
    pub fail_entry: core::mem::ManuallyDrop<kvm_run_fail_entry>,
    pub ex: core::mem::ManuallyDrop<kvm_run_ex>,
    pub io: core::mem::ManuallyDrop<kvm_run_io>,
    pub debug: core::mem::ManuallyDrop<kvm_run_debug>,
    pub mmio: core::mem::ManuallyDrop<kvm_run_mmio>,
    pub iocsr_io: core::mem::ManuallyDrop<kvm_run_mmio>,
    pub hypercall: core::mem::ManuallyDrop<kvm_run_hypercall>,
    pub tpr_access: core::mem::ManuallyDrop<kvm_run_tpr_access>,
    pub s390_sieic: core::mem::ManuallyDrop<kvm_run_s390_sieic>,
    pub s390_reset_flags: __u64,
    pub s390_ucontrol: core::mem::ManuallyDrop<kvm_run_s390_ucontrol>,
    pub dcr: core::mem::ManuallyDrop<kvm_run_dcr>,
    pub internal: core::mem::ManuallyDrop<kvm_run_internal>,
    pub emulation_failure: core::mem::ManuallyDrop<kvm_run_emulation_failure>,
    pub osi: core::mem::ManuallyDrop<kvm_run_osi>,
    pub papr_hcall: core::mem::ManuallyDrop<kvm_run_papr_hcall>,
    pub s390_tsch: core::mem::ManuallyDrop<kvm_run_s390_tsch>,
    pub epr: core::mem::ManuallyDrop<kvm_run_epr>,
    pub system_event: core::mem::ManuallyDrop<kvm_run_system_event>,
    pub s390_stsi: core::mem::ManuallyDrop<kvm_run_s390_stsi>,
    pub eoi: core::mem::ManuallyDrop<kvm_run_eoi>,
    pub hyperv: core::mem::ManuallyDrop<kvm_hyperv_exit>,
    pub arm_nisv: core::mem::ManuallyDrop<kvm_run_arm_nisv>,
    pub msr: core::mem::ManuallyDrop<kvm_run_msr>,
    pub xen: core::mem::ManuallyDrop<kvm_xen_exit>,
    pub riscv_sbi: core::mem::ManuallyDrop<kvm_run_riscv_sbi>,
    pub riscv_csr: core::mem::ManuallyDrop<kvm_run_riscv_csr>,
    pub notify: core::mem::ManuallyDrop<kvm_run_notify>,
    pub memory_fault: core::mem::ManuallyDrop<kvm_run_memory_fault>,
    pub tdx: core::mem::ManuallyDrop<kvm_run_tdx>,
    pub arm_sea: core::mem::ManuallyDrop<kvm_run_arm_sea>,
    pub snp_req_certs: core::mem::ManuallyDrop<kvm_exit_snp_req_certs>,
    pub padding: [c_char; 256],
}

#[repr(C)]
pub union kvm_run_s {
    pub regs: core::mem::ManuallyDrop<kvm_sync_regs>,
    pub padding: [c_char; SYNC_REGS_SIZE_BYTES],
}

/* for KVM_RUN, returned by mmap(vcpu_fd, offset=0) */
#[repr(C)]
pub struct kvm_run {
    /* in */
    pub request_interrupt_window: __u8,
    pub immediate_exit: __u8,
    pub padding1: [__u8; 6],

    /* out */
    pub exit_reason: __u32,
    pub ready_for_interrupt_injection: __u8,
    pub if_flag: __u8,
    pub flags: __u16,

    /* in (pre_kvm_run), out (post_kvm_run) */
    pub cr8: __u64,
    pub apic_base: __u64,

    /* Present in C when __KVM_S390:
     * pub psw_mask: __u64;
     * pub psw_addr: __u64;
     */
    pub exit: kvm_run_exit,

    /*
     * shared registers between kvm and userspace.
     * kvm_valid_regs specifies the register classes set by the host
     * kvm_dirty_regs specified the register classes dirtied by userspace
     * struct kvm_sync_regs is architecture specific, as well as the
     * bits for kvm_valid_regs and kvm_dirty_regs
     */
    pub kvm_valid_regs: __u64,
    pub kvm_dirty_regs: __u64,
    pub s: kvm_run_s,
}

/* for KVM_REGISTER_COALESCED_MMIO / KVM_UNREGISTER_COALESCED_MMIO */
#[repr(C)]
pub union kvm_coalesced_mmio_zone_unnamed {
    pub pad: __u32,
    pub pio: __u32,
}

#[repr(C)]
pub struct kvm_coalesced_mmio_zone {
    pub addr: __u64,
    pub size: __u32,
    pub u: kvm_coalesced_mmio_zone_unnamed,
}

#[repr(C)]
pub union kvm_coalesced_mmio_unnamed {
    pub pad: __u32,
    pub pio: __u32,
}

#[repr(C)]
pub struct kvm_coalesced_mmio {
    pub phys_addr: __u64,
    pub len: __u32,
    pub u: kvm_coalesced_mmio_unnamed,
    pub data: [__u8; 8],
}

#[repr(C)]
pub struct kvm_coalesced_mmio_ring {
    pub first: __u32,
    pub last: __u32,
    pub coalesced_mmio: [kvm_coalesced_mmio; 0],
}

/* KVM_COALESCED_MMIO_MAX = ((PAGE_SIZE - sizeof(struct kvm_coalesced_mmio_ring)) / sizeof(struct kvm_coalesced_mmio)) */

/* for KVM_TRANSLATE */
#[repr(C)]
pub struct kvm_translation {
    /* in */
    pub linear_address: __u64,

    /* out */
    pub physical_address: __u64,
    pub valid: __u8,
    pub writeable: __u8,
    pub usermode: __u8,
    pub pad: [__u8; 5],
}

/* for KVM_INTERRUPT */
#[repr(C)]
pub struct kvm_interrupt {
    /* in */
    pub irq: __u32,
}

/* for KVM_GET_DIRTY_LOG */
#[repr(C)]
pub union kvm_dirty_log_unnamed {
    pub dirty_bitmap: *mut c_void, /* one bit per page */
    pub padding2: __u64,
}

#[repr(C)]
pub struct kvm_dirty_log {
    pub slot: __u32,
    pub padding1: __u32,
    pub u: kvm_dirty_log_unnamed,
}

/* for KVM_CLEAR_DIRTY_LOG */
#[repr(C)]
pub union kvm_clear_dirty_log_unnamed {
    pub dirty_bitmap: *mut c_void, /* one bit per page */
    pub padding2: __u64,
}

#[repr(C)]
pub struct kvm_clear_dirty_log {
    pub slot: __u32,
    pub num_pages: __u32,
    pub first_page: __u64,
    pub u: kvm_clear_dirty_log_unnamed,
}

/* for KVM_SET_SIGNAL_MASK */
#[repr(C)]
pub struct kvm_signal_mask {
    pub len: __u32,
    pub sigset: [__u8; 0],
}

/* for KVM_TPR_ACCESS_REPORTING */
#[repr(C)]
pub struct kvm_tpr_access_ctl {
    pub enabled: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 8],
}

/* for KVM_SET_VAPIC_ADDR */
#[repr(C)]
pub struct kvm_vapic_addr {
    pub vapic_addr: __u64,
}

/* for KVM_SET_MP_STATE */
/* not all states are valid on all architectures */
pub const KVM_MP_STATE_RUNNABLE: __u32 = 0;
pub const KVM_MP_STATE_UNINITIALIZED: __u32 = 1;
pub const KVM_MP_STATE_INIT_RECEIVED: __u32 = 2;
pub const KVM_MP_STATE_HALTED: __u32 = 3;
pub const KVM_MP_STATE_SIPI_RECEIVED: __u32 = 4;
pub const KVM_MP_STATE_STOPPED: __u32 = 5;
pub const KVM_MP_STATE_CHECK_STOP: __u32 = 6;
pub const KVM_MP_STATE_OPERATING: __u32 = 7;
pub const KVM_MP_STATE_LOAD: __u32 = 8;
pub const KVM_MP_STATE_AP_RESET_HOLD: __u32 = 9;
pub const KVM_MP_STATE_SUSPENDED: __u32 = 10;

#[repr(C)]
pub struct kvm_mp_state {
    pub mp_state: __u32,
}

/* for KVM_SET_GUEST_DEBUG */
pub const KVM_GUESTDBG_ENABLE: __u32 = 0x00000001;
pub const KVM_GUESTDBG_SINGLESTEP: __u32 = 0x00000002;

#[repr(C)]
pub struct kvm_guest_debug {
    pub control: __u32,
    pub pad: __u32,
    pub arch: kvm_guest_debug_arch,
}

pub const kvm_ioeventfd_flag_nr_datamatch: __u32 = 0;
pub const kvm_ioeventfd_flag_nr_pio: __u32 = 1;
pub const kvm_ioeventfd_flag_nr_deassign: __u32 = 2;
pub const kvm_ioeventfd_flag_nr_virtio_ccw_notify: __u32 = 3;
pub const kvm_ioeventfd_flag_nr_fast_mmio: __u32 = 4;
pub const kvm_ioeventfd_flag_nr_max: __u32 = 5;

pub const KVM_IOEVENTFD_FLAG_DATAMATCH: __u32 = 1 << kvm_ioeventfd_flag_nr_datamatch;
pub const KVM_IOEVENTFD_FLAG_PIO: __u32 = 1 << kvm_ioeventfd_flag_nr_pio;
pub const KVM_IOEVENTFD_FLAG_DEASSIGN: __u32 = 1 << kvm_ioeventfd_flag_nr_deassign;
pub const KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY: __u32 = 1 << kvm_ioeventfd_flag_nr_virtio_ccw_notify;
pub const KVM_IOEVENTFD_VALID_FLAG_MASK: __u32 = (1 << kvm_ioeventfd_flag_nr_max) - 1;

#[repr(C)]
pub struct kvm_ioeventfd {
    pub datamatch: __u64,
    pub addr: __u64, /* legal pio/mmio address */
    pub len: __u32, /* 1, 2, 4, or 8 bytes; or 0 to ignore length */
    pub fd: __s32,
    pub flags: __u32,
    pub pad: [__u8; 36],
}

pub const KVM_X86_DISABLE_EXITS_MWAIT: __u32 = 1 << 0;
pub const KVM_X86_DISABLE_EXITS_HLT: __u32 = 1 << 1;
pub const KVM_X86_DISABLE_EXITS_PAUSE: __u32 = 1 << 2;
pub const KVM_X86_DISABLE_EXITS_CSTATE: __u32 = 1 << 3;
pub const KVM_X86_DISABLE_EXITS_APERFMPERF: __u32 = 1 << 4;

/* for KVM_ENABLE_CAP */
#[repr(C)]
pub struct kvm_enable_cap {
    /* in */
    pub cap: __u32,
    pub flags: __u32,
    pub args: [__u64; 4],
    pub pad: [__u8; 64],
}

pub const KVMIO: __u32 = 0xAE;

/* machine type bits, to be used as argument to KVM_CREATE_VM */
pub const KVM_VM_S390_UCONTROL: __u32 = 1;
pub const KVM_VM_PPC_HV: __u32 = 1;
pub const KVM_VM_PPC_PR: __u32 = 2;
pub const KVM_VM_MIPS_AUTO: __u32 = 0;
pub const KVM_VM_MIPS_VZ: __u32 = 1;
pub const KVM_VM_MIPS_TE: __u32 = 2;
pub const KVM_S390_SIE_PAGE_OFFSET: __u32 = 1;
pub const KVM_VM_TYPE_ARM_IPA_SIZE_MASK: __u64 = 0xff;
pub const fn KVM_VM_TYPE_ARM_IPA_SIZE(x: __u64) -> __u64 { x & KVM_VM_TYPE_ARM_IPA_SIZE_MASK }
pub const KVM_VM_TYPE_ARM_PROTECTED: c_ulong = 1usize << 31;
pub const KVM_VM_TYPE_ARM_MASK: __u64 = KVM_VM_TYPE_ARM_IPA_SIZE_MASK | KVM_VM_TYPE_ARM_PROTECTED as __u64;

/* ioctl constants reference ioctl macros from linux/ioctl.h translated elsewhere. */
/* ioctls for /dev/kvm fds: */
pub const KVM_GET_API_VERSION: c_ulong = _IO!(KVMIO, 0x00);
pub const KVM_CREATE_VM: c_ulong = _IO!(KVMIO, 0x01); /* returns a VM fd */
pub const KVM_GET_MSR_INDEX_LIST: c_ulong = _IOWR!(KVMIO, 0x02, kvm_msr_list);
pub const KVM_S390_ENABLE_SIE: c_ulong = _IO!(KVMIO, 0x06);
pub const KVM_CHECK_EXTENSION: c_ulong = _IO!(KVMIO, 0x03);
pub const KVM_GET_VCPU_MMAP_SIZE: c_ulong = _IO!(KVMIO, 0x04); /* in bytes */
pub const KVM_GET_SUPPORTED_CPUID: c_ulong = _IOWR!(KVMIO, 0x05, kvm_cpuid2);
pub const KVM_GET_EMULATED_CPUID: c_ulong = _IOWR!(KVMIO, 0x09, kvm_cpuid2);
pub const KVM_GET_MSR_FEATURE_INDEX_LIST: c_ulong = _IOWR!(KVMIO, 0x0a, kvm_msr_list);

/*
 * Extension capability list.
 */
pub const KVM_CAP_IRQCHIP: __u32 = 0;
pub const KVM_CAP_HLT: __u32 = 1;
pub const KVM_CAP_MMU_SHADOW_CACHE_CONTROL: __u32 = 2;
pub const KVM_CAP_USER_MEMORY: __u32 = 3;
pub const KVM_CAP_SET_TSS_ADDR: __u32 = 4;
pub const KVM_CAP_VAPIC: __u32 = 6;
pub const KVM_CAP_EXT_CPUID: __u32 = 7;
pub const KVM_CAP_CLOCKSOURCE: __u32 = 8;
pub const KVM_CAP_NR_VCPUS: __u32 = 9; /* returns recommended max vcpus per vm */
pub const KVM_CAP_NR_MEMSLOTS: __u32 = 10; /* returns max memory slots per vm */
pub const KVM_CAP_PIT: __u32 = 11;
pub const KVM_CAP_NOP_IO_DELAY: __u32 = 12;
pub const KVM_CAP_PV_MMU: __u32 = 13;
pub const KVM_CAP_MP_STATE: __u32 = 14;
pub const KVM_CAP_COALESCED_MMIO: __u32 = 15;
pub const KVM_CAP_SYNC_MMU: __u32 = 16; /* Changes to host mmap are reflected in guest */
pub const KVM_CAP_IOMMU: __u32 = 18;
pub const KVM_CAP_DESTROY_MEMORY_REGION_WORKS: __u32 = 21;
pub const KVM_CAP_USER_NMI: __u32 = 22;
pub const KVM_CAP_SET_GUEST_DEBUG: __u32 = 23;
/* Present in C when __KVM_HAVE_PIT: KVM_CAP_REINJECT_CONTROL = 24 */
pub const KVM_CAP_IRQ_ROUTING: __u32 = 25;
pub const KVM_CAP_IRQ_INJECT_STATUS: __u32 = 26;
pub const KVM_CAP_ASSIGN_DEV_IRQ: __u32 = 29;
pub const KVM_CAP_JOIN_MEMORY_REGIONS_WORKS: __u32 = 30;
/* Present in C when __KVM_HAVE_MCE: KVM_CAP_MCE = 31 */
pub const KVM_CAP_IRQFD: __u32 = 32;
/* Present in C when __KVM_HAVE_PIT: KVM_CAP_PIT2 = 33 */
pub const KVM_CAP_SET_BOOT_CPU_ID: __u32 = 34;
/* Present in C when __KVM_HAVE_PIT_STATE2: KVM_CAP_PIT_STATE2 = 35 */
pub const KVM_CAP_IOEVENTFD: __u32 = 36;
pub const KVM_CAP_SET_IDENTITY_MAP_ADDR: __u32 = 37;
/* Present in C when __KVM_HAVE_XEN_HVM: KVM_CAP_XEN_HVM = 38 */
pub const KVM_CAP_ADJUST_CLOCK: __u32 = 39;
pub const KVM_CAP_INTERNAL_ERROR_DATA: __u32 = 40;
/* Present in C when __KVM_HAVE_VCPU_EVENTS: KVM_CAP_VCPU_EVENTS = 41 */
pub const KVM_CAP_S390_PSW: __u32 = 42;
pub const KVM_CAP_PPC_SEGSTATE: __u32 = 43;
pub const KVM_CAP_HYPERV: __u32 = 44;
pub const KVM_CAP_HYPERV_VAPIC: __u32 = 45;
pub const KVM_CAP_HYPERV_SPIN: __u32 = 46;
pub const KVM_CAP_PCI_SEGMENT: __u32 = 47;
pub const KVM_CAP_PPC_PAIRED_SINGLES: __u32 = 48;
pub const KVM_CAP_INTR_SHADOW: __u32 = 49;
/* Present in C when __KVM_HAVE_DEBUGREGS: KVM_CAP_DEBUGREGS = 50 */
pub const KVM_CAP_X86_ROBUST_SINGLESTEP: __u32 = 51;
pub const KVM_CAP_PPC_OSI: __u32 = 52;
pub const KVM_CAP_PPC_UNSET_IRQ: __u32 = 53;
pub const KVM_CAP_ENABLE_CAP: __u32 = 54;
/* Present in C when __KVM_HAVE_XSAVE: KVM_CAP_XSAVE = 55 */
/* Present in C when __KVM_HAVE_XCRS: KVM_CAP_XCRS = 56 */
pub const KVM_CAP_PPC_GET_PVINFO: __u32 = 57;
pub const KVM_CAP_PPC_IRQ_LEVEL: __u32 = 58;
pub const KVM_CAP_ASYNC_PF: __u32 = 59;
pub const KVM_CAP_TSC_CONTROL: __u32 = 60;
pub const KVM_CAP_GET_TSC_KHZ: __u32 = 61;
pub const KVM_CAP_PPC_BOOKE_SREGS: __u32 = 62;
pub const KVM_CAP_SPAPR_TCE: __u32 = 63;
pub const KVM_CAP_PPC_SMT: __u32 = 64;
pub const KVM_CAP_PPC_RMA: __u32 = 65;
pub const KVM_CAP_MAX_VCPUS: __u32 = 66; /* returns max vcpus per vm */
pub const KVM_CAP_PPC_HIOR: __u32 = 67;
pub const KVM_CAP_PPC_PAPR: __u32 = 68;
pub const KVM_CAP_SW_TLB: __u32 = 69;
pub const KVM_CAP_ONE_REG: __u32 = 70;
pub const KVM_CAP_S390_GMAP: __u32 = 71;
pub const KVM_CAP_TSC_DEADLINE_TIMER: __u32 = 72;
pub const KVM_CAP_S390_UCONTROL: __u32 = 73;
pub const KVM_CAP_SYNC_REGS: __u32 = 74;
pub const KVM_CAP_PCI_2_3: __u32 = 75;
pub const KVM_CAP_KVMCLOCK_CTRL: __u32 = 76;
pub const KVM_CAP_SIGNAL_MSI: __u32 = 77;
pub const KVM_CAP_PPC_GET_SMMU_INFO: __u32 = 78;
pub const KVM_CAP_S390_COW: __u32 = 79;
pub const KVM_CAP_PPC_ALLOC_HTAB: __u32 = 80;
pub const KVM_CAP_READONLY_MEM: __u32 = 81;
pub const KVM_CAP_IRQFD_RESAMPLE: __u32 = 82;
pub const KVM_CAP_PPC_BOOKE_WATCHDOG: __u32 = 83;
pub const KVM_CAP_PPC_HTAB_FD: __u32 = 84;
pub const KVM_CAP_S390_CSS_SUPPORT: __u32 = 85;
pub const KVM_CAP_PPC_EPR: __u32 = 86;
pub const KVM_CAP_ARM_PSCI: __u32 = 87;
pub const KVM_CAP_ARM_SET_DEVICE_ADDR: __u32 = 88;
pub const KVM_CAP_DEVICE_CTRL: __u32 = 89;
pub const KVM_CAP_IRQ_MPIC: __u32 = 90;
pub const KVM_CAP_PPC_RTAS: __u32 = 91;
pub const KVM_CAP_IRQ_XICS: __u32 = 92;
pub const KVM_CAP_ARM_EL1_32BIT: __u32 = 93;
pub const KVM_CAP_SPAPR_MULTITCE: __u32 = 94;
pub const KVM_CAP_EXT_EMUL_CPUID: __u32 = 95;
pub const KVM_CAP_HYPERV_TIME: __u32 = 96;
pub const KVM_CAP_IOAPIC_POLARITY_IGNORED: __u32 = 97;
pub const KVM_CAP_ENABLE_CAP_VM: __u32 = 98;
pub const KVM_CAP_S390_IRQCHIP: __u32 = 99;
pub const KVM_CAP_IOEVENTFD_NO_LENGTH: __u32 = 100;
pub const KVM_CAP_VM_ATTRIBUTES: __u32 = 101;
pub const KVM_CAP_ARM_PSCI_0_2: __u32 = 102;
pub const KVM_CAP_PPC_FIXUP_HCALL: __u32 = 103;
pub const KVM_CAP_PPC_ENABLE_HCALL: __u32 = 104;
pub const KVM_CAP_CHECK_EXTENSION_VM: __u32 = 105;
pub const KVM_CAP_S390_USER_SIGP: __u32 = 106;
pub const KVM_CAP_S390_VECTOR_REGISTERS: __u32 = 107;
pub const KVM_CAP_S390_MEM_OP: __u32 = 108;
pub const KVM_CAP_S390_USER_STSI: __u32 = 109;
pub const KVM_CAP_S390_SKEYS: __u32 = 110;
pub const KVM_CAP_MIPS_FPU: __u32 = 111;
pub const KVM_CAP_MIPS_MSA: __u32 = 112;
pub const KVM_CAP_S390_INJECT_IRQ: __u32 = 113;
pub const KVM_CAP_S390_IRQ_STATE: __u32 = 114;
pub const KVM_CAP_PPC_HWRNG: __u32 = 115;
pub const KVM_CAP_DISABLE_QUIRKS: __u32 = 116;
pub const KVM_CAP_X86_SMM: __u32 = 117;
pub const KVM_CAP_MULTI_ADDRESS_SPACE: __u32 = 118;
pub const KVM_CAP_GUEST_DEBUG_HW_BPS: __u32 = 119;
pub const KVM_CAP_GUEST_DEBUG_HW_WPS: __u32 = 120;
pub const KVM_CAP_SPLIT_IRQCHIP: __u32 = 121;
pub const KVM_CAP_IOEVENTFD_ANY_LENGTH: __u32 = 122;
pub const KVM_CAP_HYPERV_SYNIC: __u32 = 123;
pub const KVM_CAP_S390_RI: __u32 = 124;
pub const KVM_CAP_SPAPR_TCE_64: __u32 = 125;
pub const KVM_CAP_ARM_PMU_V3: __u32 = 126;
pub const KVM_CAP_VCPU_ATTRIBUTES: __u32 = 127;
pub const KVM_CAP_MAX_VCPU_ID: __u32 = 128;
pub const KVM_CAP_X2APIC_API: __u32 = 129;
pub const KVM_CAP_S390_USER_INSTR0: __u32 = 130;
pub const KVM_CAP_MSI_DEVID: __u32 = 131;
pub const KVM_CAP_PPC_HTM: __u32 = 132;
pub const KVM_CAP_SPAPR_RESIZE_HPT: __u32 = 133;
pub const KVM_CAP_PPC_MMU_RADIX: __u32 = 134;
pub const KVM_CAP_PPC_MMU_HASH_V3: __u32 = 135;
pub const KVM_CAP_IMMEDIATE_EXIT: __u32 = 136;
pub const KVM_CAP_MIPS_VZ: __u32 = 137;
pub const KVM_CAP_MIPS_TE: __u32 = 138;
pub const KVM_CAP_MIPS_64BIT: __u32 = 139;
pub const KVM_CAP_S390_GS: __u32 = 140;
pub const KVM_CAP_S390_AIS: __u32 = 141;
pub const KVM_CAP_SPAPR_TCE_VFIO: __u32 = 142;
pub const KVM_CAP_X86_DISABLE_EXITS: __u32 = 143;
pub const KVM_CAP_ARM_USER_IRQ: __u32 = 144;
pub const KVM_CAP_S390_CMMA_MIGRATION: __u32 = 145;
pub const KVM_CAP_PPC_FWNMI: __u32 = 146;
pub const KVM_CAP_PPC_SMT_POSSIBLE: __u32 = 147;
pub const KVM_CAP_HYPERV_SYNIC2: __u32 = 148;
pub const KVM_CAP_HYPERV_VP_INDEX: __u32 = 149;
pub const KVM_CAP_S390_AIS_MIGRATION: __u32 = 150;
pub const KVM_CAP_PPC_GET_CPU_CHAR: __u32 = 151;
pub const KVM_CAP_S390_BPB: __u32 = 152;
pub const KVM_CAP_GET_MSR_FEATURES: __u32 = 153;
pub const KVM_CAP_HYPERV_EVENTFD: __u32 = 154;
pub const KVM_CAP_HYPERV_TLBFLUSH: __u32 = 155;
pub const KVM_CAP_S390_HPAGE_1M: __u32 = 156;
pub const KVM_CAP_NESTED_STATE: __u32 = 157;
pub const KVM_CAP_ARM_INJECT_SERROR_ESR: __u32 = 158;
pub const KVM_CAP_MSR_PLATFORM_INFO: __u32 = 159;
pub const KVM_CAP_PPC_NESTED_HV: __u32 = 160;
pub const KVM_CAP_HYPERV_SEND_IPI: __u32 = 161;
pub const KVM_CAP_COALESCED_PIO: __u32 = 162;
pub const KVM_CAP_HYPERV_ENLIGHTENED_VMCS: __u32 = 163;
pub const KVM_CAP_EXCEPTION_PAYLOAD: __u32 = 164;
pub const KVM_CAP_ARM_VM_IPA_SIZE: __u32 = 165;
pub const KVM_CAP_MANUAL_DIRTY_LOG_PROTECT: __u32 = 166; /* Obsolete */
pub const KVM_CAP_HYPERV_CPUID: __u32 = 167;
pub const KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2: __u32 = 168;
pub const KVM_CAP_PPC_IRQ_XIVE: __u32 = 169;
pub const KVM_CAP_ARM_SVE: __u32 = 170;
pub const KVM_CAP_ARM_PTRAUTH_ADDRESS: __u32 = 171;
pub const KVM_CAP_ARM_PTRAUTH_GENERIC: __u32 = 172;
pub const KVM_CAP_PMU_EVENT_FILTER: __u32 = 173;
pub const KVM_CAP_ARM_IRQ_LINE_LAYOUT_2: __u32 = 174;
pub const KVM_CAP_HYPERV_DIRECT_TLBFLUSH: __u32 = 175;
pub const KVM_CAP_PPC_GUEST_DEBUG_SSTEP: __u32 = 176;
pub const KVM_CAP_ARM_NISV_TO_USER: __u32 = 177;
pub const KVM_CAP_ARM_INJECT_EXT_DABT: __u32 = 178;
pub const KVM_CAP_S390_VCPU_RESETS: __u32 = 179;
pub const KVM_CAP_S390_PROTECTED: __u32 = 180;
pub const KVM_CAP_PPC_SECURE_GUEST: __u32 = 181;
pub const KVM_CAP_HALT_POLL: __u32 = 182;
pub const KVM_CAP_ASYNC_PF_INT: __u32 = 183;
pub const KVM_CAP_LAST_CPU: __u32 = 184;
pub const KVM_CAP_SMALLER_MAXPHYADDR: __u32 = 185;
pub const KVM_CAP_S390_DIAG318: __u32 = 186;
pub const KVM_CAP_STEAL_TIME: __u32 = 187;
pub const KVM_CAP_X86_USER_SPACE_MSR: __u32 = 188;
pub const KVM_CAP_X86_MSR_FILTER: __u32 = 189;
pub const KVM_CAP_ENFORCE_PV_FEATURE_CPUID: __u32 = 190;
pub const KVM_CAP_SYS_HYPERV_CPUID: __u32 = 191;
pub const KVM_CAP_DIRTY_LOG_RING: __u32 = 192;
pub const KVM_CAP_X86_BUS_LOCK_EXIT: __u32 = 193;
pub const KVM_CAP_PPC_DAWR1: __u32 = 194;
pub const KVM_CAP_SET_GUEST_DEBUG2: __u32 = 195;
pub const KVM_CAP_SGX_ATTRIBUTE: __u32 = 196;
pub const KVM_CAP_VM_COPY_ENC_CONTEXT_FROM: __u32 = 197;
pub const KVM_CAP_PTP_KVM: __u32 = 198;
pub const KVM_CAP_HYPERV_ENFORCE_CPUID: __u32 = 199;
pub const KVM_CAP_SREGS2: __u32 = 200;
pub const KVM_CAP_EXIT_HYPERCALL: __u32 = 201;
pub const KVM_CAP_PPC_RPT_INVALIDATE: __u32 = 202;
pub const KVM_CAP_BINARY_STATS_FD: __u32 = 203;
pub const KVM_CAP_EXIT_ON_EMULATION_FAILURE: __u32 = 204;
pub const KVM_CAP_ARM_MTE: __u32 = 205;
pub const KVM_CAP_VM_MOVE_ENC_CONTEXT_FROM: __u32 = 206;
pub const KVM_CAP_VM_GPA_BITS: __u32 = 207;
pub const KVM_CAP_XSAVE2: __u32 = 208;
pub const KVM_CAP_SYS_ATTRIBUTES: __u32 = 209;
pub const KVM_CAP_PPC_AIL_MODE_3: __u32 = 210;
pub const KVM_CAP_S390_MEM_OP_EXTENSION: __u32 = 211;
pub const KVM_CAP_PMU_CAPABILITY: __u32 = 212;
pub const KVM_CAP_DISABLE_QUIRKS2: __u32 = 213;
pub const KVM_CAP_VM_TSC_CONTROL: __u32 = 214;
pub const KVM_CAP_SYSTEM_EVENT_DATA: __u32 = 215;
pub const KVM_CAP_ARM_SYSTEM_SUSPEND: __u32 = 216;
pub const KVM_CAP_S390_PROTECTED_DUMP: __u32 = 217;
pub const KVM_CAP_X86_TRIPLE_FAULT_EVENT: __u32 = 218;
pub const KVM_CAP_X86_NOTIFY_VMEXIT: __u32 = 219;
pub const KVM_CAP_VM_DISABLE_NX_HUGE_PAGES: __u32 = 220;
pub const KVM_CAP_S390_ZPCI_OP: __u32 = 221;
pub const KVM_CAP_S390_CPU_TOPOLOGY: __u32 = 222;
pub const KVM_CAP_DIRTY_LOG_RING_ACQ_REL: __u32 = 223;
pub const KVM_CAP_S390_PROTECTED_ASYNC_DISABLE: __u32 = 224;
pub const KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP: __u32 = 225;
pub const KVM_CAP_PMU_EVENT_MASKED_EVENTS: __u32 = 226;
pub const KVM_CAP_COUNTER_OFFSET: __u32 = 227;
pub const KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE: __u32 = 228;
pub const KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES: __u32 = 229;
pub const KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES: __u32 = 230;
pub const KVM_CAP_USER_MEMORY2: __u32 = 231;
pub const KVM_CAP_MEMORY_FAULT_INFO: __u32 = 232;
pub const KVM_CAP_MEMORY_ATTRIBUTES: __u32 = 233;
pub const KVM_CAP_GUEST_MEMFD: __u32 = 234;
pub const KVM_CAP_VM_TYPES: __u32 = 235;
pub const KVM_CAP_PRE_FAULT_MEMORY: __u32 = 236;
pub const KVM_CAP_X86_APIC_BUS_CYCLES_NS: __u32 = 237;
pub const KVM_CAP_X86_GUEST_MODE: __u32 = 238;
pub const KVM_CAP_ARM_WRITABLE_IMP_ID_REGS: __u32 = 239;
pub const KVM_CAP_ARM_EL2: __u32 = 240;
pub const KVM_CAP_ARM_EL2_E2H0: __u32 = 241;
pub const KVM_CAP_RISCV_MP_STATE_RESET: __u32 = 242;
pub const KVM_CAP_ARM_CACHEABLE_PFNMAP_SUPPORTED: __u32 = 243;
pub const KVM_CAP_GUEST_MEMFD_FLAGS: __u32 = 244;
pub const KVM_CAP_ARM_SEA_TO_USER: __u32 = 245;
pub const KVM_CAP_S390_USER_OPEREXEC: __u32 = 246;
pub const KVM_CAP_S390_KEYOP: __u32 = 247;
pub const KVM_CAP_S390_VSIE_ESAMODE: __u32 = 248;
pub const KVM_CAP_S390_HPAGE_2G: __u32 = 249;

#[repr(C)]
pub struct kvm_irq_routing_irqchip {
    pub irqchip: __u32,
    pub pin: __u32,
}
#[repr(C)]
pub union kvm_irq_routing_msi_unnamed {
    pub pad: __u32,
    pub devid: __u32,
}
#[repr(C)]
pub struct kvm_irq_routing_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub u: kvm_irq_routing_msi_unnamed,
}
#[repr(C)]
pub struct kvm_irq_routing_s390_adapter {
    pub ind_addr: __u64,
    pub summary_addr: __u64,
    pub ind_offset: __u64,
    pub summary_offset: __u32,
    pub adapter_id: __u32,
}
#[repr(C)]
pub struct kvm_irq_routing_hv_sint {
    pub vcpu: __u32,
    pub sint: __u32,
}
#[repr(C)]
pub struct kvm_irq_routing_xen_evtchn {
    pub port: __u32,
    pub vcpu: __u32,
    pub priority: __u32,
}
pub const KVM_IRQ_ROUTING_XEN_EVTCHN_PRIO_2LEVEL: __u32 = -1i32 as __u32;
pub const KVM_IRQ_ROUTING_IRQCHIP: __u32 = 1;
pub const KVM_IRQ_ROUTING_MSI: __u32 = 2;
pub const KVM_IRQ_ROUTING_S390_ADAPTER: __u32 = 3;
pub const KVM_IRQ_ROUTING_HV_SINT: __u32 = 4;
pub const KVM_IRQ_ROUTING_XEN_EVTCHN: __u32 = 5;

#[repr(C)]
pub union kvm_irq_routing_entry_u {
    pub irqchip: core::mem::ManuallyDrop<kvm_irq_routing_irqchip>,
    pub msi: core::mem::ManuallyDrop<kvm_irq_routing_msi>,
    pub adapter: core::mem::ManuallyDrop<kvm_irq_routing_s390_adapter>,
    pub hv_sint: core::mem::ManuallyDrop<kvm_irq_routing_hv_sint>,
    pub xen_evtchn: core::mem::ManuallyDrop<kvm_irq_routing_xen_evtchn>,
    pub pad: [__u32; 8],
}
#[repr(C)]
pub struct kvm_irq_routing_entry {
    pub gsi: __u32,
    pub type_: __u32,
    pub flags: __u32,
    pub pad: __u32,
    pub u: kvm_irq_routing_entry_u,
}
#[repr(C)]
pub struct kvm_irq_routing {
    pub nr: __u32,
    pub flags: __u32,
    pub entries: [kvm_irq_routing_entry; 0],
}

pub const KVM_IRQFD_FLAG_DEASSIGN: __u32 = 1 << 0;
pub const KVM_IRQFD_FLAG_RESAMPLE: __u32 = 1 << 1;
#[repr(C)]
pub struct kvm_irqfd {
    pub fd: __u32,
    pub gsi: __u32,
    pub flags: __u32,
    pub resamplefd: __u32,
    pub pad: [__u8; 16],
}

pub const KVM_CLOCK_TSC_STABLE: __u32 = 2;
pub const KVM_CLOCK_REALTIME: __u32 = 1 << 2;
pub const KVM_CLOCK_HOST_TSC: __u32 = 1 << 3;
#[repr(C)]
pub struct kvm_clock_data {
    pub clock: __u64,
    pub flags: __u32,
    pub pad0: __u32,
    pub realtime: __u64,
    pub host_tsc: __u64,
    pub pad: [__u32; 4],
}

pub const KVM_MMU_FSL_BOOKE_NOHV: __u32 = 0;
pub const KVM_MMU_FSL_BOOKE_HV: __u32 = 1;
#[repr(C)]
pub struct kvm_config_tlb {
    pub params: __u64,
    pub array: __u64,
    pub mmu_type: __u32,
    pub array_len: __u32,
}
#[repr(C)]
pub struct kvm_dirty_tlb {
    pub bitmap: __u64,
    pub num_dirty: __u32,
}

pub const KVM_REG_ARCH_MASK: __u64 = 0xff00000000000000;
pub const KVM_REG_GENERIC: __u64 = 0x0000000000000000;
pub const KVM_REG_PPC: __u64 = 0x1000000000000000;
pub const KVM_REG_X86: __u64 = 0x2000000000000000;
pub const KVM_REG_IA64: __u64 = 0x3000000000000000;
pub const KVM_REG_ARM: __u64 = 0x4000000000000000;
pub const KVM_REG_S390: __u64 = 0x5000000000000000;
pub const KVM_REG_ARM64: __u64 = 0x6000000000000000;
pub const KVM_REG_MIPS: __u64 = 0x7000000000000000;
pub const KVM_REG_RISCV: __u64 = 0x8000000000000000;
pub const KVM_REG_LOONGARCH: __u64 = 0x9000000000000000;
pub const KVM_REG_SIZE_SHIFT: __u64 = 52;
pub const KVM_REG_SIZE_MASK: __u64 = 0x00f0000000000000;
pub const fn KVM_REG_SIZE(id: __u64) -> __u32 {
    1u32 << (((id & KVM_REG_SIZE_MASK) >> KVM_REG_SIZE_SHIFT) as u32)
}
pub const KVM_REG_SIZE_U8: __u64 = 0x0000000000000000;
pub const KVM_REG_SIZE_U16: __u64 = 0x0010000000000000;
pub const KVM_REG_SIZE_U32: __u64 = 0x0020000000000000;
pub const KVM_REG_SIZE_U64: __u64 = 0x0030000000000000;
pub const KVM_REG_SIZE_U128: __u64 = 0x0040000000000000;
pub const KVM_REG_SIZE_U256: __u64 = 0x0050000000000000;
pub const KVM_REG_SIZE_U512: __u64 = 0x0060000000000000;
pub const KVM_REG_SIZE_U1024: __u64 = 0x0070000000000000;
pub const KVM_REG_SIZE_U2048: __u64 = 0x0080000000000000;

#[repr(C)]
pub struct kvm_reg_list {
    pub n: __u64, /* number of regs */
    pub reg: [__u64; 0],
}
#[repr(C)]
pub struct kvm_one_reg {
    pub id: __u64,
    pub addr: __u64,
}

pub const KVM_MSI_VALID_DEVID: __u32 = 1u32 << 0;
#[repr(C)]
pub struct kvm_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub flags: __u32,
    pub devid: __u32,
    pub pad: [__u8; 12],
}
#[repr(C)]
pub struct kvm_arm_device_addr {
    pub id: __u64,
    pub addr: __u64,
}

pub const KVM_CREATE_DEVICE_TEST: __u32 = 1;
#[repr(C)]
pub struct kvm_create_device {
    pub type_: __u32, /* in: KVM_DEV_TYPE_xxx */
    pub fd: __u32, /* out: device handle */
    pub flags: __u32, /* in: KVM_CREATE_DEVICE_xxx */
}
#[repr(C)]
pub struct kvm_device_attr {
    pub flags: __u32, /* no flags currently defined */
    pub group: __u32, /* device-defined */
    pub attr: __u64, /* group-defined */
    pub addr: __u64, /* userspace address of attr data */
}

pub const KVM_DEV_VFIO_FILE: __u32 = 1;
pub const KVM_DEV_VFIO_FILE_ADD: __u32 = 1;
pub const KVM_DEV_VFIO_FILE_DEL: __u32 = 2;
pub const KVM_DEV_VFIO_GROUP: __u32 = KVM_DEV_VFIO_FILE;
pub const KVM_DEV_VFIO_GROUP_ADD: __u32 = KVM_DEV_VFIO_FILE_ADD;
pub const KVM_DEV_VFIO_GROUP_DEL: __u32 = KVM_DEV_VFIO_FILE_DEL;
pub const KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE: __u32 = 3;

#[repr(C)]
pub enum kvm_device_type {
    KVM_DEV_TYPE_FSL_MPIC_20 = 1,
    KVM_DEV_TYPE_FSL_MPIC_42,
    KVM_DEV_TYPE_XICS,
    KVM_DEV_TYPE_VFIO,
    KVM_DEV_TYPE_ARM_VGIC_V2,
    KVM_DEV_TYPE_FLIC,
    KVM_DEV_TYPE_ARM_VGIC_V3,
    KVM_DEV_TYPE_ARM_VGIC_ITS,
    KVM_DEV_TYPE_XIVE,
    KVM_DEV_TYPE_ARM_PV_TIME,
    KVM_DEV_TYPE_RISCV_AIA,
    KVM_DEV_TYPE_LOONGARCH_IPI,
    KVM_DEV_TYPE_LOONGARCH_EIOINTC,
    KVM_DEV_TYPE_LOONGARCH_PCHPIC,
    KVM_DEV_TYPE_LOONGARCH_DMSINTC,
    KVM_DEV_TYPE_ARM_VGIC_V5,
    KVM_DEV_TYPE_MAX,
}

#[repr(C)]
pub struct kvm_vfio_spapr_tce {
    pub groupfd: __s32,
    pub tablefd: __s32,
}

pub const KVM_S390_KEYOP_ISKE: __u32 = 0x01;
pub const KVM_S390_KEYOP_RRBE: __u32 = 0x02;
pub const KVM_S390_KEYOP_SSKE: __u32 = 0x03;
#[repr(C)]
pub struct kvm_s390_keyop {
    pub guest_addr: __u64,
    pub key: __u8,
    pub operation: __u8,
    pub pad: [__u8; 6],
}

/*
 * KVM_CREATE_VCPU receives as a parameter the vcpu slot, and returns
 * a vcpu fd.
 */
pub const KVM_CREATE_VCPU: c_ulong = _IO!(KVMIO, 0x41);
pub const KVM_GET_DIRTY_LOG: c_ulong = _IOW!(KVMIO, 0x42, kvm_dirty_log);
pub const KVM_SET_NR_MMU_PAGES: c_ulong = _IO!(KVMIO, 0x44);
pub const KVM_GET_NR_MMU_PAGES: c_ulong = _IO!(KVMIO, 0x45); /* deprecated */
pub const KVM_SET_USER_MEMORY_REGION: c_ulong = _IOW!(KVMIO, 0x46, kvm_userspace_memory_region);
pub const KVM_SET_TSS_ADDR: c_ulong = _IO!(KVMIO, 0x47);
pub const KVM_SET_IDENTITY_MAP_ADDR: c_ulong = _IOW!(KVMIO, 0x48, __u64);
pub const KVM_SET_USER_MEMORY_REGION2: c_ulong = _IOW!(KVMIO, 0x49, kvm_userspace_memory_region2);
pub const KVM_S390_UCAS_MAP: c_ulong = _IOW!(KVMIO, 0x50, kvm_s390_ucas_mapping);
pub const KVM_S390_UCAS_UNMAP: c_ulong = _IOW!(KVMIO, 0x51, kvm_s390_ucas_mapping);
pub const KVM_S390_VCPU_FAULT: c_ulong = _IOW!(KVMIO, 0x52, c_ulong);
pub const KVM_S390_KEYOP_IOCTL: c_ulong = _IOWR!(KVMIO, 0x53, kvm_s390_keyop);
pub const KVM_CREATE_IRQCHIP: c_ulong = _IO!(KVMIO, 0x60);
pub const KVM_IRQ_LINE: c_ulong = _IOW!(KVMIO, 0x61, kvm_irq_level);
pub const KVM_GET_IRQCHIP: c_ulong = _IOWR!(KVMIO, 0x62, kvm_irqchip);
pub const KVM_SET_IRQCHIP: c_ulong = _IOR!(KVMIO, 0x63, kvm_irqchip);
pub const KVM_CREATE_PIT: c_ulong = _IO!(KVMIO, 0x64);
pub const KVM_GET_PIT: c_ulong = _IOWR!(KVMIO, 0x65, kvm_pit_state);
pub const KVM_SET_PIT: c_ulong = _IOR!(KVMIO, 0x66, kvm_pit_state);
pub const KVM_IRQ_LINE_STATUS: c_ulong = _IOWR!(KVMIO, 0x67, kvm_irq_level);
pub const KVM_REGISTER_COALESCED_MMIO: c_ulong = _IOW!(KVMIO, 0x67, kvm_coalesced_mmio_zone);
pub const KVM_UNREGISTER_COALESCED_MMIO: c_ulong = _IOW!(KVMIO, 0x68, kvm_coalesced_mmio_zone);
pub const KVM_SET_GSI_ROUTING: c_ulong = _IOW!(KVMIO, 0x6a, kvm_irq_routing);
pub const KVM_REINJECT_CONTROL: c_ulong = _IO!(KVMIO, 0x71);
pub const KVM_IRQFD: c_ulong = _IOW!(KVMIO, 0x76, kvm_irqfd);
pub const KVM_CREATE_PIT2: c_ulong = _IOW!(KVMIO, 0x77, kvm_pit_config);
pub const KVM_SET_BOOT_CPU_ID: c_ulong = _IO!(KVMIO, 0x78);
pub const KVM_IOEVENTFD: c_ulong = _IOW!(KVMIO, 0x79, kvm_ioeventfd);
pub const KVM_XEN_HVM_CONFIG: c_ulong = _IOW!(KVMIO, 0x7a, kvm_xen_hvm_config);
pub const KVM_SET_CLOCK: c_ulong = _IOW!(KVMIO, 0x7b, kvm_clock_data);
pub const KVM_GET_CLOCK: c_ulong = _IOR!(KVMIO, 0x7c, kvm_clock_data);
pub const KVM_GET_PIT2: c_ulong = _IOR!(KVMIO, 0x9f, kvm_pit_state2);
pub const KVM_SET_PIT2: c_ulong = _IOW!(KVMIO, 0xa0, kvm_pit_state2);
pub const KVM_PPC_GET_PVINFO: c_ulong = _IOW!(KVMIO, 0xa1, kvm_ppc_pvinfo);
pub const KVM_SET_TSC_KHZ: c_ulong = _IO!(KVMIO, 0xa2);
pub const KVM_GET_TSC_KHZ: c_ulong = _IO!(KVMIO, 0xa3);
pub const KVM_SIGNAL_MSI: c_ulong = _IOW!(KVMIO, 0xa5, kvm_msi);
pub const KVM_PPC_GET_SMMU_INFO: c_ulong = _IOR!(KVMIO, 0xa6, kvm_ppc_smmu_info);
pub const KVM_PPC_ALLOCATE_HTAB: c_ulong = _IOWR!(KVMIO, 0xa7, __u32);
pub const KVM_CREATE_SPAPR_TCE: c_ulong = _IOW!(KVMIO, 0xa8, kvm_create_spapr_tce);
pub const KVM_CREATE_SPAPR_TCE_64: c_ulong = _IOW!(KVMIO, 0xa8, kvm_create_spapr_tce_64);
pub const KVM_ALLOCATE_RMA: c_ulong = _IOR!(KVMIO, 0xa9, kvm_allocate_rma);
pub const KVM_PPC_GET_HTAB_FD: c_ulong = _IOW!(KVMIO, 0xaa, kvm_get_htab_fd);
pub const KVM_ARM_SET_DEVICE_ADDR: c_ulong = _IOW!(KVMIO, 0xab, kvm_arm_device_addr);
pub const KVM_PPC_RTAS_DEFINE_TOKEN: c_ulong = _IOW!(KVMIO, 0xac, kvm_rtas_token_args);
pub const KVM_PPC_RESIZE_HPT_PREPARE: c_ulong = _IOR!(KVMIO, 0xad, kvm_ppc_resize_hpt);
pub const KVM_PPC_RESIZE_HPT_COMMIT: c_ulong = _IOR!(KVMIO, 0xae, kvm_ppc_resize_hpt);
pub const KVM_PPC_CONFIGURE_V3_MMU: c_ulong = _IOW!(KVMIO, 0xaf, kvm_ppc_mmuv3_cfg);
pub const KVM_PPC_GET_RMMU_INFO: c_ulong = _IOW!(KVMIO, 0xb0, kvm_ppc_rmmu_info);
pub const KVM_PPC_GET_CPU_CHAR: c_ulong = _IOR!(KVMIO, 0xb1, kvm_ppc_cpu_char);
pub const KVM_SET_PMU_EVENT_FILTER: c_ulong = _IOW!(KVMIO, 0xb2, kvm_pmu_event_filter);
pub const KVM_PPC_SVM_OFF: c_ulong = _IO!(KVMIO, 0xb3);
pub const KVM_ARM_MTE_COPY_TAGS: c_ulong = _IOR!(KVMIO, 0xb4, kvm_arm_copy_mte_tags);
pub const KVM_ARM_SET_COUNTER_OFFSET: c_ulong = _IOW!(KVMIO, 0xb5, kvm_arm_counter_offset);
pub const KVM_ARM_GET_REG_WRITABLE_MASKS: c_ulong = _IOR!(KVMIO, 0xb6, reg_mask_range);
pub const KVM_CREATE_DEVICE: c_ulong = _IOWR!(KVMIO, 0xe0, kvm_create_device);
pub const KVM_SET_DEVICE_ATTR: c_ulong = _IOW!(KVMIO, 0xe1, kvm_device_attr);
pub const KVM_GET_DEVICE_ATTR: c_ulong = _IOW!(KVMIO, 0xe2, kvm_device_attr);
pub const KVM_HAS_DEVICE_ATTR: c_ulong = _IOW!(KVMIO, 0xe3, kvm_device_attr);

pub const KVM_RUN: c_ulong = _IO!(KVMIO, 0x80);
pub const KVM_GET_REGS: c_ulong = _IOR!(KVMIO, 0x81, kvm_regs);
pub const KVM_SET_REGS: c_ulong = _IOW!(KVMIO, 0x82, kvm_regs);
pub const KVM_GET_SREGS: c_ulong = _IOR!(KVMIO, 0x83, kvm_sregs);
pub const KVM_SET_SREGS: c_ulong = _IOW!(KVMIO, 0x84, kvm_sregs);
pub const KVM_TRANSLATE: c_ulong = _IOWR!(KVMIO, 0x85, kvm_translation);
pub const KVM_INTERRUPT: c_ulong = _IOW!(KVMIO, 0x86, kvm_interrupt);
pub const KVM_GET_MSRS: c_ulong = _IOWR!(KVMIO, 0x88, kvm_msrs);
pub const KVM_SET_MSRS: c_ulong = _IOW!(KVMIO, 0x89, kvm_msrs);
pub const KVM_SET_CPUID: c_ulong = _IOW!(KVMIO, 0x8a, kvm_cpuid);
pub const KVM_SET_SIGNAL_MASK: c_ulong = _IOW!(KVMIO, 0x8b, kvm_signal_mask);
pub const KVM_GET_FPU: c_ulong = _IOR!(KVMIO, 0x8c, kvm_fpu);
pub const KVM_SET_FPU: c_ulong = _IOW!(KVMIO, 0x8d, kvm_fpu);
pub const KVM_GET_LAPIC: c_ulong = _IOR!(KVMIO, 0x8e, kvm_lapic_state);
pub const KVM_SET_LAPIC: c_ulong = _IOW!(KVMIO, 0x8f, kvm_lapic_state);
pub const KVM_SET_CPUID2: c_ulong = _IOW!(KVMIO, 0x90, kvm_cpuid2);
pub const KVM_GET_CPUID2: c_ulong = _IOWR!(KVMIO, 0x91, kvm_cpuid2);
pub const KVM_TPR_ACCESS_REPORTING: c_ulong = _IOWR!(KVMIO, 0x92, kvm_tpr_access_ctl);
pub const KVM_SET_VAPIC_ADDR: c_ulong = _IOW!(KVMIO, 0x93, kvm_vapic_addr);
pub const KVM_S390_INTERRUPT: c_ulong = _IOW!(KVMIO, 0x94, kvm_s390_interrupt);
pub const KVM_S390_STORE_STATUS_NOADDR: c_ulong = (!0usize);
pub const KVM_S390_STORE_STATUS_PREFIXED: c_ulong = (!1usize);
pub const KVM_S390_STORE_STATUS: c_ulong = _IOW!(KVMIO, 0x95, c_ulong);
pub const KVM_S390_SET_INITIAL_PSW: c_ulong = _IOW!(KVMIO, 0x96, kvm_s390_psw);
pub const KVM_S390_INITIAL_RESET: c_ulong = _IO!(KVMIO, 0x97);
pub const KVM_GET_MP_STATE: c_ulong = _IOR!(KVMIO, 0x98, kvm_mp_state);
pub const KVM_SET_MP_STATE: c_ulong = _IOW!(KVMIO, 0x99, kvm_mp_state);
pub const KVM_NMI: c_ulong = _IO!(KVMIO, 0x9a);
pub const KVM_SET_GUEST_DEBUG: c_ulong = _IOW!(KVMIO, 0x9b, kvm_guest_debug);
pub const KVM_X86_SETUP_MCE: c_ulong = _IOW!(KVMIO, 0x9c, __u64);
pub const KVM_X86_GET_MCE_CAP_SUPPORTED: c_ulong = _IOR!(KVMIO, 0x9d, __u64);
pub const KVM_X86_SET_MCE: c_ulong = _IOW!(KVMIO, 0x9e, kvm_x86_mce);
pub const KVM_GET_VCPU_EVENTS: c_ulong = _IOR!(KVMIO, 0x9f, kvm_vcpu_events);
pub const KVM_SET_VCPU_EVENTS: c_ulong = _IOW!(KVMIO, 0xa0, kvm_vcpu_events);
pub const KVM_GET_DEBUGREGS: c_ulong = _IOR!(KVMIO, 0xa1, kvm_debugregs);
pub const KVM_SET_DEBUGREGS: c_ulong = _IOW!(KVMIO, 0xa2, kvm_debugregs);
pub const KVM_ENABLE_CAP_IOCTL: c_ulong = _IOW!(KVMIO, 0xa3, kvm_enable_cap);
pub const KVM_GET_XSAVE: c_ulong = _IOR!(KVMIO, 0xa4, kvm_xsave);
pub const KVM_SET_XSAVE: c_ulong = _IOW!(KVMIO, 0xa5, kvm_xsave);
pub const KVM_GET_XCRS: c_ulong = _IOR!(KVMIO, 0xa6, kvm_xcrs);
pub const KVM_SET_XCRS: c_ulong = _IOW!(KVMIO, 0xa7, kvm_xcrs);
pub const KVM_DIRTY_TLB: c_ulong = _IOW!(KVMIO, 0xaa, kvm_dirty_tlb);
pub const KVM_GET_ONE_REG: c_ulong = _IOW!(KVMIO, 0xab, kvm_one_reg);
pub const KVM_SET_ONE_REG: c_ulong = _IOW!(KVMIO, 0xac, kvm_one_reg);
pub const KVM_KVMCLOCK_CTRL: c_ulong = _IO!(KVMIO, 0xad);
pub const KVM_ARM_VCPU_INIT: c_ulong = _IOW!(KVMIO, 0xae, kvm_vcpu_init);
pub const KVM_ARM_PREFERRED_TARGET: c_ulong = _IOR!(KVMIO, 0xaf, kvm_vcpu_init);
pub const KVM_GET_REG_LIST: c_ulong = _IOWR!(KVMIO, 0xb0, kvm_reg_list);
pub const KVM_S390_MEM_OP: c_ulong = _IOW!(KVMIO, 0xb1, kvm_s390_mem_op);
pub const KVM_S390_GET_SKEYS: c_ulong = _IOW!(KVMIO, 0xb2, kvm_s390_skeys);
pub const KVM_S390_SET_SKEYS: c_ulong = _IOW!(KVMIO, 0xb3, kvm_s390_skeys);
pub const KVM_S390_IRQ: c_ulong = _IOW!(KVMIO, 0xb4, kvm_s390_irq);
pub const KVM_S390_SET_IRQ_STATE: c_ulong = _IOW!(KVMIO, 0xb5, kvm_s390_irq_state);
pub const KVM_S390_GET_IRQ_STATE: c_ulong = _IOW!(KVMIO, 0xb6, kvm_s390_irq_state);
pub const KVM_SMI: c_ulong = _IO!(KVMIO, 0xb7);
pub const KVM_S390_GET_CMMA_BITS: c_ulong = _IOWR!(KVMIO, 0xb8, kvm_s390_cmma_log);
pub const KVM_S390_SET_CMMA_BITS: c_ulong = _IOW!(KVMIO, 0xb9, kvm_s390_cmma_log);
pub const KVM_MEMORY_ENCRYPT_OP: c_ulong = _IOWR!(KVMIO, 0xba, c_ulong);

#[repr(C)]
pub struct kvm_enc_region {
    pub addr: __u64,
    pub size: __u64,
}
pub const KVM_MEMORY_ENCRYPT_REG_REGION: c_ulong = _IOR!(KVMIO, 0xbb, kvm_enc_region);
pub const KVM_MEMORY_ENCRYPT_UNREG_REGION: c_ulong = _IOR!(KVMIO, 0xbc, kvm_enc_region);
pub const KVM_HYPERV_EVENTFD: c_ulong = _IOW!(KVMIO, 0xbd, kvm_hyperv_eventfd);
pub const KVM_GET_NESTED_STATE: c_ulong = _IOWR!(KVMIO, 0xbe, kvm_nested_state);
pub const KVM_SET_NESTED_STATE: c_ulong = _IOW!(KVMIO, 0xbf, kvm_nested_state);
pub const KVM_CLEAR_DIRTY_LOG: c_ulong = _IOWR!(KVMIO, 0xc0, kvm_clear_dirty_log);
pub const KVM_GET_SUPPORTED_HV_CPUID: c_ulong = _IOWR!(KVMIO, 0xc1, kvm_cpuid2);
pub const KVM_ARM_VCPU_FINALIZE: c_ulong = _IOW!(KVMIO, 0xc2, c_int);
pub const KVM_S390_NORMAL_RESET: c_ulong = _IO!(KVMIO, 0xc3);
pub const KVM_S390_CLEAR_RESET: c_ulong = _IO!(KVMIO, 0xc4);
pub const KVM_S390_PV_COMMAND: c_ulong = _IOWR!(KVMIO, 0xc5, kvm_pv_cmd);
pub const KVM_X86_SET_MSR_FILTER: c_ulong = _IOW!(KVMIO, 0xc6, kvm_msr_filter);
pub const KVM_RESET_DIRTY_RINGS: c_ulong = _IO!(KVMIO, 0xc7);
pub const KVM_XEN_HVM_GET_ATTR: c_ulong = _IOWR!(KVMIO, 0xc8, kvm_xen_hvm_attr);
pub const KVM_XEN_HVM_SET_ATTR: c_ulong = _IOW!(KVMIO, 0xc9, kvm_xen_hvm_attr);
pub const KVM_XEN_VCPU_GET_ATTR: c_ulong = _IOWR!(KVMIO, 0xca, kvm_xen_vcpu_attr);
pub const KVM_XEN_VCPU_SET_ATTR: c_ulong = _IOW!(KVMIO, 0xcb, kvm_xen_vcpu_attr);
pub const KVM_XEN_HVM_EVTCHN_SEND: c_ulong = _IOW!(KVMIO, 0xd0, kvm_irq_routing_xen_evtchn);
pub const KVM_GET_SREGS2: c_ulong = _IOR!(KVMIO, 0xcc, kvm_sregs2);
pub const KVM_SET_SREGS2: c_ulong = _IOW!(KVMIO, 0xcd, kvm_sregs2);

pub const KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE: __u32 = 1 << 0;
pub const KVM_DIRTY_LOG_INITIALLY_SET: __u32 = 1 << 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: __u32 = 0; /* default when arch has not defined it */
pub const KVM_DIRTY_GFN_F_DIRTY: c_ulong = 1usize << 0;
pub const KVM_DIRTY_GFN_F_RESET: c_ulong = 1usize << 1;
pub const KVM_DIRTY_GFN_F_MASK: __u32 = 0x3;

#[repr(C)]
pub struct kvm_dirty_gfn {
    pub flags: __u32,
    pub slot: __u32,
    pub offset: __u64,
}

pub const KVM_BUS_LOCK_DETECTION_OFF: __u32 = 1 << 0;
pub const KVM_BUS_LOCK_DETECTION_EXIT: __u32 = 1 << 1;
pub const KVM_PMU_CAP_DISABLE: __u32 = 1 << 0;

/**
 * struct kvm_stats_header - Header of per vm/vcpu binary statistics data.
 */
#[repr(C)]
pub struct kvm_stats_header {
    pub flags: __u32,
    pub name_size: __u32,
    pub num_desc: __u32,
    pub id_offset: __u32,
    pub desc_offset: __u32,
    pub data_offset: __u32,
}

pub const KVM_STATS_TYPE_SHIFT: __u32 = 0;
pub const KVM_STATS_TYPE_MASK: __u32 = 0xF << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_CUMULATIVE: __u32 = 0x0 << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_INSTANT: __u32 = 0x1 << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_PEAK: __u32 = 0x2 << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_LINEAR_HIST: __u32 = 0x3 << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_LOG_HIST: __u32 = 0x4 << KVM_STATS_TYPE_SHIFT;
pub const KVM_STATS_TYPE_MAX: __u32 = KVM_STATS_TYPE_LOG_HIST;
pub const KVM_STATS_UNIT_SHIFT: __u32 = 4;
pub const KVM_STATS_UNIT_MASK: __u32 = 0xF << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_NONE: __u32 = 0x0 << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_BYTES: __u32 = 0x1 << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_SECONDS: __u32 = 0x2 << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_CYCLES: __u32 = 0x3 << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_BOOLEAN: __u32 = 0x4 << KVM_STATS_UNIT_SHIFT;
pub const KVM_STATS_UNIT_MAX: __u32 = KVM_STATS_UNIT_BOOLEAN;
pub const KVM_STATS_BASE_SHIFT: __u32 = 8;
pub const KVM_STATS_BASE_MASK: __u32 = 0xF << KVM_STATS_BASE_SHIFT;
pub const KVM_STATS_BASE_POW10: __u32 = 0x0 << KVM_STATS_BASE_SHIFT;
pub const KVM_STATS_BASE_POW2: __u32 = 0x1 << KVM_STATS_BASE_SHIFT;
pub const KVM_STATS_BASE_MAX: __u32 = KVM_STATS_BASE_POW2;

/**
 * struct kvm_stats_desc - Descriptor of a KVM statistics.
 */
#[repr(C)]
pub struct kvm_stats_desc {
    pub flags: __u32,
    pub exponent: __s16,
    pub size: __u16,
    pub offset: __u32,
    pub bucket_size: __u32,
    /* __KERNEL__ uses fixed name[KVM_STATS_NAME_SIZE]; userspace uses flexible array. */
    pub name: [c_char; 0],
}

pub const KVM_GET_STATS_FD: c_ulong = _IO!(KVMIO, 0xce);
pub const KVM_GET_XSAVE2: c_ulong = _IOR!(KVMIO, 0xcf, kvm_xsave);
pub const KVM_S390_PV_CPU_COMMAND: c_ulong = _IOWR!(KVMIO, 0xd0, kvm_pv_cmd);
pub const KVM_X86_NOTIFY_VMEXIT_ENABLED: __u64 = 1u64 << 0;
pub const KVM_X86_NOTIFY_VMEXIT_USER: __u64 = 1u64 << 1;
pub const KVM_S390_ZPCI_OP: c_ulong = _IOW!(KVMIO, 0xd1, kvm_s390_zpci_op);
pub const KVM_SET_MEMORY_ATTRIBUTES: c_ulong = _IOW!(KVMIO, 0xd2, kvm_memory_attributes);

#[repr(C)]
pub struct kvm_memory_attributes {
    pub address: __u64,
    pub size: __u64,
    pub attributes: __u64,
    pub flags: __u64,
}

pub const KVM_MEMORY_ATTRIBUTE_PRIVATE: __u64 = 1u64 << 3;
pub const KVM_CREATE_GUEST_MEMFD: c_ulong = _IOWR!(KVMIO, 0xd4, kvm_create_guest_memfd);
pub const GUEST_MEMFD_FLAG_MMAP: __u64 = 1u64 << 0;
pub const GUEST_MEMFD_FLAG_INIT_SHARED: __u64 = 1u64 << 1;

#[repr(C)]
pub struct kvm_create_guest_memfd {
    pub size: __u64,
    pub flags: __u64,
    pub reserved: [__u64; 6],
}

pub const KVM_PRE_FAULT_MEMORY: c_ulong = _IOWR!(KVMIO, 0xd5, kvm_pre_fault_memory);

#[repr(C)]
pub struct kvm_pre_fault_memory {
    pub gpa: __u64,
    pub size: __u64,
    pub flags: __u64,
    pub padding: [__u64; 5],
}
