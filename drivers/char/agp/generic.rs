/* Direct Rust translation of generic.c. Kernel headers and external symbols are
 * supplied by the surrounding translation unit. */

pub static mut agp_gatt_table: *mut u32 = core::ptr::null_mut();
pub static mut agp_memory_reserved: i32 = 0;

pub unsafe fn agp_free_key(key: i32) {
    if key < 0 { return; }
    if key < MAXKEY { clear_bit(key as usize, (*agp_bridge).key_list); }
}

unsafe fn agp_get_key() -> i32 {
    let bit = find_first_zero_bit((*agp_bridge).key_list, MAXKEY);
    if bit < MAXKEY { set_bit(bit, (*agp_bridge).key_list); return bit as i32; }
    -1
}

pub unsafe fn agp_alloc_page_array(size: usize, mem: *mut agp_memory) { (*mem).pages = kvmalloc(size, GFP_KERNEL); }

unsafe fn agp_create_user_memory(num_agp_pages: usize) -> *mut agp_memory {
    if (i32::MAX as usize) / core::mem::size_of::<*mut page>() < num_agp_pages { return core::ptr::null_mut(); }
    let new = kzalloc_obj::<agp_memory>();
    if new.is_null() { return new; }
    (*new).key = agp_get_key();
    if (*new).key < 0 { kfree(new); return core::ptr::null_mut(); }
    agp_alloc_page_array(num_agp_pages * core::mem::size_of::<*mut page>(), new);
    if (*new).pages.is_null() { agp_free_key((*new).key); kfree(new); return core::ptr::null_mut(); }
    (*new).num_scratch_pages = 0; new
}

pub unsafe fn agp_create_memory(scratch_pages: i32) -> *mut agp_memory {
    let new = kzalloc_obj::<agp_memory>();
    if new.is_null() { return new; }
    (*new).key = agp_get_key();
    if (*new).key < 0 { kfree(new); return core::ptr::null_mut(); }
    agp_alloc_page_array(PAGE_SIZE * scratch_pages as usize, new);
    if (*new).pages.is_null() { agp_free_key((*new).key); kfree(new); return core::ptr::null_mut(); }
    (*new).num_scratch_pages = scratch_pages; (*new).type_ = AGP_NORMAL_MEMORY; new
}

pub unsafe fn agp_free_memory(curr: *mut agp_memory) {
    if curr.is_null() { return; }
    if (*curr).is_bound { agp_unbind_memory(curr); }
    if (*curr).type_ >= AGP_USER_TYPES { agp_generic_free_by_type(curr); return; }
    if (*curr).type_ != 0 { ((*(*(*curr).bridge).driver).free_by_type)(curr); return; }
    if (*curr).page_count != 0 {
        if let Some(f) = (*(*(*curr).bridge).driver).agp_destroy_pages { f(curr); }
        else { for i in 0..(*curr).page_count { ((*(*(*curr).bridge).driver).agp_destroy_page)((*curr).pages.add(i).read(), AGP_PAGE_DESTROY_UNMAP); }
               for i in 0..(*curr).page_count { ((*(*(*curr).bridge).driver).agp_destroy_page)((*curr).pages.add(i).read(), AGP_PAGE_DESTROY_FREE); } }
    }
    agp_free_key((*curr).key); agp_free_page_array(curr); kfree(curr);
}

const ENTRIES_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<usize>();

