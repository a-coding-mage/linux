// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive Public Key Algo acceleration driver
 *
 * Copyright (c) 2022 StarFive Technology
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally external.

const STARFIVE_PKA_REGS_OFFSET: usize = 0x400;
const STARFIVE_PKA_CACR_OFFSET: usize = STARFIVE_PKA_REGS_OFFSET + 0x0;
const STARFIVE_PKA_CASR_OFFSET: usize = STARFIVE_PKA_REGS_OFFSET + 0x4;
const STARFIVE_PKA_CAAR_OFFSET: usize = STARFIVE_PKA_REGS_OFFSET + 0x8;
const STARFIVE_PKA_CAER_OFFSET: usize = STARFIVE_PKA_REGS_OFFSET + 0x108;
const STARFIVE_PKA_CANR_OFFSET: usize = STARFIVE_PKA_REGS_OFFSET + 0x208;

/* R ^ 2 mod N and N0' */
const CRYPTO_CMD_PRE: u32 = 0x0;
/* A * R mod N   ==> A */
const CRYPTO_CMD_ARN: u32 = 0x5;
/* A * E * R mod N ==> A */
const CRYPTO_CMD_AERN: u32 = 0x6;
/* A * A * R mod N ==> A */
const CRYPTO_CMD_AARN: u32 = 0x7;

const STARFIVE_RSA_RESET: u32 = 0x2;

unsafe fn starfive_pka_wait_done(ctx: *mut starfive_cryp_ctx) -> i32 {
    let cryp = (*ctx).cryp;
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_PKA_CASR_OFFSET), &mut status,
        status & STARFIVE_PKA_DONE != 0, 10, 100000)
}

unsafe fn starfive_rsa_free_key(key: *mut starfive_rsa_key) {
    if (*key).key_sz == 0 { return; }
    kfree_sensitive((*key).d);
    kfree_sensitive((*key).e);
    kfree_sensitive((*key).n);
    memset(key as *mut _, 0, core::mem::size_of::<starfive_rsa_key>());
}

unsafe fn starfive_rsa_get_nbit(pa: *mut u8, snum: u32, key_sz: i32) -> u32 {
    let i = snum >> 3;
    let mut value = *pa.add(key_sz as usize - i as usize - 1);
    value >>= snum & 0x7;
    (value & 0x1) as u32
}

unsafe fn starfive_rsa_montgomery_form(ctx: *mut starfive_cryp_ctx, out: *mut u32,
    input: *mut u32, mont: u8, modu: *mut u32, bit_len: i32) -> i32 {
    let cryp = (*ctx).cryp;
    let rctx = (*ctx).rctx;
    let count = (align((*rctx).total, 4) / 4) - 1;
    let opsize = ((bit_len - 1) >> 5) as u8;
    (*rctx).csr.pka.v = 0;
    writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET));
    for lp in 0..=opsize { writel(*modu.add((opsize - lp) as usize), (*cryp).base.add(STARFIVE_PKA_CANR_OFFSET + lp as usize * 4)); }
    if mont != 0 {
        (*rctx).csr.pka.v = 0; (*rctx).csr.pka.cln_done = 1; (*rctx).csr.pka.opsize = opsize;
        (*rctx).csr.pka.exposize = opsize; (*rctx).csr.pka.cmd = CRYPTO_CMD_PRE; (*rctx).csr.pka.start = 1;
        (*rctx).csr.pka.not_r2 = 1; (*rctx).csr.pka.ie = 1;
        writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET));
        if starfive_pka_wait_done(ctx) != 0 { return -ETIMEDOUT; }
        for lp in 0..=opsize { writel(*input.add((opsize-lp) as usize), (*cryp).base.add(STARFIVE_PKA_CAAR_OFFSET + lp as usize * 4)); }
        writel(0x1000000, (*cryp).base.add(STARFIVE_PKA_CAER_OFFSET));
        for lp in 1..=opsize { writel(0, (*cryp).base.add(STARFIVE_PKA_CAER_OFFSET + lp as usize * 4)); }
        (*rctx).csr.pka.v = 0; (*rctx).csr.pka.cln_done = 1; (*rctx).csr.pka.opsize = opsize;
        (*rctx).csr.pka.exposize = opsize; (*rctx).csr.pka.cmd = CRYPTO_CMD_AERN; (*rctx).csr.pka.start = 1; (*rctx).csr.pka.ie = 1;
        writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET));
        if starfive_pka_wait_done(ctx) != 0 { return -ETIMEDOUT; }
    } else {
        (*rctx).csr.pka.v = 0; (*rctx).csr.pka.cln_done = 1; (*rctx).csr.pka.opsize = opsize;
        (*rctx).csr.pka.exposize = opsize; (*rctx).csr.pka.cmd = CRYPTO_CMD_PRE; (*rctx).csr.pka.start = 1; (*rctx).csr.pka.pre_expf = 1; (*rctx).csr.pka.ie = 1;
        writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET));
        if starfive_pka_wait_done(ctx) != 0 { return -ETIMEDOUT; }
        for lp in 0..=count { writel(*input.add((count-lp) as usize), (*cryp).base.add(STARFIVE_PKA_CAER_OFFSET + lp as usize * 4)); }
        for lp in (count+1)..=(opsize as u32) { writel(0, (*cryp).base.add(STARFIVE_PKA_CAER_OFFSET + lp as usize * 4)); }
        (*rctx).csr.pka.v = 0; (*rctx).csr.pka.cln_done = 1; (*rctx).csr.pka.opsize = opsize;
        (*rctx).csr.pka.exposize = opsize; (*rctx).csr.pka.cmd = CRYPTO_CMD_ARN; (*rctx).csr.pka.start = 1; (*rctx).csr.pka.ie = 1;
        writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET));
        if starfive_pka_wait_done(ctx) != 0 { return -ETIMEDOUT; }
    }
    for lp in 0..=opsize { *out.add((opsize-lp) as usize) = readl((*cryp).base.add(STARFIVE_PKA_CAAR_OFFSET + lp as usize * 4)); }
    0
}

