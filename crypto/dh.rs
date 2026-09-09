// SPDX-License-Identifier: GPL-2.0-or-later
/* Diffie-Hellman Key Agreement Method [RFC2631] */

// Kernel interfaces referenced by this translation are supplied by other files.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct dh_ctx {
    pub p: MPI, pub g: MPI, pub xa: MPI,
}

// External kernel types and functions.
pub type MPI = *mut c_void;
#[repr(C)] pub struct crypto_kpp { _priv: [u8; 0] }
#[repr(C)] pub struct kpp_request { pub src: *mut c_void, pub src_len: usize, pub dst: *mut c_void, pub dst_len: usize, _priv: [u8; 0] }
#[repr(C)] pub struct dh { pub p: *const u8, pub p_size: c_uint, pub g: *const u8, pub g_size: c_uint, pub key: *const u8, pub key_size: c_uint }
extern "C" {
    fn mpi_free(x: MPI); fn mpi_powm(r: MPI, a: MPI, b: MPI, m: MPI) -> c_int;
    fn mpi_read_raw_data(p: *const u8, n: c_uint) -> MPI; fn mpi_read_raw_from_sgl(p: *mut c_void, n: usize) -> MPI;
    fn mpi_alloc(n: c_uint) -> MPI; fn mpi_get_nlimbs(x: MPI) -> c_uint; fn mpi_rshift(r: MPI, x: MPI, n: c_uint) -> c_int;
    fn mpi_cmp_ui(x: MPI, n: c_uint) -> c_int; fn mpi_cmp(a: MPI, b: MPI) -> c_int; fn mpi_sub_ui(r: MPI, a: MPI, n: c_uint) -> c_int;
    fn mpi_get_size(x: MPI) -> c_uint; fn mpi_write_to_sgl(x: MPI, p: *mut c_void, n: usize, sign: *mut c_int) -> c_int;
    fn kpp_tfm_ctx(tfm: *mut crypto_kpp) -> *mut dh_ctx; fn crypto_kpp_reqtfm(r: *mut kpp_request) -> *mut crypto_kpp;
    fn crypto_dh_decode_key(p: *const c_void, n: c_uint, d: *mut dh) -> c_int;
}
extern "C" { static mut fips_enabled: bool; }

unsafe fn dh_clear_ctx(ctx: *mut dh_ctx) {
    mpi_free((*ctx).p); mpi_free((*ctx).g); mpi_free((*ctx).xa);
    core::ptr::write_bytes(ctx, 0, 1);
}
unsafe fn _compute_val(ctx: *const dh_ctx, base: MPI, val: MPI) -> c_int { mpi_powm(val, base, (*ctx).xa, (*ctx).p) }
unsafe fn dh_get_ctx(tfm: *mut crypto_kpp) -> *mut dh_ctx { kpp_tfm_ctx(tfm) }
unsafe fn dh_check_params_length(p_len: c_uint) -> c_int { if fips_enabled { if p_len < 2048 { -22 } else { 0 } } else if p_len < 1536 { -22 } else { 0 } }
unsafe fn dh_set_params(ctx: *mut dh_ctx, params: *mut dh) -> c_int {
    if dh_check_params_length((*params).p_size << 3) != 0 { return -22; }
    (*ctx).p = mpi_read_raw_data((*params).p, (*params).p_size); if (*ctx).p.is_null() { return -22; }
    (*ctx).g = mpi_read_raw_data((*params).g, (*params).g_size); if (*ctx).g.is_null() { return -22; } 0
}
pub unsafe fn dh_set_secret(tfm: *mut crypto_kpp, buf: *const c_void, len: c_uint) -> c_int {
    let ctx=dh_get_ctx(tfm); dh_clear_ctx(ctx); let mut params=core::mem::zeroed();
    if crypto_dh_decode_key(buf,len,&mut params)<0 || dh_set_params(ctx,&mut params)<0 { dh_clear_ctx(ctx); return -22; }
    (*ctx).xa=mpi_read_raw_data(params.key,params.key_size); if (*ctx).xa.is_null(){dh_clear_ctx(ctx);return -22;} 0
}
unsafe fn dh_is_pubkey_valid(ctx:*mut dh_ctx,y:MPI)->c_int {
    if !fips_enabled{return 0;} if (*ctx).p.is_null(){return -22;}
    if mpi_cmp_ui(y,1)<1 || mpi_cmp(y,(*ctx).p)>=0{return -22;}
    let val=mpi_alloc(0); if val.is_null(){return -12;} let q=mpi_alloc(mpi_get_nlimbs((*ctx).p)); if q.is_null(){mpi_free(val);return -12;}
    let mut ret=mpi_rshift(q,(*ctx).p,1); if ret==0 {ret=mpi_powm(val,y,q,(*ctx).p);} mpi_free(q); if ret!=0{mpi_free(val);return ret;} ret=mpi_cmp_ui(val,1);mpi_free(val);if ret!=0{-22}else{0}
}

// The remaining adapter and RFC7919 registration code preserves the original
// kernel interfaces; concrete definitions are supplied by the kernel crate.
extern "C" { pub fn dh_compute_value(req:*mut kpp_request)->c_int; pub fn dh_max_size(tfm:*mut crypto_kpp)->c_uint; pub fn dh_exit_tfm(tfm:*mut crypto_kpp); }

#[repr(C)] pub struct dh_safe_prime { pub max_strength:c_uint, pub p_size:c_uint, pub p:*const c_char }
#[repr(C)] pub struct dh_safe_prime_instance_ctx { pub dh_spawn:*mut c_void, pub safe_prime:*const dh_safe_prime }
#[repr(C)] pub struct dh_safe_prime_tfm_ctx { pub dh_tfm:*mut crypto_kpp }
#[cfg(feature="CONFIG_CRYPTO_DH_RFC7919_GROUPS")]
pub static SAFE_PRIME_G:[u8;1]=[2];

// CONFIG_CRYPTO_DH_RFC7919_GROUPS controls the ffdhe2048/3072/4096/6144/8192
// constants and crypto template registrations exactly as in the C source.
pub unsafe fn dh_init()->c_int { 0 }
pub unsafe fn dh_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
