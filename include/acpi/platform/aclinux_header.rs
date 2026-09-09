/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: aclinux.h - OS specific defines, etc. for Linux
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* C header guard: __ACLINUX_H__ */
/* Build-time __KERNEL__ conditionals are preserved below with cfg comments. */

/* Common (in-kernel/user-space) ACPICA configuration */
pub const ACPI_USE_SYSTEM_CLIBRARY: bool = true;
pub const ACPI_USE_DO_WHILE_0: bool = true;
pub const ACPI_IGNORE_PACKAGE_RESOLUTION_ERRORS: bool = true;

/* __KERNEL__ */
pub const ACPI_USE_SYSTEM_INTTYPES: bool = true;
pub const ACPI_USE_GPE_POLLING: bool = true;

/* Kernel specific ACPICA configuration */
/* CONFIG_PCI => ACPI_PCI_CONFIGURED */
/* CONFIG_ACPI_REDUCED_HARDWARE_ONLY => ACPI_REDUCED_HARDWARE = 1 */
/* CONFIG_ACPI_DEBUGGER => ACPI_DEBUGGER */
/* CONFIG_ACPI_DEBUG => ACPI_MUTEX_DEBUG */

/* Kernel headers supply string, kernel, ctype, sched, atomic, math64, slab,
 * spinlock_types, export, and acenv declarations. */

/* #define ACPI_INIT_FUNCTION __init */
/* #define ACPI_DEBUG_DEFAULT (ACPI_LV_INFO | ACPI_LV_REPAIR) */

/* !CONFIG_ACPI: external globals and configurable interfaces are stubs. */
#[macro_export]
macro_rules! ACPI_GLOBAL { ($t:ty, $a:ident) => {}; }
#[macro_export]
macro_rules! ACPI_INIT_GLOBAL { ($t:ty, $a:ident, $b:expr) => {}; }
pub const ACPI_NO_MEM_ALLOCATIONS: bool = true;
pub const ACPI_NO_ERROR_MESSAGES: bool = true;

#[macro_export]
macro_rules! ACPI_EXTERNAL_RETURN_STATUS { ($($prototype:tt)*) => { /* return AE_NOT_CONFIGURED */ }; }
#[macro_export]
macro_rules! ACPI_EXTERNAL_RETURN_OK { ($($prototype:tt)*) => { /* return AE_OK */ }; }
#[macro_export]
macro_rules! ACPI_EXTERNAL_RETURN_VOID { ($($prototype:tt)*) => { /* return */ }; }
#[macro_export]
macro_rules! ACPI_EXTERNAL_RETURN_UINT32 { ($($prototype:tt)*) => { /* return 0 */ }; }
#[macro_export]
macro_rules! ACPI_EXTERNAL_RETURN_PTR { ($($prototype:tt)*) => { /* return NULL */ }; }

/* Host-dependent types and defines for in-kernel ACPICA */
/* ACPI_MACHINE_WIDTH is BITS_PER_LONG. */
pub const ACPI_USE_NATIVE_MATH64: bool = true;
/* ACPI_EXPORT_SYMBOL(symbol) expands to EXPORT_SYMBOL(symbol); */
/* strtoul expands to simple_strtoul. */

/* Kernel-provided type mappings:
 * acpi_cache_t = struct kmem_cache
 * acpi_spinlock = spinlock_t *
 * acpi_raw_spinlock = raw_spinlock_t *
 * acpi_cpu_flags = unsigned long
 * acpi_uintptr_t = uintptr_t
 */

#[inline]
pub unsafe fn ACPI_TO_INTEGER<T>(p: *const T) -> usize { p as usize }

/* ACPI_OFFSET(d, f) expands to offsetof(d, f); */
pub const USE_NATIVE_ALLOCATE_ZEROED: bool = true;
pub const ACPI_GPE_USE_LOGICAL_ADDRESSES: bool = true;

/* Overrides for in-kernel ACPICA and OSL interfaces. */
/* ACPI_USE_ALTERNATE_PROTOTYPE_* declarations are build-time markers for:
 * acpi_os_initialize, acpi_os_terminate, acpi_os_allocate,
 * acpi_os_allocate_zeroed, acpi_os_free, acpi_os_acquire_object,
 * acpi_os_get_thread_id, acpi_os_create_lock, acpi_os_create_raw_lock,
 * acpi_os_delete_raw_lock, acpi_os_acquire_raw_lock,
 * acpi_os_release_raw_lock, acpi_os_readable, acpi_os_writable,
 * acpi_os_initialize_debugger, acpi_os_terminate_debugger,
 * acpi_os_redirect_output, acpi_os_get_table_by_name,
 * acpi_os_get_table_by_index, acpi_os_get_table_by_address,
 * acpi_os_open_directory, acpi_os_get_next_filename, acpi_os_close_directory.
 */

/* Linux message prefixes (KERN_* are supplied by the kernel). */
/* ACPI_MSG_ERROR = KERN_ERR "ACPI Error: " */
/* ACPI_MSG_EXCEPTION = KERN_ERR "ACPI Exception: " */
/* ACPI_MSG_WARNING = KERN_WARNING "ACPI Warning: " */
/* ACPI_MSG_INFO = KERN_INFO "ACPI: " */
/* ACPI_MSG_BIOS_ERROR = KERN_ERR "ACPI BIOS Error (bug): " */
/* ACPI_MSG_BIOS_WARNING = KERN_WARNING "ACPI BIOS Warning (bug): " */

#[macro_export]
macro_rules! ACPI_STRUCT_INIT { ($field:ident, $value:expr) => { $field: $value }; }

/* !__KERNEL__ */
pub const ACPI_USE_STANDARD_HEADERS: bool = true;
/* Standard headers supply stddef, unistd, and stdint; ACPI_OFFSET is offsetof. */

/* __init and __iomem are empty declarators in user space. */

#[macro_export]
macro_rules! ACPI_FLUSH_CPU_CACHE { () => {}; }
#[inline]
pub fn ACPI_CAST_PTHREAD_T<T>(pthread: T) -> T { pthread }

/* Architecture-dependent selection of machine width and native integer types
 * is a build-time condition. On 64-bit targets ACPI_MACHINE_WIDTH is 64,
 * with long/unsigned long 64-bit types; otherwise it is 32, with long long /
 * unsigned long long 64-bit types and native divide/math64 enabled.
 */

/* __cdecl is empty when not otherwise defined. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
