/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * KVM x86 specific structures and definitions
 *
 */

/* C dependencies removed: <linux/const.h>, <linux/bits.h>, <linux/types.h>,
 * <linux/ioctl.h>, <linux/stddef.h>.
 */

pub const KVM_PIO_PAGE_OFFSET: u32 = 1;
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 2;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u32 = 64;

pub const DE_VECTOR: u32 = 0;
pub const DB_VECTOR: u32 = 1;
pub const BP_VECTOR: u32 = 3;
pub const OF_VECTOR: u32 = 4;
pub const BR_VECTOR: u32 = 5;
pub const UD_VECTOR: u32 = 6;
pub const NM_VECTOR: u32 = 7;
pub const DF_VECTOR: u32 = 8;
pub const TS_VECTOR: u32 = 10;
pub const NP_VECTOR: u32 = 11;
pub const SS_VECTOR: u32 = 12;
pub const GP_VECTOR: u32 = 13;
pub const PF_VECTOR: u32 = 14;
pub const MF_VECTOR: u32 = 16;
pub const AC_VECTOR: u32 = 17;
pub const MC_VECTOR: u32 = 18;
pub const XM_VECTOR: u32 = 19;
pub const VE_VECTOR: u32 = 20;
pub const CP_VECTOR: u32 = 21;

pub const HV_VECTOR: u32 = 28;
pub const VC_VECTOR: u32 = 29;
pub const SX_VECTOR: u32 = 30;

/* Select x86 specific features in <linux/kvm.h> */
pub const __KVM_HAVE_PIT: bool = true;
pub const __KVM_HAVE_IOAPIC: bool = true;
pub const __KVM_HAVE_IRQ_LINE: bool = true;
pub const __KVM_HAVE_MSI: bool = true;
pub const __KVM_HAVE_USER_NMI: bool = true;
pub const __KVM_HAVE_MSIX: bool = true;
pub const __KVM_HAVE_MCE: bool = true;
pub const __KVM_HAVE_PIT_STATE2: bool = true;
pub const __KVM_HAVE_XEN_HVM: bool = true;
pub const __KVM_HAVE_VCPU_EVENTS: bool = true;
pub const __KVM_HAVE_DEBUGREGS: bool = true;
pub const __KVM_HAVE_XSAVE: bool = true;
pub const __KVM_HAVE_XCRS: bool = true;

/* Architectural interrupt line count. */
pub const KVM_NR_INTERRUPTS: usize = 256;

/* for KVM_GET_IRQCHIP and KVM_SET_IRQCHIP */
#[repr(C)]
pub struct kvm_pic_state {
    pub last_irr: __u8, /* edge detection */
    pub irr: __u8,     /* interrupt request register */
    pub imr: __u8,     /* interrupt mask register */
    pub isr: __u8,     /* interrupt service register */
    pub priority_add: __u8, /* highest irq priority */
    pub irq_base: __u8,
    pub read_reg_select: __u8,
    pub poll: __u8,
    pub special_mask: __u8,
    pub init_state: __u8,
    pub auto_eoi: __u8,
    pub rotate_on_auto_eoi: __u8,
    pub special_fully_nested_mode: __u8,
    pub init4: __u8, /* true if 4 byte init */
    pub elcr: __u8,  /* PIIX edge/trigger selection */
    pub elcr_mask: __u8,
}

pub const KVM_IOAPIC_NUM_PINS: usize = 24;

#[repr(C)]
pub struct kvm_ioapic_redirtbl_fields {
    pub vector: __u8,
    /* C bitfields packed in the following bytes:
     * delivery_mode:3, dest_mode:1, delivery_status:1, polarity:1,
     * remote_irr:1, trig_mode:1, mask:1, reserve:7.
     */
    pub bitfields: [__u8; 2],
    pub reserved: [__u8; 4],
    pub dest_id: __u8,
}

#[repr(C)]
pub union kvm_ioapic_redirtbl {
    pub bits: __u64,
    pub fields: kvm_ioapic_redirtbl_fields,
}

#[repr(C)]
pub struct kvm_ioapic_state {
    pub base_address: __u64,
    pub ioregsel: __u32,
    pub id: __u32,
    pub irr: __u32,
    pub pad: __u32,
    pub redirtbl: [kvm_ioapic_redirtbl; KVM_IOAPIC_NUM_PINS],
}

pub const KVM_IRQCHIP_PIC_MASTER: u32 = 0;
pub const KVM_IRQCHIP_PIC_SLAVE: u32 = 1;
pub const KVM_IRQCHIP_IOAPIC: u32 = 2;
pub const KVM_NR_IRQCHIPS: u32 = 3;

pub const KVM_RUN_X86_SMM: u32 = 1 << 0;
pub const KVM_RUN_X86_BUS_LOCK: u32 = 1 << 1;
pub const KVM_RUN_X86_GUEST_MODE: u32 = 1 << 2;

