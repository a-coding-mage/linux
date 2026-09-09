// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Linaro Ltd;  <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the EFI stub headers are intentionally left external.

#[repr(C)]
pub union EfiRngProtocol {
    pub functions: EfiRngProtocolFunctions,
    pub mixed_mode: EfiRngProtocolMixedMode,
}

#[repr(C)]
pub struct EfiRngProtocolFunctions {
    pub get_info: unsafe extern "efiapi" fn(
        *mut EfiRngProtocol,
        *mut ::core::ffi::c_ulong,
        *mut EfiGuid,
    ) -> EfiStatus,
    pub get_rng: unsafe extern "efiapi" fn(
        *mut EfiRngProtocol,
        *mut EfiGuid,
        ::core::ffi::c_ulong,
        *mut u8,
    ) -> EfiStatus,
}

#[repr(C)]
pub struct EfiRngProtocolMixedMode {
    pub get_info: u32,
    pub get_rng: u32,
}

// These types and constants are provided by the EFI stub environment.
pub type EfiStatus = usize;
pub type EfiGuid = crate::efi_guid_t;

/**
 * efi_get_random_bytes() - fill a buffer with random bytes
 * @size: size of the buffer
 * @out: caller allocated buffer to receive the random bytes
 *
 * The call will fail if either the firmware does not implement the
 * EFI_RNG_PROTOCOL or there are not enough random bytes available to fill
 * the buffer.
 *
 * Return: status code
 */
pub unsafe fn efi_get_random_bytes(size: ::core::ffi::c_ulong, out: *mut u8) -> EfiStatus {
    let mut rng_proto = EFI_RNG_PROTOCOL_GUID;
    let mut rng: *mut EfiRngProtocol = core::ptr::null_mut();

    let status = efi_bs_call!(locate_protocol, &mut rng_proto, core::ptr::null_mut(),
                              &mut rng as *mut _ as *mut *mut core::ffi::c_void);
    if status != EFI_SUCCESS {
        return status;
    }

    efi_call_proto!(rng, get_rng, core::ptr::null_mut(), size, out)
}

/**
 * efi_random_get_seed() - provide random seed as configuration table
 *
 * The EFI_RNG_PROTOCOL is used to read random bytes. These random bytes are
 * saved as a configuration table which can be used as entropy by the kernel
 * for the initialization of its pseudo random number generator.
 *
 * If the EFI_RNG_PROTOCOL is not available or there are not enough random bytes
 * available, the configuration table will not be installed and an error code
 * will be returned.
 *
 * Return: status code
 */
pub unsafe fn efi_random_get_seed() -> EfiStatus {
    let mut rng_proto = EFI_RNG_PROTOCOL_GUID;
    let mut rng_algo_raw = EFI_RNG_ALGORITHM_RAW;
    let mut rng_table_guid = LINUX_EFI_RANDOM_SEED_TABLE_GUID;
    let mut prev_seed: *mut LinuxEfiRandomSeed;
    let mut seed: *mut LinuxEfiRandomSeed = core::ptr::null_mut();
    let mut prev_seed_size: i32 = 0;
    let mut seed_size: i32 = EFI_RANDOM_SEED_SIZE;
    let mut nv_seed_size: ::core::ffi::c_ulong = 0;
    let mut offset: ::core::ffi::c_ulong = 0;
    let mut rng: *mut EfiRngProtocol = core::ptr::null_mut();

    let mut status = efi_bs_call!(locate_protocol, &mut rng_proto, core::ptr::null_mut(),
                                  &mut rng as *mut _ as *mut *mut core::ffi::c_void);
    if status != EFI_SUCCESS {
        seed_size = 0;
    }

    // Call GetVariable() with a zero length buffer to obtain the size
    get_efi_var!("RandomSeed", &mut rng_table_guid, core::ptr::null_mut(),
                 &mut nv_seed_size, core::ptr::null_mut());
    if seed_size == 0 && nv_seed_size == 0 {
        return status;
    }

    seed_size += nv_seed_size as i32;

    prev_seed = get_efi_config_table!(rng_table_guid);
    if !prev_seed.is_null() && (*prev_seed).size <= 512 {
        prev_seed_size = (*prev_seed).size;
        seed_size += prev_seed_size;
    }

    status = efi_bs_call!(allocate_pool, EFI_ACPI_RECLAIM_MEMORY,
                          struct_size!(seed, bits, seed_size), &mut seed as *mut _ as *mut *mut core::ffi::c_void);
    if status != EFI_SUCCESS {
        efi_warn!("Failed to allocate memory for RNG seed.\n");
        if !prev_seed.is_null() {
            efi_warn!("Retaining bootloader-supplied seed only");
        }
        return status;
    }

    if !rng.is_null() {
        status = efi_call_proto!(rng, get_rng, &mut rng_algo_raw,
                                 EFI_RANDOM_SEED_SIZE, (*seed).bits.as_mut_ptr());
        if status == EFI_UNSUPPORTED {
            // Use whatever algorithm we have available if the raw algorithm is not implemented.
            status = efi_call_proto!(rng, get_rng, core::ptr::null_mut(),
                                     EFI_RANDOM_SEED_SIZE, (*seed).bits.as_mut_ptr());
        }
        if status == EFI_SUCCESS {
            offset = EFI_RANDOM_SEED_SIZE as _;
        }
    }

    if nv_seed_size != 0 {
        status = get_efi_var!("RandomSeed", &mut rng_table_guid, core::ptr::null_mut(),
                              &mut nv_seed_size, (*seed).bits.as_mut_ptr().add(offset as usize));
        if status == EFI_SUCCESS {
            // Delete the seed and hope EFI also zeros its representation on disk.
            status = set_efi_var!("RandomSeed", &mut rng_table_guid, 0, 0, core::ptr::null_mut());
        }
        if status == EFI_SUCCESS {
            offset += nv_seed_size;
        } else {
            memzero_explicit!((*seed).bits.as_mut_ptr().add(offset as usize), nv_seed_size);
        }
    }

    if offset == 0 {
        memzero_explicit!(*seed, struct_size!(seed, bits, seed_size));
        efi_bs_call!(free_pool, seed);
        efi_warn!("Failed to obtain seed from EFI_RNG_PROTOCOL or EFI variable\n");
        if !prev_seed.is_null() {
            efi_warn!("Retaining bootloader-supplied seed only");
        }
        return status;
    }
    if prev_seed_size != 0 {
        core::ptr::copy_nonoverlapping((*prev_seed).bits.as_ptr(),
            (*seed).bits.as_mut_ptr().add(offset as usize), prev_seed_size as usize);
        offset += prev_seed_size as ::core::ffi::c_ulong;
    }
    (*seed).size = offset;
    status = efi_bs_call!(install_configuration_table, &mut rng_table_guid, seed);
    if status != EFI_SUCCESS {
        memzero_explicit!(*seed, struct_size!(seed, bits, seed_size));
        efi_bs_call!(free_pool, seed);
        efi_warn!("Failed to obtain seed from EFI_RNG_PROTOCOL or EFI variable\n");
        if !prev_seed.is_null() {
            efi_warn!("Retaining bootloader-supplied seed only");
        }
        return status;
    }
    if prev_seed_size != 0 {
        memzero_explicit!((*prev_seed).bits.as_mut_ptr(), prev_seed_size as usize);
        efi_bs_call!(free_pool, prev_seed);
    }
    return EFI_SUCCESS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
