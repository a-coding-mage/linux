// SPDX-License-Identifier: GPL-2.0
//! Helpers for early access to the EFI configuration table.
//!
//! Translation of compressed/efi.c; firmware layouts come from native headers.

use crate::bindings as b;
use crate::boot_efi_bindings as e;
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
#[cfg(CONFIG_X86_64)]
use core::mem::size_of;
use core::ptr::{addr_of, null_mut, read_unaligned};

#[inline]
unsafe fn debug_putstr(message: *const c_char) {
    #[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
    unsafe {
        b::__putstr(message);
    }
    #[cfg(not(CONFIG_X86_VERBOSE_BOOTUP))]
    let _ = message;
}

/// Determine whether the boot parameters describe 32-bit or 64-bit EFI.
#[no_mangle]
pub(crate) unsafe extern "C" fn efi_get_type(bp: *mut b::boot_params) -> e::efi_type {
    unsafe {
        // boot_params is packed. Never create a reference to its EFI fields.
        let ei = addr_of!((*bp).efi_info);
        let sig = addr_of!((*ei).efi_loader_signature).cast();
        let et = if b::strncmp(sig, b::EFI64_LOADER_SIGNATURE.as_ptr().cast(), 4) == 0 {
            e::EFI_TYPE_64
        } else if b::strncmp(sig, b::EFI32_LOADER_SIGNATURE.as_ptr().cast(), 4) == 0 {
            e::EFI_TYPE_32
        } else {
            debug_putstr(c"No EFI environment detected.\n".as_ptr());
            e::EFI_TYPE_NONE
        };

        // As in C, this is a non-EFI fallback rather than a hard error. Check
        // both high words, including the memory-map word, after the signature.
        #[cfg(not(CONFIG_X86_64))]
        if read_unaligned(addr_of!((*ei).efi_systab_hi)) != 0
            || read_unaligned(addr_of!((*ei).efi_memmap_hi)) != 0
        {
            debug_putstr(
                c"EFI system table is located above 4GB and cannot be accessed.\n".as_ptr(),
            );
            return e::EFI_TYPE_NONE;
        }

        et
    }
}

/// Return the system table physical address, or zero if it was not supplied.
#[no_mangle]
pub(crate) unsafe extern "C" fn efi_get_system_table(bp: *mut b::boot_params) -> c_ulong {
    unsafe {
        let ei = addr_of!((*bp).efi_info);
        let sys_tbl_pa = read_unaligned(addr_of!((*ei).efi_systab)) as c_ulong;
        #[cfg(CONFIG_X86_64)]
        let sys_tbl_pa =
            sys_tbl_pa | ((read_unaligned(addr_of!((*ei).efi_systab_hi)) as c_ulong) << 32);

        if sys_tbl_pa == 0 {
            debug_putstr(c"EFI system table not found.".as_ptr());
            return 0;
        }
        sys_tbl_pa
    }
}

// kexec supplies the original physical configuration-table address because
// the system table may already contain an inaccessible runtime virtual address.
#[cfg(CONFIG_X86_64)]
unsafe fn get_kexec_setup_data(bp: *mut b::boot_params) -> *mut e::efi_setup_data {
    unsafe {
        let mut esd: *mut e::efi_setup_data = null_mut();
        let mut pa_data = read_unaligned(addr_of!((*bp).hdr.setup_data));
        while pa_data != 0 {
            let data = pa_data as *const b::setup_data;
            if read_unaligned(addr_of!((*data).type_)) == b::SETUP_EFI {
                esd = pa_data.wrapping_add(size_of::<b::setup_data>() as u64)
                    as *mut e::efi_setup_data;
                break;
            }
            pa_data = read_unaligned(addr_of!((*data).next));
        }

        // An unusable first SETUP_EFI record falls back to normal EFI; do not
        // continue searching for a later EFI record or turn this into an error.
        if !esd.is_null() && read_unaligned(addr_of!((*esd).tables)) == 0 {
            debug_putstr(c"kexec EFI environment missing valid configuration table.\n".as_ptr());
            return null_mut();
        }
        esd
    }
}

