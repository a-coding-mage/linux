// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sun4i-ss-hash.c - hardware cryptographic accelerator for Allwinner A20 SoC
 *
 * Copyright (C) 2013-2015 Corentin LABBE <clabbe.montjoie@gmail.com>
 *
 * This file add support for MD5 and SHA1.
 *
 * You could find the datasheet in Documentation/arch/arm/sunxi.rst
 */

// Dependencies supplied by the surrounding kernel/driver translation.

const SS_TIMEOUT: u32 = 100;
const SS_HASH_UPDATE: u32 = 1;
const SS_HASH_FINAL: u32 = 2;

pub unsafe fn sun4i_hash_crainit(tfm: *mut crypto_tfm) -> i32 {
    let op = crypto_tfm_ctx(tfm);
    let alg = __crypto_ahash_alg((*tfm).__crt_alg);
    let algt: *mut sun4i_ss_alg_template = container_of(alg, sun4i_ss_alg_template, alg.hash);
    core::ptr::write_bytes(op as *mut u8, 0, core::mem::size_of::<sun4i_tfm_ctx>());
    (*op).ss = (*algt).ss;
    let err = pm_runtime_resume_and_get((*(*op).ss).dev);
    if err < 0 { return err; }
    crypto_ahash_set_reqsize(__crypto_ahash_cast(tfm), core::mem::size_of::<sun4i_req_ctx>());
    0
}

pub unsafe fn sun4i_hash_craexit(tfm: *mut crypto_tfm) {
    let op = crypto_tfm_ctx(tfm);
    pm_runtime_put((*(*op).ss).dev);
}

pub unsafe fn sun4i_hash_init(areq: *mut ahash_request) -> i32 {
    let op = ahash_request_ctx(areq);
    let tfm = crypto_ahash_reqtfm(areq);
    let alg = __crypto_ahash_alg((*(*tfm).base).__crt_alg);
    let algt: *mut sun4i_ss_alg_template = container_of(alg, sun4i_ss_alg_template, alg.hash);
    core::ptr::write_bytes(op as *mut u8, 0, core::mem::size_of::<sun4i_req_ctx>());
    (*op).mode = (*algt).mode;
    0
}

pub unsafe fn sun4i_hash_export_md5(areq: *mut ahash_request, out: *mut core::ffi::c_void) -> i32 {
    let op = ahash_request_ctx(areq); let octx = out as *mut md5_state;
    (*octx).byte_count = (*op).byte_count + (*op).len as u64;
    core::ptr::copy_nonoverlapping((*op).buf.as_ptr(), (*octx).block.as_mut_ptr(), (*op).len as usize);
    if (*op).byte_count != 0 { for i in 0..4 { (*octx).hash[i] = (*op).hash[i]; } }
    else { (*octx).hash[0]=SHA1_H0; (*octx).hash[1]=SHA1_H1; (*octx).hash[2]=SHA1_H2; (*octx).hash[3]=SHA1_H3; }
    0
}

pub unsafe fn sun4i_hash_import_md5(areq: *mut ahash_request, input: *const core::ffi::c_void) -> i32 {
    let op = ahash_request_ctx(areq); let ictx = input as *const md5_state;
    sun4i_hash_init(areq);
    (*op).byte_count = (*ictx).byte_count & !0x3f;
    (*op).len = (*ictx).byte_count & 0x3f;
    core::ptr::copy_nonoverlapping((*ictx).block.as_ptr(), (*op).buf.as_mut_ptr(), (*op).len as usize);
    for i in 0..4 { (*op).hash[i] = (*ictx).hash[i]; }
    0
}

pub unsafe fn sun4i_hash_export_sha1(areq: *mut ahash_request, out: *mut core::ffi::c_void) -> i32 {
    let op = ahash_request_ctx(areq); let octx = out as *mut sha1_state;
    (*octx).count = (*op).byte_count + (*op).len as u64;
    core::ptr::copy_nonoverlapping((*op).buf.as_ptr(), (*octx).buffer.as_mut_ptr(), (*op).len as usize);
    if (*op).byte_count != 0 { for i in 0..5 { (*octx).state[i] = (*op).hash[i]; } }
    else { (*octx).state[0]=SHA1_H0; (*octx).state[1]=SHA1_H1; (*octx).state[2]=SHA1_H2; (*octx).state[3]=SHA1_H3; (*octx).state[4]=SHA1_H4; }
    0
}

pub unsafe fn sun4i_hash_import_sha1(areq: *mut ahash_request, input: *const core::ffi::c_void) -> i32 {
    let op = ahash_request_ctx(areq); let ictx = input as *const sha1_state;
    sun4i_hash_init(areq);
    (*op).byte_count = (*ictx).count & !0x3f; (*op).len = (*ictx).count & 0x3f;
    core::ptr::copy_nonoverlapping((*ictx).buffer.as_ptr(), (*op).buf.as_mut_ptr(), (*op).len as usize);
    for i in 0..5 { (*op).hash[i] = (*ictx).state[i]; }
    0
}

