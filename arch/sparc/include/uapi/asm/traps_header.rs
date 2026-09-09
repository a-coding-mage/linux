/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * traps.h:  Format of entries for the Sparc trap table.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

pub const NUM_SPARC_TRAPS: u32 = 255;

/* For patching the trap table at boot time, we need to know how to
 * form various common Sparc instructions.  Thus these macros...
 */

#[macro_export]
macro_rules! SPARC_MOV_CONST_L3 {
    ($const_:expr) => {
        0xa6102000u32 | (($const_ as u32) & 0xfff)
    };
}

/* The following assumes that the branch lies before the place we
 * are branching to.  This is the case for a trap vector...
 * You have been warned.
 */
#[macro_export]
macro_rules! SPARC_BRANCH {
    ($dest_addr:expr, $inst_addr:expr) => {
        0x10800000u32 | (((($dest_addr - $inst_addr) >> 2) as u32) & 0x3fffff)
    };
}

pub const SPARC_RD_PSR_L0: u32 = 0xa1480000;
pub const SPARC_RD_WIM_L3: u32 = 0xa7500000;
pub const SPARC_NOP: u32 = 0x01000000;

/* Various interesting trap levels. */
/* First, hardware traps. */
pub const SP_TRAP_TFLT: u32 = 0x1;   /* Text fault */
pub const SP_TRAP_II: u32 = 0x2;     /* Illegal Instruction */
pub const SP_TRAP_PI: u32 = 0x3;     /* Privileged Instruction */
pub const SP_TRAP_FPD: u32 = 0x4;    /* Floating Point Disabled */
pub const SP_TRAP_WOVF: u32 = 0x5;   /* Window Overflow */
pub const SP_TRAP_WUNF: u32 = 0x6;   /* Window Underflow */
pub const SP_TRAP_MNA: u32 = 0x7;    /* Memory Address Unaligned */
pub const SP_TRAP_FPE: u32 = 0x8;    /* Floating Point Exception */
pub const SP_TRAP_DFLT: u32 = 0x9;   /* Data Fault */
pub const SP_TRAP_TOF: u32 = 0xa;    /* Tag Overflow */
pub const SP_TRAP_WDOG: u32 = 0xb;   /* Watchpoint Detected */
pub const SP_TRAP_IRQ1: u32 = 0x11;  /* IRQ level 1 */
pub const SP_TRAP_IRQ2: u32 = 0x12;  /* IRQ level 2 */
pub const SP_TRAP_IRQ3: u32 = 0x13;  /* IRQ level 3 */
pub const SP_TRAP_IRQ4: u32 = 0x14;  /* IRQ level 4 */
pub const SP_TRAP_IRQ5: u32 = 0x15;  /* IRQ level 5 */
pub const SP_TRAP_IRQ6: u32 = 0x16;  /* IRQ level 6 */
pub const SP_TRAP_IRQ7: u32 = 0x17;  /* IRQ level 7 */
pub const SP_TRAP_IRQ8: u32 = 0x18;  /* IRQ level 8 */
pub const SP_TRAP_IRQ9: u32 = 0x19;  /* IRQ level 9 */
pub const SP_TRAP_IRQ10: u32 = 0x1a; /* IRQ level 10 */
pub const SP_TRAP_IRQ11: u32 = 0x1b; /* IRQ level 11 */
pub const SP_TRAP_IRQ12: u32 = 0x1c; /* IRQ level 12 */
pub const SP_TRAP_IRQ13: u32 = 0x1d; /* IRQ level 13 */
pub const SP_TRAP_IRQ14: u32 = 0x1e; /* IRQ level 14 */
pub const SP_TRAP_IRQ15: u32 = 0x1f; /* IRQ level 15 Non-maskable */
pub const SP_TRAP_RACC: u32 = 0x20;  /* Register Access Error ??? */
pub const SP_TRAP_IACC: u32 = 0x21;  /* Instruction Access Error */
pub const SP_TRAP_CPDIS: u32 = 0x24; /* Co-Processor Disabled */
pub const SP_TRAP_BADFL: u32 = 0x25; /* Unimplemented Flush Instruction */
pub const SP_TRAP_CPEXP: u32 = 0x28; /* Co-Processor Exception */
pub const SP_TRAP_DACC: u32 = 0x29;  /* Data Access Error */
pub const SP_TRAP_DIVZ: u32 = 0x2a;  /* Divide By Zero */
pub const SP_TRAP_DSTORE: u32 = 0x2b; /* Data Store Error ??? */
pub const SP_TRAP_DMM: u32 = 0x2c;    /* Data Access MMU Miss ??? */
pub const SP_TRAP_IMM: u32 = 0x3c;    /* Instruction Access MMU Miss ??? */

