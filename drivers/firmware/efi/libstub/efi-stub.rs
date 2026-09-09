// SPDX-License-Identifier: GPL-2.0-only
/*
 * EFI stub implementation that is shared by arm and arm64 architectures.
 * This should be included by the EFI stub implementation files.
 *
 * Copyright (C) 2013,2014 Linaro Limited
 *     Roy Franz <roy.franz@linaro.org
 * Copyright (C) 2013 Red Hat, Inc.
 *     Mark Salter <msalter@redhat.com>
 */

// Dependencies are supplied by the surrounding kernel Rust environment.

const EFI_RT_VIRTUAL_BASE: u64 = SZ_512M;
// EFI_RT_VIRTUAL_OFFSET defaults to zero when not supplied by the build.
const EFI_RT_VIRTUAL_OFFSET: u64 = 0;

static mut virtmap_base: u64 = EFI_RT_VIRTUAL_BASE;
static mut flat_va_mapping: bool = EFI_RT_VIRTUAL_OFFSET != 0;

#[no_mangle]
pub unsafe extern "C" fn free_primary_display(_dpy: *mut sysfb_display_info) {}

unsafe fn setup_primary_display() -> *mut sysfb_display_info {
    let dpy = alloc_primary_display();
    if dpy.is_null() {
        return core::ptr::null_mut();
    }
    let screen = &mut (*dpy).screen;
    #[cfg(CONFIG_FIRMWARE_EDID)]
    let edid = &mut (*dpy).edid;
    #[cfg(not(CONFIG_FIRMWARE_EDID))]
    let edid: *mut edid_info = core::ptr::null_mut();

    let status = efi_setup_graphics(screen, edid);
    if status != EFI_SUCCESS {
        free_primary_display(dpy);
        return core::ptr::null_mut();
    }
    dpy
}

unsafe fn install_memreserve_table() {
    let mut rsv: *mut linux_efi_memreserve = core::ptr::null_mut();
    let memreserve_table_guid = LINUX_EFI_MEMRESERVE_TABLE_GUID;
    let status = efi_bs_call_allocate_pool(
        EFI_LOADER_DATA,
        core::mem::size_of::<linux_efi_memreserve>(),
        &mut rsv as *mut _ as *mut *mut core::ffi::c_void,
    );
    if status != EFI_SUCCESS {
        efi_err!("Failed to allocate memreserve entry!\n");
        return;
    }
    (*rsv).next = 0;
    (*rsv).size = 0;
    atomic_set(&mut (*rsv).count, 0);
    let status = efi_bs_call_install_configuration_table(&memreserve_table_guid, rsv);
    if status != EFI_SUCCESS {
        efi_err!("Failed to install memreserve config table!\n");
    }
}

unsafe fn get_supported_rt_services() -> u32 {
    let mut supported = EFI_RT_SUPPORTED_ALL;
    let rt_prop_table = get_efi_config_table(EFI_RT_PROPERTIES_TABLE_GUID);
    if !rt_prop_table.is_null() {
        supported &= (*rt_prop_table).runtime_services_supported;
    }
    supported
}

pub unsafe extern "C" fn efi_handle_cmdline(
    image: *mut efi_loaded_image_t,
    cmdline_ptr: *mut *mut core::ffi::c_char,
) -> efi_status_t {
    let cmdline = efi_convert_cmdline(image);
    if cmdline.is_null() {
        efi_err!("getting command line via LOADED_IMAGE_PROTOCOL\n");
        return EFI_OUT_OF_RESOURCES;
    }
    if !IS_ENABLED!(CONFIG_CMDLINE_FORCE) {
        let status = efi_parse_options(cmdline);
        if status != EFI_SUCCESS {
            efi_err!("Failed to parse EFI load options\n");
            return status;
        }
    }
    if IS_ENABLED!(CONFIG_CMDLINE_EXTEND) || IS_ENABLED!(CONFIG_CMDLINE_FORCE) || *cmdline == 0 {
        let status = efi_parse_options(CONFIG_CMDLINE);
        if status != EFI_SUCCESS {
            efi_err!("Failed to parse built-in command line\n");
            return status;
        }
    }
    *cmdline_ptr = cmdline;
    EFI_SUCCESS
}

