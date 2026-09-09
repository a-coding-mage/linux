// Translated from x86/include/asm/pgtable_32_areas.h.
// Dependency: asm/cpu_entry_area.h and the symbols/constants it supplies.

/*
 * Just any arbitrary offset to the start of the vmalloc VM area: the
 * current 8MB value just means that there will be a 8MB "hole" after the
 * physical memory until the kernel virtual memory starts.  That means that
 * any out-of-bounds memory accesses will hopefully be caught.
 * The vmalloc() routines leaves a hole of 4kB between each vmalloced
 * area for the same reason. ;)
 */
pub const VMALLOC_OFFSET: usize = 8 * 1024 * 1024;

// Set once high_memory is set.
extern "C" {
    pub static mut __vmalloc_start_set: bool;
}

// The following names are supplied by other translation units/headers:
// high_memory, CONFIG_X86_PAE, NR_CPUS, DIV_ROUND_UP, cpu_entry_area,
// PAGE_SIZE, FIXADDR_TOT_START, PMD_MASK, PMD_SIZE, PAGE_OFFSET, and
// __VMALLOC_RESERVE.

pub const VMALLOC_START: usize = (high_memory as usize) + VMALLOC_OFFSET;

// CONFIG_X86_PAE selects LAST_PKMAP = 512; otherwise it is 1024.
pub const LAST_PKMAP: usize = 1024;

pub const CPU_ENTRY_AREA_PAGES: usize =
    NR_CPUS * ((core::mem::size_of::<cpu_entry_area>() + PAGE_SIZE - 1) / PAGE_SIZE);

/* The +1 is for the readonly IDT page: */
pub const CPU_ENTRY_AREA_BASE: usize =
    (FIXADDR_TOT_START - PAGE_SIZE * (CPU_ENTRY_AREA_PAGES + 1)) & PMD_MASK;

pub const LDT_BASE_ADDR: usize = (CPU_ENTRY_AREA_BASE - PAGE_SIZE) & PMD_MASK;

pub const LDT_END_ADDR: usize = LDT_BASE_ADDR + PMD_SIZE;

pub const PKMAP_BASE: usize = (LDT_BASE_ADDR - PAGE_SIZE) & PMD_MASK;

// CONFIG_HIGHMEM selects the PKMAP_BASE form; otherwise the LDT_BASE_ADDR form.
pub const VMALLOC_END: usize = LDT_BASE_ADDR - 2 * PAGE_SIZE;

pub const MODULES_VADDR: usize = VMALLOC_START;
pub const MODULES_END: usize = VMALLOC_END;
pub const MODULES_LEN: usize = MODULES_VADDR - MODULES_END;

pub const MAXMEM: usize = VMALLOC_END - PAGE_OFFSET - __VMALLOC_RESERVE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
