// SPDX-License-Identifier: GPL-2.0-or-later
/* mpi-pow.c  -  MPI functions
 *	Copyright (C) 1994, 1996, 1998, 2000 Free Software Foundation, Inc.
 *
 * This file is part of GnuPG.
 *
 * Note: This code is heavily based on the GNU MP Library.
 *	 Actually it's the same code with only minor changes in the
 *	 way the data is stored; this is to support the abstraction
 *	 of an optional secure memory allocation which may be used
 *	 to avoid revealing of sensitive data due to paging etc.
 *	 The GNU MP Library itself is published under the LGPL;
 *	 however I decided to publish this code under the plain GPL.
 */

// Dependencies: linux/export.h, linux/sched.h, linux/string.h,
// "mpi-internal.h", "longlong.h".

use core::ffi::c_int;
use core::ptr::{addr_of_mut, null_mut};

/****************
 * RES = BASE ^ EXP mod MOD
 */
#[no_mangle]
pub unsafe extern "C" fn mpi_powm(res: MPI, base: MPI, exp: MPI, r#mod: MPI) -> c_int {
    let mut mp_marker: mpi_ptr_t = null_mut();
    let mut bp_marker: mpi_ptr_t = null_mut();
    let mut ep_marker: mpi_ptr_t = null_mut();
    // SAFETY: `struct karatsuba_ctx karactx = {}` is all-zero in C.
    let mut karactx: karatsuba_ctx = core::mem::zeroed();
    let mut xp_marker: mpi_ptr_t = null_mut();
    let mut tspace: mpi_ptr_t = null_mut();
    let mut rp: mpi_ptr_t;
    let mut ep: mpi_ptr_t;
    let mut mp: mpi_ptr_t;
    let mut bp: mpi_ptr_t;
    let mut bsize: mpi_size_t;
    let mut rsize: mpi_size_t = 0;
    let mut rsign: c_int = 0;
    let mut mod_shift_cnt: c_int = 0;
    let mut negative_result: bool = false;
    let mut assign_rp = false;
    let mut tsize: mpi_size_t = 0; /* to avoid compiler warning */
    /* fixme: we should check that the warning is void */
    let mut rc: c_int = -ENOMEM;

    let esize: mpi_size_t = (*exp).nlimbs as mpi_size_t;
    let msize: mpi_size_t = (*r#mod).nlimbs as mpi_size_t;
    let size: mpi_size_t = 2 * msize;
    let msign: c_int = (*r#mod).sign;

    rp = (*res).d;
    ep = (*exp).d;

    if msize == 0 {
        return -EINVAL;
    }

    'enomem: {
        'leave: {
            if esize == 0 {
                /* Exponent is zero, result is 1 mod MOD, i.e., 1 or 0
                 * depending on if MOD equals 1.  */
                (*res).nlimbs = if msize == 1 && *(*r#mod).d == 1 { 0 } else { 1 };
                if (*res).nlimbs != 0 {
                    if mpi_resize(res, 1) < 0 {
                        break 'enomem;
                    }
                    rp = (*res).d;
                    *rp = 1;
                }
                (*res).sign = 0;
                break 'leave;
            }

            /* Normalize MOD (i.e. make its most significant bit set) as required by
             * mpn_divrem.  This will make the intermediate values in the calculation
             * slightly larger, but the correct result is obtained after a final
             * reduction using the original MOD value.  */
            mp_marker = mpi_alloc_limb_space(msize as _);
            mp = mp_marker;
            if mp.is_null() {
                break 'enomem;
            }
            mod_shift_cnt = count_leading_zeros(*(*r#mod).d.add((msize - 1) as usize)) as c_int;
            if mod_shift_cnt != 0 {
                mpihelp_lshift(mp, (*r#mod).d, msize, mod_shift_cnt as _);
            } else {
                MPN_COPY!(mp, (*r#mod).d, msize);
            }

            bsize = (*base).nlimbs as mpi_size_t;
            let bsign: c_int = (*base).sign;
            if bsize > msize {
                /* The base is larger than the module. Reduce it. */
                /* Allocate (BSIZE + 1) with space for remainder and quotient.
                 * (The quotient is (bsize - msize + 1) limbs.)  */
                bp_marker = mpi_alloc_limb_space((bsize + 1) as _);
                bp = bp_marker;
                if bp.is_null() {
                    break 'enomem;
                }
                MPN_COPY!(bp, (*base).d, bsize);
                /* We don't care about the quotient, store it above the remainder,
                 * at BP + MSIZE.  */
                mpihelp_divrem(bp.add(msize as usize), 0, bp, bsize, mp, msize);
                bsize = msize;
                /* Canonicalize the base, since we are going to multiply with it
                 * quite a few times.  */
                MPN_NORMALIZE!(bp, bsize);
            } else {
                bp = (*base).d;
            }

            if bsize == 0 {
                (*res).nlimbs = 0;
                (*res).sign = 0;
                break 'leave;
            }

            if ((*res).alloced as mpi_size_t) < size {
                /* We have to allocate more space for RES.  If any of the input
                 * parameters are identical to RES, defer deallocation of the old
                 * space.  */
                if rp == ep || rp == mp || rp == bp {
                    rp = mpi_alloc_limb_space(size as _);
                    if rp.is_null() {
                        break 'enomem;
                    }
                    assign_rp = true;
                } else {
                    if mpi_resize(res, size as _) < 0 {
                        break 'enomem;
                    }
                    rp = (*res).d;
                }
            } else {
                /* Make BASE, EXP and MOD not overlap with RES.  */
                if rp == bp {
                    /* RES and BASE are identical.  Allocate temp. space for BASE.  */
                    BUG_ON(!bp_marker.is_null());
                    bp_marker = mpi_alloc_limb_space(bsize as _);
                    bp = bp_marker;
                    if bp.is_null() {
                        break 'enomem;
                    }
                    MPN_COPY!(bp, rp, bsize);
                }
                if rp == ep {
                    /* RES and EXP are identical.  Allocate temp. space for EXP.  */
                    ep_marker = mpi_alloc_limb_space(esize as _);
                    ep = ep_marker;
                    if ep.is_null() {
                        break 'enomem;
                    }
                    MPN_COPY!(ep, rp, esize);
                }
                if rp == mp {
                    /* RES and MOD are identical.  Allocate temporary space for MOD. */
                    BUG_ON(!mp_marker.is_null());
                    mp_marker = mpi_alloc_limb_space(msize as _);
                    mp = mp_marker;
                    if mp.is_null() {
                        break 'enomem;
                    }
                    MPN_COPY!(mp, rp, msize);
                }
            }

            MPN_COPY!(rp, bp, bsize);
            rsize = bsize;
            rsign = bsign;

            {
                let mut i: mpi_size_t;
                let mut xp: mpi_ptr_t;
                let mut c: c_int;
                let mut e: mpi_limb_t;
                let carry_limb: mpi_limb_t;

                xp_marker = mpi_alloc_limb_space((2 * (msize + 1)) as _);
                xp = xp_marker;
                if xp.is_null() {
                    break 'enomem;
                }

                negative_result = (*ep & 1) != 0 && (*base).sign != 0;

                i = esize - 1;
                e = *ep.add(i as usize);
                c = count_leading_zeros(e) as c_int;
                e = (e << c) << 1; /* shift the exp bits to the left, lose msb */
                c = BITS_PER_MPI_LIMB as c_int - 1 - c;

                /* Main loop.
                 *
                 * Make the result be pointed to alternately by XP and RP.  This
                 * helps us avoid block copying, which would otherwise be necessary
                 * with the overlap restrictions of mpihelp_divmod. With 50% probability
                 * the result after this loop will be in the area originally pointed
                 * by RP (==RES->d), and with 50% probability in the area originally
                 * pointed to by XP.
                 */
                loop {
                    while c != 0 {
                        let mut xsize: mpi_size_t;

                        /*if (mpihelp_mul_n(xp, rp, rp, rsize) < 0) goto enomem */
                        if rsize < KARATSUBA_THRESHOLD as mpi_size_t {
                            mpih_sqr_n_basecase(xp, rp, rsize);
                        } else {
                            if tspace.is_null() {
                                tsize = 2 * rsize;
                                tspace = mpi_alloc_limb_space(tsize as _);
                                if tspace.is_null() {
                                    break 'enomem;
                                }
                            } else if tsize < 2 * rsize {
                                mpi_free_limb_space(tspace);
                                tsize = 2 * rsize;
                                tspace = mpi_alloc_limb_space(tsize as _);
                                if tspace.is_null() {
                                    break 'enomem;
                                }
                            }
                            mpih_sqr_n(xp, rp, rsize, tspace);
                        }

                        xsize = 2 * rsize;
                        if xsize > msize {
                            mpihelp_divrem(xp.add(msize as usize), 0, xp, xsize, mp, msize);
                            xsize = msize;
                        }

                        core::mem::swap(&mut rp, &mut xp);
                        rsize = xsize;

                        if (e as mpi_limb_signed_t) < 0 {
                            /*mpihelp_mul( xp, rp, rsize, bp, bsize ); */
                            if bsize < KARATSUBA_THRESHOLD as mpi_size_t {
                                let mut tmp: mpi_limb_t = 0;
                                if mpihelp_mul(xp, rp, rsize, bp, bsize, &mut tmp) < 0 {
                                    break 'enomem;
                                }
                            } else if mpihelp_mul_karatsuba_case(xp, rp, rsize, bp, bsize, &mut karactx) < 0 {
                                break 'enomem;
                            }

                            xsize = rsize + bsize;
                            if xsize > msize {
                                mpihelp_divrem(xp.add(msize as usize), 0, xp, xsize, mp, msize);
                                xsize = msize;
                            }

                            core::mem::swap(&mut rp, &mut xp);
                            rsize = xsize;
                        }
                        e <<= 1;
                        c -= 1;
                        cond_resched();
                    }

                    i -= 1;
                    if i < 0 {
                        break;
                    }
                    e = *ep.add(i as usize);
                    c = BITS_PER_MPI_LIMB as c_int;
                }

                /* We shifted MOD, the modulo reduction argument, left MOD_SHIFT_CNT
                 * steps.  Adjust the result by reducing it with the original MOD.
                 *
                 * Also make sure the result is put in RES->d (where it already
                 * might be, see above).
                 */
                if mod_shift_cnt != 0 {
                    carry_limb = mpihelp_lshift((*res).d, rp, rsize, mod_shift_cnt as _);
                    rp = (*res).d;
                    if carry_limb != 0 {
                        *rp.add(rsize as usize) = carry_limb;
                        rsize += 1;
                    }
                } else {
                    MPN_COPY!((*res).d, rp, rsize);
                    rp = (*res).d;
                }

                if rsize >= msize {
                    mpihelp_divrem(rp.add(msize as usize), 0, rp, rsize, mp, msize);
                    rsize = msize;
                }

                /* Remove any leading zero words from the result.  */
                if mod_shift_cnt != 0 {
                    mpihelp_rshift(rp, rp, rsize, mod_shift_cnt as _);
                }
                MPN_NORMALIZE!(rp, rsize);
            }

            if negative_result && rsize != 0 {
                if mod_shift_cnt != 0 {
                    mpihelp_rshift(mp, mp, msize, mod_shift_cnt as _);
                }
                mpihelp_sub(rp, mp, msize, rp, rsize);
                rsize = msize;
                rsign = msign;
                MPN_NORMALIZE!(rp, rsize);
            }
            (*res).nlimbs = rsize as _;
            (*res).sign = rsign;
        }
        // leave:
        rc = 0;
    }
    // enomem:
    mpihelp_release_karatsuba_ctx(addr_of_mut!(karactx));
    if assign_rp {
        mpi_assign_limb_space(res, rp, size as _);
    }
    if !mp_marker.is_null() {
        mpi_free_limb_space(mp_marker);
    }
    if !bp_marker.is_null() {
        mpi_free_limb_space(bp_marker);
    }
    if !ep_marker.is_null() {
        mpi_free_limb_space(ep_marker);
    }
    if !xp_marker.is_null() {
        mpi_free_limb_space(xp_marker);
    }
    if !tspace.is_null() {
        mpi_free_limb_space(tspace);
    }
    rc
}
// EXPORT_SYMBOL_GPL(mpi_powm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
