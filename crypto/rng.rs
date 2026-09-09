// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * RNG operations.
 *
 * Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
 * Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut CRYPTO_DEFAULT_RNG_LOCK: Mutex = DEFINE_MUTEX!();
static mut CRYPTO_DEFAULT_RNG: *mut crypto_rng = core::ptr::null_mut();
static mut CRYPTO_DEFAULT_RNG_REFCNT: i32 = 0;

pub unsafe fn crypto_rng_reset(
    tfm: *mut crypto_rng,
    seed: *const u8,
    slen: u32,
) -> i32 {
    let mut buf: *mut u8 = core::ptr::null_mut();
    let mut err: i32;
    let mut seed = seed;

    if seed.is_null() && slen != 0 {
        buf = kmalloc(slen as usize, GFP_KERNEL);
        if buf.is_null() {
            return -ENOMEM;
        }

        err = get_random_bytes_wait(buf, slen);
        if err != 0 {
            kfree_sensitive(buf);
            return err;
        }
        seed = buf;
    }

    err = ((*crypto_rng_alg(tfm)).seed)(tfm, seed, slen);
    kfree_sensitive(buf);
    err
}

pub unsafe fn crypto_rng_init_tfm(_tfm: *mut crypto_tfm) -> i32 {
    0
}

unsafe fn seedsize(alg: *mut crypto_alg) -> u32 {
    let ralg = container_of!(alg, rng_alg, base);
    (*ralg).seedsize
}

#[cfg_attr(any(), allow(unused_variables))]
unsafe fn crypto_rng_report(skb: *mut sk_buff, alg: *mut crypto_alg) -> i32 {
    let mut rrng = crypto_report_rng {
        r#type: "rng",
        ..core::mem::zeroed()
    };

    rrng.seedsize = seedsize(alg);
    nla_put(skb, CRYPTOCFGA_REPORT_RNG, core::mem::size_of::<crypto_report_rng>() as u32, &rrng)
}

#[cfg_attr(any(), allow(unused_variables))]
unsafe fn crypto_rng_show(m: *mut seq_file, alg: *mut crypto_alg) {
    seq_printf(m, "type         : rng\n");
    seq_printf(m, "seedsize     : %u\n", seedsize(alg));
}

static CRYPTO_RNG_TYPE: crypto_type = crypto_type {
    extsize: crypto_alg_extsize,
    init_tfm: crypto_rng_init_tfm,
    // #ifdef CONFIG_PROC_FS
    show: crypto_rng_show,
    // #endif
    // #if IS_ENABLED(CONFIG_CRYPTO_USER)
    report: crypto_rng_report,
    // #endif
    maskclear: !CRYPTO_ALG_TYPE_MASK,
    maskset: CRYPTO_ALG_TYPE_MASK,
    r#type: CRYPTO_ALG_TYPE_RNG,
    tfmsize: core::mem::offset_of!(crypto_rng, base),
    algsize: core::mem::offset_of!(rng_alg, base),
};

pub unsafe fn crypto_alloc_rng(alg_name: *const u8, r#type: u32, mask: u32) -> *mut crypto_rng {
    crypto_alloc_tfm(alg_name, &CRYPTO_RNG_TYPE, r#type, mask)
}

unsafe fn crypto_get_default_rng() -> i32 {
    let mut rng: *mut crypto_rng;
    let mut err: i32;

    mutex_lock(&mut CRYPTO_DEFAULT_RNG_LOCK);
    if CRYPTO_DEFAULT_RNG.is_null() {
        rng = crypto_alloc_rng(b"stdrng\0".as_ptr(), 0, 0);
        err = PTR_ERR(rng);
        if IS_ERR(rng) {
            mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
            return err;
        }

        err = crypto_rng_reset(rng, core::ptr::null(), crypto_rng_seedsize(rng));
        if err != 0 {
            crypto_free_rng(rng);
            mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
            return err;
        }

        CRYPTO_DEFAULT_RNG = rng;
    }

    CRYPTO_DEFAULT_RNG_REFCNT += 1;
    mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
    0
}

unsafe fn crypto_put_default_rng() {
    mutex_lock(&mut CRYPTO_DEFAULT_RNG_LOCK);
    CRYPTO_DEFAULT_RNG_REFCNT -= 1;
    mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
}

pub unsafe fn __crypto_stdrng_get_bytes(buf: *mut core::ffi::c_void, len: u32) -> i32 {
    let mut err = crypto_get_default_rng();
    if err != 0 {
        return err;
    }

    err = crypto_rng_get_bytes(CRYPTO_DEFAULT_RNG, buf, len);
    crypto_put_default_rng();
    err
}

// #if defined(CONFIG_CRYPTO_RNG) || defined(CONFIG_CRYPTO_RNG_MODULE)
pub unsafe fn crypto_del_default_rng() -> i32 {
    let mut err = -EBUSY;

    mutex_lock(&mut CRYPTO_DEFAULT_RNG_LOCK);
    if CRYPTO_DEFAULT_RNG_REFCNT != 0 {
        mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
        return err;
    }

    crypto_free_rng(CRYPTO_DEFAULT_RNG);
    CRYPTO_DEFAULT_RNG = core::ptr::null_mut();
    err = 0;
    mutex_unlock(&mut CRYPTO_DEFAULT_RNG_LOCK);
    err
}
// #endif

unsafe fn rng_default_set_ent(_tfm: *mut crypto_rng, _data: *const u8, _len: u32) {}

pub unsafe fn crypto_register_rng(alg: *mut rng_alg) -> i32 {
    let base = &mut (*alg).base;

    if (*alg).seedsize > PAGE_SIZE / 8 {
        return -EINVAL;
    }

    base.cra_type = &CRYPTO_RNG_TYPE;
    base.cra_flags &= !CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_RNG;

    if (*alg).set_ent.is_none() {
        (*alg).set_ent = Some(rng_default_set_ent);
    }

    crypto_register_alg(base)
}

pub unsafe fn crypto_unregister_rng(alg: *mut rng_alg) {
    crypto_unregister_alg(&mut (*alg).base);
}

pub unsafe fn crypto_register_rngs(algs: *mut rng_alg, count: i32) -> i32 {
    let mut i = 0;
    while i < count {
        let ret = crypto_register_rng(algs.add(i as usize));
        if ret != 0 {
            crypto_unregister_rngs(algs, i);
            return ret;
        }
        i += 1;
    }
    0
}

pub unsafe fn crypto_unregister_rngs(algs: *mut rng_alg, count: i32) {
    let mut i = count - 1;
    while i >= 0 {
        crypto_unregister_rng(algs.add(i as usize));
        i -= 1;
    }
}

unsafe fn rng_exit() {
    let err = crypto_del_default_rng();
    if err != 0 {
        pr_err!("Failed delete default RNG: %d\n", err);
    }
}

module_exit!(rng_exit);

module_license!("GPL");
module_description!("Random Number Generator");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
