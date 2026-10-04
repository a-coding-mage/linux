// SPDX-License-Identifier: GPL-2.0
// Rust production owner of the unchanged arch/x86/kernel/platform-quirks.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_early_platform_generated.rs"
    ));
}
use bindings as b;
use core::ptr::{addr_of, addr_of_mut, read_unaligned};

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn x86_early_init_platform_quirks() {
    // Use the complete configured x86_platform_ops layout. Raw pointers avoid
    // exclusive Rust references to globals which the callback may modify.
    let platform = addr_of_mut!(b::x86_platform);
    (*platform).legacy.i8042 = b::X86_LEGACY_I8042_EXPECTED_PRESENT;
    (*platform).legacy.rtc = 1;
    (*platform).legacy.warm_reset = 1;
    (*platform).legacy.reserve_bios_regions = 0;
    (*platform).legacy.devices.pnpbios = 1;

    // Both boot_params and setup_header are packed native header types.
    let params = addr_of!(b::boot_params);
    match read_unaligned(addr_of!((*params).hdr.hardware_subarch)) {
        b::X86_SUBARCH_PC => {
            (*platform).legacy.reserve_bios_regions = 1;
        }
        b::X86_SUBARCH_XEN => {
            (*platform).legacy.devices.pnpbios = 0;
            (*platform).legacy.rtc = 0;
        }
        b::X86_SUBARCH_INTEL_MID | b::X86_SUBARCH_CE4100 => {
            (*platform).legacy.devices.pnpbios = 0;
            (*platform).legacy.rtc = 0;
            (*platform).legacy.i8042 = b::X86_LEGACY_I8042_PLATFORM_ABSENT;
        }
        _ => {}
    }

    // Keep the configured C function-pointer ABI and call after all defaults
    // and subarchitecture-specific changes, exactly as in the C owner.
    if let Some(set_legacy_features) = (*platform).set_legacy_features {
        set_legacy_features();
    }
}

#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn x86_pnpbios_disabled() -> bool {
    (*addr_of!(b::x86_platform)).legacy.devices.pnpbios == 0
}

#[cfg(CONFIG_PNPBIOS)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_pnpbios_disabled() -> bool {
    x86_pnpbios_disabled()
}
