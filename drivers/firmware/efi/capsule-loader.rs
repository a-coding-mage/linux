// SPDX-License-Identifier: GPL-2.0
/*
 * EFI capsule loader driver.
 *
 * Copyright 2015 Intel Corporation
 */

// C dependencies are supplied by the surrounding kernel translation unit.

const NO_FURTHER_WRITE_ACTION: i32 = -1;

unsafe fn efi_free_all_buff_pages(cap_info: *mut capsule_info) {
    while (*cap_info).index > 0 {
        __free_page((*cap_info).pages.offset({ (*cap_info).index -= 1; (*cap_info).index }));
    }
    (*cap_info).index = NO_FURTHER_WRITE_ACTION;
}

pub unsafe extern "C" fn __efi_capsule_setup_info(cap_info: *mut capsule_info) -> i32 {
    let pages_needed: usize = (ALIGN((*cap_info).total_size, PAGE_SIZE)) / PAGE_SIZE;
    let mut ret: i32;
    let mut temp_page: *mut core::ffi::c_void;

    if pages_needed == 0 {
        pr_err!("invalid capsule size\n");
        return -EINVAL;
    }

    ret = efi_capsule_supported(
        (*cap_info).header.guid,
        (*cap_info).header.flags,
        (*cap_info).header.imagesize,
        &mut (*cap_info).reset_type,
    );
    if ret != 0 {
        pr_err!("capsule not supported\n");
        return ret;
    }

    temp_page = krealloc(
        (*cap_info).pages as *mut core::ffi::c_void,
        pages_needed * core::mem::size_of::<*mut core::ffi::c_void>(),
        GFP_KERNEL | __GFP_ZERO,
    );
    if temp_page.is_null() {
        return -ENOMEM;
    }
    (*cap_info).pages = temp_page as *mut *mut page;

    temp_page = krealloc(
        (*cap_info).phys as *mut core::ffi::c_void,
        pages_needed * core::mem::size_of::<phys_addr_t>(),
        GFP_KERNEL | __GFP_ZERO,
    );
    if temp_page.is_null() {
        return -ENOMEM;
    }
    (*cap_info).phys = temp_page as *mut phys_addr_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn efi_capsule_setup_info(
    cap_info: *mut capsule_info,
    kbuff: *mut core::ffi::c_void,
    hdr_bytes: usize,
) -> i32 {
    if hdr_bytes < core::mem::size_of::<efi_capsule_header_t>() {
        return 0;
    }
    core::ptr::copy_nonoverlapping(
        kbuff as *const u8,
        &mut (*cap_info).header as *mut _ as *mut u8,
        core::mem::size_of_val(&(*cap_info).header),
    );
    (*cap_info).total_size = (*cap_info).header.imagesize;
    __efi_capsule_setup_info(cap_info)
}

unsafe fn efi_capsule_submit_update(cap_info: *mut capsule_info) -> isize {
    let mut do_vunmap = false;
    let ret: i32;

    if (*cap_info).capsule.is_null() {
        (*cap_info).capsule = vmap((*cap_info).pages, (*cap_info).index as usize, VM_MAP, PAGE_KERNEL);
        if (*cap_info).capsule.is_null() {
            return -ENOMEM as isize;
        }
        do_vunmap = true;
    }
    ret = efi_capsule_update((*cap_info).capsule, (*cap_info).phys);
    if do_vunmap {
        vunmap((*cap_info).capsule);
    }
    if ret != 0 {
        pr_err!("capsule update failed\n");
        return ret as isize;
    }
    (*cap_info).index = NO_FURTHER_WRITE_ACTION;
    if ((*cap_info).header.flags & EFI_CAPSULE_PERSIST_ACROSS_RESET) != 0 {
        pr_info!("Successfully uploaded capsule file with reboot type '{}'\n",
            if (*cap_info).reset_type == 0 { "RESET_COLD" }
            else if (*cap_info).reset_type == 1 { "RESET_WARM" }
            else { "RESET_SHUTDOWN" });
    } else {
        pr_info!("Successfully processed capsule file\n");
    }
    0
}

unsafe fn efi_capsule_write(file: *mut file, buff: *const u8, count: usize, _offp: *mut loff_t) -> isize {
    let cap_info = (*file).private_data as *mut capsule_info;
    let mut page: *mut page;
    let mut kbuff: *mut core::ffi::c_void = core::ptr::null_mut();
    let write_byte: usize;
    let ret: i32;

    if count == 0 { return 0; }
    if (*cap_info).index < 0 { return -EIO as isize; }
    if (*cap_info).page_bytes_remain == 0 {
        page = alloc_page(GFP_KERNEL);
        if page.is_null() { ret = -ENOMEM; goto_failed(cap_info, ret); return ret as isize; }
        *(*cap_info).pages.add((*cap_info).index as usize) = page;
        *(*cap_info).phys.add((*cap_info).index as usize) = page_to_phys(page);
        (*cap_info).page_bytes_remain = PAGE_SIZE;
        (*cap_info).index += 1;
    } else {
        page = *(*cap_info).pages.add((*cap_info).index as usize - 1);
    }
    kbuff = kmap(page).add(PAGE_SIZE - (*cap_info).page_bytes_remain);
    write_byte = core::cmp::min(count, (*cap_info).page_bytes_remain);
    if copy_from_user(kbuff, buff, write_byte) != 0 {
        kunmap(page); goto_failed(cap_info, -EFAULT); return -EFAULT as isize;
    }
    (*cap_info).page_bytes_remain -= write_byte;
    if (*cap_info).header.headersize == 0 {
        ret = efi_capsule_setup_info(cap_info, kbuff.sub((*cap_info).count), (*cap_info).count + write_byte);
        if ret != 0 { kunmap(page); goto_failed(cap_info, ret); return ret as isize; }
    }
    (*cap_info).count += write_byte;
    kunmap(page);
    if (*cap_info).header.headersize > 0 && (*cap_info).count >= (*cap_info).total_size {
        if (*cap_info).count > (*cap_info).total_size {
            pr_err!("capsule upload size exceeded header defined size\n");
            goto_failed(cap_info, -EINVAL); return -EINVAL as isize;
        }
        ret = efi_capsule_submit_update(cap_info) as i32;
        if ret != 0 { goto_failed(cap_info, ret); return ret as isize; }
    }
    write_byte as isize
}

unsafe fn goto_failed(cap_info: *mut capsule_info, _ret: i32) { efi_free_all_buff_pages(cap_info); }

unsafe fn efi_capsule_release(_inode: *mut inode, file: *mut file) -> i32 {
    let cap_info = (*file).private_data as *mut capsule_info;
    if (*cap_info).index > 0 && ((*cap_info).header.headersize == 0 || (*cap_info).count < (*cap_info).total_size) {
        pr_err!("capsule upload not complete\n");
        efi_free_all_buff_pages(cap_info);
    }
    kfree((*cap_info).pages as *mut core::ffi::c_void);
    kfree((*cap_info).phys as *mut core::ffi::c_void);
    kfree((*file).private_data);
    (*file).private_data = core::ptr::null_mut();
    0
}

unsafe fn efi_capsule_open(_inode: *mut inode, file: *mut file) -> i32 {
    let cap_info = kzalloc_obj::<capsule_info>();
    if cap_info.is_null() { return -ENOMEM; }
    (*cap_info).pages = kzalloc(core::mem::size_of::<*mut page>(), GFP_KERNEL) as *mut *mut page;
    if (*cap_info).pages.is_null() { kfree(cap_info as *mut _); return -ENOMEM; }
    (*cap_info).phys = kzalloc_obj::<phys_addr_t>();
    if (*cap_info).phys.is_null() { kfree((*cap_info).pages as *mut _); kfree(cap_info as *mut _); return -ENOMEM; }
    (*file).private_data = cap_info as *mut _;
    0
}

// The kernel's designated initializer and module registration macros are retained as
// declarations for the surrounding translation unit.
static mut efi_capsule_fops: file_operations = file_operations { owner: THIS_MODULE, open: Some(efi_capsule_open), write: Some(efi_capsule_write), release: Some(efi_capsule_release) };
static mut efi_capsule_misc: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"efi_capsule_loader\0".as_ptr(), fops: &efi_capsule_fops };

unsafe extern "C" fn efi_capsule_loader_init() -> i32 {
    if !efi_enabled(EFI_RUNTIME_SERVICES) { return -ENODEV; }
    let ret = misc_register(&mut efi_capsule_misc);
    if ret != 0 { pr_err!("Unable to register capsule loader device\n"); }
    ret
}

unsafe extern "C" fn efi_capsule_loader_exit() { misc_deregister(&mut efi_capsule_misc); }

// module_init(efi_capsule_loader_init);
// module_exit(efi_capsule_loader_exit);
// MODULE_DESCRIPTION("EFI capsule firmware binary loader");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
