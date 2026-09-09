// SPDX-License-Identifier: GPL-2.0-only
/*
 * Architecture specific sysfs attributes in /sys/kernel
 *
 * Copyright (C) 2007, Intel Corp.
 *      Huang Ying <ying.huang@intel.com>
 * Copyright (C) 2013, 2013 Red Hat, Inc.
 *      Dave Young <dyoung@redhat.com>
 */

// C headers and build-time definitions are supplied by the surrounding kernel bindings.

unsafe fn version_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, c"0x%04x\n".as_ptr(), boot_params.hdr.version)
}

static mut boot_params_version_attr: kobj_attribute = __ATTR_RO!(version);

unsafe fn boot_params_data_read(fp: *mut file, kobj: *mut kobject,
                                bin_attr: *const bin_attribute, buf: *mut c_char,
                                off: loff_t, count: usize) -> ssize_t {
    memcpy(buf as *mut c_void, (core::ptr::addr_of!(boot_params) as *const u8).add(off as usize) as *const c_void, count);
    count as ssize_t
}

static boot_params_data_attr: bin_attribute = bin_attribute {
    attr: attribute { name: c"data".as_ptr(), mode: S_IRUGO },
    read: Some(boot_params_data_read),
    size: core::mem::size_of::<BootParams>(),
};

static mut boot_params_version_attrs: [*mut attribute; 2] = [
    core::ptr::addr_of_mut!(boot_params_version_attr.attr), core::ptr::null_mut(),
];

static boot_params_data_attrs: [*const bin_attribute; 2] = [
    core::ptr::addr_of!(boot_params_data_attr), core::ptr::null(),
];

static boot_params_attr_group: attribute_group = attribute_group {
    attrs: core::ptr::addr_of_mut!(boot_params_version_attrs) as *mut *mut attribute,
    bin_attrs: core::ptr::addr_of!(boot_params_data_attrs) as *const *const bin_attribute,
};

unsafe fn kobj_to_setup_data_nr(kobj: *mut kobject, nr: *mut c_int) -> c_int {
    let name = kobject_name(kobj);
    kstrtoint(name, 10, nr)
}

