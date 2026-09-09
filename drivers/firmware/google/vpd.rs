// SPDX-License-Identifier: GPL-2.0-only
/*
 * vpd.c
 *
 * Driver for exporting VPD content to sysfs.
 *
 * Copyright 2017 Google Inc.
 */

// Kernel dependencies are supplied by the surrounding repository.

const CB_TAG_VPD: u32 = 0x2c;
const VPD_CBMEM_MAGIC: u32 = 0x43524f53;

static mut vpd_kobj: *mut kobject = core::ptr::null_mut();

#[repr(C)]
struct vpd_cbmem {
    magic: u32,
    version: u32,
    ro_size: u32,
    rw_size: u32,
    blob: [u8; 0],
}

#[repr(C)]
struct vpd_section {
    enabled: bool,
    name: *const core::ffi::c_char,
    raw_name: *mut core::ffi::c_char,
    kobj: *mut kobject,
    baseaddr: *mut core::ffi::c_char,
    bin_attr: bin_attribute,
    attribs: list_head,
}

#[repr(C)]
struct vpd_attrib_info {
    key: *mut core::ffi::c_char,
    value: *const core::ffi::c_char,
    bin_attr: bin_attribute,
    list: list_head,
}

static mut ro_vpd: vpd_section = unsafe { core::mem::zeroed() };
static mut rw_vpd: vpd_section = unsafe { core::mem::zeroed() };

unsafe extern "C" fn vpd_attrib_read(
    _filp: *mut file,
    _kobp: *mut kobject,
    bin_attr: *const bin_attribute,
    buf: *mut core::ffi::c_char,
    mut pos: loff_t,
    count: usize,
) -> isize {
    let info = (*bin_attr).private as *mut vpd_attrib_info;
    memory_read_from_buffer(buf, count, &mut pos, (*info).value, (*info).bin_attr.size)
}

/*
 * vpd_section_check_key_name()
 *
 * The VPD specification supports only [a-zA-Z0-9_]+ characters in key names but
 * old firmware versions may have entries like "S/N" which are problematic when
 * exporting them as sysfs attributes. These keys present in old firmwares are
 * ignored.
 *
 * Returns VPD_OK for a valid key name, VPD_FAIL otherwise.
 *
 * @key: The key name to check
 * @key_len: key name length
 */
unsafe fn vpd_section_check_key_name(mut key: *const u8, mut key_len: i32) -> i32 {
    while key_len > 0 {
        let c = *key as i32;
        key = key.add(1);
        key_len -= 1;
        if !isalnum(c) && c != b'_' as i32 {
            return VPD_FAIL;
        }
    }
    VPD_OK
}

unsafe extern "C" fn vpd_section_attrib_add(
    key: *const u8,
    key_len: u32,
    value: *const u8,
    value_len: u32,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let sec = arg as *mut vpd_section;
    if vpd_section_check_key_name(key, key_len as i32) != VPD_OK {
        return VPD_OK;
    }

    let info = kzalloc::<vpd_attrib_info>();
    if info.is_null() {
        return -ENOMEM;
    }

    (*info).key = kstrndup(key, key_len, GFP_KERNEL);
    if (*info).key.is_null() {
        kfree(info);
        return -ENOMEM;
    }

    sysfs_bin_attr_init(&mut (*info).bin_attr);
    (*info).bin_attr.attr.name = (*info).key;
    (*info).bin_attr.attr.mode = 0o444;
    (*info).bin_attr.size = value_len as usize;
    (*info).bin_attr.read = Some(vpd_attrib_read);
    (*info).bin_attr.private = info as *mut core::ffi::c_void;
    (*info).value = value as *const core::ffi::c_char;
    INIT_LIST_HEAD(&mut (*info).list);

    let ret = sysfs_create_bin_file((*sec).kobj, &mut (*info).bin_attr);
    if ret != 0 {
        kfree((*info).key);
        kfree(info);
        return ret;
    }
    list_add_tail(&mut (*info).list, &mut (*sec).attribs);
    0
}

unsafe fn vpd_section_attrib_destroy(sec: *mut vpd_section) {
    let mut info: *mut vpd_attrib_info;
    let mut temp: *mut vpd_attrib_info;
    list_for_each_entry_safe!(info, temp, &mut (*sec).attribs, list);
    while !info.is_null() {
        sysfs_remove_bin_file((*sec).kobj, &mut (*info).bin_attr);
        kfree((*info).key);
        kfree(info);
        info = temp;
    }
}

unsafe extern "C" fn vpd_section_read(
    _filp: *mut file, _kobp: *mut kobject, bin_attr: *const bin_attribute,
    buf: *mut core::ffi::c_char, mut pos: loff_t, count: usize,
) -> isize {
    let sec = (*bin_attr).private as *mut vpd_section;
    memory_read_from_buffer(buf, count, &mut pos, (*sec).baseaddr, (*sec).bin_attr.size)
}

