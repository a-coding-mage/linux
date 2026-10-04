// SPDX-License-Identifier: GPL-2.0
// mm/slab_common.c RCU tail, source e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Native structure layouts/constants come only from slab_common_bindings.h.

#[no_mangle]
pub unsafe extern "C" fn kfree_call_rcu_nolock(head: *mut kvfree_rcu_head, ptr: *mut Void) {
    if cfg!(CONFIG_KVFREE_RCU_BATCHED) && !rust_slab_common_rcu_is_vmalloc_addr(ptr) {
        let slab = rust_slab_common_rcu_virt_to_slab(ptr);
        if !slab.is_null()
            && (!cfg!(CONFIG_NUMA)
                || rust_slab_common_rcu_slab_nid(slab) == rust_slab_common_rcu_numa_mem_id())
            && __kfree_rcu_sheaf((*slab).slab_cache, ptr, RSC_RCU_SLAB_FREE_NOLOCK as UInt)
        {
            return;
        }
    }
    defer_kfree_rcu(head);
}

#[cfg(not(CONFIG_KVFREE_RCU_BATCHED))]
#[no_mangle]
pub unsafe extern "C" fn kvfree_call_rcu(head: *mut kvfree_rcu_head, ptr: *mut Void) {
    if !head.is_null() {
        rust_slab_common_rcu_kasan_record_aux_stack(ptr);
        call_rcu(addr_of_mut!((*head).head), Some(kvfree_rcu_cb));
        return;
    }
    rust_slab_common_rcu_might_sleep();
    synchronize_rcu();
    kvfree(ptr);
}

#[cfg(not(CONFIG_KVFREE_RCU_BATCHED))]
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier() {
    deferred_work_barrier();
    rcu_barrier();
}

#[cfg(not(CONFIG_KVFREE_RCU_BATCHED))]
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_barrier_on_cache(_s: *mut kmem_cache) {
    deferred_work_barrier();
    rcu_barrier();
}

#[cfg(not(CONFIG_KVFREE_RCU_BATCHED))]
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn kvfree_rcu_init() {}

#[cfg(CONFIG_KVFREE_RCU_BATCHED)]
mod batched_rcu {
    use super::*;

    const FREE_N_CHANNELS: usize = RSC_RCU_FREE_N_CHANNELS as usize;
    const KFREE_N_BATCHES: usize = RSC_RCU_KFREE_N_BATCHES as usize;

    static mut RCU_RECLAIM_WQ: *mut workqueue_struct = null_mut();

    // The flexible array's offset is generated from the native definition.
    unsafe fn bulk_records(bnode: *mut kvfree_rcu_bulk_data) -> *mut *mut Void {
        bnode
            .cast::<u8>()
            .add(offset_of!(kvfree_rcu_bulk_data, records))
            .cast()
    }

    unsafe fn bulk_from_list(node: *mut list_head) -> *mut kvfree_rcu_bulk_data {
        node.cast::<u8>()
            .wrapping_sub(offset_of!(kvfree_rcu_bulk_data, list))
            .cast()
    }

    unsafe fn debug_rcu_bhead_unqueue(_bhead: *mut kvfree_rcu_bulk_data) {
        #[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
        {
            let mut i: Int = 0;
            while (i as ULong) < (*_bhead).nr_records {
                rust_slab_common_rcu_debug_head_unqueue(*bulk_records(_bhead).add(i as usize));
                i = i.wrapping_add(1);
            }
        }
    }

    unsafe fn krc_this_cpu_lock(flags: *mut ULong) -> *mut kfree_rcu_cpu {
        rust_slab_common_rcu_local_irq_save(flags);
        let krcp = rust_slab_common_rcu_this_cpu();
        rust_slab_common_rcu_raw_spin_lock(addr_of_mut!((*krcp).lock));
        krcp
    }

    unsafe fn krc_this_cpu_unlock(krcp: *mut kfree_rcu_cpu, flags: ULong) {
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
    }

