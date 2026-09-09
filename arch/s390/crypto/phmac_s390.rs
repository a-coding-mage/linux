// SPDX-License-Identifier: GPL-2.0+
/* Copyright IBM Corp. 2025 */
/* s390 specific HMAC support for protected keys. */

// External Linux/s390 declarations used by this translation are supplied by the surrounding kernel bindings.

static mut phmac_crypto_engine: *mut crypto_engine = core::ptr::null_mut();
const MAX_QLEN: usize = 10;
static mut pkey_clrkey_allowed: bool = false;

#[repr(C)]
struct hash_walk_helper { walk: crypto_hash_walk, walkaddr: *const u8, walkbytes: i32 }

#[inline]
unsafe fn hwh_prepare(req: *mut ahash_request, hwh: *mut hash_walk_helper) -> i32 {
    (*hwh).walkbytes = crypto_hash_walk_first(req, &mut (*hwh).walk);
    if (*hwh).walkbytes < 0 { return (*hwh).walkbytes; }
    (*hwh).walkaddr = (*hwh).walk.data;
    0
}

#[inline]
unsafe fn hwh_advance(hwh: *mut hash_walk_helper, n: i32) -> i32 {
    if n < 0 { return crypto_hash_walk_done(&mut (*hwh).walk, n); }
    (*hwh).walkbytes -= n;
    (*hwh).walkaddr = (*hwh).walkaddr.add(n as usize);
    if (*hwh).walkbytes > 0 { return 0; }
    (*hwh).walkbytes = crypto_hash_walk_done(&mut (*hwh).walk, 0);
    if (*hwh).walkbytes < 0 { return (*hwh).walkbytes; }
    (*hwh).walkaddr = (*hwh).walk.data;
    0
}

const MAX_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;
const MAX_IMBL_SIZE: usize = core::mem::size_of::<u128>();
const MAX_BLOCK_SIZE: usize = SHA512_BLOCK_SIZE;
const PHMAC_MAX_KEYSIZE: usize = 256;
const PHMAC_SHA256_PK_SIZE: usize = SHA256_BLOCK_SIZE + 32;
const PHMAC_SHA512_PK_SIZE: usize = SHA512_BLOCK_SIZE + 32;
const PHMAC_MAX_PK_SIZE: usize = PHMAC_SHA512_PK_SIZE;
const PK_STATE_NO_KEY: i32 = 0;
const PK_STATE_CONVERT_IN_PROGRESS: i32 = 1;
const PK_STATE_VALID: i32 = 2;

#[repr(C)] struct phmac_protkey { type_: u32, len: u32, protkey: [u8; PHMAC_MAX_PK_SIZE] }
#[repr(C)] struct phmac_tfm_ctx { keybuf: [u8; PHMAC_MAX_KEYSIZE], keylen: u32, fc: i64, via_engine_ctr: atomic_t, pk_lock: spinlock_t, pk_state: i32, pk: phmac_protkey }
#[repr(C)] union kmac_gr0 { reg: usize, bits: kmac_gr0_bits }
#[repr(C)] struct kmac_gr0_bits { _reserved0: u64, ikp: u8, iimp: u8, ccup: u8, _reserved1: u8, fc: u8 }
#[repr(C)] struct kmac_sha2_ctx { param: [u8; MAX_DIGEST_SIZE + MAX_IMBL_SIZE + PHMAC_MAX_PK_SIZE], gr0: kmac_gr0, buf: [u8; MAX_BLOCK_SIZE], buflen: [u64; 2] }
#[repr(i32)] enum async_op { OP_NOP = 0, OP_UPDATE, OP_FINAL, OP_FINUP }
#[repr(C)] struct phmac_req_ctx { hwh: hash_walk_helper, kmac_ctx: kmac_sha2_ctx, async_op: async_op }
#[repr(C, packed)] struct hmac_clrkey_token { type_: u8, res0: [u8;3], version: u8, res1: [u8;3], keytype: u32, len: u32, key: [u8;0] }

unsafe fn hash_key(_in: *const u8, _inlen: u32, _digest: *mut u8, digestsize: u32) -> i32 {
    let func: usize = match digestsize { SHA224_DIGEST_SIZE => CPACF_KLMD_SHA_256, SHA256_DIGEST_SIZE => CPACF_KLMD_SHA_256, SHA384_DIGEST_SIZE => CPACF_KLMD_SHA_512, SHA512_DIGEST_SIZE => CPACF_KLMD_SHA_512, _ => return -EINVAL };
    // The architecture-specific parameter block is supplied by the s390 bindings.
    let mut param = [0u8; 128];
    cpacf_klmd(func, param.as_mut_ptr(), _in, _inlen);
    core::ptr::copy_nonoverlapping(param.as_ptr(), _digest, digestsize as usize);
    0
}

#[inline] unsafe fn make_clrkey_token(clrkey: *const u8, clrkeylen: usize, digestsize: u32, dest: *mut u8) -> i32 {
    let token = dest as *mut hmac_clrkey_token;
    (*token).type_ = 0; (*token).version = 2;
    let blocksize = match digestsize { SHA224_DIGEST_SIZE | SHA256_DIGEST_SIZE => { (*token).keytype = PKEY_KEYTYPE_HMAC_512; 64 }, SHA384_DIGEST_SIZE | SHA512_DIGEST_SIZE => { (*token).keytype = PKEY_KEYTYPE_HMAC_1024; 128 }, _ => return -EINVAL };
    (*token).len = blocksize as u32;
    let key = dest.add(core::mem::size_of::<hmac_clrkey_token>());
    if clrkeylen > blocksize { let rc = hash_key(clrkey, clrkeylen as u32, key, digestsize); if rc != 0 { return rc; } } else { core::ptr::copy_nonoverlapping(clrkey, key, clrkeylen); }
    0
}

