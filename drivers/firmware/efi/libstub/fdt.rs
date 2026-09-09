// SPDX-License-Identifier: GPL-2.0
/*
 * FDT related Helper functions used by the EFI stub on multiple
 * architectures. This should be included by the EFI stub
 * implementation files.
 *
 * Copyright 2013 Linaro Limited; author Roy Franz
 */

// C dependencies from linux/efi.h, linux/libfdt.h, asm/efi.h, and efistub.h
// are supplied by the surrounding translation unit.

const EFI_DT_ADDR_CELLS_DEFAULT: u32 = 2;
const EFI_DT_SIZE_CELLS_DEFAULT: u32 = 2;

unsafe fn fdt_update_cell_size(fdt: *mut core::ffi::c_void) {
    let offset: i32 = fdt_path_offset(fdt, "/" as *const str as *const i8);
    // Set the #address-cells and #size-cells values for an empty tree
    fdt_setprop_u32(fdt, offset, "#address-cells" as *const str as *const i8, EFI_DT_ADDR_CELLS_DEFAULT);
    fdt_setprop_u32(fdt, offset, "#size-cells" as *const str as *const i8, EFI_DT_SIZE_CELLS_DEFAULT);
}

unsafe fn update_fdt(orig_fdt: *mut core::ffi::c_void, orig_fdt_size: usize,
                     fdt: *mut core::ffi::c_void, new_fdt_size: i32,
                     cmdline_ptr: *mut i8) -> efi_status_t {
    let mut node: i32;
    let mut num_rsv: i32;
    let mut status: i32;
    let mut fdt_val32: fdt32_t;
    let mut fdt_val64: fdt64_t;

    if !orig_fdt.is_null() {
        if fdt_check_header(orig_fdt) != 0 {
            efi_err("Device Tree header not valid!\n");
            return EFI_LOAD_ERROR;
        }
        if orig_fdt_size != 0 && fdt_totalsize(orig_fdt) as usize > orig_fdt_size {
            efi_err("Truncated device tree! foo!\n");
            return EFI_LOAD_ERROR;
        }
    }

    if !orig_fdt.is_null() {
        status = fdt_open_into(orig_fdt, fdt, new_fdt_size);
    } else {
        status = fdt_create_empty_tree(fdt, new_fdt_size);
        if status == 0 { fdt_update_cell_size(fdt); }
    }
    if status != 0 { goto_fdt_set_fail(status); }

    num_rsv = fdt_num_mem_rsv(fdt);
    while { num_rsv -= 1; num_rsv >= 0 } { fdt_del_mem_rsv(fdt, num_rsv); }

    node = fdt_subnode_offset(fdt, 0, "chosen" as *const str as *const i8);
    if node < 0 {
        node = fdt_add_subnode(fdt, 0, "chosen" as *const str as *const i8);
        if node < 0 { status = node; goto_fdt_set_fail(status); }
    }
    if !cmdline_ptr.is_null() && strlen(cmdline_ptr) > 0 {
        status = fdt_setprop(fdt, node, "bootargs" as *const str as *const i8,
                             cmdline_ptr as *const core::ffi::c_void,
                             strlen(cmdline_ptr) + 1);
        if status != 0 { goto_fdt_set_fail(status); }
    }

    node = fdt_subnode_offset(fdt, 0, "chosen" as *const str as *const i8);
    fdt_val64 = cpu_to_fdt64((efi_system_table as usize) as u64);
    status = fdt_setprop_var(fdt, node, "linux,uefi-system-table" as *const str as *const i8, fdt_val64);
    if status != 0 { goto_fdt_set_fail(status); }
    fdt_val64 = cpu_to_fdt64(u64::MAX);
    status = fdt_setprop_var(fdt, node, "linux,uefi-mmap-start" as *const str as *const i8, fdt_val64);
    if status != 0 { goto_fdt_set_fail(status); }
    fdt_val32 = cpu_to_fdt32(u32::MAX);
    for name in ["linux,uefi-mmap-size", "linux,uefi-mmap-desc-size", "linux,uefi-mmap-desc-ver"] {
        status = fdt_setprop_var(fdt, node, name.as_ptr() as *const i8, fdt_val32);
        if status != 0 { goto_fdt_set_fail(status); }
    }
    if IS_ENABLED(CONFIG_RANDOMIZE_BASE) && !efi_nokaslr {
        let efi_status = efi_get_random_bytes(core::mem::size_of::<fdt64_t>(),
                                              &mut fdt_val64 as *mut _ as *mut u8);
        if efi_status == EFI_SUCCESS {
            status = fdt_setprop_var(fdt, node, "kaslr-seed" as *const str as *const i8, fdt_val64);
            if status != 0 { goto_fdt_set_fail(status); }
        }
    }
    fdt_pack(fdt);
    return EFI_SUCCESS;

    fn goto_fdt_set_fail(status: i32) -> efi_status_t {
        if status == -FDT_ERR_NOSPACE { EFI_BUFFER_TOO_SMALL } else { EFI_LOAD_ERROR }
    }
}