    unsafe fn get_cached_bnode(krcp: *mut kfree_rcu_cpu) -> *mut kvfree_rcu_bulk_data {
        if (*krcp).nr_bkv_objs == 0 {
            return null_mut();
        }
        rust_slab_common_rcu_write_nr_bkv_objs(krcp, (*krcp).nr_bkv_objs.wrapping_sub(1));
        llist_del_first(addr_of_mut!((*krcp).bkvcache)).cast()
    }

    unsafe fn put_cached_bnode(krcp: *mut kfree_rcu_cpu, bnode: *mut kvfree_rcu_bulk_data) -> bool {
        if (*krcp).nr_bkv_objs >= rust_slab_common_rcu_min_cached_objs {
            return false;
        }
        rust_slab_common_rcu_llist_add(bnode.cast(), addr_of_mut!((*krcp).bkvcache));
        rust_slab_common_rcu_write_nr_bkv_objs(krcp, (*krcp).nr_bkv_objs.wrapping_add(1));
        true
    }

    unsafe fn drain_page_cache(krcp: *mut kfree_rcu_cpu) -> Int {
        if rust_slab_common_rcu_min_cached_objs == 0 {
            return 0;
        }
        let mut flags: ULong = 0;
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        let mut pos = rust_slab_common_rcu_llist_del_all(addr_of_mut!((*krcp).bkvcache));
        rust_slab_common_rcu_write_nr_bkv_objs(krcp, 0);
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
        let mut freed: Int = 0;
        while !pos.is_null() {
            let next = (*pos).next;
            rust_slab_common_rcu_free_page(pos as ULong);
            freed = freed.wrapping_add(1);
            pos = next;
        }
        freed
    }

    unsafe fn kvfree_rcu_bulk(
        krcp: *mut kfree_rcu_cpu,
        mut bnode: *mut kvfree_rcu_bulk_data,
        idx: Int,
    ) {
        if !rust_slab_common_rcu_warn_bulk_gp(!rust_slab_common_rcu_poll_state_full(addr_of_mut!(
            (*bnode).gp_snap
        ))) {
            debug_rcu_bhead_unqueue(bnode);
            rust_slab_common_rcu_callback_acquire();
            if idx == 0 {
                rust_slab_common_rcu_trace_bulk(bnode);
                rust_slab_common_rcu_kfree_bulk((*bnode).nr_records as usize, bulk_records(bnode));
            } else {
                let mut i: Int = 0;
                while (i as ULong) < (*bnode).nr_records {
                    rust_slab_common_rcu_trace_bulk_record(bnode, i);
                    vfree(*bulk_records(bnode).add(i as usize));
                    i = i.wrapping_add(1);
                }
            }
            rust_slab_common_rcu_callback_release();
        }

        let mut flags: ULong = 0;
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        if put_cached_bnode(krcp, bnode) {
            bnode = null_mut();
        }
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
        if !bnode.is_null() {
            rust_slab_common_rcu_free_page(bnode as ULong);
        }
        rust_slab_common_rcu_cond_resched_tasks();
    }

