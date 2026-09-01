// SPDX-License-Identifier: GPL-2.0-only
/*
 * VFIO-KVM bridge pseudo device
 *
 * Copyright (C) 2013 Red Hat, Inc.  All rights reserved.
 *     Author: Alex Williamson <alex.williamson@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_t = bool;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type int32_t = core::ffi::c_int;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm {
    pub lock: mutex,
    pub devices: list_head,
}

#[repr(C)]
pub struct kvm_device {
    pub kvm: *mut kvm,
    pub ops: *const kvm_device_ops,
    pub private: *mut c_void,
    pub vm_node: list_head,
}

#[repr(C)]
pub struct kvm_device_attr {
    pub flags: u32,
    pub group: u32,
    pub attr: u64,
    pub addr: u64,
}

#[repr(C)]
pub struct kvm_device_ops {
    pub name: *const c_char,
    pub create: Option<unsafe extern "C" fn(*mut kvm_device, u32) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut kvm_device) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut kvm_device)>,
    pub release: Option<unsafe extern "C" fn(*mut kvm_device)>,
    pub set_attr: Option<unsafe extern "C" fn(*mut kvm_device, *mut kvm_device_attr) -> c_int>,
    pub get_attr: Option<unsafe extern "C" fn(*mut kvm_device, *mut kvm_device_attr) -> c_int>,
    pub has_attr: Option<unsafe extern "C" fn(*mut kvm_device, *mut kvm_device_attr) -> c_int>,
}

#[repr(C)]
pub struct kvm_vfio_spapr_tce {
    pub groupfd: c_int,
    pub tablefd: c_int,
}

#[repr(C)]
pub struct iommu_group {
    _private: [u8; 0],
}

#[repr(C)]
struct kvm_vfio_file {
    node: list_head,
    file: *mut file,
    /*
     * Present when CONFIG_SPAPR_TCE_IOMMU is enabled:
     * iommu_group: *mut iommu_group,
     */
    iommu_group: *mut iommu_group,
}

#[repr(C)]
struct kvm_vfio {
    file_list: list_head,
    lock: mutex,
    noncoherent: bool_t,
}

const EBADF: c_int = 9;
const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EFAULT: c_int = 14;
const EIO: c_int = 5;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;

const GFP_KERNEL_ACCOUNT: c_uint = 0;
const KVM_DEV_VFIO_FILE: u32 = 1;
const KVM_DEV_VFIO_FILE_ADD: u64 = 1;
const KVM_DEV_VFIO_FILE_DEL: u64 = 2;
const KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE: u64 = 3;
const KVM_DEV_TYPE_VFIO: u32 = 1;

unsafe extern "C" {
    static vfio_file_set_kvm: *const c_void;
    static vfio_file_enforced_coherent: *const c_void;
    static vfio_file_is_valid: *const c_void;
    static vfio_file_iommu_group: *const c_void;

    fn symbol_get(symbol: *const c_void) -> *const c_void;
    fn symbol_put(symbol: *const c_void);

    fn fget(fd: c_uint) -> *mut file;
    fn get_file(file: *mut file) -> *mut file;
    fn fput(file: *mut file);

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn get_user_i32(value: *mut int32_t, ptr: *const int32_t) -> c_int;
    fn u64_to_user_ptr(addr: u64) -> *mut c_void;

    fn WARN_ON_ONCE(condition: bool_t) -> bool_t;
    fn lockdep_assert_held(lock: *mut mutex);

    fn kvm_arch_register_noncoherent_dma(kvm: *mut kvm);
    fn kvm_arch_unregister_noncoherent_dma(kvm: *mut kvm);
    fn kvm_register_device_ops(ops: *const kvm_device_ops, ty: u32) -> c_int;
    fn kvm_unregister_device_ops(ty: u32);

    fn kvm_spapr_tce_release_iommu_group(kvm: *mut kvm, group: *mut iommu_group);
    fn iommu_group_put(group: *mut iommu_group);
    fn kvm_spapr_tce_attach_iommu_group(
        kvm: *mut kvm,
        tablefd: c_int,
        group: *mut iommu_group,
    ) -> c_int;
}

type c_ulong = core::ffi::c_ulong;

unsafe fn container_of_kvm_vfio_file(ptr: *mut list_head) -> *mut kvm_vfio_file {
    (ptr as *mut u8).sub(core::mem::offset_of!(kvm_vfio_file, node)) as *mut kvm_vfio_file
}