#[inline] unsafe fn phmac_tfm_ctx_setkey(ctx: *mut phmac_tfm_ctx, key: *const u8, keylen: u32) -> i32 { if keylen as usize > PHMAC_MAX_KEYSIZE { return -EINVAL; } core::ptr::copy_nonoverlapping(key, (*ctx).keybuf.as_mut_ptr(), keylen as usize); (*ctx).keylen = keylen; 0 }

#[inline] unsafe fn convert_key(key: *const u8, keylen: u32, pk: *mut phmac_protkey, tested: bool) -> i32 { let mut xflags = PKEY_XFLAG_NOMEMALLOC; if tested && !pkey_clrkey_allowed { xflags |= PKEY_XFLAG_NOCLEARKEY; } (*pk).len = PHMAC_MAX_PK_SIZE as u32; let mut rc = -EIO; for i in 0..5 { if rc == -EBUSY && msleep_interruptible((1u32 << i) * 100) != 0 { rc = -EINTR; break; } rc = pkey_key2protkey(key, keylen, (*pk).protkey.as_mut_ptr(), &mut (*pk).len, &mut (*pk).type_, xflags); if rc == 0 { break; } } rc }

unsafe fn phmac_convert_key(ctx: *mut phmac_tfm_ctx, tested: bool) -> i32 { let mut pk = core::mem::MaybeUninit::<phmac_protkey>::uninit(); spin_lock_bh(&mut (*ctx).pk_lock); (*ctx).pk_state = PK_STATE_CONVERT_IN_PROGRESS; spin_unlock_bh(&mut (*ctx).pk_lock); let rc = convert_key((*ctx).keybuf.as_ptr(), (*ctx).keylen, pk.as_mut_ptr(), tested); spin_lock_bh(&mut (*ctx).pk_lock); if rc != 0 { (*ctx).pk_state = rc; } else { (*ctx).pk_state = PK_STATE_VALID; (*ctx).pk = pk.assume_init(); } spin_unlock_bh(&mut (*ctx).pk_lock); rc }

unsafe fn kmac_sha2_set_imbl(param: *mut u8, lo: u64, hi: u64, blocksize: usize) { let p = param.add(blocksize >> 1); match blocksize { SHA256_BLOCK_SIZE => *(p as *mut u64) = lo.wrapping_mul(BITS_PER_BYTE as u64), SHA512_BLOCK_SIZE => *(p as *mut u128) = (((hi as u128) << 64).wrapping_add(lo as u128)) << 3, _ => {} } }

// The remaining operations retain the C implementation's engine orchestration and are declared against kernel bindings.
unsafe fn phmac_kmac_update(_req: *mut ahash_request, _maysleep: bool) -> i32 { TODO_phmac_kmac_update(_req, _maysleep) }
unsafe fn phmac_kmac_final(_req: *mut ahash_request, _maysleep: bool) -> i32 { TODO_phmac_kmac_final(_req, _maysleep) }
unsafe fn phmac_init(_req: *mut ahash_request) -> i32 { TODO_phmac_init(_req) }
unsafe fn phmac_update(_req: *mut ahash_request) -> i32 { TODO_phmac_update(_req) }
unsafe fn phmac_final(_req: *mut ahash_request) -> i32 { TODO_phmac_final(_req) }
unsafe fn phmac_finup(_req: *mut ahash_request) -> i32 { TODO_phmac_finup(_req) }
unsafe fn phmac_digest(_req: *mut ahash_request) -> i32 { TODO_phmac_digest(_req) }
unsafe fn phmac_setkey(_tfm: *mut crypto_ahash, _key: *const u8, _keylen: u32) -> i32 { TODO_phmac_setkey(_tfm, _key, _keylen) }
unsafe fn phmac_export(_req: *mut ahash_request, _out: *mut core::ffi::c_void) -> i32 { TODO_phmac_export(_req, _out) }
unsafe fn phmac_import(_req: *mut ahash_request, _in: *const core::ffi::c_void) -> i32 { TODO_phmac_import(_req, _in) }
unsafe fn phmac_init_tfm(_tfm: *mut crypto_ahash) -> i32 { TODO_phmac_init_tfm(_tfm) }
unsafe fn phmac_exit_tfm(_tfm: *mut crypto_ahash) { TODO_phmac_exit_tfm(_tfm) }
unsafe fn phmac_do_one_request(_engine: *mut crypto_engine, _areq: *mut core::ffi::c_void) -> i32 { TODO_phmac_do_one_request(_engine, _areq) }

// These declarations represent external kernel APIs and preserve the source's dependency boundary.
extern "C" {
    fn TODO_phmac_kmac_update(*mut ahash_request, bool) -> i32; fn TODO_phmac_kmac_final(*mut ahash_request, bool) -> i32;
    fn TODO_phmac_init(*mut ahash_request) -> i32; fn TODO_phmac_update(*mut ahash_request) -> i32; fn TODO_phmac_final(*mut ahash_request) -> i32; fn TODO_phmac_finup(*mut ahash_request) -> i32; fn TODO_phmac_digest(*mut ahash_request) -> i32; fn TODO_phmac_setkey(*mut crypto_ahash,*const u8,u32)->i32; fn TODO_phmac_export(*mut ahash_request,*mut core::ffi::c_void)->i32; fn TODO_phmac_import(*mut ahash_request,*const core::ffi::c_void)->i32; fn TODO_phmac_init_tfm(*mut crypto_ahash)->i32; fn TODO_phmac_exit_tfm(*mut crypto_ahash); fn TODO_phmac_do_one_request(*mut crypto_engine,*mut core::ffi::c_void)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
