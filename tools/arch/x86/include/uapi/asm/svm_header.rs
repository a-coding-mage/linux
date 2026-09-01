/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const SVM_EXIT_READ_CR0: u64 = 0x000;
pub const SVM_EXIT_READ_CR2: u64 = 0x002;
pub const SVM_EXIT_READ_CR3: u64 = 0x003;
pub const SVM_EXIT_READ_CR4: u64 = 0x004;
pub const SVM_EXIT_READ_CR8: u64 = 0x008;
pub const SVM_EXIT_WRITE_CR0: u64 = 0x010;
pub const SVM_EXIT_WRITE_CR2: u64 = 0x012;
pub const SVM_EXIT_WRITE_CR3: u64 = 0x013;
pub const SVM_EXIT_WRITE_CR4: u64 = 0x014;
pub const SVM_EXIT_WRITE_CR8: u64 = 0x018;
pub const SVM_EXIT_READ_DR0: u64 = 0x020;
pub const SVM_EXIT_READ_DR1: u64 = 0x021;
pub const SVM_EXIT_READ_DR2: u64 = 0x022;
pub const SVM_EXIT_READ_DR3: u64 = 0x023;
pub const SVM_EXIT_READ_DR4: u64 = 0x024;
pub const SVM_EXIT_READ_DR5: u64 = 0x025;
pub const SVM_EXIT_READ_DR6: u64 = 0x026;
pub const SVM_EXIT_READ_DR7: u64 = 0x027;
pub const SVM_EXIT_WRITE_DR0: u64 = 0x030;
pub const SVM_EXIT_WRITE_DR1: u64 = 0x031;
pub const SVM_EXIT_WRITE_DR2: u64 = 0x032;
pub const SVM_EXIT_WRITE_DR3: u64 = 0x033;
pub const SVM_EXIT_WRITE_DR4: u64 = 0x034;
pub const SVM_EXIT_WRITE_DR5: u64 = 0x035;
pub const SVM_EXIT_WRITE_DR6: u64 = 0x036;
pub const SVM_EXIT_WRITE_DR7: u64 = 0x037;
pub const SVM_EXIT_EXCP_BASE: u64 = 0x040;
pub const SVM_EXIT_LAST_EXCP: u64 = 0x05f;
pub const SVM_EXIT_INTR: u64 = 0x060;
pub const SVM_EXIT_NMI: u64 = 0x061;
pub const SVM_EXIT_SMI: u64 = 0x062;
pub const SVM_EXIT_INIT: u64 = 0x063;
pub const SVM_EXIT_VINTR: u64 = 0x064;
pub const SVM_EXIT_CR0_SEL_WRITE: u64 = 0x065;
pub const SVM_EXIT_IDTR_READ: u64 = 0x066;
pub const SVM_EXIT_GDTR_READ: u64 = 0x067;
pub const SVM_EXIT_LDTR_READ: u64 = 0x068;
pub const SVM_EXIT_TR_READ: u64 = 0x069;
pub const SVM_EXIT_IDTR_WRITE: u64 = 0x06a;
pub const SVM_EXIT_GDTR_WRITE: u64 = 0x06b;
pub const SVM_EXIT_LDTR_WRITE: u64 = 0x06c;
pub const SVM_EXIT_TR_WRITE: u64 = 0x06d;
pub const SVM_EXIT_RDTSC: u64 = 0x06e;
pub const SVM_EXIT_RDPMC: u64 = 0x06f;
pub const SVM_EXIT_PUSHF: u64 = 0x070;
pub const SVM_EXIT_POPF: u64 = 0x071;
pub const SVM_EXIT_CPUID: u64 = 0x072;
pub const SVM_EXIT_RSM: u64 = 0x073;
pub const SVM_EXIT_IRET: u64 = 0x074;
pub const SVM_EXIT_SWINT: u64 = 0x075;
pub const SVM_EXIT_INVD: u64 = 0x076;
pub const SVM_EXIT_PAUSE: u64 = 0x077;
pub const SVM_EXIT_HLT: u64 = 0x078;
pub const SVM_EXIT_INVLPG: u64 = 0x079;
pub const SVM_EXIT_INVLPGA: u64 = 0x07a;
pub const SVM_EXIT_IOIO: u64 = 0x07b;
pub const SVM_EXIT_MSR: u64 = 0x07c;
pub const SVM_EXIT_TASK_SWITCH: u64 = 0x07d;
pub const SVM_EXIT_FERR_FREEZE: u64 = 0x07e;
pub const SVM_EXIT_SHUTDOWN: u64 = 0x07f;
pub const SVM_EXIT_VMRUN: u64 = 0x080;
pub const SVM_EXIT_VMMCALL: u64 = 0x081;
pub const SVM_EXIT_VMLOAD: u64 = 0x082;
pub const SVM_EXIT_VMSAVE: u64 = 0x083;
pub const SVM_EXIT_STGI: u64 = 0x084;
pub const SVM_EXIT_CLGI: u64 = 0x085;
pub const SVM_EXIT_SKINIT: u64 = 0x086;
pub const SVM_EXIT_RDTSCP: u64 = 0x087;
pub const SVM_EXIT_ICEBP: u64 = 0x088;
pub const SVM_EXIT_WBINVD: u64 = 0x089;
pub const SVM_EXIT_MONITOR: u64 = 0x08a;
pub const SVM_EXIT_MWAIT: u64 = 0x08b;
pub const SVM_EXIT_MWAIT_COND: u64 = 0x08c;
pub const SVM_EXIT_XSETBV: u64 = 0x08d;
pub const SVM_EXIT_RDPRU: u64 = 0x08e;
pub const SVM_EXIT_EFER_WRITE_TRAP: u64 = 0x08f;
pub const SVM_EXIT_CR0_WRITE_TRAP: u64 = 0x090;
pub const SVM_EXIT_CR1_WRITE_TRAP: u64 = 0x091;
pub const SVM_EXIT_CR2_WRITE_TRAP: u64 = 0x092;
pub const SVM_EXIT_CR3_WRITE_TRAP: u64 = 0x093;
pub const SVM_EXIT_CR4_WRITE_TRAP: u64 = 0x094;
pub const SVM_EXIT_CR5_WRITE_TRAP: u64 = 0x095;
pub const SVM_EXIT_CR6_WRITE_TRAP: u64 = 0x096;
pub const SVM_EXIT_CR7_WRITE_TRAP: u64 = 0x097;
pub const SVM_EXIT_CR8_WRITE_TRAP: u64 = 0x098;
pub const SVM_EXIT_CR9_WRITE_TRAP: u64 = 0x099;
pub const SVM_EXIT_CR10_WRITE_TRAP: u64 = 0x09a;
pub const SVM_EXIT_CR11_WRITE_TRAP: u64 = 0x09b;
pub const SVM_EXIT_CR12_WRITE_TRAP: u64 = 0x09c;
pub const SVM_EXIT_CR13_WRITE_TRAP: u64 = 0x09d;
pub const SVM_EXIT_CR14_WRITE_TRAP: u64 = 0x09e;
pub const SVM_EXIT_CR15_WRITE_TRAP: u64 = 0x09f;
pub const SVM_EXIT_INVPCID: u64 = 0x0a2;
pub const SVM_EXIT_BUS_LOCK: u64 = 0x0a5;
pub const SVM_EXIT_IDLE_HLT: u64 = 0x0a6;
pub const SVM_EXIT_NPF: u64 = 0x400;
pub const SVM_EXIT_AVIC_INCOMPLETE_IPI: u64 = 0x401;
pub const SVM_EXIT_AVIC_UNACCELERATED_ACCESS: u64 = 0x402;
pub const SVM_EXIT_VMGEXIT: u64 = 0x403;

