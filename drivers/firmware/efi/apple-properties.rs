// SPDX-License-Identifier: GPL-2.0
/*
 * apple-properties.c - EFI device properties on Macs
 * Copyright (C) 2016 Lukas Wunner <lukas@wunner.de>
 *
 * Properties are stored either as:
 * u8 arrays which can be retrieved with device_property_read_u8_array() or
 * booleans which can be queried with device_property_present().
 */

// C includes and build-time kernel dependencies are supplied by the surrounding tree.

static mut dump_properties: bool = false;

unsafe extern "C" fn dump_properties_enable(_arg: *mut core::ffi::c_char) -> i32 {
    dump_properties = true;
    1
}

// __setup("dump_apple_properties", dump_properties_enable);

#[repr(C)]
struct dev_header {
    len: u32,
    prop_count: u32,
    path: [efi_dev_path; 0],
}

#[repr(C)]
struct properties_header {
    len: u32,
    version: u32,
    dev_count: u32,
    dev_header: [dev_header; 0],
}

unsafe fn unmarshal_key_value_pairs(
    dev_header: *mut dev_header,
    dev: *mut device,
    mut ptr: *const core::ffi::c_void,
    entry: *mut property_entry,
) {
    let mut i: i32 = 0;
    while i < (*dev_header).prop_count as i32 {
        let remaining = (*dev_header).len as usize
            - (ptr as usize - dev_header as usize);
        let key_len: u32;
        let val_len: u32;
        let entry_len: u32;
        let entry_data: *const u8;
        let key: *mut core::ffi::c_char;

        if core::mem::size_of::<u32>() > remaining { break; }
        key_len = *(ptr as *const u32);
        if key_len as usize + core::mem::size_of::<u32>() > remaining
            || key_len as usize < core::mem::size_of::<u32>() + core::mem::size_of::<efi_char16_t>()
            || *((ptr as *const u8).add(core::mem::size_of::<u32>()) as *const efi_char16_t) == 0 {
            dev_err(dev, "invalid property name len at %#zx\n", ptr as usize - dev_header as usize);
            break;
        }
        val_len = *((ptr as *const u8).add(key_len as usize) as *const u32);
        if key_len as usize + val_len as usize > remaining || val_len < core::mem::size_of::<u32>() as u32 {
            dev_err(dev, "invalid property val len at %#zx\n", ptr as usize - dev_header as usize + key_len as usize);
            break;
        }
        // 4 bytes to accommodate UTF-8 code points + null byte
        key = kzalloc((key_len as usize - core::mem::size_of::<u32>()) * 4 + 1, GFP_KERNEL);
        if key.is_null() {
            dev_err(dev, "cannot allocate property name\n");
            break;
        }
        ucs2_as_utf8(key, (ptr as *const u8).add(core::mem::size_of::<u32>()), key_len as usize - core::mem::size_of::<u32>());
        entry_data = (ptr as *const u8).add(key_len as usize + core::mem::size_of::<u32>());
        entry_len = val_len - core::mem::size_of::<u32>() as u32;
        if entry_len != 0 {
            *entry.add(i as usize) = PROPERTY_ENTRY_U8_ARRAY_LEN(key, entry_data, entry_len);
        } else {
            *entry.add(i as usize) = PROPERTY_ENTRY_BOOL(key);
        }
        if dump_properties {
            dev_info(dev, "property: %s\n", key);
            print_hex_dump(KERN_INFO, pr_fmt!(), DUMP_PREFIX_OFFSET, 16, 1, entry_data, entry_len, true);
        }
        ptr = (ptr as *const u8).add(key_len as usize + val_len as usize) as *const core::ffi::c_void;
        i += 1;
    }
    if i != (*dev_header).prop_count as i32 {
        dev_err(dev, "got %d device properties, expected %u\n", i, (*dev_header).prop_count);
        print_hex_dump(KERN_ERR, pr_fmt!(), DUMP_PREFIX_OFFSET, 16, 1, dev_header as *const _, (*dev_header).len, true);
        return;
    }
    dev_info(dev, "assigning %d device properties\n", i);
}

unsafe extern "C" fn unmarshal_devices(properties: *mut properties_header) -> i32 {
    let mut offset = core::mem::offset_of!(properties_header, dev_header);
    while offset + core::mem::size_of::<dev_header>() < (*properties).len as usize {
        let dev_header = (properties as *mut u8).add(offset) as *mut dev_header;
        let mut entry: *mut property_entry = core::ptr::null_mut();
        let mut ptr: *const efi_dev_path = (*dev_header).path.as_ptr();
        let mut len = (*dev_header).len as usize - core::mem::size_of::<dev_header>();
        let dev = efi_get_device_by_path(&mut ptr, &mut len);
        if IS_ERR(dev) {
            pr_err!("device path parse error %ld at %#zx:\n", PTR_ERR(dev), ptr as usize - dev_header as usize);
            print_hex_dump(KERN_ERR, pr_fmt!(), DUMP_PREFIX_OFFSET, 16, 1, dev_header as *const _, (*dev_header).len, true);
            goto_skip_device!();
        }
        entry = kzalloc_objs((*dev_header).prop_count as usize + 1);
        if entry.is_null() { dev_err(dev, "cannot allocate properties\n"); goto_skip_device!(); }
        unmarshal_key_value_pairs(dev_header, dev, ptr as *const _, entry);
        if (*entry).name.is_null() { goto_skip_device!(); }
        let ret = device_create_managed_software_node(dev, entry, core::ptr::null());
        if ret != 0 { dev_err(dev, "error %d assigning properties\n", ret); }
        let mut i = 0;
        while !(*entry.add(i)).name.is_null() { kfree((*entry.add(i)).name); i += 1; }
        goto_skip_device!();
        offset += (*dev_header).len as usize;
    }
    0
}

unsafe extern "C" fn map_properties() -> i32 {
    if !x86_apple_machine { return 0; }
    let mut pa_data = boot_params.hdr.setup_data;
    while pa_data != 0 {
        let data = memremap(pa_data, core::mem::size_of::<setup_data>(), MEMREMAP_WB);
        if data.is_null() { pr_err!("cannot map setup_data header\n"); return -ENOMEM; }
        if (*data).type_ != SETUP_APPLE_PROPERTIES { pa_data = (*data).next; memunmap(data); continue; }
        let data_len = (*data).len;
        memunmap(data);
        let data = memremap(pa_data, core::mem::size_of::<setup_data>() + data_len as usize, MEMREMAP_WB);
        if data.is_null() { pr_err!("cannot map setup_data payload\n"); return -ENOMEM; }
        let properties = (*data).data.as_mut_ptr() as *mut properties_header;
        let ret = if data_len < core::mem::size_of::<properties_header>() as u32 { pr_err!("truncated properties header\n"); -EINVAL }
            else if (*properties).version != 1 { pr_err!("unsupported version:\n"); print_hex_dump(KERN_ERR, pr_fmt!(), DUMP_PREFIX_OFFSET, 16, 1, properties as *const _, data_len, true); -ENOTSUPP }
            else if (*properties).len != data_len { pr_err!("length mismatch, expected %u\n", data_len); -EINVAL }
            else { unmarshal_devices(properties) };
        (*data).len = 0;
        memunmap(data);
        memblock_phys_free(pa_data + core::mem::size_of::<setup_data>() as u64, data_len as usize);
        return ret;
    }
    0
}

// fs_initcall(map_properties);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