unsafe fn starfive_rsa_cpu_start(ctx: *mut starfive_cryp_ctx, result: *mut u32, de: *mut u8, n: *mut u32, key_sz: i32) -> i32 {
    let cryp = (*ctx).cryp; let rctx = (*ctx).rctx; let key = &mut (*ctx).rsa_key;
    let opsize = (key_sz - 1) >> 2; let mut ret = 0; let mta = kmalloc(key_sz as usize, GFP_KERNEL) as *mut u32;
    if mta.is_null() { return -ENOMEM; }
    ret = starfive_rsa_montgomery_form(ctx, mta, (*rctx).rsa_data as *mut u32, 0, n, key_sz << 3);
    if ret != 0 { dev_err_probe((*cryp).dev, ret, "Conversion to Montgomery failed"); kfree(mta as *mut _); return ret; }
    for lp in 0..=opsize { writel(*mta.add((opsize-lp) as usize), (*cryp).base.add(STARFIVE_PKA_CAER_OFFSET + lp as usize * 4)); }
    for lp in (1..key.bitlen).rev() { let mlen = starfive_rsa_get_nbit(de, (lp-1) as u32, key_sz); (*rctx).csr.pka.v=0; (*rctx).csr.pka.cln_done=1; (*rctx).csr.pka.opsize=opsize as u8; (*rctx).csr.pka.exposize=opsize as u8; (*rctx).csr.pka.cmd=CRYPTO_CMD_AARN; (*rctx).csr.pka.start=1; (*rctx).csr.pka.ie=1; writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET)); ret=-ETIMEDOUT; if starfive_pka_wait_done(ctx)!=0 { break; } if mlen != 0 { (*rctx).csr.pka.cmd=CRYPTO_CMD_AERN; writel((*rctx).csr.pka.v, (*cryp).base.add(STARFIVE_PKA_CACR_OFFSET)); if starfive_pka_wait_done(ctx)!=0 { break; } } }
    if ret == -ETIMEDOUT { kfree(mta as *mut _); return ret; }
    for lp in 0..=opsize { *result.add((opsize-lp) as usize)=readl((*cryp).base.add(STARFIVE_PKA_CAAR_OFFSET+lp as usize*4)); }
    ret=starfive_rsa_montgomery_form(ctx,result,result,1,n,key_sz<<3); if ret!=0 { dev_err_probe((*cryp).dev,ret,"Conversion from Montgomery failed"); } kfree(mta as *mut _); ret
}

