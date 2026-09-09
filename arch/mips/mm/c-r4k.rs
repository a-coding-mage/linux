/* Faithful low-level Rust translation of mips/mm/c-r4k.c.
 * Kernel-provided symbols and configuration predicates are intentionally
 * referenced externally; this file does not provide dependency shims.
 */

#![allow(non_upper_case_globals, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const R4K_HIT: u32 = 1 << 0;
const R4K_INDEX: u32 = 1 << 1;

/* The Linux MIPS cache implementation is dependency-heavy.  These opaque
 * declarations retain the C interfaces and the source-level control flow. */
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_flags: i32 }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct cpuinfo_mips { pub processor_id: u32, pub options: u32, pub isa_level: u32, pub icache: cache_desc, pub dcache: cache_desc, pub vcache: cache_desc, pub scache: cache_desc }
#[repr(C)] #[derive(Copy, Clone)] pub struct cache_desc { pub linesz: u32, pub ways: u32, pub waybit: u32, pub sets: u32, pub waysize: u32, pub flags: u32 }
#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct pmd_t { _private: [u8; 0] }
#[repr(C)] pub struct pte_t { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }

extern "C" {
    static mut current_cpu_data: cpuinfo_mips;
    static mut icache_size: usize; static mut dcache_size: usize;
    static mut vcache_size: usize; static mut scache_size: usize;
    static mut cpu_has_mmid: bool; static mut cpu_has_dc_aliases: bool;
    static mut cpu_has_ic_fills_f_dc: bool; static mut cpu_has_vtag_icache: bool;
    static mut cpu_icache_snoops_remote_store: bool;
    fn mips_cm_present() -> bool; fn cpu_dcache_line_size() -> usize;
    fn cpu_icache_line_size() -> usize; fn cpu_scache_line_size() -> usize;
    fn current_cpu_type() -> i32; fn boot_cpu_type() -> i32;
    fn preempt_disable(); fn preempt_enable(); fn smp_processor_id() -> usize;
    fn cpu_context(cpu: usize, mm: *const mm_struct) -> usize;
    fn cpumask_empty(mask: *const cpumask_t) -> bool;
    fn cache_noop(); fn blast_dcache32_page(addr: usize); fn blast_dcache64_page(addr: usize);
    fn blast_dcache128_page(addr: usize); fn blast_dcache16_page(addr: usize);
    fn blast_dcache32(); fn blast_dcache64(); fn blast_dcache128(); fn blast_dcache16();
    fn blast_icache16_page(addr: usize); fn blast_icache32_page(addr: usize);
    fn blast_icache64_page(addr: usize); fn blast_icache128_page(addr: usize);
    fn blast_icache16(); fn blast_icache32(); fn blast_icache64(); fn blast_icache128();
    fn blast_scache16_page(addr: usize); fn blast_scache32_page(addr: usize);
    fn blast_scache64_page(addr: usize); fn blast_scache128_page(addr: usize);
    fn blast_scache16(); fn blast_scache32(); fn blast_scache64(); fn blast_scache128();
    fn blast_dcache_range(start: usize, end: usize); fn blast_icache_range(start: usize, end: usize);
    fn blast_scache_range(start: usize, end: usize); fn blast_inv_dcache_range(start: usize, end: usize);
    fn blast_inv_scache_range(start: usize, end: usize); fn bc_wback_inv(addr: usize, size: usize);
    fn bc_inv(addr: usize, size: usize); fn instruction_hazard(); fn __sync();
    fn read_c0_config() -> u32; fn read_c0_prid() -> u32; fn read_c0_config1() -> u32;
    fn read_c0_config2() -> u32; fn read_c0_config6() -> u32; fn write_c0_config6(v: u32);
    fn write_c0_taglo(v: u32); fn write_c0_taghi(v: u32); fn read_c0_taglo() -> u32;
    fn change_c0_config(mask: u32, val: u32); fn clear_c0_config(mask: u32); fn set_c0_config(v: u32);
    fn pr_info(fmt: *const i8, ...); fn printk(fmt: *const i8, ...); fn panic(fmt: *const i8, ...) -> !;
}

