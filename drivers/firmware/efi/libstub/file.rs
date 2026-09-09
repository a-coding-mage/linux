// SPDX-License-Identifier: GPL-2.0
/*
 * Helper functions used by the EFI stub on multiple architectures. This should
 * be included by the EFI stub implementation files.
 */

const MAX_FILENAME_SIZE: usize = 256;
const EFI_READ_CHUNK_SIZE: usize = SZ_1M;

#[repr(C)]
struct finfo {
    info: efi_file_info_t,
    filename: [efi_char16_t; MAX_FILENAME_SIZE],
}

unsafe fn efi_open_file(
    volume: *mut efi_file_protocol_t,
    fi: *mut finfo,
    handle: *mut *mut efi_file_protocol_t,
    file_size: *mut c_ulong,
) -> efi_status_t {
    let mut info_guid: efi_guid_t = EFI_FILE_INFO_ID;
    let mut fh: *mut efi_file_protocol_t = core::ptr::null_mut();
    let mut info_sz = core::mem::size_of::<finfo>() as c_ulong;
    let mut c = (*fi).filename.as_mut_ptr();

    /* Replace UNIX dir separators with EFI standard ones */
    while *c != 0 {
        if *c == b'/' as efi_char16_t {
            *c = b'\\' as efi_char16_t;
        }
        c = c.add(1);
    }

    let mut status = efi_call_proto!(volume, open, &mut fh, (*fi).filename.as_ptr(), EFI_FILE_MODE_READ, 0);
    if status != EFI_SUCCESS {
        efi_err!("Failed to open file: %ls\n", (*fi).filename.as_ptr());
        return status;
    }

    status = efi_call_proto!(fh, get_info, &mut info_guid, &mut info_sz, fi);
    if status != EFI_SUCCESS {
        efi_err!("Failed to get file info\n");
        efi_call_proto!(fh, close);
        return status;
    }

    *handle = fh;
    *file_size = (*fi).info.file_size;
    EFI_SUCCESS
}

unsafe fn efi_open_volume(image: *mut efi_loaded_image_t, fh: *mut *mut efi_file_protocol_t) -> efi_status_t {
    let mut fs_proto: efi_guid_t = EFI_FILE_SYSTEM_GUID;
    let mut io: *mut efi_simple_file_system_protocol_t = core::ptr::null_mut();
    let mut status = efi_bs_call!(handle_protocol, efi_table_attr!(image, device_handle), &mut fs_proto, &mut io as *mut _ as *mut core::ffi::c_void);
    if status != EFI_SUCCESS {
        efi_err!("Failed to handle fs_proto\n");
        return status;
    }
    status = efi_call_proto!(io, open_volume, fh);
    if status != EFI_SUCCESS { efi_err!("Failed to open volume\n"); }
    status
}

unsafe fn find_file_option(cmdline: *const efi_char16_t, cmdline_len: c_int, prefix: *const efi_char16_t, prefix_size: c_int, result: *mut efi_char16_t, result_len: c_int) -> c_int {
    let prefix_len = prefix_size / 2;
    let mut i = prefix_len;
    let mut found = false;
    while i < cmdline_len {
        if memcmp(cmdline.add((i - prefix_len) as usize) as *const _, prefix as *const _, prefix_size as usize) == 0 { found = true; break; }
        i += 1;
    }
    if !found { return 0; }
    while i < cmdline_len && (*cmdline.add(i as usize) == b'/' as efi_char16_t || *cmdline.add(i as usize) == b'\\' as efi_char16_t) { i += 1; }
    let mut out = result;
    let mut remaining = result_len;
    while { remaining -= 1; remaining > 0 && i < cmdline_len } {
        let c = *cmdline.add(i as usize); i += 1;
        if c == 0 || c == b'\n' as efi_char16_t || c == b' ' as efi_char16_t { break; }
        *out = c; out = out.add(1);
    }
    *out = 0;
    i
}

unsafe fn efi_open_device_path(volume: *mut *mut efi_file_protocol_t, fi: *mut finfo) -> efi_status_t {
    let mut text_to_dp_guid: efi_guid_t = EFI_DEVICE_PATH_FROM_TEXT_PROTOCOL_GUID;
    static mut TEXT_TO_DP: *mut efi_device_path_from_text_protocol_t = core::ptr::null_mut();
    let mut fs_proto: efi_guid_t = EFI_FILE_SYSTEM_GUID;
    let mut initrd_dp: *mut efi_device_path_protocol_t;
    let mut io: *mut efi_simple_file_system_protocol_t = core::ptr::null_mut();
    let mut handle: efi_handle_t = core::ptr::null_mut();
    if TEXT_TO_DP.is_null() && efi_bs_call!(locate_protocol, &mut text_to_dp_guid, core::ptr::null_mut(), &mut TEXT_TO_DP as *mut _ as *mut core::ffi::c_void) != EFI_SUCCESS { return EFI_UNSUPPORTED; }
    initrd_dp = efi_fn_call!(TEXT_TO_DP, convert_text_to_device_path, (*fi).filename.as_ptr());
    let status = efi_bs_call!(locate_device_path, &mut fs_proto, &mut initrd_dp, &mut handle).then(|| EFI_SUCCESS).unwrap_or_else(|| efi_bs_call!(handle_protocol, handle, &mut fs_proto, &mut io as *mut _ as *mut core::ffi::c_void));
    if status != EFI_SUCCESS { return EFI_NOT_FOUND; }
    if (*initrd_dp).type_ != EFI_DEV_MEDIA || (*initrd_dp).sub_type != EFI_DEV_MEDIA_FILE { efi_warn!("Unexpected device path node type: (%x, %x)\n", (*initrd_dp).type_, (*initrd_dp).sub_type); return EFI_LOAD_ERROR; }
    let fpath = initrd_dp as *mut efi_file_path_dev_path;
    core::ptr::copy_nonoverlapping((*fpath).filename.as_ptr(), (*fi).filename.as_mut_ptr(), core::cmp::min(MAX_FILENAME_SIZE, ((*fpath).header.length as usize) - core::mem::size_of_val(&(*fpath).header)));
    let status = efi_call_proto!(io, open_volume, volume);
    if status != EFI_SUCCESS { efi_err!("Failed to open volume\n"); }
    status
}

