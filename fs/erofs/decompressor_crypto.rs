// SPDX-License-Identifier: GPL-2.0-or-later
// Dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct z_erofs_decompress_req {
    pub inputsize: u32,
    pub outputsize: u32,
    pub pageofs_in: u32,
    pub pageofs_out: u32,
    pub inpages: usize,
    pub outpages: usize,
    pub in_: *mut *mut page,
    pub out: *mut *mut page,
    pub sb: *mut super_block,
    pub alg: c_int,
    pub gfp: c_uint,
}

pub type c_uint = u32;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_acomp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acomp_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_wait {
    _private: [u8; 0],
}

#[repr(C)]
pub struct z_erofs_crypto_engine {
    pub crypto_name: *mut c_char,
    pub tfm: *mut crypto_acomp,
}

extern "C" {
    fn kmap_local_page(page: *mut page) -> *mut c_void;
    fn kunmap_local(addr: *mut c_void);
    fn z_erofs_fixup_insize(rq: *mut z_erofs_decompress_req, headpage: *const u8, size: c_uint) -> *const c_char;
    fn acomp_request_alloc(tfm: *mut crypto_acomp) -> *mut acomp_req;
    fn sg_alloc_table_from_pages_segment(table: *mut sg_table, pages: *mut *mut page, n_pages: usize, offset: c_uint, size: u32, max_segment: c_uint, gfp: c_uint) -> c_int;
    fn sg_free_table(table: *mut sg_table);
    fn acomp_request_set_params(req: *mut acomp_req, src: *mut c_void, dst: *mut c_void, src_len: u32, dst_len: u32);
    fn crypto_init_wait(wait: *mut crypto_wait);
    fn acomp_request_set_callback(req: *mut acomp_req, flags: c_uint, callback: unsafe extern "C" fn(*mut c_void, c_int), data: *mut crypto_wait);
    fn crypto_req_done(data: *mut c_void, err: c_int);
    fn crypto_wait_req(err: c_int, wait: *mut crypto_wait) -> c_int;
    fn crypto_acomp_decompress(req: *mut acomp_req) -> c_int;
    fn acomp_request_free(req: *mut acomp_req);
    fn erofs_err(sb: *mut super_block, fmt: *const c_char, ...);
    fn __erofs_allocpage(pgpl: *mut *mut page, gfp: c_uint, managed: bool) -> *mut page;
    fn set_page_private(page: *mut page, value: usize);
    fn crypto_alloc_acomp(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_acomp;
    fn crypto_free_acomp(tfm: *mut crypto_acomp);
    fn strncmp(a: *const c_char, b: *const c_char, len: usize) -> c_int;
    fn scnprintf(buf: *mut c_char, size: c_int, fmt: *const c_char, ... ) -> c_int;
    fn down_read(sem: *mut c_void);
    fn up_read(sem: *mut c_void);
    fn down_write(sem: *mut c_void);
    fn up_write(sem: *mut c_void);
}

const Z_EROFS_COMPRESSION_MAX: usize = 4;
const Z_EROFS_COMPRESSION_LZ4: usize = 0;
const Z_EROFS_COMPRESSION_LZMA: usize = 1;
const Z_EROFS_COMPRESSION_DEFLATE: usize = 2;
const Z_EROFS_COMPRESSION_ZSTD: usize = 3;
const Z_EROFS_SHORTLIVED_PAGE: usize = 0;
const UINT_MAX: c_uint = u32::MAX;
const GFP_KERNEL: c_uint = 0;
const CRYPTO_TFM_REQ_MAY_BACKLOG: c_uint = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EOPNOTSUPP: c_int = 95;
const EFSCORRUPTED: c_int = 990;

static mut Z_EROFS_CRYPTO: [*mut z_erofs_crypto_engine; Z_EROFS_COMPRESSION_MAX] = [core::ptr::null_mut(); Z_EROFS_COMPRESSION_MAX];
static mut Z_EROFS_CRYPTO_RWSEM: *mut c_void = core::ptr::null_mut();

unsafe fn __z_erofs_crypto_decompress(rq: *mut z_erofs_decompress_req, tfm: *mut crypto_acomp) -> c_int {
    let mut st_src: sg_table = core::mem::zeroed();
    let mut st_dst: sg_table = core::mem::zeroed();
    let mut wait: crypto_wait = core::mem::zeroed();
    let headpage = kmap_local_page(*(*rq).in_);
    let reason = z_erofs_fixup_insize(rq, (headpage as *mut u8).add((*rq).pageofs_in as usize),
        core::cmp::min((*rq).inputsize, UINT_MAX));
    kunmap_local(headpage);
    if !reason.is_null() { return -EFSCORRUPTED; }
    let req = acomp_request_alloc(tfm);
    if req.is_null() { return -ENOMEM; }
    let mut ret = sg_alloc_table_from_pages_segment(&mut st_src, (*rq).in_, (*rq).inpages,
        (*rq).pageofs_in, (*rq).inputsize, UINT_MAX, GFP_KERNEL);
    if ret < 0 { acomp_request_free(req); return ret; }
    ret = sg_alloc_table_from_pages_segment(&mut st_dst, (*rq).out, (*rq).outpages,
        (*rq).pageofs_out, (*rq).outputsize, UINT_MAX, GFP_KERNEL);
    if ret < 0 { sg_free_table(&mut st_src); acomp_request_free(req); return ret; }
    acomp_request_set_params(req, core::ptr::null_mut(), core::ptr::null_mut(), (*rq).inputsize, (*rq).outputsize);
    crypto_init_wait(&mut wait);
    acomp_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, &mut wait);
    ret = crypto_wait_req(crypto_acomp_decompress(req), &mut wait);
    if ret != 0 { ret = -EIO; }
    sg_free_table(&mut st_dst);
    sg_free_table(&mut st_src);
    acomp_request_free(req);
    ret
}

