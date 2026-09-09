// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit:
// linux/mm.h, linux/io.h

static mut ioremap_prot_hook: ioremap_prot_hook_t = None;

pub unsafe fn arm64_ioremap_prot_hook_register(hook: ioremap_prot_hook_t) -> c_int {
	if WARN_ON(ioremap_prot_hook.is_some()) {
		return -EBUSY;
	}

	ioremap_prot_hook = hook;
	0
}

pub unsafe fn __ioremap_prot(
	phys_addr: phys_addr_t,
	size: size_t,
	mut pgprot: pgprot_t,
) -> *mut core::ffi::c_void {
	let last_addr: unsigned_long = phys_addr.wrapping_add(size).wrapping_sub(1);

	/* Don't allow outside PHYS_MASK */
	if last_addr & !PHYS_MASK != 0 {
		return core::ptr::null_mut();
	}

	/* Don't allow RAM to be mapped. */
	if WARN_ONCE(
		pfn_is_map_memory(__phys_to_pfn(phys_addr)),
		"ioremap attempted on RAM pfn\n",
	) {
		return core::ptr::null_mut();
	}

	/*
	 * If a hook is registered (e.g. for confidential computing
	 * purposes), call that now and barf if it fails.
	 */
	if unlikely(ioremap_prot_hook.is_some())
		&& WARN_ON((ioremap_prot_hook.unwrap())(phys_addr, size, &mut pgprot))
	{
		return core::ptr::null_mut();
	}

	generic_ioremap_prot(phys_addr, size, pgprot)
}

// EXPORT_SYMBOL(__ioremap_prot);

/*
 * Must be called after early_fixmap_init
 */
pub unsafe fn early_ioremap_init() {
	early_ioremap_setup();
}

pub unsafe fn arch_memremap_can_ram_remap(
	offset: resource_size_t,
	size: size_t,
	flags: unsigned_long,
) -> bool {
	let pfn: unsigned_long = PHYS_PFN(offset);

	pfn_is_map_memory(pfn)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
