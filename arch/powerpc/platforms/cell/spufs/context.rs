// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPU file system -- SPU context management
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 */

// Kernel and architecture declarations are supplied by the surrounding crate.
use crate::*;
use core::sync::atomic::{AtomicI32, Ordering};

pub static nr_spu_contexts: AtomicI32 = AtomicI32::new(0);

pub unsafe fn alloc_spu_context(gang: *mut spu_gang) -> *mut spu_context {
    let mut ctx: *mut spu_context = kzalloc_obj::<spu_context>();
    if ctx.is_null() {
        return ctx;
    }

    // Binding to physical processor deferred until spu_activate().
    if spu_init_csa(&mut (*ctx).csa) != 0 {
        kfree(ctx);
        return core::ptr::null_mut();
    }
    spin_lock_init(&mut (*ctx).mmio_lock);
    mutex_init(&mut (*ctx).mapping_lock);
    kref_init(&mut (*ctx).kref);
    mutex_init(&mut (*ctx).state_mutex);
    mutex_init(&mut (*ctx).run_mutex);
    init_waitqueue_head(&mut (*ctx).ibox_wq);
    init_waitqueue_head(&mut (*ctx).wbox_wq);
    init_waitqueue_head(&mut (*ctx).stop_wq);
    init_waitqueue_head(&mut (*ctx).mfc_wq);
    init_waitqueue_head(&mut (*ctx).run_wq);
    (*ctx).state = SPU_STATE_SAVED;
    (*ctx).ops = &spu_backing_ops;
    (*ctx).owner = get_task_mm(current);
    INIT_LIST_HEAD(&mut (*ctx).rq);
    INIT_LIST_HEAD(&mut (*ctx).aff_list);
    if !gang.is_null() {
        spu_gang_add_ctx(gang, ctx);
    }

    __spu_update_sched_info(ctx);
    spu_set_timeslice(ctx);
    (*ctx).stats.util_state = SPU_UTIL_IDLE_LOADED;
    (*ctx).stats.tstamp = ktime_get_ns();

    nr_spu_contexts.fetch_add(1, Ordering::SeqCst);
    ctx
}

pub unsafe fn destroy_spu_context(kref: *mut kref) {
    let ctx: *mut spu_context = container_of!(kref, spu_context, kref);
    spu_context_nospu_trace!(destroy_spu_context__enter, ctx);
    mutex_lock(&mut (*ctx).state_mutex);
    spu_deactivate(ctx);
    mutex_unlock(&mut (*ctx).state_mutex);
    spu_fini_csa(&mut (*ctx).csa);
    if !(*ctx).gang.is_null() {
        spu_gang_remove_ctx((*ctx).gang, ctx);
    }
    if !(*ctx).prof_priv_kref.is_null() {
        kref_put((*ctx).prof_priv_kref, (*ctx).prof_priv_release);
    }
    BUG_ON!(!list_empty(&(*ctx).rq));
    nr_spu_contexts.fetch_sub(1, Ordering::SeqCst);
    kfree((*ctx).switch_log);
    kfree(ctx);
}

pub unsafe fn get_spu_context(ctx: *mut spu_context) -> *mut spu_context {
    kref_get(&mut (*ctx).kref);
    ctx
}

pub unsafe fn put_spu_context(ctx: *mut spu_context) -> c_int {
    kref_put(&mut (*ctx).kref, destroy_spu_context)
}

/* give up the mm reference when the context is about to be destroyed */
pub unsafe fn spu_forget(ctx: *mut spu_context) {
    let mm: *mut mm_struct;

    /*
     * This is basically an open-coded spu_acquire_saved, except that
     * we don't acquire the state mutex interruptible, and we don't
     * want this context to be rescheduled on release.
     */
    mutex_lock(&mut (*ctx).state_mutex);
    if (*ctx).state != SPU_STATE_SAVED {
        spu_deactivate(ctx);
    }

    mm = (*ctx).owner;
    (*ctx).owner = core::ptr::null_mut();
    mmput(mm);
    spu_release(ctx);
}

pub unsafe fn spu_unmap_mappings(ctx: *mut spu_context) {
    mutex_lock(&mut (*ctx).mapping_lock);
    if !(*ctx).local_store.is_null() {
        unmap_mapping_range((*ctx).local_store, 0, LS_SIZE, 1);
    }
    if !(*ctx).mfc.is_null() {
        unmap_mapping_range((*ctx).mfc, 0, SPUFS_MFC_MAP_SIZE, 1);
    }
    if !(*ctx).cntl.is_null() {
        unmap_mapping_range((*ctx).cntl, 0, SPUFS_CNTL_MAP_SIZE, 1);
    }
    if !(*ctx).signal1.is_null() {
        unmap_mapping_range((*ctx).signal1, 0, SPUFS_SIGNAL_MAP_SIZE, 1);
    }
    if !(*ctx).signal2.is_null() {
        unmap_mapping_range((*ctx).signal2, 0, SPUFS_SIGNAL_MAP_SIZE, 1);
    }
    if !(*ctx).mss.is_null() {
        unmap_mapping_range((*ctx).mss, 0, SPUFS_MSS_MAP_SIZE, 1);
    }
    if !(*ctx).psmap.is_null() {
        unmap_mapping_range((*ctx).psmap, 0, SPUFS_PS_MAP_SIZE, 1);
    }
    mutex_unlock(&mut (*ctx).mapping_lock);
}

/**
 * spu_acquire_saved - lock spu contex and make sure it is in saved state
 * @ctx: spu contex to lock
 */
pub unsafe fn spu_acquire_saved(ctx: *mut spu_context) -> c_int {
    spu_context_nospu_trace!(spu_acquire_saved__enter, ctx);

    let ret = spu_acquire(ctx);
    if ret != 0 {
        return ret;
    }

    if (*ctx).state != SPU_STATE_SAVED {
        set_bit(SPU_SCHED_WAS_ACTIVE, &mut (*ctx).sched_flags);
        spu_deactivate(ctx);
    }

    0
}

/**
 * spu_release_saved - unlock spu context and return it to the runqueue
 * @ctx: context to unlock
 */
pub unsafe fn spu_release_saved(ctx: *mut spu_context) {
    BUG_ON!((*ctx).state != SPU_STATE_SAVED);

    if test_and_clear_bit(SPU_SCHED_WAS_ACTIVE, &mut (*ctx).sched_flags)
        && test_bit(SPU_SCHED_SPU_RUN, &(*ctx).sched_flags)
    {
        spu_activate(ctx, 0);
    }

    spu_release(ctx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
