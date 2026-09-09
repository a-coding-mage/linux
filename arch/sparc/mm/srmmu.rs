// SPDX-License-Identifier: GPL-2.0
/* Rust translation of sparc/mm/srmmu.c.  Kernel-provided types and routines
 * referenced by this file remain external dependencies. */

use core::{ffi::{c_char, c_int, c_void}, ptr};

#[allow(non_camel_case_types)] pub type ulong = usize;
#[repr(C)] pub struct mm_struct { pub context: isize, pub pgd: *mut pgd_t, pub page_table_lock: c_void }
#[repr(C)] pub struct task_struct { pub active_mm: *mut mm_struct }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_flags: ulong }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct page;
#[repr(C)] pub struct resource { pub start: ulong }
#[repr(C)] pub struct bit_map { pub map: *mut ulong, pub size: u32, pub used: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct pgd_t(pub u32);
#[repr(C)] #[derive(Copy, Clone)] pub struct p4d_t(pub u32);
#[repr(C)] #[derive(Copy, Clone)] pub struct pud_t(pub u32);
#[repr(C)] #[derive(Copy, Clone)] pub struct pmd_t(pub u32);
#[repr(C)] #[derive(Copy, Clone)] pub struct pte_t(pub u32);
#[repr(C)] pub struct ctxd_t(pub u32);
pub type pgtable_t = *mut pte_t;
pub type phandle = u32;

extern "C" {
    static mut srmmu_modtype: c_int; static mut vac_cache_size: c_int;
    static mut vac_line_size: c_int; static mut srmmu_ctx_table_phys: *mut ctxd_t;
    static mut srmmu_name: *mut c_char; static mut sparc_iomap: resource;
    static mut last_valid_pfn: ulong; static mut srmmu_swapper_pg_dir: *mut pgd_t;
    static mut srmmu_context_table: *mut ctxd_t; static mut viking_mxcc_present: c_int;
    static mut srmmu_nocache_pool: *mut c_void; static mut srmmu_nocache_map: bit_map;
    static mut srmmu_cache_pagetables: c_int; static mut flush_page_for_dma_global: c_int;
    static mut num_contexts: c_int;
}

const NO_CONTEXT: isize = -1;
const SRMMU_NOCACHE_BITMAP_SHIFT: usize = 12 - 4;

extern "C" {
    fn __nocache_pa(x: ulong) -> ulong; fn __nocache_fix<T>(x: *mut T) -> *mut T;
    fn set_pte(p: *mut pte_t, v: pte_t); fn __pte(v: u32) -> pte_t;
    fn __pgd(v: u32) -> pgd_t; fn __pmd(v: u32) -> pmd_t;
    fn pgd_offset_k(v: ulong) -> *mut pgd_t; fn p4d_offset(p: *mut pgd_t,v: ulong)->*mut p4d_t;
    fn pud_offset(p: *mut p4d_t,v: ulong)->*mut pud_t; fn pmd_offset(p: *mut pud_t,v: ulong)->*mut pmd_t;
    fn pte_offset_kernel(p: *mut pmd_t,v: ulong)->*mut pte_t;
    fn memset(p: *mut c_void, v: c_int, n: usize)->*mut c_void;
    fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn printk(fmt:*const c_char,...); fn panic(fmt:*const c_char,...); fn BUG();
    fn flush_cache_all(); fn flush_tlb_all(); fn flush_cache_mm(mm:*mut mm_struct); fn flush_tlb_mm(mm:*mut mm_struct);
    fn srmmu_set_context(c: isize); fn srmmu_set_ctable_ptr(p: ulong);
    fn memblock_alloc_or_panic(size:usize,align:usize)->*mut c_void;
    fn bit_map_init(m:*mut bit_map,b:*mut c_void,n:u32); fn bit_map_string_get(m:*mut bit_map,n:u32,a:u32)->c_int;
    fn bit_map_clear(m:*mut bit_map,o:c_int,n:c_int); fn is_power_of_2(x:c_int)->bool;
    fn pfn_to_page(p:ulong)->*mut page; fn page_ref_inc_return(p:*mut page)->c_int; fn page_ref_dec_return(p:*mut page)->c_int;
    fn pagetable_pte_ctor(mm:*mut mm_struct,p:*mut c_void)->bool; fn pagetable_dtor(p:*mut c_void); fn page_ptdesc(p:*mut page)->*mut c_void;
    fn pte_alloc_one_kernel(mm:*mut mm_struct)->*mut pte_t; fn spin_lock(l:*mut c_void); fn spin_unlock(l:*mut c_void);
    fn __pte_clear(p:*mut pte_t); fn srmmu_ctxd_set(ctx:*mut ctxd_t,pgd:*mut pgd_t);
    fn __srmmu_external_probe(v:ulong)->ulong;
}

static mut hwbug_bitmask: u32 = 0;
static mut srmmu_nocache_size: ulong = 0;
static mut srmmu_nocache_end: ulong = 0;
static mut ctx_list_pool: *mut ctx_list = ptr::null_mut();
static mut ctx_free: ctx_list = ctx_list::empty();
static mut ctx_used: ctx_list = ctx_list::empty();

