/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes <asm/kaslr.h> for non-assembler builds. */

/* CONFIG_KASAN controls this value at build time in the original source. */
#[cfg(feature = "CONFIG_KASAN")]
pub const KASAN_STACK_ORDER: usize = 1;
#[cfg(not(feature = "CONFIG_KASAN"))]
pub const KASAN_STACK_ORDER: usize = 0;

pub const THREAD_SIZE_ORDER: usize = 2 + KASAN_STACK_ORDER;
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

pub const EXCEPTION_STACK_ORDER: usize = 1 + KASAN_STACK_ORDER;
pub const EXCEPTION_STKSZ: usize = PAGE_SIZE << EXCEPTION_STACK_ORDER;

pub const IRQ_STACK_ORDER: usize = 2 + KASAN_STACK_ORDER;
pub const IRQ_STACK_SIZE: usize = PAGE_SIZE << IRQ_STACK_ORDER;

/* The index for the tss.ist[] array. The hardware limit is 7 entries. */
pub const IST_INDEX_DF: usize = 0;
pub const IST_INDEX_NMI: usize = 1;
pub const IST_INDEX_DB: usize = 2;
pub const IST_INDEX_MCE: usize = 3;
pub const IST_INDEX_VC: usize = 4;

/*
 * Set __PAGE_OFFSET to the most negative possible address +
 * PGDIR_SIZE*17 (pgd slot 273).
 *
 * The gap is to allow a space for LDT remap for PTI (1 pgd slot) and space for
 * a hypervisor (16 slots). Choosing 16 slots for a hypervisor is arbitrary,
 * but it's what Xen requires.
 */
pub const __PAGE_OFFSET_BASE_L5: u64 = 0xff11000000000000;
pub const __PAGE_OFFSET_BASE_L4: u64 = 0xffff888000000000;

#[macro_export]
macro_rules! __PAGE_OFFSET {
    () => { page_offset_base };
}

pub const __START_KERNEL_map: u64 = 0xffffffff80000000;

/* See Documentation/arch/x86/x86_64/mm.rst for a description of the memory map. */

pub const __PHYSICAL_MASK_SHIFT: usize = 52;

#[macro_export]
macro_rules! __VIRTUAL_MASK_SHIFT {
    () => { if pgtable_l5_enabled() { 56 } else { 47 } };
}

#[macro_export]
macro_rules! TASK_SIZE_MAX {
    () => { task_size_max() };
}

#[macro_export]
macro_rules! DEFAULT_MAP_WINDOW {
    () => { ((1usize << 47) - PAGE_SIZE) };
}

/* This decides where the kernel will search for a free chunk of vm
 * space during mmap's.
 */
#[macro_export]
macro_rules! IA32_PAGE_OFFSET {
    () => {
        if (current.personality & ADDR_LIMIT_3GB) != 0 {
            0xc0000000
        } else {
            0xffffe000
        }
    };
}

#[macro_export]
macro_rules! TASK_SIZE_LOW {
    () => { if test_thread_flag(TIF_ADDR32) { IA32_PAGE_OFFSET!() } else { DEFAULT_MAP_WINDOW!() } };
}

#[macro_export]
macro_rules! TASK_SIZE {
    () => { if test_thread_flag(TIF_ADDR32) { IA32_PAGE_OFFSET!() } else { TASK_SIZE_MAX!() } };
}

#[macro_export]
macro_rules! TASK_SIZE_OF {
    ($child:expr) => { if test_tsk_thread_flag($child, TIF_ADDR32) { IA32_PAGE_OFFSET!() } else { TASK_SIZE_MAX!() } };
}

#[macro_export]
macro_rules! STACK_TOP {
    () => { TASK_SIZE_LOW!() };
}

#[macro_export]
macro_rules! STACK_TOP_MAX {
    () => { TASK_SIZE_MAX!() };
}

/*
 * In spite of the name, KERNEL_IMAGE_SIZE is a limit on the maximum virtual
 * address for the kernel image, rather than the limit on the size itself.
 * This can be at most 1 GiB, due to the fixmap living in the next 1 GiB (see
 * level2_kernel_pgt in arch/x86/kernel/head_64.S).
 *
 * On KASLR use 1 GiB by default, leaving 1 GiB for modules once the
 * page tables are fully set up.
 *
 * If KASLR is disabled we can shrink it to 0.5 GiB and increase the size
 * of the modules area to 1.5 GiB.
 */
#[cfg(feature = "CONFIG_RANDOMIZE_BASE")]
pub const KERNEL_IMAGE_SIZE: usize = 1024 * 1024 * 1024;
#[cfg(not(feature = "CONFIG_RANDOMIZE_BASE"))]
pub const KERNEL_IMAGE_SIZE: usize = 512 * 1024 * 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