    unsafe fn kvfree_rcu_list(mut head: *mut kvfree_rcu_head) {
        while !head.is_null() {
            let ptr = rust_slab_common_rcu_obj_start(head.cast());
            let offset = (head as ULong).wrapping_sub(ptr as ULong);
            let next = (*head).next;
            rust_slab_common_rcu_debug_head_unqueue(ptr);
            rust_slab_common_rcu_callback_acquire();
            rust_slab_common_rcu_trace_list(head, offset);
            kvfree(ptr);
            rust_slab_common_rcu_callback_release();
            rust_slab_common_rcu_cond_resched_tasks();
            head = next;
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn kfree_rcu_work(work: *mut work_struct) {
        let krwp = work
            .cast::<u8>()
            .sub(offset_of!(rcu_work, work))
            .sub(offset_of!(kfree_rcu_cpu_work, rcu_work))
            .cast::<kfree_rcu_cpu_work>();
        let krcp = (*krwp).krcp;
        let mut flags: ULong = 0;
        let mut bulk_head: [list_head; FREE_N_CHANNELS] = zeroed();
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        for i in 0..FREE_N_CHANNELS {
            rust_slab_common_rcu_list_replace_init(
                addr_of_mut!((*krwp).bulk_head_free[i]),
                addr_of_mut!(bulk_head[i]),
            );
        }
        let head = (*krwp).head_free;
        (*krwp).head_free = null_mut();
        let mut head_gp_snap = core::ptr::read(addr_of!((*krwp).head_free_gp_snap));
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);

        for i in 0..FREE_N_CHANNELS {
            let list = addr_of_mut!(bulk_head[i]);
            let mut node = (*list).next;
            while node != list {
                let next = (*node).next;
                kvfree_rcu_bulk(krcp, bulk_from_list(node), i as Int);
                node = next;
            }
        }
        if !head.is_null()
            && !rust_slab_common_rcu_warn_head_gp(!rust_slab_common_rcu_poll_state_full(
                &mut head_gp_snap,
            ))
        {
            kvfree_rcu_list(head);
        }
    }

    unsafe fn kfree_rcu_sheaf(obj: *mut Void) -> bool {
        let free_flags = if cfg!(CONFIG_PREEMPT_RT) {
            RSC_RCU_SLAB_FREE_NOLOCK as UInt
        } else {
            RSC_RCU_SLAB_FREE_DEFAULT as UInt
        };
        if rust_slab_common_rcu_is_vmalloc_addr(obj) {
            return false;
        }
        let slab = rust_slab_common_rcu_virt_to_slab(obj);
        if slab.is_null() {
            return false;
        }
        let s = (*slab).slab_cache;
        if !cfg!(CONFIG_NUMA)
            || rust_slab_common_rcu_slab_nid(slab) == rust_slab_common_rcu_numa_mem_id()
        {
            return __kfree_rcu_sheaf(s, obj, free_flags);
        }
        false
    }

    unsafe fn need_offload_krc(krcp: *mut kfree_rcu_cpu) -> bool {
        for i in 0..FREE_N_CHANNELS {
            if !rust_slab_common_rcu_list_empty(addr_of!((*krcp).bulk_head[i])) {
                return true;
            }
        }
        !rust_slab_common_rcu_read_head(krcp).is_null()
    }

    unsafe fn need_wait_for_krwp_work(krwp: *mut kfree_rcu_cpu_work) -> bool {
        for i in 0..FREE_N_CHANNELS {
            if !rust_slab_common_rcu_list_empty(addr_of!((*krwp).bulk_head_free[i])) {
                return true;
            }
        }
        !(*krwp).head_free.is_null()
    }

    unsafe fn krc_count(krcp: *mut kfree_rcu_cpu) -> Int {
        let mut sum = rust_slab_common_rcu_atomic_read(addr_of!((*krcp).head_count));
        for i in 0..FREE_N_CHANNELS {
            sum = sum.wrapping_add(rust_slab_common_rcu_atomic_read(addr_of!(
                (*krcp).bulk_count[i]
            )));
        }
        sum
    }

    unsafe fn __schedule_delayed_monitor_work(krcp: *mut kfree_rcu_cpu) {
        // Original int count is converted to unsigned size_t for this comparison.
        let delay: Long = if (krc_count(krcp) as usize) >= RSC_RCU_KVFREE_BULK_MAX_ENTR as usize {
            1
        } else {
            RSC_RCU_KFREE_DRAIN_JIFFIES as Long
        };
        let work = addr_of_mut!((*krcp).monitor_work);
        if rust_slab_common_rcu_delayed_work_pending(work) {
            let delay_left = (*work)
                .timer
                .expires
                .wrapping_sub(rust_slab_common_rcu_jiffies()) as Long;
            if delay < delay_left {
                rust_slab_common_rcu_mod_delayed_work(RCU_RECLAIM_WQ, work, delay as ULong);
            }
            return;
        }
        rust_slab_common_rcu_queue_delayed_work(RCU_RECLAIM_WQ, work, delay as ULong);
    }

    unsafe fn schedule_delayed_monitor_work(krcp: *mut kfree_rcu_cpu) {
        let mut flags: ULong = 0;
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        __schedule_delayed_monitor_work(krcp);
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
    }

    unsafe fn kvfree_rcu_drain_ready(krcp: *mut kfree_rcu_cpu) {
        let mut bulk_ready: [list_head; FREE_N_CHANNELS] = zeroed();
        let mut head_ready: *mut kvfree_rcu_head = null_mut();
        let mut flags: ULong = 0;
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        for i in 0..FREE_N_CHANNELS {
            let ready = addr_of_mut!(bulk_ready[i]);
            rust_slab_common_rcu_init_list_head(ready);
            let list = addr_of_mut!((*krcp).bulk_head[i]);
            let mut node = (*list).prev;
            while node != list {
                let next = (*node).prev;
                let bnode = bulk_from_list(node);
                if !rust_slab_common_rcu_poll_state_full(addr_of_mut!((*bnode).gp_snap)) {
                    break;
                }
                rust_slab_common_rcu_atomic_sub(
                    (*bnode).nr_records as Int,
                    addr_of_mut!((*krcp).bulk_count[i]),
                );
                rust_slab_common_rcu_list_move(addr_of_mut!((*bnode).list), ready);
                node = next;
            }
        }
        if !(*krcp).head.is_null() && rust_slab_common_rcu_poll_state((*krcp).head_gp_snap) {
            head_ready = (*krcp).head;
            rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).head_count), 0);
            rust_slab_common_rcu_write_head(krcp, null_mut());
        }
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
        for i in 0..FREE_N_CHANNELS {
            let list = addr_of_mut!(bulk_ready[i]);
            let mut node = (*list).next;
            while node != list {
                let next = (*node).next;
                kvfree_rcu_bulk(krcp, bulk_from_list(node), i as Int);
                node = next;
            }
        }
        if !head_ready.is_null() {
            kvfree_rcu_list(head_ready);
        }
    }

    unsafe fn kvfree_rcu_queue_batch(krcp: *mut kfree_rcu_cpu) -> bool {
        let mut flags: ULong = 0;
        let mut queued = false;
        rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
        for i in 0..KFREE_N_BATCHES {
            let krwp = addr_of_mut!((*krcp).krw_arr[i]);
            if need_wait_for_krwp_work(krwp) {
                continue;
            }
            if need_offload_krc(krcp) {
                for j in 0..FREE_N_CHANNELS {
                    if rust_slab_common_rcu_list_empty(addr_of!((*krwp).bulk_head_free[j])) {
                        rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).bulk_count[j]), 0);
                        rust_slab_common_rcu_list_replace_init(
                            addr_of_mut!((*krcp).bulk_head[j]),
                            addr_of_mut!((*krwp).bulk_head_free[j]),
                        );
                    }
                }
                if (*krwp).head_free.is_null() {
                    (*krwp).head_free = (*krcp).head;
                    rust_slab_common_rcu_get_state_full(addr_of_mut!((*krwp).head_free_gp_snap));
                    rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).head_count), 0);
                    rust_slab_common_rcu_write_head(krcp, null_mut());
                }
                queued = queue_rcu_work(RCU_RECLAIM_WQ, addr_of_mut!((*krwp).rcu_work));
                rust_slab_common_rcu_warn_queue(!queued);
                break;
            }
        }
        rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
        queued
    }

    #[no_mangle]
    pub unsafe extern "C" fn kfree_rcu_monitor(work: *mut work_struct) {
        let krcp = work
            .cast::<u8>()
            .sub(offset_of!(kfree_rcu_cpu, monitor_work))
            .sub(offset_of!(delayed_work, work))
            .cast::<kfree_rcu_cpu>();
        kvfree_rcu_drain_ready(krcp);
        kvfree_rcu_queue_batch(krcp);
        if need_offload_krc(krcp) {
            schedule_delayed_monitor_work(krcp);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn fill_page_cache_func(work: *mut work_struct) {
        let krcp = work
            .cast::<u8>()
            .sub(offset_of!(kfree_rcu_cpu, page_cache_work))
            .sub(offset_of!(delayed_work, work))
            .cast::<kfree_rcu_cpu>();
        let nr_pages =
            if rust_slab_common_rcu_atomic_read(addr_of!((*krcp).backoff_page_cache_fill)) != 0 {
                1
            } else {
                rust_slab_common_rcu_min_cached_objs
            };
        let mut i = rust_slab_common_rcu_read_nr_bkv_objs(krcp);
        while i < nr_pages {
            let bnode = rust_slab_common_rcu_get_free_page_fill(RSC_RCU_GFP_PAGE as gfp_t)
                as *mut kvfree_rcu_bulk_data;
            if bnode.is_null() {
                break;
            }
            let mut flags: ULong = 0;
            rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((*krcp).lock), &mut flags);
            let pushed = put_cached_bnode(krcp, bnode);
            rust_slab_common_rcu_raw_spin_unlock_irqrestore(addr_of_mut!((*krcp).lock), flags);
            if !pushed {
                rust_slab_common_rcu_free_page(bnode as ULong);
                break;
            }
            i = i.wrapping_add(1);
        }
        rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).work_in_progress), 0);
        rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).backoff_page_cache_fill), 0);
    }

    unsafe fn add_ptr_to_bulk_krc_lock(
        krcp: *mut *mut kfree_rcu_cpu,
        flags: *mut ULong,
        ptr: *mut Void,
        can_alloc: bool,
    ) -> bool {
        *krcp = krc_this_cpu_lock(flags);
        if !(**krcp).initialized {
            return false;
        }
        let idx = rust_slab_common_rcu_is_vmalloc_addr(ptr) as usize;
        let mut bnode = rust_slab_common_rcu_first_bulk(addr_of_mut!((**krcp).bulk_head[idx]));
        if bnode.is_null() || (*bnode).nr_records == RSC_RCU_KVFREE_BULK_MAX_ENTR as ULong {
            bnode = get_cached_bnode(*krcp);
            if bnode.is_null() && can_alloc {
                krc_this_cpu_unlock(*krcp, *flags);
                bnode = rust_slab_common_rcu_get_free_page_add(RSC_RCU_GFP_PAGE as gfp_t)
                    as *mut kvfree_rcu_bulk_data;
                // Reacquire the original CPU's lock, even if allocation migrated us.
                rust_slab_common_rcu_raw_spin_lock_irqsave(addr_of_mut!((**krcp).lock), flags);
            }
            if bnode.is_null() {
                return false;
            }
            (*bnode).nr_records = 0;
            rust_slab_common_rcu_list_add(
                addr_of_mut!((*bnode).list),
                addr_of_mut!((**krcp).bulk_head[idx]),
            );
        }
        (*bnode).nr_records = (*bnode).nr_records.wrapping_add(1);
        *bulk_records(bnode).add((*bnode).nr_records.wrapping_sub(1) as usize) = ptr;
        rust_slab_common_rcu_get_state_full(addr_of_mut!((*bnode).gp_snap));
        rust_slab_common_rcu_atomic_inc(addr_of_mut!((**krcp).bulk_count[idx]));
        true
    }

    #[no_mangle]
    pub unsafe extern "C" fn schedule_page_work_fn(timer: *mut hrtimer) -> hrtimer_restart {
        let krcp = timer
            .cast::<u8>()
            .sub(offset_of!(kfree_rcu_cpu, hrtimer))
            .cast::<kfree_rcu_cpu>();
        rust_slab_common_rcu_queue_delayed_work(
            system_highpri_wq,
            addr_of_mut!((*krcp).page_cache_work),
            0,
        );
        RSC_RCU_HRTIMER_NORESTART as hrtimer_restart
    }

    unsafe fn run_page_cache_worker(krcp: *mut kfree_rcu_cpu) {
        if rust_slab_common_rcu_min_cached_objs == 0 {
            return;
        }
        if rcu_scheduler_active == RSC_RCU_SCHEDULER_RUNNING as Int
            && rust_slab_common_rcu_atomic_xchg(addr_of_mut!((*krcp).work_in_progress), 1) == 0
        {
            if rust_slab_common_rcu_atomic_read(addr_of!((*krcp).backoff_page_cache_fill)) != 0 {
                rust_slab_common_rcu_queue_delayed_work(
                    RCU_RECLAIM_WQ,
                    addr_of_mut!((*krcp).page_cache_work),
                    rust_slab_common_rcu_msecs_to_jiffies(
                        rust_slab_common_rcu_delay_page_cache_fill_msec as UInt,
                    ),
                );
            } else {
                hrtimer_setup(
                    addr_of_mut!((*krcp).hrtimer),
                    Some(schedule_page_work_fn),
                    RSC_RCU_CLOCK_MONOTONIC as _,
                    RSC_RCU_HRTIMER_MODE_REL as hrtimer_mode,
                );
                rust_slab_common_rcu_hrtimer_start(
                    addr_of_mut!((*krcp).hrtimer),
                    0,
                    RSC_RCU_HRTIMER_MODE_REL as hrtimer_mode,
                );
            }
        }
    }

    #[no_mangle]
    #[link_section = ".init.text"]
    #[cfg_attr(RSC_INIT_COLD, cold)]
    pub unsafe extern "C" fn kfree_rcu_scheduler_running() {
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            if need_offload_krc(krcp) {
                schedule_delayed_monitor_work(krcp);
            }
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn kvfree_call_rcu(head: *mut kvfree_rcu_head, ptr: *mut Void) {
        if head.is_null() {
            rust_slab_common_rcu_might_sleep();
        }
        if kfree_rcu_sheaf(ptr) {
            return;
        }
        if rust_slab_common_rcu_debug_head_queue(ptr) != 0 {
            rust_slab_common_rcu_warn_double_free(head);
            return;
        }
        rust_slab_common_rcu_kasan_record_aux_stack(ptr);
        let mut flags: ULong = 0;
        let mut krcp: *mut kfree_rcu_cpu = null_mut();
        let mut success = add_ptr_to_bulk_krc_lock(&mut krcp, &mut flags, ptr, head.is_null());
        if !success {
            run_page_cache_worker(krcp);
            if !head.is_null() {
                (*head).next = (*krcp).head;
                rust_slab_common_rcu_write_head(krcp, head);
                rust_slab_common_rcu_atomic_inc(addr_of_mut!((*krcp).head_count));
                (*krcp).head_gp_snap = rust_slab_common_rcu_get_state();
                success = true;
            }
        }
        // The headless failure goes straight to unlock_return in the original.
        if success {
            rust_slab_common_rcu_kmemleak_ignore(ptr);
            if rcu_scheduler_active == RSC_RCU_SCHEDULER_RUNNING as Int {
                __schedule_delayed_monitor_work(krcp);
            }
        }
        krc_this_cpu_unlock(krcp, flags);
        if !success {
            rust_slab_common_rcu_debug_head_unqueue(ptr);
            synchronize_rcu();
            kvfree(ptr);
        }
    }

    unsafe fn __kvfree_rcu_barrier() {
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            if need_offload_krc(krcp) {
                loop {
                    let queued = kvfree_rcu_queue_batch(krcp);
                    if queued || !need_offload_krc(krcp) {
                        break;
                    }
                    for i in 0..KFREE_N_BATCHES {
                        flush_rcu_work(addr_of_mut!((*krcp).krw_arr[i].rcu_work));
                    }
                }
            }
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            cancel_delayed_work_sync(addr_of_mut!((*krcp).monitor_work));
            for i in 0..KFREE_N_BATCHES {
                flush_rcu_work(addr_of_mut!((*krcp).krw_arr[i].rcu_work));
            }
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn kvfree_rcu_barrier() {
        flush_all_rcu_sheaves();
        __kvfree_rcu_barrier();
    }

    #[no_mangle]
    pub unsafe extern "C" fn kvfree_rcu_barrier_on_cache(s: *mut kmem_cache) {
        deferred_work_barrier();
        if rust_slab_common_rcu_cache_has_sheaves(s) {
            rust_slab_common_rcu_cpus_read_lock();
            flush_rcu_sheaves_on_cache(s);
            rust_slab_common_rcu_cpus_read_unlock();
        }
        rcu_barrier();
        __kvfree_rcu_barrier();
    }

    #[no_mangle]
    pub unsafe extern "C" fn kfree_rcu_shrink_count(
        _shrink: *mut shrinker,
        _sc: *mut shrink_control,
    ) -> ULong {
        let mut count: ULong = 0;
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            count = count.wrapping_add(krc_count(krcp) as ULong);
            count = count.wrapping_add(rust_slab_common_rcu_read_nr_bkv_objs(krcp) as ULong);
            rust_slab_common_rcu_atomic_set(addr_of_mut!((*krcp).backoff_page_cache_fill), 1);
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
        if count == 0 {
            RSC_RCU_SHRINK_EMPTY as ULong
        } else {
            count
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn kfree_rcu_shrink_scan(
        _shrink: *mut shrinker,
        sc: *mut shrink_control,
    ) -> ULong {
        let mut freed: Int = 0;
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            let count = krc_count(krcp).wrapping_add(drain_page_cache(krcp));
            kfree_rcu_monitor(addr_of_mut!((*krcp).monitor_work.work));
            (*sc).nr_to_scan = (*sc).nr_to_scan.wrapping_sub(count as ULong);
            freed = freed.wrapping_add(count);
            // nr_to_scan is unsigned long: the original <= 0 means == 0.
            if (*sc).nr_to_scan == 0 {
                break;
            }
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
        if freed == 0 {
            RSC_RCU_SHRINK_STOP as ULong
        } else {
            freed as ULong
        }
    }

    #[no_mangle]
    #[link_section = ".init.text"]
    #[cfg_attr(RSC_INIT_COLD, cold)]
    pub unsafe extern "C" fn kvfree_rcu_init() {
        RCU_RECLAIM_WQ = rust_slab_common_rcu_alloc_workqueue(RSC_RCU_WQ_FLAGS as UInt);
        rust_slab_common_rcu_warn_workqueue(RCU_RECLAIM_WQ.is_null());
        let maximum = (100 as Int).wrapping_mul(RSC_RCU_MSEC_PER_SEC as Int);
        if rust_slab_common_rcu_delay_page_cache_fill_msec < 0
            || rust_slab_common_rcu_delay_page_cache_fill_msec > maximum
        {
            rust_slab_common_rcu_delay_page_cache_fill_msec = min(
                max(rust_slab_common_rcu_delay_page_cache_fill_msec, 0),
                maximum,
            );
            #[cfg(CONFIG_PRINTK)]
            rust_slab_common_rcu_info_delay(rust_slab_common_rcu_delay_page_cache_fill_msec);
        }
        let mut cpu = rust_slab_common_rcu_first_cpu();
        while cpu < rust_slab_common_rcu_cpu_limit() {
            let krcp = rust_slab_common_rcu_per_cpu(cpu as Int);
            for i in 0..KFREE_N_BATCHES {
                rust_slab_common_rcu_init_rcu_work(
                    addr_of_mut!((*krcp).krw_arr[i].rcu_work),
                    Some(kfree_rcu_work),
                );
                (*krcp).krw_arr[i].krcp = krcp;
                for j in 0..FREE_N_CHANNELS {
                    rust_slab_common_rcu_init_list_head(addr_of_mut!(
                        (*krcp).krw_arr[i].bulk_head_free[j]
                    ));
                }
            }
            for i in 0..FREE_N_CHANNELS {
                rust_slab_common_rcu_init_list_head(addr_of_mut!((*krcp).bulk_head[i]));
            }
            rust_slab_common_rcu_init_monitor_work(
                addr_of_mut!((*krcp).monitor_work),
                Some(kfree_rcu_monitor),
            );
            rust_slab_common_rcu_init_page_cache_work(
                addr_of_mut!((*krcp).page_cache_work),
                Some(fill_page_cache_func),
            );
            (*krcp).initialized = true;
            cpu = rust_slab_common_rcu_next_cpu(cpu as Int);
        }
        let kfree_rcu_shrinker = shrinker_alloc(0, b"slab-kvfree-rcu\0".as_ptr().cast::<CChar>());
        if kfree_rcu_shrinker.is_null() {
            rust_slab_common_rcu_error_shrinker();
            return;
        }
        (*kfree_rcu_shrinker).count_objects = Some(kfree_rcu_shrink_count);
        (*kfree_rcu_shrinker).scan_objects = Some(kfree_rcu_shrink_scan);
        shrinker_register(kfree_rcu_shrinker);
    }
}
