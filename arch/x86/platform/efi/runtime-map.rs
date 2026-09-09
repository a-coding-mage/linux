// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Red Hat, Inc., Dave Young <dyoung@redhat.com>
 */

// Linux kernel dependencies are supplied by other translation units.

#[repr(C)]
struct EfiRuntimeMapEntry {
    md: efi_memory_desc_t,
    kobj: kobject, /* kobject for each entry */
}

static mut map_entries: *mut *mut EfiRuntimeMapEntry = core::ptr::null_mut();

#[repr(C)]
struct MapAttribute {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut EfiRuntimeMapEntry, *mut core::ffi::c_char) -> ssize_t>,
}

unsafe fn to_map_attr(attr: *mut attribute) -> *mut MapAttribute {
    container_of!(attr, MapAttribute, attr)
}

unsafe extern "C" fn type_show(entry: *mut EfiRuntimeMapEntry, buf: *mut core::ffi::c_char) -> ssize_t {
    snprintf!(buf, PAGE_SIZE, "0x%x\n", (*entry).md.type_)
}

unsafe extern "C" fn phys_addr_show(entry: *mut EfiRuntimeMapEntry, buf: *mut core::ffi::c_char) -> ssize_t {
    snprintf!(buf, PAGE_SIZE, "0x%llx\n", (*entry).md.phys_addr)
}

unsafe extern "C" fn virt_addr_show(entry: *mut EfiRuntimeMapEntry, buf: *mut core::ffi::c_char) -> ssize_t {
    snprintf!(buf, PAGE_SIZE, "0x%llx\n", (*entry).md.virt_addr)
}

unsafe extern "C" fn num_pages_show(entry: *mut EfiRuntimeMapEntry, buf: *mut core::ffi::c_char) -> ssize_t {
    snprintf!(buf, PAGE_SIZE, "0x%llx\n", (*entry).md.num_pages)
}

unsafe extern "C" fn attribute_show(entry: *mut EfiRuntimeMapEntry, buf: *mut core::ffi::c_char) -> ssize_t {
    snprintf!(buf, PAGE_SIZE, "0x%llx\n", (*entry).md.attribute)
}

unsafe fn to_map_entry(kobj: *mut kobject) -> *mut EfiRuntimeMapEntry {
    container_of!(kobj, EfiRuntimeMapEntry, kobj)
}

unsafe extern "C" fn map_attr_show(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    let entry = to_map_entry(kobj);
    let map_attr = to_map_attr(attr);
    ((*map_attr).show.unwrap())(entry, buf)
}

static mut map_type_attr: MapAttribute = __ATTR_RO_MODE!(type_, 0o400, type_show);
static mut map_phys_addr_attr: MapAttribute = __ATTR_RO_MODE!(phys_addr, 0o400, phys_addr_show);
static mut map_virt_addr_attr: MapAttribute = __ATTR_RO_MODE!(virt_addr, 0o400, virt_addr_show);
static mut map_num_pages_attr: MapAttribute = __ATTR_RO_MODE!(num_pages, 0o400, num_pages_show);
static mut map_attribute_attr: MapAttribute = __ATTR_RO_MODE!(attribute, 0o400, attribute_show);

/* These are default attributes that are added for every memmap entry. */
static mut def_attrs: [*mut attribute; 6] = [
    unsafe { &mut map_type_attr.attr },
    unsafe { &mut map_phys_addr_attr.attr },
    unsafe { &mut map_virt_addr_attr.attr },
    unsafe { &mut map_num_pages_attr.attr },
    unsafe { &mut map_attribute_attr.attr },
    core::ptr::null_mut(),
];
// ATTRIBUTE_GROUPS(def);

static map_attr_ops: sysfs_ops = sysfs_ops {
    show: Some(map_attr_show),
};

unsafe extern "C" fn map_release(kobj: *mut kobject) {
    let entry = to_map_entry(kobj);
    kfree!(entry);
}

static map_ktype: kobj_type = kobj_type {
    sysfs_ops: unsafe { &map_attr_ops },
    default_groups: unsafe { def_groups },
    release: Some(map_release),
};

static mut map_kset: *mut kset = core::ptr::null_mut();

unsafe fn add_sysfs_runtime_map_entry(
    kobj: *mut kobject,
    nr: i32,
    md: *mut efi_memory_desc_t,
) -> *mut EfiRuntimeMapEntry {
    let mut ret: i32;
    let entry: *mut EfiRuntimeMapEntry;

    if map_kset.is_null() {
        map_kset = kset_create_and_add!("runtime-map", core::ptr::null_mut(), kobj);
        if map_kset.is_null() {
            return ERR_PTR!(-ENOMEM);
        }
    }

    entry = kzalloc_obj!(EfiRuntimeMapEntry);
    if entry.is_null() {
        kset_unregister(map_kset);
        map_kset = core::ptr::null_mut();
        return ERR_PTR!(-ENOMEM);
    }

    memcpy!(&mut (*entry).md, md, core::mem::size_of::<efi_memory_desc_t>());

    kobject_init(&mut (*entry).kobj, &map_ktype);
    (*entry).kobj.kset = map_kset;
    ret = kobject_add!(&mut (*entry).kobj, core::ptr::null_mut(), "%d", nr);
    if ret != 0 {
        kobject_put(&mut (*entry).kobj);
        kset_unregister(map_kset);
        map_kset = core::ptr::null_mut();
        return ERR_PTR!(ret);
    }

    entry
}

#[no_mangle]
pub unsafe extern "C" fn efi_get_runtime_map_size() -> i32 {
    (efi.memmap.nr_map * efi.memmap.desc_size) as i32
}

#[no_mangle]
pub unsafe extern "C" fn efi_get_runtime_map_desc_size() -> i32 {
    efi.memmap.desc_size as i32
}

#[no_mangle]
pub unsafe extern "C" fn efi_runtime_map_copy(buf: *mut core::ffi::c_void, bufsz: usize) -> i32 {
    let mut sz = efi_get_runtime_map_size() as usize;
    if sz > bufsz {
        sz = bufsz;
    }
    memcpy!(buf, efi.memmap.map, sz);
    0
}

unsafe extern "C" fn efi_runtime_map_init() -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut ret: i32 = 0;
    let mut entry: *mut EfiRuntimeMapEntry;
    let mut md: *mut efi_memory_desc_t;

    if !efi_enabled(EFI_MEMMAP) || efi_kobj.is_null() {
        return 0;
    }

    map_entries = kzalloc_objs!(EfiRuntimeMapEntry, efi.memmap.nr_map);
    if map_entries.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    i = 0;
    for_each_efi_memory_desc!(md) {
        entry = add_sysfs_runtime_map_entry(efi_kobj, i, md);
        if IS_ERR!(entry) {
            ret = PTR_ERR!(entry);
            break;
        }
        *map_entries.add(i as usize) = entry;
        i += 1;
    }

    if !IS_ERR!(entry) {
        return 0;
    }

    j = i - 1;
    while j >= 0 {
        entry = *map_entries.add(j as usize);
        kobject_put(&mut (*entry).kobj);
        j -= 1;
    }
    ret
}

// subsys_initcall_sync(efi_runtime_map_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
