// SPDX-License-Identifier: GPL-2.0
/* Common KASAN code. Translated directly from common.c. */

#[cfg(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS))]
pub static mut kasan_flag_enabled: bool = false;

pub unsafe fn kasan_addr_to_slab(addr: *const core::ffi::c_void) -> *mut slab {
    if virt_addr_valid(addr) { virt_to_slab(addr) } else { core::ptr::null_mut() }
}

pub unsafe fn kasan_save_stack(flags: gfp_t, depot_flags: depot_flags_t) -> depot_stack_handle_t {
    let mut entries: [c_ulong; KASAN_STACK_DEPTH as usize] = [0; KASAN_STACK_DEPTH as usize];
    let nr_entries = stack_trace_save(entries.as_mut_ptr(), entries.len(), 0);
    stack_depot_save_flags(entries.as_ptr(), nr_entries, flags, depot_flags)
}

pub unsafe fn kasan_set_track(track: *mut kasan_track, stack: depot_stack_handle_t) {
    #[cfg(CONFIG_KASAN_EXTRA_INFO)]
    {
        let cpu: u32 = raw_smp_processor_id();
        let ts_nsec: u64 = local_clock();
        (*track).cpu = cpu;
        (*track).timestamp = ts_nsec >> 9;
    }
    (*track).pid = (*current).pid;
    (*track).stack = stack;
}

pub unsafe fn kasan_save_track(track: *mut kasan_track, flags: gfp_t) {
    let stack = kasan_save_stack(flags, STACK_DEPOT_FLAG_CAN_ALLOC);
    kasan_set_track(track, stack);
}

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
pub unsafe fn kasan_enable_current() { (*current).kasan_depth += 1; }

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
pub unsafe fn kasan_disable_current() { (*current).kasan_depth -= 1; }

pub unsafe fn __kasan_unpoison_range(address: *const core::ffi::c_void, size: usize) {
    if is_kfence_address(address) { return; }
    kasan_unpoison(address, size, false);
}

#[cfg(CONFIG_KASAN_STACK)]
pub unsafe fn kasan_unpoison_task_stack(task: *mut task_struct) {
    kasan_unpoison(task_stack_page(task), THREAD_SIZE, false);
}

#[cfg(CONFIG_KASAN_STACK)]
pub unsafe fn kasan_unpoison_task_stack_below(watermark: *const core::ffi::c_void) {
    let base = ((watermark as usize) & !(THREAD_SIZE - 1)) as *mut core::ffi::c_void;
    kasan_unpoison(base, watermark.offset_from(base) as usize, false);
}

pub unsafe fn __kasan_unpoison_pages(page: *mut page, order: c_uint, init: bool) -> bool {
    if unlikely(PageHighMem(page)) || !kasan_sample_page_alloc(order) { return false; }
    let tag = kasan_random_tag();
    kasan_unpoison(set_tag(page_address(page), tag), PAGE_SIZE << order, init);
    for i in 0..(1usize << order) { page_kasan_tag_set(page.add(i), tag); }
    true
}

pub unsafe fn __kasan_poison_pages(page: *mut page, order: c_uint, init: bool) {
    if likely(!PageHighMem(page)) { kasan_poison(page_address(page), PAGE_SIZE << order, KASAN_PAGE_FREE, init); }
}

pub unsafe fn __kasan_poison_slab(slab_: *mut slab) {
    let page = slab_page(slab_);
    for i in 0..compound_nr(page) { page_kasan_tag_reset(page.add(i)); }
    kasan_poison(page_address(page), page_size(page), KASAN_SLAB_REDZONE, false);
}

pub unsafe fn __kasan_unpoison_new_object(cache: *mut kmem_cache, object: *mut core::ffi::c_void) {
    kasan_unpoison(object, (*cache).object_size, false);
}

pub unsafe fn __kasan_poison_new_object(cache: *mut kmem_cache, object: *mut core::ffi::c_void) {
    kasan_poison(object, round_up((*cache).object_size, KASAN_GRANULE_SIZE), KASAN_SLAB_REDZONE, false);
}

unsafe fn assign_tag(cache: *mut kmem_cache, object: *const core::ffi::c_void, init: bool) -> u8 {
    if cfg!(CONFIG_KASAN_GENERIC) { return 0xff; }
    if (*cache).ctor.is_null() && ((*cache).flags & SLAB_TYPESAFE_BY_RCU) == 0 {
        return if init { KASAN_TAG_KERNEL } else { kasan_random_tag() };
    }
    if init { kasan_random_tag() } else { get_tag(object) }
}

pub unsafe fn __kasan_init_slab_obj(cache: *mut kmem_cache, object: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    if kasan_requires_meta() { kasan_init_object_meta(cache, object); }
    set_tag(object, assign_tag(cache, object, true)) as *mut core::ffi::c_void
}

