/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Linaro Ltd.
 * Copyright (C) 2021 Dávid Virág <virag.david003@gmail.com>
 * Author: Sam Protsenko <semen.protsenko@linaro.org>
 * Author: Dávid Virág <virag.david003@gmail.com>
 *
 * This file contains shared functions used by some arm64 Exynos SoCs,
 * such as Exynos7885 or Exynos850 to register and init CMUs.
 */

// C dependency: declarations supplied by "clk.h".

extern "C" {
    pub fn exynos_arm64_register_cmu(
        dev: *mut device,
        np: *mut device_node,
        cmu: *const samsung_cmu_info,
    );
    pub fn exynos_arm64_register_cmu_pm(pdev: *mut platform_device, set_manual: bool) -> i32;
    pub fn exynos_arm64_cmu_suspend(dev: *mut device) -> i32;
    pub fn exynos_arm64_cmu_resume(dev: *mut device) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
