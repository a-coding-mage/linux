// SPDX-License-Identifier: GPL-2.0-or-later
/* Management for virtio crypto devices (refer to adf_dev_mgr.c)
 *
 * Copyright 2016 HUAWEI TECHNOLOGIES CO., LTD.
 */

// Dependencies supplied by the Linux kernel and virtio crypto implementation

static mut VIRTIO_CRYPTO_TABLE: list_head = LIST_HEAD_INIT;
static mut NUM_DEVICES: u32 = 0;

/* The table_lock protects the above global list and num_devices */
static TABLE_LOCK: mutex = DEFINE_MUTEX_INIT;

const VIRTIO_CRYPTO_MAX_DEVICES: u32 = 32;

/*
 * virtcrypto_devmgr_add_dev() - Add vcrypto_dev to the acceleration
 * framework.
 * @vcrypto_dev:  Pointer to virtio crypto device.
 *
 * Function adds virtio crypto device to the global list.
 * To be used by virtio crypto device specific drivers.
 *
 * Return: 0 on success, error code othewise.
 */
pub unsafe fn virtcrypto_devmgr_add_dev(vcrypto_dev: *mut virtio_crypto) -> i32 {
    let mut itr: *mut list_head;

    mutex_lock(&TABLE_LOCK);
    if NUM_DEVICES == VIRTIO_CRYPTO_MAX_DEVICES {
        pr_info!("virtio_crypto: only support up to %d devices\n", VIRTIO_CRYPTO_MAX_DEVICES);
        mutex_unlock(&TABLE_LOCK);
        return -EFAULT;
    }

    list_for_each!(itr, &mut VIRTIO_CRYPTO_TABLE);
    while itr != (&mut VIRTIO_CRYPTO_TABLE as *mut list_head) {
        let ptr = list_entry!(itr, virtio_crypto, list);
        if ptr == vcrypto_dev {
            mutex_unlock(&TABLE_LOCK);
            return -EEXIST;
        }
        itr = (*itr).next;
    }
    atomic_set(&mut (*vcrypto_dev).ref_count, 0);
    list_add_tail(&mut (*vcrypto_dev).list, &mut VIRTIO_CRYPTO_TABLE);
    (*vcrypto_dev).dev_id = NUM_DEVICES;
    NUM_DEVICES += 1;
    mutex_unlock(&TABLE_LOCK);
    0
}

pub unsafe fn virtcrypto_devmgr_get_head() -> *mut list_head {
    &mut VIRTIO_CRYPTO_TABLE
}

/* Remove vcrypto_dev from the acceleration framework. */
pub unsafe fn virtcrypto_devmgr_rm_dev(vcrypto_dev: *mut virtio_crypto) {
    mutex_lock(&TABLE_LOCK);
    list_del(&mut (*vcrypto_dev).list);
    NUM_DEVICES -= 1;
    mutex_unlock(&TABLE_LOCK);
}

/* Increment vcrypto_dev reference count. */
pub unsafe fn virtcrypto_dev_get(vcrypto_dev: *mut virtio_crypto) -> i32 {
    if atomic_add_return(1, &mut (*vcrypto_dev).ref_count) == 1
        && !try_module_get((*vcrypto_dev).owner)
    {
        return -EFAULT;
    }
    0
}

/* Decrement vcrypto_dev reference count. */
pub unsafe fn virtcrypto_dev_put(vcrypto_dev: *mut virtio_crypto) {
    if atomic_sub_return(1, &mut (*vcrypto_dev).ref_count) == 0 {
        module_put((*vcrypto_dev).owner);
    }
}

/* Check whether device has started. */
pub unsafe fn virtcrypto_dev_started(vcrypto_dev: *mut virtio_crypto) -> i32 {
    ((*vcrypto_dev).status & VIRTIO_CRYPTO_S_HW_READY) as i32
}

