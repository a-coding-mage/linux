// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cryptographic API.
 *
 * Copyright (c) 2017-present, Facebook, Inc.
 */

// Linux kernel and crypto dependencies are supplied by the surrounding crate.

const ZSTD_DEF_LEVEL: i32 = 3;
const ZSTD_MAX_WINDOWLOG: u32 = 18;
const ZSTD_MAX_SIZE: usize = 1usize << ZSTD_MAX_WINDOWLOG;

#[repr(C)]
pub struct zstd_ctx {
    pub cctx: *mut zstd_cctx,
    pub dctx: *mut zstd_dctx,
    pub wksp_size: usize,
    pub params: zstd_parameters,
    pub wksp: [u8; 0],
}

static mut ZSTD_STREAM_LOCK: mutex = mutex::new();

unsafe fn zstd_alloc_stream() -> *mut core::ffi::c_void {
    let params = zstd_get_params(ZSTD_DEF_LEVEL, ZSTD_MAX_SIZE);
    let wksp_size = core::cmp::max(
        zstd_cstream_workspace_bound(&params.cParams),
        zstd_dstream_workspace_bound(ZSTD_MAX_SIZE),
    );
    if wksp_size == 0 {
        return ERR_PTR(-EINVAL);
    }

    let ctx = kvmalloc_flex::<zstd_ctx>(wksp_size);
    if ctx.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*ctx).params = params;
    (*ctx).wksp_size = wksp_size;
    ctx as *mut core::ffi::c_void
}

unsafe fn zstd_free_stream(ctx: *mut core::ffi::c_void) {
    kvfree(ctx);
}

static mut ZSTD_STREAMS: crypto_acomp_streams = crypto_acomp_streams {
    alloc_ctx: Some(zstd_alloc_stream),
    free_ctx: Some(zstd_free_stream),
};

unsafe fn zstd_init(acomp_tfm: *mut crypto_acomp) -> i32 {
    let mut ret = 0;
    mutex_lock(&raw mut ZSTD_STREAM_LOCK);
    ret = crypto_acomp_alloc_streams(&raw mut ZSTD_STREAMS);
    mutex_unlock(&raw mut ZSTD_STREAM_LOCK);
    ret
}

unsafe fn zstd_compress_one(
    req: *mut acomp_req,
    ctx: *mut zstd_ctx,
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    dlen: *mut u32,
) -> i32 {
    (*ctx).cctx = zstd_init_cctx((*ctx).wksp.as_mut_ptr(), (*ctx).wksp_size);
    if (*ctx).cctx.is_null() { return -EINVAL; }
    let out_len = zstd_compress_cctx((*ctx).cctx, dst, (*req).dlen, src, (*req).slen, &(*ctx).params);
    if zstd_is_error(out_len) { return -EINVAL; }
    *dlen = out_len as u32;
    0
}

