// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2019 Samsung Electronics Co., Ltd.
 */

// Linux and local header dependencies from the original implementation are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct crypto_ctx_list {
    pub ctx_lock: spinlock_t,
    pub avail_ctx: c_int,
    pub idle_ctx: list_head,
    pub ctx_wait: wait_queue_head_t,
}

static mut ctx_list: crypto_ctx_list = crypto_ctx_list {
    ctx_lock: unsafe { core::mem::zeroed() },
    avail_ctx: 0,
    idle_ctx: unsafe { core::mem::zeroed() },
    ctx_wait: unsafe { core::mem::zeroed() },
};

#[inline]
unsafe fn free_aead(aead: *mut crypto_aead) {
    if !aead.is_null() {
        crypto_free_aead(aead);
    }
}

unsafe fn alloc_aead(id: c_int) -> *mut crypto_aead {
    let tfm: *mut crypto_aead;

    match id {
        CRYPTO_AEAD_AES_GCM => {
            tfm = crypto_alloc_aead(b"gcm(aes)\\0".as_ptr() as *const c_char, 0, 0);
        }
        CRYPTO_AEAD_AES_CCM => {
            tfm = crypto_alloc_aead(b"ccm(aes)\\0".as_ptr() as *const c_char, 0, 0);
        }
        _ => {
            pr_err(b"Does not support encrypt ahead(id : %d)\\n\\0".as_ptr(), id);
            return core::ptr::null_mut();
        }
    }

    if IS_ERR(tfm) {
        pr_err(b"Failed to alloc encrypt aead : %ld\\n\\0".as_ptr(), PTR_ERR(tfm));
        return core::ptr::null_mut();
    }

    tfm
}

unsafe fn ctx_free(ctx: *mut ksmbd_crypto_ctx) {
    let mut i = 0;
    while i < CRYPTO_AEAD_MAX {
        free_aead((*ctx).ccmaes[i as usize]);
        i += 1;
    }
    kfree(ctx as *mut c_void);
}

unsafe fn ksmbd_find_crypto_ctx() -> *mut ksmbd_crypto_ctx {
    loop {
        spin_lock(&mut ctx_list.ctx_lock);
        if !list_empty(&ctx_list.idle_ctx) {
            let ctx = list_entry(ctx_list.idle_ctx.next,
                                 ksmbd_crypto_ctx,
                                 list);
            list_del(&mut (*ctx).list);
            spin_unlock(&mut ctx_list.ctx_lock);
            return ctx;
        }

        if ctx_list.avail_ctx > num_online_cpus() {
            spin_unlock(&mut ctx_list.ctx_lock);
            wait_event(ctx_list.ctx_wait, !list_empty(&ctx_list.idle_ctx));
            continue;
        }

        ctx_list.avail_ctx += 1;
        spin_unlock(&mut ctx_list.ctx_lock);

        let ctx = kzalloc_obj::<ksmbd_crypto_ctx>(KSMBD_DEFAULT_GFP);
        if ctx.is_null() {
            spin_lock(&mut ctx_list.ctx_lock);
            ctx_list.avail_ctx -= 1;
            spin_unlock(&mut ctx_list.ctx_lock);
            wait_event(ctx_list.ctx_wait, !list_empty(&ctx_list.idle_ctx));
            continue;
        }
        return ctx;
    }
}

pub unsafe fn ksmbd_release_crypto_ctx(ctx: *mut ksmbd_crypto_ctx) {
    if ctx.is_null() {
        return;
    }

    spin_lock(&mut ctx_list.ctx_lock);
    if ctx_list.avail_ctx <= num_online_cpus() {
        list_add(&mut (*ctx).list, &mut ctx_list.idle_ctx);
        spin_unlock(&mut ctx_list.ctx_lock);
        wake_up(&mut ctx_list.ctx_wait);
        return;
    }

    ctx_list.avail_ctx -= 1;
    spin_unlock(&mut ctx_list.ctx_lock);
    ctx_free(ctx);
}

unsafe fn ____crypto_aead_ctx_find(id: c_int) -> *mut ksmbd_crypto_ctx {
    if id >= CRYPTO_AEAD_MAX {
        return core::ptr::null_mut();
    }

    let ctx = ksmbd_find_crypto_ctx();
    if !(*ctx).ccmaes[id as usize].is_null() {
        return ctx;
    }

    (*ctx).ccmaes[id as usize] = alloc_aead(id);
    if !(*ctx).ccmaes[id as usize].is_null() {
        return ctx;
    }
    ksmbd_release_crypto_ctx(ctx);
    core::ptr::null_mut()
}

pub unsafe fn ksmbd_crypto_ctx_find_gcm() -> *mut ksmbd_crypto_ctx {
    ____crypto_aead_ctx_find(CRYPTO_AEAD_AES_GCM)
}

pub unsafe fn ksmbd_crypto_ctx_find_ccm() -> *mut ksmbd_crypto_ctx {
    ____crypto_aead_ctx_find(CRYPTO_AEAD_AES_CCM)
}

pub unsafe fn ksmbd_crypto_destroy() {
    while !list_empty(&ctx_list.idle_ctx) {
        let ctx = list_entry(ctx_list.idle_ctx.next, ksmbd_crypto_ctx, list);
        list_del(&mut (*ctx).list);
        ctx_free(ctx);
    }
}

pub unsafe fn ksmbd_crypto_create() -> c_int {
    spin_lock_init(&mut ctx_list.ctx_lock);
    INIT_LIST_HEAD(&mut ctx_list.idle_ctx);
    init_waitqueue_head(&mut ctx_list.ctx_wait);
    ctx_list.avail_ctx = 1;

    let ctx = kzalloc_obj::<ksmbd_crypto_ctx>(KSMBD_DEFAULT_GFP);
    if ctx.is_null() {
        return -ENOMEM;
    }
    list_add(&mut (*ctx).list, &mut ctx_list.idle_ctx);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
