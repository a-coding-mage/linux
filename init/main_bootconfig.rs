// SPDX-License-Identifier: GPL-2.0-only
//! Bootconfig discovery and ownership for the staged, unselected boot owner.
//!
//! The XBC parser remains its existing C subsystem. Inline header operations
//! below use its canonical declarations rather than nonexistent macro symbols.

use super::main_printk::main_printk;
#[cfg(any(CONFIG_BLK_DEV_INITRD, CONFIG_BOOT_CONFIG))]
use super::bindings;
#[cfg(CONFIG_BOOT_CONFIG)]
use super::main_globals;
use kernel::ffi::{c_char, c_int, c_void};
#[cfg(any(CONFIG_BLK_DEV_INITRD, CONFIG_BOOT_CONFIG))]
use kernel::ffi::c_long;

/// Locate and remove the bootconfig trailer using the original initrd bounds.
///
/// # Safety
///
/// The caller serializes the original external initrd bounds. A nonzero end
/// must have readable backing for the magic search and its preceding header,
/// just as the original C caller requires before its size check. An accepted
/// size names readable data inside that mapping. A nonnull `size_out` is writable.
#[cfg(CONFIG_BLK_DEV_INITRD)]
#[link_section = ".init.text"]
pub(super) unsafe fn get_boot_config_from_initrd(size_out: *mut usize) -> *mut c_void {
    // SAFETY: early boot supplies the mapped initrd and exclusive bounds/output.
    unsafe {
        if bindings::initrd_end == 0 {
            return core::ptr::null_mut();
        }
        let mut data = (bindings::initrd_end as *mut u8)
            .wrapping_sub(bindings::BOOTCONFIG_MAGIC_LEN as usize);
        let mut found = false;
        for _ in 0..4 {
            if bindings::memcmp(data.cast(), bindings::BOOTCONFIG_MAGIC.as_ptr().cast(),
                                bindings::BOOTCONFIG_MAGIC_LEN as usize) == 0 {
                found = true;
                break;
            }
            data = data.wrapping_sub(1);
        }
        if !found {
            return core::ptr::null_mut();
        }
        let header = data.wrapping_sub(8);
        // get_unaligned_le32 is a header operation, not an external function.
        let size = u32::from_le(header.cast::<u32>().read_unaligned());
        let checksum = u32::from_le(header.add(4).cast::<u32>().read_unaligned());
        data = header.wrapping_sub(size as usize);
        if (data as kernel::ffi::c_ulong) < bindings::initrd_start {
            main_printk!("get_boot_config_from_initrd",
                b"\x013bootconfig size %d is greater than initrd size %ld\n\0",
                size as c_int, bindings::initrd_end.wrapping_sub(bindings::initrd_start) as c_long);
            return core::ptr::null_mut();
        }
        let mut sum = 0u32;
        for offset in 0..size as usize {
            sum = sum.wrapping_add(u32::from(data.add(offset).read()));
        }
        if sum != checksum {
            main_printk!("get_boot_config_from_initrd", b"\x013bootconfig checksum failed\n\0");
            return core::ptr::null_mut();
        }
        bindings::initrd_end = data as kernel::ffi::c_ulong;
        if !size_out.is_null() {
            size_out.write(size as usize);
        }
        data.cast()
    }
}

#[cfg(not(CONFIG_BLK_DEV_INITRD))]
#[link_section = ".init.text"]
pub(super) unsafe fn get_boot_config_from_initrd(_size_out: *mut usize) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(CONFIG_BOOT_CONFIG)]
#[link_section = ".init.text"]
unsafe fn xbc_make_cmdline(key: *const c_char) -> *mut c_char {
    // SAFETY: the caller supplies the original live parsed XBC tree and key.
    // memblock retains sole ownership until a failed render or later boot use.
    unsafe {
        let root = bindings::xbc_node_find_subkey(core::ptr::null_mut(), key);
        if root.is_null() {
            return core::ptr::null_mut();
        }
        let length = bindings::xbc_snprint_cmdline(core::ptr::null_mut(), 0, root);
        if length <= 0 {
            return core::ptr::null_mut();
        }
        let size = length.wrapping_add(1) as usize;
        // This is exactly memblock_alloc's canonical inline backend/arguments.
        let command_line = bindings::memblock_alloc_try_nid(
            size as bindings::phys_addr_t,
            bindings::RUST_INIT_MAIN_SMP_CACHE_BYTES as bindings::phys_addr_t,
            bindings::RUST_INIT_MAIN_MEMBLOCK_LOW_LIMIT,
            bindings::RUST_INIT_MAIN_MEMBLOCK_ALLOC_ACCESSIBLE,
            bindings::NUMA_NO_NODE as c_int,
        ).cast::<c_char>();
        if command_line.is_null() {
            main_printk!("xbc_make_cmdline",
                b"\x013Failed to allocate memory for extra kernel cmdline.\n\0");
            return core::ptr::null_mut();
        }
        let result = bindings::xbc_snprint_cmdline(command_line, size, root);
        if result < 0 || result > length {
            main_printk!("xbc_make_cmdline", b"\x013Failed to print extra kernel cmdline.\n\0");
            bindings::memblock_free(command_line.cast(), size);
            return core::ptr::null_mut();
        }
        command_line
    }
}

