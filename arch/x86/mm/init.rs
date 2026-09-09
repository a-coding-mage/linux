// Translated from init.c. Included kernel dependencies are supplied elsewhere.

static mut __cachemode2pte_tbl: [u16; _PAGE_CACHE_MODE_NUM as usize] = [0; _PAGE_CACHE_MODE_NUM as usize];
static mut __pte2cachemode_tbl: [u8; 8] = [0; 8];

pub unsafe fn cachemode2protval(pcm: page_cache_mode) -> c_ulong {
    if likely(pcm == 0) { return 0; }
    __cachemode2pte_tbl[pcm as usize] as c_ulong
}

pub unsafe fn x86_has_pat_wp() -> bool {
    let prot = __cachemode2pte_tbl[_PAGE_CACHE_MODE_WP as usize];
    __pte2cachemode_tbl[__pte2cm_idx(prot as c_ulong) as usize] == _PAGE_CACHE_MODE_WP as u8
}

pub unsafe fn pgprot2cachemode(pgprot: pgprot_t) -> page_cache_mode {
    let masked = pgprot_val(pgprot) & _PAGE_CACHE_MASK;
    if likely(masked == 0) { return 0 as page_cache_mode; }
    __pte2cachemode_tbl[__pte2cm_idx(masked) as usize] as page_cache_mode
}

static mut pgt_buf_start: c_ulong = 0;
static mut pgt_buf_end: c_ulong = 0;
static mut pgt_buf_top: c_ulong = 0;
static mut min_pfn_mapped: c_ulong = 0;
static mut can_use_brk_pgt: bool = true;

pub unsafe fn alloc_low_pages(num: c_uint) -> *mut c_void {
    let mut pfn: c_ulong;
    if after_bootmem {
        let order = get_order((num as c_ulong) << PAGE_SHIFT);
        return __get_free_pages(GFP_ATOMIC | __GFP_ZERO, order) as *mut c_void;
    }
    if pgt_buf_end + num as c_ulong > pgt_buf_top || !can_use_brk_pgt {
        let mut ret: c_ulong = 0;
        if min_pfn_mapped < max_pfn_mapped {
            ret = memblock_phys_alloc_range(PAGE_SIZE * num as c_ulong, PAGE_SIZE,
                min_pfn_mapped << PAGE_SHIFT, max_pfn_mapped << PAGE_SHIFT);
        }
        if ret == 0 && can_use_brk_pgt { ret = __pa(extend_brk(PAGE_SIZE * num as c_ulong, PAGE_SIZE)); }
        if ret == 0 { panic!("alloc_low_pages: can not alloc memory"); }
        pfn = ret >> PAGE_SHIFT;
    } else {
        pfn = pgt_buf_end;
        pgt_buf_end += num as c_ulong;
    }
    for i in 0..num { clear_page(__va((pfn + i as c_ulong) << PAGE_SHIFT)); }
    __va(pfn << PAGE_SHIFT)
}

const INIT_PGD_PAGE_TABLES: usize = 4;
#[cfg(not(CONFIG_RANDOMIZE_MEMORY))]
const INIT_PGD_PAGE_COUNT: usize = 2 * INIT_PGD_PAGE_TABLES;
#[cfg(CONFIG_RANDOMIZE_MEMORY)]
const INIT_PGD_PAGE_COUNT: usize = 4 * INIT_PGD_PAGE_TABLES;
const INIT_PGT_BUF_SIZE: usize = INIT_PGD_PAGE_COUNT * PAGE_SIZE as usize;
RESERVE_BRK!(early_pgt_alloc, INIT_PGT_BUF_SIZE);

pub unsafe fn early_alloc_pgt_buf() {
    let tables = INIT_PGT_BUF_SIZE as c_ulong;
    let base = __pa(extend_brk(tables, PAGE_SIZE));
    pgt_buf_start = base >> PAGE_SHIFT;
    pgt_buf_end = pgt_buf_start;
    pgt_buf_top = pgt_buf_start + (tables >> PAGE_SHIFT);
}

pub static mut after_bootmem: c_int = 0;
early_param_on_off!("gbpages", "nogbpages", direct_gbpages, CONFIG_X86_DIRECT_GBPAGES);

#[repr(C)]
pub struct map_range { pub start: c_ulong, pub end: c_ulong, pub page_size_mask: c_ulong }
static mut page_size_mask: c_int = 0;

unsafe fn cr4_set_bits_and_update_boot(mask: c_ulong) {
    mmu_cr4_features |= mask;
    if !trampoline_cr4_features.is_null() { *trampoline_cr4_features = mmu_cr4_features; }
    cr4_set_bits(mask);
}

