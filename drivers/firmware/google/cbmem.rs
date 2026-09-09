// SPDX-License-Identifier: GPL-2.0-only
/*
 * cbmem.c
 *
 * Driver for exporting cbmem entries in sysfs.
 *
 * Copyright 2022 Google LLC
 */

// Linux kernel dependencies supplied by the surrounding Rust translation.

#[repr(C)]
struct CbmemEntry {
    mem_file_buf: *mut core::ffi::c_char,
    size: u32,
}

unsafe fn to_cbmem_entry(kobj: *mut Kobject) -> *mut CbmemEntry {
    dev_get_drvdata(kobj_to_dev(kobj)) as *mut CbmemEntry
}

unsafe fn mem_read(
    _filp: *mut File,
    kobj: *mut Kobject,
    _bin_attr: *const BinAttribute,
    buf: *mut core::ffi::c_char,
    pos: *mut LoffT,
    count: usize,
) -> Isize {
    let entry = to_cbmem_entry(kobj);

    memory_read_from_buffer(
        buf,
        count,
        pos,
        (*entry).mem_file_buf,
        (*entry).size as usize,
    )
}

unsafe fn mem_write(
    _filp: *mut File,
    kobj: *mut Kobject,
    _bin_attr: *const BinAttribute,
    buf: *mut core::ffi::c_char,
    pos: LoFFT,
    mut count: usize,
) -> Isize {
    let entry = to_cbmem_entry(kobj);

    if pos < 0 || pos >= (*entry).size as LoFFT {
        return -EINVAL as Isize;
    }
    if count > ((*entry).size as LoFFT - pos) as usize {
        count = ((*entry).size as LoFFT - pos) as usize;
    }

    core::ptr::copy_nonoverlapping(
        buf,
        (*entry).mem_file_buf.offset(pos),
        count,
    );
    count as Isize
}

// Equivalent of BIN_ATTR_ADMIN_RW(mem, 0).
static mut BIN_ATTR_MEM: BinAttribute = BinAttribute {
    attr: Attribute { },
    size: 0,
    read: Some(mem_read),
    write: Some(mem_write),
    mmap: None,
};

unsafe fn address_show(
    dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *mut core::ffi::c_char,
) -> Isize {
    let cbdev = dev_to_coreboot_device(dev);

    sysfs_emit(buf, "0x%llx\n", (*cbdev).cbmem_entry.address)
}

// Equivalent of DEVICE_ATTR_RO(address).
static mut DEV_ATTR_ADDRESS: DeviceAttribute = DeviceAttribute { };

unsafe fn size_show(
    dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *mut core::ffi::c_char,
) -> Isize {
    let cbdev = dev_to_coreboot_device(dev);

    sysfs_emit(buf, "0x%x\n", (*cbdev).cbmem_entry.entry_size)
}

// Equivalent of DEVICE_ATTR_RO(size).
static mut DEV_ATTR_SIZE: DeviceAttribute = DeviceAttribute { };

static mut ATTRS: [*mut Attribute; 3] = unsafe {
    [
        &mut DEV_ATTR_ADDRESS.attr,
        &mut DEV_ATTR_SIZE.attr,
        core::ptr::null_mut(),
    ]
};

static mut BIN_ATTRS: [*const BinAttribute; 2] = unsafe {
    [&BIN_ATTR_MEM, core::ptr::null()]
};

static mut CBMEM_ENTRY_GROUP: AttributeGroup = AttributeGroup {
    attrs: unsafe { &mut ATTRS as *mut _ },
    bin_attrs: unsafe { &BIN_ATTRS as *const _ },
};

static mut DEV_GROUPS: [*const AttributeGroup; 2] = unsafe {
    [&CBMEM_ENTRY_GROUP, core::ptr::null()]
};

unsafe fn cbmem_entry_probe(dev: *mut CorebootDevice) -> i32 {
    let entry: *mut CbmemEntry;

    entry = devm_kzalloc(
        &mut (*dev).dev,
        core::mem::size_of::<CbmemEntry>(),
        GFP_KERNEL,
    ) as *mut CbmemEntry;
    if entry.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*dev).dev, entry as *mut core::ffi::c_void);
    (*entry).mem_file_buf = devm_memremap(
        &mut (*dev).dev,
        (*dev).cbmem_entry.address,
        (*dev).cbmem_entry.entry_size,
        MEMREMAP_WB,
    );
    if is_err((*entry).mem_file_buf as *const core::ffi::c_void) {
        return ptr_err((*entry).mem_file_buf as *const core::ffi::c_void) as i32;
    }

    (*entry).size = (*dev).cbmem_entry.entry_size;

    0
}

static mut CBMEM_IDS: [CorebootDeviceId; 2] = [
    CorebootDeviceId { tag: LB_TAG_CBMEM_ENTRY },
    CorebootDeviceId { tag: 0 }, // sentinel
];

// Equivalent of MODULE_DEVICE_TABLE(coreboot, cbmem_ids).

static mut CBMEM_ENTRY_DRIVER: CorebootDriver = CorebootDriver {
    probe: Some(cbmem_entry_probe),
    drv: Driver {
        name: "cbmem\0".as_ptr() as *const core::ffi::c_char,
        dev_groups: unsafe { &DEV_GROUPS as *const _ },
    },
    id_table: unsafe { &CBMEM_IDS as *const _ },
};

// Equivalent of module_coreboot_driver(cbmem_entry_driver).
// MODULE_AUTHOR("Jack Rosenthal <jrosenth@chromium.org>");
// MODULE_DESCRIPTION("Driver for exporting CBMEM entries in sysfs");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
