// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-mul.c - MPI helper functions. */

// C dependencies: mpi-internal.h, longlong.h, and Linux errno/allocation APIs.

type mpi_limb_t = u64;
type mpi_size_t = usize;
type mpi_ptr_t = *mut mpi_limb_t;

const KARATSUBA_THRESHOLD: mpi_size_t = 32;
const ENOMEM: i32 = 12;

#[repr(C)]
pub struct karatsuba_ctx {
    pub tp: mpi_ptr_t,
    pub tp_size: mpi_size_t,
    pub tspace: mpi_ptr_t,
    pub tspace_size: mpi_size_t,
    pub next: *mut karatsuba_ctx,
}

extern "C" {
    fn mpihelp_mul_1(p: mpi_ptr_t, s: mpi_ptr_t, n: mpi_size_t, v: mpi_limb_t) -> mpi_limb_t;
    fn mpihelp_add_n(r: mpi_ptr_t, a: mpi_ptr_t, b: mpi_ptr_t, n: mpi_size_t) -> mpi_limb_t;
    fn mpihelp_addmul_1(r: mpi_ptr_t, s: mpi_ptr_t, n: mpi_size_t, v: mpi_limb_t) -> mpi_limb_t;
    fn mpihelp_add_1(r: mpi_ptr_t, a: mpi_ptr_t, n: mpi_size_t, v: mpi_limb_t) -> mpi_limb_t;
    fn mpihelp_sub_n(r: mpi_ptr_t, a: mpi_ptr_t, b: mpi_ptr_t, n: mpi_size_t) -> mpi_limb_t;
    fn mpihelp_cmp(a: mpi_ptr_t, b: mpi_ptr_t, n: mpi_size_t) -> i32;
    fn mpihelp_mul(r: mpi_ptr_t, u: mpi_ptr_t, usize: mpi_size_t, v: mpi_ptr_t,
                   vsize: mpi_size_t, result: *mut mpi_limb_t) -> i32;
    fn mpi_alloc_limb_space(n: mpi_size_t) -> mpi_ptr_t;
    fn mpi_free_limb_space(p: mpi_ptr_t);
    fn kzalloc_obj(ctx: karatsuba_ctx) -> *mut karatsuba_ctx;
    fn kfree(ctx: *mut karatsuba_ctx);
}

unsafe fn mpn_copy(d: mpi_ptr_t, s: mpi_ptr_t, n: mpi_size_t) {
    std::ptr::copy(s, d, n);
}
unsafe fn mpn_zero(d: mpi_ptr_t, n: mpi_size_t) {
    std::ptr::write_bytes(d, 0, n);
}

unsafe fn mul_n_basecase(mut prodp: mpi_ptr_t, up: mpi_ptr_t, vp: mpi_ptr_t, size: mpi_size_t) -> mpi_limb_t {
    let mut cy;
    let mut v_limb = *vp;
    if v_limb <= 1 { if v_limb == 1 { mpn_copy(prodp, up, size); } else { mpn_zero(prodp, size); } cy = 0; }
    else { cy = mpihelp_mul_1(prodp, up, size, v_limb); }
    *prodp.add(size) = cy; prodp = prodp.add(1);
    for i in 1..size { v_limb = *vp.add(i); if v_limb <= 1 { cy = 0; if v_limb == 1 { cy = mpihelp_add_n(prodp, prodp, up, size); } } else { cy = mpihelp_addmul_1(prodp, up, size, v_limb); } *prodp.add(size) = cy; prodp = prodp.add(1); }
    cy
}

