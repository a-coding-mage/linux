// SPDX-License-Identifier: GPL-2.0-only
/* Page Attribute Table support: direct low-level Rust translation of memtype.c. */

// External kernel types, constants, macros, and functions are supplied by the
// surrounding kernel translation unit.

static mut pat_disabled: bool = !IS_ENABLED!(CONFIG_X86_PAT);
static mut pat_msr_val: u64 = 0;

unsafe fn pat_disable(msg_reason: *const i8) {
    if pat_disabled { return; }
    pat_disabled = true;
    pr_info!("x86/PAT: %s\n", msg_reason);
    memory_caching_control &= !CACHE_PAT;
}

unsafe fn nopat(_str: *mut i8) -> i32 {
    pat_disable(c"PAT support disabled via boot option.".as_ptr());
    0
}
early_param!("nopat", nopat);

pub unsafe fn pat_enabled() -> bool { !pat_disabled }
EXPORT_SYMBOL_GPL!(pat_enabled);

pub static mut pat_debug_enable: i32 = 0;
unsafe fn pat_debug_setup(_str: *mut i8) -> i32 { pat_debug_enable = 1; 1 }
__setup!("debugpat", pat_debug_setup);

#[cfg(CONFIG_X86_PAT)]
const _PGMT_WB: usize = 0;
#[cfg(CONFIG_X86_PAT)]
const _PGMT_WC: usize = 1usize << PG_arch_1;
#[cfg(CONFIG_X86_PAT)]
const _PGMT_UC_MINUS: usize = 1usize << PG_arch_2;
#[cfg(CONFIG_X86_PAT)]
const _PGMT_WT: usize = (1usize << PG_arch_2) | (1usize << PG_arch_1);
#[cfg(CONFIG_X86_PAT)]
const _PGMT_MASK: usize = (1usize << PG_arch_2) | (1usize << PG_arch_1);
#[cfg(CONFIG_X86_PAT)]
const _PGMT_CLEAR_MASK: usize = !_PGMT_MASK;

#[cfg(CONFIG_X86_PAT)]
unsafe fn get_page_memtype(pg: *mut page) -> page_cache_mode {
    let pg_flags = (*pg).flags.f & _PGMT_MASK;
    if pg_flags == _PGMT_WB { _PAGE_CACHE_MODE_WB }
    else if pg_flags == _PGMT_WC { _PAGE_CACHE_MODE_WC }
    else if pg_flags == _PGMT_UC_MINUS { _PAGE_CACHE_MODE_UC_MINUS }
    else { _PAGE_CACHE_MODE_WT }
}
#[cfg(CONFIG_X86_PAT)]
unsafe fn set_page_memtype(pg: *mut page, memtype: page_cache_mode) {
    let memtype_flags = match memtype {
        _PAGE_CACHE_MODE_WC => _PGMT_WC,
        _PAGE_CACHE_MODE_UC_MINUS => _PGMT_UC_MINUS,
        _PAGE_CACHE_MODE_WT => _PGMT_WT,
        _ => _PGMT_WB,
    };
    let mut old_flags = READ_ONCE!((*pg).flags.f);
    loop {
        let new_flags = (old_flags & _PGMT_CLEAR_MASK) | memtype_flags;
        if try_cmpxchg!(&mut (*pg).flags.f, &mut old_flags, new_flags) { break; }
    }
}
#[cfg(not(CONFIG_X86_PAT))]
unsafe fn get_page_memtype(_pg: *mut page) -> page_cache_mode { -1 }
#[cfg(not(CONFIG_X86_PAT))]
unsafe fn set_page_memtype(_pg: *mut page, _memtype: page_cache_mode) {}