/* for KVM_GET_REGS and KVM_SET_REGS */
#[repr(C)]
pub struct kvm_regs {
    /* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
    pub rax: __u64,
    pub rbx: __u64,
    pub rcx: __u64,
    pub rdx: __u64,
    pub rsi: __u64,
    pub rdi: __u64,
    pub rsp: __u64,
    pub rbp: __u64,
    pub r8: __u64,
    pub r9: __u64,
    pub r10: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
    pub r15: __u64,
    pub rip: __u64,
    pub rflags: __u64,
}

/* for KVM_GET_LAPIC and KVM_SET_LAPIC */
pub const KVM_APIC_REG_SIZE: usize = 0x400;
#[repr(C)]
pub struct kvm_lapic_state {
    pub regs: [::core::ffi::c_char; KVM_APIC_REG_SIZE],
}

#[repr(C)]
pub struct kvm_segment {
    pub base: __u64,
    pub limit: __u32,
    pub selector: __u16,
    pub type_: __u8,
    pub present: __u8,
    pub dpl: __u8,
    pub db: __u8,
    pub s: __u8,
    pub l: __u8,
    pub g: __u8,
    pub avl: __u8,
    pub unusable: __u8,
    pub padding: __u8,
}

#[repr(C)]
pub struct kvm_dtable {
    pub base: __u64,
    pub limit: __u16,
    pub padding: [__u16; 3],
}

/* for KVM_GET_SREGS and KVM_SET_SREGS */
#[repr(C)]
pub struct kvm_sregs {
    /* out (KVM_GET_SREGS) / in (KVM_SET_SREGS) */
    pub cs: kvm_segment,
    pub ds: kvm_segment,
    pub es: kvm_segment,
    pub fs: kvm_segment,
    pub gs: kvm_segment,
    pub ss: kvm_segment,
    pub tr: kvm_segment,
    pub ldt: kvm_segment,
    pub gdt: kvm_dtable,
    pub idt: kvm_dtable,
    pub cr0: __u64,
    pub cr2: __u64,
    pub cr3: __u64,
    pub cr4: __u64,
    pub cr8: __u64,
    pub efer: __u64,
    pub apic_base: __u64,
    pub interrupt_bitmap: [__u64; (KVM_NR_INTERRUPTS + 63) / 64],
}

#[repr(C)]
pub struct kvm_sregs2 {
    /* out (KVM_GET_SREGS2) / in (KVM_SET_SREGS2) */
    pub cs: kvm_segment,
    pub ds: kvm_segment,
    pub es: kvm_segment,
    pub fs: kvm_segment,
    pub gs: kvm_segment,
    pub ss: kvm_segment,
    pub tr: kvm_segment,
    pub ldt: kvm_segment,
    pub gdt: kvm_dtable,
    pub idt: kvm_dtable,
    pub cr0: __u64,
    pub cr2: __u64,
    pub cr3: __u64,
    pub cr4: __u64,
    pub cr8: __u64,
    pub efer: __u64,
    pub apic_base: __u64,
    pub flags: __u64,
    pub pdptrs: [__u64; 4],
}
pub const KVM_SREGS2_FLAGS_PDPTRS_VALID: u32 = 1;

/* for KVM_GET_FPU and KVM_SET_FPU */
#[repr(C)]
pub struct kvm_fpu {
    pub fpr: [[__u8; 16]; 8],
    pub fcw: __u16,
    pub fsw: __u16,
    pub ftwx: __u8, /* in fxsave format */
    pub pad1: __u8,
    pub last_opcode: __u16,
    pub last_ip: __u64,
    pub last_dp: __u64,
    pub xmm: [[__u8; 16]; 16],
    pub mxcsr: __u32,
    pub pad2: __u32,
}

#[repr(C)]
pub struct kvm_msr_entry {
    pub index: __u32,
    pub reserved: __u32,
    pub data: __u64,
}

/* for KVM_GET_MSRS and KVM_SET_MSRS */
#[repr(C)]
pub struct kvm_msrs {
    pub nmsrs: __u32, /* number of msrs in entries */
    pub pad: __u32,
    pub entries: [kvm_msr_entry; 0],
}

/* for KVM_GET_MSR_INDEX_LIST */
#[repr(C)]
pub struct kvm_msr_list {
    pub nmsrs: __u32, /* number of msrs in entries */
    pub indices: [__u32; 0],
}

/* Maximum size of any access bitmap in bytes */
pub const KVM_MSR_FILTER_MAX_BITMAP_SIZE: u32 = 0x600;

/* for KVM_X86_SET_MSR_FILTER */
pub const KVM_MSR_FILTER_READ: u32 = 1 << 0;
pub const KVM_MSR_FILTER_WRITE: u32 = 1 << 1;
pub const KVM_MSR_FILTER_RANGE_VALID_MASK: u32 = KVM_MSR_FILTER_READ | KVM_MSR_FILTER_WRITE;
#[repr(C)]
pub struct kvm_msr_filter_range {
    pub flags: __u32,
    pub nmsrs: __u32, /* number of msrs in bitmap */
    pub base: __u32,  /* MSR index the bitmap starts at */
    pub bitmap: *mut __u8, /* a 1 bit allows the operations in flags, 0 denies */
}

pub const KVM_MSR_FILTER_MAX_RANGES: usize = 16;
/* C condition: #ifndef __KERNEL__ */
pub const KVM_MSR_FILTER_DEFAULT_ALLOW: u32 = 0 << 0;
pub const KVM_MSR_FILTER_DEFAULT_DENY: u32 = 1 << 0;
pub const KVM_MSR_FILTER_VALID_MASK: u32 = KVM_MSR_FILTER_DEFAULT_DENY;
#[repr(C)]
pub struct kvm_msr_filter {
    pub flags: __u32,
    pub ranges: [kvm_msr_filter_range; KVM_MSR_FILTER_MAX_RANGES],
}

#[repr(C)]
pub struct kvm_cpuid_entry {
    pub function: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: __u32,
}

/* for KVM_SET_CPUID */
#[repr(C)]
pub struct kvm_cpuid {
    pub nent: __u32,
    pub padding: __u32,
    pub entries: [kvm_cpuid_entry; 0],
}

#[repr(C)]
pub struct kvm_cpuid_entry2 {
    pub function: __u32,
    pub index: __u32,
    pub flags: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: [__u32; 3],
}

pub const KVM_CPUID_FLAG_SIGNIFCANT_INDEX: u32 = 1 << 0;
pub const KVM_CPUID_FLAG_STATEFUL_FUNC: u32 = 1 << 1;
pub const KVM_CPUID_FLAG_STATE_READ_NEXT: u32 = 1 << 2;

/* for KVM_SET_CPUID2 */
#[repr(C)]
pub struct kvm_cpuid2 {
    pub nent: __u32,
    pub padding: __u32,
    pub entries: [kvm_cpuid_entry2; 0],
}

/* for KVM_GET_PIT and KVM_SET_PIT */
#[repr(C)]
pub struct kvm_pit_channel_state {
    pub count: __u32, /* can be 65536 */
    pub latched_count: __u16,
    pub count_latched: __u8,
    pub status_latched: __u8,
    pub status: __u8,
    pub read_state: __u8,
    pub write_state: __u8,
    pub write_latch: __u8,
    pub rw_mode: __u8,
    pub mode: __u8,
    pub bcd: __u8,
    pub gate: __u8,
    pub count_load_time: __s64,
}

#[repr(C)]
pub struct kvm_debug_exit_arch {
    pub exception: __u32,
    pub pad: __u32,
    pub pc: __u64,
    pub dr6: __u64,
    pub dr7: __u64,
}

pub const KVM_GUESTDBG_USE_SW_BP: u32 = 0x00010000;
pub const KVM_GUESTDBG_USE_HW_BP: u32 = 0x00020000;
pub const KVM_GUESTDBG_INJECT_DB: u32 = 0x00040000;
pub const KVM_GUESTDBG_INJECT_BP: u32 = 0x00080000;
pub const KVM_GUESTDBG_BLOCKIRQ: u32 = 0x00100000;

/* for KVM_SET_GUEST_DEBUG */
#[repr(C)]
pub struct kvm_guest_debug_arch {
    pub debugreg: [__u64; 8],
}

#[repr(C)]
pub struct kvm_pit_state {
    pub channels: [kvm_pit_channel_state; 3],
}

pub const KVM_PIT_FLAGS_HPET_LEGACY: u32 = 0x00000001;
pub const KVM_PIT_FLAGS_SPEAKER_DATA_ON: u32 = 0x00000002;

#[repr(C)]
pub struct kvm_pit_state2 {
    pub channels: [kvm_pit_channel_state; 3],
    pub flags: __u32,
    pub reserved: [__u32; 9],
}

#[repr(C)]
pub struct kvm_reinject_control {
    pub pit_reinject: __u8,
    pub reserved: [__u8; 31],
}

/* When set in flags, include corresponding fields on KVM_SET_VCPU_EVENTS */
pub const KVM_VCPUEVENT_VALID_NMI_PENDING: u32 = 0x00000001;
pub const KVM_VCPUEVENT_VALID_SIPI_VECTOR: u32 = 0x00000002;
pub const KVM_VCPUEVENT_VALID_SHADOW: u32 = 0x00000004;
pub const KVM_VCPUEVENT_VALID_SMM: u32 = 0x00000008;
pub const KVM_VCPUEVENT_VALID_PAYLOAD: u32 = 0x00000010;
pub const KVM_VCPUEVENT_VALID_TRIPLE_FAULT: u32 = 0x00000020;

/* Interrupt shadow states */
pub const KVM_X86_SHADOW_INT_MOV_SS: u32 = 0x01;
pub const KVM_X86_SHADOW_INT_STI: u32 = 0x02;

#[repr(C)]
pub struct kvm_vcpu_events_exception {
    pub injected: __u8,
    pub nr: __u8,
    pub has_error_code: __u8,
    pub pending: __u8,
    pub error_code: __u32,
}

#[repr(C)]
pub struct kvm_vcpu_events_interrupt {
    pub injected: __u8,
    pub nr: __u8,
    pub soft: __u8,
    pub shadow: __u8,
}

#[repr(C)]
pub struct kvm_vcpu_events_nmi {
    pub injected: __u8,
    pub pending: __u8,
    pub masked: __u8,
    pub pad: __u8,
}

#[repr(C)]
pub struct kvm_vcpu_events_smi {
    pub smm: __u8,
    pub pending: __u8,
    pub smm_inside_nmi: __u8,
    pub latched_init: __u8,
}

#[repr(C)]
pub struct kvm_vcpu_events_triple_fault {
    pub pending: __u8,
}

/* for KVM_GET/SET_VCPU_EVENTS */
#[repr(C)]
pub struct kvm_vcpu_events {
    pub exception: kvm_vcpu_events_exception,
    pub interrupt: kvm_vcpu_events_interrupt,
    pub nmi: kvm_vcpu_events_nmi,
    pub sipi_vector: __u32,
    pub flags: __u32,
    pub smi: kvm_vcpu_events_smi,
    pub triple_fault: kvm_vcpu_events_triple_fault,
    pub reserved: [__u8; 26],
    pub exception_has_payload: __u8,
    pub exception_payload: __u64,
}

/* for KVM_GET/SET_DEBUGREGS */
#[repr(C)]
pub struct kvm_debugregs {
    pub db: [__u64; 4],
    pub dr6: __u64,
    pub dr7: __u64,
    pub flags: __u64,
    pub reserved: [__u64; 9],
}

/* for KVM_CAP_XSAVE and KVM_CAP_XSAVE2 */
#[repr(C)]
pub struct kvm_xsave {
    /*
     * KVM_GET_XSAVE2 and KVM_SET_XSAVE write and read as many bytes
     * as are returned by KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
     * respectively, when invoked on the vm file descriptor.
     *
     * The size value returned by KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
     * will always be at least 4096. Currently, it is only greater
     * than 4096 if a dynamic feature has been enabled with
     * ``arch_prctl()``, but this may change in the future.
     *
     * The offsets of the state save areas in struct kvm_xsave follow
     * the contents of CPUID leaf 0xD on the host.
     */
    pub region: [__u32; 1024],
    pub extra: [__u32; 0],
}

pub const KVM_MAX_XCRS: usize = 16;

#[repr(C)]
pub struct kvm_xcr {
    pub xcr: __u32,
    pub reserved: __u32,
    pub value: __u64,
}

#[repr(C)]
pub struct kvm_xcrs {
    pub nr_xcrs: __u32,
    pub flags: __u32,
    pub xcrs: [kvm_xcr; KVM_MAX_XCRS],
    pub padding: [__u64; 16],
}

pub const KVM_X86_REG_TYPE_MSR: u64 = 2;
pub const KVM_X86_REG_TYPE_KVM: u64 = 3;

pub const fn KVM_X86_KVM_REG_SIZE(reg: __u64) -> __u64 {
    if reg == KVM_REG_GUEST_SSP {
        KVM_REG_SIZE_U64
    } else {
        0
    }
}

pub const fn KVM_X86_REG_TYPE_SIZE(type_: __u64, reg: __u64) -> __u64 {
    let mut type_size: __u64 = type_ << 32;
    type_size |= if type_ == KVM_X86_REG_TYPE_MSR {
        KVM_REG_SIZE_U64
    } else if type_ == KVM_X86_REG_TYPE_KVM {
        KVM_X86_KVM_REG_SIZE(reg)
    } else {
        0
    };
    type_size
}

pub const fn KVM_X86_REG_ID(type_: __u64, index: __u64) -> __u64 {
    KVM_REG_X86 | KVM_X86_REG_TYPE_SIZE(type_, index) | index
}

pub const fn KVM_X86_REG_MSR(index: __u64) -> __u64 {
    KVM_X86_REG_ID(KVM_X86_REG_TYPE_MSR, index)
}

pub const fn KVM_X86_REG_KVM(index: __u64) -> __u64 {
    KVM_X86_REG_ID(KVM_X86_REG_TYPE_KVM, index)
}

/* KVM-defined registers starting from 0 */
pub const KVM_REG_GUEST_SSP: __u64 = 0;

pub const KVM_SYNC_X86_REGS: usize = 1usize << 0;
pub const KVM_SYNC_X86_SREGS: usize = 1usize << 1;
pub const KVM_SYNC_X86_EVENTS: usize = 1usize << 2;

pub const KVM_SYNC_X86_VALID_FIELDS: usize =
    KVM_SYNC_X86_REGS | KVM_SYNC_X86_SREGS | KVM_SYNC_X86_EVENTS;

/* kvm_sync_regs struct included by kvm_run struct */
#[repr(C)]
pub struct kvm_sync_regs {
    /* Members of this structure are potentially malicious.
     * Care must be taken by code reading, esp. interpreting,
     * data fields from them inside KVM to prevent TOCTOU and
     * double-fetch types of vulnerabilities.
     */
    pub regs: kvm_regs,
    pub sregs: kvm_sregs,
    pub events: kvm_vcpu_events,
}

pub const KVM_X86_QUIRK_LINT0_REENABLED: u32 = 1 << 0;
pub const KVM_X86_QUIRK_CD_NW_CLEARED: u32 = 1 << 1;
pub const KVM_X86_QUIRK_LAPIC_MMIO_HOLE: u32 = 1 << 2;
pub const KVM_X86_QUIRK_OUT_7E_INC_RIP: u32 = 1 << 3;
pub const KVM_X86_QUIRK_MISC_ENABLE_NO_MWAIT: u32 = 1 << 4;
pub const KVM_X86_QUIRK_FIX_HYPERCALL_INSN: u32 = 1 << 5;
pub const KVM_X86_QUIRK_MWAIT_NEVER_UD_FAULTS: u32 = 1 << 6;
pub const KVM_X86_QUIRK_SLOT_ZAP_ALL: u32 = 1 << 7;
pub const KVM_X86_QUIRK_STUFF_FEATURE_MSRS: u32 = 1 << 8;
pub const KVM_X86_QUIRK_IGNORE_GUEST_PAT: u32 = 1 << 9;
pub const KVM_X86_QUIRK_VMCS12_ALLOW_FREEZE_IN_SMM: u32 = 1 << 10;
pub const KVM_X86_QUIRK_NESTED_SVM_SHARED_PAT: u32 = 1 << 11;

pub const KVM_STATE_NESTED_FORMAT_VMX: u32 = 0;
pub const KVM_STATE_NESTED_FORMAT_SVM: u32 = 1;

pub const KVM_STATE_NESTED_GUEST_MODE: u32 = 0x00000001;
pub const KVM_STATE_NESTED_RUN_PENDING: u32 = 0x00000002;
pub const KVM_STATE_NESTED_EVMCS: u32 = 0x00000004;
pub const KVM_STATE_NESTED_MTF_PENDING: u32 = 0x00000008;
pub const KVM_STATE_NESTED_GIF_SET: u32 = 0x00000100;

pub const KVM_STATE_NESTED_SMM_GUEST_MODE: u32 = 0x00000001;
pub const KVM_STATE_NESTED_SMM_VMXON: u32 = 0x00000002;

pub const KVM_STATE_NESTED_VMX_VMCS_SIZE: usize = 0x1000;

pub const KVM_STATE_NESTED_SVM_VMCB_SIZE: usize = 0x1000;

pub const KVM_STATE_VMX_PREEMPTION_TIMER_DEADLINE: u32 = 0x00000001;

/* vendor-independent attributes for system fd (group 0) */
pub const KVM_X86_GRP_SYSTEM: u32 = 0;
pub const KVM_X86_XCOMP_GUEST_SUPP: u32 = 0;

/* vendor-specific groups and attributes for system fd */
pub const KVM_X86_GRP_SEV: u32 = 1;
pub const KVM_X86_SEV_VMSA_FEATURES: u32 = 0;
pub const KVM_X86_SNP_POLICY_BITS: u32 = 1;
pub const KVM_X86_SEV_SNP_REQ_CERTS: u32 = 2;

#[repr(C)]
pub struct kvm_vmx_nested_state_data {
    pub vmcs12: [__u8; KVM_STATE_NESTED_VMX_VMCS_SIZE],
    pub shadow_vmcs12: [__u8; KVM_STATE_NESTED_VMX_VMCS_SIZE],
}

#[repr(C)]
pub struct kvm_vmx_nested_state_hdr_smm {
    pub flags: __u16,
}

#[repr(C)]
pub struct kvm_vmx_nested_state_hdr {
    pub vmxon_pa: __u64,
    pub vmcs12_pa: __u64,
    pub smm: kvm_vmx_nested_state_hdr_smm,
    pub pad: __u16,
    pub flags: __u32,
    pub preemption_timer_deadline: __u64,
}

#[repr(C)]
pub struct kvm_svm_nested_state_data {
    /* Save area only used if KVM_STATE_NESTED_RUN_PENDING.  */
    pub vmcb12: [__u8; KVM_STATE_NESTED_SVM_VMCB_SIZE],
}

#[repr(C)]
pub struct kvm_svm_nested_state_hdr {
    pub vmcb_pa: __u64,
    pub gpat: __u64,
}

#[repr(C)]
pub union kvm_nested_state_hdr {
    pub vmx: ::core::mem::ManuallyDrop<kvm_vmx_nested_state_hdr>,
    pub svm: ::core::mem::ManuallyDrop<kvm_svm_nested_state_hdr>,
    /* Pad the header to 128 bytes.  */
    pub pad: [__u8; 120],
}

#[repr(C)]
pub union kvm_nested_state_data {
    pub vmx: [kvm_vmx_nested_state_data; 0],
    pub svm: [kvm_svm_nested_state_data; 0],
}

/* for KVM_CAP_NESTED_STATE */
#[repr(C)]
pub struct kvm_nested_state {
    pub flags: __u16,
    pub format: __u16,
    pub size: __u32,
    pub hdr: kvm_nested_state_hdr,
    /*
     * Define data region as 0 bytes to preserve backwards-compatability
     * to old definition of kvm_nested_state in order to avoid changing
     * KVM_{GET,PUT}_NESTED_STATE ioctl values.
     */
    pub data: kvm_nested_state_data,
}

/* for KVM_CAP_PMU_EVENT_FILTER */
#[repr(C)]
pub struct kvm_pmu_event_filter {
    pub action: __u32,
    pub nevents: __u32,
    pub fixed_counter_bitmap: __u32,
    pub flags: __u32,
    pub pad: [__u32; 4],
    pub events: [__u64; 0],
}

pub const KVM_PMU_EVENT_ALLOW: u32 = 0;
pub const KVM_PMU_EVENT_DENY: u32 = 1;

pub const KVM_PMU_EVENT_FLAG_MASKED_EVENTS: u64 = _BITUL(0);
pub const KVM_PMU_EVENT_FLAGS_VALID_MASK: u64 = KVM_PMU_EVENT_FLAG_MASKED_EVENTS;

/* for KVM_CAP_MCE */
#[repr(C)]
pub struct kvm_x86_mce {
    pub status: __u64,
    pub addr: __u64,
    pub misc: __u64,
    pub mcg_status: __u64,
    pub bank: __u8,
    pub pad1: [__u8; 7],
    pub pad2: [__u64; 3],
}

/* for KVM_CAP_XEN_HVM */
pub const KVM_XEN_HVM_CONFIG_HYPERCALL_MSR: u32 = 1 << 0;
pub const KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL: u32 = 1 << 1;
pub const KVM_XEN_HVM_CONFIG_SHARED_INFO: u32 = 1 << 2;
pub const KVM_XEN_HVM_CONFIG_RUNSTATE: u32 = 1 << 3;
pub const KVM_XEN_HVM_CONFIG_EVTCHN_2LEVEL: u32 = 1 << 4;
pub const KVM_XEN_HVM_CONFIG_EVTCHN_SEND: u32 = 1 << 5;
pub const KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG: u32 = 1 << 6;
pub const KVM_XEN_HVM_CONFIG_PVCLOCK_TSC_UNSTABLE: u32 = 1 << 7;
pub const KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA: u32 = 1 << 8;

pub const KVM_XEN_MSR_MIN_INDEX: u32 = 0x40000000u32;
pub const KVM_XEN_MSR_MAX_INDEX: u32 = 0x4fffffffu32;

#[repr(C)]
pub struct kvm_xen_hvm_config {
    pub flags: __u32,
    pub msr: __u32,
    pub blob_addr_32: __u64,
    pub blob_addr_64: __u64,
    pub blob_size_32: __u8,
    pub blob_size_64: __u8,
    pub pad2: [__u8; 30],
}

pub const KVM_XEN_INVALID_GFN: __u64 = !0u64;
pub const KVM_XEN_EVTCHN_DEASSIGN: u32 = 1 << 0;
pub const KVM_XEN_EVTCHN_UPDATE: u32 = 1 << 1;
pub const KVM_XEN_EVTCHN_RESET: u32 = 1 << 2;

#[repr(C)]
pub union kvm_xen_hvm_attr_shared_info {
    pub gfn: __u64,
    pub hva: __u64,
}

#[repr(C)]
pub struct kvm_xen_hvm_attr_evtchn_deliver_port {
    pub port: __u32,
    pub vcpu: __u32,
    pub priority: __u32,
}

#[repr(C)]
pub struct kvm_xen_hvm_attr_evtchn_deliver_eventfd {
    pub port: __u32, /* Zero for eventfd */
    pub fd: __s32,
}

#[repr(C)]
pub union kvm_xen_hvm_attr_evtchn_deliver {
    pub port: ::core::mem::ManuallyDrop<kvm_xen_hvm_attr_evtchn_deliver_port>,
    pub eventfd: ::core::mem::ManuallyDrop<kvm_xen_hvm_attr_evtchn_deliver_eventfd>,
    pub padding: [__u32; 4],
}

#[repr(C)]
pub struct kvm_xen_hvm_attr_evtchn {
    pub send_port: __u32,
    pub type_: __u32, /* EVTCHNSTAT_ipi / EVTCHNSTAT_interdomain */
    pub flags: __u32,
    /*
     * Events sent by the guest are either looped back to
     * the guest itself (potentially on a different port#)
     * or signalled via an eventfd.
     */
    pub deliver: kvm_xen_hvm_attr_evtchn_deliver,
}

#[repr(C)]
pub union kvm_xen_hvm_attr_u {
    pub long_mode: __u8,
    pub vector: __u8,
    pub runstate_update_flag: __u8,
    pub shared_info: kvm_xen_hvm_attr_shared_info,
    pub evtchn: ::core::mem::ManuallyDrop<kvm_xen_hvm_attr_evtchn>,
    pub xen_version: __u32,
    pub pad: [__u64; 8],
}

#[repr(C)]
pub struct kvm_xen_hvm_attr {
    pub type_: __u16,
    pub pad: [__u16; 3],
    pub u: kvm_xen_hvm_attr_u,
}

/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO */
pub const KVM_XEN_ATTR_TYPE_LONG_MODE: u32 = 0x0;
pub const KVM_XEN_ATTR_TYPE_SHARED_INFO: u32 = 0x1;
pub const KVM_XEN_ATTR_TYPE_UPCALL_VECTOR: u32 = 0x2;
/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND */
pub const KVM_XEN_ATTR_TYPE_EVTCHN: u32 = 0x3;
pub const KVM_XEN_ATTR_TYPE_XEN_VERSION: u32 = 0x4;
/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG */
pub const KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG: u32 = 0x5;
/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA */
pub const KVM_XEN_ATTR_TYPE_SHARED_INFO_HVA: u32 = 0x6;

pub const KVM_XEN_INVALID_GPA: __u64 = !0u64;

#[repr(C)]
pub struct kvm_xen_vcpu_attr_runstate {
    pub state: __u64,
    pub state_entry_time: __u64,
    pub time_running: __u64,
    pub time_runnable: __u64,
    pub time_blocked: __u64,
    pub time_offline: __u64,
}

#[repr(C)]
pub struct kvm_xen_vcpu_attr_timer {
    pub port: __u32,
    pub priority: __u32,
    pub expires_ns: __u64,
}

#[repr(C)]
pub union kvm_xen_vcpu_attr_u {
    pub gpa: __u64,
    pub hva: __u64,
    pub pad: [__u64; 8],
    pub runstate: ::core::mem::ManuallyDrop<kvm_xen_vcpu_attr_runstate>,
    pub vcpu_id: __u32,
    pub timer: ::core::mem::ManuallyDrop<kvm_xen_vcpu_attr_timer>,
    pub vector: __u8,
}

#[repr(C)]
pub struct kvm_xen_vcpu_attr {
    pub type_: __u16,
    pub pad: [__u16; 3],
    pub u: kvm_xen_vcpu_attr_u,
}

/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO */
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO: u32 = 0x0;
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_TIME_INFO: u32 = 0x1;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR: u32 = 0x2;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_CURRENT: u32 = 0x3;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_DATA: u32 = 0x4;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST: u32 = 0x5;
/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND */
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID: u32 = 0x6;
pub const KVM_XEN_VCPU_ATTR_TYPE_TIMER: u32 = 0x7;
pub const KVM_XEN_VCPU_ATTR_TYPE_UPCALL_VECTOR: u32 = 0x8;
/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA */
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA: u32 = 0x9;

/* Secure Encrypted Virtualization command */
#[repr(C)]
pub enum sev_cmd_id {
    /* Guest initialization commands */
    KVM_SEV_INIT = 0,
    KVM_SEV_ES_INIT,
    /* Guest launch commands */
    KVM_SEV_LAUNCH_START,
    KVM_SEV_LAUNCH_UPDATE_DATA,
    KVM_SEV_LAUNCH_UPDATE_VMSA,
    KVM_SEV_LAUNCH_SECRET,
    KVM_SEV_LAUNCH_MEASURE,
    KVM_SEV_LAUNCH_FINISH,
    /* Guest migration commands (outgoing) */
    KVM_SEV_SEND_START,
    KVM_SEV_SEND_UPDATE_DATA,
    KVM_SEV_SEND_UPDATE_VMSA,
    KVM_SEV_SEND_FINISH,
    /* Guest migration commands (incoming) */
    KVM_SEV_RECEIVE_START,
    KVM_SEV_RECEIVE_UPDATE_DATA,
    KVM_SEV_RECEIVE_UPDATE_VMSA,
    KVM_SEV_RECEIVE_FINISH,
    /* Guest status and debug commands */
    KVM_SEV_GUEST_STATUS,
    KVM_SEV_DBG_DECRYPT,
    KVM_SEV_DBG_ENCRYPT,
    /* Guest certificates commands */
    KVM_SEV_CERT_EXPORT,
    /* Attestation report */
    KVM_SEV_GET_ATTESTATION_REPORT,
    /* Guest Migration Extension */
    KVM_SEV_SEND_CANCEL,