pub unsafe fn agp_allocate_memory(bridge: *mut agp_bridge_data, page_count: usize, type_: u32) -> *mut agp_memory {
    if bridge.is_null() { return core::ptr::null_mut(); }
    let cur = atomic_read(&(*bridge).current_memory_agp);
    if cur + page_count as i32 > (*bridge).max_memory_agp || cur + page_count as i32 < page_count as i32 { return core::ptr::null_mut(); }
    if type_ >= AGP_USER_TYPES { let n = agp_generic_alloc_user(page_count, type_ as i32); if !n.is_null() { (*n).bridge = bridge; } return n; }
    if type_ != 0 { let n = ((*(*bridge).driver).alloc_by_type)(page_count, type_ as i32); if !n.is_null() { (*n).bridge = bridge; } return n; }
    let scratch = (page_count + ENTRIES_PER_PAGE - 1) / ENTRIES_PER_PAGE;
    let new = agp_create_memory(scratch as i32); if new.is_null() { return new; }
    if let Some(f) = (*(*bridge).driver).agp_alloc_pages { if f(bridge, new, page_count) != 0 { agp_free_memory(new); return core::ptr::null_mut(); } (*new).bridge = bridge; return new; }
    for i in 0..page_count { let p = ((*(*bridge).driver).agp_alloc_page)(bridge); if p.is_null() { agp_free_memory(new); return core::ptr::null_mut(); } (*new).pages.add(i).write(p); (*new).page_count += 1; }
    (*new).bridge = bridge; new
}

pub unsafe fn agp_bind_memory(curr: *mut agp_memory, pg_start: isize) -> i32 {
    if curr.is_null() || (*curr).is_bound { return -EINVAL; }
    if !(*curr).is_flushed { ((*(*(*curr).bridge).driver).cache_flush)(); (*curr).is_flushed = true; }
    let r = ((*(*(*curr).bridge).driver).insert_memory)(curr, pg_start, (*curr).type_); if r != 0 { return r; }
    (*curr).is_bound = true; (*curr).pg_start = pg_start; spin_lock(&(*agp_bridge).mapped_lock); list_add(&mut (*curr).mapped_list, &mut (*agp_bridge).mapped_list); spin_unlock(&(*agp_bridge).mapped_lock); 0
}

pub unsafe fn agp_unbind_memory(curr: *mut agp_memory) -> i32 {
    if curr.is_null() || !(*curr).is_bound { return -EINVAL; }
    let r = ((*(*(*curr).bridge).driver).remove_memory)(curr, (*curr).pg_start, (*curr).type_); if r != 0 { return r; }
    (*curr).is_bound = false; (*curr).pg_start = 0; spin_lock(&(*(*curr).bridge).mapped_lock); list_del(&mut (*curr).mapped_list); spin_unlock(&(*(*curr).bridge).mapped_lock); 0
}

pub unsafe fn agp_num_entries() -> i32 {
    let t = (*agp_bridge).current_size; let mut n = match (*(*agp_bridge).driver).size_type { U8_APER_SIZE => (*A_SIZE_8(t)).num_entries, U16_APER_SIZE => (*A_SIZE_16(t)).num_entries, U32_APER_SIZE => (*A_SIZE_32(t)).num_entries, LVL2_APER_SIZE => (*A_SIZE_LVL2(t)).num_entries, FIXED_APER_SIZE => (*A_SIZE_FIX(t)).num_entries, _ => 0 };
    n -= agp_memory_reserved >> PAGE_SHIFT; if n < 0 { 0 } else { n }
}

pub unsafe fn agp_copy_info(bridge: *mut agp_bridge_data, info: *mut agp_kern_info) -> i32 {
    core::ptr::write_bytes(info, 0, 1); if bridge.is_null() { (*info).chipset = NOT_SUPPORTED; return -EIO; }
    (*info).version.major = (*bridge).version.major; (*info).version.minor = (*bridge).version.minor; (*info).chipset = SUPPORTED; (*info).device = (*bridge).dev;
    (*info).mode = if (*bridge).mode & AGPSTAT_MODE_3_0 != 0 { (*bridge).mode & !AGP3_RESERVED_MASK } else { (*bridge).mode & !AGP2_RESERVED_MASK };
    (*info).aper_base = (*bridge).gart_bus_addr; (*info).aper_size = agp_return_size(); (*info).max_memory = (*bridge).max_memory_agp; (*info).current_memory = atomic_read(&(*bridge).current_memory_agp); (*info).cant_use_aperture = (*(*bridge).driver).cant_use_aperture; (*info).vm_ops = (*bridge).vm_ops; (*info).page_mask = !0usize; 0
}

