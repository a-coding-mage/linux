/*
 * sysmem-related prototypes.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 Cadence Design Systems Inc.
 */

// Dependency corresponding to: #include <linux/memblock.h>

extern "C" {
    pub fn bootmem_init();
    pub fn zones_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