unsafe fn check_slab_allocation(cache: *mut kmem_cache, mut object: *mut core::ffi::c_void, ip: c_ulong) -> bool {
    let tagged_object = object;
    object = kasan_reset_tag(object);
    if unlikely(nearest_obj(cache, virt_to_slab(object), object) != object) {
        kasan_report_invalid_free(tagged_object, ip, KASAN_REPORT_INVALID_FREE); return true;
    }
    if !kasan_byte_accessible(tagged_object) {
        kasan_report_invalid_free(tagged_object, ip, KASAN_REPORT_DOUBLE_FREE); return true;
    }
    false
}

unsafe fn poison_slab_object(cache: *mut kmem_cache, mut object: *mut core::ffi::c_void, init: bool) {
    let tagged_object = object; object = kasan_reset_tag(object);
    kasan_poison(object, round_up((*cache).object_size, KASAN_GRANULE_SIZE), KASAN_SLAB_FREE, init);
    if kasan_stack_collection_enabled() { kasan_save_free_info(cache, tagged_object); }
}

pub unsafe fn __kasan_slab_pre_free(cache: *mut kmem_cache, object: *mut core::ffi::c_void, ip: c_ulong) -> bool {
    if is_kfence_address(object) { false } else { check_slab_allocation(cache, object, ip) }
}

pub unsafe fn __kasan_slab_free(cache: *mut kmem_cache, object: *mut core::ffi::c_void, init: bool, still_accessible: bool, no_quarantine: bool) -> bool {
    if is_kfence_address(object) || still_accessible { return false; }
    poison_slab_object(cache, object, init);
    if no_quarantine { return false; }
    kasan_quarantine_put(cache, object)
}

unsafe fn check_page_allocation(ptr: *mut core::ffi::c_void, ip: c_ulong) -> bool {
    if ptr != page_address(virt_to_head_page(ptr)) { kasan_report_invalid_free(ptr, ip, KASAN_REPORT_INVALID_FREE); return true; }
    if !kasan_byte_accessible(ptr) { kasan_report_invalid_free(ptr, ip, KASAN_REPORT_DOUBLE_FREE); return true; }
    false
}

pub unsafe fn __kasan_kfree_large(ptr: *mut core::ffi::c_void, ip: c_ulong) { check_page_allocation(ptr, ip); }

unsafe fn unpoison_slab_object(cache: *mut kmem_cache, object: *mut core::ffi::c_void, flags: gfp_t, init: bool) {
    kasan_unpoison(object, (*cache).object_size, init);
    if kasan_stack_collection_enabled() && !is_kmalloc_cache(cache) { kasan_save_alloc_info(cache, object, flags); }
}

pub unsafe fn __kasan_slab_alloc(cache: *mut kmem_cache, object: *mut core::ffi::c_void, flags: gfp_t, init: bool) -> *mut core::ffi::c_void {
    if gfpflags_allow_blocking(flags) { kasan_quarantine_reduce(); }
    if object.is_null() || is_kfence_address(object) { return object; }
    let tagged_object = set_tag(object, assign_tag(cache, object, false));
    unpoison_slab_object(cache, tagged_object, flags, init); tagged_object
}

unsafe fn poison_kmalloc_redzone(cache: *mut kmem_cache, object: *const core::ffi::c_void, size: usize, flags: gfp_t) {
    if cfg!(CONFIG_KASAN_GENERIC) { kasan_poison_last_granule(object, size); }
    let start = round_up((object as usize) + size, KASAN_GRANULE_SIZE);
    let end = round_up((object as usize) + (*cache).object_size, KASAN_GRANULE_SIZE);
    kasan_poison(start as *mut _, end - start, KASAN_SLAB_REDZONE, false);
    if kasan_stack_collection_enabled() && is_kmalloc_cache(cache) { kasan_save_alloc_info(cache, object as *mut _, flags); }
}

pub unsafe fn __kasan_kmalloc(cache: *mut kmem_cache, object: *const core::ffi::c_void, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    if gfpflags_allow_blocking(flags) { kasan_quarantine_reduce(); }
    if object.is_null() || is_kfence_address(object) { return object as *mut _; }
    poison_kmalloc_redzone(cache, object, size, flags); object as *mut _
}

unsafe fn poison_kmalloc_large_redzone(ptr: *const core::ffi::c_void, size: usize, _flags: gfp_t) {
    if cfg!(CONFIG_KASAN_GENERIC) { kasan_poison_last_granule(ptr, size); }
    let start = round_up(ptr as usize + size, KASAN_GRANULE_SIZE);
    let end = ptr as usize + page_size(virt_to_page(ptr));
    kasan_poison(start as *mut _, end - start, KASAN_PAGE_REDZONE, false);
}