unsafe fn mul_n(prodp: mpi_ptr_t, up: mpi_ptr_t, vp: mpi_ptr_t, size: mpi_size_t, tspace: mpi_ptr_t) {
    if size & 1 != 0 { let esize = size - 1; let mut cy = if esize < KARATSUBA_THRESHOLD { mul_n_basecase(prodp, up, vp, esize) } else { mul_n(prodp, up, vp, esize, tspace); 0 }; cy = mpihelp_addmul_1(prodp.add(esize), up, esize, *vp.add(esize)); *prodp.add(esize + esize) = cy; cy = mpihelp_addmul_1(prodp.add(esize), vp, size, *up.add(esize)); *prodp.add(esize + size) = cy; return; }
    let h = size >> 1; let mut cy; let mut neg = 0;
    if h < KARATSUBA_THRESHOLD { mul_n_basecase(prodp.add(size), up.add(h), vp.add(h), h); } else { mul_n(prodp.add(size), up.add(h), vp.add(h), h, tspace); }
    if mpihelp_cmp(up.add(h), up, h) >= 0 { mpihelp_sub_n(prodp, up.add(h), up, h); neg = 0; } else { mpihelp_sub_n(prodp, up, up.add(h), h); neg = 1; }
    if mpihelp_cmp(vp.add(h), vp, h) >= 0 { mpihelp_sub_n(prodp.add(h), vp.add(h), vp, h); neg ^= 1; } else { mpihelp_sub_n(prodp.add(h), vp, vp.add(h), h); }
    if h < KARATSUBA_THRESHOLD { mul_n_basecase(tspace, prodp, prodp.add(h), h); } else { mul_n(tspace, prodp, prodp.add(h), h, tspace.add(size)); }
    mpn_copy(prodp.add(h), prodp.add(size), h); cy = mpihelp_add_n(prodp.add(size), prodp.add(size), prodp.add(size + h), h);
    if neg != 0 { cy = cy.wrapping_sub(mpihelp_sub_n(prodp.add(h), prodp.add(h), tspace, size)); } else { cy = cy.wrapping_add(mpihelp_add_n(prodp.add(h), prodp.add(h), tspace, size)); }
    if h < KARATSUBA_THRESHOLD { mul_n_basecase(tspace, up, vp, h); } else { mul_n(tspace, up, vp, h, tspace.add(size)); }
    cy = cy.wrapping_add(mpihelp_add_n(prodp.add(h), prodp.add(h), tspace, size)); if cy != 0 { mpihelp_add_1(prodp.add(h + size), prodp.add(h + size), h, cy); }
    mpn_copy(prodp, tspace, h); cy = mpihelp_add_n(prodp.add(h), prodp.add(h), tspace.add(h), h); if cy != 0 { mpihelp_add_1(prodp.add(size), prodp.add(size), size, 1); }
}

pub unsafe fn mpih_sqr_n_basecase(mut prodp: mpi_ptr_t, up: mpi_ptr_t, size: mpi_size_t) { mul_n_basecase(prodp, up, up, size); }
pub unsafe fn mpih_sqr_n(prodp: mpi_ptr_t, up: mpi_ptr_t, size: mpi_size_t, tspace: mpi_ptr_t) { mul_n(prodp, up, up, size, tspace); }

