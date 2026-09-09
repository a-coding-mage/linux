// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of mpi-pow.c. */

use core::ptr;

/* RES = BASE ^ EXP mod MOD */
pub unsafe fn mpi_powm(res: *mut MPI, base: *mut MPI, exp: *mut MPI, modu: *mut MPI) -> i32 {
    let mut mp_marker: mpi_ptr_t = ptr::null_mut();
    let mut bp_marker: mpi_ptr_t = ptr::null_mut();
    let mut ep_marker: mpi_ptr_t = ptr::null_mut();
    let mut karactx: karatsuba_ctx = core::mem::zeroed();
    let mut xp_marker: mpi_ptr_t = ptr::null_mut();
    let mut tspace: mpi_ptr_t = ptr::null_mut();
    let mut rp: mpi_ptr_t;
    let mut ep: mpi_ptr_t;
    let mut mp: mpi_ptr_t;
    let mut bp: mpi_ptr_t;
    let esize = (*exp).nlimbs;
    let msize = (*modu).nlimbs;
    let size = 2 * msize;
    let msign = (*modu).sign;
    rp = (*res).d;
    ep = (*exp).d;
    let mut assign_rp = false;
    let mut tsize: mpi_size_t = 0;
    let mut rc: i32 = -ENOMEM;

    if msize == 0 { return rc; }
    if esize == 0 {
        (*res).nlimbs = if msize == 1 && (*modu).d[0] == 1 { 0 } else { 1 };
        if (*res).nlimbs != 0 {
            if mpi_resize(res, 1) < 0 { return rc; }
            rp = (*res).d;
            *rp = 1;
        }
        (*res).sign = 0;
        rc = 0;
        return rc;
    }

    mp = mpi_alloc_limb_space(msize);
    mp_marker = mp;
    if mp.is_null() { return rc; }
    let mod_shift_cnt = count_leading_zeros((*modu).d[msize - 1]);
    if mod_shift_cnt != 0 { mpihelp_lshift(mp, (*modu).d, msize, mod_shift_cnt); }
    else { MPN_COPY(mp, (*modu).d, msize); }

    let mut bsize = (*base).nlimbs;
    let bsign = (*base).sign;
    if bsize > msize {
        bp = mpi_alloc_limb_space(bsize + 1);
        bp_marker = bp;
        if bp.is_null() { return rc; }
        MPN_COPY(bp, (*base).d, bsize);
        mpihelp_divrem(bp.add(msize), 0, bp, bsize, mp, msize);
        bsize = msize;
        MPN_NORMALIZE(bp, bsize);
    } else { bp = (*base).d; }

    if bsize == 0 {
        (*res).nlimbs = 0; (*res).sign = 0;
        rc = 0;
        goto cleanup;
    }

    if (*res).alloced < size {
        if rp == ep || rp == mp || rp == bp {
            rp = mpi_alloc_limb_space(size);
            if rp.is_null() { goto cleanup; }
            assign_rp = true;
        } else {
            if mpi_resize(res, size) < 0 { goto cleanup; }
            rp = (*res).d;
        }
    } else {
        if rp == bp {
            bp = mpi_alloc_limb_space(bsize); bp_marker = bp;
            if bp.is_null() { goto cleanup; }
            MPN_COPY(bp, rp, bsize);
        }
        if rp == ep {
            ep = mpi_alloc_limb_space(esize); ep_marker = ep;
            if ep.is_null() { goto cleanup; }
            MPN_COPY(ep, rp, esize);
        }
        if rp == mp {
            mp = mpi_alloc_limb_space(msize); mp_marker = mp;
            if mp.is_null() { goto cleanup; }
            MPN_COPY(mp, rp, msize);
        }
    }

    MPN_COPY(rp, bp, bsize);
    let mut rsize = bsize;
    let mut rsign = bsign;
    let xp = mpi_alloc_limb_space(2 * (msize + 1));
    xp_marker = xp;
    if xp.is_null() { goto cleanup; }
    let negative_result = (ep[0] & 1) != 0 && (*base).sign != 0;
    let mut i = esize - 1;
    let mut e = ep[i];
    let mut c = count_leading_zeros(e);
    e = (e << c) << 1;
    c = BITS_PER_MPI_LIMB - 1 - c;

    loop {
        while c != 0 {
            let mut xsize;
            if rsize < KARATSUBA_THRESHOLD { mpih_sqr_n_basecase(xp, rp, rsize); }
            else {
                if tspace.is_null() || tsize < 2 * rsize {
                    if !tspace.is_null() { mpi_free_limb_space(tspace); }
                    tsize = 2 * rsize; tspace = mpi_alloc_limb_space(tsize);
                    if tspace.is_null() { goto cleanup; }
                }
                mpih_sqr_n(xp, rp, rsize, tspace);
            }
            xsize = 2 * rsize;
            if xsize > msize { mpihelp_divrem(xp.add(msize), 0, xp, xsize, mp, msize); xsize = msize; }
            core::mem::swap(&mut rp, &mut (xp as mpi_ptr_t));
            rsize = xsize;
            if (e as mpi_limb_signed_t) < 0 {
                let mut tmp = 0;
                if bsize < KARATSUBA_THRESHOLD {
                    if mpihelp_mul(xp, rp, rsize, bp, bsize, &mut tmp) < 0 { goto cleanup; }
                } else if mpihelp_mul_karatsuba_case(xp, rp, rsize, bp, bsize, &mut karactx) < 0 { goto cleanup; }
                xsize = rsize + bsize;
                if xsize > msize { mpihelp_divrem(xp.add(msize), 0, xp, xsize, mp, msize); xsize = msize; }
                core::mem::swap(&mut rp, &mut (xp as mpi_ptr_t));
                rsize = xsize;
            }
            e <<= 1; c -= 1; cond_resched();
        }
        i -= 1;
        if i < 0 { break; }
        e = ep[i]; c = BITS_PER_MPI_LIMB;
    }

    if mod_shift_cnt != 0 {
        let carry_limb = mpihelp_lshift((*res).d, rp, rsize, mod_shift_cnt);
        rp = (*res).d;
        if carry_limb != 0 { rp[rsize] = carry_limb; rsize += 1; }
    } else { MPN_COPY((*res).d, rp, rsize); rp = (*res).d; }
    if rsize >= msize { mpihelp_divrem(rp.add(msize), 0, rp, rsize, mp, msize); rsize = msize; }
    if mod_shift_cnt != 0 { mpihelp_rshift(rp, rp, rsize, mod_shift_cnt); }
    MPN_NORMALIZE(rp, rsize);
    if negative_result && rsize != 0 {
        if mod_shift_cnt != 0 { mpihelp_rshift(mp, mp, msize, mod_shift_cnt); }
        mpihelp_sub(rp, mp, msize, rp, rsize); rsize = msize; rsign = msign; MPN_NORMALIZE(rp, rsize);
    }
    (*res).nlimbs = rsize; (*res).sign = rsign; rc = 0;

cleanup:
    mpihelp_release_karatsuba_ctx(&mut karactx);
    if assign_rp { mpi_assign_limb_space(res, rp, size); }
    if !mp_marker.is_null() { mpi_free_limb_space(mp_marker); }
    if !bp_marker.is_null() { mpi_free_limb_space(bp_marker); }
    if !ep_marker.is_null() { mpi_free_limb_space(ep_marker); }
    if !xp_marker.is_null() { mpi_free_limb_space(xp_marker); }
    if !tspace.is_null() { mpi_free_limb_space(tspace); }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
