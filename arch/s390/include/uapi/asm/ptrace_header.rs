/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* S390 version; translated from the Linux UAPI header. */

pub const PT_PSWMASK: usize = 0x00;
pub const PT_PSWADDR: usize = 0x08;
pub const PT_GPR0: usize = 0x10;
pub const PT_GPR1: usize = 0x18;
pub const PT_GPR2: usize = 0x20;
pub const PT_GPR3: usize = 0x28;
pub const PT_GPR4: usize = 0x30;
pub const PT_GPR5: usize = 0x38;
pub const PT_GPR6: usize = 0x40;
pub const PT_GPR7: usize = 0x48;
pub const PT_GPR8: usize = 0x50;
pub const PT_GPR9: usize = 0x58;
pub const PT_GPR10: usize = 0x60;
pub const PT_GPR11: usize = 0x68;
pub const PT_GPR12: usize = 0x70;
pub const PT_GPR13: usize = 0x78;
pub const PT_GPR14: usize = 0x80;
pub const PT_GPR15: usize = 0x88;
pub const PT_ACR0: usize = 0x90;
pub const PT_ACR1: usize = 0x94;
pub const PT_ACR2: usize = 0x98;
pub const PT_ACR3: usize = 0x9c;
pub const PT_ACR4: usize = 0xa0;
pub const PT_ACR5: usize = 0xa4;
pub const PT_ACR6: usize = 0xa8;
pub const PT_ACR7: usize = 0xac;
pub const PT_ACR8: usize = 0xb0;
pub const PT_ACR9: usize = 0xb4;
pub const PT_ACR10: usize = 0xb8;
pub const PT_ACR11: usize = 0xbc;
pub const PT_ACR12: usize = 0xc0;
pub const PT_ACR13: usize = 0xc4;
pub const PT_ACR14: usize = 0xc8;
pub const PT_ACR15: usize = 0xcc;
pub const PT_ORIGGPR2: usize = 0xd0;
pub const PT_FPC: usize = 0xd8;
pub const PT_FPR0: usize = 0xe0;
pub const PT_FPR1: usize = 0xe8;
pub const PT_FPR2: usize = 0xf0;
pub const PT_FPR3: usize = 0xf8;
pub const PT_FPR4: usize = 0x100;
pub const PT_FPR5: usize = 0x108;
pub const PT_FPR6: usize = 0x110;
pub const PT_FPR7: usize = 0x118;
pub const PT_FPR8: usize = 0x120;
pub const PT_FPR9: usize = 0x128;
pub const PT_FPR10: usize = 0x130;
pub const PT_FPR11: usize = 0x138;
pub const PT_FPR12: usize = 0x140;
pub const PT_FPR13: usize = 0x148;
pub const PT_FPR14: usize = 0x150;
pub const PT_FPR15: usize = 0x158;
pub const PT_CR_9: usize = 0x160;
pub const PT_CR_10: usize = 0x168;
pub const PT_CR_11: usize = 0x170;
pub const PT_IEEE_IP: usize = 0x1a8;
pub const PT_LASTOFF: usize = PT_IEEE_IP;
pub const PT_ENDREGS: usize = 0x1b0 - 1;

pub const GPR_SIZE: usize = 8;
pub const CR_SIZE: usize = 8;
pub const STACK_FRAME_OVERHEAD: usize = 160;

pub const PSW_MASK_PER: u64 = 0x4000000000000000;
pub const PSW_MASK_DAT: u64 = 0x0400000000000000;
pub const PSW_MASK_IO: u64 = 0x0200000000000000;
pub const PSW_MASK_EXT: u64 = 0x0100000000000000;
pub const PSW_MASK_BASE: u64 = 0;
pub const PSW_MASK_KEY: u64 = 0x00f0000000000000;
pub const PSW_MASK_MCHECK: u64 = 0x0004000000000000;
pub const PSW_MASK_WAIT: u64 = 0x0002000000000000;
pub const PSW_MASK_PSTATE: u64 = 0x0001000000000000;
pub const PSW_MASK_ASC: u64 = 0x0000c00000000000;
pub const PSW_MASK_CC: u64 = 0x0000300000000000;
pub const PSW_MASK_PM: u64 = 0x00000f0000000000;
pub const PSW_MASK_RI: u64 = 0x0000008000000000;
pub const PSW_MASK_EA: u64 = 0x0000000100000000;
pub const PSW_MASK_BA: u64 = 0x0000000080000000;
pub const PSW_MASK_USER: u64 = 0x0000ff0180000000;
pub const PSW_ADDR_AMODE: u64 = 0;
pub const PSW_ADDR_INSN: u64 = 0xffffffffffffffff;
pub const PSW_ASC_PRIMARY: u64 = 0;
pub const PSW_ASC_ACCREG: u64 = 0x0000400000000000;
pub const PSW_ASC_SECONDARY: u64 = 0x0000800000000000;
pub const PSW_ASC_HOME: u64 = 0x0000c00000000000;

