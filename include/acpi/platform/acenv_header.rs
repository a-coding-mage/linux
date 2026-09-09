/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acenv.h - Host and compiler configuration
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Environment configuration. The purpose of this file is to interface ACPICA
 * to the local environment. This includes compiler-specific, OS-specific,
 * and machine-specific configuration.
 */

/* Types for ACPI_MUTEX_TYPE */
pub const ACPI_BINARY_SEMAPHORE: i32 = 0;
pub const ACPI_OSL_MUTEX: i32 = 1;

/* Types for DEBUGGER_THREADING */
pub const DEBUGGER_SINGLE_THREADED: i32 = 0;
pub const DEBUGGER_MULTI_THREADED: i32 = 1;

/*
 * Configuration for ACPI tools and utilities.
 * The following declarations correspond to the C preprocessor configuration.
 * Build systems should define the applicable feature symbols.
 */

/* ACPI_ASL_COMPILER, ACPI_BIN_APP, ACPI_DUMP_APP, ACPI_HELP_APP,
 * ACPI_NAMES_APP, ACPI_SRC_APP, ACPI_XTRACT_APP, ACPI_EXAMPLE_APP, or
 * ACPI_EFI_HELLO imply ACPI_APPLICATION, ACPI_SINGLE_THREADED, and
 * USE_NATIVE_ALLOCATE_ZEROED.
 */

/* ACPI_ASL_COMPILER additionally enables:
 * ACPI_DEBUG_OUTPUT, ACPI_CONSTANT_EVAL_ONLY, ACPI_LARGE_NAMESPACE_NODE,
 * ACPI_DATA_TABLE_DISASSEMBLY, ACPI_32BIT_PHYSICAL_ADDRESS,
 * ACPI_DISASSEMBLER = 1.
 */

/* ACPI_EXEC_APP enables ACPI_APPLICATION, ACPI_FULL_DEBUG, ACPI_MUTEX_DEBUG,
 * and ACPI_DBG_TRACK_ALLOCATIONS.
 */

/* ACPI_HELP_APP enables ACPI_NO_ERROR_MESSAGES. */
/* ACPI_NAMES_APP enables ACPI_DEBUG_OUTPUT. */
/* ACPI_EXEC_APP, ACPI_EXAMPLE_APP, and ACPI_NAMES_APP enable
 * ACPI_USE_NATIVE_RSDP_POINTER.
 */
/* ACPI_DUMP_APP enables ACPI_USE_NATIVE_MEMORY_MAPPING. */
/* ACPI_EXAMPLE_APP and ACPI_NAMES_APP enable ACPI_REDUCED_HARDWARE = 1. */
/* ACPI_LIBRARY enables ACPI_USE_LOCAL_CACHE, ACPI_DEBUGGER = 1, and
 * ACPI_DISASSEMBLER = 1; _DEBUG additionally enables ACPI_DEBUG_OUTPUT.
 */
/* ACPI_APPLICATION enables ACPI_USE_LOCAL_CACHE. */
/* ACPI_FULL_DEBUG enables ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER = 1, and
 * ACPI_DISASSEMBLER = 1.
 */

/* acpisrc CR/LF support: Unix output uses LF-only line endings. */
pub const ACPI_SRC_OS_LF_ONLY: i32 = 0;

/* Host/compiler configuration files are supplied by the target environment.
 * C selection order:
 * __GNUC__ -> acgcc.h; _MSC_VER -> acmsvc.h;
 * Linux -> aclinux.h; Apple -> acmacosx.h; DragonFly -> acdragonfly.h;
 * FreeBSD -> acfreebsd.h; NetBSD -> acnetbsd.h; Sun -> acsolaris.h;
 * MODESTO -> acmodesto.h; NETWARE -> acnetware.h; Cygwin -> accygwin.h;
 * WIN32 -> acwin.h; WIN64 -> acwin64.h; VxWorks -> acvxworks.h;
 * OS/2 -> acos2.h; Haiku -> achaiku.h; QNX -> acqnx.h;
 * EFI -> acefi.h; Zephyr -> aczephyr.h.
 * Unknown target environments are a configuration error.
 */

/* 64-bit data types. These may be supplied by the host/compiler configuration. */
pub type COMPILER_DEPENDENT_INT64 = i64;
pub type COMPILER_DEPENDENT_UINT64 = u64;

/* Type of mutex supported by host. Default is binary semaphores. */
pub const ACPI_MUTEX_TYPE: i32 = ACPI_BINARY_SEMAPHORE;

/* Default global-lock acquire/release operations. */
#[inline]
pub unsafe fn ACPI_ACQUIRE_GLOBAL_LOCK(_glptr: *mut core::ffi::c_void, acquired: *mut u32) {
    *acquired = 1;
}

#[inline]
pub unsafe fn ACPI_RELEASE_GLOBAL_LOCK(_glptr: *mut core::ffi::c_void, pending: *mut u32) {
    *pending = 0;
}

/* NULL/invalid value to use for destroyed or not-yet-created semaphores. */
pub const ACPI_SEMAPHORE_NULL: *mut core::ffi::c_void = core::ptr::null_mut();

/* Flush CPU cache - used when going to sleep. Default is a no-op. */
#[inline]
pub fn ACPI_FLUSH_CPU_CACHE() {}

/* Configurable inline and calling-convention keywords are empty by default. */
/* ACPI_INLINE, ACPI_SYSTEM_XFACE, ACPI_EXTERNAL_XFACE,
 * ACPI_INTERNAL_XFACE, and ACPI_INTERNAL_VAR_XFACE are empty. */

/* Ordered initialization fallback: ACPI_STRUCT_INIT(field, value) == value. */
#[inline]
pub const fn ACPI_STRUCT_INIT<T>(_field: T, value: T) -> T {
    value
}

/*
 * Debugger threading model. The C default is multi-threaded unless
 * ACPI_APPLICATION is defined without ACPI_EXEC_APP; in that case it is
 * single-threaded. This Rust constant represents the default non-application
 * configuration; build-specific configuration may override it.
 */
pub const DEBUGGER_THREADING: i32 = DEBUGGER_MULTI_THREADED;

/* C library configuration and standard-header inclusion are supplied by the
 * surrounding Rust/runtime environment. */

/* ACPI_FILE is FILE * for ACPI_APPLICATION and void * otherwise. */
pub type ACPI_FILE = *mut core::ffi::c_void;
pub const ACPI_FILE_OUT: *mut core::ffi::c_void = core::ptr::null_mut();
pub const ACPI_FILE_ERR: *mut core::ffi::c_void = core::ptr::null_mut();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
