/* mpi-mul.c  -  MPI functions
 * Copyright (C) 1994, 1996, 1998, 2001, 2002,
 *               2003 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Note: This code is heavily based on the GNU MP Library.
 *      Actually it's the same code with only minor changes in the
 *      way the data is stored; this is to support the abstraction
 *      of an optional secure memory allocation which may be used
 *      to avoid revealing of sensitive data due to paging etc.
 */

// Dependency supplied by mpi-internal.h in the original source.

pub unsafe fn mpi_mul(w: MPI, u: MPI, v: MPI) -> i32 {
    let (mut usize_, mut vsize, mut up, mut vp, mut usign, mut vsign);
    let mut wsize: mpi_size_t;
    let mut cy: mpi_limb_t = 0;
    let sign_product: i32;
    let mut assign_wp: i32 = 0;
    let mut tmp_limb: mpi_ptr_t = core::ptr::null_mut();
    let mut err: i32 = 0;

    if (*u).nlimbs < (*v).nlimbs {
        /* Swap U and V. */
        usize_ = (*v).nlimbs;
        usign = (*v).sign;
        up = (*v).d;
        vsize = (*u).nlimbs;
        vsign = (*u).sign;
        vp = (*u).d;
    } else {
        usize_ = (*u).nlimbs;
        usign = (*u).sign;
        up = (*u).d;
        vsize = (*v).nlimbs;
        vsign = (*v).sign;
        vp = (*v).d;
    }
    sign_product = usign ^ vsign;
    let mut wp = (*w).d;

    /* Ensure W has space enough to store the result.  */
    wsize = usize_ + vsize;
    if (*w).alloced < wsize {
        if wp == up || wp == vp {
            wp = mpi_alloc_limb_space(wsize);
            if wp.is_null() {
                return -ENOMEM;
            }
            assign_wp = 1;
        } else {
            err = mpi_resize(w, wsize);
            if err != 0 {
                return err;
            }
            wp = (*w).d;
        }
    } else {
        /* Make U and V not overlap with W.  */
        if wp == up {
            /* W and U are identical.  Allocate temporary space for U. */
            up = mpi_alloc_limb_space(usize_);
            tmp_limb = up;
            if up.is_null() {
                return -ENOMEM;
            }
            /* Is V identical too?  Keep it identical with U.  */
            if wp == vp {
                vp = up;
            }
            /* Copy to the temporary space.  */
            MPN_COPY(up, wp, usize_);
        } else if wp == vp {
            /* W and V are identical.  Allocate temporary space for V. */
            vp = mpi_alloc_limb_space(vsize);
            tmp_limb = vp;
            if vp.is_null() {
                return -ENOMEM;
            }
            /* Copy to the temporary space.  */
            MPN_COPY(vp, wp, vsize);
        }
    }

    if vsize == 0 {
        wsize = 0;
    } else {
        err = mpihelp_mul(wp, up, usize_, vp, vsize, &mut cy);
        if err != 0 {
            if assign_wp != 0 {
                mpi_free_limb_space(wp);
            }
            if !tmp_limb.is_null() {
                mpi_free_limb_space(tmp_limb);
            }
            return err;
        }
        if cy == 0 {
            wsize -= 1;
        }
    }

    if assign_wp != 0 {
        mpi_assign_limb_space(w, wp, wsize);
    }
    (*w).nlimbs = wsize;
    (*w).sign = sign_product;

    if !tmp_limb.is_null() {
        mpi_free_limb_space(tmp_limb);
    }
    err
}

pub unsafe fn mpi_mulm(w: MPI, u: MPI, v: MPI, m: MPI) -> i32 {
    let err = mpi_mul(w, u, v);
    if err != 0 {
        err
    } else {
        mpi_tdiv_r(w, w, m)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
