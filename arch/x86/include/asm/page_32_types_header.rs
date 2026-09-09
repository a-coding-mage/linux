/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This handles the memory map.
 *
 * A __PAGE_OFFSET of 0xC0000000 means that the kernel has
 * a virtual address space of one gigabyte, which limits the
 * amount of physical memory you can use to about 950MB.
 *
 * If you want more physical memory than this then see the CONFIG_VMSPLIT_2G
 * and CONFIG_HIGHMEM4G options in the kernel configuration.
 */
pub const __PAGE_OFFSET_BASE: usize = CONFIG_PAGE_OFFSET;
pub const __PAGE_OFFSET: usize = __PAGE_OFFSET_BASE;

pub const __START_KERNEL_map: usize = __PAGE_OFFSET;

pub const THREAD_SIZE_ORDER: usize = 1;
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

pub const IRQ_STACK_SIZE: usize = THREAD_SIZE;

pub const N_EXCEPTION_STACKS: usize = 1;

/*
 * This is beyond the 44 bit limit imposed by the 32bit long pfns,
 * but we need the full mask to make sure inverted PROT_NONE
 * entries have all the host bits set in a guest.
 * The real limit is still 44 bits.
 */
#[cfg(CONFIG_X86_PAE)]
pub const __PHYSICAL_MASK_SHIFT: usize = 52;
#[cfg(CONFIG_X86_PAE)]
pub const __VIRTUAL_MASK_SHIFT: usize = 32;

/* !CONFIG_X86_PAE */
#[cfg(not(CONFIG_X86_PAE))]
pub const __PHYSICAL_MASK_SHIFT: usize = 32;
#[cfg(not(CONFIG_X86_PAE))]
pub const __VIRTUAL_MASK_SHIFT: usize = 32;

/*
 * User space process size: 3GB (default).
 */
pub const IA32_PAGE_OFFSET: usize = __PAGE_OFFSET;
pub const TASK_SIZE: usize = __PAGE_OFFSET;
pub const TASK_SIZE_LOW: usize = TASK_SIZE;
pub const TASK_SIZE_MAX: usize = TASK_SIZE;
pub const DEFAULT_MAP_WINDOW: usize = TASK_SIZE;
pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

/*
 * In spite of the name, KERNEL_IMAGE_SIZE is a limit on the maximum virtual
 * address for the kernel image, rather than the limit on the size itself. On
 * 32-bit, this is not a strict limit, but this value is used to limit the
 * link-time virtual address range of the kernel, and by KASLR to limit the
 * randomized address from which the kernel is executed. A relocatable kernel
 * can be loaded somewhat higher than KERNEL_IMAGE_SIZE as long as enough space
 * remains for the vmalloc area.
 */
pub const KERNEL_IMAGE_SIZE: usize = 512 * 1024 * 1024;

/* The following declarations are omitted when building as an assembler. */
unsafe extern "C" {
    pub static mut __VMALLOC_RESERVE: u32;
    pub static mut sysctl_legacy_va_layout: i32;

    pub fn find_low_pfn_range();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