unsafe fn get_setup_data_paddr(nr: c_int, paddr: *mut u64) -> c_int {
    let mut i = 0;
    let mut pa_data = boot_params.hdr.setup_data;
    while pa_data != 0 {
        if nr == i { *paddr = pa_data; return 0; }
        let data = memremap(pa_data, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data;
        if data.is_null() { return -ENOMEM; }
        pa_data = (*data).next;
        memunmap(data as *mut c_void);
        i += 1;
    }
    -EINVAL
}

unsafe fn get_setup_data_size(nr: c_int, size: *mut usize) -> c_int {
    let mut pa_data = boot_params.hdr.setup_data;
    let mut i = 0;
    while pa_data != 0 {
        let data = memremap(pa_data, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data;
        if data.is_null() { return -ENOMEM; }
        let pa_next = (*data).next;
        if nr == i {
            if (*data).type_ == SETUP_INDIRECT {
                let len = core::mem::size_of::<setup_data>() + (*data).len as usize;
                memunmap(data as *mut c_void);
                let data = memremap(pa_data, len, MEMREMAP_WB) as *mut setup_data;
                if data.is_null() { return -ENOMEM; }
                let indirect = (*data).data.as_ptr() as *mut setup_indirect;
                if (*indirect).type_ != SETUP_INDIRECT { *size = (*indirect).len as usize; }
                else { *size = (*data).len as usize; }
                memunmap(data as *mut c_void);
            } else { *size = (*data).len as usize; memunmap(data as *mut c_void); }
            return 0;
        }
        pa_data = pa_next;
        memunmap(data as *mut c_void);
        i += 1;
    }
    -EINVAL
}

unsafe fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut nr = 0; let mut ret = kobj_to_setup_data_nr(kobj, &mut nr); if ret != 0 { return ret as ssize_t; }
    let mut paddr = 0; ret = get_setup_data_paddr(nr, &mut paddr); if ret != 0 { return ret as ssize_t; }
    let mut data = memremap(paddr, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data;
    if data.is_null() { return -ENOMEM as ssize_t; }
    if (*data).type_ == SETUP_INDIRECT {
        let len = core::mem::size_of::<setup_data>() + (*data).len as usize;
        memunmap(data as *mut c_void); data = memremap(paddr, len, MEMREMAP_WB) as *mut setup_data;
        if data.is_null() { return -ENOMEM as ssize_t; }
        ret = sprintf(buf, c"0x%x\n".as_ptr(), (*( (*data).data.as_ptr() as *mut setup_indirect)).type_);
    } else { ret = sprintf(buf, c"0x%x\n".as_ptr(), (*data).type_); }
    memunmap(data as *mut c_void); ret
}

unsafe fn setup_data_data_read(fp: *mut file, kobj: *mut kobject, bin_attr: *const bin_attribute,
                               buf: *mut c_char, off: loff_t, mut count: usize) -> ssize_t {
    let mut nr = 0; let mut ret = kobj_to_setup_data_nr(kobj, &mut nr); if ret != 0 { return ret as ssize_t; }
    let mut paddr = 0; ret = get_setup_data_paddr(nr, &mut paddr); if ret != 0 { return ret as ssize_t; }
    let mut data = memremap(paddr, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data;
    if data.is_null() { return -ENOMEM as ssize_t; }
    let mut len: u64;
    if (*data).type_ == SETUP_INDIRECT {
        len = (core::mem::size_of::<setup_data>() as u64) + (*data).len as u64;
        memunmap(data as *mut c_void); data = memremap(paddr, len as usize, MEMREMAP_WB) as *mut setup_data;
        if data.is_null() { return -ENOMEM as ssize_t; }
        let indirect = (*data).data.as_ptr() as *mut setup_indirect;
        if (*indirect).type_ != SETUP_INDIRECT { paddr = (*indirect).addr; len = (*indirect).len as u64; }
        else { paddr += core::mem::size_of::<setup_data>() as u64; len = (*data).len as u64; }
    } else { paddr += core::mem::size_of::<setup_data>() as u64; len = (*data).len as u64; }
    if off as u64 > len { ret = -EINVAL; }
    else { if count as u64 > len - off as u64 { count = (len - off as u64) as usize; } if count != 0 {
        ret = count as c_int; let p = memremap(paddr, len as usize, MEMREMAP_WB); if p.is_null() { ret = -ENOMEM; }
        else { memcpy(buf as *mut c_void, (p as *mut u8).add(off as usize) as *const c_void, count); memunmap(p); }
    }}
    memunmap(data as *mut c_void); ret as ssize_t
}

static mut type_attr: kobj_attribute = __ATTR_RO!(type);
static mut data_attr: bin_attribute = bin_attribute { attr: attribute { name: c"data".as_ptr(), mode: S_IRUGO }, read: Some(setup_data_data_read), size: 0 };
static mut setup_data_type_attrs: [*mut attribute; 2] = [core::ptr::addr_of_mut!(type_attr.attr), core::ptr::null_mut()];
static setup_data_data_attrs: [*const bin_attribute; 2] = [core::ptr::addr_of!(data_attr), core::ptr::null()];
static setup_data_attr_group: attribute_group = attribute_group { attrs: core::ptr::addr_of_mut!(setup_data_type_attrs) as *mut *mut attribute, bin_attrs: core::ptr::addr_of!(setup_data_data_attrs) as *const *const bin_attribute };

unsafe fn create_setup_data_node(parent: *mut kobject, kobjp: *mut *mut kobject, nr: c_int) -> c_int {
    let mut size = 0; let mut name = [0 as c_char; 16]; snprintf(name.as_mut_ptr(), 16, c"%d".as_ptr(), nr);
    let kobj = kobject_create_and_add(name.as_ptr(), parent); if kobj.is_null() { return -ENOMEM; }
    let mut ret = get_setup_data_size(nr, &mut size); if ret != 0 { kobject_put(kobj); return ret; }
    data_attr.size = size; ret = sysfs_create_group(kobj, &setup_data_attr_group); if ret != 0 { kobject_put(kobj); return ret; }
    *kobjp = kobj; 0
}

unsafe fn cleanup_setup_data_node(kobj: *mut kobject) { sysfs_remove_group(kobj, &setup_data_attr_group); kobject_put(kobj); }

unsafe fn get_setup_data_total_num(mut pa_data: u64, nr: *mut c_int) -> c_int {
    *nr = 0; while pa_data != 0 { *nr += 1; let data = memremap(pa_data, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data; if data.is_null() { return -ENOMEM; } pa_data = (*data).next; memunmap(data as *mut c_void); } 0
}

unsafe fn create_setup_data_nodes(parent: *mut kobject) -> c_int {
    let pa_data = boot_params.hdr.setup_data; if pa_data == 0 { return 0; }
    let setup_data_kobj = kobject_create_and_add(c"setup_data".as_ptr(), parent); if setup_data_kobj.is_null() { return -ENOMEM; }
    let mut nr = 0; let mut ret = get_setup_data_total_num(pa_data, &mut nr); if ret != 0 { kobject_put(setup_data_kobj); return ret; }
    let kobjp = kmalloc_objs!(kobj, nr); if kobjp.is_null() { kobject_put(setup_data_kobj); return -ENOMEM; }
    let mut i = 0; while i < nr { ret = create_setup_data_node(setup_data_kobj, kobjp.add(i as usize), i); if ret != 0 { let mut j = i - 1; while j >= 0 { cleanup_setup_data_node(*kobjp.add(j as usize)); j -= 1; } kfree(kobjp as *mut c_void); kobject_put(setup_data_kobj); return ret; } i += 1; }
    kfree(kobjp as *mut c_void); 0
}

unsafe fn boot_params_ksysfs_init() -> c_int {
    let boot_params_kobj = kobject_create_and_add(c"boot_params".as_ptr(), kernel_kobj); if boot_params_kobj.is_null() { return -ENOMEM; }
    let mut ret = sysfs_create_group(boot_params_kobj, &boot_params_attr_group); if ret != 0 { kobject_put(boot_params_kobj); return ret; }
    ret = create_setup_data_nodes(boot_params_kobj); if ret != 0 { sysfs_remove_group(boot_params_kobj, &boot_params_attr_group); kobject_put(boot_params_kobj); return ret; } 0
}

arch_initcall!(boot_params_ksysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
