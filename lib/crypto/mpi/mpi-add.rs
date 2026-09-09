/* mpi-add.c  -  MPI functions
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

// Dependencies supplied by mpi-internal.h and the kernel build environment.

pub unsafe fn mpi_add(w: MPI, u: MPI, v: MPI) -> i32 {
    let (mut wp, mut up, mut vp): (mpi_ptr_t, mpi_ptr_t, mpi_ptr_t);
    let (mut usize, mut vsize, mut wsize): (mpi_size_t, mpi_size_t, mpi_size_t);
    let (mut usign, mut vsign, mut wsign): (i32, i32, i32);
    let mut err: i32;

    if (*u).nlimbs < (*v).nlimbs { /* Swap U and V. */
        usize = (*v).nlimbs;
        usign = (*v).sign;
        vsize = (*u).nlimbs;
        vsign = (*u).sign;
        wsize = usize + 1;
        err = RESIZE_IF_NEEDED!(w, wsize);
        if err != 0 {
            return err;
        }
        /* These must be after realloc (u or v may be the same as w).  */
        up = (*v).d;
        vp = (*u).d;
    } else {
        usize = (*u).nlimbs;
        usign = (*u).sign;
        vsize = (*v).nlimbs;
        vsign = (*v).sign;
        wsize = usize + 1;
        err = RESIZE_IF_NEEDED!(w, wsize);
        if err != 0 {
            return err;
        }
        /* These must be after realloc (u or v may be the same as w).  */
        up = (*u).d;
        vp = (*v).d;
    }
    wp = (*w).d;
    wsign = 0;

    if vsize == 0 { /* simple */
        MPN_COPY!(wp, up, usize);
        wsize = usize;
        wsign = usign;
    } else if usign != vsign { /* different sign */
        /* This test is right since USIZE >= VSIZE */
        if usize != vsize {
            mpihelp_sub(wp, up, usize, vp, vsize);
            wsize = usize;
            MPN_NORMALIZE!(wp, wsize);
            wsign = usign;
        } else if mpihelp_cmp(up, vp, usize) < 0 {
            mpihelp_sub_n(wp, vp, up, usize);
            wsize = usize;
            MPN_NORMALIZE!(wp, wsize);
            if usign == 0 {
                wsign = 1;
            }
        } else {
            mpihelp_sub_n(wp, up, vp, usize);
            wsize = usize;
            MPN_NORMALIZE!(wp, wsize);
            if usign != 0 {
                wsign = 1;
            }
        }
    } else { /* U and V have same sign. Add them. */
        let cy: mpi_limb_t = mpihelp_add(wp, up, usize, vp, vsize);
        *wp.add(usize as usize) = cy;
        wsize = usize + cy;
        if usign != 0 {
            wsign = 1;
        }
    }

    (*w).nlimbs = wsize;
    (*w).sign = wsign;
    0
}

pub unsafe fn mpi_sub(w: MPI, u: MPI, v: MPI) -> i32 {
    let mut err: i32;
    let vv: MPI;

    vv = mpi_copy(v);
    if vv.is_null() {
        return -ENOMEM;
    }

    (*vv).sign = if (*vv).sign == 0 { 1 } else { 0 };
    err = mpi_add(w, u, vv);
    mpi_free(vv);

    err
}

pub unsafe fn mpi_addm(w: MPI, u: MPI, v: MPI, m: MPI) -> i32 {
    let err = mpi_add(w, u, v);
    if err != 0 { err } else { mpi_mod(w, w, m) }
}

pub unsafe fn mpi_subm(w: MPI, u: MPI, v: MPI, m: MPI) -> i32 {
    let err = mpi_sub(w, u, v);
    if err != 0 { err } else { mpi_mod(w, w, m) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
