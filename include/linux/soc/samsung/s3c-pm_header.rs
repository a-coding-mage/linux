/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2013 Samsung Electronics Co., Ltd.
 *	Tomasz Figa <t.figa@samsung.com>
 * Copyright (c) 2004 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Written by Ben Dooks, <ben@simtec.com>
 */

// Dependency supplied by the surrounding kernel translation.

/* PM debug functions */

macro_rules! S3C_PMDBG {
    ($($arg:tt)*) => {
        pr_debug!($($arg)*)
    };
}

#[inline]
fn s3c_pm_save_uarts(_is_s3c24xx: bool) {}

#[inline]
fn s3c_pm_restore_uarts(_is_s3c24xx: bool) {}

/* suspend memory checking */

// CONFIG_SAMSUNG_PM_CHECK is a build-time configuration condition.
#[cfg(feature = "CONFIG_SAMSUNG_PM_CHECK")]
extern "C" {
    fn s3c_pm_check_prepare();
    fn s3c_pm_check_restore();
    fn s3c_pm_check_cleanup();
    fn s3c_pm_check_store();
}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM_CHECK"))]
#[inline]
fn s3c_pm_check_prepare() {}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM_CHECK"))]
#[inline]
fn s3c_pm_check_restore() {}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM_CHECK"))]
#[inline]
fn s3c_pm_check_cleanup() {}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM_CHECK"))]
#[inline]
fn s3c_pm_check_store() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
