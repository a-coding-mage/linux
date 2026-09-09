// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are required
// for `execmem_info`, `EXECMEM_DEFAULT`, `MODULES_VADDR`, `MODULES_END`, and
// `PAGE_KERNEL`.

// The C declaration is `static ... __ro_after_init`; the storage remains
// private and is initialized by execmem_arch_setup before becoming read-only.
static mut execmem_info: execmem_info = unsafe { core::mem::zeroed() };

// C: struct execmem_info __init *execmem_arch_setup(void)
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
	// C compound-literal initialization of the complete object.  The remaining
	// ranges are zero-initialized, as they are for the C designated initializer.
	execmem_info = core::mem::zeroed();
	execmem_info.ranges[EXECMEM_DEFAULT] = execmem_range {
		start: MODULES_VADDR,
		end: MODULES_END,
		pgprot: PAGE_KERNEL,
		alignment: 1,
	};

	&raw mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
