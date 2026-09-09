// SPDX-License-Identifier: GPL-2.0
/*
 * mokvar-table.c
 *
 * Copyright (c) 2020 Red Hat
 * Author: Lenny Szubowicz <lszubowi@redhat.com>
 *
 * This module contains the kernel support for the Linux EFI Machine
 * Owner Key (MOK) variable configuration table, which is identified by the
 * LINUX_EFI_MOK_VARIABLE_TABLE_GUID.
 *
 * This EFI configuration table provides a more robust alternative to
 * EFI volatile variables by which an EFI boot loader can pass the
 * contents of the Machine Owner Key (MOK) certificate stores to the
 * kernel during boot. If both the EFI MOK config table and corresponding
 * EFI MOK variables are present, the table should be considered as
 * more authoritative.
 *
 * This module includes code that validates and maps the EFI MOK table,
 * if it's presence was detected very early in boot.
 *
 * Kernel interface routines are provided to walk through all the
 * entries in the MOK config table or to search for a specific named
 * entry.
 *
 * The contents of the individual named MOK config table entries are
 * made available to user space via read-only sysfs binary files under:
 *
 * /sys/firmware/efi/mok-variables/
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

static mut efi_mokvar_table_size: usize = 0;
static mut efi_mokvar_table_va: *mut efi_mokvar_table_entry = core::ptr::null_mut();

struct efi_mokvar_sysfs_attr {
    bin_attr: bin_attribute,
    node: list_head,
}

static mut efi_mokvar_sysfs_list: list_head = LIST_HEAD_INIT;
static mut mokvar_kobj: *mut kobject = core::ptr::null_mut();

/*
 * efi_mokvar_table_init() - Early boot validation of EFI MOK config table
 */
unsafe fn efi_mokvar_table_init() {
    let mut mokvar_entry: *mut efi_mokvar_table_entry;
    let mut next_entry: *mut efi_mokvar_table_entry;
    let mut md: efi_memory_desc_t = core::mem::zeroed();
    let mut va: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut cur_offset: usize = 0;
    let offset_limit: usize;
    let mut map_size_needed: usize = 0;
    let mut size: usize;
    let mut err: i32;

    if !efi_enabled(EFI_MEMMAP) {
        return;
    }
    if efi.mokvar_table == EFI_INVALID_TABLE_ADDR {
        return;
    }
    err = efi_mem_desc_lookup(efi.mokvar_table, &mut md);
    if err != 0 {
        pr_warn!("EFI MOKvar config table is not within the EFI memory map\n");
        return;
    }
    offset_limit = efi_mem_desc_end(&md) - efi.mokvar_table;

    err = -EINVAL;
    while cur_offset + core::mem::size_of::<efi_mokvar_table_entry>() <= offset_limit {
        if !va.is_null() {
            early_memunmap(va, core::mem::size_of::<efi_mokvar_table_entry>());
        }
        va = early_memremap(
            efi.mokvar_table + cur_offset,
            core::mem::size_of::<efi_mokvar_table_entry>(),
        );
        if va.is_null() {
            pr_err!("Failed to map EFI MOKvar config table pa=0x{:x}, size={}.\n",
                    efi.mokvar_table + cur_offset,
                    core::mem::size_of::<efi_mokvar_table_entry>());
            return;
        }
        mokvar_entry = va as *mut efi_mokvar_table_entry;

        loop {
            if (*mokvar_entry).name[0] == 0 {
                if (*mokvar_entry).data_size != 0 {
                    break;
                }
                err = 0;
                map_size_needed = cur_offset + core::mem::size_of::<efi_mokvar_table_entry>();
                break;
            }
            (*mokvar_entry).name[core::mem::size_of_val(&(*mokvar_entry).name) - 1] = 0;
            size = core::mem::size_of::<efi_mokvar_table_entry>() + (*mokvar_entry).data_size;
            cur_offset += size;
            next_entry = (mokvar_entry as *mut u8).add(size) as *mut efi_mokvar_table_entry;
            if ((((mokvar_entry.add(1) as usize - 1) ^
                  (next_entry.add(1) as usize - 1)) & PAGE_MASK) == 0) {
                mokvar_entry = next_entry;
                continue;
            }
            break;
        }
    }

    if !va.is_null() {
        early_memunmap(va, core::mem::size_of::<efi_mokvar_table_entry>());
    }
    if err != 0 {
        pr_err!("EFI MOKvar config table is not valid\n");
        return;
    }
    if md.type_ == EFI_BOOT_SERVICES_DATA {
        efi_mem_reserve(efi.mokvar_table, map_size_needed);
    }
    efi_mokvar_table_size = map_size_needed;
}

