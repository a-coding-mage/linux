/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from the PowerPC ptrace UAPI header. */

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct user_pt_regs {
    pub gpr: [usize; 32],
    pub nip: usize,
    pub msr: usize,
    pub orig_gpr3: usize, // Used for restarting system calls
    pub ctr: usize,
    pub link: usize,
    pub xer: usize,
    pub ccr: usize,
    #[cfg(target_arch = "powerpc64")]
    pub softe: usize, // Soft enabled/disabled
    #[cfg(not(target_arch = "powerpc64"))]
    pub mq: usize, // 601 only (not used at present); used on APUS to hold IPL value
    pub trap: usize, // Reason for being here
    // For critical exceptions on 4xx, dar and dsisr hold srr0 and srr1.
    pub dar: usize, // Fault registers
    pub dsisr: usize, // On 4xx/Book-E used for ESR
    pub result: usize, // Result of a system call
}

#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct pt_regs {
    pub gpr: [usize; 32],
    pub nip: usize,
    pub msr: usize,
    pub orig_gpr3: usize, // Used for restarting system calls
    pub ctr: usize,
    pub link: usize,
    pub xer: usize,
    pub ccr: usize,
    #[cfg(target_arch = "powerpc64")]
    pub softe: usize, // Soft enabled/disabled
    #[cfg(not(target_arch = "powerpc64"))]
    pub mq: usize, // 601 only (not used at present); used on APUS to hold IPL value
    pub trap: usize, // Reason for being here
    // For critical exceptions on 4xx, dar and dsisr hold srr0 and srr1.
    pub dar: usize, // Fault registers
    pub dsisr: usize, // On 4xx/Book-E used for ESR
    pub result: usize, // Result of a system call
}

pub const PT_R0: usize = 0;
pub const PT_R1: usize = 1;
pub const PT_R2: usize = 2;
pub const PT_R3: usize = 3;
pub const PT_R4: usize = 4;
pub const PT_R5: usize = 5;
pub const PT_R6: usize = 6;
pub const PT_R7: usize = 7;
pub const PT_R8: usize = 8;
pub const PT_R9: usize = 9;
pub const PT_R10: usize = 10;
pub const PT_R11: usize = 11;
pub const PT_R12: usize = 12;
pub const PT_R13: usize = 13;
pub const PT_R14: usize = 14;
pub const PT_R15: usize = 15;
pub const PT_R16: usize = 16;
pub const PT_R17: usize = 17;
pub const PT_R18: usize = 18;
pub const PT_R19: usize = 19;
pub const PT_R20: usize = 20;
pub const PT_R21: usize = 21;
pub const PT_R22: usize = 22;
pub const PT_R23: usize = 23;
pub const PT_R24: usize = 24;
pub const PT_R25: usize = 25;
pub const PT_R26: usize = 26;
pub const PT_R27: usize = 27;
pub const PT_R28: usize = 28;
pub const PT_R29: usize = 29;
pub const PT_R30: usize = 30;
pub const PT_R31: usize = 31;
pub const PT_NIP: usize = 32;
pub const PT_MSR: usize = 33;
pub const PT_ORIG_R3: usize = 34;
pub const PT_CTR: usize = 35;
pub const PT_LNK: usize = 36;
pub const PT_XER: usize = 37;
pub const PT_CCR: usize = 38;
#[cfg(not(target_arch = "powerpc64"))]
pub const PT_MQ: usize = 39;
#[cfg(target_arch = "powerpc64")]
pub const PT_SOFTE: usize = 39;
pub const PT_TRAP: usize = 40;
pub const PT_DAR: usize = 41;
pub const PT_DSISR: usize = 42;
pub const PT_RESULT: usize = 43;
pub const PT_DSCR: usize = 44;
pub const PT_REGS_COUNT: usize = 44;
pub const PT_FPR0: usize = 48;
#[cfg(not(target_arch = "powerpc64"))]
pub const PT_FPR31: usize = PT_FPR0 + 2 * 31;
#[cfg(not(target_arch = "powerpc64"))]
pub const PT_FPSCR: usize = PT_FPR0 + 2 * 32 + 1;
#[cfg(target_arch = "powerpc64")]
pub const PT_FPSCR: usize = PT_FPR0 + 32;
#[cfg(target_arch = "powerpc64")]
pub const PT_VR0: usize = 82;
#[cfg(target_arch = "powerpc64")]
pub const PT_VSCR: usize = PT_VR0 + 32 * 2 + 1;
#[cfg(target_arch = "powerpc64")]
pub const PT_VRSAVE: usize = PT_VR0 + 33 * 2;
#[cfg(target_arch = "powerpc64")]
pub const PT_VSR0: usize = 150;
#[cfg(target_arch = "powerpc64")]
pub const PT_VSR31: usize = PT_VSR0 + 2 * 31;

