// SPDX-License-Identifier: GPL-2.0
/*
 * Microchip / Atmel ECC (I2C) driver.
 *
 * Copyright (c) 2017, Microchip Technology Inc.
 * Author: Tudor Ambarus
 */

// Linux kernel dependencies supplied by the surrounding repository.

static mut DRIVER_DATA: atmel_ecc_driver_data = unsafe { core::mem::zeroed() };

/*
 * struct atmel_ecdh_ctx - transformation context
 * @client: I2C client device
 * @fallback: ECDH fallback used for caller-provided private keys
 * @public_key: cached public key for the device-generated private key
 * @do_fallback: true when ECDH operations should use @fallback
 *
 * The caller must not invoke set_secret() while generate_public_key()
 * or compute_shared_secret() are in flight.
 */
#[repr(C)]
struct atmel_ecdh_ctx {
    client: *mut i2c_client,
    fallback: *mut crypto_kpp,
    public_key: *const u8,
    do_fallback: bool,
}

unsafe fn atmel_ecdh_done(work_data: *mut atmel_i2c_work_data, areq: *mut core::ffi::c_void, mut status: i32) {
    let req = areq as *mut kpp_request;
    let cmd = &mut (*work_data).cmd;
    let mut copied: usize;
    let n_sz: usize;

    if status != 0 {
        goto_free_work_data!(work_data, req, status);
    }

    /* copy only as much as requested, capped at 32 bytes */
    n_sz = core::cmp::min(ATMEL_ECC_NIST_P256_N_SIZE, (*req).dst_len);

    /* copy the shared secret */
    copied = sg_copy_from_buffer((*req).dst, sg_nents_for_len((*req).dst, n_sz),
                                 cmd.data.as_ptr().add(RSP_DATA_IDX), n_sz);
    if copied != n_sz {
        status = -EINVAL;
    }

    kfree_sensitive(work_data as *mut core::ffi::c_void);
    kpp_request_complete(req, status);
}

unsafe fn atmel_ecdh_set_secret(tfm: *mut crypto_kpp, buf: *const core::ffi::c_void, len: u32) -> i32 {
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    let mut cmd: *mut atmel_i2c_cmd;
    let mut public_key: *mut core::ffi::c_void;
    let mut params: ecdh = core::mem::zeroed();
    let mut ret: i32 = -ENOMEM;

    kfree((*ctx).public_key as *mut core::ffi::c_void);
    (*ctx).public_key = core::ptr::null();
    (*ctx).do_fallback = false;

    if crypto_ecdh_decode_key(buf, len, &mut params) < 0 {
        dev_err(&(*(*ctx).client).dev, "crypto_ecdh_decode_key failed\n");
        return -EINVAL;
    }

    if params.key_size != 0 {
        ret = crypto_kpp_set_secret((*ctx).fallback, buf, len);
        (*ctx).do_fallback = ret == 0;
        return ret;
    }

    cmd = kmalloc_obj::<atmel_i2c_cmd>();
    if cmd.is_null() { return -ENOMEM; }
    public_key = kmalloc(ATMEL_ECC_PUBKEY_SIZE, GFP_KERNEL);
    if public_key.is_null() { kfree(cmd as *mut core::ffi::c_void); return ret; }

    atmel_i2c_init_genkey_cmd(cmd, DATA_SLOT_2);
    ret = atmel_i2c_send_receive((*ctx).client, cmd);
    if ret != 0 {
        kfree(public_key);
        kfree(cmd as *mut core::ffi::c_void);
        return ret;
    }
    core::ptr::copy_nonoverlapping((*cmd).data.as_ptr().add(RSP_DATA_IDX), public_key as *mut u8,
                                   ATMEL_ECC_PUBKEY_SIZE);
    (*ctx).public_key = public_key as *const u8;
    kfree(cmd as *mut core::ffi::c_void);
    ret
}

