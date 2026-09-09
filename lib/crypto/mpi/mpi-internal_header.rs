/* SPDX-License-Identifier: GPL-2.0-or-later */
/* mpi-internal.h - Internal to the Multi Precision Integers
 *
 * This file is a source-level Rust translation of the C header.
 * C/Linux dependencies and build-time configuration are supplied externally.
 */

// C includes: linux/module.h, linux/kernel.h, linux/slab.h, linux/string.h,
// linux/mpi.h, and linux/errno.h.

/* C macros log_debug and log_bug map to printk. */
// C: #define log_debug printk
// C: #define log_bug printk

#[macro_export]
macro_rules! assert {
    ($x:expr) => {
        if !$x {
            log_bug!("failed assertion\n");
        }
    };
}

/* If KARATSUBA_THRESHOLD is not already defined, its usual value is 16. */
pub const KARATSUBA_THRESHOLD: usize = 16;

/* The code cannot handle KARATSUBA_THRESHOLD smaller than 2. */
// Build-time configurations defining a smaller value must use 2 instead.

pub type mpi_ptr_t = *mut mpi_limb_t; /* pointer to a limb */
pub type mpi_size_t = libc::c_int; /* must be a signed type */

#[inline]
pub unsafe fn RESIZE_IF_NEEDED(a: MPI, b: libc::c_uint) -> libc::c_int {
    if (*a).alloced < b {
        mpi_resize(a, b)
    } else {
        0
    }
}

/* Copy N limbs from S to D. */
#[macro_export]
macro_rules! MPN_COPY {
    ($d:expr, $s:expr, $n:expr) => {{
        let mut _i: mpi_size_t = 0;
        while _i < $n {
            *$d.add(_i as usize) = *$s.add(_i as usize);
            _i += 1;
        }
    }};
}

#[macro_export]
macro_rules! MPN_COPY_DECR {
    ($d:expr, $s:expr, $n:expr) => {{
        let mut _i: mpi_size_t = $n - 1;
        while _i >= 0 {
            *$d.add(_i as usize) = *$s.add(_i as usize);
            _i -= 1;
        }
    }};
}

#[macro_export]
macro_rules! MPN_ZERO {
    ($d:expr, $n:expr) => {{
        let mut _i: libc::c_int = 0;
        while _i < $n {
            *$d.add(_i as usize) = 0;
            _i += 1;
        }
    }};
}

#[macro_export]
macro_rules! MPN_NORMALIZE {
    ($d:expr, $n:expr) => {{
        while $n > 0 {
            if *$d.add(($n - 1) as usize) != 0 { break; }
            $n -= 1;
        }
    }};
}

#[macro_export]
macro_rules! MPN_MUL_N_RECURSE {
    ($prodp:expr, $up:expr, $vp:expr, $size:expr, $tspace:expr) => {{
        if $size < KARATSUBA_THRESHOLD as mpi_size_t {
            mul_n_basecase($prodp, $up, $vp, $size);
        } else {
            mul_n($prodp, $up, $vp, $size, $tspace);
        }
    }};
}

/* UDIV_QRNND_PREINV is retained as an external-operation macro. */
// Its C implementation uses umul_ppmm and sub_ddmmss with the exact
// two-limb quotient/remainder correction sequence.
#[macro_export]
macro_rules! UDIV_QRNND_PREINV {
    ($q:expr, $r:expr, $nh:expr, $nl:expr, $d:expr, $di:expr) => {{
        let mut _ql: mpi_limb_t = 0;
        let mut _q: mpi_limb_t = 0;
        let mut _r: mpi_limb_t = 0;
        let mut _xh: mpi_limb_t = 0;
        let mut _xl: mpi_limb_t = 0;
        umul_ppmm!(_q, _ql, $nh, $di);
        _q = _q.wrapping_add($nh);
        umul_ppmm!(_xh, _xl, _q, $d);
        sub_ddmmss!(_xh, _r, $nh, $nl, _xh, _xl);
        if _xh != 0 {
            sub_ddmmss!(_xh, _r, _xh, _r, 0, $d);
            _q = _q.wrapping_add(1);
            if _xh != 0 {
                sub_ddmmss!(_xh, _r, _xh, _r, 0, $d);
                _q = _q.wrapping_add(1);
            }
        }
        if _r >= $d {
            _r = _r.wrapping_sub($d);
            _q = _q.wrapping_add(1);
        }
        $r = _r;
        $q = _q;
    }};
}

