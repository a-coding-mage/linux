/* SPDX-License-Identifier: GPL-2.0-or-later */
/* mpi-inline.h - Internal to the Multi Precision Integers
 *
 * Rust translation of the original C header.  The original includes and
 * header guards are intentionally omitted; mpi_limb_t, mpi_ptr_t, and
 * mpi_size_t are supplied by the surrounding MPI implementation.
 */

unsafe extern "C" {
    fn mpihelp_add_n(
        res_ptr: mpi_ptr_t,
        s1_ptr: mpi_ptr_t,
        s2_ptr: mpi_ptr_t,
        size: mpi_size_t,
    ) -> mpi_limb_t;
    fn mpihelp_sub_n(
        res_ptr: mpi_ptr_t,
        s1_ptr: mpi_ptr_t,
        s2_ptr: mpi_ptr_t,
        size: mpi_size_t,
    ) -> mpi_limb_t;
}

#[inline]
pub unsafe fn mpihelp_add_1(
    mut res_ptr: mpi_ptr_t,
    mut s1_ptr: mpi_ptr_t,
    mut s1_size: mpi_size_t,
    mut s2_limb: mpi_limb_t,
) -> mpi_limb_t {
    let mut x: mpi_limb_t;

    x = *s1_ptr;
    s1_ptr = s1_ptr.add(1);
    s2_limb = s2_limb.wrapping_add(x);
    *res_ptr = s2_limb;
    res_ptr = res_ptr.add(1);
    if s2_limb < x {
        while {
            s1_size = s1_size.wrapping_sub(1);
            s1_size != 0
        } {
            x = (*s1_ptr).wrapping_add(1);
            s1_ptr = s1_ptr.add(1);
            *res_ptr = x;
            res_ptr = res_ptr.add(1);
            if x != 0 {
                break;
            }
        }
        if s1_size == 0 {
            return 1;
        }
    }

    if res_ptr != s1_ptr {
        let mut i: mpi_size_t = 0;
        while i < s1_size.wrapping_sub(1) {
            *res_ptr.add(i) = *s1_ptr.add(i);
            i = i.wrapping_add(1);
        }
    }
    0
}

#[inline]
pub unsafe fn mpihelp_add(
    res_ptr: mpi_ptr_t,
    s1_ptr: mpi_ptr_t,
    s1_size: mpi_size_t,
    s2_ptr: mpi_ptr_t,
    s2_size: mpi_size_t,
) -> mpi_limb_t {
    let mut cy: mpi_limb_t = 0;

    if s2_size != 0 {
        cy = mpihelp_add_n(res_ptr, s1_ptr, s2_ptr, s2_size);
    }
    if s1_size.wrapping_sub(s2_size) != 0 {
        cy = mpihelp_add_1(
            res_ptr.add(s2_size),
            s1_ptr.add(s2_size),
            s1_size.wrapping_sub(s2_size),
            cy,
        );
    }
    cy
}

#[inline]
pub unsafe fn mpihelp_sub_1(
    mut res_ptr: mpi_ptr_t,
    mut s1_ptr: mpi_ptr_t,
    mut s1_size: mpi_size_t,
    mut s2_limb: mpi_limb_t,
) -> mpi_limb_t {
    let mut x: mpi_limb_t;

    x = *s1_ptr;
    s1_ptr = s1_ptr.add(1);
    s2_limb = x.wrapping_sub(s2_limb);
    *res_ptr = s2_limb;
    res_ptr = res_ptr.add(1);
    if s2_limb > x {
        while {
            s1_size = s1_size.wrapping_sub(1);
            s1_size != 0
        } {
            x = *s1_ptr;
            s1_ptr = s1_ptr.add(1);
            *res_ptr = x.wrapping_sub(1);
            res_ptr = res_ptr.add(1);
            if x != 0 {
                break;
            }
        }
        if s1_size == 0 {
            return 1;
        }
    }

    if res_ptr != s1_ptr {
        let mut i: mpi_size_t = 0;
        while i < s1_size.wrapping_sub(1) {
            *res_ptr.add(i) = *s1_ptr.add(i);
            i = i.wrapping_add(1);
        }
    }
    0
}

#[inline]
pub unsafe fn mpihelp_sub(
    res_ptr: mpi_ptr_t,
    s1_ptr: mpi_ptr_t,
    s1_size: mpi_size_t,
    s2_ptr: mpi_ptr_t,
    s2_size: mpi_size_t,
) -> mpi_limb_t {
    let mut cy: mpi_limb_t = 0;

    if s2_size != 0 {
        cy = mpihelp_sub_n(res_ptr, s1_ptr, s2_ptr, s2_size);
    }
    if s1_size.wrapping_sub(s2_size) != 0 {
        cy = mpihelp_sub_1(
            res_ptr.add(s2_size),
            s1_ptr.add(s2_size),
            s1_size.wrapping_sub(s2_size),
            cy,
        );
    }
    cy
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
