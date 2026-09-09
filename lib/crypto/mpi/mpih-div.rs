// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-div.c - MPI helper functions (faithful Rust translation). */

// The following types, constants, macros, and helper functions are supplied by
// the surrounding MPI implementation: mpi_limb_t, mpi_ptr_t, mpi_size_t,
// BITS_PER_MPI_LIMB, UMUL_TIME, UDIV_TIME, UDIV_NEEDS_NORMALIZATION,
// count_leading_zeros, udiv_qrnnd, UDIV_QRNND_PREINV, sub_ddmmss,
// add_ssaaaa, umul_ppmm, mpihelp_cmp, mpihelp_sub_n, MPN_COPY_DECR,
// mpihelp_submul_1, and mpihelp_add_n.

pub unsafe fn mpihelp_mod_1(
    dividend_ptr: mpi_ptr_t,
    dividend_size: mpi_size_t,
    mut divisor_limb: mpi_limb_t,
) -> mpi_limb_t {
    let mut i: isize;
    let mut n1: mpi_limb_t;
    let mut n0: mpi_limb_t;
    let mut r: mpi_limb_t;
    let mut dummy: mpi_limb_t = 0;
    if dividend_size == 0 { return 0; }

    if UDIV_TIME > (2 * UMUL_TIME + 6)
        && (UDIV_TIME - (2 * UMUL_TIME + 6)) * dividend_size > UDIV_TIME
    {
        let normalization_steps: i32 = count_leading_zeros(divisor_limb);
        if normalization_steps != 0 {
            let divisor_limb_inverted: mpi_limb_t;
            divisor_limb <<= normalization_steps;
            if (divisor_limb << 1) == 0 {
                divisor_limb_inverted = !0;
            } else {
                udiv_qrnnd!(divisor_limb_inverted, dummy, -divisor_limb, 0, divisor_limb);
            }
            n1 = *dividend_ptr.add(dividend_size - 1);
            r = n1 >> (BITS_PER_MPI_LIMB - normalization_steps);
            i = dividend_size as isize - 2;
            while i >= 0 {
                n0 = *dividend_ptr.add(i as usize);
                UDIV_QRNND_PREINV!(dummy, r, r,
                    (n1 << normalization_steps) |
                    (n0 >> (BITS_PER_MPI_LIMB - normalization_steps)),
                    divisor_limb, divisor_limb_inverted);
                n1 = n0; i -= 1;
            }
            UDIV_QRNND_PREINV!(dummy, r, r, n1 << normalization_steps,
                divisor_limb, divisor_limb_inverted);
            return r >> normalization_steps;
        } else {
            let divisor_limb_inverted: mpi_limb_t;
            if (divisor_limb << 1) == 0 { divisor_limb_inverted = !0; }
            else { udiv_qrnnd!(divisor_limb_inverted, dummy, -divisor_limb, 0, divisor_limb); }
            i = dividend_size as isize - 1; r = *dividend_ptr.add(i as usize);
            if r >= divisor_limb { r = 0; } else { i -= 1; }
            while i >= 0 {
                n0 = *dividend_ptr.add(i as usize);
                UDIV_QRNND_PREINV!(dummy, r, r, n0, divisor_limb, divisor_limb_inverted);
                i -= 1;
            }
            return r;
        }
    } else {
        if UDIV_NEEDS_NORMALIZATION {
            let normalization_steps: i32 = count_leading_zeros(divisor_limb);
            if normalization_steps != 0 {
                divisor_limb <<= normalization_steps;
                n1 = *dividend_ptr.add(dividend_size - 1);
                r = n1 >> (BITS_PER_MPI_LIMB - normalization_steps);
                i = dividend_size as isize - 2;
                while i >= 0 {
                    n0 = *dividend_ptr.add(i as usize);
                    udiv_qrnnd!(dummy, r, r, (n1 << normalization_steps) |
                        (n0 >> (BITS_PER_MPI_LIMB - normalization_steps)), divisor_limb);
                    n1 = n0; i -= 1;
                }
                udiv_qrnnd!(dummy, r, r, n1 << normalization_steps, divisor_limb);
                return r >> normalization_steps;
            }
        }
        i = dividend_size as isize - 1; r = *dividend_ptr.add(i as usize);
        if r >= divisor_limb { r = 0; } else { i -= 1; }
        while i >= 0 { n0 = *dividend_ptr.add(i as usize); udiv_qrnnd!(dummy, r, r, n0, divisor_limb); i -= 1; }
        r
    }
}