/* SEV-ES software-defined VMGEXIT events */
pub const SVM_VMGEXIT_MMIO_READ: u64 = 0x80000001;
pub const SVM_VMGEXIT_MMIO_WRITE: u64 = 0x80000002;
pub const SVM_VMGEXIT_NMI_COMPLETE: u64 = 0x80000003;
pub const SVM_VMGEXIT_AP_HLT_LOOP: u64 = 0x80000004;
pub const SVM_VMGEXIT_AP_JUMP_TABLE: u64 = 0x80000005;
pub const SVM_VMGEXIT_SET_AP_JUMP_TABLE: u64 = 0;
pub const SVM_VMGEXIT_GET_AP_JUMP_TABLE: u64 = 1;
pub const SVM_VMGEXIT_PSC: u64 = 0x80000010;
pub const SVM_VMGEXIT_GUEST_REQUEST: u64 = 0x80000011;
pub const SVM_VMGEXIT_EXT_GUEST_REQUEST: u64 = 0x80000012;
pub const SVM_VMGEXIT_AP_CREATION: u64 = 0x80000013;
pub const SVM_VMGEXIT_AP_CREATE_ON_INIT: u64 = 0;
pub const SVM_VMGEXIT_AP_CREATE: u64 = 1;
pub const SVM_VMGEXIT_AP_DESTROY: u64 = 2;
pub const SVM_VMGEXIT_SNP_RUN_VMPL: u64 = 0x80000018;
pub const SVM_VMGEXIT_SAVIC: u64 = 0x8000001a;
pub const SVM_VMGEXIT_SAVIC_REGISTER_GPA: u64 = 0;
pub const SVM_VMGEXIT_SAVIC_UNREGISTER_GPA: u64 = 1;
pub const SVM_VMGEXIT_SAVIC_SELF_GPA: u64 = !0u64;
pub const SVM_VMGEXIT_HV_FEATURES: u64 = 0x8000fffd;
pub const SVM_VMGEXIT_TERM_REQUEST: u64 = 0x8000fffe;

