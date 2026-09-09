// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2016-2021 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

unsafe fn encaps_handle_do_release(
    handle: *mut hl_cs_encaps_sig_handle,
    put_hw_sob: bool,
    put_ctx: bool,
) {
    let mgr = unsafe { &mut (*(*handle).ctx).sig_mgr };

    if put_hw_sob {
        unsafe { hw_sob_put((*handle).hw_sob) };
    }

    unsafe { spin_lock(&mut mgr.lock) };
    unsafe { idr_remove(&mut mgr.handles, (*handle).id) };
    unsafe { spin_unlock(&mut mgr.lock) };

    if put_ctx {
        unsafe { hl_ctx_put((*handle).ctx) };
    }

    unsafe { kfree(handle as *mut core::ffi::c_void) };
}

pub unsafe extern "C" fn hl_encaps_release_handle_and_put_ctx(ref_: *mut kref) {
    let handle = unsafe { container_of(ref_, core::mem::offset_of!(hl_cs_encaps_sig_handle, refcount)) };
    unsafe { encaps_handle_do_release(handle, false, true) };
}

unsafe extern "C" fn hl_encaps_release_handle_and_put_sob(ref_: *mut kref) {
    let handle = unsafe { container_of(ref_, core::mem::offset_of!(hl_cs_encaps_sig_handle, refcount)) };
    unsafe { encaps_handle_do_release(handle, true, false) };
}

pub unsafe extern "C" fn hl_encaps_release_handle_and_put_sob_ctx(ref_: *mut kref) {
    let handle = unsafe { container_of(ref_, core::mem::offset_of!(hl_cs_encaps_sig_handle, refcount)) };
    unsafe { encaps_handle_do_release(handle, true, true) };
}

unsafe fn hl_encaps_sig_mgr_init(mgr: *mut hl_encaps_signals_mgr) {
    unsafe { spin_lock_init(&mut (*mgr).lock) };
    unsafe { idr_init(&mut (*mgr).handles) };
}

unsafe fn hl_encaps_sig_mgr_fini(hdev: *mut hl_device, mgr: *mut hl_encaps_signals_mgr) {
    let idp = unsafe { &mut (*mgr).handles };

    /* The IDR is expected to be empty at this stage, because any left signal should have been
     * released as part of CS roll-back.
     */
    if unsafe { !idr_is_empty(idp) } {
        unsafe { dev_warn((*hdev).dev, c"device released while some encaps signals handles are still allocated\n") };
        let mut id: u32 = 0;
        let mut handle: *mut hl_cs_encaps_sig_handle = core::ptr::null_mut();
        unsafe {
            idr_for_each_entry(idp, &mut handle, &mut id);
            kref_put(&mut (*handle).refcount, hl_encaps_release_handle_and_put_sob);
        }
    }

    unsafe { idr_destroy(idp) };
}

unsafe fn hl_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = unsafe { (*ctx).hdev };

    /* Release all allocated HW block mapped list entries and destroy
     * the mutex.
     */
    unsafe { hl_hw_block_mem_fini(ctx) };

    for i in 0..unsafe { (*hdev).asic_prop.max_pending_cs } {
        unsafe { hl_fence_put((*ctx).cs_pending.add(i as usize).read()) };
    }

    unsafe { kfree((*ctx).cs_pending as *mut core::ffi::c_void) };

    if unsafe { (*ctx).asid != HL_KERNEL_ASID_ID } {
        unsafe { dev_dbg((*hdev).dev, c"closing user context, asid=%u\n", (*ctx).asid) };

        /* The engines are stopped as there is no executing CS, but the
         * Coresight might be still working by accessing addresses
         * related to the stopped engines. Hence stop it explicitly.
         */
        if unsafe { (*hdev).in_debug } {
            unsafe { hl_device_set_debug_mode(hdev, ctx, false) };
        }

        unsafe { ((*hdev).asic_funcs).as_ref().unwrap().ctx_fini(ctx) };
        unsafe { hl_dec_ctx_fini(ctx) };
        unsafe { hl_cb_va_pool_fini(ctx) };
        unsafe { hl_vm_ctx_fini(ctx) };
        unsafe { hl_asid_free(hdev, (*ctx).asid) };
        unsafe { hl_encaps_sig_mgr_fini(hdev, &mut (*ctx).sig_mgr) };
        unsafe { mutex_destroy(&mut (*ctx).ts_reg_lock) };
    } else {
        unsafe { dev_dbg((*hdev).dev, c"closing kernel context\n") };
        unsafe { ((*hdev).asic_funcs).as_ref().unwrap().ctx_fini(ctx) };
        unsafe { hl_vm_ctx_fini(ctx) };
        unsafe { hl_mmu_ctx_fini(ctx) };
    }
}

