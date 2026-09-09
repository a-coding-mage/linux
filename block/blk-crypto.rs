// SPDX-License-Identifier: GPL-2.0
/* Copyright 2019 Google LLC */
/* Refer to Documentation/block/inline-encryption.rst for detailed explanation. */

// Includes and symbols are supplied by the surrounding kernel translation unit.

pub static blk_crypto_modes: [blk_crypto_mode; BLK_ENCRYPTION_MODE_MAX as usize] = [
    blk_crypto_mode { name: "AES-256-XTS", cipher_str: "xts(aes)", keysize: 64, security_strength: 32, ivsize: 16 },
    blk_crypto_mode { name: "AES-128-CBC-ESSIV", cipher_str: "essiv(cbc(aes),sha256)", keysize: 16, security_strength: 16, ivsize: 16 },
    blk_crypto_mode { name: "Adiantum", cipher_str: "adiantum(xchacha12,aes)", keysize: 32, security_strength: 32, ivsize: 32 },
    blk_crypto_mode { name: "SM4-XTS", cipher_str: "xts(sm4)", keysize: 32, security_strength: 16, ivsize: 16 },
];

static mut num_prealloc_crypt_ctxs: i32 = 128;
static mut bio_crypt_ctx_cache: *mut kmem_cache = core::ptr::null_mut();
static mut bio_crypt_ctx_pool: *mut mempool_t = core::ptr::null_mut();

unsafe fn bio_crypt_ctx_init() -> i32 {
    bio_crypt_ctx_cache = KMEM_CACHE(bio_crypt_ctx, 0);
    if bio_crypt_ctx_cache.is_null() { panic!("Failed to allocate mem for bio crypt ctxs\n"); }
    bio_crypt_ctx_pool = mempool_create_slab_pool(num_prealloc_crypt_ctxs, bio_crypt_ctx_cache);
    if bio_crypt_ctx_pool.is_null() { panic!("Failed to allocate mem for bio crypt ctxs\n"); }
    BUILD_BUG_ON(BLK_ENCRYPTION_MODE_INVALID != 0);
    for i in 0..BLK_ENCRYPTION_MODE_MAX as usize {
        BUG_ON(blk_crypto_modes[i].keysize > BLK_CRYPTO_MAX_RAW_KEY_SIZE);
        BUG_ON(blk_crypto_modes[i].security_strength > blk_crypto_modes[i].keysize);
        BUG_ON(blk_crypto_modes[i].ivsize > BLK_CRYPTO_MAX_IV_SIZE);
    }
    0
}

pub unsafe fn bio_crypt_set_ctx(bio: *mut bio, key: *const blk_crypto_key,
                                dun: *const u64, gfp_mask: gfp_t) {
    WARN_ON_ONCE((gfp_mask & __GFP_DIRECT_RECLAIM) == 0);
    let bc = mempool_alloc(bio_crypt_ctx_pool, gfp_mask) as *mut bio_crypt_ctx;
    (*bc).bc_key = key;
    core::ptr::copy_nonoverlapping(dun, (*bc).bc_dun.as_mut_ptr(), BLK_CRYPTO_DUN_ARRAY_SIZE);
    (*bio).bi_crypt_context = bc;
}

pub unsafe fn __bio_crypt_free_ctx(bio: *mut bio) {
    mempool_free((*bio).bi_crypt_context as *mut _, bio_crypt_ctx_pool);
    (*bio).bi_crypt_context = core::ptr::null_mut();
}

pub unsafe fn __bio_crypt_clone(dst: *mut bio, src: *mut bio, gfp_mask: gfp_t) -> i32 {
    (*dst).bi_crypt_context = mempool_alloc(bio_crypt_ctx_pool, gfp_mask) as *mut _;
    if (*dst).bi_crypt_context.is_null() { return -ENOMEM; }
    *(*dst).bi_crypt_context = *(*src).bi_crypt_context;
    0
}

