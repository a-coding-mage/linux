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
    pub r0: kernel::ffi::c_ulong,
    pub r1: kernel::ffi::c_ulong,
    pub r2: kernel::ffi::c_ulong,
    pub r3: kernel::ffi::c_ulong,
    pub r4: kernel::ffi::c_ulong,
    pub r5: kernel::ffi::c_ulong,
    pub r6: kernel::ffi::c_ulong,
    pub r7: kernel::ffi::c_ulong,
    pub r8: kernel::ffi::c_ulong,
    pub r9: kernel::ffi::c_ulong,
    pub r10: kernel::ffi::c_ulong,
    pub r11: kernel::ffi::c_ulong,
    pub r12: kernel::ffi::c_ulong,
    pub r13: kernel::ffi::c_ulong,
    pub r14: kernel::ffi::c_ulong,
    pub r15: kernel::ffi::c_ulong,
    pub r16: kernel::ffi::c_ulong,
    pub r17: kernel::ffi::c_ulong,
    pub r18: kernel::ffi::c_ulong,
    pub r19: kernel::ffi::c_ulong,
    pub r20: kernel::ffi::c_ulong,
    pub r21: kernel::ffi::c_ulong,
    pub r22: kernel::ffi::c_ulong,
    pub r23: kernel::ffi::c_ulong,
    pub r24: kernel::ffi::c_ulong,
    pub r25: kernel::ffi::c_ulong,
    pub r26: kernel::ffi::c_ulong,
    pub r27: kernel::ffi::c_ulong,
    pub r28: kernel::ffi::c_ulong,
    pub r29: kernel::ffi::c_ulong,
    pub r30: kernel::ffi::c_ulong,
    pub r31: kernel::ffi::c_ulong,
    pub sa0: kernel::ffi::c_ulong,
    pub lc0: kernel::ffi::c_ulong,
    pub sa1: kernel::ffi::c_ulong,
    pub lc1: kernel::ffi::c_ulong,
    pub m0: kernel::ffi::c_ulong,
    pub m1: kernel::ffi::c_ulong,
    pub usr: kernel::ffi::c_ulong,
    pub p3_0: kernel::ffi::c_ulong,
    pub gp: kernel::ffi::c_ulong,
    pub ugp: kernel::ffi::c_ulong,
    pub pc: kernel::ffi::c_ulong,
    pub cause: kernel::ffi::c_ulong,
    pub badva: kernel::ffi::c_ulong,
    /* cs0 and cs1 are only available with HEXAGON_ARCH_VERSION >= 4 */
    pub cs0: kernel::ffi::c_ulong,
    pub cs1: kernel::ffi::c_ulong,
    pub pad1: kernel::ffi::c_ulong, /* pad out to 48 words total */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
