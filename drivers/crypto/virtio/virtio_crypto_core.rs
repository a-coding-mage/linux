// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for Virtio crypto device.
 *
 * Copyright 2016 HUAWEI TECHNOLOGIES CO., LTD.
 */

// Dependencies are supplied by the surrounding kernel/Rust bindings.

pub unsafe fn virtcrypto_clear_request(vc_req: *mut virtio_crypto_request) {
    if !vc_req.is_null() {
        kfree_sensitive((*vc_req).req_data);
        kfree((*vc_req).sgs);
    }
}

unsafe fn virtio_crypto_ctrlq_callback(vc_ctrl_req: *mut virtio_crypto_ctrl_request) {
    complete(&mut (*vc_ctrl_req).compl);
}

unsafe fn virtcrypto_ctrlq_callback(vq: *mut virtqueue) {
    let vcrypto = (*(*vq).vdev).priv_;
    let mut vc_ctrl_req: *mut virtio_crypto_ctrl_request;
    let mut flags: c_ulong = 0;
    let mut len: c_uint = 0;
    spin_lock_irqsave(&mut (*vcrypto).ctrl_lock, &mut flags);
    loop {
        virtqueue_disable_cb(vq);
        while { vc_ctrl_req = virtqueue_get_buf(vq, &mut len); !vc_ctrl_req.is_null() } {
            spin_unlock_irqrestore(&mut (*vcrypto).ctrl_lock, flags);
            virtio_crypto_ctrlq_callback(vc_ctrl_req);
            spin_lock_irqsave(&mut (*vcrypto).ctrl_lock, &mut flags);
        }
        if virtqueue_enable_cb(vq) { break; }
    }
    spin_unlock_irqrestore(&mut (*vcrypto).ctrl_lock, flags);
}

pub unsafe fn virtio_crypto_ctrl_vq_request(
    vcrypto: *mut virtio_crypto, sgs: *mut *mut scatterlist,
    out_sgs: c_uint, in_sgs: c_uint,
    vc_ctrl_req: *mut virtio_crypto_ctrl_request,
) -> c_int {
    let mut flags: c_ulong = 0;
    init_completion(&mut (*vc_ctrl_req).compl);
    spin_lock_irqsave(&mut (*vcrypto).ctrl_lock, &mut flags);
    let err = virtqueue_add_sgs((*vcrypto).ctrl_vq, sgs, out_sgs, in_sgs, vc_ctrl_req, GFP_ATOMIC);
    if err < 0 {
        spin_unlock_irqrestore(&mut (*vcrypto).ctrl_lock, flags);
        return err;
    }
    virtqueue_kick((*vcrypto).ctrl_vq);
    spin_unlock_irqrestore(&mut (*vcrypto).ctrl_lock, flags);
    wait_for_completion(&mut (*vc_ctrl_req).compl);
    0
}

unsafe fn virtcrypto_done_work(work: *mut work_struct) {
    let data_vq = from_work!(data_vq, work, done_work);
    let vq = (*data_vq).vq;
    let mut vc_req: *mut virtio_crypto_request;
    let mut flags: c_ulong = 0;
    let mut len: c_uint = 0;
    spin_lock_irqsave(&mut (*data_vq).lock, &mut flags);
    loop {
        virtqueue_disable_cb(vq);
        while { vc_req = virtqueue_get_buf(vq, &mut len); !vc_req.is_null() } {
            spin_unlock_irqrestore(&mut (*data_vq).lock, flags);
            if let Some(cb) = (*vc_req).alg_cb { cb(vc_req, len); }
            spin_lock_irqsave(&mut (*data_vq).lock, &mut flags);
        }
        if virtqueue_enable_cb(vq) { break; }
    }
    spin_unlock_irqrestore(&mut (*data_vq).lock, flags);
}

unsafe fn virtcrypto_dataq_callback(vq: *mut virtqueue) {
    let vcrypto = (*(*vq).vdev).priv_;
    let dq = &mut (*vcrypto).data_vq[(*vq).index as usize];
    queue_work(system_bh_wq, &mut dq.done_work);
}

