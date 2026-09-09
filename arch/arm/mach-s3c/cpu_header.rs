/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com/
 *
 * Copyright (c) 2004-2005 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * Header file for Samsung CPU support
 */

/* todo - fix when rmk changes iodescs to use `void __iomem *` */

pub static mut samsung_cpu_id: ::core::ffi::c_ulong = 0;

pub const S3C6400_CPU_ID: ::core::ffi::c_ulong = 0x36400000;
pub const S3C6410_CPU_ID: ::core::ffi::c_ulong = 0x36410000;
pub const S3C64XX_CPU_MASK: ::core::ffi::c_ulong = 0xFFFFF000;

pub const S5PV210_CPU_ID: ::core::ffi::c_ulong = 0x43110000;
pub const S5PV210_CPU_MASK: ::core::ffi::c_ulong = 0xFFFFF000;

#[inline]
pub unsafe fn is_samsung_s3c6400() -> ::core::ffi::c_int {
    (if (samsung_cpu_id & S3C64XX_CPU_MASK) == (S3C6400_CPU_ID & S3C64XX_CPU_MASK) { 1 } else { 0 })
}

#[inline]
pub unsafe fn is_samsung_s3c6410() -> ::core::ffi::c_int {
    (if (samsung_cpu_id & S3C64XX_CPU_MASK) == (S3C6410_CPU_ID & S3C64XX_CPU_MASK) { 1 } else { 0 })
}

/* CONFIG_CPU_S3C6400 or CONFIG_CPU_S3C6410 selects the runtime checks. */
#[cfg(any(feature = "CONFIG_CPU_S3C6400", feature = "CONFIG_CPU_S3C6410"))]
#[inline]
pub unsafe fn soc_is_s3c6400() -> ::core::ffi::c_int { is_samsung_s3c6400() }

#[cfg(not(any(feature = "CONFIG_CPU_S3C6400", feature = "CONFIG_CPU_S3C6410")))]
#[inline]
pub fn soc_is_s3c6400() -> ::core::ffi::c_int { 0 }

#[cfg(feature = "CONFIG_CPU_S3C6410")]
#[inline]
pub unsafe fn soc_is_s3c6410() -> ::core::ffi::c_int { is_samsung_s3c6410() }

#[cfg(not(feature = "CONFIG_CPU_S3C6410"))]
#[inline]
pub fn soc_is_s3c6410() -> ::core::ffi::c_int { 0 }

#[cfg(any(feature = "CONFIG_CPU_S3C6400", feature = "CONFIG_CPU_S3C6410"))]
#[inline]
pub unsafe fn soc_is_s3c64xx() -> ::core::ffi::c_int {
    if is_samsung_s3c6400() != 0 || is_samsung_s3c6410() != 0 { 1 } else { 0 }
}

#[cfg(not(any(feature = "CONFIG_CPU_S3C6400", feature = "CONFIG_CPU_S3C6410")))]
#[inline]
pub fn soc_is_s3c64xx() -> ::core::ffi::c_int { 0 }

pub const MHZ: ::core::ffi::c_ulong = 1000 * 1000;

#[macro_export]
macro_rules! print_mhz {
    ($m:expr) => { (($m) / $crate::MHZ, ((($m) / 1000) % 1000)) };
}

pub struct s3c24xx_uart_resources;
pub struct platform_device;
pub struct s3c2410_uartcfg;
pub struct map_desc;

#[repr(C)]
pub struct cpu_table {
    pub idcode: ::core::ffi::c_ulong,
    pub idmask: ::core::ffi::c_ulong,
    pub map_io: Option<unsafe extern "C" fn()>,
    pub init_uarts: Option<unsafe extern "C" fn(*mut s3c2410_uartcfg, ::core::ffi::c_int)>,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub name: *const ::core::ffi::c_char,
}

extern "C" {
    pub fn s3c_init_cpu(
        idcode: ::core::ffi::c_ulong,
        cpus: *mut cpu_table,
        cputab_size: ::core::ffi::c_uint,
    );
    pub fn s3c64xx_init_cpu();
    pub fn s3c24xx_init_uarts(cfg: *mut s3c2410_uartcfg, no: ::core::ffi::c_int);
    pub fn s3c24xx_init_uartdevs(
        name: *mut ::core::ffi::c_char,
        res: *mut s3c24xx_uart_resources,
        cfg: *mut s3c2410_uartcfg,
        no: ::core::ffi::c_int,
    );
    pub static s3c6410_subsys: bus_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