pub const fn SVM_VMGEXIT_TERM_REASON(reason_set: u64, reason_code: u64) -> u64 {
    /* SW_EXITINFO1[3:0] */
    ((reason_set & 0xf) |
    /* SW_EXITINFO1[11:4] */
    ((reason_code & 0xff) << 4))
}

pub const SVM_VMGEXIT_UNSUPPORTED_EVENT: u64 = 0x8000ffff;

/* Exit code reserved for hypervisor/software use */
pub const SVM_EXIT_SW: u64 = 0xf0000000;

pub const SVM_EXIT_ERR: u64 = (-1i64) as u64;

/*
 * Requires exception vector constants supplied by other translated headers:
 * DE_VECTOR, DB_VECTOR, BP_VECTOR, OF_VECTOR, BR_VECTOR, UD_VECTOR, NM_VECTOR,
 * DF_VECTOR, TS_VECTOR, NP_VECTOR, SS_VECTOR, GP_VECTOR, PF_VECTOR, MF_VECTOR,
 * AC_VECTOR, MC_VECTOR, and XM_VECTOR.
 */
#[macro_export]
macro_rules! SVM_EXIT_REASONS {
    () => {
        [
            (SVM_EXIT_READ_CR0, "read_cr0"),
            (SVM_EXIT_READ_CR2, "read_cr2"),
            (SVM_EXIT_READ_CR3, "read_cr3"),
            (SVM_EXIT_READ_CR4, "read_cr4"),
            (SVM_EXIT_READ_CR8, "read_cr8"),
            (SVM_EXIT_WRITE_CR0, "write_cr0"),
            (SVM_EXIT_WRITE_CR2, "write_cr2"),
            (SVM_EXIT_WRITE_CR3, "write_cr3"),
            (SVM_EXIT_WRITE_CR4, "write_cr4"),
            (SVM_EXIT_WRITE_CR8, "write_cr8"),
            (SVM_EXIT_READ_DR0, "read_dr0"),
            (SVM_EXIT_READ_DR1, "read_dr1"),
            (SVM_EXIT_READ_DR2, "read_dr2"),
            (SVM_EXIT_READ_DR3, "read_dr3"),
            (SVM_EXIT_READ_DR4, "read_dr4"),
            (SVM_EXIT_READ_DR5, "read_dr5"),
            (SVM_EXIT_READ_DR6, "read_dr6"),
            (SVM_EXIT_READ_DR7, "read_dr7"),
            (SVM_EXIT_WRITE_DR0, "write_dr0"),
            (SVM_EXIT_WRITE_DR1, "write_dr1"),
            (SVM_EXIT_WRITE_DR2, "write_dr2"),
            (SVM_EXIT_WRITE_DR3, "write_dr3"),
            (SVM_EXIT_WRITE_DR4, "write_dr4"),
            (SVM_EXIT_WRITE_DR5, "write_dr5"),
            (SVM_EXIT_WRITE_DR6, "write_dr6"),
            (SVM_EXIT_WRITE_DR7, "write_dr7"),
            (SVM_EXIT_EXCP_BASE + DE_VECTOR, "DE excp"),
            (SVM_EXIT_EXCP_BASE + DB_VECTOR, "DB excp"),
            (SVM_EXIT_EXCP_BASE + BP_VECTOR, "BP excp"),
            (SVM_EXIT_EXCP_BASE + OF_VECTOR, "OF excp"),
            (SVM_EXIT_EXCP_BASE + BR_VECTOR, "BR excp"),
            (SVM_EXIT_EXCP_BASE + UD_VECTOR, "UD excp"),
            (SVM_EXIT_EXCP_BASE + NM_VECTOR, "NM excp"),
            (SVM_EXIT_EXCP_BASE + DF_VECTOR, "DF excp"),
            (SVM_EXIT_EXCP_BASE + TS_VECTOR, "TS excp"),
            (SVM_EXIT_EXCP_BASE + NP_VECTOR, "NP excp"),
            (SVM_EXIT_EXCP_BASE + SS_VECTOR, "SS excp"),
            (SVM_EXIT_EXCP_BASE + GP_VECTOR, "GP excp"),
            (SVM_EXIT_EXCP_BASE + PF_VECTOR, "PF excp"),
            (SVM_EXIT_EXCP_BASE + MF_VECTOR, "MF excp"),
            (SVM_EXIT_EXCP_BASE + AC_VECTOR, "AC excp"),
            (SVM_EXIT_EXCP_BASE + MC_VECTOR, "MC excp"),
            (SVM_EXIT_EXCP_BASE + XM_VECTOR, "XF excp"),
            (SVM_EXIT_INTR, "interrupt"),
            (SVM_EXIT_NMI, "nmi"),
            (SVM_EXIT_SMI, "smi"),
            (SVM_EXIT_INIT, "init"),
            (SVM_EXIT_VINTR, "vintr"),
            (SVM_EXIT_CR0_SEL_WRITE, "cr0_sel_write"),
            (SVM_EXIT_IDTR_READ, "read_idtr"),
            (SVM_EXIT_GDTR_READ, "read_gdtr"),
            (SVM_EXIT_LDTR_READ, "read_ldtr"),
            (SVM_EXIT_TR_READ, "read_rt"),
            (SVM_EXIT_IDTR_WRITE, "write_idtr"),
            (SVM_EXIT_GDTR_WRITE, "write_gdtr"),
            (SVM_EXIT_LDTR_WRITE, "write_ldtr"),
            (SVM_EXIT_TR_WRITE, "write_rt"),
            (SVM_EXIT_RDTSC, "rdtsc"),
            (SVM_EXIT_RDPMC, "rdpmc"),
            (SVM_EXIT_PUSHF, "pushf"),
            (SVM_EXIT_POPF, "popf"),
            (SVM_EXIT_CPUID, "cpuid"),
            (SVM_EXIT_RSM, "rsm"),
            (SVM_EXIT_IRET, "iret"),
            (SVM_EXIT_SWINT, "swint"),
            (SVM_EXIT_INVD, "invd"),
            (SVM_EXIT_PAUSE, "pause"),
            (SVM_EXIT_HLT, "hlt"),
            (SVM_EXIT_INVLPG, "invlpg"),
            (SVM_EXIT_INVLPGA, "invlpga"),
            (SVM_EXIT_IOIO, "io"),
            (SVM_EXIT_MSR, "msr"),
            (SVM_EXIT_TASK_SWITCH, "task_switch"),
            (SVM_EXIT_FERR_FREEZE, "ferr_freeze"),
            (SVM_EXIT_SHUTDOWN, "shutdown"),
            (SVM_EXIT_VMRUN, "vmrun"),
            (SVM_EXIT_VMMCALL, "hypercall"),
            (SVM_EXIT_VMLOAD, "vmload"),
            (SVM_EXIT_VMSAVE, "vmsave"),
            (SVM_EXIT_STGI, "stgi"),
            (SVM_EXIT_CLGI, "clgi"),
            (SVM_EXIT_SKINIT, "skinit"),
            (SVM_EXIT_RDTSCP, "rdtscp"),
            (SVM_EXIT_ICEBP, "icebp"),
            (SVM_EXIT_WBINVD, "wbinvd"),
            (SVM_EXIT_MONITOR, "monitor"),
            (SVM_EXIT_MWAIT, "mwait"),
            (SVM_EXIT_XSETBV, "xsetbv"),
            (SVM_EXIT_EFER_WRITE_TRAP, "write_efer_trap"),
            (SVM_EXIT_CR0_WRITE_TRAP, "write_cr0_trap"),
            (SVM_EXIT_CR4_WRITE_TRAP, "write_cr4_trap"),
            (SVM_EXIT_CR8_WRITE_TRAP, "write_cr8_trap"),
            (SVM_EXIT_INVPCID, "invpcid"),
            (SVM_EXIT_BUS_LOCK, "buslock"),
            (SVM_EXIT_IDLE_HLT, "idle-halt"),
            (SVM_EXIT_NPF, "npf"),
            (SVM_EXIT_AVIC_INCOMPLETE_IPI, "avic_incomplete_ipi"),
            (SVM_EXIT_AVIC_UNACCELERATED_ACCESS, "avic_unaccelerated_access"),
            (SVM_EXIT_VMGEXIT, "vmgexit"),
            (SVM_VMGEXIT_MMIO_READ, "vmgexit_mmio_read"),
            (SVM_VMGEXIT_MMIO_WRITE, "vmgexit_mmio_write"),
            (SVM_VMGEXIT_NMI_COMPLETE, "vmgexit_nmi_complete"),
            (SVM_VMGEXIT_AP_HLT_LOOP, "vmgexit_ap_hlt_loop"),
            (SVM_VMGEXIT_AP_JUMP_TABLE, "vmgexit_ap_jump_table"),
            (SVM_VMGEXIT_PSC, "vmgexit_page_state_change"),
            (SVM_VMGEXIT_GUEST_REQUEST, "vmgexit_guest_request"),
            (SVM_VMGEXIT_EXT_GUEST_REQUEST, "vmgexit_ext_guest_request"),
            (SVM_VMGEXIT_AP_CREATION, "vmgexit_ap_creation"),
            (SVM_VMGEXIT_HV_FEATURES, "vmgexit_hypervisor_feature"),
            (SVM_EXIT_ERR, "invalid_guest_state"),
        ]
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