/* Increments @dun by @inc, treating @dun as a multi-limb integer. */
pub unsafe fn bio_crypt_dun_increment(dun: *mut u64, mut inc: u32) {
    let mut i = 0;
    while inc != 0 && i < BLK_CRYPTO_DUN_ARRAY_SIZE {
        *dun.add(i) = (*dun.add(i)).wrapping_add(inc as u64);
        inc = if *dun.add(i) < inc as u64 { 1 } else { 0 };
        i += 1;
    }
}

pub unsafe fn __bio_crypt_advance(bio: *mut bio, bytes: u32) {
    let bc = (*bio).bi_crypt_context;
    bio_crypt_dun_increment((*bc).bc_dun.as_mut_ptr(), bytes >> (*(*bc).bc_key).data_unit_size_bits);
}

pub unsafe fn bio_crypt_dun_is_contiguous(bc: *const bio_crypt_ctx, bytes: u32, next: *const u64) -> bool {
    let mut carry = bytes >> (*(*bc).bc_key).data_unit_size_bits;
    for i in 0..BLK_CRYPTO_DUN_ARRAY_SIZE {
        let sum = (*bc).bc_dun[i].wrapping_add(carry as u64);
        if sum != *next.add(i) { return false; }
        carry = if sum < carry as u64 { 1 } else { 0 };
    }
    carry == 0
}

unsafe fn bio_crypt_ctx_compatible(bc1: *mut bio_crypt_ctx, bc2: *mut bio_crypt_ctx) -> bool {
    if bc1.is_null() { return bc2.is_null(); }
    !bc2.is_null() && (*bc1).bc_key == (*bc2).bc_key
}

pub unsafe fn bio_crypt_rq_ctx_compatible(rq: *mut request, bio: *mut bio) -> bool {
    bio_crypt_ctx_compatible((*rq).crypt_ctx, (*bio).bi_crypt_context)
}

pub unsafe fn bio_crypt_ctx_mergeable(bc1: *mut bio_crypt_ctx, bytes: u32, bc2: *mut bio_crypt_ctx) -> bool {
    bio_crypt_ctx_compatible(bc1, bc2) && (bc1.is_null() || bio_crypt_dun_is_contiguous(bc1, bytes, (*bc2).bc_dun.as_ptr()))
}

pub unsafe fn __blk_crypto_rq_get_keyslot(rq: *mut request) -> blk_status_t {
    blk_crypto_get_keyslot((*(*rq).q).crypto_profile, (*(*rq).crypt_ctx).bc_key, &mut (*rq).crypt_keyslot)
}
pub unsafe fn __blk_crypto_rq_put_keyslot(rq: *mut request) { blk_crypto_put_keyslot((*rq).crypt_keyslot); (*rq).crypt_keyslot = core::ptr::null_mut(); }
pub unsafe fn __blk_crypto_free_request(rq: *mut request) {
    if WARN_ON_ONCE(!(*rq).crypt_keyslot.is_null()) { __blk_crypto_rq_put_keyslot(rq); }
    mempool_free((*rq).crypt_ctx as *mut _, bio_crypt_ctx_pool); (*rq).crypt_ctx = core::ptr::null_mut();
}

pub unsafe fn __blk_crypto_submit_bio(bio: *mut bio) -> bool {
    let key = (*(*bio).bi_crypt_context).bc_key;
    if WARN_ON_ONCE(!bio_has_data(bio)) { bio_io_error(bio); return false; }
    if !blk_crypto_config_supported_natively((*bio).bi_bdev, &(*key).crypto_cfg) {
        // Build-time fallback condition preserved from CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK.
        return blk_crypto_fallback_bio_prep(bio);
    }
    true
}

pub unsafe fn __blk_crypto_rq_bio_prep(rq: *mut request, bio: *mut bio, gfp_mask: gfp_t) -> i32 {
    if (*rq).crypt_ctx.is_null() { (*rq).crypt_ctx = mempool_alloc(bio_crypt_ctx_pool, gfp_mask) as *mut _; if (*rq).crypt_ctx.is_null() { return -ENOMEM; } }
    *(*rq).crypt_ctx = *(*bio).bi_crypt_context; 0
}

