/* SPDX-License-Identifier: GPL-2.0 */
// Translated from sfp-util.h. Kernel and architecture-provided dependencies
// remain external to this translation.

pub type UDItype = u64;

#[inline]
pub fn add_ssaaaa(sh: &mut u64, sl: &mut u64, ah: u64, al: u64, bh: u64, bl: u64) {
    *sl = al.wrapping_add(bl);
    *sh = ah
        .wrapping_add(bh)
        .wrapping_add((*sl < al) as u64);
}

#[inline]
pub fn sub_ddmmss(sh: &mut u64, sl: &mut u64, ah: u64, al: u64, bh: u64, bl: u64) {
    *sl = al.wrapping_sub(bl);
    *sh = ah
        .wrapping_sub(bh)
        .wrapping_sub((al < bl) as u64);
}

#[inline]
pub fn umul_ppmm(wh: &mut UDItype, wl: &mut UDItype, u: UDItype, v: UDItype) {
    let product = (u as u128) * (v as u128);
    *wl = product as UDItype;
    *wh = (product >> 64) as UDItype;
}

extern "C" {
    pub fn __udiv_qrnnd(
        remainder: *mut libc::c_ulong,
        n1: libc::c_ulong,
        n0: libc::c_ulong,
        divisor: libc::c_ulong,
    ) -> libc::c_ulong;
}

#[inline]
pub unsafe fn udiv_qrnnd(
    q: &mut libc::c_ulong,
    r: &mut libc::c_ulong,
    n1: libc::c_ulong,
    n0: libc::c_ulong,
    d: libc::c_ulong,
) {
    let mut remainder: libc::c_ulong = 0;
    *q = __udiv_qrnnd(&mut remainder, n1, n0, d);
    *r = remainder;
}

pub const UDIV_NEEDS_NORMALIZATION: i32 = 1;

// C macro: abort() expands to `goto bad_insn`; Rust has no goto syntax.
// Callers must preserve the corresponding bad_insn control-flow destination.
#[macro_export]
macro_rules! abort {
    () => {{
        unreachable!("bad_insn")
    }};
}

pub const __LITTLE_ENDIAN: i32 = -1;
pub const __BYTE_ORDER: i32 = __LITTLE_ENDIAN;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
