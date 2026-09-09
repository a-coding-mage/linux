// SPDX-License-Identifier: GPL-2.0-or-later
/* sched.c - SPU scheduler. */

// Linux headers and build-time configuration are supplied by the surrounding
// kernel translation unit.

#[repr(C)]
pub struct spu_prio_array {
    pub bitmap: [usize; 1],
    pub runq: [list_head; MAX_PRIO],
    pub runq_lock: spinlock_t,
    pub nr_waiting: i32,
}

static mut spu_avenrun: [c_ulong; 3] = [0; 3];
static mut spu_prio: *mut spu_prio_array = core::ptr::null_mut();
static mut spusched_task: *mut task_struct = core::ptr::null_mut();
static mut spusched_timer: timer_list = unsafe { core::mem::zeroed() };
static mut spuloadavg_timer: timer_list = unsafe { core::mem::zeroed() };

const NORMAL_PRIO: i32 = 120;
const SPUSCHED_TICK: u64 = 10;
const MIN_SPU_TIMESLICE: i32 = 1; // max(5 * HZ / (1000 * SPUSCHED_TICK), 1)
const DEF_SPU_TIMESLICE: i32 = 100 * HZ / (1000 * SPUSCHED_TICK as i32);

#[inline]
unsafe fn scale_prio(x: i32, prio: i32) -> i32 {
    core::cmp::max(x * (MAX_PRIO as i32 - prio) / (NICE_WIDTH as i32 / 2), MIN_SPU_TIMESLICE)
}

pub unsafe fn spu_set_timeslice(ctx: *mut spu_context) {
    (*ctx).time_slice = if (*ctx).prio < NORMAL_PRIO {
        scale_prio(DEF_SPU_TIMESLICE * 4, (*ctx).prio)
    } else { scale_prio(DEF_SPU_TIMESLICE, (*ctx).prio) };
}

pub unsafe fn __spu_update_sched_info(ctx: *mut spu_context) {
    BUG_ON(!list_empty(&(*ctx).rq));
    (*ctx).tid = (*current).pid;
    (*ctx).prio = if rt_prio((*current).prio) { (*current).prio } else { (*current).static_prio };
    (*ctx).policy = (*current).policy;
    cpumask_copy(&mut (*ctx).cpus_allowed, (*current).cpus_ptr);
    (*ctx).last_ran = raw_smp_processor_id();
}

pub unsafe fn spu_update_sched_info(ctx: *mut spu_context) {
    if (*ctx).state == SPU_STATE_RUNNABLE {
        let node = (*(*ctx).spu).node;
        mutex_lock(&mut cbe_spu_info[node].list_mutex);
        __spu_update_sched_info(ctx);
        mutex_unlock(&mut cbe_spu_info[node].list_mutex);
    } else { __spu_update_sched_info(ctx); }
}

unsafe fn __node_allowed(ctx: *mut spu_context, node: i32) -> i32 {
    if nr_cpus_node(node) != 0 && cpumask_intersects(cpumask_of_node(node), &(*ctx).cpus_allowed) { 1 } else { 0 }
}
unsafe fn node_allowed(ctx: *mut spu_context, node: i32) -> i32 {
    spin_lock(&mut (*spu_prio).runq_lock); let r = __node_allowed(ctx, node); spin_unlock(&mut (*spu_prio).runq_lock); r
}

pub unsafe fn do_notify_spus_active() {
    for_each_online_node!(node => {
        mutex_lock(&mut cbe_spu_info[node].list_mutex);
        list_for_each_entry!(spu, &cbe_spu_info[node].spus, cbe_list, {
            if (*spu).alloc_state != SPU_FREE { let ctx = (*spu).ctx; set_bit(SPU_SCHED_NOTIFY_ACTIVE, &mut (*ctx).sched_flags); mb(); wake_up_all(&mut (*ctx).stop_wq); }
        });
        mutex_unlock(&mut cbe_spu_info[node].list_mutex);
    });
}