unsafe fn pat_get_cache_mode(pat_val: u32, msg: *mut i8) -> page_cache_mode {
    let (cache, text) = match pat_val {
        X86_MEMTYPE_UC => (_PAGE_CACHE_MODE_UC, b"UC  "),
        X86_MEMTYPE_WC => (_PAGE_CACHE_MODE_WC, b"WC  "),
        X86_MEMTYPE_WT => (_PAGE_CACHE_MODE_WT, b"WT  "),
        X86_MEMTYPE_WP => (_PAGE_CACHE_MODE_WP, b"WP  "),
        X86_MEMTYPE_WB => (_PAGE_CACHE_MODE_WB, b"WB  "),
        X86_MEMTYPE_UC_MINUS => (_PAGE_CACHE_MODE_UC_MINUS, b"UC- "),
        _ => (_PAGE_CACHE_MODE_WB, b"WB  "),
    };
    core::ptr::copy_nonoverlapping(text.as_ptr() as *const i8, msg, 4);
    cache
}

unsafe fn init_cache_modes(pat: u64) {
    let mut msg = [0i8; 33];
    for i in (0..8).rev() {
        let cache = pat_get_cache_mode(((pat >> (i * 8)) & 7) as u32, msg.as_mut_ptr().add(4 * i));
        update_cache_mode_entry(i as i32, cache);
    }
    pr_info!("x86/PAT: Configuration [0-7]: %s\n", msg.as_ptr());
}

pub unsafe fn pat_cpu_init() {
    if !boot_cpu_has(X86_FEATURE_PAT) { panic!("x86/PAT: PAT enabled, but not supported by secondary CPU\n"); }
    wrmsrq(MSR_IA32_CR_PAT, pat_msr_val);
    __flush_tlb_all();
}

pub unsafe fn pat_bp_init() {
    let c = &boot_cpu_data;
    if !IS_ENABLED!(CONFIG_X86_PAT) { pr_info_once!("x86/PAT: PAT support disabled because CONFIG_X86_PAT is disabled in the kernel.\n"); }
    if !cpu_feature_enabled(X86_FEATURE_PAT) { pat_disable(c"PAT not supported by the CPU.".as_ptr()); }
    else { rdmsrq(MSR_IA32_CR_PAT, &mut pat_msr_val); }
    if pat_msr_val == 0 { pat_disable(c"PAT support disabled by the firmware.".as_ptr()); pat_msr_val = PAT_VALUE!(WB, WT, UC_MINUS, UC, WB, WT, UC_MINUS, UC); }
    if pat_disabled || cpu_feature_enabled(X86_FEATURE_XENPV) { init_cache_modes(pat_msr_val); return; }
    if (c.x86_vfm >= INTEL_PENTIUM_PRO && c.x86_vfm <= INTEL_PENTIUM_M_DOTHAN) ||
       (c.x86_vfm >= INTEL_P4_WILLAMETTE && c.x86_vfm <= INTEL_P4_CEDARMILL) {
        pat_msr_val = PAT_VALUE!(WB, WC, UC_MINUS, UC, WB, WC, UC_MINUS, UC);
    } else { pat_msr_val = PAT_VALUE!(WB, WC, UC_MINUS, UC, WB, WP, UC_MINUS, WT); }
    memory_caching_control |= CACHE_PAT;
    init_cache_modes(pat_msr_val);
}

static mut memtype_lock: spinlock = DEFINE_SPINLOCK!();

unsafe fn pat_x_mtrr_type(start: u64, end: u64, req_type: page_cache_mode) -> page_cache_mode {
    if req_type == _PAGE_CACHE_MODE_WB {
        let mut uniform = 0u8;
        let t = mtrr_type_lookup(start, end, &mut uniform);
        if t != MTRR_TYPE_WRBACK { return _PAGE_CACHE_MODE_UC_MINUS; }
        return _PAGE_CACHE_MODE_WB;
    }
    req_type
}