/* Get vcrypto_dev on the node. */
pub unsafe fn virtcrypto_get_dev_node(
    node: i32,
    service: u32,
    algo: u32,
) -> *mut virtio_crypto {
    let mut vcrypto_dev: *mut virtio_crypto = core::ptr::null_mut();
    let mut tmp_dev: *mut virtio_crypto;
    let mut best: usize = usize::MAX;
    let mut ctr: usize;

    mutex_lock(&TABLE_LOCK);
    list_for_each_entry!(tmp_dev, virtcrypto_devmgr_get_head(), list);
    while !tmp_dev.is_null() {
        if ((node == dev_to_node(&(*(*tmp_dev).vdev).dev)
            || dev_to_node(&(*(*tmp_dev).vdev).dev) < 0)
            && virtcrypto_dev_started(tmp_dev) != 0
            && virtcrypto_algo_is_supported(tmp_dev, service, algo))
        {
            ctr = atomic_read(&(*tmp_dev).ref_count) as usize;
            if best > ctr {
                vcrypto_dev = tmp_dev;
                best = ctr;
            }
        }
        tmp_dev = list_next_entry!(tmp_dev, list);
    }

    if vcrypto_dev.is_null() {
        pr_info!("virtio_crypto: Could not find a device on node %d\n", node);
        list_for_each_entry!(tmp_dev, virtcrypto_devmgr_get_head(), list);
        while !tmp_dev.is_null() {
            if virtcrypto_dev_started(tmp_dev) != 0
                && virtcrypto_algo_is_supported(tmp_dev, service, algo)
            {
                vcrypto_dev = tmp_dev;
                break;
            }
            tmp_dev = list_next_entry!(tmp_dev, list);
        }
    }
    mutex_unlock(&TABLE_LOCK);
    if vcrypto_dev.is_null() {
        return core::ptr::null_mut();
    }
    virtcrypto_dev_get(vcrypto_dev);
    vcrypto_dev
}

/* Start virtio crypto device. */
pub unsafe fn virtcrypto_dev_start(vcrypto: *mut virtio_crypto) -> i32 {
    if virtio_crypto_skcipher_algs_register(vcrypto) != 0 {
        pr_err!("virtio_crypto: Failed to register crypto skcipher algs\n");
        return -EFAULT;
    }
    if virtio_crypto_akcipher_algs_register(vcrypto) != 0 {
        pr_err!("virtio_crypto: Failed to register crypto akcipher algs\n");
        virtio_crypto_skcipher_algs_unregister(vcrypto);
        return -EFAULT;
    }
    0
}

/* Stop virtio crypto device. */
pub unsafe fn virtcrypto_dev_stop(vcrypto: *mut virtio_crypto) {
    virtio_crypto_skcipher_algs_unregister(vcrypto);
    virtio_crypto_akcipher_algs_unregister(vcrypto);
}

/* Validate if the virtio crypto device supports a service and algo. */
pub unsafe fn virtcrypto_algo_is_supported(
    vcrypto: *mut virtio_crypto,
    service: u32,
    mut algo: u32,
) -> bool {
    let service_mask: u32 = 1u32 << service;
    let mut algo_mask: u32 = 0;
    let mut low = true;

    if algo > 31 {
        algo -= 32;
        low = false;
    }
    if (*vcrypto).crypto_services & service_mask == 0 {
        return false;
    }
    match service {
        VIRTIO_CRYPTO_SERVICE_CIPHER => {
            algo_mask = if low { (*vcrypto).cipher_algo_l } else { (*vcrypto).cipher_algo_h };
        }
        VIRTIO_CRYPTO_SERVICE_HASH => algo_mask = (*vcrypto).hash_algo,
        VIRTIO_CRYPTO_SERVICE_MAC => {
            algo_mask = if low { (*vcrypto).mac_algo_l } else { (*vcrypto).mac_algo_h };
        }
        VIRTIO_CRYPTO_SERVICE_AEAD => algo_mask = (*vcrypto).aead_algo,
        VIRTIO_CRYPTO_SERVICE_AKCIPHER => algo_mask = (*vcrypto).akcipher_algo,
        _ => {}
    }
    algo_mask & (1u32 << algo) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
