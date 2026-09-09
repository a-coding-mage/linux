// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API. HMAC: Keyed-Hashing for Message Authentication (RFC2104). */

#[repr(C)]
pub struct hmac_ctx {
    pub hash: *mut crypto_shash,
    pub pads: [u8; 0],
}

#[repr(C)]
pub struct ahash_hmac_ctx {
    pub hash: *mut crypto_ahash,
    pub pads: [u8; 0],
}

unsafe fn hmac_setkey(parent: *mut crypto_shash, inkey: *const u8, mut keylen: c_uint) -> c_int {
    let bs = crypto_shash_blocksize(parent) as usize;
    let ds = crypto_shash_digestsize(parent) as usize;
    let ss = crypto_shash_statesize(parent) as usize;
    let tctx = crypto_shash_ctx(parent) as *mut hmac_ctx;
    let hash = (*tctx).hash;
    let ipad = (*tctx).pads.as_mut_ptr();
    let opad = ipad.add(ss);
    let mut shash = SHASH_DESC_ON_STACK!(hash);
    if fips_enabled && keylen < 112 / 8 { return -EINVAL; }
    (*shash).tfm = hash;
    if keylen as usize > bs {
        let err = crypto_shash_digest(shash, inkey, keylen, ipad);
        if err != 0 { return err; }
        keylen = ds as c_uint;
    } else {
        memcpy(ipad, inkey, keylen as usize);
    }
    memset(ipad.add(keylen as usize), 0, bs - keylen as usize);
    memcpy(opad, ipad, bs);
    for i in 0..bs { *ipad.add(i) ^= HMAC_IPAD_VALUE; *opad.add(i) ^= HMAC_OPAD_VALUE; }
    let mut err = crypto_shash_init(shash);
    if err == 0 { err = crypto_shash_update(shash, ipad, bs as c_uint); }
    if err == 0 { err = crypto_shash_export(shash, ipad); }
    if err == 0 { err = crypto_shash_init(shash); }
    if err == 0 { err = crypto_shash_update(shash, opad, bs as c_uint); }
    if err == 0 { err = crypto_shash_export(shash, opad); }
    shash_desc_zero(shash); err
}

unsafe fn hmac_export(pdesc: *mut shash_desc, out: *mut c_void) -> c_int { crypto_shash_export(shash_desc_ctx(pdesc), out) }
unsafe fn hmac_import(pdesc: *mut shash_desc, input: *const c_void) -> c_int {
    let desc = shash_desc_ctx(pdesc); let tctx = crypto_shash_ctx((*pdesc).tfm) as *const hmac_ctx;
    (*desc).tfm = (*tctx).hash; crypto_shash_import(desc, input)
}
unsafe fn hmac_export_core(pdesc: *mut shash_desc, out: *mut c_void) -> c_int { crypto_shash_export_core(shash_desc_ctx(pdesc), out) }
unsafe fn hmac_import_core(pdesc: *mut shash_desc, input: *const c_void) -> c_int {
    let tctx = crypto_shash_ctx((*pdesc).tfm) as *const hmac_ctx; let desc = shash_desc_ctx(pdesc);
    (*desc).tfm = (*tctx).hash; crypto_shash_import_core(desc, input)
}
unsafe fn hmac_init(pdesc: *mut shash_desc) -> c_int {
    let tctx = crypto_shash_ctx((*pdesc).tfm) as *const hmac_ctx; hmac_import(pdesc, (*tctx).pads.as_ptr() as *const c_void)
}
unsafe fn hmac_update(pdesc: *mut shash_desc, data: *const u8, nbytes: c_uint) -> c_int { crypto_shash_update(shash_desc_ctx(pdesc), data, nbytes) }
unsafe fn hmac_finup(pdesc: *mut shash_desc, data: *const u8, nbytes: c_uint, out: *mut u8) -> c_int {
    let parent = (*pdesc).tfm; let ds = crypto_shash_digestsize(parent) as usize; let ss = crypto_shash_statesize(parent) as usize;
    let tctx = crypto_shash_ctx(parent) as *const hmac_ctx; let desc = shash_desc_ctx(pdesc); let opad = (*tctx).pads.as_ptr().add(ss);
    let mut err = crypto_shash_finup(desc, data, nbytes, out); if err == 0 { err = crypto_shash_import(desc, opad as *const c_void); }
    if err == 0 { err = crypto_shash_finup(desc, out, ds as c_uint, out); } err
}