#[repr(C)]
struct pagerange_state { cur_pfn: usize, ram: i32, not_ram: i32 }
unsafe fn pagerange_is_ram_callback(initial_pfn: usize, total_nr_pages: usize, arg: *mut core::ffi::c_void) -> i32 {
    let s = &mut *(arg as *mut pagerange_state);
    s.not_ram |= (initial_pfn > s.cur_pfn) as i32;
    s.ram |= (total_nr_pages > 0) as i32;
    s.cur_pfn = initial_pfn + total_nr_pages;
    (s.ram != 0 && s.not_ram != 0) as i32
}
unsafe fn pat_pagerange_is_ram(start: resource_size_t, end: resource_size_t) -> i32 {
    let mut start_pfn = start >> PAGE_SHIFT;
    let end_pfn = (end + PAGE_SIZE - 1) >> PAGE_SHIFT;
    let mut state = pagerange_state { cur_pfn: start_pfn, ram: 0, not_ram: 0 };
    if start_pfn < (ISA_END_ADDRESS >> PAGE_SHIFT) { start_pfn = ISA_END_ADDRESS >> PAGE_SHIFT; }
    let ret = if start_pfn < end_pfn { walk_system_ram_range(start_pfn, end_pfn-start_pfn, &mut state as *mut _ as *mut _, pagerange_is_ram_callback) } else { 0 };
    if ret > 0 { -1 } else if state.ram != 0 { 1 } else { 0 }
}

unsafe fn reserve_ram_pages_type(start: u64, end: u64, mut req_type: page_cache_mode, new_type: *mut page_cache_mode) -> i32 {
    if req_type == _PAGE_CACHE_MODE_WP { if !new_type.is_null() { *new_type = _PAGE_CACHE_MODE_UC_MINUS; } return -EINVAL; }
    if req_type == _PAGE_CACHE_MODE_UC { WARN_ON_ONCE!(1); req_type = _PAGE_CACHE_MODE_UC_MINUS; }
    let mut pfn = start >> PAGE_SHIFT;
    while pfn < (end >> PAGE_SHIFT) { let typ = get_page_memtype(pfn_to_page(pfn)); if typ != _PAGE_CACHE_MODE_WB { if !new_type.is_null() { *new_type = typ; } return -EBUSY; } pfn += 1; }
    if !new_type.is_null() { *new_type = req_type; }
    pfn = start >> PAGE_SHIFT;
    while pfn < (end >> PAGE_SHIFT) { set_page_memtype(pfn_to_page(pfn), req_type); pfn += 1; }
    0
}
unsafe fn free_ram_pages_type(start: u64, end: u64) -> i32 { let mut p = start >> PAGE_SHIFT; while p < end >> PAGE_SHIFT { set_page_memtype(pfn_to_page(p), _PAGE_CACHE_MODE_WB); p += 1; } 0 }
unsafe fn sanitize_phys(address: u64) -> u64 { if IS_ENABLED!(CONFIG_X86_64) { address & __PHYSICAL_MASK } else { address } }

pub unsafe fn memtype_reserve(mut start: u64, mut end: u64, req_type: page_cache_mode, new_type: *mut page_cache_mode) -> i32 {
    start = sanitize_phys(start); end = sanitize_phys(end - 1) + 1;
    if start >= end { return -EINVAL; }
    if !pat_enabled() { if !new_type.is_null() { *new_type = req_type; } return 0; }
    if x86_platform.is_untracked_pat_range(start, end) { if !new_type.is_null() { *new_type = _PAGE_CACHE_MODE_WB; } return 0; }
    let actual = pat_x_mtrr_type(start, end, req_type);
    if !new_type.is_null() { *new_type = actual; }
    let ram = pat_pagerange_is_ram(start, end);
    if ram == 1 { return reserve_ram_pages_type(start, end, req_type, new_type); }
    if ram < 0 { return -EINVAL; }
    let e = kzalloc_obj!(memtype); if e.is_null() { return -ENOMEM; }
    (*e).start = start; (*e).end = end; (*e).type_ = actual;
    spin_lock!(&mut memtype_lock);
    let err = memtype_check_insert(e, new_type);
    if err != 0 { kfree(e); spin_unlock!(&mut memtype_lock); return err; }
    spin_unlock!(&mut memtype_lock); 0
}