unsafe fn atmel_ecdh_generate_public_key(req: *mut kpp_request) -> i32 {
    let tfm = crypto_kpp_reqtfm(req);
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    if (*ctx).do_fallback {
        kpp_request_set_tfm(req, (*ctx).fallback);
        return crypto_kpp_generate_public_key(req);
    }
    if (*ctx).public_key.is_null() { return -EINVAL; }
    /* copy only as much as requested, capped at 64 bytes */
    let nbytes = core::cmp::min(ATMEL_ECC_PUBKEY_SIZE, (*req).dst_len);
    let copied = sg_copy_from_buffer((*req).dst, sg_nents_for_len((*req).dst, nbytes),
                                     (*ctx).public_key, nbytes);
    if copied != nbytes { return -EINVAL; }
    0
}

unsafe fn atmel_ecdh_compute_shared_secret(req: *mut kpp_request) -> i32 {
    let tfm = crypto_kpp_reqtfm(req);
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    if (*ctx).do_fallback {
        kpp_request_set_tfm(req, (*ctx).fallback);
        return crypto_kpp_compute_shared_secret(req);
    }
    if (*ctx).public_key.is_null() || (*req).src_len != ATMEL_ECC_PUBKEY_SIZE { return -EINVAL; }
    let gfp = if ((*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    let work_data = kmalloc_obj_gfp::<atmel_i2c_work_data>(gfp);
    if work_data.is_null() { return -ENOMEM; }
    (*work_data).ctx = ctx as *mut _;
    (*work_data).client = (*ctx).client;
    let ret = atmel_i2c_init_ecdh_cmd(&mut (*work_data).cmd, (*req).src);
    if ret != 0 { kfree(work_data as *mut core::ffi::c_void); return ret; }
    atmel_i2c_enqueue(work_data, atmel_ecdh_done, req as *mut core::ffi::c_void);
    -EINPROGRESS
}

unsafe fn atmel_ecc_i2c_client_alloc() -> *mut i2c_client {
    let mut min_i2c_priv: *mut atmel_i2c_client_priv = core::ptr::null_mut();
    let mut client = ERR_PTR(-ENODEV);
    let mut min_tfm_cnt = INT_MAX;
    spin_lock(&mut DRIVER_DATA.i2c_list_lock);
    if list_empty(&DRIVER_DATA.i2c_client_list) {
        spin_unlock(&mut DRIVER_DATA.i2c_list_lock); return ERR_PTR(-ENODEV);
    }
    list_for_each_entry!(i2c_priv, &DRIVER_DATA.i2c_client_list, i2c_client_list_node, {
        let tfm_cnt = atomic_read(&(*i2c_priv).tfm_count);
        if tfm_cnt < min_tfm_cnt { min_tfm_cnt = tfm_cnt; min_i2c_priv = i2c_priv; }
        if min_tfm_cnt == 0 { break; }
    });
    if !min_i2c_priv.is_null() { atomic_inc(&mut (*min_i2c_priv).tfm_count); client = (*min_i2c_priv).client; }
    spin_unlock(&mut DRIVER_DATA.i2c_list_lock);
    client
}

unsafe fn atmel_ecc_i2c_client_free(client: *mut i2c_client) {
    let i2c_priv = i2c_get_clientdata(client);
    atomic_dec(&mut (*i2c_priv).tfm_count);
}

// The remaining registration and lifecycle declarations retain the C driver's
// externally supplied kernel types and callbacks.
unsafe fn atmel_ecdh_init_tfm(tfm: *mut crypto_kpp) -> i32 {
    let alg = kpp_alg_name(tfm);
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    (*ctx).client = atmel_ecc_i2c_client_alloc();
    if IS_ERR((*ctx).client) { pr_err!("tfm - i2c_client binding failed\n"); return PTR_ERR((*ctx).client); }
    let fallback = crypto_alloc_kpp(alg, 0, CRYPTO_ALG_NEED_FALLBACK);
    if IS_ERR(fallback) { atmel_ecc_i2c_client_free((*ctx).client); return PTR_ERR(fallback); }
    crypto_kpp_set_flags(fallback, crypto_kpp_get_flags(tfm));
    (*ctx).fallback = fallback;
    0
}

unsafe fn atmel_ecdh_exit_tfm(tfm: *mut crypto_kpp) {
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    kfree((*ctx).public_key as *mut core::ffi::c_void);
    crypto_free_kpp((*ctx).fallback);
    atmel_ecc_i2c_client_free((*ctx).client);
}

unsafe fn atmel_ecdh_max_size(tfm: *mut crypto_kpp) -> u32 {
    let ctx = kpp_tfm_ctx(tfm) as *mut atmel_ecdh_ctx;
    crypto_kpp_maxsize((*ctx).fallback)
}

static mut ATMEL_ECDH_NIST_P256: kpp_alg = kpp_alg {
    set_secret: Some(atmel_ecdh_set_secret), generate_public_key: Some(atmel_ecdh_generate_public_key),
    compute_shared_secret: Some(atmel_ecdh_compute_shared_secret), init: Some(atmel_ecdh_init_tfm),
    exit: Some(atmel_ecdh_exit_tfm), max_size: Some(atmel_ecdh_max_size),
    base: crypto_alg { cra_flags: CRYPTO_ALG_NEED_FALLBACK, cra_name: "ecdh-nist-p256", cra_driver_name: "atmel-ecdh",
        cra_priority: ATMEL_ECC_PRIORITY, cra_module: THIS_MODULE, cra_ctxsize: core::mem::size_of::<atmel_ecdh_ctx>() },
};

unsafe fn atmel_ecc_probe(client: *mut i2c_client) -> i32 {
    let mut ret = atmel_i2c_probe(client);
    if ret != 0 { return ret; }
    let i2c_priv = i2c_get_clientdata(client);
    spin_lock(&mut DRIVER_DATA.i2c_list_lock);
    list_add_tail(&mut (*i2c_priv).i2c_client_list_node, &mut DRIVER_DATA.i2c_client_list);
    spin_unlock(&mut DRIVER_DATA.i2c_list_lock);
    ret = crypto_register_kpp(&mut ATMEL_ECDH_NIST_P256);
    if ret != 0 {
        spin_lock(&mut DRIVER_DATA.i2c_list_lock);
        list_del(&mut (*i2c_priv).i2c_client_list_node);
        spin_unlock(&mut DRIVER_DATA.i2c_list_lock);
        dev_err!(&(*client).dev, "%s alg registration failed\n", "atmel-ecdh");
    } else {
        dev_info!(&(*client).dev, "atmel ecc algorithms registered in /proc/crypto\n");
    }
    ret
}

unsafe fn atmel_ecc_remove(client: *mut i2c_client) {
    let i2c_priv = i2c_get_clientdata(client);
    /* Return EBUSY if i2c client already allocated. */
    if atomic_read(&(*i2c_priv).tfm_count) != 0 {
        /* After we return here, the memory backing the device is freed. */
        dev_emerg!(&(*client).dev, "Device is busy, expect memory corruption.\n");
        return;
    }
    crypto_unregister_kpp(&mut ATMEL_ECDH_NIST_P256);
    spin_lock(&mut DRIVER_DATA.i2c_list_lock);
    list_del(&mut (*i2c_priv).i2c_client_list_node);
    spin_unlock(&mut DRIVER_DATA.i2c_list_lock);
}

static ATmel_ECC_DT_IDS: &[of_device_id] = &[
    of_device_id { compatible: "atmel,atecc508a" },
    of_device_id { compatible: "atmel,atecc608b" },
    of_device_id::default(),
];

static ATMEL_ECC_ID: &[i2c_device_id] = &[
    i2c_device_id { name: "atecc508a" },
    i2c_device_id { name: "atecc608b" },
    i2c_device_id::default(),
];

static mut ATMEL_ECC_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver { name: "atmel-ecc", of_match_table: ATmel_ECC_DT_IDS },
    probe: Some(atmel_ecc_probe), remove: Some(atmel_ecc_remove), id_table: ATMEL_ECC_ID,
};

unsafe fn atmel_ecc_init() -> i32 {
    spin_lock_init(&mut DRIVER_DATA.i2c_list_lock);
    INIT_LIST_HEAD(&mut DRIVER_DATA.i2c_client_list);
    i2c_add_driver(&mut ATMEL_ECC_DRIVER)
}

unsafe fn atmel_ecc_exit() {
    atmel_i2c_flush_queue();
    i2c_del_driver(&mut ATMEL_ECC_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