extern "C" {
    pub fn mpi_alloc_limb_space(nlimbs: libc::c_uint) -> mpi_ptr_t;
    pub fn mpi_free_limb_space(a: mpi_ptr_t);
    pub fn mpi_assign_limb_space(a: MPI, ap: mpi_ptr_t, nlimbs: libc::c_uint);

    pub fn mpihelp_add_1(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_add_n(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s2_ptr: mpi_ptr_t, size: mpi_size_t) -> mpi_limb_t;
    pub fn mpihelp_add(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_ptr: mpi_ptr_t, s2_size: mpi_size_t) -> mpi_limb_t;
    pub fn mpihelp_sub_1(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_sub_n(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s2_ptr: mpi_ptr_t, size: mpi_size_t) -> mpi_limb_t;
    pub fn mpihelp_sub(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_ptr: mpi_ptr_t, s2_size: mpi_size_t) -> mpi_limb_t;
    pub fn mpihelp_cmp(op1_ptr: mpi_ptr_t, op2_ptr: mpi_ptr_t, size: mpi_size_t) -> libc::c_int;

    pub fn mpihelp_release_karatsuba_ctx(ctx: *mut karatsuba_ctx);
    pub fn mpihelp_addmul_1(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_submul_1(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_mul(prodp: mpi_ptr_t, up: mpi_ptr_t, usize_: mpi_size_t, vp: mpi_ptr_t, vsize: mpi_size_t, result: *mut mpi_limb_t) -> libc::c_int;
    pub fn mpih_sqr_n_basecase(prodp: mpi_ptr_t, up: mpi_ptr_t, size: mpi_size_t);
    pub fn mpih_sqr_n(prodp: mpi_ptr_t, up: mpi_ptr_t, size: mpi_size_t, tspace: mpi_ptr_t);
    pub fn mpihelp_mul_karatsuba_case(prodp: mpi_ptr_t, up: mpi_ptr_t, usize_: mpi_size_t, vp: mpi_ptr_t, vsize: mpi_size_t, ctx: *mut karatsuba_ctx) -> libc::c_int;
    pub fn mpihelp_mul_1(res_ptr: mpi_ptr_t, s1_ptr: mpi_ptr_t, s1_size: mpi_size_t, s2_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_mod_1(dividend_ptr: mpi_ptr_t, dividend_size: mpi_size_t, divisor_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_divrem(qp: mpi_ptr_t, qextra_limbs: mpi_size_t, np: mpi_ptr_t, nsize: mpi_size_t, dp: mpi_ptr_t, dsize: mpi_size_t) -> mpi_limb_t;
    pub fn mpihelp_divmod_1(quot_ptr: mpi_ptr_t, dividend_ptr: mpi_ptr_t, dividend_size: mpi_size_t, divisor_limb: mpi_limb_t) -> mpi_limb_t;
    pub fn mpihelp_lshift(wp: mpi_ptr_t, up: mpi_ptr_t, usize_: mpi_size_t, cnt: libc::c_uint) -> mpi_limb_t;
    pub fn mpihelp_rshift(wp: mpi_ptr_t, up: mpi_ptr_t, usize_: mpi_size_t, cnt: libc::c_uint) -> mpi_limb_t;
}

#[repr(C)]
pub struct karatsuba_ctx {
    pub next: *mut karatsuba_ctx,
    pub tspace: mpi_ptr_t,
    pub tspace_size: mpi_size_t,
    pub tp: mpi_ptr_t,
    pub tp_size: mpi_size_t,
}

/* Definitions for longlong.h. */
// C: #define W_TYPE_SIZE BITS_PER_MPI_LIMB
pub const W_TYPE_SIZE: usize = BITS_PER_MPI_LIMB;
pub type UWtype = mpi_limb_t;
pub type UHWtype = libc::c_uint;
pub type UQItype = libc::c_uchar;
pub type SItype = libc::c_int;
pub type USItype = libc::c_uint;
pub type DItype = libc::c_long;
pub type UDItype = libc::c_ulong;

// GCC-only mpi-inline.h declarations are supplied by the surrounding build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
