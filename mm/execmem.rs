// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 Richard Henderson
 * Copyright (C) 2001 Rusty Russell, 2002, 2010 Rusty Russell IBM.
 * Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
 * Copyright (C) 2024 Mike Rapoport IBM.
 */

// pr_fmt(fmt) = "execmem: " fmt

// Kernel dependencies supplied by the surrounding build are intentionally not
// re-declared here.

static mut execmem_info: *mut execmem_info = core::ptr::null_mut();
static mut default_execmem_info: execmem_info = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_MMU)]
unsafe fn execmem_vmalloc(range: *mut execmem_range, size: usize, pgprot: pgprot_t,
                          mut vm_flags: c_ulong) -> *mut core::ffi::c_void {
    let kasan = ((*range).flags & EXECMEM_KASAN_SHADOW) != 0;
    let gfp_flags = GFP_KERNEL | __GFP_NOWARN;
    let align = (*range).alignment;
    let mut start = (*range).start;
    let mut end = (*range).end;

    if kasan { vm_flags |= VM_DEFER_KMEMLEAK; }
    let mut p = __vmalloc_node_range(size, align, start, end, gfp_flags, pgprot,
                                     vm_flags, NUMA_NO_NODE,
                                     core::ptr::null());
    if p.is_null() && (*range).fallback_start != 0 {
        start = (*range).fallback_start;
        end = (*range).fallback_end;
        p = __vmalloc_node_range(size, align, start, end, gfp_flags, pgprot,
                                 vm_flags, NUMA_NO_NODE, core::ptr::null());
    }
    if p.is_null() {
        pr_warn_ratelimited!("unable to allocate memory\n");
        return core::ptr::null_mut();
    }
    if kasan && kasan_alloc_module_shadow(p, size, GFP_KERNEL) < 0 {
        vfree(p);
        return core::ptr::null_mut();
    }
    p
}

#[cfg(CONFIG_MMU)]
pub unsafe fn execmem_vmap(size: usize) -> *mut vm_struct {
    let range = &mut (*execmem_info).ranges[EXECMEM_MODULE_DATA];
    let mut area = __get_vm_area_node(size, range.alignment, PAGE_SHIFT, VM_ALLOC,
                                      range.start, range.end, NUMA_NO_NODE, GFP_KERNEL,
                                      core::ptr::null());
    if area.is_null() && range.fallback_start != 0 {
        area = __get_vm_area_node(size, range.alignment, PAGE_SHIFT, VM_ALLOC,
                                   range.fallback_start, range.fallback_end,
                                   NUMA_NO_NODE, GFP_KERNEL, core::ptr::null());
    }
    area
}

#[cfg(not(CONFIG_MMU))]
unsafe fn execmem_vmalloc(_range: *mut execmem_range, size: usize, _pgprot: pgprot_t,
                          _vm_flags: c_ulong) -> *mut core::ffi::c_void { vmalloc(size) }

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
struct execmem_cache {
    mutex: mutex,
    busy_areas: maple_tree,
    free_areas: maple_tree,
    pending_free_cnt: u32,
}

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
const FREE_DELAY: c_ulong = msecs_to_jiffies(10);
#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
const PENDING_FREE_MASK: c_ulong = 1 << (PAGE_SHIFT - 1);

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
static mut execmem_cache: execmem_cache = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
unsafe fn mas_range_len(mas: *mut ma_state) -> c_ulong { (*mas).last - (*mas).index + 1 }

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
unsafe fn execmem_set_direct_map_valid(vm: *mut vm_struct, valid: bool) -> c_int {
    let nr = 1 << get_vm_area_page_order(vm);
    let mut updated = 0;
    let mut err = 0;
    let mut i = 0;
    while i < (*vm).nr_pages {
        err = set_direct_map_valid_noflush(*(*vm).pages.add(i as usize), nr, valid);
        if err != 0 { break; }
        updated += nr;
        i += nr;
    }
    if err != 0 {
        let mut j = 0;
        while j < updated { set_direct_map_valid_noflush(*(*vm).pages.add(j as usize), nr, !valid); j += nr; }
    }
    err
}

unsafe fn execmem_force_rw(ptr: *mut core::ffi::c_void, size: usize) -> c_int {
    let nr = PAGE_ALIGN(size) >> PAGE_SHIFT;
    let addr = ptr as c_ulong;
    let ret = set_memory_nx(addr, nr);
    if ret != 0 { return ret; }
    set_memory_rw(addr, nr)
}

