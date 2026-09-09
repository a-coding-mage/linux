// SPDX-License-Identifier: GPL-2.0-only
/*
 * SHA-512 routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/NX implementation.

#[repr(C)]
pub struct Sha512StateBe {
    pub state: [u64; SHA512_DIGEST_SIZE / 8],
    pub count: [u64; 2],
}

unsafe fn nx_crypto_ctx_sha512_init(tfm: *mut crypto_shash) -> i32 {
    let nx_ctx = crypto_shash_ctx(tfm);
    let mut err: i32;

    err = nx_crypto_ctx_sha_init(tfm);
    if err != 0 {
        return err;
    }

    nx_ctx_init(nx_ctx, HCOP_FC_SHA);

    (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_SHA512];

    NX_CPB_SET_DIGEST_SIZE((*nx_ctx).csbcpb, NX_DS_SHA512);

    0
}

unsafe fn nx_sha512_init(desc: *mut shash_desc) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha512StateBe;

    (*sctx).state[0] = __cpu_to_be64(SHA512_H0);
    (*sctx).state[1] = __cpu_to_be64(SHA512_H1);
    (*sctx).state[2] = __cpu_to_be64(SHA512_H2);
    (*sctx).state[3] = __cpu_to_be64(SHA512_H3);
    (*sctx).state[4] = __cpu_to_be64(SHA512_H4);
    (*sctx).state[5] = __cpu_to_be64(SHA512_H5);
    (*sctx).state[6] = __cpu_to_be64(SHA512_H6);
    (*sctx).state[7] = __cpu_to_be64(SHA512_H7);
    (*sctx).count[0] = 0;
    (*sctx).count[1] = 0;

    0
}

unsafe fn nx_sha512_update(desc: *mut shash_desc, mut data: *const u8, len: u32) -> i32 {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let sctx = shash_desc_ctx(desc) as *mut Sha512StateBe;
    let csbcpb = (*nx_ctx).csbcpb as *mut nx_csbcpb;
    let mut to_process: u64;
    let mut leftover: u64;
    let mut total = len as u64;
    let out_sg: *mut nx_sg;
    let irq_flags: ulong;
    let mut rc: i32 = 0;
    let mut data_len: i32;
    let mut max_sg_len: u32;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);

    memcpy((*csbcpb).cpb.sha512.message_digest.as_mut_ptr() as *mut u8,
           (*sctx).state.as_ptr() as *const u8, SHA512_DIGEST_SIZE);
    NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;

    max_sg_len = core::cmp::min((*nx_ctx).ap.sglen as u64,
        nx_driver.of.max_sg_len as u64 / core::mem::size_of::<nx_sg>() as u64) as u32;
    max_sg_len = core::cmp::min(max_sg_len as u64,
        (*nx_ctx).ap.databytelen as u64 / NX_PAGE_SIZE as u64) as u32;

    data_len = SHA512_DIGEST_SIZE as i32;
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, (*sctx).state.as_mut_ptr() as *mut u8,
                              &mut data_len, max_sg_len);
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg as usize - out_sg as usize) as u64 * core::mem::size_of::<nx_sg>() as u64;

    if data_len != SHA512_DIGEST_SIZE as i32 {
        rc = -EINVAL;
        goto_out!(out);
    }

    loop {
        let mut in_sg = (*nx_ctx).in_sg;
        to_process = total & !(SHA512_BLOCK_SIZE as u64 - 1);
        data_len = to_process as i32;
        in_sg = nx_build_sg_list(in_sg, data as *mut u8, &mut data_len, max_sg_len);
        (*nx_ctx).op.inlen = ((*nx_ctx).in_sg as usize - in_sg as usize) as u64 * core::mem::size_of::<nx_sg>() as u64;
        to_process = data_len as u64;
        leftover = total - to_process;

        memcpy((*csbcpb).cpb.sha512.input_partial_digest.as_mut_ptr() as *mut u8,
               (*csbcpb).cpb.sha512.message_digest.as_ptr() as *const u8, SHA512_DIGEST_SIZE);

        if (*nx_ctx).op.inlen == 0 || (*nx_ctx).op.outlen == 0 {
            rc = -EINVAL;
            goto_out!(out);
        }
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
        if rc != 0 { goto_out!(out); }
        atomic_inc(&mut (*(*nx_ctx).stats).sha512_ops);
        total -= to_process;
        data = data.add(to_process as usize);
        (*sctx).count[0] = (*sctx).count[0].wrapping_add(to_process);
        if (*sctx).count[0] < to_process { (*sctx).count[1] = (*sctx).count[1].wrapping_add(1); }
        if leftover < SHA512_BLOCK_SIZE as u64 { break; }
    }
    rc = leftover as i32;
    memcpy((*sctx).state.as_mut_ptr() as *mut u8,
           (*csbcpb).cpb.sha512.message_digest.as_ptr() as *const u8, SHA512_DIGEST_SIZE);
out:
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
    rc
}

unsafe fn nx_sha512_finup(desc: *mut shash_desc, src: *const u8, nbytes: u32,
                          out: *mut u8) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha512StateBe;
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let csbcpb = (*nx_ctx).csbcpb as *mut nx_csbcpb;
    let irq_flags: ulong;
    let mut rc = 0i32;
    let mut len: i32;
    let mut count0 = (*sctx).count[0].wrapping_add(nbytes as u64);
    let count1 = (*sctx).count[1];
    let mut max_sg_len = core::cmp::min((*nx_ctx).ap.sglen as u64,
        nx_driver.of.max_sg_len as u64 / core::mem::size_of::<nx_sg>() as u64) as u32;
    max_sg_len = core::cmp::min(max_sg_len as u64,
        (*nx_ctx).ap.databytelen as u64 / NX_PAGE_SIZE as u64) as u32;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);
    memcpy((*csbcpb).cpb.sha512.input_partial_digest.as_mut_ptr() as *mut u8,
           (*sctx).state.as_ptr() as *const u8, SHA512_DIGEST_SIZE);
    NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
    (*csbcpb).cpb.sha512.message_bit_length_lo = count0 << 3;
    (*csbcpb).cpb.sha512.message_bit_length_hi = (count1 << 3) | (count0 >> 61);

    len = nbytes as i32;
    let in_sg = nx_build_sg_list((*nx_ctx).in_sg, src as *mut u8, &mut len, max_sg_len);
    if len != nbytes as i32 { rc = -EINVAL; goto_out!(out); }
    len = SHA512_DIGEST_SIZE as i32;
    let out_sg = nx_build_sg_list((*nx_ctx).out_sg, out, &mut len, max_sg_len);
    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg as usize - in_sg as usize) as u64 * core::mem::size_of::<nx_sg>() as u64;
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg as usize - out_sg as usize) as u64 * core::mem::size_of::<nx_sg>() as u64;
    if (*nx_ctx).op.outlen == 0 { rc = -EINVAL; goto_out!(out); }
    rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
    if rc != 0 { goto_out!(out); }
    atomic_inc(&mut (*(*nx_ctx).stats).sha512_ops);
    atomic64_add(count0, &mut (*(*nx_ctx).stats).sha512_bytes);
    memcpy(out, (*csbcpb).cpb.sha512.message_digest.as_ptr() as *const u8, SHA512_DIGEST_SIZE);
out:
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
    rc
}

unsafe fn nx_sha512_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha512StateBe;
    let p = out as *mut u64;
    for i in 0..(SHA512_DIGEST_SIZE / core::mem::size_of::<u64>()) {
        put_unaligned(be64_to_cpu((*sctx).state[i]), p.add(i));
    }
    put_unaligned((*sctx).count[0], p.add(8));
    put_unaligned((*sctx).count[1], p.add(9));
    0
}

unsafe fn nx_sha512_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha512StateBe;
    let p = input as *const u64;
    for i in 0..(SHA512_DIGEST_SIZE / core::mem::size_of::<u64>()) {
        (*sctx).state[i] = cpu_to_be64(get_unaligned(p.add(i)));
    }
    (*sctx).count[0] = get_unaligned(p.add(8));
    (*sctx).count[1] = get_unaligned(p.add(9));
    0
}

pub static mut nx_shash_sha512_alg: shash_alg = shash_alg {
    digestsize: SHA512_DIGEST_SIZE,
    init: Some(nx_sha512_init),
    update: Some(nx_sha512_update),
    finup: Some(nx_sha512_finup),
    export: Some(nx_sha512_export),
    import: Some(nx_sha512_import),
    init_tfm: Some(nx_crypto_ctx_sha512_init),
    exit_tfm: Some(nx_crypto_ctx_shash_exit),
    descsize: core::mem::size_of::<Sha512StateBe>(),
    statesize: core::mem::size_of::<Sha512StateBe>(),
    base: shash_alg_base {
        cra_name: b"sha512\0".as_ptr() as *const i8,
        cra_driver_name: b"sha512-nx\0".as_ptr() as *const i8,
        cra_priority: 300,
        cra_flags: CRYPTO_AHASH_ALG_BLOCK_ONLY,
        cra_blocksize: SHA512_BLOCK_SIZE,
        cra_module: THIS_MODULE,
        cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
