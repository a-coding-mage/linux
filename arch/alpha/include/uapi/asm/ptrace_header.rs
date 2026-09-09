/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This struct defines the way the registers are stored on the
 * kernel stack during a system call or other kernel entry
 *
 * NOTE! I want to minimize the overhead of system calls, so this
 * struct has as little information as possible. It does not have
 *
 *  - floating point regs: the kernel doesn't change those
 *  - r9-15: saved by the C compiler
 *
 * This makes "fork()" and "exec()" a bit more complex, but should
 * give us low system call latency.
 */
#[repr(C)]
pub struct pt_regs {
    pub r0: u64,
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub hae: u64,
    /* JRP - These are the values provided to a0-a2 by PALcode */
    pub trap_a0: u64,
    pub trap_a1: u64,
    pub trap_a2: u64,
    /* This makes the stack 16-byte aligned as GCC expects */
    pub usp: u64,
    /* These are saved by PAL-code: */
    pub ps: u64,
    pub pc: u64,
    pub gp: u64,
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
}

/*
 * This is the extended stack used by signal handlers and the context
 * switcher: it's pushed after the normal "struct pt_regs".
 */
#[repr(C)]
pub struct switch_stack {
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub r26: u64,
    /* __KERNEL__ condition: floating-point registers are present outside the kernel. */
    #[cfg(not(feature = "__KERNEL__"))]
    pub fp: [u64; 32], /* fp[31] is fpcr */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