unsafe fn update_fdt_memmap(fdt: *mut core::ffi::c_void, map: *mut efi_boot_memmap) -> efi_status_t {
    let node = fdt_path_offset(fdt, "/chosen" as *const str as *const i8);
    if node < 0 { return EFI_LOAD_ERROR; }
    let mut val64 = cpu_to_fdt64((*map).map as usize as u64);
    if fdt_setprop_inplace_var(fdt, node, "linux,uefi-mmap-start" as *const str as *const i8, val64) != 0 { return EFI_LOAD_ERROR; }
    let mut val32 = cpu_to_fdt32((*map).map_size as u32);
    if fdt_setprop_inplace_var(fdt, node, "linux,uefi-mmap-size" as *const str as *const i8, val32) != 0 { return EFI_LOAD_ERROR; }
    val32 = cpu_to_fdt32((*map).desc_size as u32);
    if fdt_setprop_inplace_var(fdt, node, "linux,uefi-mmap-desc-size" as *const str as *const i8, val32) != 0 { return EFI_LOAD_ERROR; }
    val32 = cpu_to_fdt32((*map).desc_ver as u32);
    if fdt_setprop_inplace_var(fdt, node, "linux,uefi-mmap-desc-ver" as *const str as *const i8, val32) != 0 { return EFI_LOAD_ERROR; }
    EFI_SUCCESS
}

#[repr(C)]
struct exit_boot_struct {
    boot_memmap: *mut efi_boot_memmap,
    runtime_map: *mut efi_memory_desc_t,
    runtime_entry_count: i32,
    new_fdt_addr: *mut core::ffi::c_void,
}

unsafe fn exit_boot_func(map: *mut efi_boot_memmap, priv_: *mut core::ffi::c_void) -> efi_status_t {
    let p = &mut *(priv_ as *mut exit_boot_struct);
    p.boot_memmap = map;
    efi_get_virtmap((*map).map, (*map).map_size, (*map).desc_size, p.runtime_map, &mut p.runtime_entry_count);
    update_fdt_memmap(p.new_fdt_addr, map)
}

#[cfg(not(MAX_FDT_SIZE))]
const MAX_FDT_SIZE: usize = SZ_2M;