/// Discover initrd/embedded configuration and preserve the original opt-in.
///
/// # Safety
///
/// Called during serialized early boot with the mapped initrd and terminated
/// boot command line. The XBC and memblock subsystems are available at their
/// original initialization phase. This does not establish that boot lifecycle.
#[cfg(CONFIG_BOOT_CONFIG)]
#[link_section = ".init.text"]
pub(super) unsafe fn setup_boot_config() {
    // SAFETY: the caller owns early boot state and all subsystem output storage.
    unsafe {
        let mut size = 0usize;
        let initrd_data = get_boot_config_from_initrd(&mut size).cast::<c_char>().cast_const();
        let from_embedded = initrd_data.is_null();
        let data = if from_embedded {
            #[cfg(CONFIG_BOOT_CONFIG_EMBED)]
            { bindings::xbc_get_embedded_bootconfig(&mut size) }
            #[cfg(not(CONFIG_BOOT_CONFIG_EMBED))]
            { core::ptr::null() }
        } else { initrd_data };
        let mut offset = 0;
        main_globals::bootconfig_found = bindings::bootconfig_cmdline_requested(
            core::ptr::addr_of!(main_globals::boot_command_line).cast(), &mut offset,
        );
        if !(main_globals::bootconfig_found || cfg!(CONFIG_BOOT_CONFIG_FORCE)) {
            return;
        }
        main_globals::initargs_offs = offset as usize;
        if data.is_null() {
            if main_globals::bootconfig_found {
                main_printk!("setup_boot_config",
                    b"\x013'bootconfig' found on command line, but no bootconfig found\n\0");
            } else {
                main_printk!("setup_boot_config",
                    b"\x016No bootconfig data provided, so skipping bootconfig\0");
            }
            return;
        }
        if size >= bindings::XBC_DATA_MAX as usize {
            main_printk!("setup_boot_config", b"\x013bootconfig size %ld greater than max size %d\n\0",
                         size as c_long, bindings::XBC_DATA_MAX as c_int);
            return;
        }
        let mut message = core::ptr::null();
        let mut position = 0;
        let mut result = bindings::xbc_init(data, size, &mut message, &mut position);
        if result < 0 {
            if position < 0 {
                main_printk!("setup_boot_config", b"\x013Failed to init bootconfig: %s.\n\0", message);
            } else {
                main_printk!("setup_boot_config", b"\x013Failed to parse bootconfig: %s at %d.\n\0",
                             message, position);
            }
        } else {
            bindings::xbc_get_info(&mut result, core::ptr::null_mut());
            main_printk!("setup_boot_config", b"\x016Load bootconfig: %ld bytes %d nodes\n\0", size as c_long, result);
            #[cfg(CONFIG_CMDLINE_FROM_BOOTCONFIG)]
            let applied = from_embedded && bindings::xbc_embedded_cmdline_applied();
            #[cfg(not(CONFIG_CMDLINE_FROM_BOOTCONFIG))]
            let applied = { let _ = from_embedded; false };
            if !applied {
                main_globals::extra_command_line = xbc_make_cmdline(c"kernel".as_ptr().cast());
            }
            main_globals::extra_init_args = xbc_make_cmdline(c"init".as_ptr().cast());
        }
    }
}

#[cfg(not(CONFIG_BOOT_CONFIG))]
#[link_section = ".init.text"]
pub(super) unsafe fn setup_boot_config() {
    // SAFETY: the same mapped initrd and serialized early boot as the C stub.
    unsafe { get_boot_config_from_initrd(core::ptr::null_mut()); }
}

#[link_section = ".init.text"]
pub(super) unsafe fn exit_boot_config() {
    #[cfg(CONFIG_BOOT_CONFIG)]
    // SAFETY: the caller has finished using the parsed bootconfig state.
    unsafe { bindings::_xbc_exit(false); }
}

#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn warn_bootconfig(_value: *mut c_char) -> c_int {
    #[cfg(not(CONFIG_BOOT_CONFIG))]
    #[cfg_attr(not(CONFIG_PRINTK), allow(unused_unsafe))]
    // SAFETY: the original warning is a fixed format without variadic values.
    unsafe {
        main_printk!("warn_bootconfig",
            b"\x014WARNING: 'bootconfig' found on the kernel command line but CONFIG_BOOT_CONFIG is not set.\n\0");
    }
    0
}

super::main_setup::setup_param!("bootconfig", bootconfig_record, Some(warn_bootconfig), 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
