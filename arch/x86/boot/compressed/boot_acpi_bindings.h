/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_ACPI_BINDINGS_H
#define LUPOS_BOOT_ACPI_BINDINGS_H

/* Match acpi.c's configured early-boot type and constant environment. */
#define BOOT_CTYPE_H
#include "misc.h"
#include <linux/numa.h>

/* Layouts and constants only: the ACPI parser and all state live in Rust. */
enum {
	LUPOS_BOOT_ACPI_MAX_NUMNODES = MAX_NUMNODES,
	LUPOS_BOOT_ACPI_RSDT_ENTRY_SIZE = ACPI_RSDT_ENTRY_SIZE,
	LUPOS_BOOT_ACPI_XSDT_ENTRY_SIZE = ACPI_XSDT_ENTRY_SIZE,
};

#endif /* LUPOS_BOOT_ACPI_BINDINGS_H */
