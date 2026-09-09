// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
struct amdgpu_vm_pt_cursor {
    pfn: u64,
    parent: *mut amdgpu_vm_bo_base,
    entry: *mut amdgpu_vm_bo_base,
    level: u32,
}

unsafe fn amdgpu_vm_pt_level_shift(adev: *mut amdgpu_device, level: u32) -> u32 {
    match level {
        AMDGPU_VM_PDB3 | AMDGPU_VM_PDB2 | AMDGPU_VM_PDB1 | AMDGPU_VM_PDB0 =>
            9 * (AMDGPU_VM_PDB0 - level) + (*adev).vm_manager.block_size,
        AMDGPU_VM_PTB => 0,
        _ => !0,
    }
}

unsafe fn amdgpu_vm_pt_num_entries(adev: *mut amdgpu_device, level: u32) -> u32 {
    let shift = amdgpu_vm_pt_level_shift(adev, (*adev).vm_manager.root_level);
    if level == (*adev).vm_manager.root_level {
        round_up((*adev).vm_manager.max_pfn, 1u64 << shift) >> shift
    } else if level != AMDGPU_VM_PTB { 512 } else { AMDGPU_VM_PTE_COUNT(adev) }
}

unsafe fn amdgpu_vm_pt_entries_mask(adev: *mut amdgpu_device, level: u32) -> u32 {
    if level <= (*adev).vm_manager.root_level { 0xffffffff }
    else if level != AMDGPU_VM_PTB { 0x1ff } else { AMDGPU_VM_PTE_COUNT(adev) - 1 }
}

unsafe fn amdgpu_vm_pt_size(adev: *mut amdgpu_device, level: u32) -> u32 {
    AMDGPU_GPU_PAGE_ALIGN(amdgpu_vm_pt_num_entries(adev, level) * 8)
}

unsafe fn amdgpu_vm_pt_parent(pt: *mut amdgpu_vm_bo_base) -> *mut amdgpu_vm_bo_base {
    let parent = (*(*pt).bo).parent;
    if parent.is_null() { core::ptr::null_mut() } else { (*parent).vm_bo }
}

unsafe fn amdgpu_vm_pt_start(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, start: u64, cursor: *mut amdgpu_vm_pt_cursor) {
    (*cursor).pfn = start; (*cursor).parent = core::ptr::null_mut();
    (*cursor).entry = &mut (*vm).root; (*cursor).level = (*adev).vm_manager.root_level;
}

unsafe fn amdgpu_vm_pt_descendant(adev: *mut amdgpu_device, cursor: *mut amdgpu_vm_pt_cursor) -> bool {
    if (*cursor).level == AMDGPU_VM_PTB || (*cursor).entry.is_null() || (*(*cursor).entry).bo.is_null() { return false; }
    let mask = amdgpu_vm_pt_entries_mask(adev, (*cursor).level);
    let shift = amdgpu_vm_pt_level_shift(adev, (*cursor).level);
    (*cursor).level += 1;
    let idx = ((*cursor).pfn >> shift) & mask as u64;
    (*cursor).parent = (*cursor).entry;
    (*cursor).entry = &mut (*to_amdgpu_bo_vm((*(*cursor).entry).bo)).entries[idx as usize]; true
}

unsafe fn amdgpu_vm_pt_sibling(adev: *mut amdgpu_device, cursor: *mut amdgpu_vm_pt_cursor) -> bool {
    if (*cursor).parent.is_null() { return false; }
    let shift = amdgpu_vm_pt_level_shift(adev, (*cursor).level - 1);
    let num_entries = amdgpu_vm_pt_num_entries(adev, (*cursor).level - 1);
    let parent = to_amdgpu_bo_vm((*(*cursor).parent).bo);
    if (*cursor).entry == (*parent).entries.as_mut_ptr().add(num_entries as usize - 1) { return false; }
    (*cursor).pfn += 1u64 << shift; (*cursor).pfn &= !((1u64 << shift) - 1); (*cursor).entry = (*cursor).entry.add(1); true
}

unsafe fn amdgpu_vm_pt_ancestor(cursor: *mut amdgpu_vm_pt_cursor) -> bool {
    if (*cursor).parent.is_null() { return false; }
    (*cursor).level -= 1; (*cursor).entry = (*cursor).parent;
    (*cursor).parent = amdgpu_vm_pt_parent((*cursor).parent); true
}