unsafe fn agp_return_size() -> i32 { let t = (*agp_bridge).current_size; let mut n = match (*(*agp_bridge).driver).size_type { U8_APER_SIZE => (*A_SIZE_8(t)).size, U16_APER_SIZE => (*A_SIZE_16(t)).size, U32_APER_SIZE => (*A_SIZE_32(t)).size, LVL2_APER_SIZE => (*A_SIZE_LVL2(t)).size, FIXED_APER_SIZE => (*A_SIZE_FIX(t)).size, _ => 0 }; n -= agp_memory_reserved / (1024*1024); if n < 0 { 0 } else { n } }

// The remaining routines retain the C control flow and call external kernel
// helpers supplied by the surrounding translation. Macros are represented by
// their direct Rust expressions where applicable.
pub unsafe fn agp_generic_alloc_by_type(_: usize, _: i32) -> *mut agp_memory { core::ptr::null_mut() }
pub unsafe fn agp_generic_free_by_type(curr: *mut agp_memory) { agp_free_page_array(curr); agp_free_key((*curr).key); kfree(curr); }
pub unsafe fn agp_generic_alloc_user(page_count: usize, type_: i32) -> *mut agp_memory { let n=agp_create_user_memory(page_count); if n.is_null(){return n;} for i in 0..page_count { (*n).pages.add(i).write(core::ptr::null_mut()); } (*n).page_count=0; (*n).type_=type_; (*n).num_scratch_pages=((page_count+ENTRIES_PER_PAGE-1)/ENTRIES_PER_PAGE) as i32; n }
pub unsafe fn agp_enable(bridge: *mut agp_bridge_data, mode: u32) { if !bridge.is_null() { ((*(*bridge).driver).agp_enable)(bridge, mode); } }
pub unsafe fn agp_generic_find_bridge(_: *mut pci_dev) -> *mut agp_bridge_data { if list_empty(&agp_bridges) { core::ptr::null_mut() } else { agp_bridge } }
unsafe fn ipi_handler(_: *mut core::ffi::c_void) { flush_agp_cache(); }
pub unsafe fn global_cache_flush() { on_each_cpu(ipi_handler, core::ptr::null_mut(), 1); }
pub unsafe fn agp_generic_mask_memory(bridge: *mut agp_bridge_data, addr: dma_addr_t, _: i32) -> usize { if !(*(*bridge).driver).masks.is_null() { addr | (*(*bridge).driver).masks[0].mask } else { addr } }
pub unsafe fn agp_generic_type_to_mask_type(_: *mut agp_bridge_data, type_: i32) -> i32 { if type_ >= AGP_USER_TYPES { 0 } else { type_ } }

pub static agp3_generic_sizes: [aper_size_info_16; AGP_GENERIC_SIZES_ENTRIES] = [
    aper_size_info_16{size:4096,size_value:1048576,page_order:10,num_entries:0x000}, aper_size_info_16{size:2048,size_value:524288,page_order:9,num_entries:0x800}, aper_size_info_16{size:1024,size_value:262144,page_order:8,num_entries:0xc00}, aper_size_info_16{size:512,size_value:131072,page_order:7,num_entries:0xe00}, aper_size_info_16{size:256,size_value:65536,page_order:6,num_entries:0xf00}, aper_size_info_16{size:128,size_value:32768,page_order:5,num_entries:0xf20}, aper_size_info_16{size:64,size_value:16384,page_order:4,num_entries:0xf30}, aper_size_info_16{size:32,size_value:8192,page_order:3,num_entries:0xf38}, aper_size_info_16{size:16,size_value:4096,page_order:2,num_entries:0xf3c}, aper_size_info_16{size:8,size_value:2048,page_order:1,num_entries:0xf3e}, aper_size_info_16{size:4,size_value:1024,page_order:0,num_entries:0xf3f}
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
