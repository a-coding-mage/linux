// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation */

// C headers and build-provided symbols are intentionally left as external
// dependencies of this source-level translation.

const SEC: u32 = ADF_KERNEL_SEC;

static mut qat_compression: service_hndl = service_hndl { };

pub unsafe fn qat_compression_put_instance(inst: *mut qat_compression_instance) {
    atomic_dec(&mut (*inst).refctr);
    adf_dev_put((*inst).accel_dev);
}

unsafe fn qat_compression_free_instances(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut list_ptr: *mut list_head;
    let mut tmp: *mut list_head;
    let mut inst: *mut qat_compression_instance;
    let mut i: i32;

    list_for_each_safe(&mut list_ptr, &mut tmp, &mut (*accel_dev).compression_list) {
        inst = list_entry(list_ptr, qat_compression_instance, list);

        i = 0;
        while i < atomic_read(&(*inst).refctr) {
            qat_compression_put_instance(inst);
            i += 1;
        }
        if !(*inst).dc_tx.is_null() { adf_remove_ring((*inst).dc_tx); }
        if !(*inst).dc_rx.is_null() { adf_remove_ring((*inst).dc_rx); }
        list_del(list_ptr);
        kfree(inst as *mut core::ffi::c_void);
    }
    0
}

pub unsafe fn qat_compression_get_instance_node(node: i32, alg: i32) -> *mut qat_compression_instance {
    let mut inst: *mut qat_compression_instance = core::ptr::null_mut();
    let mut hw_data: *mut adf_hw_device_data = core::ptr::null_mut();
    let mut accel_dev: *mut adf_accel_dev = core::ptr::null_mut();
    let mut best: usize = !0;
    let mut itr: *mut list_head;
    let mut caps: u32;
    let mut mask: u32;

    list_for_each(&mut itr, adf_devmgr_get_head()) {
        let tmp_dev = list_entry(itr, adf_accel_dev, list);
        let tmp_dev_node: i32 = dev_to_node(&mut GET_DEV(tmp_dev));
        if alg == QAT_ZSTD || alg == QAT_LZ4S {
            hw_data = (*tmp_dev).hw_device;
            caps = (*hw_data).accel_capabilities_ext_mask;
            mask = ADF_ACCEL_CAPABILITIES_EXT_ZSTD | ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S;
            if caps & mask == 0 { continue; }
        }
        if (node == tmp_dev_node || tmp_dev_node < 0) && adf_dev_started(tmp_dev) && !list_empty(&mut (*tmp_dev).compression_list) {
            let ctr = atomic_read(&(*tmp_dev).ref_count) as usize;
            if best > ctr { accel_dev = tmp_dev; best = ctr; }
        }
    }
    if accel_dev.is_null() {
        pr_debug_ratelimited!("QAT: Could not find a device on node %d\n", node);
        list_for_each(&mut itr, adf_devmgr_get_head()) {
            let tmp_dev = list_entry(itr, adf_accel_dev, list);
            if alg == QAT_ZSTD || alg == QAT_LZ4S {
                hw_data = (*tmp_dev).hw_device;
                caps = (*hw_data).accel_capabilities_ext_mask;
                mask = ADF_ACCEL_CAPABILITIES_EXT_ZSTD | ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S;
                if caps & mask == 0 { continue; }
            }
            if adf_dev_started(tmp_dev) && !list_empty(&mut (*tmp_dev).compression_list) { accel_dev = tmp_dev; break; }
        }
    }
    if accel_dev.is_null() { return core::ptr::null_mut(); }
    best = !0;
    list_for_each(&mut itr, &mut (*accel_dev).compression_list) {
        let tmp_inst = list_entry(itr, qat_compression_instance, list);
        let ctr = atomic_read(&(*tmp_inst).refctr) as usize;
        if best > ctr { inst = tmp_inst; best = ctr; }
    }
    if !inst.is_null() {
        if adf_dev_get(accel_dev) != 0 { dev_err!(&mut GET_DEV(accel_dev), "Could not increment dev refctr\n"); return core::ptr::null_mut(); }
        atomic_inc(&mut (*inst).refctr);
    }
    inst
}

