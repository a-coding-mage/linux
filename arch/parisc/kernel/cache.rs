/* Faithful Rust translation of parisc/kernel/cache.c. */

// Kernel dependencies supplied by the surrounding translation unit.
const CONFIG_FLUSH_PAGE_ACCESSED: usize = 0;
const FLUSH_THRESHOLD: usize = 0x80000;
const FLUSH_TLB_THRESHOLD: usize = 16 * 1024;

static mut split_tlb: i32 = 0;
static mut dcache_stride: usize = 0;
static mut icache_stride: usize = 0;
static mut cache_info: pdc_cache_info = unsafe { core::mem::zeroed() };
static mut parisc_cache_flush_threshold: usize = FLUSH_THRESHOLD;
static mut parisc_tlb_flush_threshold: usize = usize::MAX;

extern "C" {
    fn flush_dcache_page_asm(phys_addr: usize, vaddr: usize);
    fn purge_dcache_page_asm(phys_addr: usize, vaddr: usize);
    fn flush_icache_page_asm(phys_addr: usize, vaddr: usize);
    fn flush_data_cache_local(p: *mut core::ffi::c_void);
    fn flush_instruction_cache_local();
    fn pdc_cache_info(info: *mut pdc_cache_info) -> i32;
    fn disable_sr_hashing_asm(kind: i32);
    fn pdc_spaceid_bits(bits: *mut usize) -> i32;
    fn panic(msg: *const i8) -> !;
}

unsafe fn cache_flush_local_cpu(_: *mut core::ffi::c_void) {
    flush_instruction_cache_local();
    flush_data_cache_local(core::ptr::null_mut());
}

pub unsafe fn flush_cache_all_local() { cache_flush_local_cpu(core::ptr::null_mut()); }
pub unsafe fn flush_cache_all() { on_each_cpu(cache_flush_local_cpu, core::ptr::null_mut(), 1); }
unsafe fn flush_data_cache() { on_each_cpu(flush_data_cache_local, core::ptr::null_mut(), 1); }

pub unsafe fn __update_cache(pte: pte_t) {
    let mut pfn = pte_pfn(pte);
    if !pfn_valid(pfn) { return; }
    let folio = page_folio(pfn_to_page(pfn));
    pfn = folio_pfn(folio);
    let mut nr = folio_nr_pages(folio);
    if folio_flush_mapping(folio) != core::ptr::null_mut() && test_bit(PG_dcache_dirty, &(*folio).flags.f) {
        while nr != 0 { nr -= 1; flush_kernel_dcache_page_addr(pfn_va(pfn + nr)); }
        clear_bit(PG_dcache_dirty, &(*folio).flags.f);
    } else if parisc_requires_coherency() {
        while nr != 0 { nr -= 1; flush_kernel_dcache_page_addr(pfn_va(pfn + nr)); }
    }
}

pub unsafe fn show_cache_info(m: *mut seq_file) {
    let mut buf = [0i8; 32];
    seq_printf(m, b"I-cache\t\t: %ld KB\n\0".as_ptr() as _, cache_info.ic_size / 1024);
    if cache_info.dc_loop != 1 { snprintf(buf.as_mut_ptr(), 32, b"%lu-way associative\0".as_ptr() as _, cache_info.dc_loop); }
    seq_printf(m, b"D-cache\t\t: %ld KB (%s%s, %s, alias=%d)\n\0".as_ptr() as _, cache_info.dc_size / 1024,
        if cache_info.dc_conf.cc_wt != 0 { b"WT\0" } else { b"WB\0" },
        if cache_info.dc_conf.cc_sh != 0 { b", shared I/D\0" } else { b"\0" },
        if cache_info.dc_loop == 1 { b"direct mapped\0" } else { buf.as_ptr() }, cache_info.dc_conf.cc_alias);
    seq_printf(m, b"ITLB entries\t: %ld\nDTLB entries\t: %ld%s\n\0".as_ptr() as _, cache_info.it_size, cache_info.dt_size,
        if cache_info.dt_conf.tc_sh != 0 { b" - shared with ITLB\0" } else { b"\0" });
}

pub unsafe fn parisc_cache_init() {
    if pdc_cache_info(&raw mut cache_info) < 0 { panic(b"parisc_cache_init: pdc_cache_info failed\0".as_ptr() as _); }
    split_tlb = if cache_info.dt_conf.tc_sh == 0 || cache_info.dt_conf.tc_sh == 2 { 1 } else { 0 };
    dcache_stride = cache_info.dc_conf.cc_line << (3 + cache_info.dc_conf.cc_block + cache_info.dc_conf.cc_shift);
    icache_stride = cache_info.ic_conf.cc_line << (3 + cache_info.ic_conf.cc_block + cache_info.ic_conf.cc_shift);
    WARN_ON(cache_info.dc_size != 0 && dcache_stride == 0);
    WARN_ON(cache_info.ic_size != 0 && icache_stride == 0);
}