/* efi_mokvar_entry_next() - Get next entry in the EFI MOK config table */
unsafe fn efi_mokvar_entry_next(
    mokvar_entry: *mut *mut efi_mokvar_table_entry,
) -> *mut efi_mokvar_table_entry {
    let mokvar_cur = *mokvar_entry;
    *mokvar_entry = core::ptr::null_mut();
    if efi_mokvar_table_va.is_null() {
        return core::ptr::null_mut();
    }
    let mokvar_next = if mokvar_cur.is_null() {
        efi_mokvar_table_va
    } else {
        if (*mokvar_cur).name[0] == 0 {
            return core::ptr::null_mut();
        }
        let size_cur = core::mem::size_of::<efi_mokvar_table_entry>() + (*mokvar_cur).data_size;
        (mokvar_cur as *mut u8).add(size_cur) as *mut efi_mokvar_table_entry
    };
    if (*mokvar_next).name[0] == 0 {
        return core::ptr::null_mut();
    }
    *mokvar_entry = mokvar_next;
    mokvar_next
}

/* efi_mokvar_entry_find() - Find EFI MOK config entry by name */
unsafe fn efi_mokvar_entry_find(name: *const core::ffi::c_char) -> *mut efi_mokvar_table_entry {
    let mut mokvar_entry = core::ptr::null_mut();
    while !efi_mokvar_entry_next(&mut mokvar_entry).is_null() {
        if strncmp(name, (*mokvar_entry).name.as_ptr(), core::mem::size_of_val(&(*mokvar_entry).name)) == 0 {
            return mokvar_entry;
        }
    }
    core::ptr::null_mut()
}

/* efi_mokvar_sysfs_read() - sysfs binary file read routine */
unsafe fn efi_mokvar_sysfs_read(
    _file: *mut file, _kobj: *mut kobject, bin_attr: *const bin_attribute,
    buf: *mut core::ffi::c_char, off: loff_t, mut count: usize,
) -> isize {
    let mokvar_entry = (*bin_attr).private as *mut efi_mokvar_table_entry;
    if !capable(CAP_SYS_ADMIN) || off >= (*mokvar_entry).data_size as loff_t {
        return 0;
    }
    if count > ((*mokvar_entry).data_size as loff_t - off) as usize {
        count = ((*mokvar_entry).data_size as loff_t - off) as usize;
    }
    memcpy(buf, (*mokvar_entry).data.add(off as usize), count);
    count as isize
}

/* efi_mokvar_sysfs_init() - Map EFI MOK config table and create sysfs */
unsafe fn efi_mokvar_sysfs_init() -> i32 {
    if efi_mokvar_table_size == 0 {
        return -ENOENT;
    }
    let config_va = memremap(efi.mokvar_table, efi_mokvar_table_size, MEMREMAP_WB);
    if config_va.is_null() {
        pr_err!("Failed to map EFI MOKvar config table\n");
        return -ENOMEM;
    }
    efi_mokvar_table_va = config_va as *mut efi_mokvar_table_entry;
    mokvar_kobj = kobject_create_and_add(b"mok-variables\0".as_ptr() as *const _, efi_kobj);
    if mokvar_kobj.is_null() {
        pr_err!("Failed to create EFI mok-variables sysfs entry\n");
        return -ENOMEM;
    }
    let mut mokvar_entry = core::ptr::null_mut();
    let mut mokvar_sysfs: *mut efi_mokvar_sysfs_attr = core::ptr::null_mut();
    let mut err = 0;
    while !efi_mokvar_entry_next(&mut mokvar_entry).is_null() {
        mokvar_sysfs = kzalloc_obj::<efi_mokvar_sysfs_attr>();
        if mokvar_sysfs.is_null() { err = -ENOMEM; break; }
        sysfs_bin_attr_init(&mut (*mokvar_sysfs).bin_attr);
        (*mokvar_sysfs).bin_attr.private = mokvar_entry as *mut _;
        (*mokvar_sysfs).bin_attr.attr.name = (*mokvar_entry).name.as_ptr();
        (*mokvar_sysfs).bin_attr.attr.mode = 0o400;
        (*mokvar_sysfs).bin_attr.size = (*mokvar_entry).data_size;
        (*mokvar_sysfs).bin_attr.read = Some(efi_mokvar_sysfs_read);
        err = sysfs_create_bin_file(mokvar_kobj, &(*mokvar_sysfs).bin_attr);
        if err != 0 { break; }
        list_add_tail(&mut (*mokvar_sysfs).node, &mut efi_mokvar_sysfs_list);
    }
    if err != 0 {
        pr_err!("Failed to create some EFI mok-variables sysfs entries\n");
        kfree(mokvar_sysfs as *mut _);
    }
    err
}

// fs_initcall(efi_mokvar_sysfs_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