unsafe fn qat_compression_create_instances(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut inst: *mut qat_compression_instance;
    let mut key = [0i8; ADF_CFG_MAX_KEY_LEN_IN_BYTES];
    let mut val = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let mut num_inst: usize = 0;
    let mut num_msg_dc: usize = 0;
    let mut bank: usize = 0;
    let mut msg_size: i32;
    let mut ret: i32;
    let mut i = 0usize;
    INIT_LIST_HEAD(&mut (*accel_dev).compression_list);
    strscpy(key.as_mut_ptr(), ADF_NUM_DC);
    ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_mut_ptr(), val.as_mut_ptr()); if ret != 0 { return ret; }
    ret = kstrtoul(val.as_ptr(), 10, &mut num_inst); if ret != 0 { return ret; }
    while i < num_inst {
        inst = kzalloc_node(core::mem::size_of::<qat_compression_instance>(), GFP_KERNEL, dev_to_node(&mut GET_DEV(accel_dev)));
        if inst.is_null() { ret = -ENOMEM; goto_err!(qat_compression_free_instances(accel_dev)); return ret; }
        list_add_tail(&mut (*inst).list, &mut (*accel_dev).compression_list); (*inst).id = i as i32; atomic_set(&mut (*inst).refctr, 0); (*inst).accel_dev = accel_dev;
        snprintf!(key.as_mut_ptr(), key.len(), concat!(ADF_DC, "%d", ADF_RING_DC_BANK_NUM), i); ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_mut_ptr(), val.as_mut_ptr()); if ret != 0 { return ret; }
        ret = kstrtoul(val.as_ptr(), 10, &mut bank); if ret != 0 { return ret; }
        snprintf!(key.as_mut_ptr(), key.len(), concat!(ADF_DC, "%d", ADF_RING_DC_SIZE), i); ret = adf_cfg_get_param_value(accel_dev, SEC, key.as_mut_ptr(), val.as_mut_ptr()); if ret != 0 { return ret; }
        ret = kstrtoul(val.as_ptr(), 10, &mut num_msg_dc); if ret != 0 { return ret; }
        msg_size = ICP_QAT_FW_REQ_DEFAULT_SZ; snprintf!(key.as_mut_ptr(), key.len(), concat!(ADF_DC, "%d", ADF_RING_DC_TX), i); ret = adf_create_ring(accel_dev, SEC, bank, num_msg_dc, msg_size, key.as_mut_ptr(), core::ptr::null_mut(), 0, &mut (*inst).dc_tx); if ret != 0 { return ret; }
        msg_size = ICP_QAT_FW_RESP_DEFAULT_SZ; snprintf!(key.as_mut_ptr(), key.len(), concat!(ADF_DC, "%d", ADF_RING_DC_RX), i); ret = adf_create_ring(accel_dev, SEC, bank, num_msg_dc, msg_size, key.as_mut_ptr(), qat_comp_alg_callback, 0, &mut (*inst).dc_rx); if ret != 0 { return ret; }
        (*inst).dc_data = (*accel_dev).dc_data; INIT_LIST_HEAD(&mut (*inst).backlog.list); spin_lock_init(&mut (*inst).backlog.lock); i += 1;
    }
    return 0;
}

unsafe fn qat_compression_alloc_dc_data(accel_dev: *mut adf_accel_dev) -> i32 {
    let dev = &mut GET_DEV(accel_dev); let mut obuff_p = DMA_MAPPING_ERROR; let ovf_buff_sz = QAT_COMP_MAX_SKID; let mut dc_data: *mut adf_dc_data = core::ptr::null_mut(); let mut obuff: *mut u8 = core::ptr::null_mut();
    dc_data = kzalloc_node(core::mem::size_of::<adf_dc_data>(), GFP_KERNEL, dev_to_node(dev)); if dc_data.is_null() { goto_err!(()); return -ENOMEM; }
    obuff = kzalloc_node(ovf_buff_sz, GFP_KERNEL, dev_to_node(dev)); if obuff.is_null() { goto_err!(kfree(dc_data)); return -ENOMEM; }
    obuff_p = dma_map_single(dev, obuff, ovf_buff_sz, DMA_BIDIRECTIONAL); if unlikely!(dma_mapping_error(dev, obuff_p)) { goto_err!(kfree(obuff); devm_kfree(dev, dc_data)); return -ENOMEM; }
    (*dc_data).ovf_buff = obuff; (*dc_data).ovf_buff_p = obuff_p; (*dc_data).ovf_buff_sz = ovf_buff_sz; (*accel_dev).dc_data = dc_data; 0
}

unsafe fn qat_free_dc_data(accel_dev: *mut adf_accel_dev) { let dc_data = (*accel_dev).dc_data; let dev = &mut GET_DEV(accel_dev); if dc_data.is_null() { return; } dma_unmap_single(dev, (*dc_data).ovf_buff_p, (*dc_data).ovf_buff_sz, DMA_BIDIRECTIONAL); kfree_sensitive((*dc_data).ovf_buff); kfree(dc_data); (*accel_dev).dc_data = core::ptr::null_mut(); }
unsafe fn qat_compression_init(accel_dev: *mut adf_accel_dev) -> i32 { let ret = qat_compression_alloc_dc_data(accel_dev); if ret != 0 { return ret; } let ret = qat_compression_create_instances(accel_dev); if ret != 0 { qat_free_dc_data(accel_dev); } ret }
unsafe fn qat_compression_shutdown(accel_dev: *mut adf_accel_dev) -> i32 { qat_free_dc_data(accel_dev); qat_compression_free_instances(accel_dev) }
unsafe fn qat_compression_event_handler(accel_dev: *mut adf_accel_dev, event: adf_event) -> i32 { match event { ADF_EVENT_INIT => qat_compression_init(accel_dev), ADF_EVENT_SHUTDOWN => qat_compression_shutdown(accel_dev), _ => 0 } }
pub unsafe fn qat_compression_register() -> i32 { core::ptr::write_bytes(&mut qat_compression, 0, 1); qat_compression.event_hld = Some(qat_compression_event_handler); qat_compression.name = "qat_compression\0".as_ptr() as *const i8; adf_service_register(&mut qat_compression) }
pub unsafe fn qat_compression_unregister() -> i32 { adf_service_unregister(&mut qat_compression) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