unsafe fn zstd_compress(req: *mut acomp_req) -> i32 {
    let s = crypto_acomp_lock_stream_bh(&raw mut ZSTD_STREAMS);
    let ctx = (*s).ctx as *mut zstd_ctx;
    let mut total_out: u32 = 0;
    let mut data_available = true;
    let mut outbuf: zstd_out_buffer = core::mem::zeroed();
    let mut walk: acomp_walk = core::mem::zeroed();
    let mut inbuf: zstd_in_buffer = core::mem::zeroed();
    let mut ret = acomp_walk_virt(&mut walk, req, true);
    if ret != 0 { (*req).dlen = 0; crypto_acomp_unlock_stream_bh(s); return ret; }
    (*ctx).cctx = zstd_init_cstream(&(*ctx).params, 0, (*ctx).wksp.as_mut_ptr(), (*ctx).wksp_size);
    if (*ctx).cctx.is_null() { ret = -EINVAL; }
    if ret == 0 {
        'outer: loop {
            let dcur = acomp_walk_next_dst(&mut walk);
            if dcur == 0 { ret = -ENOSPC; break; }
            outbuf.pos = 0; outbuf.dst = walk.dst.virt.addr as *mut u8; outbuf.size = dcur as usize;
            loop {
                let scur = acomp_walk_next_src(&mut walk);
                if dcur == (*req).dlen && scur == (*req).slen {
                    ret = zstd_compress_one(req, ctx, walk.src.virt.addr, walk.dst.virt.addr, &mut total_out);
                    acomp_walk_done_src(&mut walk, scur); acomp_walk_done_dst(&mut walk, dcur); break 'outer;
                }
                if scur == 0 { data_available = false; break; }
                inbuf.pos = 0; inbuf.src = walk.src.virt.addr as *const u8; inbuf.size = scur as usize;
                let n = zstd_compress_stream((*ctx).cctx, &mut outbuf, &mut inbuf);
                if ZSTD_isError(n) { ret = -EIO; break 'outer; }
                let p = zstd_flush_stream((*ctx).cctx, &mut outbuf);
                if ZSTD_isError(p) { ret = -EIO; break 'outer; }
                acomp_walk_done_src(&mut walk, inbuf.pos as u32);
                if dcur == outbuf.pos as u32 { break; }
            }
            total_out = total_out.wrapping_add(outbuf.pos as u32);
            acomp_walk_done_dst(&mut walk, dcur);
            if !data_available { break; }
        }
        if ret == 0 && data_available {
            let pos = outbuf.pos;
            let n = zstd_end_stream((*ctx).cctx, &mut outbuf);
            if ZSTD_isError(n) { ret = -EIO; } else { total_out = total_out.wrapping_add((outbuf.pos - pos) as u32); }
        }
    }
    (*req).dlen = if ret != 0 { 0 } else { total_out };
    crypto_acomp_unlock_stream_bh(s); ret
}

unsafe fn zstd_decompress_one(req: *mut acomp_req, ctx: *mut zstd_ctx, src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, dlen: *mut u32) -> i32 {
    (*ctx).dctx = zstd_init_dctx((*ctx).wksp.as_mut_ptr(), (*ctx).wksp_size);
    if (*ctx).dctx.is_null() { return -EINVAL; }
    let out_len = zstd_decompress_dctx((*ctx).dctx, dst, (*req).dlen, src, (*req).slen);
    if zstd_is_error(out_len) { return -EINVAL; }
    *dlen = out_len as u32; 0
}

unsafe fn zstd_decompress(req: *mut acomp_req) -> i32 {
    let s = crypto_acomp_lock_stream_bh(&raw mut ZSTD_STREAMS);
    let ctx = (*s).ctx as *mut zstd_ctx;
    let mut total_out = 0u32;
    let mut outbuf: zstd_out_buffer = core::mem::zeroed();
    let mut inbuf: zstd_in_buffer = core::mem::zeroed();
    let mut walk: acomp_walk = core::mem::zeroed();
    let mut ret = acomp_walk_virt(&mut walk, req, true);
    if ret == 0 { (*ctx).dctx = zstd_init_dstream(ZSTD_MAX_SIZE, (*ctx).wksp.as_mut_ptr(), (*ctx).wksp_size); if (*ctx).dctx.is_null() { ret = -EINVAL; } }
    if ret == 0 { 'outer: loop { let scur = acomp_walk_next_src(&mut walk); if scur == 0 { break; } inbuf.pos=0; inbuf.size=scur as usize; inbuf.src=walk.src.virt.addr as *const u8; loop { let dcur=acomp_walk_next_dst(&mut walk); if dcur==(*req).dlen && scur==(*req).slen { ret=zstd_decompress_one(req,ctx,walk.src.virt.addr,walk.dst.virt.addr,&mut total_out); acomp_walk_done_dst(&mut walk,dcur); acomp_walk_done_src(&mut walk,scur); break 'outer; } if dcur==0 { ret=-ENOSPC; break 'outer; } outbuf.pos=0; outbuf.dst=walk.dst.virt.addr as *mut u8; outbuf.size=dcur as usize; let n=zstd_decompress_stream((*ctx).dctx,&mut outbuf,&mut inbuf); if ZSTD_isError(n) { ret=-EIO; break 'outer; } total_out=total_out.wrapping_add(outbuf.pos as u32); acomp_walk_done_dst(&mut walk,outbuf.pos as u32); if inbuf.pos as u32==scur { break; } } acomp_walk_done_src(&mut walk,scur); } }
    (*req).dlen=if ret!=0 {0} else {total_out}; crypto_acomp_unlock_stream_bh(s); ret
}

static mut ZSTD_ACOMP: acomp_alg = acomp_alg { base: crypto_alg { cra_name: c"zstd", cra_driver_name: c"zstd-generic", cra_flags: CRYPTO_ALG_REQ_VIRT, cra_module: THIS_MODULE }, init: Some(zstd_init), compress: Some(zstd_compress), decompress: Some(zstd_decompress) };

unsafe fn zstd_mod_init() -> i32 { crypto_register_acomp(&raw mut ZSTD_ACOMP) }
unsafe fn zstd_mod_fini() { crypto_unregister_acomp(&raw mut ZSTD_ACOMP); crypto_acomp_free_streams(&raw mut ZSTD_STREAMS); }

// module_init(zstd_mod_init); module_exit(zstd_mod_fini);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Zstd Compression Algorithm"); MODULE_ALIAS_CRYPTO("zstd");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