pub unsafe extern "C" fn hl_ctx_do_release(ref_: *mut kref) {
    let ctx = unsafe { container_of(ref_, core::mem::offset_of!(hl_ctx, refcount)) };
    unsafe { hl_ctx_fini(ctx) };

    if unsafe { !(*ctx).hpriv.is_null() } {
        let hpriv = unsafe { (*ctx).hpriv };
        unsafe { mutex_lock(&mut (*hpriv).ctx_lock) };
        unsafe { (*hpriv).ctx = core::ptr::null_mut() };
        unsafe { mutex_unlock(&mut (*hpriv).ctx_lock) };
        unsafe { hl_hpriv_put(hpriv) };
    }

    unsafe { kfree(ctx as *mut core::ffi::c_void) };
}

pub unsafe fn hl_ctx_create(hdev: *mut hl_device, hpriv: *mut hl_fpriv) -> i32 {
    let ctx_mgr = unsafe { &mut (*hpriv).ctx_mgr };
    let ctx = unsafe { kzalloc::<hl_ctx>() };
    if ctx.is_null() {
        return -ENOMEM;
    }

    unsafe { mutex_lock(&mut ctx_mgr.lock) };
    let rc = unsafe { idr_alloc(&mut ctx_mgr.handles, ctx, 1, 0, GFP_KERNEL) };
    unsafe { mutex_unlock(&mut ctx_mgr.lock) };
    if rc < 0 {
        unsafe { dev_err((*hdev).dev, c"Failed to allocate IDR for a new CTX\n") };
        unsafe { kfree(ctx as *mut core::ffi::c_void) };
        return rc;
    }

    unsafe { (*ctx).handle = rc };
    let rc = unsafe { hl_ctx_init(hdev, ctx, false) };
    if rc != 0 {
        unsafe { mutex_lock(&mut ctx_mgr.lock) };
        unsafe { idr_remove(&mut ctx_mgr.handles, (*ctx).handle) };
        unsafe { mutex_unlock(&mut ctx_mgr.lock) };
        unsafe { kfree(ctx as *mut core::ffi::c_void) };
        return rc;
    }

    unsafe { hl_hpriv_get(hpriv) };
    unsafe { (*ctx).hpriv = hpriv };
    unsafe { (*hpriv).ctx = ctx };
    unsafe { (*hdev).is_compute_ctx_active = true };
    0
}