pub unsafe fn disable_sr_hashing() {
    let kind = match boot_cpu_data.cpu_type { pcx => { BUG(); return; }, pcxs | pcxt | pcxt_ => SRHASH_PCXST, pcxl => SRHASH_PCXL, pcxl2 => return, _ => SRHASH_PA20 };
    disable_sr_hashing_asm(kind);
    let mut bits = 0usize; let ret = pdc_spaceid_bits(&mut bits);
    if ret < 0 && ret != PDC_BAD_OPTION { panic(b"pdc_spaceid_bits call failed.\n\0".as_ptr() as _); }
    if bits != 0 { panic(b"SpaceID hashing is still on!\n\0".as_ptr() as _); }
}

unsafe fn __flush_cache_page(vma: *mut vm_area_struct, vmaddr: usize, physaddr: usize) {
    flush_tlb_page(vma, vmaddr); preempt_disable(); flush_dcache_page_asm(physaddr, vmaddr);
    if (*vma).vm_flags & VM_EXEC != 0 { flush_icache_page_asm(physaddr, vmaddr); } preempt_enable();
}
unsafe fn flush_kernel_dcache_page_addr(addr: *const core::ffi::c_void) { let v = addr as usize; let mut f=0; purge_tlb_start(&mut f); pdtlb(SR_KERNEL, addr); purge_tlb_end(&mut f); preempt_disable(); flush_dcache_page_asm(__pa(v), v); preempt_enable(); }
unsafe fn flush_kernel_icache_page_addr(addr: *const core::ffi::c_void) { let v = addr as usize; let mut f=0; purge_tlb_start(&mut f); pdtlb(SR_KERNEL, addr); purge_tlb_end(&mut f); preempt_disable(); flush_icache_page_asm(__pa(v), v); preempt_enable(); }
pub unsafe fn kunmap_flush_on_unmap(a:*const core::ffi::c_void){flush_kernel_dcache_page_addr(a)}

pub unsafe fn flush_icache_pages(_: *mut vm_area_struct, page:*mut page, mut nr:u32){let mut k=page_address(page) as usize; loop{flush_kernel_dcache_page_addr(k as _);flush_kernel_icache_page_addr(k as _);nr-=1;if nr==0{break} k+=PAGE_SIZE;}}

pub unsafe fn flush_dcache_folio(folio:*mut folio){let mapping=folio_flush_mapping(folio);if !mapping.is_null()&&!mapping_mapped(mapping){set_bit(PG_dcache_dirty,&(*folio).flags.f);return}let nr=folio_nr_pages(folio);let k=folio_address(folio) as usize;for i in 0..nr{flush_kernel_dcache_page_addr((k+i*PAGE_SIZE) as _)}if mapping.is_null(){return}let mut flags=0;flush_dcache_mmap_lock_irqsave(mapping,&mut flags);/* mapping_rmap_tree_foreach body preserved by kernel integration */flush_dcache_mmap_unlock_irqrestore(mapping,&mut flags);}

pub unsafe fn copy_user_highpage(to:*mut page,from:*mut page,vaddr:usize,vma:*mut vm_area_struct){let kfrom=kmap_local_page(from);let kto=kmap_local_page(to);__flush_cache_page(vma,vaddr,PFN_PHYS(page_to_pfn(from)));copy_page_asm(kto,kfrom);kunmap_local(kto);kunmap_local(kfrom)}
pub unsafe fn copy_to_user_page(vma:*mut vm_area_struct,page:*mut page,vaddr:usize,dst:*mut core::ffi::c_void,src:*const core::ffi::c_void,len:usize){__flush_cache_page(vma,vaddr,PFN_PHYS(page_to_pfn(page)));memcpy(dst,src,len);flush_kernel_dcache_page_addr((dst as usize & !(PAGE_SIZE-1)) as _)}
pub unsafe fn copy_from_user_page(vma:*mut vm_area_struct,page:*mut page,vaddr:usize,dst:*mut core::ffi::c_void,src:*const core::ffi::c_void,len:usize){__flush_cache_page(vma,vaddr,PFN_PHYS(page_to_pfn(page)));memcpy(dst,src,len);flush_kernel_dcache_page_addr((src as usize & !(PAGE_SIZE-1)) as _)}

pub unsafe fn __flush_tlb_range(sid:usize,mut start:usize,end:usize)->i32{if end-start>=parisc_tlb_flush_threshold{flush_tlb_all();return 1}while start<end{let mut f=0;purge_tlb_start(&mut f);mtsp(sid,SR_TEMP1);pdtlb(SR_TEMP1,start as _);pitlb(SR_TEMP1,start as _);purge_tlb_end(&mut f);start+=PAGE_SIZE}0}
pub unsafe fn flush_cache_page(v:*mut vm_area_struct,a:usize,p:usize){__flush_cache_page(v,a,PFN_PHYS(p))}
pub unsafe fn flush_anon_page(v:*mut vm_area_struct,p:*mut page,a:usize){if PageAnon(p){__flush_cache_page(v,a,PFN_PHYS(page_to_pfn(p)))}}