static builtin_cmdline: [efi_char16_t; 1] = [0];

pub unsafe fn handle_cmdline_files(image: *mut efi_loaded_image_t, optstr: *const efi_char16_t, optstr_size: c_int, soft_limit: c_ulong, hard_limit: c_ulong, load_addr: *mut c_ulong, load_size: *mut c_ulong) -> efi_status_t {
    let ignore_load_options = IS_ENABLED!(CONFIG_CMDLINE_OVERRIDE) || IS_ENABLED!(CONFIG_CMDLINE_FORCE);
    let mut cmdline = efi_table_attr!(image, load_options);
    let mut cmdline_len = efi_table_attr!(image, load_options_size) / core::mem::size_of::<efi_char16_t>() as u32;
    let mut efi_chunk_size = ULONG_MAX;
    let mut volume: *mut efi_file_protocol_t = core::ptr::null_mut();
    let mut alloc_addr = 0; let mut alloc_size = 0; let mut twopass; let mut offset;
    if load_addr.is_null() || load_size.is_null() { return EFI_INVALID_PARAMETER; }
    efi_apply_loadoptions_quirk!(&mut cmdline as *mut _ as *mut _, &mut cmdline_len);
    if IS_ENABLED!(CONFIG_X86) && !efi_nochunk { efi_chunk_size = EFI_READ_CHUNK_SIZE; }
    if !ignore_load_options && cmdline_len > 0 { twopass = IS_ENABLED!(CONFIG_CMDLINE_BOOL) || IS_ENABLED!(CONFIG_CMDLINE_EXTEND); } else { cmdline = builtin_cmdline.as_ptr(); cmdline_len = 0; twopass = false; }
    loop {
        let mut fi: finfo = core::mem::zeroed(); let mut size = 0; let mut file = core::ptr::null_mut();
        offset = find_file_option(cmdline, cmdline_len as c_int, optstr, optstr_size, fi.filename.as_mut_ptr(), fi.filename.len() as c_int);
        if offset == 0 { break; }
        cmdline = cmdline.add(offset as usize); cmdline_len -= offset as u32;
        let mut status = efi_open_device_path(&mut volume, &mut fi);
        if status == EFI_UNSUPPORTED || status == EFI_NOT_FOUND { status = efi_open_volume(image, &mut volume); }
        if status != EFI_SUCCESS { efi_free!(alloc_size, alloc_addr); return status; }
        status = efi_open_file(volume, &mut fi, &mut file, &mut size);
        if status != EFI_SUCCESS { efi_call_proto!(volume, close); efi_free!(alloc_size, alloc_addr); return status; }
        if round_up!(alloc_size + size, EFI_ALLOC_ALIGN) > round_up!(alloc_size, EFI_ALLOC_ALIGN) {
            let old_addr = alloc_addr; status = EFI_OUT_OF_RESOURCES;
            if soft_limit < hard_limit { status = efi_allocate_pages!(alloc_size + size, &mut alloc_addr, soft_limit); }
            if status == EFI_OUT_OF_RESOURCES { status = efi_allocate_pages!(alloc_size + size, &mut alloc_addr, hard_limit); }
            if status != EFI_SUCCESS { efi_err!("Failed to allocate memory for files\n"); efi_call_proto!(file, close); efi_call_proto!(volume, close); efi_free!(alloc_size, alloc_addr); return status; }
            if old_addr != 0 { core::ptr::copy_nonoverlapping(old_addr as *const u8, alloc_addr as *mut u8, alloc_size as usize); efi_free!(alloc_size, old_addr); }
        }
        let mut addr = (alloc_addr + alloc_size) as *mut u8; alloc_size += size;
        while size != 0 { let mut chunksize = core::cmp::min(size, efi_chunk_size); status = efi_call_proto!(file, read, &mut chunksize, addr as *mut _); if status != EFI_SUCCESS { efi_err!("Failed to read file\n"); efi_call_proto!(file, close); efi_call_proto!(volume, close); efi_free!(alloc_size, alloc_addr); return status; } addr = addr.add(chunksize as usize); size -= chunksize; }
        efi_call_proto!(file, close); efi_call_proto!(volume, close);
        if offset <= 0 { break; }
    }
    if twopass { cmdline = builtin_cmdline.as_ptr(); cmdline_len = 0; twopass = false; continue; }
    *load_addr = alloc_addr; *load_size = alloc_size; if *load_size == 0 { EFI_NOT_READY } else { EFI_SUCCESS }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
