// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Yun Liu <liuyun@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the architecture and EFI stub headers are intentionally
// left as external names, matching the C translation unit.

type KernelEntry = unsafe extern "C" fn(efi: bool, cmdline: ::core::ffi::c_ulong,
                                         systab: ::core::ffi::c_ulong) -> !;

pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    EFI_SUCCESS
}

pub unsafe extern "C" fn efi_cache_sync_image(
    _image_base: ::core::ffi::c_ulong,
    _alloc_size: ::core::ffi::c_ulong,
) {
    ::core::arch::asm!("ibar 0", options(nostack, preserves_flags));
}

pub unsafe extern "C" fn efi_get_kimg_kaslr_address() -> ::core::ffi::c_ulong {
    let mut random_offset: u32 = 0;

    // CONFIG_RANDOMIZE_BASE is a build-time condition in the original source.
    #[cfg(CONFIG_RANDOMIZE_BASE)]
    {
        if !efi_nokaslr {
            efi_get_random_bytes(
                ::core::mem::size_of::<u32>(),
                &mut random_offset as *mut u32 as *mut u8,
            );
            random_offset ^= random_get_entropy() << 16;
            random_offset &= CONFIG_RANDOMIZE_BASE_MAX_OFFSET - 1;
            random_offset = ALIGN(random_offset + SZ_64K, SZ_64K);
        }
    }

    PHYSADDR(VMLINUX_LOAD_ADDRESS) + random_offset as ::core::ffi::c_ulong
}

#[repr(C)]
pub struct exit_boot_struct {
    pub runtime_map: *mut efi_memory_desc_t,
    pub runtime_entry_count: ::core::ffi::c_int,
}

unsafe extern "C" fn exit_boot_func(
    map: *mut efi_boot_memmap,
    priv_: *mut ::core::ffi::c_void,
) -> efi_status_t {
    let p = priv_ as *mut exit_boot_struct;

    /*
     * Update the memory map with virtual addresses. The function will also
     * populate @runtime_map with copies of just the EFI_MEMORY_RUNTIME
     * entries so that we can pass it straight to SetVirtualAddressMap()
     */
    efi_get_virtmap(
        (*map).map,
        (*map).map_size,
        (*map).desc_size,
        (*p).runtime_map,
        &mut (*p).runtime_entry_count,
    );

    EFI_SUCCESS
}

pub unsafe extern "C" fn kernel_entry_address(
    kernel_addr: ::core::ffi::c_ulong,
    _image: *mut efi_loaded_image_t,
) -> ::core::ffi::c_ulong {
    *(kernel_addr.wrapping_add(8) as *const ::core::ffi::c_ulong)
        - PHYSADDR(VMLINUX_LOAD_ADDRESS)
        + kernel_addr
}

pub unsafe extern "C" fn efi_boot_kernel(
    handle: *mut ::core::ffi::c_void,
    image: *mut efi_loaded_image_t,
    kernel_addr: ::core::ffi::c_ulong,
    cmdline_ptr: *mut ::core::ffi::c_char,
) -> efi_status_t {
    let real_kernel_entry: KernelEntry;
    let mut priv_: exit_boot_struct = ::core::mem::MaybeUninit::zeroed().assume_init();
    let mut desc_size: ::core::ffi::c_ulong = 0;
    let mut status: efi_status_t;
    let mut desc_ver: u32 = 0;

    status = efi_alloc_virtmap(&mut priv_.runtime_map, &mut desc_size, &mut desc_ver);
    if status != EFI_SUCCESS {
        efi_err("Unable to retrieve UEFI memory map.\0".as_ptr() as *const ::core::ffi::c_char);
        return status;
    }

    efi_info("Exiting boot services\n\0".as_ptr() as *const ::core::ffi::c_char);

    efi_novamap = false;
    status = efi_exit_boot_services(handle, &mut priv_ as *mut _ as *mut _, Some(exit_boot_func));
    if status != EFI_SUCCESS {
        return status;
    }

    /* Install the new virtual address map */
    efi_rt_call(
        set_virtual_address_map,
        priv_.runtime_entry_count as ::core::ffi::c_ulong * desc_size,
        desc_size,
        desc_ver,
        priv_.runtime_map,
    );

    /* Config Direct Mapping */
    csr_write(CSR_DMW0_INIT, LOONGARCH_CSR_DMWIN0);
    csr_write(CSR_DMW1_INIT, LOONGARCH_CSR_DMWIN1);
    csr_write(CSR_DMW2_INIT, LOONGARCH_CSR_DMWIN2);
    csr_write(CSR_DMW3_INIT, LOONGARCH_CSR_DMWIN3);

    real_kernel_entry = ::core::mem::transmute(kernel_entry_address(kernel_addr, image));
    real_kernel_entry(true, cmdline_ptr as ::core::ffi::c_ulong, efi_system_table as ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
