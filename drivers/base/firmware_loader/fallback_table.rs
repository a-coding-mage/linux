// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/kconfig.h, linux/list.h, linux/slab.h,
// linux/export.h, linux/security.h, linux/highmem.h, linux/umh.h,
// linux/sysctl.h, fallback.h, and firmware.h.

/*
 * firmware fallback configuration table
 */

pub static mut fw_fallback_config: firmware_fallback_config = firmware_fallback_config {
    // IS_ENABLED(CONFIG_FW_LOADER_USER_HELPER_FALLBACK)
    force_sysfs_fallback: cfg!(feature = "CONFIG_FW_LOADER_USER_HELPER_FALLBACK") as _,
    loading_timeout: 60,
    old_timeout: 60,
};

// EXPORT_SYMBOL_NS_GPL(fw_fallback_config, "FIRMWARE_LOADER_PRIVATE");

// CONFIG_SYSCTL
#[cfg(feature = "CONFIG_SYSCTL")]
static firmware_config_table: [ctl_table; 2] = [
    ctl_table {
        procname: "force_sysfs_fallback",
        data: unsafe { &mut fw_fallback_config.force_sysfs_fallback },
        maxlen: core::mem::size_of::<u32>(),
        mode: 0o644,
        proc_handler: proc_douintvec_minmax,
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE,
    },
    ctl_table {
        procname: "ignore_sysfs_fallback",
        data: unsafe { &mut fw_fallback_config.ignore_sysfs_fallback },
        maxlen: core::mem::size_of::<u32>(),
        mode: 0o644,
        proc_handler: proc_douintvec_minmax,
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE,
    },
];

#[cfg(feature = "CONFIG_SYSCTL")]
static mut firmware_config_sysct_table_header: *mut ctl_table_header = core::ptr::null_mut();

#[cfg(feature = "CONFIG_SYSCTL")]
pub unsafe fn register_firmware_config_sysctl() -> i32 {
    firmware_config_sysct_table_header = register_sysctl(
        "kernel/firmware_config",
        firmware_config_table.as_ptr(),
    );
    if firmware_config_sysct_table_header.is_null() {
        return -ENOMEM;
    }
    0
}

// EXPORT_SYMBOL_NS_GPL(register_firmware_config_sysctl, "FIRMWARE_LOADER_PRIVATE");

#[cfg(feature = "CONFIG_SYSCTL")]
pub unsafe fn unregister_firmware_config_sysctl() {
    unregister_sysctl_table(firmware_config_sysct_table_header);
    firmware_config_sysct_table_header = core::ptr::null_mut();
}

// EXPORT_SYMBOL_NS_GPL(unregister_firmware_config_sysctl, "FIRMWARE_LOADER_PRIVATE");

// #endif /* CONFIG_SYSCTL */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
