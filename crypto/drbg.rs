/* DRBG: Deterministic Random Bits Generator
 * Implementation of the HMAC SHA-512 DRBG from NIST SP800-90A
 *
 * This is a source-level translation of drbg.c.  Kernel-provided types and
 * functions are intentionally left as external dependencies.
 */

const DRBG_STATE_LEN: usize = SHA512_DIGEST_SIZE;
const DRBG_SEC_STRENGTH: usize = SHA512_DIGEST_SIZE / 2;
const DRBG_MAX_REQUESTS: usize = 4096;
const DRBG_MAX_REQUEST_BYTES: usize = 1 << 16;
const DRBG_MAX_ADDTL_BYTES: u32 = u32::MAX - 1;

#[repr(C)]
struct drbg_state {
    drbg_mutex: mutex,
    v: [u8; DRBG_STATE_LEN],
    key: hmac_sha512_key,
    reseed_ctr: usize,
    instantiated: bool,
    jent: *mut crypto_rng,
    test_entropy: *const u8,
    test_entropylen: usize,
}

unsafe fn drbg_hmac_update(drbg: *mut drbg_state, data1: *const u8, data1_len: usize,
                           data2: *const u8, data2_len: usize) {
    let mut hmac_ctx: hmac_sha512_ctx = core::mem::zeroed();
    let mut new_key = [0u8; DRBG_STATE_LEN];
    for i in 0u8..2 {
        hmac_sha512_init(&mut hmac_ctx, &(*drbg).key);
        hmac_sha512_update(&mut hmac_ctx, (*drbg).v.as_ptr(), DRBG_STATE_LEN);
        hmac_sha512_update(&mut hmac_ctx, &i, 1);
        hmac_sha512_update(&mut hmac_ctx, data1, data1_len);
        hmac_sha512_update(&mut hmac_ctx, data2, data2_len);
        hmac_sha512_final(&mut hmac_ctx, new_key.as_mut_ptr());
        hmac_sha512_preparekey(&mut (*drbg).key, new_key.as_ptr(), DRBG_STATE_LEN);
        hmac_sha512(&(*drbg).key, (*drbg).v.as_ptr(), DRBG_STATE_LEN, (*drbg).v.as_mut_ptr());
        if data1_len == 0 && data2_len == 0 { break; }
    }
    memzero_explicit(new_key.as_mut_ptr(), new_key.len());
}

unsafe fn drbg_hmac_generate(drbg: *mut drbg_state, mut out: *mut u8, mut outlen: usize,
                             addtl1: *const u8, addtl1_len: usize) {
    let mut addtl2 = [0u8; 32];
    let mut addtl2_len = 0usize;
    if (*drbg).test_entropylen == 0 {
        get_random_bytes(addtl2.as_mut_ptr(), addtl2.len());
        addtl2_len = addtl2.len();
    }
    if addtl1_len != 0 || addtl2_len != 0 { drbg_hmac_update(drbg, addtl1, addtl1_len, addtl2.as_ptr(), addtl2_len); }
    while outlen != 0 {
        let n = core::cmp::min(DRBG_STATE_LEN, outlen);
        hmac_sha512(&(*drbg).key, (*drbg).v.as_ptr(), DRBG_STATE_LEN, (*drbg).v.as_mut_ptr());
        core::ptr::copy_nonoverlapping((*drbg).v.as_ptr(), out, n);
        out = out.add(n); outlen -= n;
    }
    drbg_hmac_update(drbg, addtl1, addtl1_len, addtl2.as_ptr(), addtl2_len);
    memzero_explicit(addtl2.as_mut_ptr(), addtl2.len());
}

unsafe fn drbg_seed(drbg: *mut drbg_state, pers: *const u8, pers_len: usize, reseed: bool) -> i32 {
    let mut ret = 0i32;
    let mut entropy_buf = [0u8; (32 + 16) * 2];
    let (entropy, entropylen): (*const u8, usize);
    if pers_len > DRBG_MAX_ADDTL_BYTES as usize { return -EINVAL; }
    if (*drbg).test_entropylen != 0 {
        entropy = (*drbg).test_entropy; entropylen = (*drbg).test_entropylen;
    } else {
        entropy = entropy_buf.as_ptr();
        entropylen = if !reseed { ((DRBG_SEC_STRENGTH + 1) / 2) * 3 } else { DRBG_SEC_STRENGTH };
        get_random_bytes(entropy_buf.as_mut_ptr(), entropylen);
        if !(*drbg).jent.is_null() {
            ret = crypto_rng_get_bytes((*drbg).jent, entropy_buf.as_mut_ptr().add(entropylen), entropylen);
            if fips_enabled && ret != 0 && (!reseed || ret != -EAGAIN) { memzero_explicit(entropy_buf.as_mut_ptr(), entropy_buf.len()); return ret; }
        }
    }
    drbg_hmac_update(drbg, entropy, entropylen, pers, pers_len);
    (*drbg).reseed_ctr = 1;
    memzero_explicit(entropy_buf.as_mut_ptr(), entropy_buf.len());
    0
}

