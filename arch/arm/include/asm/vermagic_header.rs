/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Add the ARM architecture version to the version magic string.
 *
 * The C header obtains __LINUX_ARM_ARCH__ through the build configuration;
 * stringify! preserves that token-level expansion in Rust.
 */
pub const MODULE_ARCH_VERMAGIC_ARMVSN: &str = concat!(
    "ARMv",
    stringify!(__LINUX_ARM_ARCH__),
    " ",
);

/* Add __virt_to_phys patching state as well.
 *
 * CONFIG_ARM_PATCH_PHYS_VIRT is a build-time C preprocessor condition.  The
 * corresponding Rust configuration is represented by the feature of the same
 * name.
 */
#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub const MODULE_ARCH_VERMAGIC_P2V: &str = "p2v8 ";
#[cfg(not(feature = "CONFIG_ARM_PATCH_PHYS_VIRT"))]
pub const MODULE_ARCH_VERMAGIC_P2V: &str = "";

/* Add instruction set architecture tag to distinguish ARM/Thumb kernels.
 * CONFIG_THUMB2_KERNEL is a build-time C preprocessor condition; the Rust
 * feature of the same name preserves that condition.
 */
#[cfg(feature = "CONFIG_THUMB2_KERNEL")]
pub const MODULE_ARCH_VERMAGIC_ARMTHUMB: &str = "thumb2 ";
#[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
pub const MODULE_ARCH_VERMAGIC_ARMTHUMB: &str = "";

pub const MODULE_ARCH_VERMAGIC: &str = concat!(
    MODULE_ARCH_VERMAGIC_ARMVSN,
    MODULE_ARCH_VERMAGIC_ARMTHUMB,
    MODULE_ARCH_VERMAGIC_P2V,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