pub unsafe fn blk_crypto_init_key(blk_key: *mut blk_crypto_key, key_bytes: *const u8, key_size: usize, key_type: blk_crypto_key_type, crypto_mode: blk_crypto_mode_num, dun_bytes: u32, data_unit_size: u32, flags: i32) -> i32 {
    core::ptr::write_bytes(blk_key, 0, 1);
    if crypto_mode as usize >= blk_crypto_modes.len() || (flags & !BLK_CRYPTO_CFG_ALLOW_HW) != 0 { return -EINVAL; }
    let mode = &blk_crypto_modes[crypto_mode as usize];
    match key_type { BLK_CRYPTO_KEY_TYPE_RAW if key_size != mode.keysize => return -EINVAL, BLK_CRYPTO_KEY_TYPE_HW_WRAPPED if (key_size < mode.security_strength || key_size > BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE || flags & BLK_CRYPTO_CFG_ALLOW_HW == 0) => return -EINVAL, BLK_CRYPTO_KEY_TYPE_RAW | BLK_CRYPTO_KEY_TYPE_HW_WRAPPED => (), _ => return -EINVAL }
    if dun_bytes == 0 || dun_bytes > mode.ivsize || !is_power_of_2(data_unit_size) { return -EINVAL; }
    (*blk_key).crypto_cfg.crypto_mode = crypto_mode; (*blk_key).crypto_cfg.dun_bytes = dun_bytes; (*blk_key).crypto_cfg.data_unit_size = data_unit_size; (*blk_key).crypto_cfg.key_type = key_type; (*blk_key).crypto_cfg.flags = flags; (*blk_key).data_unit_size_bits = ilog2(data_unit_size); (*blk_key).size = key_size;
    core::ptr::copy_nonoverlapping(key_bytes, (*blk_key).bytes.as_mut_ptr(), key_size); 0
}

pub unsafe fn blk_crypto_config_supported_natively(bdev: *mut block_device, cfg: *const blk_crypto_config) -> bool {
    let p = (*bdev_get_queue(bdev)).crypto_profile; if p.is_null() || (*cfg).flags & BLK_CRYPTO_CFG_ALLOW_HW == 0 { return false; }
    ((*p).modes_supported[(*cfg).crypto_mode as usize] & (*cfg).data_unit_size) != 0 && (*p).max_dun_bytes_supported >= (*cfg).dun_bytes && ((*p).key_types_supported & (*cfg).key_type) != 0
}

pub unsafe fn blk_crypto_start_using_key(bdev: *mut block_device, key: *const blk_crypto_key) -> i32 { if blk_crypto_config_supported_natively(bdev, &(*key).crypto_cfg) { 0 } else if (*key).crypto_cfg.key_type != BLK_CRYPTO_KEY_TYPE_RAW { -EOPNOTSUPP } else { blk_crypto_fallback_start_using_mode((*key).crypto_cfg.crypto_mode) } }
pub unsafe fn blk_crypto_evict_key(bdev: *mut block_device, key: *const blk_crypto_key) { let q = bdev_get_queue(bdev); let err = if blk_crypto_config_supported_natively(bdev, &(*key).crypto_cfg) { __blk_crypto_evict_key((*q).crypto_profile, key) } else { blk_crypto_fallback_evict_key(key) }; if err != 0 { pr_warn_ratelimited!("error %d evicting key", err); } }

unsafe fn blk_crypto_ioctl_import_key(profile: *mut blk_crypto_profile, argp: *mut core::ffi::c_void) -> i32 {
    let mut arg: blk_crypto_import_key_arg = core::mem::zeroed();
    let mut raw_key = [0u8; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize];
    let mut lt_key = [0u8; BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE as usize];
    if copy_from_user(&mut arg, argp, core::mem::size_of_val(&arg)) != 0 { return -EFAULT; }
    if memchr_inv(arg.reserved.as_ptr(), 0, arg.reserved.len()) || arg.raw_key_size < 16 || arg.raw_key_size > raw_key.len() { return -EINVAL; }
    let mut ret = if copy_from_user(raw_key.as_mut_ptr(), u64_to_user_ptr(arg.raw_key_ptr), arg.raw_key_size) != 0 { -EFAULT } else { blk_crypto_import_key(profile, raw_key.as_ptr(), arg.raw_key_size, lt_key.as_mut_ptr()) };
    if ret >= 0 { if ret > arg.lt_key_size { ret = -EOVERFLOW; } else { arg.lt_key_size = ret; if copy_to_user(u64_to_user_ptr(arg.lt_key_ptr), lt_key.as_ptr(), arg.lt_key_size) != 0 || copy_to_user(argp, &arg, core::mem::size_of_val(&arg)) != 0 { ret = -EFAULT; } else { ret = 0; } } }
    memzero_explicit(raw_key.as_mut_ptr(), raw_key.len()); memzero_explicit(lt_key.as_mut_ptr(), lt_key.len()); ret
}