unsafe fn starfive_rsa_start(ctx:*mut starfive_cryp_ctx,result:*mut u8,de:*mut u8,n:*mut u8,key_sz:i32)->i32 { starfive_rsa_cpu_start(ctx,result as *mut u32,de,n as *mut u32,key_sz) }

// The remaining akcipher glue preserves the original external kernel interfaces.
unsafe fn starfive_rsa_enc_core(ctx:*mut starfive_cryp_ctx,enc:i32)->i32 { let cryp=(*ctx).cryp; let rctx=(*ctx).rctx; let key=&mut (*ctx).rsa_key; writel(STARFIVE_RSA_RESET,(*cryp).base.add(STARFIVE_PKA_CACR_OFFSET)); let mut shift=0; if ((*rctx).total&3)!=0 { shift=4-((*rctx).total&3); memset((*rctx).rsa_data,0,shift as usize); } (*rctx).total=sg_copy_to_buffer((*rctx).in_sg,sg_nents((*rctx).in_sg),(*rctx).rsa_data.add(shift as usize),(*rctx).total); key.bitlen=if enc!=0{key.e_bitlen}else{key.d_bitlen}; let ret=starfive_rsa_start(ctx,(*rctx).rsa_data,if enc!=0{key.e}else{key.d},key.n,key.key_sz as i32); if ret==0 { sg_copy_buffer((*rctx).out_sg,sg_nents((*rctx).out_sg),(*rctx).rsa_data,key.key_sz,0,0); } writel(STARFIVE_RSA_RESET,(*cryp).base.add(STARFIVE_PKA_CACR_OFFSET)); ret }

unsafe fn starfive_rsa_set_n(key:*mut starfive_rsa_key,value:*const u8,mut vlen:usize)->i32 { let mut ptr=value; while vlen!=0 && *ptr==0 {ptr=ptr.add(1);vlen-=1;} (*key).key_sz=vlen; let bitslen=vlen<<3; if bitslen&0x1f!=0{return -EINVAL;} (*key).n=kmemdup(ptr,vlen,GFP_KERNEL); if (*key).n.is_null(){(*key).key_sz=0;starfive_rsa_free_key(key);return -ENOMEM;} 0 }
unsafe fn starfive_rsa_set_e(key:*mut starfive_rsa_key,value:*const u8,mut vlen:usize)->i32 { let mut ptr=value; while vlen!=0&&*ptr==0{ptr=ptr.add(1);vlen-=1;} if (*key).key_sz==0||vlen==0||vlen>(*key).key_sz{(*key).e=core::ptr::null_mut();return -EINVAL;} let pt=*ptr;(*key).e=kzalloc((*key).key_sz,GFP_KERNEL);if (*key).e.is_null(){return -ENOMEM;} let mut lp=8;while lp>0&&pt>>(lp-1)==0{lp-=1;}(*key).e_bitlen=((vlen-1)*8+lp as usize) as u32;memcpy((*key).e.add((*key).key_sz-vlen),ptr,vlen);0 }
unsafe fn starfive_rsa_set_d(key:*mut starfive_rsa_key,value:*const u8,mut vlen:usize)->i32 { let mut ptr=value;while vlen!=0&&*ptr==0{ptr=ptr.add(1);vlen-=1;}if (*key).key_sz==0||vlen==0||vlen>(*key).key_sz{(*key).d=core::ptr::null_mut();return -EINVAL;}let pt=*ptr;(*key).d=kzalloc((*key).key_sz,GFP_KERNEL);if (*key).d.is_null(){return -ENOMEM;}let mut lp=8;while lp>0&&pt>>(lp-1)==0{lp-=1;}(*key).d_bitlen=((vlen-1)*8+lp as usize) as u32;memcpy((*key).d.add((*key).key_sz-vlen),ptr,vlen);0 }

// Public key parsing, fallback dispatch, tfm initialization, algorithm registration,
// and the remaining thin wrappers retain the original external kernel ABI.
extern "C" { pub fn starfive_rsa_enc(req:*mut akcipher_request)->i32; pub fn starfive_rsa_dec(req:*mut akcipher_request)->i32; }

pub unsafe fn starfive_rsa_register_algs()->i32 { crypto_register_akcipher(&mut starfive_rsa) }
pub unsafe fn starfive_rsa_unregister_algs() { crypto_unregister_akcipher(&mut starfive_rsa); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