pub const PTRACE_GETVRREGS: usize = 0x12;
pub const PTRACE_SETVRREGS: usize = 0x13;
pub const PTRACE_GETEVRREGS: usize = 0x14;
pub const PTRACE_SETEVRREGS: usize = 0x15;
pub const PTRACE_GETVSRREGS: usize = 0x1b;
pub const PTRACE_SETVSRREGS: usize = 0x1c;
pub const PTRACE_SYSEMU: usize = 0x1d;
pub const PTRACE_SYSEMU_SINGLESTEP: usize = 0x1e;
pub const PTRACE_GET_DEBUGREG: usize = 0x19;
pub const PTRACE_SET_DEBUGREG: usize = 0x1a;
pub const PTRACE_GETREGS: usize = 0xc;
pub const PTRACE_SETREGS: usize = 0xd;
pub const PTRACE_GETFPREGS: usize = 0xe;
pub const PTRACE_SETFPREGS: usize = 0xf;
pub const PTRACE_GETREGS64: usize = 0x16;
pub const PTRACE_SETREGS64: usize = 0x17;
pub const PPC_PTRACE_PEEKTEXT_3264: usize = 0x95;
pub const PPC_PTRACE_PEEKDATA_3264: usize = 0x94;
pub const PPC_PTRACE_POKETEXT_3264: usize = 0x93;
pub const PPC_PTRACE_POKEDATA_3264: usize = 0x92;
pub const PPC_PTRACE_PEEKUSR_3264: usize = 0x91;
pub const PPC_PTRACE_POKEUSR_3264: usize = 0x90;
pub const PTRACE_SINGLEBLOCK: usize = 0x100;
pub const PPC_PTRACE_GETHWDBGINFO: usize = 0x89;
pub const PPC_PTRACE_SETHWDEBUG: usize = 0x88;
pub const PPC_PTRACE_DELHWDEBUG: usize = 0x87;

#[repr(C)]
pub struct ppc_debug_info {
    pub version: u32,
    pub num_instruction_bps: u32,
    pub num_data_bps: u32,
    pub num_condition_regs: u32,
    pub data_bp_alignment: u32,
    pub sizeof_condition: u32,
    pub features: u64,
}

pub const PPC_DEBUG_FEATURE_INSN_BP_RANGE: u64 = 0x0000000000000001;
pub const PPC_DEBUG_FEATURE_INSN_BP_MASK: u64 = 0x0000000000000002;
pub const PPC_DEBUG_FEATURE_DATA_BP_RANGE: u64 = 0x0000000000000004;
pub const PPC_DEBUG_FEATURE_DATA_BP_MASK: u64 = 0x0000000000000008;
pub const PPC_DEBUG_FEATURE_DATA_BP_DAWR: u64 = 0x0000000000000010;
pub const PPC_DEBUG_FEATURE_DATA_BP_ARCH_31: u64 = 0x0000000000000020;

#[repr(C)]
pub struct ppc_hw_breakpoint {
    pub version: u32,
    pub trigger_type: u32,
    pub addr_mode: u32,
    pub condition_mode: u32,
    pub addr: u64,
    pub addr2: u64,
    pub condition_value: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
