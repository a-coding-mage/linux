// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/init.h, linux/pnp.h, asm/setup.h, and
// asm/bios_ebda.h.

// The following symbols and types are provided by other translated files.
use crate::{boot_params, x86_platform};
use crate::{X86_LEGACY_I8042_EXPECTED_PRESENT, X86_LEGACY_I8042_PLATFORM_ABSENT};
use crate::{X86_SUBARCH_CE4100, X86_SUBARCH_INTEL_MID, X86_SUBARCH_PC, X86_SUBARCH_XEN};

// __init is a linker/section annotation in the C source and has no direct
// Rust-language equivalent.
pub unsafe fn x86_early_init_platform_quirks() {
    x86_platform.legacy.i8042 = X86_LEGACY_I8042_EXPECTED_PRESENT;
    x86_platform.legacy.rtc = 1;
    x86_platform.legacy.warm_reset = 1;
    x86_platform.legacy.reserve_bios_regions = 0;
    x86_platform.legacy.devices.pnpbios = 1;

    match boot_params.hdr.hardware_subarch {
        X86_SUBARCH_PC => {
            x86_platform.legacy.reserve_bios_regions = 1;
        }
        X86_SUBARCH_XEN => {
            x86_platform.legacy.devices.pnpbios = 0;
            x86_platform.legacy.rtc = 0;
        }
        X86_SUBARCH_INTEL_MID | X86_SUBARCH_CE4100 => {
            x86_platform.legacy.devices.pnpbios = 0;
            x86_platform.legacy.rtc = 0;
            x86_platform.legacy.i8042 = X86_LEGACY_I8042_PLATFORM_ABSENT;
        }
        _ => {}
    }

    if let Some(set_legacy_features) = x86_platform.set_legacy_features {
        set_legacy_features();
    }
}

pub unsafe fn x86_pnpbios_disabled() -> bool {
    x86_platform.legacy.devices.pnpbios == 0
}

// Preserve the CONFIG_PNPBIOS build-time condition from the C source.
#[cfg(CONFIG_PNPBIOS)]
pub unsafe fn arch_pnpbios_disabled() -> bool {
    x86_pnpbios_disabled()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