/* Now the Software Traps... */
pub const SP_TRAP_SUNOS: u32 = 0x80;   /* SunOS System Call */
pub const SP_TRAP_SBPT: u32 = 0x81;    /* Software Breakpoint */
pub const SP_TRAP_SDIVZ: u32 = 0x82;   /* Software Divide-by-Zero trap */
pub const SP_TRAP_FWIN: u32 = 0x83;    /* Flush Windows */
pub const SP_TRAP_CWIN: u32 = 0x84;    /* Clean Windows */
pub const SP_TRAP_RCHK: u32 = 0x85;    /* Range Check */
pub const SP_TRAP_FUNA: u32 = 0x86;    /* Fix Unaligned Access */
pub const SP_TRAP_IOWFL: u32 = 0x87;   /* Integer Overflow */
pub const SP_TRAP_SOLARIS: u32 = 0x88; /* Solaris System Call */
pub const SP_TRAP_NETBSD: u32 = 0x89;  /* NetBSD System Call */
pub const SP_TRAP_LINUX: u32 = 0x90;   /* Linux System Call */

/* Names used for compatibility with SunOS */
pub const ST_SYSCALL: u32 = 0x00;
pub const ST_BREAKPOINT: u32 = 0x01;
pub const ST_DIV0: u32 = 0x02;
pub const ST_FLUSH_WINDOWS: u32 = 0x03;
pub const ST_CLEAN_WINDOWS: u32 = 0x04;
pub const ST_RANGE_CHECK: u32 = 0x05;
pub const ST_FIX_ALIGN: u32 = 0x06;
pub const ST_INT_OVERFLOW: u32 = 0x07;

/* Special traps... */
pub const SP_TRAP_KBPT1: u32 = 0xfe; /* KADB/PROM Breakpoint one */
pub const SP_TRAP_KBPT2: u32 = 0xff; /* KADB/PROM Breakpoint two */

/* Handy Macros */
/* Is this a trap we never expect to get? */
#[macro_export]
macro_rules! BAD_TRAP_P {
    ($level:expr) => {
        (($level > $crate::SP_TRAP_WDOG && $level < $crate::SP_TRAP_IRQ1)
            || ($level > $crate::SP_TRAP_IACC && $level < $crate::SP_TRAP_CPDIS)
            || ($level > $crate::SP_TRAP_BADFL && $level < $crate::SP_TRAP_CPEXP)
            || ($level > $crate::SP_TRAP_DMM && $level < $crate::SP_TRAP_IMM)
            || ($level > $crate::SP_TRAP_IMM && $level < $crate::SP_TRAP_SUNOS)
            || ($level > $crate::SP_TRAP_LINUX && $level < $crate::SP_TRAP_KBPT1))
    };
}

/* Is this a Hardware trap? */
#[macro_export]
macro_rules! HW_TRAP_P {
    ($level:expr) => {
        (($level > 0) && ($level < $crate::SP_TRAP_SUNOS))
    };
}

/* Is this a Software trap? */
#[macro_export]
macro_rules! SW_TRAP_P {
    ($level:expr) => {
        (($level >= $crate::SP_TRAP_SUNOS) && ($level <= $crate::SP_TRAP_KBPT2))
    };
}

/* Is this a system call for some OS we know about? */
#[macro_export]
macro_rules! SCALL_TRAP_P {
    ($level:expr) => {
        (($level == $crate::SP_TRAP_SUNOS)
            || ($level == $crate::SP_TRAP_SOLARIS)
            || ($level == $crate::SP_TRAP_NETBSD)
            || ($level == $crate::SP_TRAP_LINUX))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