unsafe fn amdgpu_vm_pt_next(adev: *mut amdgpu_device, cursor: *mut amdgpu_vm_pt_cursor) {
    if amdgpu_vm_pt_descendant(adev, cursor) { return; }
    while !amdgpu_vm_pt_sibling(adev, cursor) {
        if !amdgpu_vm_pt_ancestor(cursor) { (*cursor).pfn = !0u64; return; }
    }
}

unsafe fn amdgpu_vm_pt_first_dfs(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, start: *mut amdgpu_vm_pt_cursor, cursor: *mut amdgpu_vm_pt_cursor) {
    if !start.is_null() { *cursor = *start; } else { amdgpu_vm_pt_start(adev, vm, 0, cursor); }
    while amdgpu_vm_pt_descendant(adev, cursor) {}
}

unsafe fn amdgpu_vm_pt_continue_dfs(start: *mut amdgpu_vm_pt_cursor, entry: *mut amdgpu_vm_bo_base) -> bool {
    !entry.is_null() && (start.is_null() || entry != (*start).entry)
}

unsafe fn amdgpu_vm_pt_next_dfs(adev: *mut amdgpu_device, cursor: *mut amdgpu_vm_pt_cursor) {
    if (*cursor).entry.is_null() { return; }
    if (*cursor).parent.is_null() { (*cursor).entry = core::ptr::null_mut(); }
    else if amdgpu_vm_pt_sibling(adev, cursor) { while amdgpu_vm_pt_descendant(adev, cursor) {} }
    else { amdgpu_vm_pt_ancestor(cursor); }
}

pub unsafe fn amdgpu_vm_pt_clear(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, vmbo: *mut amdgpu_bo_vm, immediate: bool) -> i32 {
    let mut level = (*adev).vm_manager.root_level;
    let mut ctx = ttm_operation_ctx { interruptible: true, no_wait_gpu: false };
    let mut ancestor = &mut (*vmbo).bo as *mut amdgpu_bo;
    let entries = amdgpu_bo_size(&(*vmbo).bo) / 8;
    let bo = &mut (*vmbo).bo as *mut amdgpu_bo;
    let mut value = 0u64; let mut flags = 0u64; let mut r;
    if !(*ancestor).parent.is_null() { level += 1; while !(*(*ancestor).parent).parent.is_null() { level += 1; ancestor = (*ancestor).parent; } }
    r = ttm_bo_validate(&mut (*bo).tbo, &(*bo).placement, &mut ctx); if r != 0 { return r; }
    let mut idx = 0; if !drm_dev_enter(adev_to_drm(adev), &mut idx) { return -ENODEV; }
    r = (*(*vm).update_funcs).map_table(vmbo); if r != 0 { drm_dev_exit(idx); return r; }
    let mut params: amdgpu_vm_update_params = core::mem::zeroed(); params.adev = adev; params.vm = vm; params.immediate = immediate;
    r = (*(*vm).update_funcs).prepare(&mut params, core::ptr::null_mut(), AMDGPU_KERNEL_JOB_ID_VM_PT_CLEAR); if r != 0 { drm_dev_exit(idx); return r; }
    if (*adev).asic_type >= CHIP_VEGA10 { if level != AMDGPU_VM_PTB { flags |= AMDGPU_PDE_PTE_FLAG(adev); amdgpu_gmc_get_vm_pde(adev, level, &mut value, &mut flags); } else { flags = AMDGPU_PTE_EXECUTABLE | (*adev).gmc.init_pte_flags; } }
    r = (*(*vm).update_funcs).update(&mut params, vmbo, 0, 0, entries, value, flags); if r == 0 { r = (*(*vm).update_funcs).commit(&mut params, core::ptr::null_mut()); }
    drm_dev_exit(idx); r
}

pub unsafe fn amdgpu_vm_pt_create(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, level: i32, immediate: bool, vmbo: *mut *mut amdgpu_bo_vm, xcp_id: i32) -> i32 {
    let mut bp: amdgpu_bo_param = core::mem::zeroed();
    bp.size = amdgpu_vm_pt_size(adev, level as u32); bp.byte_align = AMDGPU_GPU_PAGE_SIZE;
    bp.domain = if !(*adev).gmc.is_app_apu { AMDGPU_GEM_DOMAIN_VRAM } else { AMDGPU_GEM_DOMAIN_GTT };
    bp.domain = amdgpu_bo_get_preferred_domain(adev, bp.domain);
    bp.flags = AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS | AMDGPU_GEM_CREATE_CPU_GTT_USWC;
    let num_entries = if level < AMDGPU_VM_PTB as i32 { amdgpu_vm_pt_num_entries(adev, level as u32) } else { 0 };
    bp.bo_ptr_size = struct_size!((*vmbo), entries, num_entries);
    if (*vm).use_cpu_for_update { bp.flags |= AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED; }
    bp.r#type = ttm_bo_type_kernel; bp.no_wait_gpu = immediate; bp.xcp_id_plus1 = xcp_id + 1;
    if !(*vm).root.bo.is_null() { bp.resv = (*(*vm).root.bo).tbo.base.resv; }
    amdgpu_bo_create_vm(adev, &mut bp, vmbo)
}

