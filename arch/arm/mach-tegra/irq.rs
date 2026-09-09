// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@android.com>
 *
 * Copyright (C) 2010,2013, NVIDIA Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

const SGI_MASK: u32 = 0xFFFF;

#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut tegra_gic_cpu_base: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn tegra_pending_sgi() -> bool {
    let pending_set: u32;
    let distbase = IO_ADDRESS(TEGRA_ARM_INT_DIST_BASE);

    pending_set = readl_relaxed(distbase.add(GIC_DIST_PENDING_SET as usize));

    if pending_set & SGI_MASK != 0 {
        return true;
    }

    false
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn tegra_gic_notifier(
    _self: *mut notifier_block,
    cmd: usize,
    _v: *mut core::ffi::c_void,
) -> i32 {
    match cmd {
        CPU_PM_ENTER => {
            writel_relaxed(0x1E0, tegra_gic_cpu_base.add(GIC_CPU_CTRL as usize));
        }
        _ => {}
    }

    NOTIFY_OK
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut tegra_gic_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(tegra_gic_notifier),
};

#[cfg(feature = "CONFIG_PM_SLEEP")]
static tegra114_dt_gic_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"arm,cortex-a15-gic\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe fn tegra114_gic_cpu_pm_registration() {
    let dn = of_find_matching_node(core::ptr::null_mut(), tegra114_dt_gic_match.as_ptr());

    if dn.is_null() {
        return;
    }

    tegra_gic_cpu_base = of_iomap(dn, 1);

    cpu_pm_register_notifier(&raw mut tegra_gic_notifier_block);
}

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
unsafe fn tegra114_gic_cpu_pm_registration() {}

static tegra_ictlr_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"nvidia,tegra20-ictlr\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: b"nvidia,tegra30-ictlr\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

pub unsafe fn tegra_init_irq() {
    let dn = of_find_matching_node(core::ptr::null_mut(), tegra_ictlr_match.as_ptr());

    if WARN_ON(dn.is_null()) {
        pr_warn!("Outdated DT detected, suspend/resume will NOT work\n");
    }

    tegra114_gic_cpu_pm_registration();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