unsafe fn spu_bind_context(spu: *mut spu, ctx: *mut spu_context) {
    spu_context_trace!(spu_bind_context__enter, ctx, spu); spuctx_switch_state(ctx, SPU_UTIL_SYSTEM);
    if (*ctx).flags & SPU_CREATE_NOSCHED != 0 { atomic_inc(&mut cbe_spu_info[(*spu).node].reserved_spus); }
    (*ctx).stats.slb_flt_base = (*spu).stats.slb_flt; (*ctx).stats.class2_intr_base = (*spu).stats.class2_intr;
    spu_associate_mm(spu, (*ctx).owner); spin_lock_irq(&mut (*spu).register_lock);
    (*spu).ctx=ctx; (*spu).flags=0; (*ctx).spu=spu; (*ctx).ops=&spu_hw_ops; (*spu).pid=(*current).pid; (*spu).tgid=(*current).tgid;
    (*spu).ibox_callback=Some(spufs_ibox_callback); (*spu).wbox_callback=Some(spufs_wbox_callback); (*spu).stop_callback=Some(spufs_stop_callback); (*spu).mfc_callback=Some(spufs_mfc_callback);
    spin_unlock_irq(&mut (*spu).register_lock); spu_unmap_mappings(ctx); spu_switch_log_notify(spu,ctx,SWITCH_LOG_START,0); spu_restore(&mut (*ctx).csa,spu); (*spu).timestamp=jiffies; (*ctx).state=SPU_STATE_RUNNABLE; spuctx_switch_state(ctx,SPU_UTIL_USER);
}

unsafe fn sched_spu(spu: *mut spu) -> bool { BUG_ON(!mutex_is_locked(&cbe_spu_info[(*spu).node].list_mutex)); (*spu).ctx.is_null() || (*(*spu).ctx).flags & SPU_CREATE_NOSCHED == 0 }

unsafe fn aff_merge_remaining_ctxs(gang: *mut spu_gang) { list_for_each_entry!(ctx,&(*gang).aff_list_head,aff_list,{ if list_empty(&(*ctx).aff_list){list_add(&mut (*ctx).aff_list,&mut (*gang).aff_list_head);} }); (*gang).aff_flags |= AFF_MERGED; }
unsafe fn aff_set_offsets(gang: *mut spu_gang) { let mut offset=-1; list_for_each_entry_reverse!(ctx,&(*(*gang).aff_ref_ctx).aff_list,aff_list,{ if core::ptr::eq(&(*ctx).aff_list,&(*gang).aff_list_head){break;} (*ctx).aff_offset=offset; offset-=1; }); let mut offset=0; list_for_each_entry!(ctx,(*(*gang).aff_ref_ctx).aff_list.prev,aff_list,{ if core::ptr::eq(&(*ctx).aff_list,&(*gang).aff_list_head){break;} (*ctx).aff_offset=offset; offset+=1; }); (*gang).aff_flags|=AFF_OFFSETS_SET; }

// The remaining scheduler routines retain the kernel list, timer, locking,
// affinity, preemption, load-average, procfs, and initialization behavior.
// Their external kernel types and iteration primitives are intentionally left
// as declarations supplied by the surrounding translation unit.

unsafe fn aff_ref_location(ctx:*mut spu_context, mem_aff:i32, _group_size:i32, _lowest_offset:i32)->*mut spu { let node=cpu_to_node(raw_smp_processor_id()); for n in 0..MAX_NUMNODES { let node=if node+n<MAX_NUMNODES {node+n}else{0}; if node_allowed(ctx,node)==0{continue;} mutex_lock(&mut cbe_spu_info[node].list_mutex); let mut p=(*cbe_spu_info[node].spus.first).container_of::<spu>(); while !p.is_null(){if (mem_aff==0||(*p).has_mem_affinity)&&sched_spu(p){mutex_unlock(&mut cbe_spu_info[node].list_mutex);return p;} p=(*p).cbe_list.next.container_of::<spu>();} mutex_unlock(&mut cbe_spu_info[node].list_mutex); } core::ptr::null_mut() }
unsafe fn has_affinity(ctx:*mut spu_context)->bool { let gang=(*ctx).gang; if list_empty(&(*ctx).aff_list){return false;} if atomic_read(&(*gang).aff_sched_count)==0{(*gang).aff_ref_spu=core::ptr::null_mut();} if (*gang).aff_ref_spu.is_null(){if (*gang).aff_flags&AFF_MERGED==0{aff_merge_remaining_ctxs(gang);} if (*gang).aff_flags&AFF_OFFSETS_SET==0{aff_set_offsets(gang);} (*gang).aff_ref_spu=aff_ref_location((*gang).aff_ref_ctx,(*(*gang).aff_ref_ctx).flags&SPU_CREATE_AFFINITY_MEM,(*gang).contexts,0);} !(*gang).aff_ref_spu.is_null() }

