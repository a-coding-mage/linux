// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/firmware/memmap.c
 *  Copyright (C) 2008 SUSE LINUX Products GmbH
 *  by Bernhard Walle <bernhard.walle@gmx.de>
 */

/* Kernel headers and build-time attributes are supplied by the surrounding
 * kernel translation unit. */

/* Firmware map entry. Firmware memory maps are flat and linked. */
#[repr(C)]
struct firmware_map_entry {
    start: u64,
    end: u64,
    type_: *const core::ffi::c_char,
    list: list_head,
    kobj: kobject,
}

/* Forward declarations. */
unsafe fn memmap_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut core::ffi::c_char) -> ssize_t;
unsafe fn start_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t;
unsafe fn end_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t;
unsafe fn type_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t;
unsafe fn firmware_map_find_entry(start: u64, end: u64, type_: *const core::ffi::c_char) -> *mut firmware_map_entry;

#[repr(C)]
struct memmap_attribute {
    attr: attribute,
    show: Option<unsafe fn(*mut firmware_map_entry, *mut core::ffi::c_char) -> ssize_t>,
}

/* __ATTR_RO(start), __ATTR_RO(end), and __ATTR_RO(type). */
static mut memmap_start_attr: memmap_attribute = memmap_attribute { attr: unsafe { core::mem::zeroed() }, show: Some(start_show) };
static mut memmap_end_attr: memmap_attribute = memmap_attribute { attr: unsafe { core::mem::zeroed() }, show: Some(end_show) };
static mut memmap_type_attr: memmap_attribute = memmap_attribute { attr: unsafe { core::mem::zeroed() }, show: Some(type_show) };

/* ATTRIBUTE_GROUPS(def); */
static mut def_attrs: [*mut attribute; 4] = [
    unsafe { &raw mut memmap_start_attr.attr },
    unsafe { &raw mut memmap_end_attr.attr },
    unsafe { &raw mut memmap_type_attr.attr },
    core::ptr::null_mut(),
];

static mut memmap_attr_ops: sysfs_ops = sysfs_ops { show: Some(memmap_attr_show) };
/* LIST_HEAD(map_entries); DEFINE_SPINLOCK(map_entries_lock); */
static mut map_entries: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut map_entries_lock: spinlock_t = spinlock_t::new();
/* LIST_HEAD(map_entries_bootmem); DEFINE_SPINLOCK(map_entries_bootmem_lock); */
static mut map_entries_bootmem: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut map_entries_bootmem_lock: spinlock_t = spinlock_t::new();

unsafe fn to_memmap_entry(kobj: *mut kobject) -> *mut firmware_map_entry {
    container_of!(kobj, firmware_map_entry, kobj)
}

unsafe fn release_firmware_map_entry(kobj: *mut kobject) {
    let entry = to_memmap_entry(kobj);
    if PageReserved(virt_to_page(entry)) {
        spin_lock(&raw mut map_entries_bootmem_lock);
        list_add(&raw mut (*entry).list, &raw mut map_entries_bootmem);
        spin_unlock(&raw mut map_entries_bootmem_lock);
        return;
    }
    kfree(entry as *mut core::ffi::c_void);
}

/* static const struct kobj_type memmap_ktype = ...; */

unsafe fn firmware_map_add_entry(start: u64, end: u64, type_: *const core::ffi::c_char,
                                 entry: *mut firmware_map_entry) -> i32 {
    BUG_ON!(start > end);
    (*entry).start = start;
    (*entry).end = end.wrapping_sub(1);
    (*entry).type_ = type_;
    INIT_LIST_HEAD(&raw mut (*entry).list);
    kobject_init(&raw mut (*entry).kobj, &raw const memmap_ktype);
    spin_lock(&raw mut map_entries_lock);
    list_add_tail(&raw mut (*entry).list, &raw mut map_entries);
    spin_unlock(&raw mut map_entries_lock);
    0
}

unsafe fn firmware_map_remove_entry(entry: *mut firmware_map_entry) { list_del(&raw mut (*entry).list); }

