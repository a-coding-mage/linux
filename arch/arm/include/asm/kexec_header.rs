/* SPDX-License-Identifier: GPL-2.0 */

/* Maximum physical address we can use pages from. */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can reach in physical address mode. */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can use for the control code buffer. */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

pub const KEXEC_ARCH: _ = KEXEC_ARCH_ARM;

pub const KEXEC_ARM_ATAGS_OFFSET: usize = 0x1000;
pub const KEXEC_ARM_ZIMAGE_OFFSET: usize = 0x8000;

pub const ARCH_HAS_KIMAGE_ARCH: bool = true;

#[repr(C)]
pub struct kimage_arch {
	pub kernel_r2: u32,
}

/**
 * crash_setup_regs() - save registers for the panic kernel
 * @newregs: registers are saved here
 * @oldregs: registers to be saved (may be NULL)
 *
 * Function copies machine registers from @oldregs to @newregs. If @oldregs is
 * NULL then current registers are stored there.
 */
pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
	if !oldregs.is_null() {
		core::ptr::copy_nonoverlapping(
			oldregs as *const u8,
			newregs as *mut u8,
			core::mem::size_of::<pt_regs>(),
		);
	} else {
		/*
		 * The original ARM inline assembly stores r0-r12, sp, lr, pc, and
		 * cpsr into newregs. Its register constraints and ARM assembly syntax
		 * have no file-local Rust equivalent; preserve the required operation
		 * as an explicit target-specific translation point.
		 */
		// TODO: translate the ARM register capture using the target's asm support.
	}
}

pub unsafe fn phys_to_boot_phys(phys: phys_addr_t) -> c_ulong {
	phys_to_idmap(phys)
}

pub unsafe fn boot_phys_to_phys(entry: c_ulong) -> phys_addr_t {
	idmap_to_phys(entry)
}

pub unsafe fn page_to_boot_pfn(page: *mut page) -> c_ulong {
	page_to_pfn(page) + (arch_phys_to_idmap_offset >> PAGE_SHIFT)
}

pub unsafe fn boot_pfn_to_page(boot_pfn: c_ulong) -> *mut page {
	pfn_to_page(boot_pfn - (arch_phys_to_idmap_offset >> PAGE_SHIFT))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
