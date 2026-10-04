/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_EFI_BINDINGS_H
#define LUPOS_BOOT_EFI_BINDINGS_H
#ifdef __BINDGEN__
/* Expose native declarations without changing their types or definitions. */
#pragma GCC visibility push(default)
#endif
#include "misc.h"

/* Layout values only. All table-access and search bodies live in Rust. */
enum {
	LUPOS_BOOT_EFI_GUID_SIZE = sizeof(efi_guid_t),
	LUPOS_BOOT_EFI_GUID_ALIGN = __alignof__(efi_guid_t),
	LUPOS_BOOT_EFI_GUID_BYTES = sizeof(((efi_guid_t *)0)->b),
	LUPOS_BOOT_EFI_GUID_B_OFFSET = __builtin_offsetof(efi_guid_t, b),
	LUPOS_BOOT_EFI_CONFIG64_SIZE = sizeof(efi_config_table_64_t),
	LUPOS_BOOT_EFI_CONFIG64_ALIGN = __alignof__(efi_config_table_64_t),
	LUPOS_BOOT_EFI_CONFIG64_TABLE_OFFSET = __builtin_offsetof(efi_config_table_64_t, table),
	LUPOS_BOOT_EFI_CONFIG32_SIZE = sizeof(efi_config_table_32_t),
	LUPOS_BOOT_EFI_CONFIG32_ALIGN = __alignof__(efi_config_table_32_t),
	LUPOS_BOOT_EFI_CONFIG32_TABLE_OFFSET = __builtin_offsetof(efi_config_table_32_t, table),
	LUPOS_BOOT_EFI_SYSTEM64_SIZE = sizeof(efi_system_table_64_t),
	LUPOS_BOOT_EFI_SYSTEM64_NR_OFFSET = __builtin_offsetof(efi_system_table_64_t, nr_tables),
	LUPOS_BOOT_EFI_SYSTEM64_TABLES_OFFSET = __builtin_offsetof(efi_system_table_64_t, tables),
	LUPOS_BOOT_EFI_SYSTEM32_SIZE = sizeof(efi_system_table_32_t),
	LUPOS_BOOT_EFI_SYSTEM32_NR_OFFSET = __builtin_offsetof(efi_system_table_32_t, nr_tables),
	LUPOS_BOOT_EFI_SYSTEM32_TABLES_OFFSET = __builtin_offsetof(efi_system_table_32_t, tables),
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
