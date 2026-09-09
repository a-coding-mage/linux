/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Rust translation of mips/kvm/trace.h.
 */

// TRACE_SYSTEM = kvm; TRACE_INCLUDE_PATH = .; TRACE_INCLUDE_FILE = trace

/// Opaque external type supplied by the KVM implementation.
#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

extern "C" {
    pub static mut kvm_trace_guest_mode_change: bool;
    pub fn kvm_guest_mode_change_trace_reg();
    pub fn kvm_guest_mode_change_trace_unreg();
}

// DECLARE_EVENT_CLASS(kvm_transition):
// TP_PROTO(struct kvm_vcpu *vcpu), TP_ARGS(vcpu)
// entry: unsigned long pc = vcpu->arch.pc
// print: "PC: 0x%08lx"
// DEFINE_EVENT(kvm_transition, kvm_enter)
// DEFINE_EVENT(kvm_transition, kvm_reenter)
// DEFINE_EVENT(kvm_transition, kvm_out)

pub const KVM_TRACE_EXIT_INT: u32 = 0;
pub const KVM_TRACE_EXIT_TLBMOD: u32 = 1;
pub const KVM_TRACE_EXIT_TLBMISS_LD: u32 = 2;
pub const KVM_TRACE_EXIT_TLBMISS_ST: u32 = 3;
pub const KVM_TRACE_EXIT_ADDRERR_LD: u32 = 4;
pub const KVM_TRACE_EXIT_ADDRERR_ST: u32 = 5;
pub const KVM_TRACE_EXIT_SYSCALL: u32 = 8;
pub const KVM_TRACE_EXIT_BREAK_INST: u32 = 9;
pub const KVM_TRACE_EXIT_RESVD_INST: u32 = 10;
pub const KVM_TRACE_EXIT_COP_UNUSABLE: u32 = 11;
pub const KVM_TRACE_EXIT_TRAP_INST: u32 = 13;
pub const KVM_TRACE_EXIT_MSA_FPE: u32 = 14;
pub const KVM_TRACE_EXIT_FPE: u32 = 15;
pub const KVM_TRACE_EXIT_MSA_DISABLED: u32 = 21;
pub const KVM_TRACE_EXIT_GUEST_EXIT: u32 = 27;
pub const KVM_TRACE_EXIT_WAIT: u32 = 32;
pub const KVM_TRACE_EXIT_CACHE: u32 = 33;
pub const KVM_TRACE_EXIT_SIGNAL: u32 = 34;
pub const KVM_TRACE_EXIT_GEXCCODE_BASE: u32 = 64;
pub const KVM_TRACE_EXIT_GPSI: u32 = 64;
pub const KVM_TRACE_EXIT_GSFC: u32 = 65;
pub const KVM_TRACE_EXIT_HC: u32 = 66;
pub const KVM_TRACE_EXIT_GRR: u32 = 67;
pub const KVM_TRACE_EXIT_GVA: u32 = 72;
pub const KVM_TRACE_EXIT_GHFC: u32 = 73;
pub const KVM_TRACE_EXIT_GPA: u32 = 74;

pub const KVM_TRACE_MFC0: u32 = 0;
pub const KVM_TRACE_MTC0: u32 = 1;
pub const KVM_TRACE_DMFC0: u32 = 2;
pub const KVM_TRACE_DMTC0: u32 = 3;
pub const KVM_TRACE_RDHWR: u32 = 4;
pub const KVM_TRACE_HWR_COP0: u32 = 0;
pub const KVM_TRACE_HWR_HWR: u32 = 1;

#[inline]
pub const fn KVM_TRACE_COP0(reg: u32, sel: u32) -> u32 {
    (KVM_TRACE_HWR_COP0 << 8) | (reg << 3) | sel
}

#[inline]
pub const fn KVM_TRACE_HWR(reg: u32, sel: u32) -> u32 {
    (KVM_TRACE_HWR_HWR << 8) | (reg << 3) | sel
}

pub const KVM_TRACE_AUX_RESTORE: u32 = 0;
pub const KVM_TRACE_AUX_SAVE: u32 = 1;
pub const KVM_TRACE_AUX_ENABLE: u32 = 2;
pub const KVM_TRACE_AUX_DISABLE: u32 = 3;
pub const KVM_TRACE_AUX_DISCARD: u32 = 4;
pub const KVM_TRACE_AUX_FPU: u32 = 1;
pub const KVM_TRACE_AUX_MSA: u32 = 2;
pub const KVM_TRACE_AUX_FPU_MSA: u32 = 3;

pub const KVM_TRACE_EXIT_SYMBOLS: &[(u32, &str)] = &[
    (0, "Interrupt"), (1, "TLB Mod"), (2, "TLB Miss (LD)"),
    (3, "TLB Miss (ST)"), (4, "Address Error (LD)"), (5, "Address Err (ST)"),
    (8, "System Call"), (9, "Break Inst"), (10, "Reserved Inst"),
    (11, "COP0/1 Unusable"), (13, "Trap Inst"), (14, "MSA FPE"),
    (15, "FPE"), (21, "MSA Disabled"), (27, "Guest Exit"), (32, "WAIT"),
    (33, "CACHE"), (34, "Signal"), (64, "GPSI"), (65, "GSFC"),
    (66, "HC"), (67, "GRR"), (72, "GVA"), (73, "GHFC"), (74, "GPA"),
];

pub const KVM_TRACE_HWR_OP_SYMBOLS: &[(u32, &str)] = &[
    (0, "MFC0"), (1, "MTC0"), (2, "DMFC0"), (3, "DMTC0"), (4, "RDHWR"),
];
pub const KVM_TRACE_HWR_COP_SYMBOLS: &[(u32, &str)] = &[(0, "COP0"), (1, "HWR")];

// TRACE_EVENT(kvm_exit), TRACE_EVENT(kvm_hwr), TRACE_EVENT(kvm_aux),
// TRACE_EVENT(kvm_asid_change), and TRACE_EVENT(kvm_guestid_change) retain
// their C tracepoint schemas, TP_fast_assign bodies, and TP_printk formats.
// TRACE_EVENT_FN(kvm_guest_mode_change) retains its registration callbacks
// kvm_guest_mode_change_trace_reg and kvm_guest_mode_change_trace_unreg.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