#[repr(C)] struct ctx_list { next:*mut ctx_list, prev:*mut ctx_list, ctx_number:u32, ctx_mm:*mut mm_struct }
impl ctx_list { const fn empty()->Self { Self{next:ptr::null_mut(),prev:ptr::null_mut(),ctx_number:0,ctx_mm:ptr::null_mut()} } }

unsafe fn __srmmu_get_nocache(mut size:c_int, align:c_int)->*mut c_void {
    let minsz = 1i32 << SRMMU_NOCACHE_BITMAP_SHIFT;
    if size < minsz { size=minsz; } if size & (minsz-1) != 0 { size += minsz-1; }
    let off=bit_map_string_get(&mut srmmu_nocache_map,size as u32 >> SRMMU_NOCACHE_BITMAP_SHIFT,align as u32 >> SRMMU_NOCACHE_BITMAP_SHIFT);
    if off == -1 { return ptr::null_mut(); }
    (0xf8000000usize + ((off as usize)<<SRMMU_NOCACHE_BITMAP_SHIFT)) as *mut c_void
}
pub unsafe fn srmmu_get_nocache(size:c_int,align:c_int)->*mut c_void { let p=__srmmu_get_nocache(size,align); if !p.is_null(){memset(p,0,size as usize);} p }
pub unsafe fn srmmu_free_nocache(addr:*mut c_void,size:c_int) { let off=((addr as ulong)-0xf8000000)>>SRMMU_NOCACHE_BITMAP_SHIFT; bit_map_clear(&mut srmmu_nocache_map,off as c_int,(size as usize>>SRMMU_NOCACHE_BITMAP_SHIFT) as c_int); }

unsafe fn remove_from_ctx_list(e:*mut ctx_list) { (*(*e).next).prev=(*e).prev; (*(*e).prev).next=(*e).next; }
unsafe fn add_to_ctx_list(h:*mut ctx_list,e:*mut ctx_list) { (*e).next=h; (*e).prev=(*h).prev; (*(*e).prev).next=e; (*h).prev=e; }
unsafe fn alloc_context(old:*mut mm_struct, mm:*mut mm_struct) { let mut c=(*ctx_free.next); if c!=&mut ctx_free { remove_from_ctx_list(c); add_to_ctx_list(&mut ctx_used,c); (*mm).context=(*c).ctx_number as isize; (*c).ctx_mm=mm; return; } c=ctx_used.next; if (*c).ctx_mm==old {c=(*c).next;} if c==&mut ctx_used {panic(b"out of mmu contexts\0".as_ptr() as *const c_char);} flush_cache_mm((*c).ctx_mm); flush_tlb_mm((*c).ctx_mm); remove_from_ctx_list(c); add_to_ctx_list(&mut ctx_used,c); (*(*c).ctx_mm).context=NO_CONTEXT; (*c).ctx_mm=mm; (*mm).context=(*c).ctx_number as isize; }

pub unsafe fn get_pgd_fast()->*mut pgd_t { let p=__srmmu_get_nocache(4096,4096) as *mut pgd_t; if !p.is_null(){memset(p,0,256); } p }
pub unsafe fn pte_alloc_one(mm:*mut mm_struct)->pgtable_t { pte_alloc_one_kernel(mm) }
pub unsafe fn pte_free(_mm:*mut mm_struct,p:*mut pte_t) { srmmu_free_nocache(p as *mut c_void,4096); }
pub unsafe fn init_new_context(_tsk:*mut task_struct,mm:*mut mm_struct)->c_int {(*mm).context=NO_CONTEXT;0}
pub unsafe fn destroy_context(mm:*mut mm_struct){ if (*mm).context!=NO_CONTEXT { flush_cache_mm(mm); (*mm).context=NO_CONTEXT; } }
pub unsafe fn switch_mm(old:*mut mm_struct,mm:*mut mm_struct,_tsk:*mut task_struct){ if (*mm).context==NO_CONTEXT {alloc_context(old,mm); srmmu_ctxd_set(srmmu_context_table.add((*mm).context as usize),(*mm).pgd);} srmmu_set_context((*mm).context); }

pub unsafe fn srmmu_mapiorange(bus:u32,mut xpa:ulong,mut xva:ulong,mut len:u32){ while len!=0 {len-=4096; let p=pgd_offset_k(xva); let q=p4d_offset(p,xva); let r=pud_offset(q,xva); let s=pmd_offset(r,xva); let t=pte_offset_kernel(s,xva); set_pte(t,__pte(((xpa>>4)|(bus as ulong<<28)|0x20000001) as u32)); xpa+=4096;xva+=4096;} flush_tlb_all(); }
pub unsafe fn srmmu_unmapiorange(mut va:ulong,mut len:u32){while len!=0{len-=4096;let p=pgd_offset_k(va);let q=p4d_offset(p,va);let r=pud_offset(q,va);let s=pmd_offset(r,va);__pte_clear(pte_offset_kernel(s,va));va+=4096;}flush_tlb_all();}

pub unsafe fn mmu_info(_m:*mut seq_file) {}
pub unsafe fn load_mmu() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