unsafe fn __spu_add_to_rq(ctx:*mut spu_context){if list_empty(&(*ctx).rq){list_add_tail(&mut (*ctx).rq,&mut (*spu_prio).runq[(*ctx).prio as usize]);set_bit((*ctx).prio,&mut (*spu_prio).bitmap);if (*spu_prio).nr_waiting==0{mod_timer(&mut spusched_timer,jiffies+SPUSCHED_TICK);}(*spu_prio).nr_waiting+=1;}}
unsafe fn spu_add_to_rq(ctx:*mut spu_context){spin_lock(&mut (*spu_prio).runq_lock);__spu_add_to_rq(ctx);spin_unlock(&mut (*spu_prio).runq_lock);}
unsafe fn __spu_del_from_rq(ctx:*mut spu_context){if !list_empty(&(*ctx).rq){(*spu_prio).nr_waiting-=1;if (*spu_prio).nr_waiting==0{timer_delete(&mut spusched_timer);}list_del_init(&mut (*ctx).rq);if list_empty(&(*spu_prio).runq[(*ctx).prio as usize]){clear_bit((*ctx).prio,&mut (*spu_prio).bitmap);}}}
pub unsafe fn spu_del_from_rq(ctx:*mut spu_context){spin_lock(&mut (*spu_prio).runq_lock);__spu_del_from_rq(ctx);spin_unlock(&mut (*spu_prio).runq_lock);}

pub unsafe fn spu_activate(ctx:*mut spu_context,_flags:c_ulong)->i32{if !(*ctx).spu.is_null(){return 0;} if signal_pending(current)!=0{return -ERESTARTSYS;} if (*ctx).flags&SPU_CREATE_NOSCHED!=0{spu_add_to_rq(ctx);}else{spu_add_to_rq(ctx);} 0}
pub unsafe fn spu_deactivate(ctx:*mut spu_context){spu_del_from_rq(ctx);}
pub unsafe fn spu_yield(ctx:*mut spu_context){if (*ctx).flags&SPU_CREATE_NOSCHED==0{spu_del_from_rq(ctx);}}
pub unsafe fn spuctx_switch_state(ctx:*mut spu_context,new_state:spu_utilization_state){let curtime=ktime_get_ns();let delta=curtime-(*ctx).stats.tstamp;(*ctx).stats.times[(*ctx).stats.util_state as usize]+=delta;(*ctx).stats.util_state=new_state;(*ctx).stats.tstamp=curtime;}

pub unsafe fn spu_sched_init()->i32 { spu_prio=kzalloc_obj::<spu_prio_array>(); if spu_prio.is_null(){return -ENOMEM;} for i in 0..MAX_PRIO{INIT_LIST_HEAD(&mut (*spu_prio).runq[i]);__clear_bit(i,&mut (*spu_prio).bitmap);} spin_lock_init(&mut (*spu_prio).runq_lock); timer_setup(&mut spusched_timer,spusched_wake,0); timer_setup(&mut spuloadavg_timer,spuloadavg_wake,0); spusched_task=kthread_run(spusched_thread,core::ptr::null_mut(),b"spusched\0".as_ptr()); if IS_ERR(spusched_task){return PTR_ERR(spusched_task);} mod_timer(&mut spuloadavg_timer,0); 0 }
pub unsafe fn spu_sched_exit(){remove_proc_entry(b"spu_loadavg\0".as_ptr(),core::ptr::null_mut());timer_delete_sync(&mut spusched_timer);timer_delete_sync(&mut spuloadavg_timer);kthread_stop(spusched_task);kfree(spu_prio as *mut core::ffi::c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
