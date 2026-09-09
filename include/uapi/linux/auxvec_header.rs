/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: <asm/auxvec.h>

/* Symbolic values for the entries in the auxiliary table
   put on the initial stack */
pub const AT_NULL: i32 = 0; // end of vector
pub const AT_IGNORE: i32 = 1; // entry should be ignored
pub const AT_EXECFD: i32 = 2; // file descriptor of program
pub const AT_PHDR: i32 = 3; // program headers for program
pub const AT_PHENT: i32 = 4; // size of program header entry
pub const AT_PHNUM: i32 = 5; // number of program headers
pub const AT_PAGESZ: i32 = 6; // system page size
pub const AT_BASE: i32 = 7; // base address of interpreter
pub const AT_FLAGS: i32 = 8; // flags
pub const AT_ENTRY: i32 = 9; // entry point of program
pub const AT_NOTELF: i32 = 10; // program is not ELF
pub const AT_UID: i32 = 11; // real uid
pub const AT_EUID: i32 = 12; // effective uid
pub const AT_GID: i32 = 13; // real gid
pub const AT_EGID: i32 = 14; // effective gid
pub const AT_PLATFORM: i32 = 15; // string identifying CPU for optimizations
pub const AT_HWCAP: i32 = 16; // arch dependent hints at CPU capabilities
pub const AT_CLKTCK: i32 = 17; // frequency at which times() increments
// AT_* values 18 through 22 are reserved
pub const AT_SECURE: i32 = 23; // secure mode boolean
pub const AT_BASE_PLATFORM: i32 = 24; // string identifying real platform, may
// differ from AT_PLATFORM.
pub const AT_RANDOM: i32 = 25; // address of 16 random bytes
pub const AT_HWCAP2: i32 = 26; // extension of AT_HWCAP
pub const AT_RSEQ_FEATURE_SIZE: i32 = 27; // rseq supported feature size
pub const AT_RSEQ_ALIGN: i32 = 28; // rseq allocation alignment
pub const AT_HWCAP3: i32 = 29; // extension of AT_HWCAP
pub const AT_HWCAP4: i32 = 30; // extension of AT_HWCAP

pub const AT_EXECFN: i32 = 31; // filename of program

// C preprocessor condition: define only when AT_MINSIGSTKSZ is not already defined.
pub const AT_MINSIGSTKSZ: i32 = 51; // minimal stack size for signal delivery

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
