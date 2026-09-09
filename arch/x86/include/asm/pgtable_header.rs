/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust translation of x86/include/asm/pgtable.h.  Types and symbols
// supplied by the surrounding kernel translation are intentionally external.

#[allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
pub mod pgtable {
    extern "C" {
        pub static mut boot_cpu_data: BootCpuData;
        pub static mut pgd_lock: spinlock_t;
        pub static mut pgd_list: list_head;
        pub static mut early_pmd_flags: pmdval_t;
        pub static mut direct_gbpages: i32;
        pub static mut x86_platform: X86Platform;
    }

    pub type u16 = ::core::primitive::u16; pub type u32 = ::core::primitive::u32;
    pub type u64 = ::core::primitive::u64; pub type ulong = ::core::primitive::usize;
    pub type phys_addr_t = u64; pub type pteval_t = u64; pub type pmdval_t = u64;
    pub type pudval_t = u64; pub type pgprotval_t = u64;
    #[repr(C)] pub struct BootCpuData { pub x86: i32 }
    #[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
    #[repr(C)] pub struct list_head { _private: [u8; 0] }
    #[repr(C)] pub struct X86Platform { pub is_untracked_pat_range: unsafe extern "C" fn(u64, u64) -> bool }
    #[repr(C)] pub struct seq_file { _private: [u8; 0] }
    #[repr(C)] pub struct mm_struct { pub tlb_flush_pending: atomic_t }
    #[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct }
    #[repr(C)] pub struct vm_fault { _private: [u8; 0] }
    #[repr(C)] pub struct atomic_t { _private: [u8; 0] }
    #[repr(C)] #[derive(Copy, Clone)] pub struct pte_t { pub pte: u64 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct pmd_t { pub pmd: u64 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct pud_t { pub pud: u64 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct p4d_t { pub p4d: u64 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct pgd_t { pub pgd: u64 }
    #[repr(C)] #[derive(Copy, Clone)] pub struct pgprot_t { pub pgprot: u64 }
    #[repr(C)] pub enum page_cache_mode { _PAGE_CACHE_MODE_UC_MINUS, _PAGE_CACHE_MODE_WC, _PAGE_CACHE_MODE_WT, _PAGE_CACHE_MODE_WB }
    #[repr(C)] pub enum pg_level { PG_LEVEL_4K, PG_LEVEL_2M, PG_LEVEL_1G, PG_LEVEL_512G, PG_LEVEL_256T }

    extern "C" {
        pub static early_top_pgt: [pgd_t; PTRS_PER_PGD];
        pub fn native_pte_val(x: pte_t) -> pteval_t; pub fn native_make_pte(x: pteval_t) -> pte_t;
        pub fn native_pmd_val(x: pmd_t) -> pmdval_t; pub fn native_make_pmd(x: pmdval_t) -> pmd_t;
        pub fn native_pud_val(x: pud_t) -> pudval_t; pub fn native_make_pud(x: pudval_t) -> pud_t;
        pub fn native_pgd_val(x: pgd_t) -> u64; pub fn native_make_pgd(x: u64) -> pgd_t;
        pub fn native_p4d_val(x: p4d_t) -> u64; pub fn native_make_p4d(x: u64) -> p4d_t;
        pub fn pte_flags(x: pte_t) -> u64; pub fn pmd_flags(x: pmd_t) -> u64; pub fn pud_flags(x: pud_t) -> u64;
        pub fn pte_val(x: pte_t) -> u64; pub fn pmd_val(x: pmd_t) -> u64; pub fn pud_val(x: pud_t) -> u64;
        pub fn __pte(x: u64) -> pte_t; pub fn __pmd(x: u64) -> pmd_t; pub fn __pud(x: u64) -> pud_t; pub fn __pgprot(x: u64) -> pgprot_t;
        pub fn pgprot_val(x: pgprot_t) -> u64; pub fn protnone_mask(x: u64) -> u64;
        pub fn pmd_pfn_mask(x: pmd_t) -> u64; pub fn pud_pfn_mask(x: pud_t) -> u64; pub fn p4d_pfn_mask(x: p4d_t) -> u64;
        pub fn cpu_feature_enabled(x: u64) -> bool; pub fn boot_cpu_has(x: u64) -> bool; pub fn boot_cpu_has_bug(x: u64) -> bool;
        pub fn pgtable_l5_enabled() -> bool; pub fn cc_mkenc(x: u64) -> u64; pub fn cc_mkdec(x: u64) -> u64;
        pub fn pfn_to_page(x: usize) -> *mut u8; pub fn __va(x: u64) -> *mut u8;
        pub fn read_pkru() -> u32; pub fn __pkru_allows_read(x: u32, p: u16) -> bool; pub fn __pkru_allows_write(x: u32, p: u16) -> bool;
        pub fn WARN_ON_ONCE(x: bool) -> bool; pub fn atomic_read(x: *const atomic_t) -> i32;
        pub fn native_pmd_clear(x: *mut pmd_t); pub fn native_pud_clear(x: *mut pud_t); pub fn native_pte_clear(mm: *mut mm_struct, a: usize, p: *mut pte_t);
        pub fn native_ptep_get_and_clear(p: *mut pte_t) -> pte_t; pub fn native_pmdp_get_and_clear(p: *mut pmd_t) -> pmd_t; pub fn native_pudp_get_and_clear(p: *mut pud_t) -> pud_t;
        pub fn set_pte(p: *mut pte_t, v: pte_t); pub fn set_pmd(p: *mut pmd_t, v: pmd_t); pub fn set_pud(p: *mut pud_t, v: pud_t); pub fn set_p4d(p: *mut p4d_t, v: p4d_t); pub fn set_pgd(p: *mut pgd_t, v: pgd_t);
        pub fn page_table_check_pte_clear(mm: *mut mm_struct, a: usize, p: pte_t); pub fn page_table_check_pmd_clear(mm: *mut mm_struct, a: usize, p: pmd_t); pub fn page_table_check_pud_clear(mm: *mut mm_struct, a: usize, p: pud_t);
        pub fn page_table_check_pmd_set(mm: *mut mm_struct,a:usize,p:*mut pmd_t,v:pmd_t); pub fn page_table_check_pud_set(mm:*mut mm_struct,a:usize,p:*mut pud_t,v:pud_t);
        pub fn flip_protnone_guard(a:u64,b:u64,c:u64)->u64; pub fn check_pgprot(x:pgprot_t)->u64;
    }

    pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;
    pub unsafe fn pmd_set_flags(p: pmd_t, set: u64) -> pmd_t { native_make_pmd(native_pmd_val(p) | set) }
    pub unsafe fn pmd_clear_flags(p: pmd_t, clear: u64) -> pmd_t { native_make_pmd(native_pmd_val(p) & !clear) }
    pub unsafe fn pud_set_flags(p: pud_t, set: u64) -> pud_t { native_make_pud(native_pud_val(p) | set) }
    pub unsafe fn pud_clear_flags(p: pud_t, clear: u64) -> pud_t { native_make_pud(native_pud_val(p) & !clear) }
    pub unsafe fn pte_set_flags(p: pte_t, set: u64) -> pte_t { native_make_pte(native_pte_val(p) | set) }
    pub unsafe fn pte_clear_flags(p: pte_t, clear: u64) -> pte_t { native_make_pte(native_pte_val(p) & !clear) }
    pub unsafe fn pte_dirty(p: pte_t)->bool { pte_flags(p) & _PAGE_DIRTY_BITS != 0 }
    pub unsafe fn pte_shstk(p: pte_t)->bool { cpu_feature_enabled(X86_FEATURE_SHSTK) && pte_flags(p)&(_PAGE_RW|_PAGE_DIRTY)==_PAGE_DIRTY }
    pub unsafe fn pte_young(p: pte_t)->i32 { (pte_flags(p)&_PAGE_ACCESSED) as i32 }
    pub unsafe fn pte_decrypted(p:pte_t)->bool { cc_mkdec(pte_val(p))==pte_val(p) }
    pub unsafe fn pmd_dirty(p:pmd_t)->bool { pmd_flags(p)&_PAGE_DIRTY_BITS != 0 }
    pub unsafe fn pmd_shstk(p:pmd_t)->bool { cpu_feature_enabled(X86_FEATURE_SHSTK) && pmd_flags(p)&(_PAGE_RW|_PAGE_DIRTY|_PAGE_PSE)==(_PAGE_DIRTY|_PAGE_PSE) }
    pub unsafe fn pmd_young(p:pmd_t)->i32 {(pmd_flags(p)&_PAGE_ACCESSED) as i32}
    pub unsafe fn pud_dirty(p:pud_t)->bool {pud_flags(p)&_PAGE_DIRTY_BITS != 0}
    pub unsafe fn pud_young(p:pud_t)->i32 {(pud_flags(p)&_PAGE_ACCESSED) as i32}
    pub unsafe fn pud_shstk(p:pud_t)->bool {cpu_feature_enabled(X86_FEATURE_SHSTK)&&pud_flags(p)&(_PAGE_RW|_PAGE_DIRTY|_PAGE_PSE)==(_PAGE_DIRTY|_PAGE_PSE)}
    pub unsafe fn pte_write(p:pte_t)->i32 {(if pte_flags(p)&_PAGE_RW !=0 || pte_shstk(p){1}else{0})}
    pub unsafe fn pmd_write(p:pmd_t)->i32 {(if pmd_flags(p)&_PAGE_RW !=0 || pmd_shstk(p){1}else{0})}
    pub unsafe fn pud_write(p:pud_t)->i32 {(pud_flags(p)&_PAGE_RW) as i32}
    pub unsafe fn pte_huge(p:pte_t)->i32 {(pte_flags(p)&_PAGE_PSE) as i32}
    pub unsafe fn pte_global(p:pte_t)->i32 {(pte_flags(p)&_PAGE_GLOBAL) as i32}
    pub unsafe fn pte_exec(p:pte_t)->i32 {if pte_flags(p)&_PAGE_NX==0{1}else{0}}
    pub unsafe fn pte_special(p:pte_t)->i32 {(pte_flags(p)&_PAGE_SPECIAL) as i32}
    pub unsafe fn pte_pfn(p:pte_t)->usize {let mut v=pte_val(p);v^=protnone_mask(v);((v&PTE_PFN_MASK)>>PAGE_SHIFT) as usize}
    pub unsafe fn pmd_pfn(p:pmd_t)->usize {let mut v=pmd_val(p);v^=protnone_mask(v);((v&pmd_pfn_mask(p))>>PAGE_SHIFT) as usize}
    pub unsafe fn pud_pfn(p:pud_t)->usize {let mut v=pud_val(p);v^=protnone_mask(v);((v&pud_pfn_mask(p))>>PAGE_SHIFT) as usize}
    pub unsafe fn p4d_pfn(p:p4d_t)->usize {((p4d_val(p)&p4d_pfn_mask(p))>>PAGE_SHIFT) as usize}
    pub unsafe fn pgd_pfn(p:pgd_t)->usize {((native_pgd_val(p)&PTE_PFN_MASK)>>PAGE_SHIFT) as usize}
    pub unsafe fn pmd_leaf(p:pmd_t)->bool {pmd_flags(p)&_PAGE_PSE !=0}
    pub unsafe fn pte_mkclean(p:pte_t)->pte_t {pte_clear_flags(p,_PAGE_DIRTY_BITS)}
    pub unsafe fn pte_mkold(p:pte_t)->pte_t {pte_clear_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pte_mkexec(p:pte_t)->pte_t {pte_clear_flags(p,_PAGE_NX)}
    pub unsafe fn pte_mkyoung(p:pte_t)->pte_t {pte_set_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pte_mkhuge(p:pte_t)->pte_t {pte_set_flags(p,_PAGE_PSE)}
    pub unsafe fn pte_clrhuge(p:pte_t)->pte_t {pte_clear_flags(p,_PAGE_PSE)}
    pub unsafe fn pte_mkglobal(p:pte_t)->pte_t {pte_set_flags(p,_PAGE_GLOBAL)}
    pub unsafe fn pte_clrglobal(p:pte_t)->pte_t {pte_clear_flags(p,_PAGE_GLOBAL)}
    pub unsafe fn pte_mkspecial(p:pte_t)->pte_t {pte_set_flags(p,_PAGE_SPECIAL)}
    pub unsafe fn mksaveddirty_shift(mut v:u64)->u64 {let c=(!v>>_PAGE_BIT_RW)&1;v|=((v>>_PAGE_BIT_DIRTY)&c)<<_PAGE_BIT_SAVED_DIRTY;v&=!(c<<_PAGE_BIT_DIRTY);v}
    pub unsafe fn clear_saveddirty_shift(mut v:u64)->u64 {let c=(v>>_PAGE_BIT_RW)&1;v|=((v>>_PAGE_BIT_SAVED_DIRTY)&c)<<_PAGE_BIT_DIRTY;v&=!(c<<_PAGE_BIT_SAVED_DIRTY);v}
    pub unsafe fn pte_mksaveddirty(p:pte_t)->pte_t {native_make_pte(mksaveddirty_shift(native_pte_val(p)))}
    pub unsafe fn pte_clear_saveddirty(p:pte_t)->pte_t {native_make_pte(clear_saveddirty_shift(native_pte_val(p)))}
    pub unsafe fn pte_wrprotect(p:pte_t)->pte_t {pte_mksaveddirty(pte_clear_flags(p,_PAGE_RW))}
    pub unsafe fn pte_mkdirty(p:pte_t)->pte_t {pte_mksaveddirty(pte_set_flags(p,_PAGE_DIRTY|_PAGE_SOFT_DIRTY))}
    pub unsafe fn pte_mkwrite_shstk(p:pte_t)->pte_t {pte_set_flags(pte_clear_flags(p,_PAGE_RW),_PAGE_DIRTY)}
    pub unsafe fn pte_mkwrite_novma(p:pte_t)->pte_t {pte_set_flags(p,_PAGE_RW)}
    pub unsafe fn pmd_mksaveddirty(p:pmd_t)->pmd_t {native_make_pmd(mksaveddirty_shift(native_pmd_val(p)))}
    pub unsafe fn pmd_clear_saveddirty(p:pmd_t)->pmd_t {native_make_pmd(clear_saveddirty_shift(native_pmd_val(p)))}
    pub unsafe fn pmd_wrprotect(p:pmd_t)->pmd_t {pmd_mksaveddirty(pmd_clear_flags(p,_PAGE_RW))}
    pub unsafe fn pmd_mkold(p:pmd_t)->pmd_t {pmd_clear_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pmd_mkclean(p:pmd_t)->pmd_t {pmd_clear_flags(p,_PAGE_DIRTY_BITS)}
    pub unsafe fn pmd_mkdirty(p:pmd_t)->pmd_t {pmd_mksaveddirty(pmd_set_flags(p,_PAGE_DIRTY|_PAGE_SOFT_DIRTY))}
    pub unsafe fn pmd_mkwrite_shstk(p:pmd_t)->pmd_t {pmd_set_flags(pmd_clear_flags(p,_PAGE_RW),_PAGE_DIRTY)}
    pub unsafe fn pmd_mkhuge(p:pmd_t)->pmd_t {pmd_set_flags(p,_PAGE_PSE)}
    pub unsafe fn pmd_mkyoung(p:pmd_t)->pmd_t {pmd_set_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pmd_mkwrite_novma(p:pmd_t)->pmd_t {pmd_set_flags(p,_PAGE_RW)}
    pub unsafe fn pud_mksaveddirty(p:pud_t)->pud_t {native_make_pud(mksaveddirty_shift(native_pud_val(p)))}
    pub unsafe fn pud_clear_saveddirty(p:pud_t)->pud_t {native_make_pud(clear_saveddirty_shift(native_pud_val(p)))}
    pub unsafe fn pud_mkold(p:pud_t)->pud_t {pud_clear_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pud_mkclean(p:pud_t)->pud_t {pud_clear_flags(p,_PAGE_DIRTY_BITS)}
    pub unsafe fn pud_wrprotect(p:pud_t)->pud_t {pud_mksaveddirty(pud_clear_flags(p,_PAGE_RW))}
    pub unsafe fn pud_mkdirty(p:pud_t)->pud_t {pud_mksaveddirty(pud_set_flags(p,_PAGE_DIRTY|_PAGE_SOFT_DIRTY))}
    pub unsafe fn pud_mkhuge(p:pud_t)->pud_t {pud_set_flags(p,_PAGE_PSE)}
    pub unsafe fn pud_mkyoung(p:pud_t)->pud_t {pud_set_flags(p,_PAGE_ACCESSED)}
    pub unsafe fn pud_mkwrite(p:pud_t)->pud_t {pud_clear_saveddirty(pud_set_flags(p,_PAGE_RW))}
    pub unsafe fn pte_pgprot(p:pte_t)->pgprot_t {__pgprot(pte_flags(p))}
    pub unsafe fn pmd_pgprot(p:pmd_t)->pgprot_t {__pgprot(pmd_flags(p))}
    pub unsafe fn pud_pgprot(p:pud_t)->pgprot_t {__pgprot(pud_flags(p))}
    pub unsafe fn p4d_pgprot(p:p4d_t)->pgprot_t {__pgprot(p4d_flags(p))}
    pub unsafe fn page_level_shift(level:pg_level)->usize {(PAGE_SHIFT-PTE_SHIFT)+(level as usize)*PTE_SHIFT}
    pub unsafe fn page_level_size(level:pg_level)->usize {1usize<<page_level_shift(level)}
    pub unsafe fn page_level_mask(level:pg_level)->usize {!(page_level_size(level)-1)}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
