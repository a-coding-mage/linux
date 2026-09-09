/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by another translation unit.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cd_types {
    S3C_SDHCI_CD_INTERNAL, // use mmc internal CD line
    S3C_SDHCI_CD_EXTERNAL, // use external callback
    S3C_SDHCI_CD_GPIO,     // use external gpio pin for CD line
    S3C_SDHCI_CD_NONE,     // no CD line, use polling to detect card
    S3C_SDHCI_CD_PERMANENT, // no CD line, card permanently wired to host
}

/**
 * struct s3c_sdhci_platdata() - Platform device data for Samsung SDHCI
 * @max_width: The maximum number of data bits supported.
 * @host_caps: Standard MMC host capabilities bit field.
 * @host_caps2: The second standard MMC host capabilities bit field.
 * @cd_type: Type of Card Detection method (see cd_types enum above)
 * @ext_cd_init: Initialize external card detect subsystem. Called on
 *               sdhci-s3c driver probe when cd_type == S3C_SDHCI_CD_EXTERNAL.
 *               notify_func argument is a callback to the sdhci-s3c driver
 *               that triggers the card detection event. Callback arguments:
 *               dev is pointer to platform device of the host controller,
 *               state is new state of the card (0 - removed, 1 - inserted).
 * @ext_cd_cleanup: Cleanup external card detect subsystem. Called on
 *                  sdhci-s3c driver remove when cd_type == S3C_SDHCI_CD_EXTERNAL.
 *                  notify_func argument is the same callback as for ext_cd_init.
 * @ext_cd_gpio: gpio pin used for external CD line, valid only if
 *               cd_type == S3C_SDHCI_CD_GPIO
 * @ext_cd_gpio_invert: invert values for external CD gpio line
 * @cfg_gpio: Configure the GPIO for a specific card bit-width
 * 
 * Initialisation data specific to either the machine or the platform
 * for the device driver to use or call-back when configuring gpio or
 * card speed information.
*/
#[repr(C)]
pub struct s3c_sdhci_platdata {
    pub max_width: core::ffi::c_uint,
    pub host_caps: core::ffi::c_uint,
    pub host_caps2: core::ffi::c_uint,
    pub pm_caps: core::ffi::c_uint,
    pub cd_type: cd_types,

    pub ext_cd_gpio: core::ffi::c_int,
    pub ext_cd_gpio_invert: bool,
    pub ext_cd_init: Option<unsafe extern "C" fn(
        notify_func: Option<unsafe extern "C" fn(*mut platform_device, core::ffi::c_int)>,
    ) -> core::ffi::c_int>,
    pub ext_cd_cleanup: Option<unsafe extern "C" fn(
        notify_func: Option<unsafe extern "C" fn(*mut platform_device, core::ffi::c_int)>,
    ) -> core::ffi::c_int>,

    pub cfg_gpio: Option<unsafe extern "C" fn(dev: *mut platform_device, width: core::ffi::c_int)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
