// SPDX-License-Identifier: GPL-2.0
/*
 * CMA SysFS Interface
 *
 * Copyright (c) 2021 Minchan Kim <minchan@kernel.org>
 */

// Dependencies supplied by the kernel and by cma.h remain external to this translation.

static mut alloc_pages_success_attr: kobj_attribute = __ATTR_RO!(alloc_pages_success);
static mut alloc_pages_fail_attr: kobj_attribute = __ATTR_RO!(alloc_pages_fail);
static mut release_pages_success_attr: kobj_attribute = __ATTR_RO!(release_pages_success);
static mut total_pages_attr: kobj_attribute = __ATTR_RO!(total_pages);
static mut available_pages_attr: kobj_attribute = __ATTR_RO!(available_pages);

pub unsafe fn cma_sysfs_account_success_pages(cma: *mut cma, nr_pages: c_ulong) {
    atomic64_add(nr_pages, &mut (*cma).nr_pages_succeeded);
}

pub unsafe fn cma_sysfs_account_fail_pages(cma: *mut cma, nr_pages: c_ulong) {
    atomic64_add(nr_pages, &mut (*cma).nr_pages_failed);
}

pub unsafe fn cma_sysfs_account_release_pages(cma: *mut cma, nr_pages: c_ulong) {
    atomic64_add(nr_pages, &mut (*cma).nr_pages_released);
}

#[inline]
unsafe fn cma_from_kobj(kobj: *mut kobject) -> *mut cma {
    (*container_of!(kobj, cma_kobject, kobj)).cma
}

unsafe fn alloc_pages_success_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cma = cma_from_kobj(kobj);

    sysfs_emit!(buf, "%llu\n", atomic64_read(&(*cma).nr_pages_succeeded))
}

unsafe fn alloc_pages_fail_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cma = cma_from_kobj(kobj);

    sysfs_emit!(buf, "%llu\n", atomic64_read(&(*cma).nr_pages_failed))
}

unsafe fn release_pages_success_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cma = cma_from_kobj(kobj);

    sysfs_emit!(buf, "%llu\n", atomic64_read(&(*cma).nr_pages_released))
}

unsafe fn total_pages_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cma = cma_from_kobj(kobj);

    sysfs_emit!(buf, "%lu\n", (*cma).count)
}

unsafe fn available_pages_show(
    kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cma = cma_from_kobj(kobj);

    sysfs_emit!(buf, "%lu\n", (*cma).available_count)
}

unsafe fn cma_kobj_release(kobj: *mut kobject) {
    let cma = cma_from_kobj(kobj);
    let cma_kobj = (*cma).cma_kobj;

    kfree(cma_kobj);
    (*cma).cma_kobj = core::ptr::null_mut();
}

static mut cma_attrs: [*mut attribute; 6] = [
    unsafe { &mut alloc_pages_success_attr.attr },
    unsafe { &mut alloc_pages_fail_attr.attr },
    unsafe { &mut release_pages_success_attr.attr },
    unsafe { &mut total_pages_attr.attr },
    unsafe { &mut available_pages_attr.attr },
    core::ptr::null_mut(),
];

static mut cma_groups: *mut attribute_group = ATTRIBUTE_GROUPS!(cma);

static cma_ktype: kobj_type = kobj_type {
    release: Some(cma_kobj_release),
    sysfs_ops: &kobj_sysfs_ops,
    default_groups: cma_groups,
};

unsafe fn cma_sysfs_init() -> c_int {
    let cma_kobj_root: *mut kobject;
    let mut cma_kobj: *mut cma_kobject;
    let mut cma: *mut cma;
    let mut i: c_int = 0;
    let mut err: c_int;

    cma_kobj_root = kobject_create_and_add(c"cma".as_ptr(), mm_kobj);
    if cma_kobj_root.is_null() {
        return -ENOMEM;
    }

    while i < cma_area_count {
        cma = &mut cma_areas[i as usize];
        if !test_bit(CMA_ACTIVATED, &(*cma).flags) {
            i += 1;
            continue;
        }

        cma_kobj = kzalloc_obj!(*cma_kobject);
        if cma_kobj.is_null() {
            err = -ENOMEM;
            break;
        }

        (*cma).cma_kobj = cma_kobj;
        (*cma_kobj).cma = cma;
        err = kobject_init_and_add(
            &mut (*cma_kobj).kobj,
            &cma_ktype,
            cma_kobj_root,
            c"%s".as_ptr(),
            (*cma).name,
        );
        if err != 0 {
            kobject_put(&mut (*cma_kobj).kobj);
            break;
        }
        i += 1;
    }

    if i == cma_area_count {
        return 0;
    }

    while i > 0 {
        i -= 1;
        cma = &mut cma_areas[i as usize];
        if !(*cma).cma_kobj.is_null() {
            kobject_put(&mut (*(*cma).cma_kobj).kobj);
        }
    }
    kobject_put(cma_kobj_root);

    err
}

subsys_initcall!(cma_sysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
