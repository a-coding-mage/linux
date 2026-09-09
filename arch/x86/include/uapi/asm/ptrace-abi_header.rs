/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* C header condition: __i386__ */
#[cfg(target_arch = "x86")]
pub const EBX: usize = 0;
#[cfg(target_arch = "x86")]
pub const ECX: usize = 1;
#[cfg(target_arch = "x86")]
pub const EDX: usize = 2;
#[cfg(target_arch = "x86")]
pub const ESI: usize = 3;
#[cfg(target_arch = "x86")]
pub const EDI: usize = 4;
#[cfg(target_arch = "x86")]
pub const EBP: usize = 5;
#[cfg(target_arch = "x86")]
pub const EAX: usize = 6;
#[cfg(target_arch = "x86")]
pub const DS: usize = 7;
#[cfg(target_arch = "x86")]
pub const ES: usize = 8;
#[cfg(target_arch = "x86")]
pub const FS: usize = 9;
#[cfg(target_arch = "x86")]
pub const GS: usize = 10;
#[cfg(target_arch = "x86")]
pub const ORIG_EAX: usize = 11;
#[cfg(target_arch = "x86")]
pub const EIP: usize = 12;
#[cfg(target_arch = "x86")]
pub const CS: usize = 13;
#[cfg(target_arch = "x86")]
pub const EFL: usize = 14;
#[cfg(target_arch = "x86")]
pub const UESP: usize = 15;
#[cfg(target_arch = "x86")]
pub const SS: usize = 16;
#[cfg(target_arch = "x86")]
pub const FRAME_SIZE: usize = 17;

/* C header condition: non-i386, and __ASSEMBLER__ or __FRAME_OFFSETS. */
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R15: usize = 0;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R14: usize = 8;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R13: usize = 16;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R12: usize = 24;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RBP: usize = 32;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RBX: usize = 40;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R11: usize = 48;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R10: usize = 56;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R9: usize = 64;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const R8: usize = 72;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RAX: usize = 80;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RCX: usize = 88;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RDX: usize = 96;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RSI: usize = 104;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RDI: usize = 112;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const ORIG_RAX: usize = 120;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RIP: usize = 128;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const CS: usize = 136;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const EFLAGS: usize = 144;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const RSP: usize = 152;
#[cfg(all(not(target_arch = "x86"), any(feature = "assembler", feature = "frame_offsets")))]
pub const SS: usize = 160;

#[cfg(not(target_arch = "x86"))]
pub const FRAME_SIZE: usize = 168;

/* Arbitrarily choose the same ptrace numbers as used by the Sparc code. */
pub const PTRACE_GETREGS: usize = 12;
pub const PTRACE_SETREGS: usize = 13;
pub const PTRACE_GETFPREGS: usize = 14;
pub const PTRACE_SETFPREGS: usize = 15;
pub const PTRACE_GETFPXREGS: usize = 18;
pub const PTRACE_SETFPXREGS: usize = 19;
pub const PTRACE_OLDSETOPTIONS: usize = 21;

/* only useful for access 32bit programs / kernels */
pub const PTRACE_GET_THREAD_AREA: usize = 25;
pub const PTRACE_SET_THREAD_AREA: usize = 26;

/* C header condition: __x86_64__ */
#[cfg(target_arch = "x86_64")]
pub const PTRACE_ARCH_PRCTL: usize = 30;

pub const PTRACE_SYSEMU: usize = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: usize = 32;
pub const PTRACE_SINGLEBLOCK: usize = 33; /* resume execution until next branch */

/* <linux/types.h> is a C-only dependency; it declares no local items here. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