unsafe fn container_of_kvm_device(ptr: *mut list_head) -> *mut kvm_device {
    (ptr as *mut u8).sub(core::mem::offset_of!(kvm_device, vm_node)) as *mut kvm_device
}

unsafe fn kvm_vfio_file_set_kvm(file: *mut file, kvm: *mut kvm) {
    let mut fn_: Option<unsafe extern "C" fn(*mut file, *mut kvm)>;

    fn_ = core::mem::transmute(symbol_get(vfio_file_set_kvm));
    if fn_.is_none() {
        return;
    }

    fn_.unwrap()(file, kvm);

    symbol_put(vfio_file_set_kvm);
}

unsafe fn kvm_vfio_file_enforced_coherent(file: *mut file) -> bool_t {
    let mut fn_: Option<unsafe extern "C" fn(*mut file) -> bool_t>;
    let ret: bool_t;

    fn_ = core::mem::transmute(symbol_get(vfio_file_enforced_coherent));
    if fn_.is_none() {
        return false;
    }

    ret = fn_.unwrap()(file);

    symbol_put(vfio_file_enforced_coherent);

    ret
}

unsafe fn kvm_vfio_file_is_valid(file: *mut file) -> bool_t {
    let mut fn_: Option<unsafe extern "C" fn(*mut file) -> bool_t>;
    let ret: bool_t;

    fn_ = core::mem::transmute(symbol_get(vfio_file_is_valid));
    if fn_.is_none() {
        return false;
    }

    ret = fn_.unwrap()(file);

    symbol_put(vfio_file_is_valid);

    ret
}

/* CONFIG_SPAPR_TCE_IOMMU */
unsafe fn kvm_vfio_file_iommu_group(file: *mut file) -> *mut iommu_group {
    let mut fn_: Option<unsafe extern "C" fn(*mut file) -> *mut iommu_group>;
    let ret: *mut iommu_group;

    fn_ = core::mem::transmute(symbol_get(vfio_file_iommu_group));
    if fn_.is_none() {
        return core::ptr::null_mut();
    }

    ret = fn_.unwrap()(file);

    symbol_put(vfio_file_iommu_group);

    ret
}

/* CONFIG_SPAPR_TCE_IOMMU */
unsafe fn kvm_spapr_tce_release_vfio_group(kvm: *mut kvm, kvf: *mut kvm_vfio_file) {
    if WARN_ON_ONCE((*kvf).iommu_group.is_null()) {
        return;
    }

    kvm_spapr_tce_release_iommu_group(kvm, (*kvf).iommu_group);
    iommu_group_put((*kvf).iommu_group);
    (*kvf).iommu_group = core::ptr::null_mut();
}

/*
 * Groups/devices can use the same or different IOMMU domains. If the same
 * then adding a new group/device may change the coherency of groups/devices
 * we've previously been told about. We don't want to care about any of
 * that so we retest each group/device and bail as soon as we find one that's
 * noncoherent.  This means we only ever [un]register_noncoherent_dma once
 * for the whole device.
 */
unsafe fn kvm_vfio_update_coherency(dev: *mut kvm_device) {
    let kv: *mut kvm_vfio = (*dev).private as *mut kvm_vfio;
    let mut noncoherent: bool_t = false;
    let mut pos: *mut list_head;

    pos = (*kv).file_list.next;
    while pos != &mut (*kv).file_list {
        let kvf: *mut kvm_vfio_file = container_of_kvm_vfio_file(pos);
        if !kvm_vfio_file_enforced_coherent((*kvf).file) {
            noncoherent = true;
            break;
        }
        pos = (*pos).next;
    }

    if noncoherent != (*kv).noncoherent {
        (*kv).noncoherent = noncoherent;

        if (*kv).noncoherent {
            kvm_arch_register_noncoherent_dma((*dev).kvm);
        } else {
            kvm_arch_unregister_noncoherent_dma((*dev).kvm);
        }
    }
}