unsafe fn probe_page_size_mask() {
    if boot_cpu_has(X86_FEATURE_PSE) && !debug_pagealloc_enabled() { page_size_mask |= 1 << PG_LEVEL_2M; } else { direct_gbpages = 0; }
    if boot_cpu_has(X86_FEATURE_PSE) { cr4_set_bits_and_update_boot(X86_CR4_PSE); }
    __supported_pte_mask &= !_PAGE_GLOBAL;
    if boot_cpu_has(X86_FEATURE_PGE) { cr4_set_bits_and_update_boot(X86_CR4_PGE); __supported_pte_mask |= _PAGE_GLOBAL; }
    __default_kernel_pte_mask = __supported_pte_mask;
    if cpu_feature_enabled(X86_FEATURE_PTI) { __default_kernel_pte_mask &= !_PAGE_GLOBAL; }
    if direct_gbpages != 0 && boot_cpu_has(X86_FEATURE_GBPAGES) {
        printk(KERN_INFO, "Using GB pages for direct mapping\n"); page_size_mask |= 1 << PG_LEVEL_1G;
    } else { direct_gbpages = 0; }
}

unsafe fn setup_pcid() {
    if !IS_ENABLED(CONFIG_X86_64) || !boot_cpu_has(X86_FEATURE_PCID) { return; }
    let m = x86_match_cpu(invlpg_miss_ids.as_ptr());
    if !m.is_null() && boot_cpu_data.microcode < (*m).driver_data { pr_info!("Incomplete global flushes, disabling PCID"); setup_clear_cpu_cap(X86_FEATURE_PCID); return; }
    if boot_cpu_has(X86_FEATURE_PGE) { cr4_set_bits(X86_CR4_PCIDE); } else { setup_clear_cpu_cap(X86_FEATURE_PCID); }
}

static invlpg_miss_ids: [x86_cpu_id; 7] = [
    X86_MATCH_VFM!(INTEL_ALDERLAKE, 0x2e), X86_MATCH_VFM!(INTEL_ALDERLAKE_L, 0x42c),
    X86_MATCH_VFM!(INTEL_ATOM_GRACEMONT, 0x11), X86_MATCH_VFM!(INTEL_RAPTORLAKE, 0x118),
    X86_MATCH_VFM!(INTEL_RAPTORLAKE_P, 0x4117), X86_MATCH_VFM!(INTEL_RAPTORLAKE_S, 0x2e),
    X86_CPU_ID_EMPTY,
];

const NR_RANGE_MR: usize = if cfg!(CONFIG_X86_32) { 3 } else { 5 };

unsafe fn save_mr(mr: *mut map_range, mut nr: usize, start_pfn: c_ulong, end_pfn: c_ulong, mask: c_ulong) -> usize {
    if start_pfn < end_pfn {
        if nr >= NR_RANGE_MR { panic!("run out of range for init_memory_mapping\n"); }
        (*mr.add(nr)).start = start_pfn << PAGE_SHIFT; (*mr.add(nr)).end = end_pfn << PAGE_SHIFT;
        (*mr.add(nr)).page_size_mask = mask; nr += 1;
    } nr
}

unsafe fn adjust_range_page_size_mask(mr: *mut map_range, nr: usize) {
    for i in 0..nr {
        if page_size_mask & (1 << PG_LEVEL_2M) != 0 && (*mr.add(i)).page_size_mask & (1 << PG_LEVEL_2M) == 0 {
            let start = round_down((*mr.add(i)).start, PMD_SIZE); let end = round_up((*mr.add(i)).end, PMD_SIZE);
            if (!IS_ENABLED(CONFIG_X86_32) || (end >> PAGE_SHIFT) <= max_low_pfn) && memblock_is_region_memory(start, end-start) { (*mr.add(i)).page_size_mask |= 1 << PG_LEVEL_2M; }
        }
        if page_size_mask & (1 << PG_LEVEL_1G) != 0 && (*mr.add(i)).page_size_mask & (1 << PG_LEVEL_1G) == 0 {
            let start = round_down((*mr.add(i)).start, PUD_SIZE); let end = round_up((*mr.add(i)).end, PUD_SIZE);
            if memblock_is_region_memory(start, end-start) { (*mr.add(i)).page_size_mask |= 1 << PG_LEVEL_1G; }
        }
    }
}