pub unsafe fn memtype_free(mut start: u64, mut end: u64) -> i32 {
    if !pat_enabled() { return 0; }
    start = sanitize_phys(start); end = sanitize_phys(end);
    if x86_platform.is_untracked_pat_range(start, end) { return 0; }
    let ram = pat_pagerange_is_ram(start, end);
    if ram == 1 { return free_ram_pages_type(start, end); }
    if ram < 0 { return -EINVAL; }
    spin_lock!(&mut memtype_lock); let old = memtype_erase(start, end); spin_unlock!(&mut memtype_lock);
    if IS_ERR!(old) { return -EINVAL; } kfree(old); 0
}

unsafe fn lookup_memtype(paddr: u64) -> page_cache_mode {
    if x86_platform.is_untracked_pat_range(paddr, paddr + PAGE_SIZE) { return _PAGE_CACHE_MODE_WB; }
    if pat_pagerange_is_ram(paddr, paddr + PAGE_SIZE) != 0 { return get_page_memtype(pfn_to_page(paddr >> PAGE_SHIFT)); }
    spin_lock!(&mut memtype_lock); let e = memtype_lookup(paddr); let t = if !e.is_null() { (*e).type_ } else { _PAGE_CACHE_MODE_UC_MINUS }; spin_unlock!(&mut memtype_lock); t
}

pub unsafe fn pat_pfn_immune_to_uc_mtrr(pfn: usize) -> bool { let cm = lookup_memtype(PFN_PHYS!(pfn)); cm == _PAGE_CACHE_MODE_UC || cm == _PAGE_CACHE_MODE_UC_MINUS || cm == _PAGE_CACHE_MODE_WC }
EXPORT_SYMBOL_FOR_KVM!(pat_pfn_immune_to_uc_mtrr);

pub unsafe fn memtype_reserve_io(start: resource_size_t, end: resource_size_t, typ: *mut page_cache_mode) -> i32 { let mut nt = *typ; let ret = memtype_reserve(start, end, *typ, &mut nt); if ret != 0 { return ret; } if !is_new_memtype_allowed(start, end-start, *typ, nt) || memtype_kernel_map_sync(start, end-start, nt) < 0 { memtype_free(start,end); return -EBUSY; } *typ=nt; 0 }
pub unsafe fn memtype_free_io(start: resource_size_t, end: resource_size_t) { memtype_free(start,end); }

#[cfg(CONFIG_X86_PAT)]
pub unsafe fn arch_io_reserve_memtype_wc(start: resource_size_t, size: resource_size_t) -> i32 { let mut t=_PAGE_CACHE_MODE_WC; memtype_reserve_io(start,start+size,&mut t) }
#[cfg(CONFIG_X86_PAT)]
pub unsafe fn arch_io_free_memtype_wc(start: resource_size_t, size: resource_size_t) { memtype_free_io(start,start+size); }

pub unsafe fn phys_mem_access_prot(_file: *mut file, pfn: usize, size: usize, mut prot: pgprot_t) -> pgprot_t { if !phys_mem_access_encrypted(pfn << PAGE_SHIFT,size) { prot=pgprot_decrypted(prot); } prot }
unsafe fn pgprot_set_cachemode(prot: *mut pgprot_t, pcm: page_cache_mode) { *prot=__pgprot((pgprot_val(*prot) & !_PAGE_CACHE_MASK) | cachemode2protval(pcm)); }
pub unsafe fn phys_mem_access_prot_allowed(file: *mut file, pfn: usize, size: usize, prot: *mut pgprot_t) -> i32 { let mut pcm=_PAGE_CACHE_MODE_WB; if !pat_enabled(){return 1;} if !range_is_allowed(pfn,size){return 0;} if (*file).f_flags & O_DSYNC != 0 {pcm=_PAGE_CACHE_MODE_UC_MINUS;} pgprot_set_cachemode(prot,pcm); 1 }