unsafe fn kvm_vfio_file_add(dev: *mut kvm_device, fd: c_uint) -> c_int {
    let kv: *mut kvm_vfio = (*dev).private as *mut kvm_vfio;
    let mut pos: *mut list_head;
    let mut filp: *mut file = core::ptr::null_mut();

    filp = fget(fd);
    if filp.is_null() {
        return -EBADF;
    }

    /* Ensure the FD is a vfio FD. */
    if !kvm_vfio_file_is_valid(filp) {
        fput(filp);
        return -EINVAL;
    }

    mutex_lock(&mut (*kv).lock);

    pos = (*kv).file_list.next;
    while pos != &mut (*kv).file_list {
        let kvf: *mut kvm_vfio_file = container_of_kvm_vfio_file(pos);
        if (*kvf).file == filp {
            mutex_unlock(&mut (*kv).lock);
            fput(filp);
            return -EEXIST;
        }
        pos = (*pos).next;
    }

    let kvf: *mut kvm_vfio_file =
        kzalloc(core::mem::size_of::<kvm_vfio_file>(), GFP_KERNEL_ACCOUNT) as *mut kvm_vfio_file;
    if kvf.is_null() {
        mutex_unlock(&mut (*kv).lock);
        fput(filp);
        return -ENOMEM;
    }

    (*kvf).file = get_file(filp);
    list_add_tail(&mut (*kvf).node, &mut (*kv).file_list);

    kvm_vfio_file_set_kvm((*kvf).file, (*dev).kvm);
    kvm_vfio_update_coherency(dev);

    mutex_unlock(&mut (*kv).lock);
    fput(filp);

    0
}

unsafe fn kvm_vfio_file_free(dev: *mut kvm_device, kvf: *mut kvm_vfio_file) {
    /* CONFIG_SPAPR_TCE_IOMMU */
    kvm_spapr_tce_release_vfio_group((*dev).kvm, kvf);
    kvm_vfio_file_set_kvm((*kvf).file, core::ptr::null_mut());
    fput((*kvf).file);
    list_del(&mut (*kvf).node);
    kfree(kvf as *mut c_void);
}

unsafe fn kvm_vfio_file_del(dev: *mut kvm_device, fd: c_uint) -> c_int {
    let kv: *mut kvm_vfio = (*dev).private as *mut kvm_vfio;
    let f: *mut file = fget(fd);
    let mut pos: *mut list_head;

    if f.is_null() {
        return -EBADF;
    }

    mutex_lock(&mut (*kv).lock);

    pos = (*kv).file_list.next;
    while pos != &mut (*kv).file_list {
        let kvf: *mut kvm_vfio_file = container_of_kvm_vfio_file(pos);
        if (*kvf).file == f {
            kvm_vfio_file_free(dev, kvf);
            kvm_vfio_update_coherency(dev);
            mutex_unlock(&mut (*kv).lock);
            fput(f);
            return 0;
        }
        pos = (*pos).next;
    }

    mutex_unlock(&mut (*kv).lock);
    fput(f);

    -ENOENT
}

/* CONFIG_SPAPR_TCE_IOMMU */
unsafe fn kvm_vfio_file_set_spapr_tce(dev: *mut kvm_device, arg: *mut c_void) -> c_int {
    let mut param: kvm_vfio_spapr_tce = core::mem::zeroed();
    let kv: *mut kvm_vfio = (*dev).private as *mut kvm_vfio;
    let mut pos: *mut list_head;

    if copy_from_user(
        &mut param as *mut _ as *mut c_void,
        arg as *const c_void,
        core::mem::size_of::<kvm_vfio_spapr_tce>(),
    ) != 0
    {
        return -EFAULT;
    }

    let f: *mut file = fget(param.groupfd as c_uint);
    if f.is_null() {
        return -EBADF;
    }

    mutex_lock(&mut (*kv).lock);

    pos = (*kv).file_list.next;
    while pos != &mut (*kv).file_list {
        let kvf: *mut kvm_vfio_file = container_of_kvm_vfio_file(pos);
        if (*kvf).file != f {
            pos = (*pos).next;
            continue;
        }

        if (*kvf).iommu_group.is_null() {
            (*kvf).iommu_group = kvm_vfio_file_iommu_group((*kvf).file);
            if WARN_ON_ONCE((*kvf).iommu_group.is_null()) {
                mutex_unlock(&mut (*kv).lock);
                fput(f);
                return -EIO;
            }
        }

        let ret = kvm_spapr_tce_attach_iommu_group((*dev).kvm, param.tablefd, (*kvf).iommu_group);
        mutex_unlock(&mut (*kv).lock);
        fput(f);
        return ret;
    }

    mutex_unlock(&mut (*kv).lock);
    fput(f);

    -ENOENT
}

