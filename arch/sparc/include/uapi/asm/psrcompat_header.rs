/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency supplied by the corresponding pstate translation. */

/* Old 32-bit PSR fields for the compatibility conversion code. */
pub const PSR_CWP: u32 = 0x0000001f; /* current window pointer */
pub const PSR_ET: u32 = 0x00000020; /* enable traps field */
pub const PSR_PS: u32 = 0x00000040; /* previous privilege level */
pub const PSR_S: u32 = 0x00000080; /* current privilege level */
pub const PSR_PIL: u32 = 0x00000f00; /* processor interrupt level */
pub const PSR_EF: u32 = 0x00001000; /* enable floating point */
pub const PSR_EC: u32 = 0x00002000; /* enable co-processor */
pub const PSR_SYSCALL: u32 = 0x00004000; /* inside of a syscall */
pub const PSR_LE: u32 = 0x00008000; /* SuperSparcII little-endian */
pub const PSR_ICC: u32 = 0x00f00000; /* integer condition codes */
pub const PSR_C: u32 = 0x00100000; /* carry bit */
pub const PSR_V: u32 = 0x00200000; /* overflow bit */
pub const PSR_Z: u32 = 0x00400000; /* zero bit */
pub const PSR_N: u32 = 0x00800000; /* negative bit */
pub const PSR_VERS: u32 = 0x0f000000; /* cpu-version field */
pub const PSR_IMPL: u32 = 0xf0000000; /* cpu-implementation field */

pub const PSR_V8PLUS: u32 = 0xff000000; /* fake impl/ver, meaning a 64bit CPU is present */
pub const PSR_XCC: u32 = 0x000f0000; /* if PSR_V8PLUS, this is %xcc */

#[inline]
pub fn tstate_to_psr(tstate: u64) -> u32 {
    ((tstate & TSTATE_CWP)
        | (PSR_S as u64)
        | ((tstate & TSTATE_ICC) >> 12)
        | ((tstate & TSTATE_XCC) >> 20)
        | if (tstate & TSTATE_SYSCALL) != 0 {
            PSR_SYSCALL as u64
        } else {
            0
        }
        | (PSR_V8PLUS as u64)) as u32
}

#[inline]
pub fn psr_to_tstate_icc(psr: u32) -> u64 {
    let mut tstate: u64 = ((psr & PSR_ICC) as u64) << 12;
    if (psr & (PSR_VERS | PSR_IMPL)) == PSR_V8PLUS {
        tstate |= ((psr & PSR_XCC) as u64) << 20;
    }
    tstate
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
