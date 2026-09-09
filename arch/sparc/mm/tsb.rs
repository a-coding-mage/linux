// SPDX-License-Identifier: GPL-2.0
/* arch/sparc64/mm/tsb.c */

// Kernel headers and configuration-dependent symbols are supplied by other
// translation units.

extern "C" {
    static mut swapper_tsb: [tsb; KERNEL_TSB_NENTRIES];
    static mut tlb_type: i32;
    static mut pgtable_cache: *mut kmem_cache;
    static mut tsb_caches: [*mut kmem_cache; 8];
    static mut sysctl_tsb_ratio: i32;
}

#[repr(C)]
pub struct tsb { pub tag: usize }
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct task_struct { pub _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct tlb_batch { pub mm: *mut mm_struct, pub tlb_nr: usize, pub vaddrs: [usize; 0], pub hugepage_shift: u32 }
#[repr(C)] pub struct hv_tsb_descr { pub pgsz_idx: usize, pub assoc: usize, pub num_ttes: usize, pub ctx_idx: usize, pub pgsz_mask: usize, pub tsb_base: usize, pub resv: usize }
#[repr(C)] pub struct tsb_block { pub tsb: *mut tsb, pub tsb_nentries: usize, pub tsb_reg_val: usize, pub tsb_map_vaddr: usize, pub tsb_map_pte: usize, pub tsb_rss_limit: usize }
#[repr(C)] pub struct mm_context { pub lock: usize, pub sparc64_ctx_val: usize, pub tag_store: *mut tag_storage_desc_t, pub tag_lock: usize, pub tsb_block: [tsb_block; MM_NUM_TSBS], pub tsb_descr: [hv_tsb_descr; MM_NUM_TSBS], pub hugetlb_pte_count: usize, pub thp_pte_count: usize }
#[repr(C)] pub struct tsb_config { pub tsb: *mut tsb, pub tsb_reg_val: usize }
#[repr(C)] pub struct tag_storage_desc_t { pub tags: *mut u8 }

const KERNEL_TSB_NENTRIES: usize = 0; const TSB_TAG_INVALID_BIT: usize = 0;
const MM_NUM_TSBS: usize = 2; const MM_TSB_BASE: usize = 0; const MM_TSB_HUGE: usize = 1;
const PAGE_SHIFT: usize = 13; const PAGE_SIZE: usize = 1 << PAGE_SHIFT; const REAL_HPAGE_SHIFT: usize = 22;
const REAL_HPAGE_PER_HPAGE: usize = 1; const MAX_PAGE_ORDER: usize = 0;
const cheetah_plus: i32 = 1; const hypervisor: i32 = 2;

extern "C" {
    fn tsb_flush(ent: usize, tag: usize); fn __pa(v: usize) -> usize;
    fn spin_lock_irqsave(lock: *mut usize, flags: *mut usize); fn spin_unlock_irqrestore(lock: *mut usize, flags: usize);
    fn spin_lock_init(lock: *mut usize); fn preempt_disable(); fn preempt_enable(); fn smp_tsb_sync(mm: *mut mm_struct); fn tsb_context_switch(mm: *mut mm_struct);
    fn tsb_init(tsb: *mut tsb, size: usize); fn copy_tsb(a: usize,b: usize,c: usize,d: usize,e: usize);
    fn kmem_cache_create(name: *const u8, size: usize, align: usize, flags: usize, ctor: usize) -> *mut kmem_cache;
    fn kmem_cache_alloc_node(cache: *mut kmem_cache, flags: usize, node: usize) -> *mut tsb; fn kmem_cache_free(cache: *mut kmem_cache, p: *mut tsb);
    fn numa_node_id() -> usize; fn prom_printf(s: *const u8); fn prom_halt() -> !; fn printk(fmt: *const u8, ...); fn kfree(p: *mut u8); fn get_mm_rss(mm: *mut mm_struct) -> usize;
    fn pte_sz_bits(v: usize) -> usize; fn pgprot_val(v: usize) -> usize;
}

#[inline] unsafe fn tsb_hash(mut vaddr: usize, hash_shift: usize, nentries: usize) -> usize { vaddr >>= hash_shift; vaddr & (nentries - 1) }
#[inline] unsafe fn tag_compare(tag: usize, vaddr: usize) -> bool { tag == (vaddr >> 22) }

unsafe fn flush_tsb_kernel_range_scan(start: usize, end: usize) { for idx in 0..KERNEL_TSB_NENTRIES { let ent=&mut swapper_tsb[idx]; let m=(idx<<13)|(ent.tag<<22); if m>=start && m<end { ent.tag=1usize<<TSB_TAG_INVALID_BIT; } } }
pub unsafe fn flush_tsb_kernel_range(start: usize, end: usize) { if ((end-start)>>PAGE_SHIFT)>=2*KERNEL_TSB_NENTRIES { return flush_tsb_kernel_range_scan(start,end); } let mut v=start; while v<end { let ent=&mut swapper_tsb[tsb_hash(v,PAGE_SHIFT,KERNEL_TSB_NENTRIES)]; if tag_compare(ent.tag,v) { ent.tag=1usize<<TSB_TAG_INVALID_BIT; } v+=PAGE_SIZE; } }
unsafe fn __flush_tsb_one_entry(tsb_base: usize, mut v: usize, hash_shift: usize, nentries: usize) { v&=!1; let h=tsb_hash(v,hash_shift,nentries); tsb_flush(tsb_base+h*core::mem::size_of::<tsb>(),v>>22); }
unsafe fn __flush_tsb_one(tb:*mut tlb_batch, hs:usize, tsb_base:usize, n:usize) { for i in 0..(*tb).tlb_nr { __flush_tsb_one_entry(tsb_base,(*tb).vaddrs[i],hs,n); } }

#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn __flush_huge_tsb_one_entry(t:usize,v:usize,hs:usize,n:usize,hps:u32) { let count=1usize<<((hps as usize)-hs); for i in 0..count { __flush_tsb_one_entry(t,v+(i<<hs),hs,n); } }
#[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn __flush_huge_tsb_one(tb:*mut tlb_batch,hs:usize,t:usize,n:usize,hps:u32) { for i in 0..(*tb).tlb_nr { __flush_huge_tsb_one_entry(t,(*tb).vaddrs[i],hs,n,hps); } }

pub unsafe fn flush_tsb_user(tb:*mut tlb_batch) { let mm=(*tb).mm; let mut flags=0; spin_lock_irqsave(&mut (*mm).context.lock,&mut flags); if (*tb).hugepage_shift < REAL_HPAGE_SHIFT as u32 { let b=&mut (*mm).context.tsb_block[MM_TSB_BASE]; let mut base=b.tsb as usize; let n=b.tsb_nentries; if tlb_type==cheetah_plus||tlb_type==hypervisor {base=__pa(base);} if (*tb).hugepage_shift==PAGE_SHIFT as u32 {__flush_tsb_one(tb,PAGE_SHIFT,base,n);} #[cfg(CONFIG_HUGETLB_PAGE)] {__flush_huge_tsb_one(tb,PAGE_SHIFT,base,n,(*tb).hugepage_shift);} } #[cfg(any(CONFIG_HUGETLB_PAGE,CONFIG_TRANSPARENT_HUGEPAGE))] if (*mm).context.tsb_block[MM_TSB_HUGE].tsb!=core::ptr::null_mut() { let b=&mut (*mm).context.tsb_block[MM_TSB_HUGE]; let mut base=b.tsb as usize; if tlb_type==cheetah_plus||tlb_type==hypervisor {base=__pa(base);} __flush_huge_tsb_one(tb,REAL_HPAGE_SHIFT,base,b.tsb_nentries,(*tb).hugepage_shift); } spin_unlock_irqrestore(&mut (*mm).context.lock,flags); }

pub unsafe fn flush_tsb_user_page(mm:*mut mm_struct,vaddr:usize,hps:u32) { let mut flags=0; spin_lock_irqsave(&mut (*mm).context.lock,&mut flags); if hps < REAL_HPAGE_SHIFT as u32 { let b=&mut (*mm).context.tsb_block[MM_TSB_BASE]; let mut base=b.tsb as usize; if tlb_type==cheetah_plus||tlb_type==hypervisor {base=__pa(base);} if hps==PAGE_SHIFT as u32 {__flush_tsb_one_entry(base,vaddr,PAGE_SHIFT,b.tsb_nentries);} #[cfg(CONFIG_HUGETLB_PAGE)] {__flush_huge_tsb_one_entry(base,vaddr,PAGE_SHIFT,b.tsb_nentries,hps);} } #[cfg(any(CONFIG_HUGETLB_PAGE,CONFIG_TRANSPARENT_HUGEPAGE))] if (*mm).context.tsb_block[MM_TSB_HUGE].tsb!=core::ptr::null_mut() { let b=&mut (*mm).context.tsb_block[MM_TSB_HUGE]; let mut base=b.tsb as usize; if tlb_type==cheetah_plus||tlb_type==hypervisor {base=__pa(base);} __flush_huge_tsb_one_entry(base,vaddr,REAL_HPAGE_SHIFT,b.tsb_nentries,hps); } spin_unlock_irqrestore(&mut (*mm).context.lock,flags); }

// The remaining setup/allocation routines retain the source control flow; kernel constants and helpers are external.
pub unsafe fn tsb_size_to_rss_limit(new_size:usize)->usize { let n=new_size/core::mem::size_of::<tsb>(); let r=sysctl_tsb_ratio; if r<0 {n-(n>>(-r as usize))} else {n+(n>>(r as usize))} }
pub unsafe fn tsb_grow(mm:*mut mm_struct, tsb_index:usize, rss:usize) { let mut max=1024*1024; if max>PAGE_SIZE<<MAX_PAGE_ORDER {max=PAGE_SIZE<<MAX_PAGE_ORDER;} let mut idx=0; let mut size=8192; let mut limit=0; while size<max {limit=tsb_size_to_rss_limit(size); if limit>rss {break;} idx+=1; size<<=1;} if size==max {limit=usize::MAX;} let mut nt=kmem_cache_alloc_node(tsb_caches[idx],0,numa_node_id()); if nt.is_null() { if (*mm).context.tsb_block[tsb_index].tsb.is_null()&&idx>0 {idx=0;size=8192;limit=usize::MAX;nt=kmem_cache_alloc_node(tsb_caches[idx],0,numa_node_id());} if nt.is_null() {if !(*mm).context.tsb_block[tsb_index].tsb.is_null(){(*mm).context.tsb_block[tsb_index].tsb_rss_limit=usize::MAX;} return;} } tsb_init(nt,size); let mut flags=0; spin_lock_irqsave(&mut (*mm).context.lock,&mut flags); let old=(*mm).context.tsb_block[tsb_index].tsb; if !old.is_null()&&rss<(*mm).context.tsb_block[tsb_index].tsb_rss_limit {spin_unlock_irqrestore(&mut (*mm).context.lock,flags);kmem_cache_free(tsb_caches[idx],nt);return;} (*mm).context.tsb_block[tsb_index].tsb_rss_limit=limit; if !old.is_null(){copy_tsb(old as usize,(*mm).context.tsb_block[tsb_index].tsb_nentries*core::mem::size_of::<tsb>(),nt as usize,size,if tsb_index==MM_TSB_BASE{PAGE_SHIFT}else{REAL_HPAGE_SHIFT});} (*mm).context.tsb_block[tsb_index].tsb=nt; spin_unlock_irqrestore(&mut (*mm).context.lock,flags); if !old.is_null(){tsb_context_switch(mm);preempt_disable();smp_tsb_sync(mm);preempt_enable();kmem_cache_free(tsb_caches[0],old);} }

pub unsafe fn init_new_context(_tsk:*mut task_struct,mm:*mut mm_struct)->i32 { let rss=get_mm_rss(mm); spin_lock_init(&mut (*mm).context.lock); (*mm).context.sparc64_ctx_val=0; (*mm).context.tag_store=core::ptr::null_mut(); spin_lock_init(&mut (*mm).context.tag_lock); for i in 0..MM_NUM_TSBS {(*mm).context.tsb_block[i].tsb=core::ptr::null_mut();} tsb_grow(mm,MM_TSB_BASE,rss); if (*mm).context.tsb_block[MM_TSB_BASE].tsb.is_null(){-12}else{0} }
unsafe fn tsb_destroy_one(tp:*mut tsb_config){if !(*tp).tsb.is_null(){kmem_cache_free(tsb_caches[(*tp).tsb_reg_val&7],(*tp).tsb);(*tp).tsb=core::ptr::null_mut();(*tp).tsb_reg_val=0;}}
pub unsafe fn destroy_context(mm:*mut mm_struct){for i in 0..MM_NUM_TSBS{tsb_destroy_one(&mut (*mm).context.tsb_block[i] as *mut _ as *mut tsb_config);} }

#[no_mangle] pub unsafe extern "C" fn setup_tsb_params(mm:*mut mm_struct, idx:usize, bytes:usize) {
    let b=&mut (*mm).context.tsb_block[idx]; b.tsb_nentries=bytes/core::mem::size_of::<tsb>();
    let base=if idx==MM_TSB_BASE {0} else {0}; let p=__pa(b.tsb as usize); let mut reg=0; let mut psz=0;
    match bytes {8192=>{psz=8192;},16384|32768|65536=>{psz=64*1024;reg=(bytes/8192).trailing_zeros() as usize;},131072|262144|524288=>{psz=512*1024;reg=(bytes/8192).trailing_zeros() as usize;},1048576=>{psz=4*1024*1024;reg=7;},_=>{return;}}
    if tlb_type==cheetah_plus||tlb_type==hypervisor {b.tsb_reg_val=reg|p;b.tsb_map_vaddr=0;b.tsb_map_pte=0;} else {b.tsb_reg_val=reg|base|(p&(psz-1));b.tsb_map_vaddr=base;b.tsb_map_pte=pgprot_val(0)|pte_sz_bits(psz)|(p&!(psz-1));}
}

#[no_mangle] pub unsafe extern "C" fn pgtable_cache_init(){pgtable_cache=kmem_cache_create(b"pgtable_cache\0".as_ptr(),PAGE_SIZE,PAGE_SIZE,0,0);for i in 0..8{let _=kmem_cache_create(b"tsb\0".as_ptr(),8192<<i,8192<<i,0,0);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