unsafe fn kvm_vfio_set_file(dev: *mut kvm_device, attr: c_long, arg: *mut c_void) -> c_int {
    let argp: *mut int32_t = arg as *mut int32_t;
    let mut fd: int32_t = 0;

    match attr as u64 {
        KVM_DEV_VFIO_FILE_ADD => {
            if get_user_i32(&mut fd, argp as *const int32_t) != 0 {
                return -EFAULT;
            }
            return kvm_vfio_file_add(dev, fd as c_uint);
        }

        KVM_DEV_VFIO_FILE_DEL => {
            if get_user_i32(&mut fd, argp as *const int32_t) != 0 {
                return -EFAULT;
            }
            return kvm_vfio_file_del(dev, fd as c_uint);
        }

        /* CONFIG_SPAPR_TCE_IOMMU */
        KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE => {
            return kvm_vfio_file_set_spapr_tce(dev, arg);
        }

        _ => {}
    }

    -ENXIO
}

unsafe fn kvm_vfio_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    match (*attr).group {
        KVM_DEV_VFIO_FILE => {
            return kvm_vfio_set_file(dev, (*attr).attr as c_long, u64_to_user_ptr((*attr).addr));
        }
        _ => {}
    }

    -ENXIO
}

unsafe fn kvm_vfio_has_attr(_dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    match (*attr).group {
        KVM_DEV_VFIO_FILE => {
            match (*attr).attr {
                KVM_DEV_VFIO_FILE_ADD | KVM_DEV_VFIO_FILE_DEL => {
                    return 0;
                }

                /* CONFIG_SPAPR_TCE_IOMMU */
                KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE => {
                    return 0;
                }

                _ => {}
            }

            return -ENXIO;
        }
        _ => {}
    }

    -ENXIO
}

unsafe fn kvm_vfio_release(dev: *mut kvm_device) {
    let kv: *mut kvm_vfio = (*dev).private as *mut kvm_vfio;
    let mut pos: *mut list_head;
    let mut tmp: *mut list_head;

    pos = (*kv).file_list.next;
    while pos != &mut (*kv).file_list {
        tmp = (*pos).next;
        let kvf: *mut kvm_vfio_file = container_of_kvm_vfio_file(pos);
        kvm_vfio_file_free(dev, kvf);
        pos = tmp;
    }

    kvm_vfio_update_coherency(dev);

    kfree(kv as *mut c_void);
    kfree(dev as *mut c_void); /* alloc by kvm_ioctl_create_device, free by .release */
}

unsafe fn kvm_vfio_create(dev: *mut kvm_device, _type: u32) -> c_int {
    let mut pos: *mut list_head;
    let kv: *mut kvm_vfio;

    lockdep_assert_held(&mut (*(*dev).kvm).lock);

    /* Only one VFIO "device" per VM */
    pos = (*(*dev).kvm).devices.next;
    while pos != &mut (*(*dev).kvm).devices {
        let tmp: *mut kvm_device = container_of_kvm_device(pos);
        if (*tmp).ops == &kvm_vfio_ops {
            return -EBUSY;
        }
        pos = (*pos).next;
    }

    kv = kzalloc(core::mem::size_of::<kvm_vfio>(), GFP_KERNEL_ACCOUNT) as *mut kvm_vfio;
    if kv.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*kv).file_list);
    mutex_init(&mut (*kv).lock);

    (*dev).private = kv as *mut c_void;

    0
}

static KVM_VFIO_NAME: &[u8] = b"kvm-vfio\0";

static kvm_vfio_ops: kvm_device_ops = kvm_device_ops {
    name: KVM_VFIO_NAME.as_ptr() as *const c_char,
    create: Some(kvm_vfio_create),
    init: None,
    destroy: None,
    release: Some(kvm_vfio_release),
    set_attr: Some(kvm_vfio_set_attr),
    get_attr: None,
    has_attr: Some(kvm_vfio_has_attr),
};

#[no_mangle]
pub unsafe extern "C" fn kvm_vfio_ops_init() -> c_int {
    kvm_register_device_ops(&kvm_vfio_ops, KVM_DEV_TYPE_VFIO)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_vfio_ops_exit() {
    kvm_unregister_device_ops(KVM_DEV_TYPE_VFIO);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