    /* Second time is the charm; improved versions of the above ioctls.  */
    KVM_SEV_INIT2,

    /* SNP-specific commands */
    KVM_SEV_SNP_LAUNCH_START = 100,
    KVM_SEV_SNP_LAUNCH_UPDATE,
    KVM_SEV_SNP_LAUNCH_FINISH,
    KVM_SEV_SNP_ENABLE_REQ_CERTS,

    KVM_SEV_NR_MAX,
}

#[repr(C)]
pub struct kvm_sev_cmd {
    pub id: __u32,
    pub pad0: __u32,
    pub data: __u64,
    pub error: __u32,
    pub sev_fd: __u32,
}

#[repr(C)]
pub struct kvm_sev_init {
    pub vmsa_features: __u64,
    pub flags: __u32,
    pub ghcb_version: __u16,
    pub pad1: __u16,
    pub pad2: [__u32; 8],
}

#[repr(C)]
pub struct kvm_sev_launch_start {
    pub handle: __u32,
    pub policy: __u32,
    pub dh_uaddr: __u64,
    pub dh_len: __u32,
    pub pad0: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad1: __u32,
}

#[repr(C)]
pub struct kvm_sev_launch_update_data {
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
pub struct kvm_sev_launch_secret {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
pub struct kvm_sev_launch_measure {
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
pub struct kvm_sev_guest_status {
    pub handle: __u32,
    pub policy: __u32,
    pub state: __u32,
}

#[repr(C)]
pub struct kvm_sev_dbg {
    pub src_uaddr: __u64,
    pub dst_uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
pub struct kvm_sev_attestation_report {
    pub mnonce: [__u8; 16],
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
pub struct kvm_sev_send_start {
    pub policy: __u32,
    pub pad0: __u32,
    pub pdh_cert_uaddr: __u64,
    pub pdh_cert_len: __u32,
    pub pad1: __u32,
    pub plat_certs_uaddr: __u64,
    pub plat_certs_len: __u32,
    pub pad2: __u32,
    pub amd_certs_uaddr: __u64,
    pub amd_certs_len: __u32,
    pub pad3: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad4: __u32,
}

#[repr(C)]
pub struct kvm_sev_send_update_data {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
pub struct kvm_sev_receive_start {
    pub handle: __u32,
    pub policy: __u32,
    pub pdh_uaddr: __u64,
    pub pdh_len: __u32,
    pub pad0: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad1: __u32,
}

#[repr(C)]
pub struct kvm_sev_receive_update_data {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
pub struct kvm_sev_snp_launch_start {
    pub policy: __u64,
    pub gosvw: [__u8; 16],
    pub flags: __u16,
    pub pad0: [__u8; 6],
    pub pad1: [__u64; 4],
}

/* Kept in sync with firmware values for simplicity. */
pub const KVM_SEV_PAGE_TYPE_INVALID: u32 = 0x0;
pub const KVM_SEV_SNP_PAGE_TYPE_NORMAL: u32 = 0x1;
pub const KVM_SEV_SNP_PAGE_TYPE_ZERO: u32 = 0x3;
pub const KVM_SEV_SNP_PAGE_TYPE_UNMEASURED: u32 = 0x4;
pub const KVM_SEV_SNP_PAGE_TYPE_SECRETS: u32 = 0x5;
pub const KVM_SEV_SNP_PAGE_TYPE_CPUID: u32 = 0x6;

#[repr(C)]
pub struct kvm_sev_snp_launch_update {
    pub gfn_start: __u64,
    pub uaddr: __u64,
    pub len: __u64,
    pub type_: __u8,
    pub pad0: __u8,
    pub flags: __u16,
    pub pad1: __u32,
    pub pad2: [__u64; 4],
}

pub const KVM_SEV_SNP_ID_BLOCK_SIZE: usize = 96;
pub const KVM_SEV_SNP_ID_AUTH_SIZE: usize = 4096;
pub const KVM_SEV_SNP_FINISH_DATA_SIZE: usize = 32;

#[repr(C)]
pub struct kvm_sev_snp_launch_finish {
    pub id_block_uaddr: __u64,
    pub id_auth_uaddr: __u64,
    pub id_block_en: __u8,
    pub auth_key_en: __u8,
    pub vcek_disabled: __u8,
    pub host_data: [__u8; KVM_SEV_SNP_FINISH_DATA_SIZE],
    pub pad0: [__u8; 3],
    pub flags: __u16,
    pub pad1: [__u64; 4],
}

pub const KVM_X2APIC_API_USE_32BIT_IDS: u64 = _BITULL(0);
pub const KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK: u64 = _BITULL(1);
pub const KVM_X2APIC_ENABLE_SUPPRESS_EOI_BROADCAST: u64 = _BITULL(2);
pub const KVM_X2APIC_DISABLE_SUPPRESS_EOI_BROADCAST: u64 = _BITULL(3);

#[repr(C)]
pub struct kvm_hyperv_eventfd {
    pub conn_id: __u32,
    pub fd: __s32,
    pub flags: __u32,
    pub padding: [__u32; 3],
}

pub const KVM_HYPERV_CONN_ID_MASK: u32 = 0x00ffffff;
pub const KVM_HYPERV_EVENTFD_DEASSIGN: u32 = 1 << 0;

/*
 * Masked event layout.
 * Bits   Description
 * ----   -----------
 * 7:0    event select (low bits)
 * 15:8   umask match
 * 31:16  unused
 * 35:32  event select (high bits)
 * 36:54  unused
 * 55     exclude bit
 * 63:56  umask mask
 */

pub const fn KVM_PMU_ENCODE_MASKED_ENTRY(
    event_select: __u64,
    mask: __u64,
    match_: __u64,
    exclude: bool,
) -> __u64 {
    ((event_select & 0xFFu64)
        | ((event_select & 0xF00u64) << 24)
        | ((mask & 0xFFu64) << 56)
        | ((match_ & 0xFFu64) << 8)
        | ((exclude as __u64) << 55))
}

pub const KVM_PMU_MASKED_ENTRY_EVENT_SELECT: u64 = __GENMASK_ULL(7, 0) | __GENMASK_ULL(35, 32);
pub const KVM_PMU_MASKED_ENTRY_UMASK_MASK: u64 = __GENMASK_ULL(63, 56);
pub const KVM_PMU_MASKED_ENTRY_UMASK_MATCH: u64 = __GENMASK_ULL(15, 8);
pub const KVM_PMU_MASKED_ENTRY_EXCLUDE: u64 = _BITULL(55);
pub const KVM_PMU_MASKED_ENTRY_UMASK_MASK_SHIFT: u32 = 56;

/* for KVM_{GET,SET,HAS}_DEVICE_ATTR */
pub const KVM_VCPU_TSC_CTRL: u32 = 0; /* control group for the timestamp counter (TSC) */
pub const KVM_VCPU_TSC_OFFSET: u32 = 0; /* attribute for the TSC offset */

/* x86-specific KVM_EXIT_HYPERCALL flags. */
pub const KVM_EXIT_HYPERCALL_LONG_MODE: u64 = _BITULL(0);

pub const KVM_X86_DEFAULT_VM: u32 = 0;
pub const KVM_X86_SW_PROTECTED_VM: u32 = 1;
pub const KVM_X86_SEV_VM: u32 = 2;
pub const KVM_X86_SEV_ES_VM: u32 = 3;
pub const KVM_X86_SNP_VM: u32 = 4;
pub const KVM_X86_TDX_VM: u32 = 5;

/* Trust Domain eXtension sub-ioctl() commands. */
#[repr(C)]
pub enum kvm_tdx_cmd_id {
    KVM_TDX_CAPABILITIES = 0,
    KVM_TDX_INIT_VM,
    KVM_TDX_INIT_VCPU,
    KVM_TDX_INIT_MEM_REGION,
    KVM_TDX_FINALIZE_VM,
    KVM_TDX_GET_CPUID,

