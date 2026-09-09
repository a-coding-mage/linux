/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Layout for registers passed in elf core dumps to userspace.
 *
 * Basically a rearranged subset of "pt_regs".
 *
 * Interested parties:  libc, gdb...
 */

#[repr(C)]
pub struct user_regs_struct {
    pub r0: core::ffi::c_ulong,
    pub r1: core::ffi::c_ulong,
    pub r2: core::ffi::c_ulong,
    pub r3: core::ffi::c_ulong,
    pub r4: core::ffi::c_ulong,
    pub r5: core::ffi::c_ulong,
    pub r6: core::ffi::c_ulong,
    pub r7: core::ffi::c_ulong,
    pub r8: core::ffi::c_ulong,
    pub r9: core::ffi::c_ulong,
    pub r10: core::ffi::c_ulong,
    pub r11: core::ffi::c_ulong,
    pub r12: core::ffi::c_ulong,
    pub r13: core::ffi::c_ulong,
    pub r14: core::ffi::c_ulong,
    pub r15: core::ffi::c_ulong,
    pub r16: core::ffi::c_ulong,
    pub r17: core::ffi::c_ulong,
    pub r18: core::ffi::c_ulong,
    pub r19: core::ffi::c_ulong,
    pub r20: core::ffi::c_ulong,
    pub r21: core::ffi::c_ulong,
    pub r22: core::ffi::c_ulong,
    pub r23: core::ffi::c_ulong,
    pub r24: core::ffi::c_ulong,
    pub r25: core::ffi::c_ulong,
    pub r26: core::ffi::c_ulong,
    pub r27: core::ffi::c_ulong,
    pub r28: core::ffi::c_ulong,
    pub r29: core::ffi::c_ulong,
    pub r30: core::ffi::c_ulong,
    pub r31: core::ffi::c_ulong,
    pub sa0: core::ffi::c_ulong,
    pub lc0: core::ffi::c_ulong,
    pub sa1: core::ffi::c_ulong,
    pub lc1: core::ffi::c_ulong,
    pub m0: core::ffi::c_ulong,
    pub m1: core::ffi::c_ulong,
    pub usr: core::ffi::c_ulong,
    pub p3_0: core::ffi::c_ulong,
    pub gp: core::ffi::c_ulong,
    pub ugp: core::ffi::c_ulong,
    pub pc: core::ffi::c_ulong,
    pub cause: core::ffi::c_ulong,
    pub badva: core::ffi::c_ulong,
    /* cs0 and cs1 are only available with HEXAGON_ARCH_VERSION >= 4 */
    pub cs0: core::ffi::c_ulong,
    pub cs1: core::ffi::c_ulong,
    pub pad1: core::ffi::c_ulong, /* pad out to 48 words total */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