unsafe fn page_size_string(mr: *const map_range) -> *const c_char {
    static S1: &[u8] = b"1G\0"; static S2: &[u8] = b"2M\0"; static S4: &[u8] = b"4M\0"; static S4K: &[u8] = b"4k\0";
    if (*mr).page_size_mask & (1<<PG_LEVEL_1G) != 0 { return S1.as_ptr() as *const c_char; }
    if IS_ENABLED(CONFIG_X86_32) && !IS_ENABLED(CONFIG_X86_PAE) && (*mr).page_size_mask & (1<<PG_LEVEL_2M) != 0 { return S4.as_ptr() as *const c_char; }
    if (*mr).page_size_mask & (1<<PG_LEVEL_2M) != 0 { return S2.as_ptr() as *const c_char; }
    S4K.as_ptr() as *const c_char
}

unsafe fn split_mem_range(mr: *mut map_range, mut nr: usize, start: c_ulong, end: c_ulong) -> usize {
    let limit = PFN_DOWN(end); let mut pfn = PFN_DOWN(start); let mut end_pfn;
    end_pfn = if IS_ENABLED(CONFIG_X86_32) && pfn == 0 { PFN_DOWN(PMD_SIZE) } else { round_up(pfn, PFN_DOWN(PMD_SIZE)) }; end_pfn = min(end_pfn, limit);
    if pfn < end_pfn { nr=save_mr(mr,nr,pfn,end_pfn,0); pfn=end_pfn; }
    let mut start_pfn=round_up(pfn,PFN_DOWN(PMD_SIZE));
    end_pfn=if IS_ENABLED(CONFIG_X86_32){round_down(limit,PFN_DOWN(PMD_SIZE))}else{min(round_up(pfn,PFN_DOWN(PUD_SIZE)),round_down(limit,PFN_DOWN(PMD_SIZE)))};
    if start_pfn<end_pfn {nr=save_mr(mr,nr,start_pfn,end_pfn,page_size_mask as c_ulong & (1<<PG_LEVEL_2M));pfn=end_pfn;}
    #[cfg(CONFIG_X86_64)] { start_pfn=round_up(pfn,PFN_DOWN(PUD_SIZE));end_pfn=round_down(limit,PFN_DOWN(PUD_SIZE));if start_pfn<end_pfn{nr=save_mr(mr,nr,start_pfn,end_pfn,page_size_mask as c_ulong & ((1<<PG_LEVEL_2M)|(1<<PG_LEVEL_1G)));pfn=end_pfn;} start_pfn=round_up(pfn,PFN_DOWN(PMD_SIZE));end_pfn=round_down(limit,PFN_DOWN(PMD_SIZE));if start_pfn<end_pfn{nr=save_mr(mr,nr,start_pfn,end_pfn,page_size_mask as c_ulong&(1<<PG_LEVEL_2M));pfn=end_pfn;} }
    nr=save_mr(mr,nr,pfn,limit,0); if after_bootmem==0 {adjust_range_page_size_mask(mr,nr);} nr
}

#[repr(C)] pub struct range { pub start: c_ulong, pub end: c_ulong }
pub static mut pfn_mapped: [range; E820_MAX_ENTRIES as usize] = [range{start:0,end:0}; E820_MAX_ENTRIES as usize];
pub static mut nr_pfn_mapped: c_int = 0;
unsafe fn add_pfn_range_mapped(start: c_ulong,end: c_ulong){nr_pfn_mapped=add_range_with_merge(pfn_mapped.as_mut_ptr(),E820_MAX_ENTRIES,nr_pfn_mapped,start,end);nr_pfn_mapped=clean_sort_range(pfn_mapped.as_mut_ptr(),E820_MAX_ENTRIES);max_pfn_mapped=max(max_pfn_mapped,end);if start<(1UL<<(32-PAGE_SHIFT)){max_low_pfn_mapped=max(max_low_pfn_mapped,min(end,1UL<<(32-PAGE_SHIFT)));}}
pub unsafe fn pfn_range_is_mapped(start:c_ulong,end:c_ulong)->bool{for i in 0..nr_pfn_mapped as usize{if start>=pfn_mapped[i].start&&end<=pfn_mapped[i].end{return true;}}false}

pub unsafe fn init_memory_mapping(start:c_ulong,end:c_ulong,prot:pgprot_t)->c_ulong{let mut mr:[map_range;NR_RANGE_MR]=[map_range{start:0,end:0,page_size_mask:0};NR_RANGE_MR];let nr=split_mem_range(mr.as_mut_ptr(),0,start,end);let mut ret=0;for i in 0..nr{ret=kernel_physical_mapping_init(mr[i].start,mr[i].end,mr[i].page_size_mask,prot);}add_pfn_range_mapped(start>>PAGE_SHIFT,ret>>PAGE_SHIFT);ret>>PAGE_SHIFT}

