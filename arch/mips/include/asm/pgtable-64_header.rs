/*
 * Rust translation of asm/pgtable-64.h.  C includes and build-time
 * preprocessor conditions are represented by comments/cfg-style sections;
 * referenced kernel types and symbols are supplied by other headers.
 */

/* PAGE_SHIFT, PAGE_SIZE, TASK_SIZE64, address constants, table types and
 * helper functions are external dependencies from the original headers. */

#[cfg(__PAGETABLE_PMD_FOLDED)]
pub const PGDIR_SHIFT: usize = PAGE_SHIFT + PAGE_SHIFT - 3;

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - 3);
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

#[cfg(all(not(__PAGETABLE_PMD_FOLDED), __PAGETABLE_PUD_FOLDED))]
pub const PGDIR_SHIFT: usize = PMD_SHIFT + (PAGE_SHIFT + PMD_TABLE_ORDER - 3);

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const PUD_SHIFT: usize = PMD_SHIFT + (PAGE_SHIFT + PMD_TABLE_ORDER - 3);
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const PGDIR_SHIFT: usize = PUD_SHIFT + (PAGE_SHIFT + PUD_TABLE_ORDER - 3);

pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/* Table orders selected by the original CONFIG_PAGE_SIZE_* and VA config. */
/* CONFIG_PAGE_SIZE_4KB: PGD_TABLE_ORDER=1 (or 0 with CONFIG_MIPS_VA_BITS_48),
 * PUD_TABLE_ORDER=0 (or aieeee_attempt_to_allocate_pud), PMD_TABLE_ORDER=0. */
/* CONFIG_PAGE_SIZE_8KB: PGD_TABLE_ORDER=0, PUD_TABLE_ORDER=
 * aieeee_attempt_to_allocate_pud, PMD_TABLE_ORDER=0. */
/* CONFIG_PAGE_SIZE_16KB: PGD_TABLE_ORDER=1 (or 0), PUD_TABLE_ORDER=
 * aieeee_attempt_to_allocate_pud, PMD_TABLE_ORDER=0. */
/* CONFIG_PAGE_SIZE_32KB: PGD_TABLE_ORDER=0, PUD_TABLE_ORDER=
 * aieeee_attempt_to_allocate_pud, PMD_TABLE_ORDER=0. */
/* CONFIG_PAGE_SIZE_64KB: PGD_TABLE_ORDER=0, PUD_TABLE_ORDER=
 * aieeee_attempt_to_allocate_pud, PMD_TABLE_ORDER=0 (or
 * aieeee_attempt_to_allocate_pmd). */

pub const PTRS_PER_PGD: usize = (PAGE_SIZE << PGD_TABLE_ORDER) / core::mem::size_of::<pgd_t>();
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const PTRS_PER_PUD: usize = (PAGE_SIZE << PUD_TABLE_ORDER) / core::mem::size_of::<pud_t>();
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub const PTRS_PER_PMD: usize = (PAGE_SIZE << PMD_TABLE_ORDER) / core::mem::size_of::<pmd_t>();
pub const PTRS_PER_PTE: usize = PAGE_SIZE / core::mem::size_of::<pte_t>();
pub const USER_PTRS_PER_PGD: usize = if TASK_SIZE64 / PGDIR_SIZE != 0 { TASK_SIZE64 / PGDIR_SIZE } else { 1 };

pub const VMALLOC_START: usize = MAP_BASE + (2 * PAGE_SIZE);
/* VMALLOC_END = MAP_BASE + min(PTRS_PER_PGD * PTRS_PER_PUD * PTRS_PER_PMD *
 * PTRS_PER_PTE * PAGE_SIZE, 1UL << cpu_vmbits) - (1UL << 32). */

#[cfg(all(CONFIG_MODULES, KBUILD_64BIT_SYM32))]
/* Original condition additionally requires VMALLOC_START != CKSSEG. */
pub const MODULES_VADDR: usize = CKSSEG;
#[cfg(all(CONFIG_MODULES, KBUILD_64BIT_SYM32))]
pub const MODULES_END: usize = FIXADDR_START - 2 * PAGE_SIZE;

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t { pub pud: usize }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub const fn __pud(x: usize) -> pud_t { pud_t { pud: x } }

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
extern "C" {
    pub static mut invalid_pud_table: [pud_t; PTRS_PER_PUD];
}
extern "C" { pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE]; }

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t { pub pmd: usize }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub const fn __pmd(x: usize) -> pmd_t { pmd_t { pmd: x } }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
extern "C" { pub static mut invalid_pmd_table: [pmd_t; PTRS_PER_PMD]; }

#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_none(p4d: p4d_t) -> i32 { (p4d_val(p4d) == invalid_pud_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_bad(p4d: p4d_t) -> i32 { ((p4d_val(p4d) & !PAGE_MASK) != 0) as i32 }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_present(p4d: p4d_t) -> i32 { (p4d_val(p4d) != invalid_pud_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_clear(p4dp: *mut p4d_t) { p4d_val(*p4dp) = invalid_pud_table.as_ptr() as usize; }
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub unsafe fn p4d_pgtable(p4d: p4d_t) -> *mut pud_t { p4d_val(p4d) as *mut pud_t }

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_none(pmd: pmd_t) -> i32 { (pmd.pmd == invalid_pte_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_bad(pmd: pmd_t) -> i32 { ((pmd.pmd & !PAGE_MASK) != 0) as i32 }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { (pmd.pmd != invalid_pte_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { (*pmdp).pmd = invalid_pte_table.as_ptr() as usize; }

#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pud_none(pud: pud_t) -> i32 { (pud.pud == invalid_pmd_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub fn pud_bad(pud: pud_t) -> usize { pud.pud & !PAGE_MASK }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pud_present(pud: pud_t) -> i32 { (pud.pud != invalid_pmd_table.as_ptr() as usize) as i32 }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub unsafe fn pud_clear(pudp: *mut pud_t) { (*pudp).pud = invalid_pmd_table.as_ptr() as usize; }

pub unsafe fn mk_swap_pte(type_: usize, offset: usize) -> pte_t {
    let mut pte: pte_t = core::mem::zeroed();
    pte_val(pte) = ((type_ & 0x7f) << 16) | (offset << 24); pte
}
pub const _PAGE_SWP_EXCLUSIVE: usize = 1 << 23;

extern "C" {
    pub fn pgd_init(addr: *mut core::ffi::c_void);
    pub fn pud_init(addr: *mut core::ffi::c_void);
    pub fn pmd_init(addr: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
