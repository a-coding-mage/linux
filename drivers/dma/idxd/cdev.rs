// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */
// Linux kernel dependencies and build-time configuration are supplied by the surrounding crate.

#[repr(C)]
pub struct idxd_cdev_context {
    pub name: *const core::ffi::c_char,
    pub devt: dev_t,
    pub minor_ida: ida,
}

static mut file_ida: ida = ida_INIT;

static mut ictx: [idxd_cdev_context; IDXD_TYPE_MAX as usize] = [
    idxd_cdev_context { name: b"dsa\0".as_ptr() as _, devt: 0, minor_ida: ida_INIT },
    idxd_cdev_context { name: b"iax\0".as_ptr() as _, devt: 0, minor_ida: ida_INIT },
];

#[repr(C)]
pub struct idxd_user_context {
    pub wq: *mut idxd_wq,
    pub task: *mut task_struct,
    pub pasid: u32,
    pub mm: *mut mm_struct,
    pub flags: u32,
    pub sva: *mut iommu_sva,
    pub idxd_dev: idxd_dev,
    pub counters: [u64; COUNTER_MAX as usize],
    pub id: i32,
    pub pid: pid_t,
}

extern "C" {
    fn idxd_cdev_evl_drain_pasid(wq: *mut idxd_wq, pasid: u32);
    fn idxd_xa_pasid_remove(ctx: *mut idxd_user_context);
}

unsafe fn dev_to_uctx(dev: *mut device) -> *mut idxd_user_context {
    container_of(confdev_to_idxd_dev(dev), idxd_user_context, idxd_dev)
}

unsafe fn cr_faults_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let ctx = dev_to_uctx(dev);
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as _, (*ctx).counters[COUNTER_FAULTS as usize])
}
unsafe fn cr_fault_failures_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let ctx = dev_to_uctx(dev);
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as _, (*ctx).counters[COUNTER_FAULT_FAILS as usize])
}
unsafe fn pid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let ctx = dev_to_uctx(dev);
    sysfs_emit(buf, b"%u\n\0".as_ptr() as _, (*ctx).pid)
}

unsafe fn cdev_file_attr_visible(kobj: *mut kobject, a: *mut attribute, _n: i32) -> umode_t {
    let dev = container_of(kobj, device, kobj);
    let ctx = dev_to_uctx(dev);
    if !wq_pasid_enabled((*ctx).wq) { 0 } else { (*a).mode }
}

unsafe fn idxd_file_dev_release(dev: *mut device) {
    let ctx = dev_to_uctx(dev); let wq = (*ctx).wq; let idxd = (*wq).idxd;
    ida_free(&mut file_ida, (*ctx).id);
    if wq_shared(wq) { idxd_device_drain_pasid(idxd, (*ctx).pasid); }
    else if device_user_pasid_enabled(idxd) { let rc = idxd_wq_disable_pasid(wq); if rc < 0 { dev_err(dev, b"wq disable pasid failed.\n\0".as_ptr() as _); } }
    else { idxd_wq_drain(wq); }
    if !(*ctx).sva.is_null() { idxd_cdev_evl_drain_pasid(wq, (*ctx).pasid); iommu_sva_unbind_device((*ctx).sva); idxd_xa_pasid_remove(ctx); }
    kfree(ctx); mutex_lock(&mut (*wq).wq_lock); idxd_wq_put(wq); mutex_unlock(&mut (*wq).wq_lock);
}

unsafe fn idxd_cdev_dev_release(dev: *mut device) { kfree(dev_to_cdev(dev)); }
unsafe fn inode_idxd_cdev(inode: *mut inode) -> *mut idxd_cdev { container_of((*inode).i_cdev, idxd_cdev, cdev) }
unsafe fn inode_wq(inode: *mut inode) -> *mut idxd_wq { (*inode_idxd_cdev(inode)).wq }

unsafe fn idxd_xa_pasid_remove_impl(ctx: *mut idxd_user_context) {
    let wq = (*ctx).wq; mutex_lock(&mut (*wq).uc_lock);
    let p = xa_cmpxchg(&mut (*wq).upasid_xa, (*ctx).pasid as _, ctx as _, core::ptr::null_mut(), GFP_KERNEL);
    if p != ctx as _ { dev_warn(&mut (*(*wq).idxd).pdev.dev, b"xarray cmpxchg failed for pasid %u\n\0".as_ptr() as _, (*ctx).pasid); }
    mutex_unlock(&mut (*wq).uc_lock);
}

pub unsafe fn idxd_user_counter_increment(wq: *mut idxd_wq, pasid: u32, index: i32) {
    if index >= COUNTER_MAX { return; } mutex_lock(&mut (*wq).uc_lock);
    let ctx = xa_load(&mut (*wq).upasid_xa, pasid as _) as *mut idxd_user_context;
    if !ctx.is_null() { (*ctx).counters[index as usize] = (*ctx).counters[index as usize].wrapping_add(1); }
    mutex_unlock(&mut (*wq).uc_lock);
}

// The remaining file-local operations retain the kernel ABI and are expressed as external Rust declarations.
extern "C" {
    fn idxd_cdev_open(inode: *mut inode, filp: *mut file) -> i32;
    fn idxd_cdev_release(inode: *mut inode, filep: *mut file) -> i32;
    fn idxd_cdev_mmap(filp: *mut file, vma: *mut vm_area_struct) -> i32;
    fn idxd_cdev_write(filp: *mut file, buf: *const c_char, len: usize, unused: *mut loff_t) -> ssize_t;
    fn idxd_cdev_poll(filp: *mut file, wait: *mut poll_table_struct) -> __poll_t;
    fn idxd_cdev_get_major(idxd: *mut idxd_device) -> i32;
    fn idxd_wq_add_cdev(wq: *mut idxd_wq) -> i32;
    fn idxd_wq_del_cdev(wq: *mut idxd_wq);
    fn idxd_cdev_register() -> i32;
    fn idxd_cdev_remove();
    fn idxd_copy_cr(wq: *mut idxd_wq, pasid: ioasid_t, addr: usize, cr: *mut c_void, len: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