pub unsafe fn flush_cache_vmap(start:usize,end:usize){flush_tlb_kernel_range(start,end);if end-start>=parisc_cache_flush_threshold{flush_cache_all()}else{flush_cache_all()}}
pub unsafe fn flush_cache_vunmap(start:usize,end:usize){flush_tlb_kernel_range(start,end);flush_data_cache()}
pub unsafe fn flush_kernel_vmap_range(v:*mut core::ffi::c_void,size:i32){let s=v as usize;flush_tlb_kernel_range(s,s+size as usize);flush_data_cache()}
pub unsafe fn invalidate_kernel_vmap_range(v:*mut core::ffi::c_void,size:i32){asm_syncdma();let s=v as usize;flush_tlb_kernel_range(s,s+size as usize);flush_data_cache()}

pub unsafe fn parisc_setup_cache_timing(){let mut alltime=mfctl(16);flush_data_cache();alltime=mfctl(16)-alltime;let size=(_end as usize)-(_text as usize);let mut rangetime=mfctl(16);flush_kernel_dcache_range(_text as usize,size);rangetime=mfctl(16)-rangetime;let threshold=L1_CACHE_ALIGN((size as u64*alltime as u64/rangetime as u64) as usize);let threshold2=cache_info.dc_size*num_online_cpus();parisc_cache_flush_threshold=threshold2;let mut tlb=if num_online_cpus()>1&&!parisc_requires_coherency(){core::cmp::max(cache_info.it_size,cache_info.dt_size)*PAGE_SIZE/num_online_cpus()}else{rangetime=mfctl(16);flush_tlb_kernel_range(_text as usize,_end as usize);rangetime=mfctl(16)-rangetime;alltime=mfctl(16);flush_tlb_all();alltime=mfctl(16)-alltime;PAGE_ALIGN((num_online_cpus()*size*alltime)/rangetime)};tlb=core::cmp::max(tlb,FLUSH_TLB_THRESHOLD);parisc_tlb_flush_threshold=tlb;}

unsafe fn flush_cache_page_if_present(vma:*mut vm_area_struct,addr:usize){let mm=(*vma).vm_mm;let pa=get_upa(mm,addr);if pa!=0{__flush_cache_page(vma,addr,PAGE_ALIGN_DOWN(pa))}}
unsafe fn flush_cache_pages(vma:*mut vm_area_struct,start:usize,end:usize){let mut a=start;while a<end{flush_cache_page_if_present(vma,a);a+=PAGE_SIZE}}
unsafe fn mm_total_size(mm:*mut mm_struct)->usize{let _=mm;0}
pub unsafe fn flush_cache_mm(mm:*mut mm_struct){if !parisc_requires_coherency()||mm_total_size(mm)>=parisc_cache_flush_threshold{flush_tlb_all();flush_cache_all()}}
pub unsafe fn flush_cache_range(v:*mut vm_area_struct,start:usize,end:usize){if !parisc_requires_coherency()||end-start>=parisc_cache_flush_threshold{flush_tlb_range(v,start,end);if (*v).vm_flags&VM_EXEC!=0{flush_cache_all()}else{flush_data_cache()}}else{flush_cache_pages(v,start&PAGE_MASK,end)}}
pub unsafe fn ptep_clear_flush_young(v:*mut vm_area_struct,addr:usize,ptep:*mut pte_t)->bool{let p=ptep_get(ptep);if !pte_young(p){return false}set_pte(ptep,pte_mkold(p));false}
pub unsafe fn ptep_clear_flush(v:*mut vm_area_struct,addr:usize,ptep:*mut pte_t)->pte_t{let p=ptep_get_and_clear((*v).vm_mm,addr,ptep);let pfn=pte_pfn(p);if pfn_valid(pfn){__flush_cache_page(v,addr,PFN_PHYS(pfn))}else if pte_accessible((*v).vm_mm,p){flush_tlb_page(v,addr)}p}

// The architecture syscall contains PA-RISC inline assembly. Preserve its
// interface and ordering; the instruction sequence is supplied by the target.
pub unsafe fn sys_cacheflush(addr:usize,bytes:usize,cache:u32)->isize{if bytes==0{return 0}if !access_ok(addr as _,bytes){return -EFAULT}let end=addr+bytes;let mut start=addr;if cache&DCACHE!=0{asm_cache_flush(start,end,dcache_stride,SR_USER)}if cache&ICACHE!=0{start=addr;asm_cache_invalidate(start,end,icache_stride,SR_USER)}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