pub const NUM_GPRS: usize = 16;
pub const NUM_FPRS: usize = 16;
pub const NUM_CRS: usize = 16;
pub const NUM_ACRS: usize = 16;
pub const NUM_CR_WORDS: usize = 3;
pub const FPR_SIZE: usize = 8;
pub const FPC_SIZE: usize = 4;
pub const FPC_PAD_SIZE: usize = 4;
pub const ACR_SIZE: usize = 4;
pub const PTRACE_OLDSETOPTIONS: u32 = 21;
pub const PTRACE_SYSEMU: u32 = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: u32 = 32;

#[repr(C)]
pub union freg_t { pub f: f32, pub d: f64, pub ui: u64, pub fp: freg_fp }
#[repr(C)]
pub struct freg_fp { pub hi: u32, pub lo: u32 }
#[repr(C)]
pub struct s390_fp_regs { pub fpc: u32, pub pad: u32, pub fprs: [freg_t; NUM_FPRS] }
pub const FPC_EXCEPTION_MASK: u32 = 0xf8000000;
pub const FPC_FLAGS_MASK: u32 = 0x00f80000;
pub const FPC_DXC_MASK: u32 = 0x0000ff00;
pub const FPC_RM_MASK: u32 = 0x00000003;

#[repr(C, align(8))]
pub struct psw_t { pub mask: usize, pub addr: usize }
#[repr(C)]
pub struct s390_regs { pub psw: psw_t, pub gprs: [usize; NUM_GPRS], pub acrs: [u32; NUM_ACRS], pub orig_gpr2: usize }
#[repr(C)]
pub struct user_pt_regs { pub args: [usize; 1], pub psw: psw_t, pub gprs: [usize; NUM_GPRS] }
#[repr(C)]
pub struct per_cr_words { pub cr: [usize; NUM_CR_WORDS] }
pub const PER_EM_MASK: u32 = 0xe8000000;

/* C bit-fields are represented as their containing 32-bit words. */
#[repr(C)]
pub struct per_cr_bits { pub bits0: u32, pub starting_addr: usize, pub ending_addr: usize }
#[repr(C)]
pub struct per_lowcore_words { pub perc_atmid: u16, pub address: usize, pub access_id: u8 }
#[repr(C)]
pub struct per_lowcore_bits { pub bits0: u32, pub address: usize, pub bits1: u32 }
#[repr(C)]
pub union per_control_regs { pub words: per_cr_words, pub bits: per_cr_bits }
#[repr(C)]
pub union per_lowcore { pub words: per_lowcore_words, pub bits: per_lowcore_bits }
#[repr(C)]
pub struct per_struct { pub control_regs: per_control_regs, pub single_step_instruction_fetch: u32, pub starting_addr: usize, pub ending_addr: usize, pub lowcore: per_lowcore }
#[repr(C)]
pub struct ptrace_area { pub len: u32, pub kernel_addr: usize, pub process_addr: usize }

pub const PTRACE_PEEKUSR_AREA: u32 = 0x5000;
pub const PTRACE_POKEUSR_AREA: u32 = 0x5001;
pub const PTRACE_PEEKTEXT_AREA: u32 = 0x5002;
pub const PTRACE_PEEKDATA_AREA: u32 = 0x5003;
pub const PTRACE_POKETEXT_AREA: u32 = 0x5004;
pub const PTRACE_POKEDATA_AREA: u32 = 0x5005;
pub const PTRACE_GET_LAST_BREAK: u32 = 0x5006;
pub const PTRACE_PEEK_SYSTEM_CALL: u32 = 0x5007;
pub const PTRACE_POKE_SYSTEM_CALL: u32 = 0x5008;
pub const PTRACE_ENABLE_TE: u32 = 0x5009;
pub const PTRACE_DISABLE_TE: u32 = 0x5010;
pub const PTRACE_TE_ABORT_RAND: u32 = 0x5011;
pub const PTRACE_SINGLEBLOCK: u32 = 12;
pub const PTRACE_PROT: u32 = 21;
#[repr(u32)]
pub enum ptprot_flags { ptprot_set_access_watchpoint, ptprot_set_write_watchpoint, ptprot_disable_watchpoint }
#[repr(C)]
pub struct ptprot_area { pub lowaddr: usize, pub hiaddr: usize, pub prot: ptprot_flags }
pub const S390_BREAKPOINT: [u8; 2] = [0x0, 0x1];
pub const S390_BREAKPOINT_U16: u16 = 0x0001;
pub const S390_SYSCALL_OPCODE: u16 = 0x0a00;
pub const S390_SYSCALL_SIZE: usize = 2;

#[repr(C)]
pub struct user_regs_struct { pub psw: psw_t, pub gprs: [usize; NUM_GPRS], pub acrs: [u32; NUM_ACRS], pub orig_gpr2: usize, pub fp_regs: s390_fp_regs, pub per_info: per_struct, pub ieee_instruction_pointer: usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