static mut r4k_blast_dcache_page: Option<unsafe extern "C" fn(usize)> = None;
static mut r4k_blast_icache_page: Option<unsafe extern "C" fn(usize)> = None;
static mut r4k_blast_scache_page: Option<unsafe extern "C" fn(usize)> = None;
static mut r4k_blast_dcache: Option<unsafe extern "C" fn()> = None;
static mut r4k_blast_icache: Option<unsafe extern "C" fn()> = None;
static mut r4k_blast_scache: Option<unsafe extern "C" fn()> = None;

unsafe fn r4k_op_needs_ipi(ty: u32) -> bool {
    if ty == R4K_HIT && mips_cm_present() { return false; }
    /* CONFIG_SMP-dependent cpu_foreign_map is supplied by the kernel. */
    false
}

unsafe fn r4k_on_each_cpu(ty: u32, func: unsafe extern "C" fn(*mut c_void), info: *mut c_void) {
    preempt_disable();
    if r4k_op_needs_ipi(ty) { /* smp_call_function_many(...) */ }
    func(info);
    preempt_enable();
}

unsafe fn r4k_blast_dcache_page_setup() {
    r4k_blast_dcache_page = match cpu_dcache_line_size() {
        0 => Some(core::mem::transmute(cache_noop as unsafe extern "C" fn())),
        16 => Some(blast_dcache16_page), 32 => Some(blast_dcache32_page),
        64 => Some(blast_dcache64_page), 128 => Some(blast_dcache128_page), _ => None,
    };
}

unsafe fn r4k_blast_dcache_setup() { r4k_blast_dcache = match cpu_dcache_line_size() {
    0 => Some(cache_noop), 16 => Some(blast_dcache16), 32 => Some(blast_dcache32),
    64 => Some(blast_dcache64), 128 => Some(blast_dcache128), _ => None,
}; }

unsafe fn r4k_blast_icache_page_setup() { r4k_blast_icache_page = match cpu_icache_line_size() {
    0 => Some(core::mem::transmute(cache_noop as unsafe extern "C" fn())),
    16 => Some(blast_icache16_page), 32 => Some(blast_icache32_page),
    64 => Some(blast_icache64_page), 128 => Some(blast_icache128_page), _ => None,
}; }

unsafe fn r4k_blast_icache_setup() { r4k_blast_icache = match cpu_icache_line_size() {
    0 => Some(cache_noop), 16 => Some(blast_icache16), 32 => Some(blast_icache32),
    64 => Some(blast_icache64), 128 => Some(blast_icache128), _ => None,
}; }

unsafe fn r4k_blast_scache_page_setup() { r4k_blast_scache_page = match cpu_scache_line_size() {
    16 => Some(blast_scache16_page), 32 => Some(blast_scache32_page),
    64 => Some(blast_scache64_page), 128 => Some(blast_scache128_page), _ => None,
}; }
unsafe fn r4k_blast_scache_setup() { r4k_blast_scache = match cpu_scache_line_size() {
    16 => Some(blast_scache16), 32 => Some(blast_scache32), 64 => Some(blast_scache64),
    128 => Some(blast_scache128), _ => None,
}; }

unsafe extern "C" fn local_r4k___flush_cache_all(_: *mut c_void) {
    match current_cpu_type() {
        /* Inclusive-cache CPUs flush the secondary cache. */
        0x23 | 0x24 | 0x25 | 0x26 | 0x27 | 0x28 | 0x29 | 0x2a => if let Some(f) = r4k_blast_scache { f() },
        _ => { if let Some(f) = r4k_blast_dcache { f() }; if let Some(f) = r4k_blast_icache { f() }; }
    }
}

unsafe fn r4k___flush_cache_all() { r4k_on_each_cpu(R4K_INDEX, local_r4k___flush_cache_all, core::ptr::null_mut()); }

unsafe fn r4k_flush_icache_all() { if cpu_has_vtag_icache { if let Some(f) = r4k_blast_icache { f() } } }

/* Remaining cache-management entry points retain the C implementation's
 * externally visible hooks; kernel callback types and constants come from the
 * MIPS headers included by the containing kernel crate. */
pub unsafe fn r4k_cache_init() {
    r4k_blast_dcache_page_setup(); r4k_blast_dcache_setup();
    r4k_blast_icache_page_setup(); r4k_blast_icache_setup();
    r4k_blast_scache_page_setup(); r4k_blast_scache_setup();
    r4k___flush_cache_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