pub unsafe extern "C" fn efi_stub_common(
    handle: efi_handle_t,
    image: *mut efi_loaded_image_t,
    image_addr: c_ulong,
    cmdline_ptr: *mut core::ffi::c_char,
) -> efi_status_t {
    let status = check_platform_features();
    if status != EFI_SUCCESS { return status; }
    let dpy = setup_primary_display();
    efi_retrieve_eventlog();
    efi_enable_reset_attack_mitigation();
    efi_load_initrd(image, ULONG_MAX, efi_get_max_initrd_addr(image_addr), core::ptr::null_mut());
    efi_random_get_seed();
    efi_novamap |= !(get_supported_rt_services() & EFI_RT_SUPPORTED_SET_VIRTUAL_ADDRESS_MAP);
    install_memreserve_table();
    let status = efi_boot_kernel(handle, image, image_addr, cmdline_ptr);
    free_primary_display(dpy);
    status
}

/* Create a pool allocation large enough for the current EFI memory map. */
pub unsafe extern "C" fn efi_alloc_virtmap(
    virtmap: *mut *mut efi_memory_desc_t,
    desc_size: *mut c_ulong,
    desc_ver: *mut u32,
) -> efi_status_t {
    let mut size: c_ulong = 0;
    let mut mmap_key: c_ulong = 0;
    let status = efi_bs_call_get_memory_map(&mut size, core::ptr::null_mut(), &mut mmap_key, desc_size, desc_ver);
    if status != EFI_BUFFER_TOO_SMALL { return EFI_LOAD_ERROR; }
    efi_bs_call_allocate_pool(EFI_LOADER_DATA, size, virtmap as *mut *mut core::ffi::c_void)
}

/* Populate virtual addresses for EFI_MEMORY_RUNTIME descriptors and copy them. */
pub unsafe extern "C" fn efi_get_virtmap(
    memory_map: *mut efi_memory_desc_t,
    map_size: c_ulong,
    desc_size: c_ulong,
    runtime_map: *mut efi_memory_desc_t,
    count: *mut i32,
) {
    let mut efi_virt_base = virtmap_base;
    let mut out = runtime_map;
    *count = 0;
    let mut l: c_ulong = 0;
    while l < map_size {
        let in_desc = (memory_map as *mut u8).add(l as usize) as *mut efi_memory_desc_t;
        if (*in_desc).attribute & EFI_MEMORY_RUNTIME == 0 { l += desc_size; continue; }
        let paddr = (*in_desc).phys_addr;
        let mut size = (*in_desc).num_pages * EFI_PAGE_SIZE;
        (*in_desc).virt_addr = (*in_desc).phys_addr + EFI_RT_VIRTUAL_OFFSET;
        if efi_novamap { l += desc_size; continue; }
        if !flat_va_mapping {
            let aligned = round_down((*in_desc).phys_addr, SZ_64K);
            size += (*in_desc).phys_addr - aligned;
            if IS_ALIGNED!((*in_desc).phys_addr, SZ_2M) && size >= SZ_2M {
                efi_virt_base = round_up(efi_virt_base, SZ_2M);
            } else {
                efi_virt_base = round_up(efi_virt_base, SZ_64K);
            }
            (*in_desc).virt_addr += efi_virt_base - aligned;
            efi_virt_base += size;
        }
        core::ptr::copy_nonoverlapping(in_desc as *const u8, out as *mut u8, desc_size as usize);
        out = (out as *mut u8).add(desc_size as usize) as *mut efi_memory_desc_t;
        *count += 1;
        l += desc_size;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