unsafe fn vpd_section_create_attribs(sec: *mut vpd_section) -> i32 {
    let mut consumed: i32 = 0;
    loop {
        let ret = vpd_decode_string((*sec).bin_attr.size, (*sec).baseaddr, &mut consumed,
                                    Some(vpd_section_attrib_add), sec as *mut core::ffi::c_void);
        if ret != VPD_OK { break; }
    }
    0
}

unsafe fn vpd_section_init(name: *const core::ffi::c_char, sec: *mut vpd_section,
                           physaddr: phys_addr_t, size: usize) -> i32 {
    (*sec).baseaddr = memremap(physaddr, size, MEMREMAP_WB);
    if (*sec).baseaddr.is_null() { return -ENOMEM; }
    (*sec).name = name;
    (*sec).raw_name = kasprintf(GFP_KERNEL, b"%s_raw\0".as_ptr() as *const i8, name);
    if (*sec).raw_name.is_null() { memunmap((*sec).baseaddr); return -ENOMEM; }
    sysfs_bin_attr_init(&mut (*sec).bin_attr);
    (*sec).bin_attr.attr.name = (*sec).raw_name;
    (*sec).bin_attr.attr.mode = 0o444;
    (*sec).bin_attr.size = size;
    (*sec).bin_attr.read = Some(vpd_section_read);
    (*sec).bin_attr.private = sec as *mut core::ffi::c_void;
    let err = sysfs_create_bin_file(vpd_kobj, &mut (*sec).bin_attr);
    if err != 0 { kfree((*sec).raw_name); memunmap((*sec).baseaddr); return err; }
    (*sec).kobj = kobject_create_and_add(name, vpd_kobj);
    if (*sec).kobj.is_null() {
        sysfs_remove_bin_file(vpd_kobj, &mut (*sec).bin_attr); kfree((*sec).raw_name);
        memunmap((*sec).baseaddr); return -EINVAL;
    }
    INIT_LIST_HEAD(&mut (*sec).attribs);
    vpd_section_create_attribs(sec);
    (*sec).enabled = true;
    0
}

unsafe fn vpd_section_destroy(sec: *mut vpd_section) -> i32 {
    if (*sec).enabled {
        vpd_section_attrib_destroy(sec);
        kobject_put((*sec).kobj);
        sysfs_remove_bin_file(vpd_kobj, &mut (*sec).bin_attr);
        kfree((*sec).raw_name);
        memunmap((*sec).baseaddr);
        (*sec).enabled = false;
    }
    0
}

unsafe fn vpd_sections_init(physaddr: phys_addr_t) -> i32 {
    let temp = memremap(physaddr, core::mem::size_of::<vpd_cbmem>(), MEMREMAP_WB) as *const vpd_cbmem;
    if temp.is_null() { return -ENOMEM; }
    let header = core::ptr::read(temp);
    memunmap(temp as *mut core::ffi::c_void);
    if header.magic != VPD_CBMEM_MAGIC { return -ENODEV; }
    if header.ro_size != 0 {
        let ret = vpd_section_init(b"ro\0".as_ptr() as *const i8, &mut ro_vpd,
            physaddr + core::mem::size_of::<vpd_cbmem>(), header.ro_size as usize);
        if ret != 0 { return ret; }
    }
    if header.rw_size != 0 {
        let ret = vpd_section_init(b"rw\0".as_ptr() as *const i8, &mut rw_vpd,
            physaddr + core::mem::size_of::<vpd_cbmem>() + header.ro_size as usize,
            header.rw_size as usize);
        if ret != 0 { vpd_section_destroy(&mut ro_vpd); return ret; }
    }
    0
}

unsafe extern "C" fn vpd_probe(dev: *mut coreboot_device) -> i32 {
    vpd_kobj = kobject_create_and_add(b"vpd\0".as_ptr() as *const i8, firmware_kobj);
    if vpd_kobj.is_null() { return -ENOMEM; }
    let ret = vpd_sections_init((*dev).cbmem_ref.cbmem_addr);
    if ret != 0 { kobject_put(vpd_kobj); return ret; }
    0
}

unsafe extern "C" fn vpd_remove(_dev: *mut coreboot_device) {
    vpd_section_destroy(&mut ro_vpd);
    vpd_section_destroy(&mut rw_vpd);
    kobject_put(vpd_kobj);
}

static vpd_ids: [coreboot_device_id; 2] = [
    coreboot_device_id { tag: CB_TAG_VPD },
    coreboot_device_id { tag: 0 },
];

static mut vpd_driver: coreboot_driver = coreboot_driver {
    probe: Some(vpd_probe),
    remove: Some(vpd_remove),
    drv: driver { name: b"vpd\0".as_ptr() as *const i8 },
    id_table: vpd_ids.as_ptr(),
};

// MODULE_DEVICE_TABLE(coreboot, vpd_ids);
// module_coreboot_driver(vpd_driver);
// MODULE_AUTHOR("Google, Inc.");
// MODULE_DESCRIPTION("Driver for exporting Vital Product Data content to sysfs");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