unsafe fn virtcrypto_find_vqs(vi: *mut virtio_crypto) -> c_int {
    let total_vqs = (*vi).max_data_queues + 1;
    let mut ret: c_int = -ENOMEM;
    let vqs = kzalloc_objs::<*mut virtqueue>(total_vqs);
    if vqs.is_null() { return ret; }
    let vqs_info = kzalloc_objs::<virtqueue_info>(total_vqs);
    if vqs_info.is_null() { kfree(vqs); return ret; }
    (*vqs_info.add(total_vqs as usize - 1)).callback = Some(virtcrypto_ctrlq_callback);
    (*vqs_info.add(total_vqs as usize - 1)).name = c"controlq".as_ptr();
    for i in 0..(*vi).max_data_queues as usize {
        (*vqs_info.add(i)).callback = Some(virtcrypto_dataq_callback);
        snprintf((*vi).data_vq[i].name.as_mut_ptr(), (*vi).data_vq[i].name.len(), c"dataq.%d".as_ptr(), i);
        (*vqs_info.add(i)).name = (*vi).data_vq[i].name.as_ptr();
    }
    ret = virtio_find_vqs((*vi).vdev, total_vqs, vqs, vqs_info, core::ptr::null_mut());
    if ret != 0 { kfree(vqs_info); kfree(vqs); return ret; }
    (*vi).ctrl_vq = *vqs.add(total_vqs as usize - 1);
    for i in 0..(*vi).max_data_queues as usize {
        spin_lock_init(&mut (*vi).data_vq[i].lock);
        (*vi).data_vq[i].vq = *vqs.add(i);
        (*vi).data_vq[i].engine = crypto_engine_alloc_init_and_set(&mut (*(*vi).vdev).dev, true, true, virtqueue_get_vring_size(*vqs.add(i)));
        if (*vi).data_vq[i].engine.is_null() { ret = -ENOMEM; break; }
        INIT_WORK(&mut (*vi).data_vq[i].done_work, virtcrypto_done_work);
    }
    kfree(vqs_info); kfree(vqs); ret
}

unsafe fn virtcrypto_alloc_queues(vi: *mut virtio_crypto) -> c_int {
    (*vi).data_vq = kzalloc_objs((*vi).max_data_queues);
    if (*vi).data_vq.is_null() { -ENOMEM } else { 0 }
}

unsafe fn virtcrypto_clean_affinity(vi: *mut virtio_crypto, _hcpu: c_long) {
    if (*vi).affinity_hint_set {
        for i in 0..(*vi).max_data_queues as usize { virtqueue_set_affinity((*vi).data_vq[i].vq, core::ptr::null()); }
        (*vi).affinity_hint_set = false;
    }
}

unsafe fn virtcrypto_set_affinity(vcrypto: *mut virtio_crypto) {
    if (*vcrypto).curr_queue == 1 || (*vcrypto).max_data_queues == 1 { virtcrypto_clean_affinity(vcrypto, -1); return; }
    let mut i = 0usize;
    for_each_online_cpu!(cpu, {
        virtqueue_set_affinity((*vcrypto).data_vq[i].vq, cpumask_of(cpu));
        i += 1; if i >= (*vcrypto).max_data_queues as usize { break; }
    });
    (*vcrypto).affinity_hint_set = true;
}

unsafe fn virtcrypto_free_queues(vi: *mut virtio_crypto) { kfree((*vi).data_vq); }

unsafe fn virtcrypto_init_vqs(vi: *mut virtio_crypto) -> c_int {
    let mut ret = virtcrypto_alloc_queues(vi); if ret != 0 { return ret; }
    ret = virtcrypto_find_vqs(vi); if ret != 0 { virtcrypto_free_queues(vi); return ret; }
    cpus_read_lock(); virtcrypto_set_affinity(vi); cpus_read_unlock(); 0
}

unsafe fn virtcrypto_update_status(vcrypto: *mut virtio_crypto) -> c_int {
    let mut status: u32 = 0;
    virtio_cread_le((*vcrypto).vdev, virtio_crypto_config, status, &mut status);
    if status & !VIRTIO_CRYPTO_S_HW_READY != 0 { dev_warn!(&(*(*vcrypto).vdev).dev, "Unknown status bits: 0x{:x}\n", status); virtio_break_device((*vcrypto).vdev); return -EPERM; }
    if (*vcrypto).status == status { return 0; }
    (*vcrypto).status = status;
    if status & VIRTIO_CRYPTO_S_HW_READY != 0 {
        if virtcrypto_dev_start(vcrypto) != 0 { dev_err!(&(*(*vcrypto).vdev).dev, "Failed to start virtio crypto device.\n"); return -EPERM; }
        dev_info!(&(*(*vcrypto).vdev).dev, "Accelerator device is ready\n");
    } else { virtcrypto_dev_stop(vcrypto); dev_info!(&(*(*vcrypto).vdev).dev, "Accelerator is not ready\n"); }
    0
}

// Remaining lifecycle routines retain the C driver's control flow and use the external kernel bindings.
unsafe fn virtcrypto_start_crypto_engines(v: *mut virtio_crypto) -> c_int {
    let mut i: i32 = 0;
    while i < (*v).max_data_queues as i32 { let e = (*v).data_vq[i as usize].engine; if !e.is_null() { let r = crypto_engine_start(e); if r != 0 { while i > 0 { i -= 1; let p = (*v).data_vq[i as usize].engine; if !p.is_null() { crypto_engine_exit(p); } } return r; } } i += 1; } 0
}
unsafe fn virtcrypto_clear_crypto_engines(v: *mut virtio_crypto) { for i in 0..(*v).max_data_queues as usize { let e=(*v).data_vq[i].engine; if !e.is_null(){crypto_engine_exit(e);} } }
unsafe fn virtcrypto_del_vqs(v: *mut virtio_crypto) { virtcrypto_clean_affinity(v,-1); (*(*v).vdev).config.del_vqs((*v).vdev); virtcrypto_free_queues(v); }
unsafe fn vcrypto_config_changed_work(work: *mut work_struct) { let v = container_of!(work, virtio_crypto, config_work); virtcrypto_update_status(v); }