    KVM_TDX_CMD_NR_MAX,
}

#[repr(C)]
pub struct kvm_tdx_cmd {
    /* enum kvm_tdx_cmd_id */
    pub id: __u32,
    /* flags for sub-commend. If sub-command doesn't use this, set zero. */
    pub flags: __u32,
    /*
     * data for each sub-command. An immediate or a pointer to the actual
     * data in process virtual address.  If sub-command doesn't use it,
     * set zero.
     */
    pub data: __u64,
    /*
     * Auxiliary error code.  The sub-command may return TDX SEAMCALL
     * status code in addition to -Exxx.
     */
    pub hw_error: __u64,
}

#[repr(C)]
pub struct kvm_tdx_capabilities {
    pub supported_attrs: __u64,
    pub supported_xfam: __u64,

    pub kernel_tdvmcallinfo_1_r11: __u64,
    pub user_tdvmcallinfo_1_r11: __u64,
    pub kernel_tdvmcallinfo_1_r12: __u64,
    pub user_tdvmcallinfo_1_r12: __u64,

    pub reserved: [__u64; 250],

    /* Configurable CPUID bits for userspace */
    pub cpuid: kvm_cpuid2,
}

#[repr(C)]
pub struct kvm_tdx_init_vm {
    pub attributes: __u64,
    pub xfam: __u64,
    pub mrconfigid: [__u64; 6], /* sha384 digest */
    pub mrowner: [__u64; 6],    /* sha384 digest */
    pub mrownerconfig: [__u64; 6], /* sha384 digest */

    /* The total space for TD_PARAMS before the CPUIDs is 256 bytes */
    pub reserved: [__u64; 12],

    /*
     * Call KVM_TDX_INIT_VM before vcpu creation, thus before
     * KVM_SET_CPUID2.
     * This configuration supersedes KVM_SET_CPUID2s for VCPUs because the
     * TDX module directly virtualizes those CPUIDs without VMM.  The user
     * space VMM, e.g. qemu, should make KVM_SET_CPUID2 consistent with
     * those values.  If it doesn't, KVM may have wrong idea of vCPUIDs of
     * the guest, and KVM may wrongly emulate CPUIDs or MSRs that the TDX
     * module doesn't virtualize.
     */
    pub cpuid: kvm_cpuid2,
}

pub const KVM_TDX_MEASURE_MEMORY_REGION: u64 = _BITULL(0);

#[repr(C)]
pub struct kvm_tdx_init_mem_region {
    pub source_addr: __u64,
    pub gpa: __u64,
    pub nr_pages: __u64,
}