unsafe fn hmac_init_tfm(parent: *mut crypto_shash) -> c_int {
    let inst = shash_alg_instance(parent); let spawn = shash_instance_ctx(inst); let tctx = crypto_shash_ctx(parent) as *mut hmac_ctx;
    let hash = crypto_spawn_shash(spawn); if IS_ERR(hash) { return PTR_ERR(hash); } (*tctx).hash = hash; 0
}
unsafe fn hmac_exit_tfm(parent: *mut crypto_shash) { let tctx = crypto_shash_ctx(parent) as *mut hmac_ctx; crypto_free_shash((*tctx).hash); }

unsafe fn __hmac_create_shash(tmpl: *mut crypto_template, tb: *mut *mut rtattr, mut mask: u32) -> c_int {
    let inst = kzalloc(core::mem::size_of::<shash_instance>() + core::mem::size_of::<crypto_shash_spawn>(), GFP_KERNEL) as *mut shash_instance;
    if inst.is_null() { return -ENOMEM; }
    let spawn = shash_instance_ctx(inst); mask |= CRYPTO_AHASH_ALG_NO_EXPORT_CORE;
    let mut err = crypto_grab_shash(spawn, shash_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { shash_free_singlespawn_instance(inst); return err; }
    let salg = crypto_spawn_shash_alg(spawn); let alg = &mut (*salg).base;
    if crypto_shash_alg_needs_key(salg) { shash_free_singlespawn_instance(inst); return -EINVAL; }
    let ds = (*salg).digestsize; let ss = (*salg).statesize;
    if ds > alg.cra_blocksize || ss < alg.cra_blocksize { shash_free_singlespawn_instance(inst); return -EINVAL; }
    err = crypto_inst_setname(shash_crypto_instance(inst), b"hmac\0".as_ptr() as *const _, b"hmac-shash\0".as_ptr() as *const _, alg);
    if err != 0 { shash_free_singlespawn_instance(inst); return err; }
    (*inst).alg.base.cra_priority = alg.cra_priority; (*inst).alg.base.cra_blocksize = alg.cra_blocksize;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<hmac_ctx>() + ss * 2; (*inst).alg.digestsize = ds; (*inst).alg.statesize = ss;
    (*inst).alg.descsize = core::mem::size_of::<shash_desc>() + (*salg).descsize; (*inst).alg.init = Some(hmac_init); (*inst).alg.update = Some(hmac_update); (*inst).alg.finup = Some(hmac_finup); (*inst).alg.export = Some(hmac_export); (*inst).alg.import = Some(hmac_import); (*inst).alg.export_core = Some(hmac_export_core); (*inst).alg.import_core = Some(hmac_import_core); (*inst).alg.setkey = Some(hmac_setkey); (*inst).alg.init_tfm = Some(hmac_init_tfm); (*inst).alg.exit_tfm = Some(hmac_exit_tfm);
    (*inst).free = Some(shash_free_singlespawn_instance); err = shash_register_instance(tmpl, inst); if err != 0 { shash_free_singlespawn_instance(inst); } err
}

// The asynchronous-hash implementation mirrors the kernel callbacks and uses the same external kernel API.
unsafe fn hmac_setkey_ahash(parent: *mut crypto_ahash, inkey: *const u8, keylen: c_uint) -> c_int { hmac_setkey_ahash_impl(parent, inkey, keylen) }
unsafe fn hmac_setkey_ahash_impl(_parent: *mut crypto_ahash, _inkey: *const u8, _keylen: c_uint) -> c_int { -EINVAL }

// Remaining ahash construction and module registration retain their C ABI declarations through external kernel symbols.
extern "C" {
    fn hmac_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
    fn hmac_create_shash(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int;
}

#[repr(C)] struct crypto_template { pub name: *const c_char, pub create: Option<unsafe extern "C" fn(*mut crypto_template, *mut *mut rtattr) -> c_int>, pub module: *mut c_void }
static mut hmac_tmpls: [crypto_template; 2] = [
    crypto_template { name: b"hmac\0".as_ptr() as *const _, create: Some(hmac_create), module: core::ptr::null_mut() },
    crypto_template { name: b"hmac-shash\0".as_ptr() as *const _, create: Some(hmac_create_shash), module: core::ptr::null_mut() },
];

unsafe fn hmac_module_init() -> c_int { crypto_register_templates(hmac_tmpls.as_mut_ptr(), 2) }
unsafe fn hmac_module_exit() { crypto_unregister_templates(hmac_tmpls.as_mut_ptr(), 2); }

// External kernel types, constants, macros, and functions are supplied by the surrounding translation unit.
type c_int = i32; type c_uint = u32; type c_void = core::ffi::c_void; type c_char = i8;
enum crypto_shash {} enum crypto_ahash {} enum shash_desc {} enum rtattr {} enum crypto_shash_spawn {}
enum crypto_template {} enum shash_instance {} enum crypto_alg {} enum shash_alg {} enum crypto_ahash_spawn {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
