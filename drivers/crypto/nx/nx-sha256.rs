// SPDX-License-Identifier: GPL-2.0-only
/*
 * SHA-256 routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/NX implementation are
// intentionally referenced but not reimplemented here.

#[repr(C)]
pub struct Sha256StateBe {
    pub state: [u32; SHA256_DIGEST_SIZE / 4],
    pub count: u64,
}

unsafe fn nx_crypto_ctx_sha256_init(tfm: *mut crypto_shash) -> i32 {
    let nx_ctx = crypto_shash_ctx(tfm);
    let mut err: i32;

    err = nx_crypto_ctx_sha_init(tfm);
    if err != 0 {
        return err;
    }

    nx_ctx_init(nx_ctx, HCOP_FC_SHA);

    (*nx_ctx).ap = &mut (*nx_ctx).props[NX_PROPS_SHA256];

    NX_CPB_SET_DIGEST_SIZE((*nx_ctx).csbcpb, NX_DS_SHA256);

    0
}

unsafe fn nx_sha256_init(desc: *mut shash_desc) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha256StateBe;

    (*sctx).state[0] = __cpu_to_be32(SHA256_H0);
    (*sctx).state[1] = __cpu_to_be32(SHA256_H1);
    (*sctx).state[2] = __cpu_to_be32(SHA256_H2);
    (*sctx).state[3] = __cpu_to_be32(SHA256_H3);
    (*sctx).state[4] = __cpu_to_be32(SHA256_H4);
    (*sctx).state[5] = __cpu_to_be32(SHA256_H5);
    (*sctx).state[6] = __cpu_to_be32(SHA256_H6);
    (*sctx).state[7] = __cpu_to_be32(SHA256_H7);
    (*sctx).count = 0;

    0
}

unsafe fn nx_sha256_update(
    desc: *mut shash_desc,
    mut data: *const u8,
    len: u32,
) -> i32 {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let sctx = shash_desc_ctx(desc) as *mut Sha256StateBe;
    let csbcpb = (*nx_ctx).csbcpb as *mut nx_csbcpb;
    let mut to_process: u64;
    let mut leftover: u64;
    let mut total = len as u64;
    let mut out_sg: *mut nx_sg;
    let mut irq_flags: usize = 0;
    let mut rc: i32 = 0;
    let mut data_len: i32;
    let max_sg_len: u32;

    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);

    core::ptr::copy_nonoverlapping(
        (*sctx).state.as_ptr() as *const u8,
        (*csbcpb).cpb.sha256.message_digest.as_mut_ptr(),
        SHA256_DIGEST_SIZE,
    );
    NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;

    max_sg_len = core::cmp::min((*nx_ctx).ap.sglen as u64,
        nx_driver.of.max_sg_len as u64 / core::mem::size_of::<nx_sg>() as u64) as u32;
    let max_sg_len = core::cmp::min(max_sg_len as u64,
        (*nx_ctx).ap.databytelen as u64 / NX_PAGE_SIZE as u64) as u32;

    data_len = SHA256_DIGEST_SIZE as i32;
    out_sg = nx_build_sg_list((*nx_ctx).out_sg, (*sctx).state.as_mut_ptr() as *mut u8,
        &mut data_len, max_sg_len);
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize)
        * core::mem::size_of::<nx_sg>();

    if data_len != SHA256_DIGEST_SIZE as i32 {
        rc = -EINVAL;
        goto_out(&mut rc, nx_ctx, irq_flags);
        return rc;
    }

    loop {
        let mut in_sg = (*nx_ctx).in_sg;
        to_process = total & !(SHA256_BLOCK_SIZE as u64 - 1);
        data_len = to_process as i32;
        in_sg = nx_build_sg_list(in_sg, data as *mut u8, &mut data_len, max_sg_len);
        (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize)
            * core::mem::size_of::<nx_sg>();
        to_process = data_len as u64;
        leftover = total - to_process;

        core::ptr::copy_nonoverlapping(
            (*csbcpb).cpb.sha256.message_digest.as_ptr(),
            (*csbcpb).cpb.sha256.input_partial_digest.as_mut_ptr(),
            SHA256_DIGEST_SIZE,
        );

        if (*nx_ctx).op.inlen == 0 || (*nx_ctx).op.outlen == 0 {
            rc = -EINVAL;
            goto_out(&mut rc, nx_ctx, irq_flags);
            return rc;
        }
        rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
        if rc != 0 {
            goto_out(&mut rc, nx_ctx, irq_flags);
            return rc;
        }
        atomic_inc(&mut (*(*nx_ctx).stats).sha256_ops);
        total -= to_process;
        data = data.add(to_process as usize);
        (*sctx).count = (*sctx).count.wrapping_add(to_process);
        if leftover < SHA256_BLOCK_SIZE as u64 { break; }
    }

    rc = leftover as i32;
    core::ptr::copy_nonoverlapping((*csbcpb).cpb.sha256.message_digest.as_ptr(),
        (*sctx).state.as_mut_ptr() as *mut u8, SHA256_DIGEST_SIZE);
    goto_out(&mut rc, nx_ctx, irq_flags);
    rc
}

unsafe fn goto_out(rc: &mut i32, nx_ctx: *mut nx_crypto_ctx, irq_flags: usize) {
    let _ = rc;
    spin_unlock_irqrestore(&mut (*nx_ctx).lock, irq_flags);
}

unsafe fn nx_sha256_finup(desc: *mut shash_desc, src: *const u8, nbytes: u32, out: *mut u8) -> i32 {
    let nx_ctx = crypto_shash_ctx((*desc).tfm);
    let sctx = shash_desc_ctx(desc) as *mut Sha256StateBe;
    let csbcpb = (*nx_ctx).csbcpb as *mut nx_csbcpb;
    let mut irq_flags: usize = 0;
    let mut rc: i32 = 0;
    let max_sg_len = core::cmp::min((*nx_ctx).ap.sglen as u64,
        nx_driver.of.max_sg_len as u64 / core::mem::size_of::<nx_sg>() as u64);
    let max_sg_len = core::cmp::min(max_sg_len,
        (*nx_ctx).ap.databytelen as u64 / NX_PAGE_SIZE as u64) as u32;
    spin_lock_irqsave(&mut (*nx_ctx).lock, &mut irq_flags);
    core::ptr::copy_nonoverlapping((*sctx).state.as_ptr() as *const u8,
        (*csbcpb).cpb.sha256.input_partial_digest.as_mut_ptr(), SHA256_DIGEST_SIZE);
    NX_CPB_FDM(csbcpb) &= !NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
    (*sctx).count = (*sctx).count.wrapping_add(nbytes as u64);
    (*csbcpb).cpb.sha256.message_bit_length = (*sctx).count.wrapping_mul(8);
    let mut len = nbytes as i32;
    let in_sg = nx_build_sg_list((*nx_ctx).in_sg, src as *mut u8, &mut len, max_sg_len);
    if len != nbytes as i32 { rc = -EINVAL; goto_out(&mut rc, nx_ctx, irq_flags); return rc; }
    len = SHA256_DIGEST_SIZE as i32;
    let out_sg = nx_build_sg_list((*nx_ctx).out_sg, out, &mut len, max_sg_len);
    if len != SHA256_DIGEST_SIZE as i32 { rc = -EINVAL; goto_out(&mut rc, nx_ctx, irq_flags); return rc; }
    (*nx_ctx).op.inlen = ((*nx_ctx).in_sg.offset_from(in_sg) as usize) * core::mem::size_of::<nx_sg>();
    (*nx_ctx).op.outlen = ((*nx_ctx).out_sg.offset_from(out_sg) as usize) * core::mem::size_of::<nx_sg>();
    if (*nx_ctx).op.outlen == 0 { rc = -EINVAL; goto_out(&mut rc, nx_ctx, irq_flags); return rc; }
    rc = nx_hcall_sync(nx_ctx, &mut (*nx_ctx).op, 0);
    if rc == 0 {
        atomic_inc(&mut (*(*nx_ctx).stats).sha256_ops);
        atomic64_add((*sctx).count, &mut (*(*nx_ctx).stats).sha256_bytes);
        core::ptr::copy_nonoverlapping((*csbcpb).cpb.sha256.message_digest.as_ptr(), out, SHA256_DIGEST_SIZE);
    }
    goto_out(&mut rc, nx_ctx, irq_flags);
    rc
}

unsafe fn nx_sha256_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha256StateBe;
    for i in 0..(SHA256_DIGEST_SIZE / core::mem::size_of::<u32>()) {
        put_unaligned(be32_to_cpu((*sctx).state[i]), (out as *mut u32).add(i));
    }
    put_unaligned((*sctx).count, (out as *mut u64).add(1));
    0
}

unsafe fn nx_sha256_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 {
    let sctx = shash_desc_ctx(desc) as *mut Sha256StateBe;
    for i in 0..(SHA256_DIGEST_SIZE / core::mem::size_of::<u32>()) {
        (*sctx).state[i] = cpu_to_be32(get_unaligned((input as *const u32).add(i)));
    }
    (*sctx).count = get_unaligned((input as *const u64).add(1));
    0
}

// The surrounding kernel provides the concrete `shash_alg` and callback ABI.
pub static mut nx_shash_sha256_alg: shash_alg = shash_alg {
    digestsize: SHA256_DIGEST_SIZE,
    init: Some(nx_sha256_init),
    update: Some(nx_sha256_update),
    finup: Some(nx_sha256_finup),
    export: Some(nx_sha256_export),
    import: Some(nx_sha256_import),
    init_tfm: Some(nx_crypto_ctx_sha256_init),
    exit_tfm: Some(nx_crypto_ctx_shash_exit),
    descsize: core::mem::size_of::<Sha256StateBe>(),
    statesize: core::mem::size_of::<Sha256StateBe>(),
    base: crypto_alg {
        cra_name: b"sha256\0".as_ptr() as *const i8,
        cra_driver_name: b"sha256-nx\0".as_ptr() as *const i8,
        cra_priority: 300,
        cra_flags: CRYPTO_AHASH_ALG_BLOCK_ONLY,
        cra_blocksize: SHA256_BLOCK_SIZE,
        cra_module: THIS_MODULE,
        cra_ctxsize: core::mem::size_of::<nx_crypto_ctx>(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
