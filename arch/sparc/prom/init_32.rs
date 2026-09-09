// SPDX-License-Identifier: GPL-2.0
/*
 * init.c:  Initialize internal variables used by the PROM
 *          library functions.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// linux/kernel.h, linux/init.h, linux/module.h,
// asm/openprom.h, and asm/oplib.h provide the declarations used here.

static mut romvec: *mut linux_romvec = core::ptr::null_mut();
// EXPORT_SYMBOL(romvec);

static mut prom_vers: prom_major_version = PROM_V0;
static mut prom_rev: u32 = 0;
static mut prom_prev: u32 = 0;

/* The root node of the prom device tree. */
static mut prom_root_node: phandle = 0;
// EXPORT_SYMBOL(prom_root_node);

/* Pointer to the device tree operations structure. */
static mut prom_nodeops: *mut linux_nodeops = core::ptr::null_mut();

/* You must call prom_init() before you attempt to use any of the
 * routines in the prom library.
 * It gets passed the pointer to the PROM vector.
 */

pub unsafe extern "C" fn prom_init(rp: *mut linux_romvec) {
	romvec = rp;

	match (*romvec).pv_romvers {
		0 => {
			prom_vers = PROM_V0;
		},
		2 => {
			prom_vers = PROM_V2;
		},
		3 => {
			prom_vers = PROM_V3;
		},
		_ => {
			prom_printf(c"PROMLIB: Bad PROM version %d\n", (*romvec).pv_romvers);
			prom_halt();
		},
	}

	prom_rev = (*romvec).pv_plugin_revision;
	prom_prev = (*romvec).pv_printrev;
	prom_nodeops = (*romvec).pv_nodeops;

	prom_root_node = prom_getsibling(0);
	if prom_root_node == 0 || (prom_root_node as i32) == -1 {
		prom_halt();
	}

	if (prom_nodeops as usize) == 0 || (prom_nodeops as usize) == usize::MAX {
		prom_halt();
	}

	prom_meminit();

	prom_ranges_init();

	printk(
		c"PROMLIB: Sun Boot Prom Version %d Revision %d\n",
		(*romvec).pv_romvers,
		prom_rev,
	);

	/* Initialization successful. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