unsafe fn z_erofs_crypto_get_engine(alg: c_int) -> *mut crypto_acomp {
    let mut e = Z_EROFS_CRYPTO[alg as usize];
    while !e.is_null() {
        if !(*e).crypto_name.is_null() && !(*e).tfm.is_null() { return (*e).tfm; }
        e = e.add(1);
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_decompress(rq: *mut z_erofs_decompress_req, pgpl: *mut *mut page) -> c_int {
    down_read(Z_EROFS_CRYPTO_RWSEM);
    let tfm = z_erofs_crypto_get_engine((*rq).alg);
    if tfm.is_null() { up_read(Z_EROFS_CRYPTO_RWSEM); return -EOPNOTSUPP; }
    for i in 0..(*rq).outpages {
        if (*(*rq).out.add(i)).is_null() {
            let victim = __erofs_allocpage(pgpl, (*rq).gfp, true);
            if victim.is_null() { up_read(Z_EROFS_CRYPTO_RWSEM); return -ENOMEM; }
            set_page_private(victim, Z_EROFS_SHORTLIVED_PAGE);
            *(*rq).out.add(i) = victim;
        }
    }
    let ret = __z_erofs_crypto_decompress(rq, tfm);
    up_read(Z_EROFS_CRYPTO_RWSEM);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_enable_engine(name: *const c_char, len: c_int) -> c_int {
    down_write(Z_EROFS_CRYPTO_RWSEM);
    for alg in 0..Z_EROFS_COMPRESSION_MAX {
        let mut e = Z_EROFS_CRYPTO[alg];
        while !e.is_null() && !(*e).crypto_name.is_null() {
            if strncmp(name, (*e).crypto_name, len as usize) == 0 {
                if !(*e).tfm.is_null() { break; }
                let tfm = crypto_alloc_acomp((*e).crypto_name, 0, 0);
                if tfm.is_null() { up_write(Z_EROFS_CRYPTO_RWSEM); return -EOPNOTSUPP; }
                (*e).tfm = tfm;
                break;
            }
            e = e.add(1);
        }
    }
    up_write(Z_EROFS_CRYPTO_RWSEM);
    0
}

#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_disable_all_engines() {
    down_write(Z_EROFS_CRYPTO_RWSEM);
    for alg in 0..Z_EROFS_COMPRESSION_MAX {
        let mut e = Z_EROFS_CRYPTO[alg];
        while !e.is_null() && !(*e).crypto_name.is_null() {
            if !(*e).tfm.is_null() {
                crypto_free_acomp((*e).tfm);
                (*e).tfm = core::ptr::null_mut();
            }
            e = e.add(1);
        }
    }
    up_write(Z_EROFS_CRYPTO_RWSEM);
}

#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_show_engines(buf: *mut c_char, size: c_int, sep: c_char) -> c_int {
    let mut len = 0;
    for alg in 0..Z_EROFS_COMPRESSION_MAX {
        let mut e = Z_EROFS_CRYPTO[alg];
        while !e.is_null() && !(*e).crypto_name.is_null() {
            if !(*e).tfm.is_null() {
                len += scnprintf(buf.add(len as usize), size - len, b"%s%c\0".as_ptr() as *const c_char, (*e).crypto_name, sep);
            }
            e = e.add(1);
        }
    }
    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