pub unsafe fn mpihelp_divrem(
    mut qp: mpi_ptr_t, qextra_limbs: mpi_size_t, mut np: mpi_ptr_t,
    nsize: mpi_size_t, dp: mpi_ptr_t, dsize: mpi_size_t,
) -> mpi_limb_t {
    let mut most_significant_q_limb: mpi_limb_t = 0;
    match dsize {
        0 => return 1 / dsize,
        1 => {
            let d = *dp; let mut n1 = *np.add(nsize - 1);
            if n1 >= d { n1 -= d; most_significant_q_limb = 1; }
            qp = qp.add(qextra_limbs);
            let mut i = nsize as isize - 2;
            while i >= 0 { udiv_qrnnd!(*qp.add(i as usize), n1, n1, *np.add(i as usize), d); i -= 1; }
            qp = qp.sub(qextra_limbs);
            i = qextra_limbs as isize - 1;
            while i >= 0 { udiv_qrnnd!(*qp.add(i as usize), n1, n1, 0, d); i -= 1; }
            *np = n1;
        }
        2 => {
            np = np.add(nsize - 2); let d1 = *dp.add(1); let d0 = *dp;
            let mut n1 = *np.add(1); let mut n0 = *np;
            if n1 >= d1 && (n1 > d1 || n0 >= d0) { sub_ddmmss!(n1,n0,n1,n0,d1,d0); most_significant_q_limb=1; }
            let mut i = (qextra_limbs + nsize - 3) as isize;
            while i >= 0 {
                let mut q; let mut r;
                if i as usize >= qextra_limbs { np = np.sub(1); } else { *np = 0; }
                if n1 == d1 {
                    q = !0; r = n0 + d1;
                    if r < d1 { add_ssaaaa!(n1,n0,r - d0,*np,0,d0); *qp.add(i as usize)=q; i-=1; continue; }
                    n1 = d0 - if d0 != 0 { 1 } else { 0 }; n0 = -d0;
                } else { udiv_qrnnd!(q,r,n1,n0,d1); umul_ppmm!(n1,n0,d0,q); }
                let n2 = *np;
                while n1 > r || (n1 == r && n0 > n2) { q-=1; sub_ddmmss!(n1,n0,n1,n0,0,d0); r += d1; if r < d1 { break; } }
                *qp.add(i as usize)=q; sub_ddmmss!(n1,n0,r,n2,n1,n0); i-=1;
            }
            *np.add(1)=n1; *np=n0;
        }
        _ => {
            np = np.add(nsize - dsize); let d_x=*dp.add(dsize-1); let d1=*dp.add(dsize-2); let mut n0=*np.add(dsize-1);
            if n0 >= d_x && (n0 > d_x || mpihelp_cmp(np,dp,dsize-1)>=0) { mpihelp_sub_n(np,np,dp,dsize); n0=*np.add(dsize-1); most_significant_q_limb=1; }
            let mut i=(qextra_limbs+nsize-dsize-1) as isize;
            while i>=0 { let q; let n2; if i as usize>=qextra_limbs { np=np.sub(1); n2=*np.add(dsize); } else { n2=*np.add(dsize-1); MPN_COPY_DECR!(np.add(1),np,dsize-1); *np=0; }
                let mut qv; if n0==d_x { qv=!0; } else { let mut r; udiv_qrnnd!(qv,r,n0,*np.add(dsize-1),d_x); umul_ppmm!(n0 /* n1 */,n0,d1,qv); while n0>r || (n0==r && n0>*np.add(dsize-2)) { qv-=1; r+=d_x; if r<d_x { break; } } }
                let cy=mpihelp_submul_1(np,dp,dsize,qv); if n2!=cy { mpihelp_add_n(np,np,dp,dsize); qv-=1; } *qp.add(i as usize)=qv; n0=*np.add(dsize-1); i-=1; }
        }
    } most_significant_q_limb
}

pub unsafe fn mpihelp_divmod_1(mut quot_ptr: mpi_ptr_t, dividend_ptr: mpi_ptr_t, dividend_size: mpi_size_t, mut divisor_limb: mpi_limb_t) -> mpi_limb_t {
    let mut r: mpi_limb_t = 0; let mut dummy: mpi_limb_t = 0;
    if dividend_size == 0 { return 0; }
    let mut i=dividend_size as isize-1; r=*dividend_ptr.add(i as usize);
    if r>=divisor_limb { r=0; } else { *quot_ptr.add(i as usize)=0; i-=1; }
    while i>=0 { udiv_qrnnd!(*quot_ptr.add(i as usize),r,r,*dividend_ptr.add(i as usize),divisor_limb); i-=1; }
    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
