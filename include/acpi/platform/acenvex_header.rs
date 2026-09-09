/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acenvex.h - Extra host and compiler configuration
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * [Begin] no source code translation
 *
 * Extra host configuration files. All ACPICA headers are included before
 * including these files.
 *
 * The following C conditional includes are intentionally preserved as
 * configuration intent. The corresponding platform/compiler definitions are
 * supplied by other translation units.
 *
 * #if defined(_LINUX) || defined(__linux__)
 *     include <acpi/platform/aclinuxex.h>
 * #elif defined(__DragonFly__)
 *     include "acdragonflyex.h"
 * #elif defined(_AED_EFI) || defined(_GNU_EFI) || defined(_EDK2_EFI)
 *     include "acefiex.h"
 * #endif
 *
 * #if defined(__GNUC__)
 *     include "acgccex.h"
 * #elif defined(_MSC_VER)
 *     include "acmsvcex.h"
 * #endif
 *
 * [End] no source code translation
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
