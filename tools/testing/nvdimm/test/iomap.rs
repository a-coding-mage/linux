// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 */

// Translated from testing/nvdimm/test/iomap.c.
// C includes referenced linux/memremap.h, linux/rculist.h, linux/export.h,
// linux/ioport.h, linux/module.h, linux/types.h, linux/acpi.h, linux/io.h,
// linux/mm.h, and "nfit_test.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub type resource_size_t = u64;
pub type size_t = usize;
pub type u64_t = u64;
pub type bool_t = bool;
pub type acpi_status = c_int;
pub type acpi_handle = *mut c_void;
pub type acpi_string = *const c_char;
pub type nfit_test_lookup_fn =
    Option<unsafe extern "C" fn(resource: resource_size_t) -> *mut nfit_test_resource>;
pub type nfit_test_evaluate_dsm_fn = Option<
    unsafe extern "C" fn(
        handle: acpi_handle,
        guid: *const guid_t,
        rev: u64_t,
        func: u64_t,
        argv4: *mut acpi_object,
    ) -> *mut acpi_object,
>;

pub const GFP_KERNEL: c_int = 0;
pub const IORESOURCE_BUSY: c_ulong = 0x8000_0000;
pub const ENXIO: c_int = 6;
pub const AE_OK: acpi_status = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const c_char,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct percpu_ref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct range {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct dev_pagemap {
    pub range: range,
    pub ref_: percpu_ref,
    pub done: completion,
}

#[repr(C)]
pub struct nfit_test_request {
    pub res: resource,
    pub list: list_head,
}

#[repr(C)]
pub struct nfit_test_resource {
    pub res: resource,
    pub buf: *mut u8,
    pub lock: spinlock_t,
    pub requests: list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct guid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_object_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_buffer {
    pub length: size_t,
    pub pointer: *mut c_void,
}

#[repr(C)]
struct iomap_ops {
    nfit_test_lookup: nfit_test_lookup_fn,
    evaluate_dsm: nfit_test_evaluate_dsm_fn,
    list: list_head,
}

static mut iomap_head: list_head = list_head {
    next: ptr::addr_of_mut!(iomap_head),
    prev: ptr::addr_of_mut!(iomap_head),
};

static mut iomap_ops: iomap_ops = iomap_ops {
    nfit_test_lookup: None,
    evaluate_dsm: None,
    list: list_head {
        next: ptr::addr_of_mut!(iomap_ops.list),
        prev: ptr::addr_of_mut!(iomap_ops.list),
    },
};

unsafe extern "C" {
    static mut iomem_resource: resource;

    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn synchronize_rcu();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: c_ulong) -> *mut c_void;
    fn devm_memremap(
        dev: *mut device,
        offset: resource_size_t,
        size: size_t,
        flags: c_ulong,
    ) -> *mut c_void;
    fn devm_memremap_pages(dev: *mut device, pgmap: *mut dev_pagemap) -> *mut c_void;
    fn init_completion(done: *mut completion);
    fn percpu_ref_init(
        ref_: *mut percpu_ref,
        release: unsafe extern "C" fn(*mut percpu_ref),
        flags: c_int,
        gfp: c_int,
    ) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn percpu_ref_kill(ref_: *mut percpu_ref);
    fn wait_for_completion(done: *mut completion);
    fn percpu_ref_exit(ref_: *mut percpu_ref);
    fn memremap(offset: resource_size_t, size: size_t, flags: c_ulong) -> *mut c_void;
    fn devm_memunmap(dev: *mut device, addr: *mut c_void);
    fn ioremap(offset: resource_size_t, size: c_ulong) -> *mut c_void;
    fn ioremap_wc(offset: resource_size_t, size: c_ulong) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn memunmap(addr: *mut c_void);
    fn resource_size(res: *const resource) -> resource_size_t;
    fn devres_release(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_: unsafe extern "C" fn(*mut device, *mut c_void, *mut c_void) -> c_int,
        match_data: *mut c_void,
    ) -> c_int;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn kfree(ptr: *mut c_void);
    fn resource_type(res: *mut resource) -> c_ulong;
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn devres_alloc(
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        size: size_t,
        gfp: c_int,
    ) -> *mut c_void;
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn __devm_request_region(
        dev: *mut device,
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
        name: *const c_char,
    ) -> *mut resource;
    fn __request_region(
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
        name: *const c_char,
        flags: c_int,
    ) -> *mut resource;
    fn insert_resource(parent: *mut resource, res: *mut resource) -> c_int;
    fn remove_resource(res: *mut resource) -> c_int;
    fn __release_region(parent: *mut resource, start: resource_size_t, n: resource_size_t);
    fn __devm_release_region(
        dev: *mut device,
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
    );
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn acpi_evaluate_object(
        handle: acpi_handle,
        path: acpi_string,
        p: *mut acpi_object_list,
        buf: *mut acpi_buffer,
    ) -> acpi_status;
    fn acpi_evaluate_dsm(
        handle: acpi_handle,
        guid: *const guid_t,
        rev: u64_t,
        func: u64_t,
        argv4: *mut acpi_object,
    ) -> *mut acpi_object;
}

unsafe fn WARN_ON(condition: bool) -> bool {
    condition
}

unsafe fn WARN(condition: bool, _fmt: *const c_char, _args: ...) -> bool {
    condition
}

unsafe fn pr_debug(_fmt: *const c_char, _args: ...) {}

unsafe fn ERR_PTR<T>(error: c_int) -> *mut T {
    error as isize as *mut T
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn list_first_or_null_rcu_iomap_ops() -> *mut iomap_ops {
    if iomap_head.next == ptr::addr_of_mut!(iomap_head) {
        ptr::null_mut()
    } else {
        (iomap_head.next as *mut u8).sub(core::mem::offset_of!(iomap_ops, list)) as *mut iomap_ops
    }
}

unsafe fn container_of_dev_pagemap_ref(ref_: *mut percpu_ref) -> *mut dev_pagemap {
    (ref_ as *mut u8).sub(core::mem::offset_of!(dev_pagemap, ref_)) as *mut dev_pagemap
}

unsafe fn nfit_test_ioremap(
    offset: resource_size_t,
    size: c_ulong,
    fallback_fn: unsafe extern "C" fn(resource_size_t, c_ulong) -> *mut c_void,
) -> *mut c_void {
    let nfit_res = get_nfit_res(offset);

    if !nfit_res.is_null() {
        (*nfit_res)
            .buf
            .add((offset - (*nfit_res).res.start) as usize) as *mut c_void
    } else {
        fallback_fn(offset, size)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nfit_test_setup(
    lookup: nfit_test_lookup_fn,
    evaluate: nfit_test_evaluate_dsm_fn,
) {
    iomap_ops.nfit_test_lookup = lookup;
    iomap_ops.evaluate_dsm = evaluate;
    list_add_rcu(ptr::addr_of_mut!(iomap_ops.list), ptr::addr_of_mut!(iomap_head));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nfit_test_teardown() {
    list_del_rcu(ptr::addr_of_mut!(iomap_ops.list));
    synchronize_rcu();
}

unsafe fn __get_nfit_res(resource: resource_size_t) -> *mut nfit_test_resource {
    let ops = list_first_or_null_rcu_iomap_ops();

    if !ops.is_null() {
        return ((*ops).nfit_test_lookup.unwrap())(resource);
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_nfit_res(resource: resource_size_t) -> *mut nfit_test_resource {
    let res: *mut nfit_test_resource;

    rcu_read_lock();
    res = __get_nfit_res(resource);
    rcu_read_unlock();

    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_devm_ioremap(
    dev: *mut device,
    offset: resource_size_t,
    size: c_ulong,
) -> *mut c_void {
    let nfit_res = get_nfit_res(offset);

    if !nfit_res.is_null() {
        return (*nfit_res)
            .buf
            .add((offset - (*nfit_res).res.start) as usize) as *mut c_void;
    }
    devm_ioremap(dev, offset, size)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_devm_memremap(
    dev: *mut device,
    offset: resource_size_t,
    size: size_t,
    flags: c_ulong,
) -> *mut c_void {
    let nfit_res = get_nfit_res(offset);

    if !nfit_res.is_null() {
        return (*nfit_res)
            .buf
            .add((offset - (*nfit_res).res.start) as usize) as *mut c_void;
    }
    devm_memremap(dev, offset, size, flags)
}

unsafe extern "C" fn nfit_test_kill(_pgmap: *mut c_void) {
    let pgmap = _pgmap as *mut dev_pagemap;

    WARN_ON(pgmap.is_null());

    percpu_ref_kill(ptr::addr_of_mut!((*pgmap).ref_));

    wait_for_completion(ptr::addr_of_mut!((*pgmap).done));
    percpu_ref_exit(ptr::addr_of_mut!((*pgmap).ref_));
}

unsafe extern "C" fn dev_pagemap_percpu_release(ref_: *mut percpu_ref) {
    let pgmap = container_of_dev_pagemap_ref(ref_);

    complete(ptr::addr_of_mut!((*pgmap).done));
}

unsafe extern "C" {
    fn complete(done: *mut completion);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_devm_memremap_pages(
    dev: *mut device,
    pgmap: *mut dev_pagemap,
) -> *mut c_void {
    let mut error: c_int;
    let offset: resource_size_t = (*pgmap).range.start;
    let nfit_res = get_nfit_res(offset);

    if nfit_res.is_null() {
        return devm_memremap_pages(dev, pgmap);
    }

    init_completion(ptr::addr_of_mut!((*pgmap).done));
    error = percpu_ref_init(
        ptr::addr_of_mut!((*pgmap).ref_),
        dev_pagemap_percpu_release,
        0,
        GFP_KERNEL,
    );
    if error != 0 {
        return ERR_PTR(error);
    }

    error = devm_add_action_or_reset(dev, nfit_test_kill, pgmap as *mut c_void);
    if error != 0 {
        return ERR_PTR(error);
    }
    (*nfit_res)
        .buf
        .add((offset - (*nfit_res).res.start) as usize) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_memremap(
    offset: resource_size_t,
    size: size_t,
    flags: c_ulong,
) -> *mut c_void {
    let nfit_res = get_nfit_res(offset);

    if !nfit_res.is_null() {
        return (*nfit_res)
            .buf
            .add((offset - (*nfit_res).res.start) as usize) as *mut c_void;
    }
    memremap(offset, size, flags)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_devm_memunmap(dev: *mut device, addr: *mut c_void) {
    let nfit_res = get_nfit_res(addr as c_long as resource_size_t);

    if !nfit_res.is_null() {
        return;
    }
    devm_memunmap(dev, addr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_ioremap(offset: resource_size_t, size: c_ulong) -> *mut c_void {
    nfit_test_ioremap(offset, size, ioremap)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_ioremap_wc(offset: resource_size_t, size: c_ulong) -> *mut c_void {
    nfit_test_ioremap(offset, size, ioremap_wc)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_iounmap(addr: *mut c_void) {
    let nfit_res = get_nfit_res(addr as c_long as resource_size_t);
    if !nfit_res.is_null() {
        return;
    }
    iounmap(addr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_memunmap(addr: *mut c_void) {
    let nfit_res = get_nfit_res(addr as c_long as resource_size_t);

    if !nfit_res.is_null() {
        return;
    }
    memunmap(addr);
}

unsafe fn nfit_test_release_region(
    dev: *mut device,
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
) -> bool {
    if parent == ptr::addr_of_mut!(iomem_resource) {
        let nfit_res = get_nfit_res(start);

        if !nfit_res.is_null() {
            let mut req: *mut nfit_test_request;
            let mut res: *mut resource = ptr::null_mut();

            if !dev.is_null() {
                devres_release(
                    dev,
                    nfit_devres_release,
                    match_,
                    ptr::addr_of!(start) as *mut c_void,
                );
                return true;
            }

            spin_lock(ptr::addr_of_mut!((*nfit_res).lock));
            req = (*nfit_res).requests.next as *mut nfit_test_request;
            while !req.is_null() && ptr::addr_of_mut!((*req).list) != ptr::addr_of_mut!((*nfit_res).requests) {
                if (*req).res.start == start {
                    res = ptr::addr_of_mut!((*req).res);
                    list_del(ptr::addr_of_mut!((*req).list));
                    break;
                }
                req = (*req).list.next as *mut nfit_test_request;
            }
            spin_unlock(ptr::addr_of_mut!((*nfit_res).lock));

            WARN(
                res.is_null() || resource_size(res) != n,
                b"%s: start: %llx n: %llx mismatch: %pr\n\0".as_ptr() as *const c_char,
                b"nfit_test_release_region\0".as_ptr(),
                start,
                n,
                res,
            );
            if !res.is_null() {
                kfree(req as *mut c_void);
            }
            return true;
        }
    }
    false
}

unsafe extern "C" {
    fn list_del(entry: *mut list_head);
}

unsafe extern "C" fn nfit_devres_release(_dev: *mut device, data: *mut c_void) {
    let res = *(data as *mut *mut resource);

    WARN_ON(!nfit_test_release_region(
        ptr::null_mut(),
        ptr::addr_of_mut!(iomem_resource),
        (*res).start,
        resource_size(res),
    ));
}

unsafe extern "C" fn match_(
    _dev: *mut device,
    __res: *mut c_void,
    match_data: *mut c_void,
) -> c_int {
    let res = *(__res as *mut *mut resource);
    let start = *(match_data as *mut resource_size_t);

    ((*res).start == start) as c_int
}

unsafe fn nfit_test_request_region(
    dev: *mut device,
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
    name: *const c_char,
    flags: c_int,
) -> *mut resource {
    let nfit_res: *mut nfit_test_resource;

    if parent == ptr::addr_of_mut!(iomem_resource) {
        nfit_res = get_nfit_res(start);
        if !nfit_res.is_null() {
            let mut req: *mut nfit_test_request;
            let mut res: *mut resource = ptr::null_mut();

            if start + n > (*nfit_res).res.start + resource_size(ptr::addr_of_mut!((*nfit_res).res)) {
                pr_debug(
                    b"%s: start: %llx n: %llx overflow: %pr\n\0".as_ptr() as *const c_char,
                    b"nfit_test_request_region\0".as_ptr(),
                    start,
                    n,
                    ptr::addr_of_mut!((*nfit_res).res),
                );
                return ptr::null_mut();
            }

            spin_lock(ptr::addr_of_mut!((*nfit_res).lock));
            req = (*nfit_res).requests.next as *mut nfit_test_request;
            while !req.is_null() && ptr::addr_of_mut!((*req).list) != ptr::addr_of_mut!((*nfit_res).requests) {
                if start == (*req).res.start {
                    res = ptr::addr_of_mut!((*req).res);
                    break;
                }
                req = (*req).list.next as *mut nfit_test_request;
            }
            spin_unlock(ptr::addr_of_mut!((*nfit_res).lock));

            if !res.is_null() {
                WARN(
                    true,
                    b"%pr already busy\n\0".as_ptr() as *const c_char,
                    res,
                );
                return ptr::null_mut();
            }

            req = kzalloc(size_of::<nfit_test_request>(), GFP_KERNEL) as *mut nfit_test_request;
            if req.is_null() {
                return ptr::null_mut();
            }
            INIT_LIST_HEAD(ptr::addr_of_mut!((*req).list));
            res = ptr::addr_of_mut!((*req).res);

            (*res).start = start;
            (*res).end = start + n - 1;
            (*res).name = name;
            (*res).flags = resource_type(parent);
            (*res).flags |= IORESOURCE_BUSY | flags as c_ulong;
            spin_lock(ptr::addr_of_mut!((*nfit_res).lock));
            list_add(
                ptr::addr_of_mut!((*req).list),
                ptr::addr_of_mut!((*nfit_res).requests),
            );
            spin_unlock(ptr::addr_of_mut!((*nfit_res).lock));

            if !dev.is_null() {
                let d: *mut *mut resource;

                d = devres_alloc(
                    nfit_devres_release,
                    size_of::<*mut resource>(),
                    GFP_KERNEL,
                ) as *mut *mut resource;
                if d.is_null() {
                    return ptr::null_mut();
                }
                *d = res;
                devres_add(dev, d as *mut c_void);
            }

            pr_debug(
                b"%s: %pr\n\0".as_ptr() as *const c_char,
                b"nfit_test_request_region\0".as_ptr(),
                res,
            );
            return res;
        }
    }
    if !dev.is_null() {
        return __devm_request_region(dev, parent, start, n, name);
    }
    __request_region(parent, start, n, name, flags)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap___request_region(
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
    name: *const c_char,
    flags: c_int,
) -> *mut resource {
    nfit_test_request_region(ptr::null_mut(), parent, start, n, name, flags)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_insert_resource(parent: *mut resource, res: *mut resource) -> c_int {
    if !get_nfit_res((*res).start).is_null() {
        return 0;
    }
    insert_resource(parent, res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_remove_resource(res: *mut resource) -> c_int {
    if !get_nfit_res((*res).start).is_null() {
        return 0;
    }
    remove_resource(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap___devm_request_region(
    dev: *mut device,
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
    name: *const c_char,
) -> *mut resource {
    if dev.is_null() {
        return ptr::null_mut();
    }
    nfit_test_request_region(dev, parent, start, n, name, 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap___release_region(
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
) {
    if !nfit_test_release_region(ptr::null_mut(), parent, start, n) {
        __release_region(parent, start, n);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap___devm_release_region(
    dev: *mut device,
    parent: *mut resource,
    start: resource_size_t,
    n: resource_size_t,
) {
    if !nfit_test_release_region(dev, parent, start, n) {
        __devm_release_region(dev, parent, start, n);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_acpi_evaluate_object(
    handle: acpi_handle,
    path: acpi_string,
    p: *mut acpi_object_list,
    buf: *mut acpi_buffer,
) -> acpi_status {
    let nfit_res = get_nfit_res(handle as c_long as resource_size_t);
    let obj: *mut *mut acpi_object;

    if nfit_res.is_null()
        || strcmp(path, b"_FIT\0".as_ptr() as *const c_char) != 0
        || buf.is_null()
    {
        return acpi_evaluate_object(handle, path, p, buf);
    }

    obj = (*nfit_res).buf as *mut *mut acpi_object;
    (*buf).length = size_of::<acpi_object>();
    (*buf).pointer = *obj as *mut c_void;
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_acpi_evaluate_dsm(
    handle: acpi_handle,
    guid: *const guid_t,
    rev: u64_t,
    func: u64_t,
    argv4: *mut acpi_object,
) -> *mut acpi_object {
    let mut obj: *mut acpi_object = ERR_PTR(-ENXIO);
    let ops: *mut iomap_ops;

    rcu_read_lock();
    ops = list_first_or_null_rcu_iomap_ops();
    if !ops.is_null() {
        obj = ((*ops).evaluate_dsm.unwrap())(handle, guid, rev, func, argv4);
    }
    rcu_read_unlock();

    if IS_ERR(obj) {
        return acpi_evaluate_dsm(handle, guid, rev, func, argv4);
    }
    obj
}

// MODULE_DESCRIPTION("NVDIMM unit test");
// MODULE_LICENSE("GPL v2");