pub unsafe fn mpihelp_mul_karatsuba_case(mut prodp: mpi_ptr_t, mut up: mpi_ptr_t, mut usize: mpi_size_t, vp: mpi_ptr_t, vsize: mpi_size_t, ctx: *mut karatsuba_ctx) -> i32 {
    let mut cy;
    if (*ctx).tspace.is_null() || (*ctx).tspace_size < vsize {
        if !(*ctx).tspace.is_null() { mpi_free_limb_space((*ctx).tspace); }
        (*ctx).tspace = mpi_alloc_limb_space(2 * vsize);
        if (*ctx).tspace.is_null() { return -ENOMEM; }
        (*ctx).tspace_size = vsize;
    }
    if vsize < KARATSUBA_THRESHOLD { mul_n_basecase(prodp, up, vp, vsize); } else { mul_n(prodp, up, vp, vsize, (*ctx).tspace); }
    prodp = prodp.add(vsize); up = up.add(vsize); usize -= vsize;
    if usize >= vsize {
        if (*ctx).tp.is_null() || (*ctx).tp_size < vsize {
            if !(*ctx).tp.is_null() { mpi_free_limb_space((*ctx).tp); }
            (*ctx).tp = mpi_alloc_limb_space(2 * vsize);
            if (*ctx).tp.is_null() { mpi_free_limb_space((*ctx).tspace); (*ctx).tspace = std::ptr::null_mut(); return -ENOMEM; }
            (*ctx).tp_size = vsize;
        }
        while usize >= vsize {
            if vsize < KARATSUBA_THRESHOLD { mul_n_basecase((*ctx).tp, up, vp, vsize); } else { mul_n((*ctx).tp, up, vp, vsize, (*ctx).tspace); }
            cy = mpihelp_add_n(prodp, prodp, (*ctx).tp, vsize); mpihelp_add_1(prodp.add(vsize), (*ctx).tp.add(vsize), vsize, cy);
            prodp = prodp.add(vsize); up = up.add(vsize); usize -= vsize;
        }
    }
    if usize != 0 {
        if usize < KARATSUBA_THRESHOLD { let mut tmp = 0; if mpihelp_mul((*ctx).tspace, vp, vsize, up, usize, &mut tmp) < 0 { return -ENOMEM; } }
        else {
            if (*ctx).next.is_null() { (*ctx).next = kzalloc_obj(std::ptr::read(ctx)); if (*ctx).next.is_null() { return -ENOMEM; } }
            if mpihelp_mul_karatsuba_case((*ctx).tspace, vp, vsize, up, usize, (*ctx).next) < 0 { return -ENOMEM; }
        }
        cy = mpihelp_add_n(prodp, prodp, (*ctx).tspace, vsize); mpihelp_add_1(prodp.add(vsize), (*ctx).tspace.add(vsize), usize, cy);
    }
    0
}

pub unsafe fn mpihelp_release_karatsuba_ctx(mut ctx: *mut karatsuba_ctx) {
    if !(*ctx).tp.is_null() { mpi_free_limb_space((*ctx).tp); }
    if !(*ctx).tspace.is_null() { mpi_free_limb_space((*ctx).tspace); }
    while !ctx.is_null() { let next = (*ctx).next; if !(*ctx).tp.is_null() { mpi_free_limb_space((*ctx).tp); } if !(*ctx).tspace.is_null() { mpi_free_limb_space((*ctx).tspace); } kfree(ctx); ctx = next; }
}

pub unsafe fn mpihelp_mul(mut prodp: mpi_ptr_t, mut up: mpi_ptr_t, usize: mpi_size_t, vp: mpi_ptr_t, vsize: mpi_size_t, result: *mut mpi_limb_t) -> i32 {
    if vsize < KARATSUBA_THRESHOLD {
        if vsize == 0 { *result = 0; return 0; }
        let mut cy; let mut v = *vp;
        if v <= 1 { if v == 1 { mpn_copy(prodp, up, usize); } else { mpn_zero(prodp, usize); } cy = 0; } else { cy = mpihelp_mul_1(prodp, up, usize, v); }
        *prodp.add(usize) = cy; prodp = prodp.add(1);
        for i in 1..vsize { v = *vp.add(i); if v <= 1 { cy = 0; if v == 1 { cy = mpihelp_add_n(prodp, prodp, up, usize); } } else { cy = mpihelp_addmul_1(prodp, up, usize, v); } *prodp.add(usize) = cy; prodp = prodp.add(1); }
        *result = cy; return 0;
    }
    let mut ctx: karatsuba_ctx = std::mem::zeroed();
    if mpihelp_mul_karatsuba_case(prodp, up, usize, vp, vsize, &mut ctx) < 0 { return -ENOMEM; }
    mpihelp_release_karatsuba_ctx(&mut ctx); *result = *prodp.add(usize + vsize - 1); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