unsafe fn blk_crypto_ioctl_generate_key(profile: *mut blk_crypto_profile, argp: *mut core::ffi::c_void) -> i32 {
    let mut arg: blk_crypto_generate_key_arg = core::mem::zeroed(); let mut lt_key = [0u8; BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE as usize];
    if copy_from_user(&mut arg, argp, core::mem::size_of_val(&arg)) != 0 { return -EFAULT; }
    if memchr_inv(arg.reserved.as_ptr(), 0, arg.reserved.len()) { return -EINVAL; }
    let mut ret = blk_crypto_generate_key(profile, lt_key.as_mut_ptr());
    if ret >= 0 { if ret > arg.lt_key_size { ret = -EOVERFLOW; } else { arg.lt_key_size = ret; if copy_to_user(u64_to_user_ptr(arg.lt_key_ptr), lt_key.as_ptr(), arg.lt_key_size) != 0 || copy_to_user(argp, &arg, core::mem::size_of_val(&arg)) != 0 { ret = -EFAULT; } else { ret = 0; } } }
    memzero_explicit(lt_key.as_mut_ptr(), lt_key.len()); ret
}

unsafe fn blk_crypto_ioctl_prepare_key(profile: *mut blk_crypto_profile, argp: *mut core::ffi::c_void) -> i32 {
    let mut arg: blk_crypto_prepare_key_arg = core::mem::zeroed(); let mut lt_key = [0u8; BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE as usize]; let mut eph_key = [0u8; BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE as usize];
    if copy_from_user(&mut arg, argp, core::mem::size_of_val(&arg)) != 0 { return -EFAULT; }
    if memchr_inv(arg.reserved.as_ptr(), 0, arg.reserved.len()) || arg.lt_key_size > lt_key.len() { return -EINVAL; }
    let mut ret = if copy_from_user(lt_key.as_mut_ptr(), u64_to_user_ptr(arg.lt_key_ptr), arg.lt_key_size) != 0 { -EFAULT } else { blk_crypto_prepare_key(profile, lt_key.as_ptr(), arg.lt_key_size, eph_key.as_mut_ptr()) };
    if ret >= 0 { if ret > arg.eph_key_size { ret = -EOVERFLOW; } else { arg.eph_key_size = ret; if copy_to_user(u64_to_user_ptr(arg.eph_key_ptr), eph_key.as_ptr(), arg.eph_key_size) != 0 || copy_to_user(argp, &arg, core::mem::size_of_val(&arg)) != 0 { ret = -EFAULT; } else { ret = 0; } } }
    memzero_explicit(lt_key.as_mut_ptr(), lt_key.len()); memzero_explicit(eph_key.as_mut_ptr(), eph_key.len()); ret
}

pub unsafe fn blk_crypto_ioctl(bdev: *mut block_device, cmd: u32, argp: *mut core::ffi::c_void) -> i32 {
    let profile = (*bdev_get_queue(bdev)).crypto_profile; if profile.is_null() { return -EOPNOTSUPP; }
    match cmd { BLKCRYPTOIMPORTKEY => blk_crypto_ioctl_import_key(profile, argp), BLKCRYPTOGENERATEKEY => blk_crypto_ioctl_generate_key(profile, argp), BLKCRYPTOPREPAREKEY => blk_crypto_ioctl_prepare_key(profile, argp), _ => -ENOTTY }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
