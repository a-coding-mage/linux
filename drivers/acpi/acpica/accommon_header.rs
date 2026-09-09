/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: accommon.h - Common include files for generation of ACPICA source
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Common set of dependencies for all ACPICA source files.
 * The original C header centralizes these includes because they are shared
 * throughout the source tree. They are supplied by other translated files.
 * The order of these dependencies is important.
 */

// Global configuration constants: acpi/acconfig.h
// C macros: acmacros.h
// Internal data types: aclocal.h
// ACPI internal object: acobject.h
// Common structures: acstruct.h
// All global variables: acglobal.h
// Hardware defines and interfaces: achware.h
// Utility interfaces: acutils.h

/* The C library interfaces are included only when ACPI_USE_SYSTEM_CLIBRARY
 * is not defined. Preserve that build-time condition for the Rust consumer. */
#[cfg(not(feature = "ACPI_USE_SYSTEM_CLIBRARY"))]
// C library interfaces: acclib.h
pub mod acclib_dependency {
    // Dependency declarations are supplied by the translated ACPICA sources.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
