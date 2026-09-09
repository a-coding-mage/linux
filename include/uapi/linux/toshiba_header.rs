/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* toshiba.h -- Linux driver for accessing the SMM on Toshiba laptops
 *
 * Copyright (c) 1996-2000  Jonathan A. Buzzard (jonathan@buzzard.org.uk)
 * Copyright (c) 2015  Azael Avalos <coproscefalo@gmail.com>
 *
 * Thanks to Juergen Heinzl <juergen@monocerus.demon.co.uk> for the pointers
 * on making sure the structure is aligned and packed.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 */

/* Toshiba modules paths */
pub const TOSH_PROC: &str = "/proc/toshiba";
pub const TOSH_DEVICE: &str = "/dev/toshiba";
pub const TOSHIBA_ACPI_PROC: &str = "/proc/acpi/toshiba";
pub const TOSHIBA_ACPI_DEVICE: &str = "/dev/toshiba_acpi";

/* Toshiba SMM structure */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SMMRegisters {
    pub eax: ::core::ffi::c_uint,
    pub ebx: ::core::ffi::c_uint,
    pub ecx: ::core::ffi::c_uint,
    pub edx: ::core::ffi::c_uint,
    pub esi: ::core::ffi::c_uint,
    pub edi: ::core::ffi::c_uint,
}

/* IOCTLs (0x90 - 0x91) */
#[macro_export]
macro_rules! TOSH_SMM {
    () => { _IOWR!('t', 0x90, $crate::SMMRegisters) };
}

/*
 * Convenience toshiba_acpi command.
 *
 * The System Configuration Interface (SCI) is opened/closed internally
 * to avoid userspace of buggy BIOSes.
 *
 * The toshiba_acpi module checks whether the eax register is set with
 * SCI_GET (0xf300) or SCI_SET (0xf400), returning -EINVAL if not.
 */
#[macro_export]
macro_rules! TOSHIBA_ACPI_SCI {
    () => { _IOWR!('t', 0x91, $crate::SMMRegisters) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