unsafe fn drbg_generate(drbg: *mut drbg_state, out: *mut u8, outlen: usize,
                        mut addtl: *const u8, mut addtl_len: usize) -> i32 {
    if !(*drbg).instantiated || out.is_null() || outlen == 0 || (addtl.is_null() && addtl_len != 0) { return -EINVAL; }
    if outlen > DRBG_MAX_REQUEST_BYTES || addtl_len > DRBG_MAX_ADDTL_BYTES as usize { return -EINVAL; }
    if (*drbg).reseed_ctr > DRBG_MAX_REQUESTS {
        let err = drbg_seed(drbg, addtl, addtl_len, true); if err != 0 { return err; }
        addtl = core::ptr::null(); addtl_len = 0;
    }
    drbg_hmac_generate(drbg, out, outlen, addtl, addtl_len);
    (*drbg).reseed_ctr += 1;
    0
}

unsafe fn drbg_kcapi_init(tfm: *mut crypto_tfm) -> i32 { let drbg = crypto_tfm_ctx(tfm); mutex_init(&mut (*drbg).drbg_mutex); 0 }

unsafe fn drbg_kcapi_set_entropy(tfm: *mut crypto_rng, data: *const u8, len: u32) {
    let drbg = crypto_rng_ctx(tfm); mutex_lock(&mut (*drbg).drbg_mutex); (*drbg).test_entropy = data; (*drbg).test_entropylen = len as usize; mutex_unlock(&mut (*drbg).drbg_mutex);
}

unsafe fn drbg_kcapi_seed(tfm: *mut crypto_rng, pers: *const u8, pers_len: u32) -> i32 {
    let drbg = crypto_rng_ctx(tfm);
    if (*drbg).instantiated { return drbg_seed(drbg, pers, pers_len as usize, true); }
    (*drbg).v.fill(1); let initial_key = [0u8; DRBG_STATE_LEN];
    hmac_sha512_preparekey(&mut (*drbg).key, initial_key.as_ptr(), DRBG_STATE_LEN);
    if (*drbg).test_entropylen == 0 { (*drbg).jent = crypto_alloc_rng(b"jitterentropy_rng\0".as_ptr() as *const i8, 0, 0); }
    let ret = drbg_seed(drbg, pers, pers_len as usize, false); if ret != 0 { return ret; }
    (*drbg).instantiated = true; 0
}

unsafe fn drbg_kcapi_generate(tfm: *mut crypto_rng, src: *const u8, slen: u32, mut dst: *mut u8, mut dlen: u32) -> i32 {
    let drbg = crypto_rng_ctx(tfm);
    while dlen != 0 { let n = core::cmp::min(dlen as usize, DRBG_MAX_REQUEST_BYTES) as u32; mutex_lock(&mut (*drbg).drbg_mutex); let err = drbg_generate(drbg, dst, n as usize, src, slen as usize); mutex_unlock(&mut (*drbg).drbg_mutex); if err < 0 { return err; } dst = dst.add(n as usize); dlen -= n; }
    0
}

unsafe fn drbg_kcapi_exit(tfm: *mut crypto_tfm) { let drbg = crypto_tfm_ctx(tfm); crypto_free_rng((*drbg).jent); memzero_explicit(drbg as *mut u8, core::mem::size_of::<drbg_state>()); }

unsafe fn drbg_healthcheck_sanity() -> i32 {
    const OUTBUFLEN: usize = 16;
    if !fips_enabled { return 0; }
    let mut drbg: *mut drbg_state = kzalloc_obj();
    if drbg.is_null() { return -ENOMEM; }
    mutex_init(&mut (*drbg).drbg_mutex);
    (*drbg).instantiated = true;
    let mut buf = [0u8; OUTBUFLEN];
    let ret = drbg_generate(drbg, buf.as_mut_ptr(), OUTBUFLEN, buf.as_ptr(), DRBG_MAX_ADDTL_BYTES as usize + 1);
    BUG_ON(ret == 0);
    let ret = drbg_generate(drbg, buf.as_mut_ptr(), DRBG_MAX_REQUEST_BYTES + 1, core::ptr::null(), 0);
    BUG_ON(ret == 0);
    let ret = drbg_seed(drbg, buf.as_ptr(), DRBG_MAX_ADDTL_BYTES as usize + 1, false);
    BUG_ON(ret == 0);
    kfree(drbg as *mut core::ffi::c_void);
    0
}

// The following aggregate is initialized by the kernel crypto API bindings.
// Its fields correspond to the C designated initializer in drbg.c.
extern "C" {
    static mut drbg_alg: rng_alg;
}

unsafe fn drbg_init() -> i32 {
    let ret = drbg_healthcheck_sanity();
    if ret != 0 { return ret; }
    if fips_enabled { (*core::ptr::addr_of_mut!(drbg_alg)).base.cra_priority += 2000; }
    crypto_register_rng(core::ptr::addr_of_mut!(drbg_alg))
}

unsafe fn drbg_exit() { crypto_unregister_rng(core::ptr::addr_of_mut!(drbg_alg)); }

// module_init(drbg_init); module_exit(drbg_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Stephan Mueller <smueller@chronox.de>");
// MODULE_DESCRIPTION("NIST SP800-90A Deterministic Random Bit Generator (DRBG)");
// MODULE_ALIAS_CRYPTO("stdrng");
// MODULE_ALIAS_CRYPTO("drbg_nopr_hmac_sha512");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