pub unsafe fn hl_ctx_init(hdev: *mut hl_device, ctx: *mut hl_ctx, is_kernel_ctx: bool) -> i32 {
    let mut rc: i32 = 0;
    unsafe {
        (*ctx).hdev = hdev;
        kref_init(&mut (*ctx).refcount);
        (*ctx).cs_sequence = 1;
        spin_lock_init(&mut (*ctx).cs_lock);
        atomic_set(&mut (*ctx).thread_ctx_switch_token, 1);
        (*ctx).thread_ctx_switch_wait_token = 0;
        (*ctx).cs_pending = kzalloc_array((*hdev).asic_prop.max_pending_cs as usize, core::mem::size_of::<*mut hl_fence>(), GFP_KERNEL);
        if (*ctx).cs_pending.is_null() { return -ENOMEM; }
        INIT_LIST_HEAD(&mut (*ctx).outcome_store.used_list);
        INIT_LIST_HEAD(&mut (*ctx).outcome_store.free_list);
        hash_init(&mut (*ctx).outcome_store.outcome_map);
        for i in 0..core::mem::size_of_val(&(*ctx).outcome_store.nodes_pool) / core::mem::size_of::<hl_outcome_node>() {
            list_add(&mut (*ctx).outcome_store.nodes_pool[i].list_link, &mut (*ctx).outcome_store.free_list);
        }
        hl_hw_block_mem_init(ctx);
    }

    if is_kernel_ctx {
        unsafe { (*ctx).asid = HL_KERNEL_ASID_ID; rc = hl_vm_ctx_init(ctx); }
        if rc != 0 { unsafe { dev_err((*hdev).dev, c"Failed to init mem ctx module\n") }; rc = -ENOMEM; unsafe { hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return rc; }
        rc = unsafe { ((*hdev).asic_funcs).as_ref().unwrap().ctx_init(ctx) };
        if rc != 0 { unsafe { dev_err((*hdev).dev, c"ctx_init failed\n"); hl_vm_ctx_fini(ctx); hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return rc; }
    } else {
        unsafe { (*ctx).asid = hl_asid_alloc(hdev) };
        if unsafe { (*ctx).asid == 0 } { unsafe { dev_err((*hdev).dev, c"No free ASID, failed to create context\n"); hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return -ENOMEM; }
        rc = unsafe { hl_vm_ctx_init(ctx) };
        if rc != 0 { unsafe { dev_err((*hdev).dev, c"Failed to init mem ctx module\n"); hl_asid_free(hdev, (*ctx).asid); hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return -ENOMEM; }
        rc = unsafe { hl_cb_va_pool_init(ctx) };
        if rc != 0 { unsafe { dev_err((*hdev).dev, c"Failed to init VA pool for mapped CB\n"); hl_vm_ctx_fini(ctx); hl_asid_free(hdev, (*ctx).asid); hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return rc; }
        rc = unsafe { ((*hdev).asic_funcs).as_ref().unwrap().ctx_init(ctx) };
        if rc != 0 { unsafe { dev_err((*hdev).dev, c"ctx_init failed\n"); hl_cb_va_pool_fini(ctx); hl_vm_ctx_fini(ctx); hl_asid_free(hdev, (*ctx).asid); hl_hw_block_mem_fini(ctx); kfree((*ctx).cs_pending as *mut core::ffi::c_void) }; return rc; }
        unsafe { hl_encaps_sig_mgr_init(&mut (*ctx).sig_mgr); mutex_init(&mut (*ctx).ts_reg_lock); dev_dbg((*hdev).dev, c"create user context, comm=\"%s\", asid=%u\n", current.comm.as_ptr(), (*ctx).asid); }
    }
    rc
}

unsafe fn hl_ctx_get_unless_zero(ctx: *mut hl_ctx) -> i32 { unsafe { kref_get_unless_zero(&mut (*ctx).refcount) } }
pub unsafe fn hl_ctx_get(ctx: *mut hl_ctx) { unsafe { kref_get(&mut (*ctx).refcount) } }
pub unsafe fn hl_ctx_put(ctx: *mut hl_ctx) -> i32 { unsafe { kref_put(&mut (*ctx).refcount, hl_ctx_do_release) } }

pub unsafe fn hl_get_compute_ctx(hdev: *mut hl_device) -> *mut hl_ctx {
    let mut ctx = core::ptr::null_mut();
    unsafe { mutex_lock(&mut (*hdev).fpriv_list_lock); }
    unsafe { list_for_each_entry(|hpriv: *mut hl_fpriv| { mutex_lock(&mut (*hpriv).ctx_lock); ctx = (*hpriv).ctx; if !ctx.is_null() && hl_ctx_get_unless_zero(ctx) == 0 { ctx = core::ptr::null_mut(); } mutex_unlock(&mut (*hpriv).ctx_lock); }, &(*hdev).fpriv_list, dev_node); }
    unsafe { mutex_unlock(&mut (*hdev).fpriv_list_lock); }
    ctx
}

unsafe fn hl_ctx_get_fence_locked(ctx: *mut hl_ctx, seq: u64) -> *mut hl_fence {
    let asic_prop = unsafe { &(*ctx).hdev.as_ref().unwrap().asic_prop };
    if seq >= unsafe { (*ctx).cs_sequence } { return ERR_PTR(-EINVAL); }
    if seq.wrapping_add(asic_prop.max_pending_cs as u64) < unsafe { (*ctx).cs_sequence } { return core::ptr::null_mut(); }
    let fence = unsafe { (*ctx).cs_pending.add((seq & (asic_prop.max_pending_cs as u64 - 1)) as usize).read() };
    unsafe { hl_fence_get(fence); }
    fence
}

pub unsafe fn hl_ctx_get_fence(ctx: *mut hl_ctx, seq: u64) -> *mut hl_fence { unsafe { spin_lock(&mut (*ctx).cs_lock); let fence = hl_ctx_get_fence_locked(ctx, seq); spin_unlock(&mut (*ctx).cs_lock); fence } }

pub unsafe fn hl_ctx_get_fences(ctx: *mut hl_ctx, seq_arr: *mut u64, fence: *mut *mut hl_fence, arr_len: u32) -> i32 {
    let fence_arr_base = fence;
    let mut i = 0;
    let mut rc = 0;
    unsafe { spin_lock(&mut (*ctx).cs_lock); }
    while i < arr_len {
        let seq = unsafe { seq_arr.add(i as usize).read() };
        unsafe { fence.add(i as usize).write(hl_ctx_get_fence_locked(ctx, seq)); }
        if unsafe { IS_ERR(fence.add(i as usize).read()) } { unsafe { dev_err((*ctx).hdev).dev, c"Failed to get fence for CS with seq 0x%llx\n", seq) }; rc = unsafe { PTR_ERR(fence.add(i as usize).read()) }; break; }
        i += 1;
    }
    unsafe { spin_unlock(&mut (*ctx).cs_lock); }
    if rc != 0 { unsafe { hl_fences_put(fence_arr_base, i) }; }
    rc
}

pub unsafe fn hl_ctx_mgr_init(ctx_mgr: *mut hl_ctx_mgr) { unsafe { mutex_init(&mut (*ctx_mgr).lock); idr_init(&mut (*ctx_mgr).handles); } }

pub unsafe fn hl_ctx_mgr_fini(_hdev: *mut hl_device, ctx_mgr: *mut hl_ctx_mgr) {
    let idp = unsafe { &mut (*ctx_mgr).handles };
    let mut id: u32 = 0;
    let mut ctx: *mut hl_ctx = core::ptr::null_mut();
    unsafe { idr_for_each_entry(idp, &mut ctx, &mut id); kref_put(&mut (*ctx).refcount, hl_ctx_do_release); idr_destroy(idp); mutex_destroy(&mut (*ctx_mgr).lock); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
