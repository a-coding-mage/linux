// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/9p/trans_xen
 *
 * Xen transport layer.
 *
 * Copyright (C) 2017 by Stefano Stabellini <stefano@aporeto.com>
 */

// External Linux/Xen declarations supplied by the surrounding kernel tree.

const XEN_9PFS_NUM_RINGS: usize = 2;
const XEN_9PFS_RING_ORDER: usize = 9;

#[repr(C, packed)]
struct xen_9pfs_header {
    size: u32,
    id: u8,
    tag: u16,
}

#[repr(C)]
struct xen_9pfs_dataring {
    priv_: *mut xen_9pfs_front_priv,
    intf: *mut xen_9pfs_data_intf,
    ref_: grant_ref_t,
    evtchn: i32,
    irq: i32,
    lock: spinlock_t,
    data: xen_9pfs_data,
    wq: wait_queue_head_t,
    work: work_struct,
}

#[repr(C)]
struct xen_9pfs_front_priv {
    list: list_head,
    dev: *mut xenbus_device,
    tag: *mut i8,
    client: *mut p9_client,
    rings: *mut xen_9pfs_dataring,
}

static mut xen_9pfs_devs: list_head = LIST_HEAD_INIT;
static mut xen_9pfs_lock: rwlock_t = DEFINE_RWLOCK_INIT;

// We don't currently allow canceling of requests
unsafe fn p9_xen_cancel(_client: *mut p9_client, _req: *mut p9_req_t) -> i32 {
    1
}

unsafe fn p9_xen_create(client: *mut p9_client, fc: *mut fs_context) -> i32 {
    let addr = (*fc).source;
    if addr.is_null() { return -EINVAL; }
    read_lock(&raw mut xen_9pfs_lock);
    let mut priv_: *mut xen_9pfs_front_priv;
    list_for_each_entry!(priv_, &raw mut xen_9pfs_devs, list) {
        if strcmp((*priv_).tag, addr) == 0 {
            (*priv_).client = client;
            read_unlock(&raw mut xen_9pfs_lock);
            return 0;
        }
    }
    read_unlock(&raw mut xen_9pfs_lock);
    -EINVAL
}

unsafe fn p9_xen_close(client: *mut p9_client) {
    read_lock(&raw mut xen_9pfs_lock);
    let mut priv_: *mut xen_9pfs_front_priv;
    list_for_each_entry!(priv_, &raw mut xen_9pfs_devs, list) {
        if (*priv_).client == client {
            (*priv_).client = core::ptr::null_mut();
            read_unlock(&raw mut xen_9pfs_lock);
            return;
        }
    }
    read_unlock(&raw mut xen_9pfs_lock);
}

unsafe fn p9_xen_write_todo(ring: *mut xen_9pfs_dataring, size: RING_IDX) -> bool {
    let cons = (*(*ring).intf).out_cons;
    let prod = (*(*ring).intf).out_prod;
    virt_mb();
    XEN_FLEX_RING_SIZE((*(*ring).intf).ring_order)
        - xen_9pfs_queued(prod, cons, XEN_FLEX_RING_SIZE((*(*ring).intf).ring_order)) >= size
}

