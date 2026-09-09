/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2004-2007, 2010-2015 Freescale Semiconductor, Inc.
 * Copyright (C) 2008 Juergen Beisert (kernel@pengutronix.de)
 */

// C dependencies: linux/types.h and soc/imx/cpu.h.
// This header is intended to be included through the MXC hardware header.

pub const IMX_DDR_TYPE_LPDDR2: u32 = 1;

// The CONFIG_SOC_IMX6SL build condition is preserved by providing the
// configuration-dependent implementation at the call site.
#[inline]
pub unsafe fn cpu_is_imx6sl() -> bool {
    #[cfg(CONFIG_SOC_IMX6SL)]
    {
        __mxc_cpu_type == MXC_CPU_IMX6SL
    }
    #[cfg(not(CONFIG_SOC_IMX6SL))]
    {
        false
    }
}

#[inline]
pub unsafe fn cpu_is_imx6dl() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6DL
}

#[inline]
pub unsafe fn cpu_is_imx6sx() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6SX
}

#[inline]
pub unsafe fn cpu_is_imx6ul() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6UL
}

#[inline]
pub unsafe fn cpu_is_imx6ull() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6ULL
}

#[inline]
pub unsafe fn cpu_is_imx6ulz() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6ULZ
}

#[inline]
pub unsafe fn cpu_is_imx6sll() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6SLL
}

#[inline]
pub unsafe fn cpu_is_imx6q() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX6Q
}

#[inline]
pub unsafe fn cpu_is_imx7d() -> bool {
    __mxc_cpu_type == MXC_CPU_IMX7D
}

#[repr(C)]
pub struct cpu_op {
    pub cpu_rate: u32,
}

extern "C" {
    pub fn tzic_enable_wake() -> i32;
    pub static mut get_cpu_op: Option<unsafe extern "C" fn(op: *mut i32) -> *mut cpu_op>;
}

// C aliases:
// #define imx_readl  readl_relaxed
// #define imx_readw  readw_relaxed
// #define imx_writel writel_relaxed
// #define imx_writew writew_relaxed
#[macro_export]
macro_rules! imx_readl { ($($args:tt)*) => { readl_relaxed!($($args)*) }; }
#[macro_export]
macro_rules! imx_readw { ($($args:tt)*) => { readw_relaxed!($($args)*) }; }
#[macro_export]
macro_rules! imx_writel { ($($args:tt)*) => { writel_relaxed!($($args)*) }; }
#[macro_export]
macro_rules! imx_writew { ($($args:tt)*) => { writew_relaxed!($($args)*) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