pub unsafe fn memtype_kernel_map_sync(base: u64, size: usize, pcm: page_cache_mode) -> i32 { if base > __pa!(high_memory-1) || !page_is_ram(base >> PAGE_SHIFT) {return 0;} let id_sz=if __pa!(high_memory-1)<=base+size as u64 {__pa!(high_memory)-base} else {size as u64}; if ioremap_change_attr(__va!(base) as usize,id_sz as usize,pcm)<0{-EINVAL}else{0} }
unsafe fn reserve_pfn_range(paddr:u64,size:usize,prot:*mut pgprot_t)->i32 { let ram=pat_pagerange_is_ram(paddr,paddr+size as u64); let want=pgprot2cachemode(*prot); if ram!=0 {if !pat_enabled(){return 0;} let pcm=lookup_memtype(paddr); if want!=pcm {pgprot_set_cachemode(prot,pcm);} return 0;} let mut pcm=want; let ret=memtype_reserve(paddr,paddr+size as u64,want,&mut pcm); if ret!=0{return ret;} if pcm!=want {if !is_new_memtype_allowed(paddr,size,want,pcm){memtype_free(paddr,paddr+size as u64);return -EINVAL;} pgprot_set_cachemode(prot,pcm);} if memtype_kernel_map_sync(paddr,size,pcm)<0 {memtype_free(paddr,paddr+size as u64);return -EINVAL;} 0 }
unsafe fn free_pfn_range(paddr:u64,size:usize){if pat_pagerange_is_ram(paddr,paddr+size as u64)==0{memtype_free(paddr,paddr+size as u64);}}
pub unsafe fn pfnmap_setup_cachemode(pfn:usize,mut size:usize,prot:*mut pgprot_t)->i32 {if !pat_enabled(){return 0;} let mut p=(pfn as u64)<<PAGE_SHIFT; let pcm=lookup_memtype(p); while size>PAGE_SIZE {size-=PAGE_SIZE;p+=PAGE_SIZE;if pcm!=lookup_memtype(p){return -EINVAL;}} pgprot_set_cachemode(prot,pcm);0}
pub unsafe fn pfnmap_track(pfn:usize,size:usize,prot:*mut pgprot_t)->i32{reserve_pfn_range((pfn as u64)<<PAGE_SHIFT,size,prot)}
pub unsafe fn pfnmap_untrack(pfn:usize,size:usize){free_pfn_range((pfn as u64)<<PAGE_SHIFT,size)}
pub unsafe fn pgprot_writecombine(mut prot:pgprot_t)->pgprot_t{pgprot_set_cachemode(&mut prot,_PAGE_CACHE_MODE_WC);prot}
EXPORT_SYMBOL_GPL!(pgprot_writecombine);
pub unsafe fn pgprot_writethrough(mut prot:pgprot_t)->pgprot_t{pgprot_set_cachemode(&mut prot,_PAGE_CACHE_MODE_WT);prot}
EXPORT_SYMBOL_GPL!(pgprot_writethrough);

// CONFIG_DEBUG_FS && CONFIG_X86_PAT sequence/debugfs implementation is
// intentionally represented by the same external kernel interfaces here.
#[cfg(all(CONFIG_DEBUG_FS, CONFIG_X86_PAT))]
unsafe fn pat_memtype_list_init() -> i32 { if pat_enabled(){debugfs_create_file!("pat_memtype_list",S_IRUSR,arch_debugfs_dir,core::ptr::null_mut(),&memtype_fops);} 0 }
#[cfg(all(CONFIG_DEBUG_FS, CONFIG_X86_PAT))]
late_initcall!(pat_memtype_list_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