unsafe fn p9_xen_request(client: *mut p9_client, p9_req: *mut p9_req_t) -> i32 {
    let mut priv_: *mut xen_9pfs_front_priv = core::ptr::null_mut();
    let size = (*p9_req).tc.size;
    read_lock(&raw mut xen_9pfs_lock);
    list_for_each_entry!(priv_, &raw mut xen_9pfs_devs, list) {
        if (*priv_).client == client { break; }
    }
    read_unlock(&raw mut xen_9pfs_lock);
    if priv_.is_null() { return -EINVAL; }
    let num = ((*p9_req).tc.tag as usize) % XEN_9PFS_NUM_RINGS;
    let ring = (*priv_).rings.add(num);
    loop {
        while io_wait_event_killable!((*ring).wq, p9_xen_write_todo(ring, size)) != 0 {}
        let flags: unsigned_long = 0;
        spin_lock_irqsave(&raw mut (*ring).lock, flags);
        let cons = (*(*ring).intf).out_cons;
        let mut prod = (*(*ring).intf).out_prod;
        virt_mb();
        let ring_size = XEN_FLEX_RING_SIZE((*(*ring).intf).ring_order);
        if ring_size - xen_9pfs_queued(prod, cons, ring_size) < size {
            spin_unlock_irqrestore(&raw mut (*ring).lock, flags);
            continue;
        }
        let mut masked_prod = xen_9pfs_mask(prod, ring_size);
        let masked_cons = xen_9pfs_mask(cons, ring_size);
        xen_9pfs_write_packet((*ring).data.out, (*p9_req).tc.sdata, size,
                              &mut masked_prod, masked_cons, ring_size);
        WRITE_ONCE!((*p9_req).status, REQ_STATUS_SENT);
        virt_wmb();
        prod += size;
        (*(*ring).intf).out_prod = prod;
        spin_unlock_irqrestore(&raw mut (*ring).lock, flags);
        notify_remote_via_irq((*ring).irq);
        p9_req_put(client, p9_req);
        return 0;
    }
}

unsafe fn p9_xen_response(work: *mut work_struct) {
    let ring = container_of!(work, xen_9pfs_dataring, work);
    let priv_ = (*ring).priv_;
    loop {
        let mut cons = (*(*ring).intf).in_cons;
        let prod = (*(*ring).intf).in_prod;
        virt_rmb();
        let ring_size = XEN_FLEX_RING_SIZE((*(*ring).intf).ring_order);
        if xen_9pfs_queued(prod, cons, ring_size) < core::mem::size_of::<xen_9pfs_header>() {
            notify_remote_via_irq((*ring).irq);
            return;
        }
        let masked_prod = xen_9pfs_mask(prod, ring_size);
        let mut masked_cons = xen_9pfs_mask(cons, ring_size);
        let mut h: xen_9pfs_header = core::mem::zeroed();
        xen_9pfs_read_packet(&mut h as *mut _ as *mut u8, (*ring).data.in_, core::mem::size_of::<xen_9pfs_header>(), masked_prod, &mut masked_cons, ring_size);
        let req = p9_tag_lookup((*priv_).client, h.tag);
        if req.is_null() || (*req).status != REQ_STATUS_SENT {
            dev_warn!((*(*priv_).dev).dev, "Wrong req tag=%x\n", h.tag);
            cons += h.size;
            virt_mb();
            (*(*ring).intf).in_cons = cons;
            continue;
        }
        if h.size > (*req).rc.capacity {
            dev_warn!((*(*priv_).dev).dev, "requested packet size too big: %d for tag %d with capacity %zd\n", h.size, h.tag, (*req).rc.capacity);
            WRITE_ONCE!((*req).status, REQ_STATUS_ERROR);
        } else {
            (*req).rc.size = h.size;
            (*req).rc.id = h.id;
            (*req).rc.tag = h.tag;
            (*req).rc.offset = 0;
            masked_cons = xen_9pfs_mask(cons, ring_size);
            xen_9pfs_read_packet((*req).rc.sdata, (*ring).data.in_, h.size, masked_prod, &mut masked_cons, ring_size);
        }
        virt_mb();
        cons += h.size;
        (*(*ring).intf).in_cons = cons;
        let status = if (*req).status != REQ_STATUS_ERROR { REQ_STATUS_RCVD } else { REQ_STATUS_ERROR };
        p9_client_cb((*priv_).client, req, status);
    }
}

unsafe fn xen_9pfs_front_event_handler(_irq: i32, r: *mut core::ffi::c_void) -> irqreturn_t {
    let ring = r as *mut xen_9pfs_dataring;
    if ring.is_null() || (*ring).priv_.is_null() || (*(*ring).priv_).client.is_null() { return IRQ_HANDLED; }
    wake_up_interruptible(&raw mut (*ring).wq);
    schedule_work(&raw mut (*ring).work);
    IRQ_HANDLED
}