unsafe fn sun4i_hash(areq: *mut ahash_request) -> i32 {
    let mut i: u32 = 0; let mut end: u32; let mut fill: u32; let mut min_fill: u32; let mut nwait: u32; let mut nbw: u32 = 0; let mut j: u32 = 0; let mut todo: u32;
    let mut in_i: u32 = 0; let mut spaces: u32; let mut rx_cnt: u32 = SS_RX_DEFAULT; let mut bf = [0u32;32]; let mut v: u32; let mut ivmode: u32 = 0;
    let op = ahash_request_ctx(areq); let tfm = crypto_ahash_reqtfm(areq); let alg = __crypto_ahash_alg((*(*tfm).base).__crt_alg);
    let tfmctx = crypto_ahash_ctx(tfm); let ss = (*tfmctx).ss; let mut algt: *mut sun4i_ss_alg_template; let mut in_sg = (*areq).src; let mut mi: sg_mapping_iter = core::mem::zeroed();
    let mut in_r: usize; let mut err: i32 = 0; let mut copied: usize = 0; let mut wb: u32 = 0;
    if (*areq).nbytes == 0 && ((*op).flags & SS_HASH_FINAL) == 0 { return 0; }
    if (*areq).nbytes > (u32::MAX - (*op).len) { dev_err((*ss).dev, c"Cannot process too large request\n"); return -EINVAL; }
    if (*op).len + (*areq).nbytes < 64 && ((*op).flags & SS_HASH_FINAL) == 0 { copied=sg_pcopy_to_buffer((*areq).src, sg_nents((*areq).src), (*op).buf.as_mut_ptr().add((*op).len as usize), (*areq).nbytes as usize, 0); (*op).len += copied as u32; return 0; }
    spin_lock_bh(&mut (*ss).slock);
    if (*op).byte_count != 0 { ivmode=SS_IV_ARBITRARY; for k in 0..(crypto_ahash_digestsize(tfm)/4) { writel((*op).hash[k], (*ss).base.add(SS_IV0 + k*4)); } }
    writel((*op).mode | SS_ENABLED | ivmode, (*ss).base.add(SS_CTL));
    if ((*op).flags & SS_HASH_UPDATE) == 0 { goto_hash_final: (); } else {
        if ((*op).flags & SS_HASH_FINAL) == 0 { end=(((*areq).nbytes+(*op).len)/64)*64-(*op).len; if end>(*areq).nbytes || (*areq).nbytes-end>63 { err=-EINVAL; goto_release_ss: (); writel(0,(*ss).base.add(SS_CTL)); spin_unlock_bh(&mut (*ss).slock); return err; } }
        else { end=if (*areq).nbytes<4 {0} else {(((*areq).nbytes+(*op).len)/4)*4-(*op).len}; }
        let mut valid=1; while !in_sg.is_null() && valid==1 { if (*in_sg).length%4!=0 {valid=0;} in_sg=sg_next(in_sg); }
        sg_miter_start(&mut mi, (*areq).src, sg_nents((*areq).src), SG_MITER_FROM_SG|SG_MITER_ATOMIC); sg_miter_next(&mut mi); in_i=0;
        loop { if (*op).len!=0 || mi.length-in_i<4 { while (*op).len<64 && i<end { in_r=core::cmp::min((end-i) as usize,(64-(*op).len) as usize); in_r=core::cmp::min((mi.length-in_i) as usize,in_r); core::ptr::copy_nonoverlapping(mi.addr.add(in_i as usize),(*op).buf.as_mut_ptr().add((*op).len as usize),in_r); (*op).len+=in_r as u32;i+=in_r as u32;in_i+=in_r as u32;if in_i==mi.length {sg_miter_next(&mut mi);in_i=0;} } if (*op).len>3 && (*op).len%4==0 {writesl((*ss).base.add(SS_RXFIFO),(*op).buf.as_ptr(),(*op).len/4);(*op).byte_count+=(*op).len as u64;(*op).len=0;} } if mi.length-in_i>3 && i<end {in_r=core::cmp::min((mi.length-in_i) as usize,(*areq).nbytes as usize-i as usize);in_r=core::cmp::min(((mi.length-in_i)/4*4) as usize,in_r);todo=core::cmp::min(core::cmp::min((end-i)/4,rx_cnt),(in_r/4) as u32);writesl((*ss).base.add(SS_RXFIFO),mi.addr.add(in_i as usize),todo);(*op).byte_count+=(todo*4) as u64;i+=todo*4;in_i+=todo*4;rx_cnt-=todo;if rx_cnt==0 {spaces=readl((*ss).base.add(SS_FCSR));rx_cnt=SS_RXFIFO_SPACES(spaces);}if in_i==mi.length {sg_miter_next(&mut mi);in_i=0;} } if i>=end {break;} }
        if (*areq).nbytes-i<64 { while i<(*areq).nbytes && in_i<mi.length && (*op).len<64 {in_r=core::cmp::min(((*areq).nbytes-i) as usize,(64-(*op).len) as usize);in_r=core::cmp::min((mi.length-in_i) as usize,in_r);core::ptr::copy_nonoverlapping(mi.addr.add(in_i as usize),(*op).buf.as_mut_ptr().add((*op).len as usize),in_r);(*op).len+=in_r as u32;i+=in_r as u32;in_i+=in_r as u32;if in_i==mi.length {sg_miter_next(&mut mi);in_i=0;}} } sg_miter_stop(&mut mi);
        if ((*op).flags & SS_HASH_FINAL)==0 {writel((*op).mode|SS_ENABLED|SS_DATA_END,(*ss).base.add(SS_CTL));i=0;loop{v=readl((*ss).base.add(SS_CTL));i+=1;if !(i<SS_TIMEOUT && v&SS_DATA_END!=0){break;}}if i>=SS_TIMEOUT {err=-EIO;writel(0,(*ss).base.add(SS_CTL));spin_unlock_bh(&mut (*ss).slock);return err;}ndelay(1);for k in 0..(crypto_ahash_digestsize(tfm)/4){(*op).hash[k]=readl((*ss).base.add(SS_MD0+k*4));}writel(0,(*ss).base.add(SS_CTL));spin_unlock_bh(&mut (*ss).slock);return 0;}
    }
    if (*op).len!=0 {nwait=(*op).len/4;if nwait!=0 {writesl((*ss).base.add(SS_RXFIFO),(*op).buf.as_ptr(),nwait);(*op).byte_count+=(4*nwait) as u64;}nbw=(*op).len-4*nwait;if nbw!=0 {wb=le32_to_cpup((*op).buf.as_ptr().add((nwait*4) as usize));wb&=(1u32<<(nbw*8))-1;(*op).byte_count+=nbw as u64;}}
    wb|=1u32<<(7+nbw*8);bf[j as usize]=cpu_to_le32(wb);fill=64-((*op).byte_count%64) as u32;min_fill=8+if nbw==0{4}else{0};if fill<min_fill{fill+=64;}j+=(fill-min_fill)/4;if (*op).mode==SS_OP_SHA1 {let p=bf.as_mut_ptr().add(j as usize) as *mut u64;*p=cpu_to_be64((*op).byte_count<<3);}else{let p=bf.as_mut_ptr().add(j as usize) as *mut u64;*p=cpu_to_le64((*op).byte_count<<3);}j+=2; writesl((*ss).base.add(SS_RXFIFO),bf.as_ptr(),j);writel((*op).mode|SS_ENABLED|SS_DATA_END,(*ss).base.add(SS_CTL));i=0;loop{v=readl((*ss).base.add(SS_CTL));i+=1;if !(i<SS_TIMEOUT&&v&SS_DATA_END!=0){break;}}if i>=SS_TIMEOUT{err=-EIO;}else{ndelay(1);for k in 0..if (*op).mode==SS_OP_SHA1{5}else{4}{v=readl((*ss).base.add(SS_MD0+k*4));if (*op).mode==SS_OP_SHA1{if (*ss).variant.sha1_in_be{put_unaligned_le32(v,(*areq).result.add(k*4));}else{put_unaligned_be32(v,(*areq).result.add(k*4));}}else{put_unaligned_le32(v,(*areq).result.add(k*4));}}}writel(0,(*ss).base.add(SS_CTL));spin_unlock_bh(&mut (*ss).slock);err
}

pub unsafe fn sun4i_hash_final(areq:*mut ahash_request)->i32{let op=ahash_request_ctx(areq);(*op).flags=SS_HASH_FINAL;sun4i_hash(areq)}
pub unsafe fn sun4i_hash_update(areq:*mut ahash_request)->i32{let op=ahash_request_ctx(areq);(*op).flags=SS_HASH_UPDATE;sun4i_hash(areq)}
pub unsafe fn sun4i_hash_finup(areq:*mut ahash_request)->i32{let op=ahash_request_ctx(areq);(*op).flags=SS_HASH_UPDATE|SS_HASH_FINAL;sun4i_hash(areq)}
pub unsafe fn sun4i_hash_digest(areq:*mut ahash_request)->i32{let op=ahash_request_ctx(areq);let err=sun4i_hash_init(areq);if err!=0{return err;}(*op).flags=SS_HASH_UPDATE|SS_HASH_FINAL;sun4i_hash(areq)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