#[cfg(not(CONFIG_X86_64))]
unsafe fn get_kexec_setup_data(_bp: *mut b::boot_params) -> *mut e::efi_setup_data {
    null_mut()
}

/// Return the configuration table address and entry count, leaving both output
/// parameters unchanged on error.
#[no_mangle]
pub(crate) unsafe extern "C" fn efi_get_conf_table(
    bp: *mut b::boot_params,
    cfg_tbl_pa: *mut c_ulong,
    cfg_tbl_len: *mut c_uint,
) -> c_int {
    unsafe {
        if cfg_tbl_pa.is_null() || cfg_tbl_len.is_null() {
            return -(b::EINVAL as c_int);
        }
        let sys_tbl_pa = efi_get_system_table(bp);
        if sys_tbl_pa == 0 {
            return -(b::EINVAL as c_int);
        }

        let et = efi_get_type(bp);
        if et == e::EFI_TYPE_64 {
            let stbl = sys_tbl_pa as *const e::efi_system_table_64_t;
            let esd = get_kexec_setup_data(bp);
            *cfg_tbl_pa = if !esd.is_null() {
                read_unaligned(addr_of!((*esd).tables)) as c_ulong
            } else {
                read_unaligned(addr_of!((*stbl).tables)) as c_ulong
            };
            *cfg_tbl_len = read_unaligned(addr_of!((*stbl).nr_tables));
        } else if et == e::EFI_TYPE_32 {
            let stbl = sys_tbl_pa as *const e::efi_system_table_32_t;
            *cfg_tbl_pa = read_unaligned(addr_of!((*stbl).tables)) as c_ulong;
            *cfg_tbl_len = read_unaligned(addr_of!((*stbl).nr_tables));
        } else {
            return -(b::EINVAL as c_int);
        }
        0
    }
}

// Get the physical address and GUID at the specified configuration-table index.
unsafe fn get_vendor_table(
    cfg_tbl: *const c_void,
    idx: c_uint,
    vendor_tbl_pa: *mut c_ulong,
    vendor_tbl_guid: *mut e::efi_guid_t,
    et: e::efi_type,
) -> c_int {
    unsafe {
        if et == e::EFI_TYPE_64 {
            let entry = cfg_tbl.cast::<e::efi_config_table_64_t>().add(idx as usize);
            let table = read_unaligned(addr_of!((*entry).table));
            #[cfg(not(CONFIG_X86_64))]
            if table >> 32 != 0 {
                debug_putstr(c"Error: EFI config table entry located above 4GB.\n".as_ptr());
                return -(b::EINVAL as c_int);
            }
            *vendor_tbl_pa = table as c_ulong;
            *vendor_tbl_guid = read_unaligned(addr_of!((*entry).guid));
        } else if et == e::EFI_TYPE_32 {
            let entry = cfg_tbl.cast::<e::efi_config_table_32_t>().add(idx as usize);
            *vendor_tbl_pa = read_unaligned(addr_of!((*entry).table)) as c_ulong;
            *vendor_tbl_guid = read_unaligned(addr_of!((*entry).guid));
        } else {
            return -(b::EINVAL as c_int);
        }
        0
    }
}

/// Search in firmware order and return the first matching vendor-table address.
#[no_mangle]
pub(crate) unsafe extern "C" fn efi_find_vendor_table(
    bp: *mut b::boot_params,
    cfg_tbl_pa: c_ulong,
    cfg_tbl_len: c_uint,
    guid: e::efi_guid_t,
) -> c_ulong {
    unsafe {
        let et = efi_get_type(bp);
        if et == e::EFI_TYPE_NONE {
            return 0;
        }
        for i in 0..cfg_tbl_len {
            let mut vendor_tbl_pa = 0;
            let mut vendor_tbl_guid = e::efi_guid_t { b: [0; 16] };
            if get_vendor_table(
                cfg_tbl_pa as *const c_void,
                i,
                &mut vendor_tbl_pa,
                &mut vendor_tbl_guid,
                et,
            ) != 0
            {
                return 0;
            }
            // efi_guidcmp compares all 16 bytes; equality needs no C inline body.
            if guid.b == vendor_tbl_guid.b {
                return vendor_tbl_pa;
            }
        }
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
