// SPDX-License-Identifier: GPL-2.0
/*
 * Support for extracting embedded firmware for peripherals from EFI code,
 *
 * Copyright (c) 2018 Hans de Goede <hdegoede@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/dmi.h, linux/efi.h, linux/efi_embedded_fw.h, linux/io.h,
// linux/slab.h, linux/types.h, linux/vmalloc.h, and crypto/sha2.h.

/* Exported for use by lib/test_firmware.c only */
#[no_mangle]
pub static mut efi_embedded_fw_list: ListHead = ListHead::new();

#[no_mangle]
pub static mut efi_embedded_fw_checked: bool = false;

static embedded_fw_table: [*const DmiSystemId; 2] = [
    #[cfg(CONFIG_TOUCHSCREEN_DMI)]
    touchscreen_dmi_table,
    core::ptr::null(),
];

/*
 * Note the efi_check_for_embedded_firmwares() code currently makes the
 * following 2 assumptions. This may needs to be revisited if embedded firmware
 * is found where this is not true:
 * 1) The firmware is only found in EFI_BOOT_SERVICES_CODE memory segments
 * 2) The firmware always starts at an offset which is a multiple of 8 bytes
 */
unsafe fn efi_check_md_for_embedded_firmware(
    md: *mut EfiMemoryDesc,
    desc: *const EfiEmbeddedFwDesc,
) -> i32 {
    let mut fw: *mut EfiEmbeddedFw;
    let mut hash = [0u8; 32];
    let mut i: u64;
    let size: u64;
    let map: *mut u8;

    size = (*md).num_pages << EFI_PAGE_SHIFT;
    map = memremap((*md).phys_addr, size, MEMREMAP_WB);
    if map.is_null() {
        pr_err("Error mapping EFI mem at %#llx\n", (*md).phys_addr);
        return -ENOMEM;
    }

    i = 0;
    while i.wrapping_add((*desc).length) <= size {
        if memcmp(
            map.add(i as usize) as *const core::ffi::c_void,
            (*desc).prefix as *const core::ffi::c_void,
            EFI_EMBEDDED_FW_PREFIX_LEN,
        ) != 0 {
            i = i.wrapping_add(8);
            continue;
        }

        sha256(map.add(i as usize), (*desc).length as usize, hash.as_mut_ptr());
        if memcmp(
            hash.as_ptr() as *const core::ffi::c_void,
            (*desc).sha256 as *const core::ffi::c_void,
            32,
        ) == 0 {
            break;
        }
        i = i.wrapping_add(8);
    }
    if i.wrapping_add((*desc).length) > size {
        memunmap(map);
        return -ENOENT;
    }

    pr_info("Found EFI embedded fw '%s'\n", (*desc).name);

    fw = kmalloc_obj::<EfiEmbeddedFw>();
    if fw.is_null() {
        memunmap(map);
        return -ENOMEM;
    }

    (*fw).data = kmemdup(map.add(i as usize), (*desc).length as usize, GFP_KERNEL);
    memunmap(map);
    if (*fw).data.is_null() {
        kfree(fw);
        return -ENOMEM;
    }

    (*fw).name = (*desc).name;
    (*fw).length = (*desc).length;
    list_add(&mut (*fw).list, &mut efi_embedded_fw_list);

    0
}

pub unsafe fn efi_check_for_embedded_firmwares() {
    let mut fw_desc: *const EfiEmbeddedFwDesc;
    let mut dmi_id: *const DmiSystemId;
    let mut md: *mut EfiMemoryDesc;
    let mut i: i32;
    let mut r: i32;

    i = 0;
    while !embedded_fw_table[i as usize].is_null() {
        dmi_id = dmi_first_match(embedded_fw_table[i as usize]);
        if dmi_id.is_null() {
            i += 1;
            continue;
        }

        fw_desc = (*dmi_id).driver_data as *const EfiEmbeddedFwDesc;

        /*
         * In some drivers the struct driver_data contains may contain
         * other driver specific data after the fw_desc struct; and the
         * fw_desc struct itself may be empty, skip these.
         */
        if (*fw_desc).name.is_null() {
            i += 1;
            continue;
        }

        for_each_efi_memory_desc!(md) {
            if (*md).type != EFI_BOOT_SERVICES_CODE {
                continue;
            }

            r = efi_check_md_for_embedded_firmware(md, fw_desc);
            if r == 0 {
                break;
            }
        }
        i += 1;
    }

    efi_embedded_fw_checked = true;
}

pub unsafe fn efi_get_embedded_fw(
    name: *const core::ffi::c_char,
    data: *mut *const u8,
    size: *mut usize,
) -> i32 {
    let mut iter: *mut EfiEmbeddedFw;
    let mut fw: *mut EfiEmbeddedFw = core::ptr::null_mut();

    if !efi_embedded_fw_checked {
        pr_warn!("Warning %s called while we did not check for embedded fw\n", __func__);
        return -ENOENT;
    }

    list_for_each_entry!(iter, &mut efi_embedded_fw_list, list) {
        if strcmp(name, (*iter).name) == 0 {
            fw = iter;
            break;
        }
    }

    if fw.is_null() {
        return -ENOENT;
    }

    *data = (*fw).data;
    *size = (*fw).length as usize;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
