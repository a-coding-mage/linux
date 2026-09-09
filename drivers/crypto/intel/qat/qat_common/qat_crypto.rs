// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// C includes are supplied by the surrounding kernel/QAT translation unit.

const SEC: u32 = ADF_KERNEL_SEC;

static mut qat_crypto: service_hndl = service_hndl { _private: [] };

pub unsafe fn qat_crypto_put_instance(inst: *mut qat_crypto_instance) {
    atomic_dec(&mut (*inst).refctr);
    adf_dev_put((*inst).accel_dev);
}

unsafe fn qat_crypto_free_instances(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut inst: *mut qat_crypto_instance;
    let mut tmp: *mut qat_crypto_instance;
    let mut i: i32;

    list_for_each_entry_safe!(inst, tmp, &mut (*accel_dev).crypto_list, list, {
        i = 0;
        while i < atomic_read(&mut (*inst).refctr) {
            qat_crypto_put_instance(inst);
            i += 1;
        }

        if !(*inst).sym_tx.is_null() {
            adf_remove_ring((*inst).sym_tx);
        }
        if !(*inst).sym_rx.is_null() {
            adf_remove_ring((*inst).sym_rx);
        }
        if !(*inst).pke_tx.is_null() {
            adf_remove_ring((*inst).pke_tx);
        }
        if !(*inst).pke_rx.is_null() {
            adf_remove_ring((*inst).pke_rx);
        }

        list_del(&mut (*inst).list);
        kfree(inst.cast());
    });
    0
}

pub unsafe fn qat_crypto_get_instance_node(node: i32) -> *mut qat_crypto_instance {
    let mut accel_dev: *mut adf_accel_dev = core::ptr::null_mut();
    let mut tmp_dev: *mut adf_accel_dev;
    let mut inst: *mut qat_crypto_instance = core::ptr::null_mut();
    let mut tmp_inst: *mut qat_crypto_instance;
    let mut best: usize = usize::MAX;

    list_for_each_entry!(tmp_dev, adf_devmgr_get_head(), list, {
        let ctr: usize;
        if ((node == dev_to_node(&GET_DEV!(tmp_dev)) || dev_to_node(&GET_DEV!(tmp_dev)) < 0)
            && adf_dev_started(tmp_dev)
            && !list_empty(&(*tmp_dev).crypto_list))
        {
            ctr = atomic_read(&mut (*tmp_dev).ref_count) as usize;
            if best > ctr {
                accel_dev = tmp_dev;
                best = ctr;
            }
        }
    });

    if accel_dev.is_null() {
        pr_debug_ratelimited!("QAT: Could not find a device on node %d\n", node);
        // Get any started device.
        list_for_each_entry!(tmp_dev, adf_devmgr_get_head(), list, {
            if adf_dev_started(tmp_dev) && !list_empty(&(*tmp_dev).crypto_list) {
                accel_dev = tmp_dev;
                break;
            }
        });
    }

    if accel_dev.is_null() {
        return core::ptr::null_mut();
    }

    best = usize::MAX;
    list_for_each_entry!(tmp_inst, &(*accel_dev).crypto_list, list, {
        let ctr = atomic_read(&mut (*tmp_inst).refctr) as usize;
        if best > ctr {
            inst = tmp_inst;
            best = ctr;
        }
    });
    if !inst.is_null() {
        if adf_dev_get(accel_dev) != 0 {
            dev_err!(&GET_DEV!(accel_dev), "Could not increment dev refctr\n");
            return core::ptr::null_mut();
        }
        atomic_inc(&mut (*inst).refctr);
    }
    inst
}

/// qat_crypto_vf_dev_config() - create dev config required to create crypto inst.
///
/// Creates device configuration required to create asym, sym or crypto instances.
pub unsafe fn qat_crypto_vf_dev_config(accel_dev: *mut adf_accel_dev) -> i32 {
    let ring_to_svc_map: u16 = (*GET_HW_DATA!(accel_dev)).ring_to_svc_map;
    if ring_to_svc_map != ADF_GEN2_DEFAULT_RING_TO_SRV_MAP {
        dev_err!(&GET_DEV!(accel_dev), "Unsupported ring/service mapping present on PF");
        return -EFAULT;
    }
    ((*GET_HW_DATA!(accel_dev)).dev_config)(accel_dev)
}

