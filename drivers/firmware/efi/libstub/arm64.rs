// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013, 2014 Linaro Ltd;  <roy.franz@linaro.org>
 *
 * This file implements the EFI boot stub for the arm64 kernel.
 * Adapted from ARM version by Mark Salter <msalter@redhat.com>
 */

// Dependencies supplied by the surrounding kernel EFI stub build are intentionally
// referenced here rather than reimplemented in this translation unit.

#[repr(C)]
pub struct EfiSmbiosType4Record {
    pub processor_id: [u8; 8],
    // Remaining fields are supplied by the ABI and are accessed externally.
}

type EfiStatus = u64;

extern "C" {
    fn efi_get_smbios_record(ty: u8) -> *mut core::ffi::c_void;
    fn efi_get_smbios_string(record: *const EfiSmbiosType4Record, field: u32) -> *const u8;
    fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32;
    fn efi_warn(fmt: *const u8, ...);
    fn efi_err(fmt: *const u8, ...);
    fn read_cpuid(reg: u32) -> u64;
    fn read_cpuid_effective_cachetype() -> u32;
    fn cpuid_feature_extract_unsigned_field(value: u32, shift: u32) -> u32;
    fn efi_remap_image(image_base: usize, alloc_size: usize, code_size: u32);
    fn dsb(scope: u32);
    fn isb();
    static mut efi_novamap: bool;
    static VA_BITS_MIN: u32;
    static code_size: u32;
}

const EFI_SUCCESS: EfiStatus = 0;
const EFI_UNSUPPORTED: EfiStatus = 3;
const ISH: u32 = 0xb;

unsafe fn system_needs_vamap() -> bool {
    let record = efi_get_smbios_record(4) as *mut EfiSmbiosType4Record;
    if record.is_null() {
        return false;
    }

    let socid = (*record).processor_id.as_ptr() as *const u32;
    let altra = b"Ampere(TM) Altra(TM) Processor\0";
    let emag = b"eMAG\0";
    let version: *const u8;

    match (*socid & 0xffff000f) {
        0x0a160001 | 0x0a160002 => {
            efi_warn(b"Working around broken SetVirtualAddressMap()\n\0".as_ptr());
            return true;
        }
        _ => {
            // processor_version is the SMBIOS type 4 processor-version field.
            version = efi_get_smbios_string(record, 0);
            if version.is_null()
                || (strncmp(version, altra.as_ptr(), altra.len() - 1) != 0
                    && strncmp(version, emag.as_ptr(), emag.len() - 1) != 0)
            {
                return false;
            }
            efi_warn(b"Working around broken SetVirtualAddressMap()\n\0".as_ptr());
            return true;
        }
    }
}

pub unsafe fn check_platform_features() -> EfiStatus {
    let mut tg: u64;

    if *VA_BITS_MIN >= 48 && !system_needs_vamap() {
        efi_novamap = true;
    }

    // UEFI mandates support for 4 KB granularity, no need to check.
    // Build-time CONFIG_ARM64_4K_PAGES condition is supplied by the kernel build.
    if cfg!(feature = "CONFIG_ARM64_4K_PAGES") {
        return EFI_SUCCESS;
    }

    tg = (read_cpuid(0) >> 28) & 0xf;
    if tg < 0 || tg > 0xf {
        if cfg!(feature = "CONFIG_ARM64_64K_PAGES") {
            efi_err(b"This 64 KB granular kernel is not supported by your CPU\n\0".as_ptr());
        } else {
            efi_err(b"This 16 KB granular kernel is not supported by your CPU\n\0".as_ptr());
        }
        return EFI_UNSUPPORTED;
    }
    EFI_SUCCESS
}

#[no_mangle]
pub static mut code_size: u32 = 0;

pub unsafe fn efi_cache_sync_image(image_base: usize, alloc_size: usize) {
    let ctr = read_cpuid_effective_cachetype();
    let lsize = 4u64 << cpuid_feature_extract_unsigned_field(ctr, 0);

    // only perform the cache maintenance if needed for I/D coherency
    if (ctr & (1 << 0)) == 0 {
        let mut base = image_base as u64;
        let mut size = code_size as u64;
        loop {
            core::arch::asm!("dc cvau, {0}", in(reg) base);
            base = base.wrapping_add(lsize);
            size = size.wrapping_sub(lsize);
            if size < lsize { break; }
        }
    }

    core::arch::asm!("ic ialluis");
    dsb(ISH);
    isb();
    efi_remap_image(image_base, alloc_size, code_size);
}

pub unsafe fn primary_entry_offset() -> usize {
    /*
     * By default, we can invoke the kernel via the branch instruction in
     * the image header, so offset #0. This will be overridden by the EFI
     * stub build that is linked into the core kernel, as in that case, the
     * image header may not have been loaded into memory, or may be mapped
     * with non-executable permissions.
     */
    0
}

pub unsafe fn efi_enter_kernel(entrypoint: usize, fdt_addr: usize, _fdt_size: usize) -> ! {
    let entry = entrypoint.wrapping_add(primary_entry_offset());
    let enter_kernel: extern "C" fn(u64, u64, u64, u64) -> ! = core::mem::transmute(entry);
    enter_kernel(fdt_addr as u64, 0, 0, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
