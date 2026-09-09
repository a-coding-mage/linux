/* mpi-div.c  -  MPI functions
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

// Dependencies supplied by mpi-internal.h and longlong.h remain external.

pub unsafe fn mpi_fdiv_r(rem: MPI, dividend: MPI, mut divisor: MPI) -> i32 {
    let divisor_sign = (*divisor).sign;
    let mut temp_divisor: MPI = core::ptr::null_mut();
    let mut err: i32;

    /* We need the original value of the divisor after the remainder has been
     * preliminary calculated. We have to copy it to temporary space if it's
     * the same variable as REM.
     */
    if rem == divisor {
        temp_divisor = mpi_copy(divisor);
        if temp_divisor.is_null() {
            return -ENOMEM;
        }
        divisor = temp_divisor;
    }

    err = mpi_tdiv_r(rem, dividend, divisor);
    if err != 0 {
        mpi_free(temp_divisor);
        return err;
    }

    if (((if divisor_sign != 0 { 1 } else { 0 })
        ^ (if (*dividend).sign != 0 { 1 } else { 0 })) != 0)
        && (*rem).nlimbs != 0
    {
        err = mpi_add(rem, rem, divisor);
    }

    mpi_free(temp_divisor);
    err
}

/* If den == quot, den needs temporary storage.
 * If den == rem, den needs temporary storage.
 * If num == quot, num needs temporary storage.
 * If den has temporary storage, it can be normalized while being copied,
 *   i.e no extra storage should be allocated.
 */

pub unsafe fn mpi_tdiv_r(rem: MPI, num: MPI, den: MPI) -> i32 {
    mpi_tdiv_qr(core::ptr::null_mut(), rem, num, den)
}

pub unsafe fn mpi_tdiv_qr(quot: MPI, rem: MPI, num: MPI, den: MPI) -> i32 {
    let mut np: mpi_ptr_t;
    let mut dp: mpi_ptr_t;
    let mut qp: mpi_ptr_t;
    let mut rp: mpi_ptr_t;
    let nsize = (*num).nlimbs;
    let dsize = (*den).nlimbs;
    let mut qsize: mpi_size_t;
    let mut rsize: mpi_size_t;
    let sign_remainder = (*num).sign;
    let sign_quotient = (*num).sign ^ (*den).sign;
    let mut normalization_steps: u32;
    let mut q_limb: mpi_limb_t;
    let mut marker: [mpi_ptr_t; 5] = [core::ptr::null_mut(); 5];
    let mut markidx: usize = 0;
    let mut err: i32;

    /* Ensure space is enough for quotient and remainder.
     * We need space for an extra limb in the remainder, because it's
     * up-shifted (normalized) below.
     */
    rsize = nsize + 1;
    err = mpi_resize(rem, rsize);
    if err != 0 {
        return err;
    }

    qsize = rsize - dsize;
    if qsize <= 0 {
        if num != rem {
            (*rem).nlimbs = (*num).nlimbs;
            (*rem).sign = (*num).sign;
            MPN_COPY((*rem).d, (*num).d, nsize);
        }
        if !quot.is_null() {
            (*quot).nlimbs = 0;
            (*quot).sign = 0;
        }
        return 0;
    }

    if !quot.is_null() {
        err = mpi_resize(quot, qsize);
        if err != 0 {
            return err;
        }
    }

    np = (*num).d;
    dp = (*den).d;
    rp = (*rem).d;

    if dsize == 1 {
        let rlimb: mpi_limb_t;
        if !quot.is_null() {
            qp = (*quot).d;
            rlimb = mpihelp_divmod_1(qp, np, nsize, *dp);
            qsize -= if *qp.add(qsize - 1) == 0 { 1 } else { 0 };
            (*quot).nlimbs = qsize;
            (*quot).sign = sign_quotient;
        } else {
            rlimb = mpihelp_mod_1(np, nsize, *dp);
        }
        *rp = rlimb;
        rsize = if rlimb != 0 { 1 } else { 0 };
        (*rem).nlimbs = rsize;
        (*rem).sign = sign_remainder;
        return 0;
    }

    err = -ENOMEM;
    if !quot.is_null() {
        qp = (*quot).d;
        if qp == np {
            np = mpi_alloc_limb_space(nsize);
            marker[markidx] = np;
            markidx += 1;
            if np.is_null() {
                goto out_free_marker;
            }
            MPN_COPY(np, qp, nsize);
        }
    } else {
        qp = rp.add(dsize as usize);
    }

    normalization_steps = count_leading_zeros(*dp.add((dsize - 1) as usize));

    if normalization_steps != 0 {
        let tp = mpi_alloc_limb_space(dsize);
        marker[markidx] = tp;
        markidx += 1;
        if tp.is_null() {
            goto out_free_marker;
        }
        mpihelp_lshift(tp, dp, dsize, normalization_steps);
        dp = tp;

        let nlimb = mpihelp_lshift(rp, np, nsize, normalization_steps);
        if nlimb != 0 {
            *rp.add(nsize as usize) = nlimb;
            rsize = nsize + 1;
        } else {
            rsize = nsize;
        }
    } else {
        if dp == rp || (!quot.is_null() && dp == qp) {
            let tp = mpi_alloc_limb_space(dsize);
            marker[markidx] = tp;
            markidx += 1;
            if tp.is_null() {
                goto out_free_marker;
            }
            MPN_COPY(tp, dp, dsize);
            dp = tp;
        }

        if rp != np {
            MPN_COPY(rp, np, nsize);
        }
        rsize = nsize;
    }

    q_limb = mpihelp_divrem(qp, 0, rp, rsize, dp, dsize);

    if !quot.is_null() {
        qsize = rsize - dsize;
        if q_limb != 0 {
            *qp.add(qsize as usize) = q_limb;
            qsize += 1;
        }
        (*quot).nlimbs = qsize;
        (*quot).sign = sign_quotient;
    }

    rsize = dsize;
    MPN_NORMALIZE(rp, rsize);

    if normalization_steps != 0 && rsize != 0 {
        mpihelp_rshift(rp, rp, rsize, normalization_steps);
        rsize -= if *rp.add((rsize - 1) as usize) == 0 { 1 } else { 0 };
    }

    (*rem).nlimbs = rsize;
    (*rem).sign = sign_remainder;
    err = 0;

out_free_marker:
    while markidx != 0 {
        markidx -= 1;
        mpi_free_limb_space(marker[markidx]);
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