unsafe fn allocate_new_fdt_and_exit_boot(handle: *mut core::ffi::c_void, image: *mut efi_loaded_image_t,
                                         new_fdt_addr: *mut usize, cmdline_ptr: *mut i8) -> efi_status_t {
    let mut desc_size: usize = 0;
    let mut desc_ver: u32 = 0;
    let mut status: efi_status_t;
    let mut priv_ = exit_boot_struct { boot_memmap: core::ptr::null_mut(), runtime_map: core::ptr::null_mut(), runtime_entry_count: 0, new_fdt_addr: core::ptr::null_mut() };
    let mut fdt_addr: usize = 0;
    let mut fdt_size: usize = 0;
    if !efi_novamap {
        status = efi_alloc_virtmap(&mut priv_.runtime_map, &mut desc_size, &mut desc_ver);
        if status != EFI_SUCCESS { efi_err("Unable to retrieve UEFI memory map.\n"); return status; }
    }
    if !IS_ENABLED(CONFIG_EFI_ARMSTUB_DTB_LOADER) || efi_get_secureboot() != efi_secureboot_mode_disabled {
        if strstr(cmdline_ptr, "dtb=" as *const str as *const i8) != core::ptr::null_mut() { efi_err("Ignoring DTB from command line.\n"); }
    } else {
        status = efi_load_dtb(image, &mut fdt_addr, &mut fdt_size);
        if status != EFI_SUCCESS && status != EFI_NOT_READY { efi_err("Failed to load device tree!\n"); goto_fail(&mut priv_, fdt_size, fdt_addr); return EFI_LOAD_ERROR; }
    }
    if fdt_addr != 0 { efi_info("Using DTB from command line\n"); }
    else { fdt_addr = get_fdt(&mut fdt_size) as usize; if fdt_addr != 0 { efi_info("Using DTB from configuration table\n"); } }
    if fdt_addr == 0 { efi_info("Generating empty DTB\n"); }
    efi_info("Exiting boot services...\n");
    status = efi_allocate_pages(MAX_FDT_SIZE, new_fdt_addr, ULONG_MAX);
    if status != EFI_SUCCESS { efi_err("Unable to allocate memory for new device tree.\n"); goto_fail(&mut priv_, fdt_size, fdt_addr); return EFI_LOAD_ERROR; }
    status = update_fdt(fdt_addr as *mut _, fdt_size, *new_fdt_addr as *mut _, MAX_FDT_SIZE as i32, cmdline_ptr);
    if status != EFI_SUCCESS { efi_err("Unable to construct new device tree.\n"); efi_free(MAX_FDT_SIZE, *new_fdt_addr); goto_fail(&mut priv_, fdt_size, fdt_addr); return EFI_LOAD_ERROR; }
    priv_.new_fdt_addr = *new_fdt_addr as *mut _;
    status = efi_exit_boot_services(handle, &mut priv_ as *mut _ as *mut _, exit_boot_func);
    if status == EFI_SUCCESS {
        if efi_novamap { return EFI_SUCCESS; }
        let svam = (*efi_system_table).runtime.set_virtual_address_map;
        status = svam((priv_.runtime_entry_count as usize) * desc_size, desc_size, desc_ver, priv_.runtime_map);
        if status != EFI_SUCCESS {
            let mut l = 0; while l < (*priv_.boot_memmap).map_size { let p = ((*priv_.boot_memmap).map as *mut u8).add(l) as *mut efi_memory_desc_t; if (*p).attribute & EFI_MEMORY_RUNTIME != 0 { (*p).virt_addr = u64::MAX; } l += (*priv_.boot_memmap).desc_size; }
        }
        return EFI_SUCCESS;
    }
    efi_err("Exit boot services failed.\n");
    efi_free(MAX_FDT_SIZE, *new_fdt_addr);
    goto_fail(&mut priv_, fdt_size, fdt_addr);
    EFI_LOAD_ERROR
}

unsafe fn goto_fail(priv_: &mut exit_boot_struct, fdt_size: usize, fdt_addr: usize) {
    efi_free(fdt_size, fdt_addr);
    if !efi_novamap { efi_bs_call(free_pool, priv_.runtime_map); }
}

pub unsafe fn efi_boot_kernel(handle: *mut core::ffi::c_void, image: *mut efi_loaded_image_t,
                              kernel_addr: usize, cmdline_ptr: *mut i8) -> efi_status_t {
    let mut fdt_addr = 0usize;
    let status = allocate_new_fdt_and_exit_boot(handle, image, &mut fdt_addr, cmdline_ptr);
    if status != EFI_SUCCESS { efi_err("Failed to update FDT and exit boot services\n"); return status; }
    if IS_ENABLED(CONFIG_ARM) { efi_handle_post_ebs_state(); }
    efi_enter_kernel(kernel_addr, fdt_addr, fdt_totalsize(fdt_addr as *mut _));
}

pub unsafe fn get_fdt(fdt_size: *mut usize) -> *mut core::ffi::c_void {
    let fdt = get_efi_config_table(DEVICE_TREE_GUID);
    if fdt.is_null() { return core::ptr::null_mut(); }
    if fdt_check_header(fdt) != 0 { efi_err("Invalid header detected on UEFI supplied FDT, ignoring ...\n"); return core::ptr::null_mut(); }
    *fdt_size = fdt_totalsize(fdt) as usize;
    fdt
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
