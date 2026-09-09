// SPDX-License-Identifier: GPL-2.0
/*
 * Export Runtime Configuration Interface Table Version 2 (RCI2)
 * to sysfs
 *
 * Copyright (C) 2019 Dell Inc
 * by Narendra K <Narendra.K@dell.com>
 *
 * System firmware advertises the address of the RCI2 Table via
 * an EFI Configuration Table entry. This code retrieves the RCI2
 * table from the address and exports it to sysfs as a binary
 * attribute 'rci2' under /sys/firmware/efi/tables directory.
 */

// Linux kernel dependencies supplied by other translation units.

const RCI_SIGNATURE: &[u8; 4] = b"_RC_";

#[repr(C, packed)]
struct rci2_table_global_hdr {
    type_: u16,
    resvd0: u16,
    hdr_len: u16,
    rci2_sig: [u8; 4],
    resvd1: u16,
    resvd2: u32,
    resvd3: u32,
    major_rev: u8,
    minor_rev: u8,
    num_of_structs: u16,
    rci2_len: u32,
    rci2_chksum: u16,
}

static mut rci2_base: *mut u8 = core::ptr::null_mut();
static mut rci2_table_len: u32 = 0;
// __ro_after_init
static mut rci2_table_phys: usize = EFI_INVALID_TABLE_ADDR;

// __ro_after_init BIN_ATTR_SIMPLE_ADMIN_RO(rci2);
extern "C" {
    static mut bin_attr_rci2: BinAttribute;
    static efi_kobj: *mut KObject;
    static EFI_INVALID_TABLE_ADDR: usize;

    fn memremap(addr: usize, size: usize, flags: u64) -> *mut u8;
    fn memunmap(addr: *mut u8);
    fn kobject_create_and_add(name: *const u8, parent: *mut KObject) -> *mut KObject;
    fn kobject_del(kobj: *mut KObject);
    fn kobject_put(kobj: *mut KObject);
    fn sysfs_create_bin_file(kobj: *mut KObject, attr: *mut BinAttribute) -> i32;
    fn pr_debug(fmt: *const u8, ...);
}

#[repr(C)]
struct KObject {
    _private: [u8; 0],
}

#[repr(C)]
struct BinAttribute {
    size: usize,
    private: *mut u8,
}

const MEMREMAP_WB: u64 = 1;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;

unsafe fn checksum() -> u16 {
    let len_is_odd: u8 = (rci2_table_len % 2) as u8;
    let mut chksum_len: u32 = rci2_table_len;
    let mut base = rci2_base as *mut u16;
    let mut buf = [0u8; 2];
    let mut offset: u32 = 0;
    let mut chksum: u16 = 0;

    if len_is_odd != 0 {
        chksum_len -= 1;
    }

    while offset < chksum_len {
        chksum = chksum.wrapping_add(core::ptr::read_unaligned(base));
        offset += 2;
        base = base.add(1);
    }

    if len_is_odd != 0 {
        buf[0] = core::ptr::read(base as *mut u8);
        chksum = chksum.wrapping_add(core::ptr::read_unaligned(buf.as_ptr() as *const u16));
    }

    chksum
}

// __init
unsafe fn efi_rci2_sysfs_init() -> i32 {
    let mut tables_kobj: *mut KObject;
    let mut ret: i32 = -ENOMEM;

    if rci2_table_phys == EFI_INVALID_TABLE_ADDR {
        return 0;
    }

    rci2_base = memremap(
        rci2_table_phys,
        core::mem::size_of::<rci2_table_global_hdr>(),
        MEMREMAP_WB,
    );
    if rci2_base.is_null() {
        pr_debug(b"RCI2 table init failed - could not map RCI2 table\0".as_ptr());
        goto_err();
        return ret;
    }

    if core::slice::from_raw_parts(rci2_base.add(offsetof_rci2_sig()), 4) != RCI_SIGNATURE {
        pr_debug(b"RCI2 table init failed - incorrect signature\0".as_ptr());
        ret = -ENODEV;
        memunmap(rci2_base);
        goto_err_unmap();
        return ret;
    }

    rci2_table_len = core::ptr::read_unaligned(
        rci2_base.add(offsetof_rci2_len()) as *const u32,
    );

    memunmap(rci2_base);

    if rci2_table_len == 0 {
        pr_debug(b"RCI2 table init failed - incorrect table length\0".as_ptr());
        goto_err();
        return ret;
    }

    rci2_base = memremap(rci2_table_phys, rci2_table_len as usize, MEMREMAP_WB);
    if rci2_base.is_null() {
        pr_debug(b"RCI2 table - could not map RCI2 table\0".as_ptr());
        goto_err();
        return ret;
    }

    if checksum() != 0 {
        pr_debug(b"RCI2 table - incorrect checksum\0".as_ptr());
        ret = -ENODEV;
        memunmap(rci2_base);
        goto_err_unmap();
        return ret;
    }

    tables_kobj = kobject_create_and_add(b"tables\0".as_ptr(), efi_kobj);
    if tables_kobj.is_null() {
        pr_debug(b"RCI2 table - tables_kobj creation failed\0".as_ptr());
        memunmap(rci2_base);
        goto_err_unmap();
        return ret;
    }

    bin_attr_rci2.size = rci2_table_len as usize;
    bin_attr_rci2.private = rci2_base;
    ret = sysfs_create_bin_file(tables_kobj, &mut bin_attr_rci2);
    if ret != 0 {
        pr_debug(b"RCI2 table - rci2 sysfs bin file creation failed\0".as_ptr());
        kobject_del(tables_kobj);
        kobject_put(tables_kobj);
        memunmap(rci2_base);
        goto_err_unmap();
        return ret;
    }

    0
}

const fn offsetof_rci2_sig() -> usize {
    6
}

const fn offsetof_rci2_len() -> usize {
    24
}

unsafe fn goto_err_unmap() {
    pr_debug(b"RCI2 table - sysfs initialization failed\0".as_ptr());
}

unsafe fn goto_err() {
    pr_debug(b"RCI2 table - sysfs initialization failed\0".as_ptr());
}

// late_initcall(efi_rci2_sysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