unsafe fn virtcrypto_free_unused_reqs(v: *mut virtio_crypto) {
    for i in 0..(*v).max_data_queues as usize {
        let q = (*v).data_vq[i].vq;
        loop { let req = virtqueue_detach_unused_buf(q); if req.is_null() { break; } virtcrypto_clear_request(req); }
        cond_resched();
    }
}

unsafe fn virtcrypto_remove(vdev: *mut virtio_device) {
    let v = (*vdev).priv_;
    dev_info!(&(*vdev).dev, "Start virtcrypto_remove.\n");
    flush_work(&mut (*v).config_work);
    if virtcrypto_dev_started(v) { virtcrypto_dev_stop(v); }
    for i in 0..(*v).max_data_queues as usize { cancel_work_sync(&mut (*v).data_vq[i].done_work); }
    virtio_reset_device(vdev); virtcrypto_free_unused_reqs(v); virtcrypto_clear_crypto_engines(v); virtcrypto_del_vqs(v); virtcrypto_devmgr_rm_dev(v); kfree(v);
}

unsafe fn virtcrypto_config_changed(vdev: *mut virtio_device) { schedule_work(&mut (*(*vdev).priv_).config_work); }

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn virtcrypto_freeze(vdev: *mut virtio_device) -> c_int {
    let v=(*vdev).priv_; flush_work(&mut (*v).config_work); virtio_reset_device(vdev); virtcrypto_free_unused_reqs(v); if virtcrypto_dev_started(v){virtcrypto_dev_stop(v);} virtcrypto_clear_crypto_engines(v); virtcrypto_del_vqs(v); 0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn virtcrypto_restore(vdev: *mut virtio_device) -> c_int {
    let v=(*vdev).priv_; let mut err=virtcrypto_init_vqs(v); if err!=0{return err;} err=virtcrypto_start_crypto_engines(v); if err!=0{virtio_reset_device(vdev);virtcrypto_del_vqs(v);return err;} virtio_device_ready(vdev); err=virtcrypto_dev_start(v); if err!=0{virtcrypto_clear_crypto_engines(v);virtio_reset_device(vdev);virtcrypto_del_vqs(v);} err
}

static FEATURES: [c_uint; 0] = [];
static ID_TABLE: [virtio_device_id; 2] = [
    virtio_device_id { device: VIRTIO_ID_CRYPTO, vendor: VIRTIO_DEV_ANY_ID },
    virtio_device_id { device: 0, vendor: 0 },
];

static mut virtio_crypto_driver: virtio_driver = virtio_driver {
    driver: driver { name: KBUILD_MODNAME },
    feature_table: FEATURES.as_ptr(), feature_table_size: FEATURES.len(), id_table: ID_TABLE.as_ptr(),
    probe: Some(virtcrypto_probe), remove: Some(virtcrypto_remove), config_changed: Some(virtcrypto_config_changed),
    #[cfg(CONFIG_PM_SLEEP)] freeze: Some(virtcrypto_freeze),
    #[cfg(CONFIG_PM_SLEEP)] restore: Some(virtcrypto_restore),
};

unsafe fn virtcrypto_probe(vdev: *mut virtio_device) -> c_int {
    if !virtio_has_feature(vdev, VIRTIO_F_VERSION_1) { return -ENODEV; }
    if (*vdev).config.get.is_none() { return -EINVAL; }
    let v = kzalloc_node::<virtio_crypto>(dev_to_node(&(*vdev).dev)); if v.is_null(){return -ENOMEM;}
    (*v).vdev=vdev; (*v).curr_queue=1; (*v).max_data_queues=1;
    let err=virtcrypto_devmgr_add_dev(v); if err!=0{kfree(v);return err;} (*vdev).priv_=v;
    let mut err=virtcrypto_init_vqs(v); if err!=0{virtcrypto_devmgr_rm_dev(v);kfree(v);return err;}
    err=virtcrypto_start_crypto_engines(v); if err!=0{virtcrypto_del_vqs(v);virtcrypto_devmgr_rm_dev(v);kfree(v);return err;}
    virtio_device_ready(vdev); err=virtcrypto_update_status(v); if err!=0{virtcrypto_clear_crypto_engines(v);virtio_reset_device(vdev);virtcrypto_del_vqs(v);virtcrypto_devmgr_rm_dev(v);kfree(v);return err;}
    INIT_WORK(&mut (*v).config_work,vcrypto_config_changed_work); 0
}

module_virtio_driver!(virtio_crypto_driver);
MODULE_DEVICE_TABLE!(virtio, ID_TABLE);
MODULE_DESCRIPTION!("virtio crypto device driver");
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Gonglei <arei.gonglei@huawei.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