unsafe fn amdgpu_vm_pt_alloc(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, cursor: *mut amdgpu_vm_pt_cursor, immediate: bool) -> i32 {
    let entry = (*cursor).entry; if !(*entry).bo.is_null() { return 0; }
    amdgpu_vm_eviction_unlock(vm);
    let mut pt = core::ptr::null_mut(); let r = amdgpu_vm_pt_create(adev, vm, (*cursor).level as i32, immediate, &mut pt, (*(*vm).root.bo).xcp_id);
    amdgpu_vm_eviction_lock(vm); if r != 0 { return r; }
    (*pt).bo.parent = amdgpu_bo_ref((*(*cursor).parent).bo); amdgpu_vm_bo_base_init(entry, vm, &mut (*pt).bo);
    let r = amdgpu_vm_pt_clear(adev, vm, pt, immediate); if r != 0 { amdgpu_bo_unref(&mut (*pt).bo); } r
}

unsafe fn amdgpu_vm_pt_free(entry: *mut amdgpu_vm_bo_base) {
    if (*entry).bo.is_null() { return; }
    amdgpu_vm_update_stats(entry, (*(*entry).bo).tbo.resource, -1); (*(*entry).bo).vm_bo = core::ptr::null_mut();
    ttm_bo_set_bulk_move(&mut (*(*entry).bo).tbo, core::ptr::null_mut()); list_del(&mut (*entry).vm_status); amdgpu_bo_unref(&mut (*entry).bo);
}

pub unsafe fn amdgpu_vm_pt_free_list(_adev: *mut amdgpu_device, params: *mut amdgpu_vm_update_params) {
    if list_empty(&(*params).tlb_flush_waitlist) { return; }
    WARN_ON((*params).unlocked);
    let mut entry = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
    list_for_each_entry_safe!(entry, next, &mut (*params).tlb_flush_waitlist, vm_status) { amdgpu_vm_pt_free(entry); }
}

pub unsafe fn amdgpu_vm_pt_free_root(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) {
    let mut cursor: amdgpu_vm_pt_cursor = core::mem::zeroed(); amdgpu_vm_pt_first_dfs(adev, vm, core::ptr::null_mut(), &mut cursor);
    while !cursor.entry.is_null() { amdgpu_vm_pt_free(cursor.entry); amdgpu_vm_pt_next_dfs(adev, &mut cursor); }
}

pub unsafe fn amdgpu_vm_pde_update(params: *mut amdgpu_vm_update_params, entry: *mut amdgpu_vm_bo_base) -> i32 {
    let parent = amdgpu_vm_pt_parent(entry); if WARN_ON(parent.is_null()) { return -EINVAL; }
    let bo = (*parent).bo; let mut pbo = (*bo).parent; let mut level = 0; while !pbo.is_null() { level += 1; pbo = (*pbo).parent; }
    level += (*(*params).adev).vm_manager.root_level; let mut pt=0; let mut flags=0; amdgpu_gmc_get_pde_for_bo((*entry).bo, level, &mut pt, &mut flags);
    let pde = entry.offset_from((*to_amdgpu_bo_vm(bo)).entries.as_mut_ptr()) as u64 * 8;
    (*(*(*params).vm).update_funcs).update(params, to_amdgpu_bo_vm(bo), pde, pt, 1, 0, flags)
}