pub unsafe fn __kasan_kmalloc_large(ptr: *const core::ffi::c_void, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    if gfpflags_allow_blocking(flags) { kasan_quarantine_reduce(); }
    if ptr.is_null() { return core::ptr::null_mut(); }
    poison_kmalloc_large_redzone(ptr, size, flags); ptr as *mut _
}

pub unsafe fn __kasan_krealloc(object: *const core::ffi::c_void, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    if gfpflags_allow_blocking(flags) { kasan_quarantine_reduce(); }
    if object == ZERO_SIZE_PTR { return object as *mut _; }
    if is_kfence_address(object) { return object as *mut _; }
    kasan_unpoison(object, size, false);
    let slab_ = virt_to_slab(object);
    if slab_.is_null() { poison_kmalloc_large_redzone(object, size, flags); }
    else { poison_kmalloc_redzone((*slab_).slab_cache, object, size, flags); }
    object as *mut _
}

pub unsafe fn __kasan_mempool_poison_pages(page: *mut page, order: c_uint, ip: c_ulong) -> bool {
    if unlikely(PageHighMem(page)) || (!cfg!(CONFIG_KASAN_GENERIC) && page_kasan_tag(page) == KASAN_TAG_KERNEL) { return true; }
    let ptr = page_address(page);
    if check_page_allocation(ptr, ip) { return false; }
    kasan_poison(ptr, PAGE_SIZE << order, KASAN_PAGE_FREE, false); true
}

pub unsafe fn __kasan_mempool_unpoison_pages(page: *mut page, order: c_uint, _ip: c_ulong) { __kasan_unpoison_pages(page, order, false); }

pub unsafe fn __kasan_mempool_poison_object(ptr: *mut core::ffi::c_void, ip: c_ulong) -> bool {
    let page_ = virt_to_page(ptr);
    if unlikely(PageLargeKmalloc(page_)) { if check_page_allocation(ptr, ip) { return false; } kasan_poison(ptr, page_size(page_), KASAN_PAGE_FREE, false); return true; }
    if is_kfence_address(ptr) { return true; }
    let slab_ = page_slab(page_);
    if check_slab_allocation((*slab_).slab_cache, ptr, ip) { return false; }
    poison_slab_object((*slab_).slab_cache, ptr, false); true
}

pub unsafe fn __kasan_mempool_unpoison_object(ptr: *mut core::ffi::c_void, size: usize, _ip: c_ulong) {
    let flags: gfp_t = 0; let slab_ = virt_to_slab(ptr);
    if slab_.is_null() { kasan_unpoison(ptr, size, false); poison_kmalloc_large_redzone(ptr, size, flags); return; }
    if is_kfence_address(ptr) { return; }
    unpoison_slab_object((*slab_).slab_cache, ptr, flags, false);
    if is_kmalloc_cache((*slab_).slab_cache) { poison_kmalloc_redzone((*slab_).slab_cache, ptr, size, flags); }
}

pub unsafe fn __kasan_check_byte(address: *const core::ffi::c_void, ip: c_ulong) -> bool {
    if !kasan_byte_accessible(address) { kasan_report(address, 1, false, ip); false } else { true }
}

#[cfg(CONFIG_KASAN_VMALLOC)]
pub unsafe fn __kasan_unpoison_vmap_areas(vms: *mut *mut vm_struct, nr_vms: c_int, flags: kasan_vmalloc_flags_t) {
    if WARN_ON_ONCE(flags & KASAN_VMALLOC_KEEP_TAG != 0) { return; }
    let first = *vms; let size = (*first).size; let addr = (*first).addr;
    (*first).addr = __kasan_unpoison_vmalloc(addr, size, flags);
    let tag = get_tag((*first).addr);
    for area in 1..nr_vms { let vm = *vms.add(area as usize); let addr = set_tag((*vm).addr, tag); (*vm).addr = __kasan_unpoison_vmalloc(addr, (*vm).size, flags | KASAN_VMALLOC_KEEP_TAG); }
}

#[cfg(CONFIG_KASAN_VMALLOC)]
pub unsafe fn __kasan_vrealloc(addr: *const core::ffi::c_void, mut old_size: c_ulong, mut new_size: c_ulong) {
    if new_size < old_size { kasan_poison_last_granule(addr, new_size as usize); new_size = round_up(new_size as usize, KASAN_GRANULE_SIZE) as c_ulong; old_size = round_up(old_size as usize, KASAN_GRANULE_SIZE) as c_ulong; if new_size < old_size { __kasan_poison_vmalloc(addr.add(new_size as usize), (old_size - new_size) as usize); } }
    else if new_size > old_size { old_size = round_down(old_size as usize, KASAN_GRANULE_SIZE) as c_ulong; __kasan_unpoison_vmalloc(addr.add(old_size as usize), (new_size - old_size) as usize, KASAN_VMALLOC_PROT_NORMAL | KASAN_VMALLOC_VM_ALLOC | KASAN_VMALLOC_KEEP_TAG); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