unsafe fn qat_crypto_create_instances(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut num_inst: usize = 0;
    let mut num_msg_sym: usize = 0;
    let mut num_msg_asym: usize = 0;
    let mut key = [0i8; ADF_CFG_MAX_KEY_LEN_IN_BYTES as usize];
    let mut val = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES as usize];
    let mut sym_bank: usize = 0;
    let mut asym_bank: usize = 0;
    let mut inst: *mut qat_crypto_instance;
    let mut msg_size: i32;
    let mut ret: i32;

    INIT_LIST_HEAD!(&mut (*accel_dev).crypto_list);
    ret = adf_cfg_get_param_value(accel_dev, SEC, ADF_NUM_CY, val.as_mut_ptr());
    if ret != 0 { return ret; }
    ret = kstrtoul(val.as_ptr(), 0, &mut num_inst);
    if ret != 0 { return ret; }

    for i in 0..num_inst {
        inst = kzalloc_node(core::mem::size_of::<qat_crypto_instance>(), GFP_KERNEL,
                            dev_to_node(&GET_DEV!(accel_dev))) as *mut qat_crypto_instance;
        if inst.is_null() { ret = -ENOMEM; goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        list_add_tail(&mut (*inst).list, &mut (*accel_dev).crypto_list);
        (*inst).id = i as i32;
        atomic_set(&mut (*inst).refctr, 0);
        (*inst).accel_dev = accel_dev;

        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_SYM_BANK_NUM, i);
        ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_ptr(), val.as_mut_ptr());
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        ret = kstrtoul(val.as_ptr(), 10, &mut sym_bank);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_ASYM_BANK_NUM, i);
        ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_ptr(), val.as_mut_ptr());
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        ret = kstrtoul(val.as_ptr(), 10, &mut asym_bank);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_SYM_SIZE, i);
        ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_ptr(), val.as_mut_ptr());
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        ret = kstrtoul(val.as_ptr(), 10, &mut num_msg_sym);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        num_msg_sym >>= 1;
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_ASYM_SIZE, i);
        ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_ptr(), val.as_mut_ptr());
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        ret = kstrtoul(val.as_ptr(), 10, &mut num_msg_asym);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        num_msg_asym >>= 1;

        msg_size = ICP_QAT_FW_REQ_DEFAULT_SZ;
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_SYM_TX, i);
        ret = adf_create_ring(accel_dev, SEC, sym_bank, num_msg_sym, msg_size, key.as_ptr(), core::ptr::null_mut(), 0, &mut (*inst).sym_tx);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        msg_size >>= 1;
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_ASYM_TX, i);
        ret = adf_create_ring(accel_dev, SEC, asym_bank, num_msg_asym, msg_size, key.as_ptr(), core::ptr::null_mut(), 0, &mut (*inst).pke_tx);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        msg_size = ICP_QAT_FW_RESP_DEFAULT_SZ;
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_SYM_RX, i);
        ret = adf_create_ring(accel_dev, SEC, sym_bank, num_msg_sym, msg_size, key.as_ptr(), qat_alg_callback, 0, &mut (*inst).sym_rx);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        snprintf!(key.as_mut_ptr(), key.len(), ADF_CY "%d" ADF_RING_ASYM_RX, i);
        ret = adf_create_ring(accel_dev, SEC, asym_bank, num_msg_asym, msg_size, key.as_ptr(), qat_alg_asym_callback, 0, &mut (*inst).pke_rx);
        if ret != 0 { goto_err!(qat_crypto_free_instances(accel_dev), ret); }
        INIT_LIST_HEAD!(&mut (*inst).backlog.list);
        spin_lock_init(&mut (*inst).backlog.lock);
    }
    return 0;
}

unsafe fn qat_crypto_init(accel_dev: *mut adf_accel_dev) -> i32 {
    if qat_crypto_create_instances(accel_dev) != 0 { return -EFAULT; }
    0
}

unsafe fn qat_crypto_shutdown(accel_dev: *mut adf_accel_dev) -> i32 { qat_crypto_free_instances(accel_dev) }

unsafe fn qat_crypto_event_handler(accel_dev: *mut adf_accel_dev, event: adf_event) -> i32 {
    match event {
        ADF_EVENT_INIT => qat_crypto_init(accel_dev),
        ADF_EVENT_SHUTDOWN => qat_crypto_shutdown(accel_dev),
        _ => 0,
    }
}

pub unsafe fn qat_crypto_register() -> i32 {
    core::ptr::write_bytes(&mut qat_crypto as *mut service_hndl, 0, 1);
    qat_crypto.event_hld = Some(qat_crypto_event_handler);
    qat_crypto.name = b"qat_crypto\0".as_ptr() as *const i8;
    adf_service_register(&mut qat_crypto)
}

pub unsafe fn qat_crypto_unregister() -> i32 { adf_service_unregister(&mut qat_crypto) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