pub unsafe fn amdgpu_vm_pt_map_tables(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> i32 {
    let mut c: amdgpu_vm_pt_cursor = core::mem::zeroed(); amdgpu_vm_pt_first_dfs(adev, vm, core::ptr::null_mut(), &mut c);
    while !c.entry.is_null() { if !(*c.entry).bo.is_null() { let r=(*(*vm).update_funcs).map_table(to_amdgpu_bo_vm((*c.entry).bo)); if r != 0 { return r; } } amdgpu_vm_pt_next_dfs(adev, &mut c); } 0
}

unsafe fn amdgpu_vm_pte_update_noretry_flags(adev: *mut amdgpu_device, flags: *mut u64) {
    if (*flags & AMDGPU_VM_NORETRY_FLAGS) == AMDGPU_VM_NORETRY_FLAGS { *flags &= !AMDGPU_VM_NORETRY_FLAGS; *flags |= (*adev).gmc.noretry_flags; }
}

unsafe fn amdgpu_vm_pte_update_flags(params: *mut amdgpu_vm_update_params, pt: *mut amdgpu_bo_vm, level: u32, pe: u64, mut addr: u64, count: u32, incr: u32, mut flags: u64) {
    let adev=(*params).adev;
    if level != AMDGPU_VM_PTB { flags |= AMDGPU_PDE_PTE_FLAG(adev); amdgpu_gmc_get_vm_pde(adev, level, &mut addr, &mut flags); }
    else if (*adev).asic_type >= CHIP_VEGA10 && flags & AMDGPU_PTE_VALID == 0 && flags & AMDGPU_PTE_PRT_FLAG(adev) == 0 { flags |= AMDGPU_PTE_EXECUTABLE | (*adev).gmc.init_pte_flags; }
    if level == AMDGPU_VM_PTB { amdgpu_vm_pte_update_noretry_flags(adev, &mut flags); }
    (*(*(*params).vm).update_funcs).update(params, pt, pe, addr, count, incr, flags);
}

unsafe fn amdgpu_vm_pte_fragment(params: *mut amdgpu_vm_update_params, start: u64, end: u64, _flags: u64, frag: *mut u32, frag_end: *mut u64) {
    let max_frag=if (*(*params).adev).asic_type < CHIP_VEGA10 { (*(*params).adev).vm_manager.fragment_size } else { 31 };
    if !(*params).pages_addr.is_null() { *frag=0; *frag_end=end; return; }
    *frag=min_t!(u32, ffs(start)-1, fls64(end-start)-1);
    if *frag >= max_frag { *frag=max_frag; *frag_end=end & !((1u64<<max_frag)-1); } else { *frag_end=start+(1u64<<*frag); }
}

pub unsafe fn amdgpu_vm_ptes_update(params: *mut amdgpu_vm_update_params, start: u64, end: u64, mut dst: u64, flags: u64) -> i32 {
    let adev=(*params).adev; let mut cursor: amdgpu_vm_pt_cursor=core::mem::zeroed(); amdgpu_vm_pt_start(adev,(*params).vm,start,&mut cursor);
    let mut frag_start=start; let mut frag=0; let mut frag_end=0; amdgpu_vm_pte_fragment(params,start,end,flags,&mut frag,&mut frag_end);
    while cursor.pfn < end {
        let shift=amdgpu_vm_pt_level_shift(adev,cursor.level); let parent_shift=amdgpu_vm_pt_level_shift(adev,cursor.level-1);
        if !(*params).unlocked { let r=amdgpu_vm_pt_alloc(adev,(*params).vm,&mut cursor,(*params).immediate); if r!=0{return r;} }
        if (*params).unlocked { if amdgpu_vm_pt_descendant(adev,&mut cursor){continue;} }
        else if frag < shift { if amdgpu_vm_pt_descendant(adev,&mut cursor){continue;} }
        else if frag >= parent_shift { if !amdgpu_vm_pt_ancestor(&mut cursor){return -EINVAL;} continue; }
        let pt=(*cursor.entry).bo; if pt.is_null() { if flags&AMDGPU_PTE_VALID!=0{return -ENOENT;} if !amdgpu_vm_pt_ancestor(&mut cursor){return -EINVAL;} }
        let incr=(AMDGPU_GPU_PAGE_SIZE as u64)<<shift; let mask=amdgpu_vm_pt_entries_mask(adev,cursor.level); let mut pe=((cursor.pfn>>shift)&mask as u64)*8; let entry_end=min!(((mask as u64+1)<<shift)+(cursor.pfn&!(((mask as u64+1)<<shift)-1)),end);
        while frag_start < entry_end { let upd_end=min!(entry_end,frag_end); let nptes=max!((upd_end-frag_start)>>shift,1); amdgpu_vm_pte_update_flags(params,to_amdgpu_bo_vm((*cursor.entry).bo),cursor.level,pe,dst,nptes as u32,incr as u32,flags|AMDGPU_PTE_FRAG(frag)); pe+=nptes*8; dst+=nptes*incr; frag_start=upd_end; if frag_start>=frag_end {amdgpu_vm_pte_fragment(params,frag_start,end,flags,&mut frag,&mut frag_end); if frag<shift{break;}} }
        amdgpu_vm_pt_next(adev,&mut cursor);
    } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