// The remaining routines preserve the original control flow and use the kernel symbols supplied by other translation units.
pub unsafe fn free_init_pages(what:*const c_char, mut begin:c_ulong, mut end:c_ulong){let ba=PAGE_ALIGN(begin);let ea=end&_PAGE_MASK;if WARN_ON(ba!=begin||ea!=end){begin=ba;end=ea;}if begin>=end{return;}if debug_pagealloc_enabled(){pr_info!("debug: unmapping init [mem %#010lx-%#010lx]",begin,end-1);kmemleak_free_part(begin as *mut c_void,end-begin);set_memory_np(begin,(end-begin)>>PAGE_SHIFT);}else{set_memory_nx(begin,(end-begin)>>PAGE_SHIFT);set_memory_rw(begin,(end-begin)>>PAGE_SHIFT);free_reserved_area(begin as *mut c_void,end as *mut c_void,POISON_FREE_INITMEM,what);}}
pub unsafe fn free_kernel_image_pages(what:*const c_char,begin:*mut c_void,end:*mut c_void){let b=begin as c_ulong;let e=end as c_ulong;free_init_pages(what,b,e);if IS_ENABLED(CONFIG_X86_64)&&cpu_feature_enabled(X86_FEATURE_PTI){set_memory_np_noalias(b,(e-b)>>PAGE_SHIFT);}}
pub unsafe fn free_initmem(){e820__reallocate_tables();mem_encrypt_free_decrypted_mem();free_kernel_image_pages(b"unused kernel image (initmem)\0".as_ptr() as *const c_char,&__init_begin as *const _ as *mut c_void,&__init_end as *const _ as *mut c_void);}
#[cfg(CONFIG_BLK_DEV_INITRD)] pub unsafe fn free_initrd_mem(start:c_ulong,end:c_ulong){free_init_pages(b"initrd\0".as_ptr() as *const c_char,start,PAGE_ALIGN(end));}
pub unsafe fn arch_zone_limits_init(max_zone_pfns:*mut c_ulong){#[cfg(CONFIG_ZONE_DMA)]{*max_zone_pfns.add(ZONE_DMA as usize)=min(MAX_DMA_PFN,max_low_pfn);}#[cfg(CONFIG_ZONE_DMA32)]{*max_zone_pfns.add(ZONE_DMA32 as usize)=min(MAX_DMA32_PFN,max_low_pfn);}*max_zone_pfns.add(ZONE_NORMAL as usize)=max_low_pfn;#[cfg(CONFIG_HIGHMEM)]{*max_zone_pfns.add(ZONE_HIGHMEM as usize)=max_pfn;}}
pub unsafe fn update_cache_mode_entry(entry:c_uint,cache:page_cache_mode){BUG_ON(entry==0&&cache!=_PAGE_CACHE_MODE_WB);__cachemode2pte_tbl[cache as usize]=__cm_idx2pte(entry) as u16;__pte2cachemode_tbl[entry as usize]=cache as u8;}

unsafe fn init_range_memory_mapping(rs:c_ulong,re:c_ulong)->c_ulong{let mut total=0;let mut i=0;let mut sp=0;let mut ep=0;while for_each_mem_pfn_range(i,MAX_NUMNODES,&mut sp,&mut ep,core::ptr::null_mut()){let s=clamp_val(PFN_PHYS(sp),rs,re);let e=clamp_val(PFN_PHYS(ep),rs,re);if s<e{can_use_brk_pgt=max(s,(pgt_buf_end as u64)<<PAGE_SHIFT)>=min(e,(pgt_buf_top as u64)<<PAGE_SHIFT);init_memory_mapping(s,e,PAGE_KERNEL);total+=e-s;can_use_brk_pgt=true;}i+=1;true} { } total}
unsafe fn memory_map_bottom_up(ms:c_ulong,me:c_ulong){let mut start=ms;let mut step=PMD_SIZE;let mut total=0;min_pfn_mapped=start>>PAGE_SHIFT;while start<me{let next=if step!=0&&me-start>step{min(round_up(start+1,step),me)}else{me};total+=init_range_memory_mapping(start,next);start=next;if total>=step{step=get_new_step_size(step);}}}
unsafe fn get_new_step_size(step:c_ulong)->c_ulong{step<<(PMD_SHIFT-PAGE_SHIFT-1)}
unsafe fn memory_map_top_down(ms:c_ulong,me:c_ulong){let a=memblock_phys_alloc_range(PMD_SIZE,PMD_SIZE,ms,me);let real=if a==0{max(ms,ALIGN_DOWN(me,PMD_SIZE))}else{memblock_phys_free(a,PMD_SIZE);a+PMD_SIZE};let mut step=PMD_SIZE;max_pfn_mapped=0;min_pfn_mapped=real>>PAGE_SHIFT;let mut last=real;let mut total=0;while last>ms{let start=if last>step{max(round_down(last-1,step),ms)}else{ms};total+=init_range_memory_mapping(start,last);last=start;min_pfn_mapped=last>>PAGE_SHIFT;if total>=step{step=get_new_step_size(step);}}if real<me{init_range_memory_mapping(real,me);}}
unsafe fn init_trampoline(){#[cfg(CONFIG_X86_64)]{if !kaslr_memory_enabled(){trampoline_pgd_entry=init_top_pgt[pgd_index(__PAGE_OFFSET)];}else{init_trampoline_kaslr();}}}
pub unsafe fn init_mem_mapping(){pti_check_boottime_disable();probe_page_size_mask();setup_pcid();let end=if IS_ENABLED(CONFIG_X86_64){max_pfn<<PAGE_SHIFT}else{max_low_pfn<<PAGE_SHIFT};init_memory_mapping(0,ISA_END_ADDRESS,PAGE_KERNEL);init_trampoline();if memblock_bottom_up(){let ke=__pa_symbol(_end);memory_map_bottom_up(ke,end);memory_map_bottom_up(ISA_END_ADDRESS,ke);}else{memory_map_top_down(ISA_END_ADDRESS,end);}if IS_ENABLED(CONFIG_X86_64)&&max_pfn>max_low_pfn{max_low_pfn=max_pfn;}else if !IS_ENABLED(CONFIG_X86_64){early_ioremap_page_table_range_init();}load_cr3(swapper_pg_dir);__flush_tlb_all();x86_init.hyper.init_mem_mapping();early_memtest(0,max_pfn_mapped<<PAGE_SHIFT);}
pub unsafe fn poking_init(){let mut ptl: *mut spinlock_t=core::ptr::null_mut();text_poke_mm=mm_alloc();BUG_ON(text_poke_mm.is_null());paravirt_enter_mmap(text_poke_mm);set_notrack_mm(text_poke_mm);text_poke_mm_addr=TASK_UNMAPPED_BASE;if IS_ENABLED(CONFIG_RANDOMIZE_BASE){text_poke_mm_addr+=(kaslr_get_random_long(b"Poking\0".as_ptr() as *const c_char)&PAGE_MASK)%(TASK_SIZE-TASK_UNMAPPED_BASE-3*PAGE_SIZE);}if ((text_poke_mm_addr+PAGE_SIZE)&!PMD_MASK)==0{text_poke_mm_addr+=PAGE_SIZE;}let ptep=get_locked_pte(text_poke_mm,text_poke_mm_addr,&mut ptl);BUG_ON(ptep.is_null());pte_unmap_unlock(ptep,ptl);}
pub unsafe fn devmem_is_allowed(pagenr:c_ulong)->c_int{if region_intersects(PFN_PHYS(pagenr),PAGE_SIZE,IORESOURCE_SYSTEM_RAM,IORES_DESC_NONE)!=REGION_DISJOINT{if pagenr<256{return 2;}return 0;}if iomem_is_exclusive(pagenr<<PAGE_SHIFT){if pagenr<256{return 1;}return 0;}1}
#[cfg(CONFIG_SWAP)] pub unsafe fn arch_max_swapfile_size()->c_ulong{let mut pages=generic_max_swapfile_size();if boot_cpu_has_bug(X86_BUG_L1TF)&&l1tf_mitigation!=L1TF_MITIGATION_OFF{let mut lim=l1tf_pfn_limit();#[cfg(any())] {lim<<=PAGE_SHIFT-SWP_OFFSET_FIRST_BIT;}pages=min(lim,pages);}pages}
#[cfg(CONFIG_EXECMEM)] pub static mut execmem_info: execmem_info_t=execmem_info_t::zeroed();
#[cfg(CONFIG_EXECMEM)] pub unsafe fn execmem_arch_setup()->*mut execmem_info_t{let mut off=0;if kaslr_enabled(){off=get_random_u32_inclusive(1,1024) as c_ulong*PAGE_SIZE;}let start=MODULES_VADDR+off;execmem_info=execmem_info_t::default_for(start,MODULES_END,MODULE_ALIGN);&mut execmem_info}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