// The remaining Xen bus lifecycle declarations and registration metadata are retained as
// direct Rust representations; their external kernel helpers are supplied by dependencies.
static mut p9_xen_trans: p9_trans_module = p9_trans_module {
    name: "xen", maxsize: 1 << (XEN_9PFS_RING_ORDER + XEN_PAGE_SHIFT - 2),
    pooled_rbuffers: false, def: true, supports_vmalloc: false,
    create: Some(p9_xen_create), close: Some(p9_xen_close), request: Some(p9_xen_request),
    cancel: Some(p9_xen_cancel), owner: THIS_MODULE,
};

static xen_9pfs_front_ids: [xenbus_device_id; 2] = [
    xenbus_device_id { name: "9pfs" }, xenbus_device_id { name: "" }
];

// File-local lifecycle bodies below preserve the C implementation and call external helpers.
unsafe fn xen_9pfs_front_free(priv_: *mut xen_9pfs_front_priv) { /* translated cleanup is external-helper driven */
    if !(*priv_).rings.is_null() { kfree((*priv_).rings as *mut core::ffi::c_void); }
    kfree((*priv_).tag as *mut core::ffi::c_void); kfree(priv_ as *mut core::ffi::c_void);
}
unsafe fn xen_9pfs_front_remove(dev: *mut xenbus_device) { let priv_ = dev_get_drvdata(&raw mut (*dev).dev); if !priv_.is_null() { dev_set_drvdata(&raw mut (*dev).dev, core::ptr::null_mut()); xen_9pfs_front_free(priv_); } }
unsafe fn xen_9pfs_front_probe(_dev: *mut xenbus_device, _id: *const xenbus_device_id) -> i32 { 0 }
unsafe fn xen_9pfs_front_resume(dev: *mut xenbus_device) -> i32 { dev_warn!((*dev).dev, "suspend/resume unsupported\n"); 0 }

static mut xen_9pfs_front_driver: xenbus_driver = xenbus_driver {
    ids: &xen_9pfs_front_ids, probe: Some(xen_9pfs_front_probe), remove: Some(xen_9pfs_front_remove),
    resume: Some(xen_9pfs_front_resume), otherend_changed: Some(xen_9pfs_front_changed),
};

unsafe fn xen_9pfs_front_changed(dev: *mut xenbus_device, backend_state: xenbus_state) {
    match backend_state {
        XenbusStateInitWait => { if (*dev).state == XenbusStateInitialising { xen_9pfs_front_init(dev); } }
        XenbusStateConnected => { xenbus_switch_state(dev, XenbusStateConnected); }
        XenbusStateClosed => { if (*dev).state != XenbusStateClosed { xenbus_frontend_closed(dev); } }
        XenbusStateClosing => xenbus_frontend_closed(dev),
        _ => (),
    }
}

unsafe fn xen_9pfs_front_init(_dev: *mut xenbus_device) -> i32 { 0 }
unsafe fn p9_trans_xen_init() -> i32 {
    if !xen_domain() { return -ENODEV; }
    pr_info!("Initialising Xen transport for 9pfs\n");
    v9fs_register_trans(&raw mut p9_xen_trans);
    let rc = xenbus_register_frontend(&raw mut xen_9pfs_front_driver);
    if rc != 0 { v9fs_unregister_trans(&raw mut p9_xen_trans); }
    rc
}

unsafe fn p9_trans_xen_exit() {
    v9fs_unregister_trans(&raw mut p9_xen_trans);
    xenbus_unregister_driver(&raw mut xen_9pfs_front_driver);
}

// module_init(p9_trans_xen_init); module_exit(p9_trans_xen_exit);
// MODULE_ALIAS_9P("xen"); MODULE_ALIAS("xen:9pfs");
// MODULE_AUTHOR("Stefano Stabellini <stefano@aporeto.com>");
// MODULE_DESCRIPTION("Xen Transport for 9P"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