unsafe fn add_sysfs_fw_map_entry(entry: *mut firmware_map_entry) -> i32 {
    static mut map_entries_nr: i32 = 0;
    static mut mmap_kset: *mut kset = core::ptr::null_mut();
    if (*entry).kobj.state_in_sysfs { return -EEXIST; }
    if mmap_kset.is_null() {
        mmap_kset = kset_create_and_add(b"memmap\0".as_ptr() as _, core::ptr::null(), firmware_kobj);
        if mmap_kset.is_null() { return -ENOMEM; }
    }
    (*entry).kobj.kset = mmap_kset;
    if kobject_add(&raw mut (*entry).kobj, core::ptr::null(), b"%d\0".as_ptr() as _, map_entries_nr) != 0 {
        kobject_put(&raw mut (*entry).kobj);
    }
    map_entries_nr += 1;
    0
}

unsafe fn remove_sysfs_fw_map_entry(entry: *mut firmware_map_entry) { kobject_put(&raw mut (*entry).kobj); }

unsafe fn firmware_map_find_entry_in_list(start: u64, end: u64, type_: *const core::ffi::c_char,
                                          list: *mut list_head) -> *mut firmware_map_entry {
    let mut entry: *mut firmware_map_entry = core::ptr::null_mut();
    list_for_each_entry!(entry, list, list) {
        if (*entry).start == start && (*entry).end == end && strcmp((*entry).type_, type_) == 0 { return entry; }
    }
    core::ptr::null_mut()
}

unsafe fn firmware_map_find_entry_bootmem(start: u64, end: u64, type_: *const core::ffi::c_char) -> *mut firmware_map_entry {
    firmware_map_find_entry_in_list(start, end, type_, &raw mut map_entries_bootmem)
}

pub unsafe fn firmware_map_add_hotplug(start: u64, end: u64, type_: *const core::ffi::c_char) -> i32 {
    let mut entry = firmware_map_find_entry(start, end.wrapping_sub(1), type_);
    if !entry.is_null() { return 0; }
    entry = firmware_map_find_entry_bootmem(start, end.wrapping_sub(1), type_);
    if entry.is_null() {
        entry = kzalloc_obj!(firmware_map_entry, GFP_ATOMIC);
        if entry.is_null() { return -ENOMEM; }
    } else {
        spin_lock(&raw mut map_entries_bootmem_lock);
        list_del(&raw mut (*entry).list);
        spin_unlock(&raw mut map_entries_bootmem_lock);
        core::ptr::write_bytes(entry, 0, 1);
    }
    firmware_map_add_entry(start, end, type_, entry);
    add_sysfs_fw_map_entry(entry);
    0
}

pub unsafe fn firmware_map_add_early(start: u64, end: u64, type_: *const core::ffi::c_char) -> i32 {
    let entry = memblock_alloc(core::mem::size_of::<firmware_map_entry>(), SMP_CACHE_BYTES) as *mut firmware_map_entry;
    if WARN_ON(entry.is_null()) { return -ENOMEM; }
    firmware_map_add_entry(start, end, type_, entry)
}

pub unsafe fn firmware_map_remove(start: u64, end: u64, type_: *const core::ffi::c_char) -> i32 {
    spin_lock(&raw mut map_entries_lock);
    let entry = firmware_map_find_entry(start, end.wrapping_sub(1), type_);
    if entry.is_null() { spin_unlock(&raw mut map_entries_lock); return -EINVAL; }
    firmware_map_remove_entry(entry);
    spin_unlock(&raw mut map_entries_lock);
    remove_sysfs_fw_map_entry(entry);
    0
}

unsafe fn start_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t { snprintf(buf, PAGE_SIZE, b"0x%llx\n\0".as_ptr() as _, (*entry).start) }
unsafe fn end_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t { snprintf(buf, PAGE_SIZE, b"0x%llx\n\0".as_ptr() as _, (*entry).end) }
unsafe fn type_show(entry: *mut firmware_map_entry, buf: *mut core::ffi::c_char) -> ssize_t { snprintf(buf, PAGE_SIZE, b"%s\n\0".as_ptr() as _, (*entry).type_) }
unsafe fn to_memmap_attr(attr: *mut attribute) -> *mut memmap_attribute { container_of!(attr, memmap_attribute, attr) }
unsafe fn memmap_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    let entry = to_memmap_entry(kobj); let memmap_attr = to_memmap_attr(attr); ((*memmap_attr).show.unwrap())(entry, buf)
}

unsafe fn firmware_memmap_init() -> i32 {
    let mut entry: *mut firmware_map_entry = core::ptr::null_mut();
    list_for_each_entry!(entry, &raw mut map_entries, list) { add_sysfs_fw_map_entry(entry); }
    0
}
/* late_initcall(firmware_memmap_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