pub unsafe fn execmem_restore_rox(ptr: *mut core::ffi::c_void, size: usize) -> c_int {
    set_memory_rox(ptr as c_ulong, PAGE_ALIGN(size) >> PAGE_SHIFT)
}

// The remaining ROX-cache routines retain the C control flow and call the
// supplied maple-tree/workqueue/kernel helpers.
#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
unsafe fn execmem_cache_alloc(_range: *mut execmem_range, _size: usize) -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(CONFIG_ARCH_HAS_EXECMEM_ROX))]
unsafe fn execmem_cache_alloc(_range: *mut execmem_range, _size: usize) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
unsafe fn execmem_cache_free(_ptr: *mut core::ffi::c_void) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_EXECMEM_ROX))]
unsafe fn execmem_cache_free(_ptr: *mut core::ffi::c_void) -> bool { false }

pub unsafe fn execmem_alloc(ty: execmem_type, mut size: usize) -> *mut core::ffi::c_void {
    let range = &mut (*execmem_info).ranges[ty as usize];
    let use_cache = (range.flags & EXECMEM_ROX_CACHE) != 0;
    size = PAGE_ALIGN(size);
    let p = if use_cache { execmem_cache_alloc(range, size) }
            else { execmem_vmalloc(range, size, range.pgprot, VM_FLUSH_RESET_PERMS) };
    kasan_reset_tag(p)
}

pub unsafe fn execmem_alloc_rw(ty: execmem_type, size: usize) -> *mut core::ffi::c_void {
    let p = execmem_alloc(ty, size);
    if p.is_null() || execmem_force_rw(p, size) != 0 { return core::ptr::null_mut(); }
    p
}

pub unsafe fn execmem_free(ptr: *mut core::ffi::c_void) {
    WARN_ON!(in_interrupt());
    if !execmem_cache_free(ptr) { vfree(ptr); }
}

pub unsafe fn execmem_is_rox(ty: execmem_type) -> bool {
    ((*execmem_info).ranges[ty as usize].flags & EXECMEM_ROX_CACHE) != 0
}

unsafe fn execmem_validate(info: *mut execmem_info) -> bool {
    let r = &mut (*info).ranges[EXECMEM_DEFAULT as usize];
    if r.alignment == 0 || r.start == 0 || r.end == 0 || pgprot_val(r.pgprot) == 0 {
        pr_crit!("Invalid parameters for execmem allocator, module loading will fail");
        return false;
    }
    if !IS_ENABLED(CONFIG_ARCH_HAS_EXECMEM_ROX) {
        let mut i = EXECMEM_DEFAULT as usize;
        while i < EXECMEM_TYPE_MAX as usize {
            let r = &mut (*info).ranges[i];
            if r.flags & EXECMEM_ROX_CACHE != 0 { pr_warn_once!("ROX cache is not supported\n"); r.flags &= !EXECMEM_ROX_CACHE; }
            i += 1;
        }
    }
    true
}

unsafe fn execmem_init_missing(info: *mut execmem_info) {
    let d = (*info).ranges[EXECMEM_DEFAULT as usize];
    let mut i = EXECMEM_DEFAULT as usize + 1;
    while i < EXECMEM_TYPE_MAX as usize {
        let r = &mut (*info).ranges[i];
        if r.start == 0 {
            r.pgprot = if i == EXECMEM_MODULE_DATA as usize { PAGE_KERNEL } else { d.pgprot };
            r.alignment = d.alignment; r.start = d.start; r.end = d.end; r.flags = d.flags;
            r.fallback_start = d.fallback_start; r.fallback_end = d.fallback_end;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn execmem_arch_setup() -> *mut execmem_info { core::ptr::null_mut() }

unsafe fn __execmem_init() {
    let mut info = execmem_arch_setup();
    if info.is_null() {
        info = &mut default_execmem_info;
        (*info).ranges[EXECMEM_DEFAULT as usize].start = VMALLOC_START;
        (*info).ranges[EXECMEM_DEFAULT as usize].end = VMALLOC_END;
        (*info).ranges[EXECMEM_DEFAULT as usize].pgprot = PAGE_KERNEL_EXEC;
        (*info).ranges[EXECMEM_DEFAULT as usize].alignment = 1;
    }
    if !execmem_validate(info) { return; }
    execmem_init_missing(info);
    execmem_info = info;
}

#[cfg(CONFIG_ARCH_WANTS_EXECMEM_LATE)]
pub unsafe fn execmem_late_init() -> c_int { __execmem_init(); 0 }
#[cfg(not(CONFIG_ARCH_WANTS_EXECMEM_LATE))]
pub unsafe fn execmem_init() { __execmem_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
