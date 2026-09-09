// SPDX-License-Identifier: GPL-2.0
/*
 * ECDH helper functions - KPP wrappings
 *
 * Copyright (C) 2017 Intel Corporation
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
 * CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
 * COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
 * SOFTWARE IS DISCLAIMED.
 */

#[repr(C)]
pub struct crypto_kpp {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kpp_request {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}
#[repr(C)]
pub struct crypto_wait {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ecdh {
    pub key: *const u8,
    pub key_size: u32,
}

extern "C" {
    fn __swab64(value: u64) -> u64;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree_sensitive(ptr: *mut u8);
    fn kfree(ptr: *mut u8);
    fn kpp_request_alloc(tfm: *mut crypto_kpp, flags: u32) -> *mut kpp_request;
    fn kpp_request_free(req: *mut kpp_request);
    fn sg_init_one(sg: *mut scatterlist, buf: *mut u8, len: usize);
    fn kpp_request_set_input(req: *mut kpp_request, src: *mut scatterlist, len: usize);
    fn kpp_request_set_output(req: *mut kpp_request, dst: *mut scatterlist, len: usize);
    fn kpp_request_set_callback(req: *mut kpp_request, flags: u32, cb: unsafe extern "C" fn(), data: *mut crypto_wait);
    fn crypto_req_done();
    fn crypto_kpp_compute_shared_secret(req: *mut kpp_request) -> i32;
    fn crypto_kpp_generate_public_key(req: *mut kpp_request) -> i32;
    fn crypto_wait_req(err: i32, wait: *mut crypto_wait) -> i32;
    fn crypto_ecdh_key_len(p: *mut ecdh) -> u32;
    fn crypto_ecdh_encode_key(buf: *mut u8, len: u32, p: *mut ecdh) -> i32;
    fn crypto_kpp_set_secret(tfm: *mut crypto_kpp, buf: *mut u8, len: u32) -> i32;
    fn pr_err(fmt: *const u8, ...);
}

const GFP_KERNEL: u32 = 0;
const CRYPTO_TFM_REQ_MAY_BACKLOG: u32 = 1;

unsafe fn swap_digits(in_: *const u64, out: *mut u64, ndigits: u32) {
    let mut i = 0;
    while i < ndigits {
        *out.add(i as usize) = __swab64(*in_.add((ndigits - 1 - i) as usize));
        i += 1;
    }
}

pub unsafe fn compute_ecdh_secret(tfm: *mut crypto_kpp, public_key: *const u8, secret: *mut u8) -> i32 {
    let mut result = core::mem::MaybeUninit::<crypto_wait>::uninit();
    let mut src = core::mem::MaybeUninit::<scatterlist>::uninit();
    let mut dst = core::mem::MaybeUninit::<scatterlist>::uninit();
    let tmp = kmalloc(64, GFP_KERNEL);
    if tmp.is_null() { return -12; }
    let req = kpp_request_alloc(tfm, GFP_KERNEL);
    if req.is_null() { kfree_sensitive(tmp); return -12; }
    swap_digits(public_key as *const u64, tmp as *mut u64, 4);
    swap_digits(public_key.add(32) as *const u64, tmp.add(32) as *mut u64, 4);
    sg_init_one(src.as_mut_ptr() as *mut scatterlist, tmp, 64);
    sg_init_one(dst.as_mut_ptr() as *mut scatterlist, secret, 32);
    kpp_request_set_input(req, src.as_mut_ptr() as *mut scatterlist, 64);
    kpp_request_set_output(req, dst.as_mut_ptr() as *mut scatterlist, 32);
    kpp_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, result.as_mut_ptr());
    let mut err = crypto_wait_req(crypto_kpp_compute_shared_secret(req), result.as_mut_ptr());
    if err < 0 {
        pr_err(b"alg: ecdh: compute shared secret failed. err %d\0".as_ptr(), err);
    } else {
        swap_digits(secret as *const u64, tmp as *mut u64, 4);
        core::ptr::copy_nonoverlapping(tmp, secret, 32);
    }
    kpp_request_free(req);
    kfree_sensitive(tmp);
    err
}

pub unsafe fn set_ecdh_privkey(tfm: *mut crypto_kpp, private_key: *const u8) -> i32 {
    let mut tmp = core::ptr::null_mut();
    let mut p = ecdh { key: core::ptr::null(), key_size: 0 };
    if !private_key.is_null() {
        tmp = kmalloc(32, GFP_KERNEL);
        if tmp.is_null() { return -12; }
        swap_digits(private_key as *const u64, tmp as *mut u64, 4);
        p.key = tmp;
        p.key_size = 32;
    }
    let buf_len = crypto_ecdh_key_len(&mut p);
    let buf = kmalloc(buf_len as usize, GFP_KERNEL);
    if buf.is_null() { kfree_sensitive(tmp); return -12; }
    let mut err = crypto_ecdh_encode_key(buf, buf_len, &mut p);
    if err == 0 { err = crypto_kpp_set_secret(tfm, buf, buf_len); }
    kfree_sensitive(buf);
    kfree_sensitive(tmp);
    err
}

pub unsafe fn generate_ecdh_public_key(tfm: *mut crypto_kpp, public_key: *mut u8) -> i32 {
    let mut result = core::mem::MaybeUninit::<crypto_wait>::uninit();
    let mut dst = core::mem::MaybeUninit::<scatterlist>::uninit();
    let tmp = kmalloc(64, GFP_KERNEL);
    if tmp.is_null() { return -12; }
    let req = kpp_request_alloc(tfm, GFP_KERNEL);
    if req.is_null() { kfree(tmp); return -12; }
    sg_init_one(dst.as_mut_ptr() as *mut scatterlist, tmp, 64);
    kpp_request_set_input(req, core::ptr::null_mut(), 0);
    kpp_request_set_output(req, dst.as_mut_ptr() as *mut scatterlist, 64);
    kpp_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, result.as_mut_ptr());
    let err = crypto_wait_req(crypto_kpp_generate_public_key(req), result.as_mut_ptr());
    if err >= 0 {
        swap_digits(tmp as *const u64, public_key as *mut u64, 4);
        swap_digits(tmp.add(32) as *const u64, public_key.add(32) as *mut u64, 4);
    }
    kpp_request_free(req);
    kfree(tmp);
    err
}

pub unsafe fn generate_ecdh_keys(tfm: *mut crypto_kpp, public_key: *mut u8) -> i32 {
    let err = set_ecdh_privkey(tfm, core::ptr::null());
    if err != 0 { return err; }
    generate_ecdh_public_key(tfm, public_key)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
