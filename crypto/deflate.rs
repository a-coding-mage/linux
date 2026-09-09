// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Deflate algorithm (RFC 1951), implemented here primarily for use
 * by IPCOMP (RFC 3173 & RFC 2394).
 *
 * Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2023 Google, LLC. <ardb@kernel.org>
 * Copyright (c) 2025 Herbert Xu <herbert@gondor.apana.org.au>
 */

const DEFLATE_DEF_LEVEL: i32 = Z_DEFAULT_COMPRESSION;
const DEFLATE_DEF_WINBITS: i32 = 11;
const DEFLATE_DEF_MEMLEVEL: i32 = MAX_MEM_LEVEL;

#[repr(C)]
struct deflate_stream {
    stream: z_stream_s,
    workspace: [u8; 0],
}

static mut deflate_stream_lock: mutex = DEFINE_MUTEX!();

unsafe fn deflate_alloc_stream() -> *mut c_void {
    let size = core::cmp::max(
        zlib_inflate_workspacesize(),
        zlib_deflate_workspacesize(-DEFLATE_DEF_WINBITS, DEFLATE_DEF_MEMLEVEL),
    );
    let ctx: *mut deflate_stream = kvmalloc_flex::<deflate_stream>(size);
    if ctx.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*ctx).stream.workspace = (*ctx).workspace.as_mut_ptr();
    ctx.cast()
}

unsafe fn deflate_free_stream(ctx: *mut c_void) {
    kvfree(ctx);
}

static mut deflate_streams: crypto_acomp_streams = crypto_acomp_streams {
    alloc_ctx: Some(deflate_alloc_stream),
    free_ctx: Some(deflate_free_stream),
};

unsafe fn deflate_compress_one(req: *mut acomp_req, ds: *mut deflate_stream) -> i32 {
    let stream: *mut z_stream_s = &mut (*ds).stream;
    let mut walk: acomp_walk = core::mem::zeroed();
    let mut ret: i32;

    ret = acomp_walk_virt(&mut walk, req, true);
    if ret != 0 { return ret; }

    loop {
        let dcur: u32 = acomp_walk_next_dst(&mut walk);
        if dcur == 0 { return -ENOSPC; }

        (*stream).avail_out = dcur;
        (*stream).next_out = walk.dst.virt.addr;

        loop {
            let mut flush: i32 = Z_FINISH;
            let mut scur: u32;
            (*stream).avail_in = 0;
            (*stream).next_in = core::ptr::null_mut();

            scur = acomp_walk_next_src(&mut walk);
            if scur != 0 {
                if acomp_walk_more_src(&mut walk, scur) { flush = Z_NO_FLUSH; }
                (*stream).avail_in = scur;
                (*stream).next_in = walk.src.virt.addr;
            }

            ret = zlib_deflate(stream, flush);
            if scur != 0 {
                scur -= (*stream).avail_in;
                acomp_walk_done_src(&mut walk, scur);
            }
            if !(ret == Z_OK && (*stream).avail_out != 0) { break; }
        }
        acomp_walk_done_dst(&mut walk, dcur);
        if ret != Z_OK { break; }
    }

    if ret != Z_STREAM_END { return -EINVAL; }
    (*req).dlen = (*stream).total_out;
    0
}

unsafe fn deflate_compress(req: *mut acomp_req) -> i32 {
    let s: *mut crypto_acomp_stream = crypto_acomp_lock_stream_bh(&mut deflate_streams);
    let ds: *mut deflate_stream = (*s).ctx.cast();
    let mut err = zlib_deflateInit2(&mut (*ds).stream, DEFLATE_DEF_LEVEL, Z_DEFLATED,
                                    -DEFLATE_DEF_WINBITS, DEFLATE_DEF_MEMLEVEL,
                                    Z_DEFAULT_STRATEGY);
    if err != Z_OK { err = -EINVAL; } else { err = deflate_compress_one(req, ds); }
    crypto_acomp_unlock_stream_bh(s);
    err
}

unsafe fn deflate_decompress_one(req: *mut acomp_req, ds: *mut deflate_stream) -> i32 {
    let stream: *mut z_stream_s = &mut (*ds).stream;
    let mut out_of_space = false;
    let mut walk: acomp_walk = core::mem::zeroed();
    let mut ret = acomp_walk_virt(&mut walk, req, true);
    if ret != 0 { return ret; }

    loop {
        let scur: u32;
        (*stream).avail_in = 0;
        (*stream).next_in = core::ptr::null_mut();
        scur = acomp_walk_next_src(&mut walk);
        if scur != 0 { (*stream).avail_in = scur; (*stream).next_in = walk.src.virt.addr; }

        loop {
            let dcur = acomp_walk_next_dst(&mut walk);
            (*stream).avail_out = dcur;
            (*stream).next_out = walk.dst.virt.addr;
            let avail_in = (*stream).avail_in;
            ret = zlib_inflate(stream, Z_NO_FLUSH);
            if dcur == 0 && avail_in == (*stream).avail_in { out_of_space = true; break; }
            acomp_walk_done_dst(&mut walk, dcur - (*stream).avail_out);
            if !(ret == Z_OK && (*stream).avail_in != 0) { break; }
        }
        if scur != 0 { acomp_walk_done_src(&mut walk, scur); }
        if out_of_space { return -ENOSPC; }
        if ret != Z_OK { break; }
    }
    if ret != Z_STREAM_END { return -EINVAL; }
    (*req).dlen = (*stream).total_out;
    0
}

unsafe fn deflate_decompress(req: *mut acomp_req) -> i32 {
    let s = crypto_acomp_lock_stream_bh(&mut deflate_streams);
    let ds: *mut deflate_stream = (*s).ctx.cast();
    let mut err = zlib_inflateInit2(&mut (*ds).stream, -DEFLATE_DEF_WINBITS);
    if err != Z_OK { err = -EINVAL; } else { err = deflate_decompress_one(req, ds); }
    crypto_acomp_unlock_stream_bh(s);
    err
}

unsafe fn deflate_init(_tfm: *mut crypto_acomp) -> i32 {
    mutex_lock(&mut deflate_stream_lock);
    let ret = crypto_acomp_alloc_streams(&mut deflate_streams);
    mutex_unlock(&mut deflate_stream_lock);
    ret
}

static mut acomp: acomp_alg = acomp_alg {
    compress: Some(deflate_compress),
    decompress: Some(deflate_decompress),
    init: Some(deflate_init),
    base: crypto_alg {
        cra_name: c"deflate",
        cra_driver_name: c"deflate-generic",
        cra_flags: CRYPTO_ALG_REQ_VIRT,
        cra_module: THIS_MODULE,
    },
};

unsafe fn deflate_mod_init() -> i32 { crypto_register_acomp(&mut acomp) }

unsafe fn deflate_mod_fini() {
    crypto_unregister_acomp(&mut acomp);
    crypto_acomp_free_streams(&mut deflate_streams);
}

module_init!(deflate_mod_init);
module_exit!(deflate_mod_fini);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Deflate Compression Algorithm for IPCOMP");
MODULE_AUTHOR!("James Morris <jmorris@intercode.com.au>");
MODULE_AUTHOR!("Ard Biesheuvel <ardb@kernel.org>");
MODULE_AUTHOR!("Herbert Xu <herbert@gondor.apana.org.au>");
MODULE_ALIAS_CRYPTO!("deflate");
MODULE_ALIAS_CRYPTO!("deflate-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
