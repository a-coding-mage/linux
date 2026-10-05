// SPDX-License-Identifier: GPL-2.0-only
// core.c:9350..9876. Root/task-group storage and registration tables supplied by
// the sole native authority; callback algorithms and RCU lifetime stay Rust.
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn alloc_uclamp_sched_group(tg: *mut task_group, parent: *mut task_group) {
    #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
    // SAFETY: The creator owns a new child distinct from its live parent;
    // configured native clamp IDs bound initialization and nonoverlapping copies.
    unsafe {
        for id in 0..LUPOS_CORE_UCLAMP_CNT as usize {
            lupos_core_uclamp_se_set(
                addr_of_mut!((*tg).uclamp_req[id]),
                lupos_core_uclamp_none(id as uclamp_id),
                false,
            );
            core::ptr::copy_nonoverlapping(
                addr_of!((*parent).uclamp[id]),
                addr_of_mut!((*tg).uclamp[id]),
                1,
            );
        }
    }
    #[cfg(not(CONFIG_UCLAMP_TASK_GROUP))]
    {
        let _ = (tg, parent);
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn sched_free_group(tg: *mut task_group) {
    // SAFETY: The caller owns the dead/unpublished group and has completed
    // required grace periods; native class/autogroup teardown precedes cache free.
    unsafe {
        lupos_core_header_free_fair_sched_group(tg);
        free_rt_sched_group(tg);
        lupos_core_header_autogroup_free(tg);
        kmem_cache_free(*lupos_core_task_group_cache_slot(), tg.cast());
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe extern "C" fn sched_free_group_rcu(rcu: *mut rcu_head) {
    // SAFETY: The RCU callback receives the embedded node of the retired
    // group after its required grace period and owns its final reclamation.
    unsafe {
        sched_free_group(rcu.cast::<u8>().sub(offset_of!(task_group, rcu)).cast());
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn sched_unregister_group(tg: *mut task_group) {
    // SAFETY: The caller owns the unlinked group after its first grace
    // period; native unregister precedes the second deferred free callback.
    unsafe {
        lupos_core_header_unregister_fair_sched_group(tg);
        unregister_rt_sched_group(tg);
        call_rcu(addr_of_mut!((*tg).rcu), Some(sched_free_group_rcu));
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_create_group(parent: *mut task_group) -> *mut task_group {
    // SAFETY: The caller pins parent; native allocation yields exclusive
    // child storage, and every class-allocation failure reaches group cleanup.
    unsafe {
        let tg = lupos_core_alloc_sched_group(
            *lupos_core_task_group_cache_slot(),
            LUPOS_CORE_GFP_KERNEL | LUPOS_CORE___GFP_ZERO,
        )
        .cast::<task_group>();
        if tg.is_null() {
            return lupos_core_err_ptr(-(LUPOS_CORE_ENOMEM as c_long)).cast();
        }
        if lupos_core_header_alloc_fair_sched_group(tg, parent) == 0
            || alloc_rt_sched_group(tg, parent) == 0
        {
            sched_free_group(tg);
            return lupos_core_err_ptr(-(LUPOS_CORE_ENOMEM as c_long)).cast();
        }
        lupos_core_header_scx_tg_init(tg);
        alloc_uclamp_sched_group(tg, parent);
        tg
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_online_group(tg: *mut task_group, parent: *mut task_group) {
    // SAFETY: The cgroup caller pins the new group and its parent; native
    // group locking protects list publication and child/parent linkage.
    unsafe {
        let lock = lupos_core_task_group_lock();
        let flags = lupos_core_spin_lock_irqsave(lock);
        lupos_core_list_add_tail_rcu(addr_of_mut!((*tg).list), lupos_core_task_groups());
        lupos_core_warn_site_9445(parent.is_null());
        (*tg).parent = parent;
        lupos_core_init_list_head(addr_of_mut!((*tg).children));
        lupos_core_list_add_rcu(
            addr_of_mut!((*tg).siblings),
            addr_of_mut!((*parent).children),
        );
        lupos_core_spin_unlock_irqrestore(lock, flags);
        lupos_core_header_online_fair_sched_group(tg);
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe extern "C" fn sched_unregister_group_rcu(rcu: *mut rcu_head) {
    // SAFETY: The callback receives a retired group's embedded RCU node;
    // its first grace period permits class unregister and the second deferred free.
    unsafe {
        sched_unregister_group(rcu.cast::<u8>().sub(offset_of!(task_group, rcu)).cast());
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_destroy_group(tg: *mut task_group) {
    // SAFETY: The caller owns the retired live group and its RCU node;
    // no competing callback uses that node while unregister is queued.
    unsafe {
        call_rcu(addr_of_mut!((*tg).rcu), Some(sched_unregister_group_rcu));
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_release_group(tg: *mut task_group) {
    // SAFETY: The cgroup caller keeps tg live; native group locking
    // protects list removal before later grace-period-based unregister/free.
    unsafe {
        let lock = lupos_core_task_group_lock();
        let flags = lupos_core_spin_lock_irqsave(lock);
        lupos_core_list_del_rcu(addr_of_mut!((*tg).list));
        lupos_core_list_del_rcu(addr_of_mut!((*tg).siblings));
        lupos_core_spin_unlock_irqrestore(lock, flags);
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn sched_change_group(tsk: *mut task_struct) {
    // SAFETY: The caller holds the task-rq locks and pins the new cgroup;
    // class callback or native set_task_rq updates that live task's associations.
    unsafe {
        let css = lupos_core_task_css_check_cpu(tsk, true);
        let mut tg = css
            .cast::<u8>()
            .sub(offset_of!(task_group, css))
            .cast::<task_group>();
        tg = lupos_core_autogroup_task_group(tsk, tg);
        (*tsk).sched_task_group = tg;
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        if let Some(cb) = (*(*tsk).sched_class).task_change_group {
            cb(tsk);
            return;
        }
        lupos_core_header_set_task_rq(tsk, lupos_core_task_cpu(tsk) as c_uint);
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_move_task(tsk: *mut task_struct, for_autogroup: bool) {
    // SAFETY: The caller keeps tsk/new cgroup live. Task-rq locking spans
    // class-change scope, wakeup/reschedule decisions and balance callbacks.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        let rq = _task_rq_lock(tsk, rf);
        let ctx = sched_change_begin(
            tsk,
            (LUPOS_CORE_DEQUEUE_SAVE | LUPOS_CORE_DEQUEUE_MOVE) as c_uint,
        );
        sched_change_group(tsk);
        if !for_autogroup {
            lupos_core_header_scx_cgroup_move_task(tsk);
        }
        let resched = (*ctx).running;
        let queued = (*ctx).queued;
        sched_change_end(ctx);
        if resched {
            resched_curr(rq);
        } else if queued {
            wakeup_preempt(rq, tsk, 0);
        }
        __balance_callbacks(rq, rf);
        lupos_core_task_rq_unlock(rq, tsk, rf);
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_css_alloc(
    parent_css: *mut cgroup_subsys_state,
) -> *mut cgroup_subsys_state {
    // SAFETY: The cgroup core pins parent_css when non-null; allocation
    // returns the embedded CSS of an owned group or the permanent root CSS.
    unsafe {
        let parent = lupos_core_css_tg(parent_css);
        if parent.is_null() {
            return addr_of_mut!(root_task_group.css);
        }
        let tg = sched_create_group(parent);
        if lupos_core_is_err(tg.cast()) {
            return lupos_core_err_ptr(-(LUPOS_CORE_ENOMEM as c_long)).cast();
        }
        addr_of_mut!((*tg).css)
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_css_online(css: *mut cgroup_subsys_state) -> c_int {
    // SAFETY: The cgroup core pins css/parent. Group publication and
    // clamp mutex/RCU propagation complete before successful online return.
    unsafe {
        let tg = lupos_core_css_tg(css);
        let parent = lupos_core_css_tg((*css).parent);
        let ret = lupos_core_header_scx_tg_online(tg);
        if ret != 0 {
            return ret;
        }
        if !parent.is_null() {
            sched_online_group(tg, parent);
        }
        #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
        {
            lupos_core_mutex_lock(addr_of_mut!(lupos_core_uclamp_mutex));
            lupos_core_rcu_read_lock();
            cpu_util_update_eff(css);
            lupos_core_rcu_read_unlock();
            lupos_core_mutex_unlock(addr_of_mut!(lupos_core_uclamp_mutex));
        }
        0
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_css_offline(css: *mut cgroup_subsys_state) {
    // SAFETY: The cgroup core pins css while the configured native
    // scheduler-extension hook processes group offlining.
    unsafe {
        lupos_core_header_scx_tg_offline(lupos_core_css_tg(css));
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_css_released(css: *mut cgroup_subsys_state) {
    // SAFETY: The cgroup core owns the released CSS and keeps its group
    // live while it is removed from native RCU-visible lists.
    unsafe {
        sched_release_group(lupos_core_css_tg(css));
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_css_free(css: *mut cgroup_subsys_state) {
    // SAFETY: The cgroup core has completed the release grace period;
    // class unregister and final deferred reclamation own the live group.
    unsafe {
        sched_unregister_group(lupos_core_css_tg(css));
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_can_attach(tset: *mut cgroup_taskset) -> c_int {
    // SAFETY: The cgroup core pins the taskset and target groups; native
    // iteration keeps tasks valid for RT and scheduler-extension admission checks.
    unsafe {
        #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
        if lupos_core_rt_group_sched_enabled() {
            let mut css = null_mut();
            let mut task = cgroup_taskset_first(tset, &mut css);
            while !task.is_null() {
                if sched_rt_can_attach(lupos_core_css_tg(css), task) == 0 {
                    return -(LUPOS_CORE_EINVAL as c_int);
                }
                task = cgroup_taskset_next(tset, &mut css);
            }
        }
        lupos_core_header_scx_cgroup_can_attach(tset)
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_attach(tset: *mut cgroup_taskset) {
    // SAFETY: The cgroup core pins the taskset/new CSS associations while
    // native iteration and task-rq-locked group movement process each task.
    unsafe {
        let mut css = null_mut();
        let mut task = cgroup_taskset_first(tset, &mut css);
        while !task.is_null() {
            sched_move_task(task, false);
            task = cgroup_taskset_next(tset, &mut css);
        }
    }
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_cancel_attach(tset: *mut cgroup_taskset) {
    // SAFETY: The cgroup core supplies a live taskset to the configured
    // native scheduler-extension cancellation callback.
    unsafe {
        lupos_core_header_scx_cgroup_cancel_attach(tset);
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
unsafe fn cpu_util_update_eff(mut css: *mut cgroup_subsys_state) {
    // SAFETY: The caller holds the clamp mutex and RCU protection for
    // the live subtree; configured IDs bound native clamp arrays and propagation.
    unsafe {
        let top = css;
        lupos_core_assert_mutex_held(addr_of_mut!(lupos_core_uclamp_mutex));
        lupos_core_warn_once_site_9660(!lupos_core_rcu_read_lock_held());
        css = css_next_descendant_pre(null_mut(), top);
        while !css.is_null() {
            let tg = lupos_core_css_tg(css);
            let parent = (*tg).parent;
            let mut eff = [0 as c_uint; LUPOS_CORE_UCLAMP_CNT as usize];
            for id in 0..LUPOS_CORE_UCLAMP_CNT as usize {
                eff[id] = lupos_core_uclamp_se_value(addr_of!((*tg).uclamp_req[id]));
                if !parent.is_null() {
                    eff[id] = eff[id].min(lupos_core_uclamp_se_value(addr_of!((*parent).uclamp[id])));
                }
            }
            eff[LUPOS_CORE_UCLAMP_MIN as usize] =
                eff[LUPOS_CORE_UCLAMP_MIN as usize].min(eff[LUPOS_CORE_UCLAMP_MAX as usize]);
            let mut clamps: c_uint = 0;
            for id in 0..LUPOS_CORE_UCLAMP_CNT as usize {
                let uc = addr_of_mut!((*tg).uclamp[id]);
                if eff[id] != lupos_core_uclamp_se_value(uc) {
                    lupos_core_uclamp_se_set_value(uc, eff[id]);
                    lupos_core_uclamp_se_set_bucket_id(uc, lupos_core_uclamp_bucket_id(eff[id]));
                    clamps |= (1 as c_uint).wrapping_shl(id as u32);
                }
            }
            if clamps == 0 {
                css = css_rightmost_descendant(css);
            } else {
                uclamp_update_active_tasks(css);
            }
            css = css_next_descendant_pre(css, top);
        }
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
unsafe fn capacity_from_percent(mut buf: *mut c_char) -> uclamp_request {
    // SAFETY: buf is live writable NUL-terminated input; native trimming
    // and parsing use a valid local result and native unsigned decimal shift.
    unsafe {
        let mut req = MaybeUninit::<uclamp_request>::zeroed().assume_init();
        req.percent = LUPOS_CORE_UCLAMP_PERCENT_SCALE as i64;
        req.util = LUPOS_CORE_SCHED_CAPACITY_SCALE as u64;
        buf = strim(buf);
        if strcmp(buf, b"max\0".as_ptr().cast()) != 0 {
            req.ret = cgroup_parse_float(
                buf,
                LUPOS_CORE_UCLAMP_PERCENT_SHIFT,
                &mut req.percent,
            );
            if req.ret != 0 {
                return req;
            }
            if req.percent as u64 > LUPOS_CORE_UCLAMP_PERCENT_SCALE as u64 {
                req.ret = -(LUPOS_CORE_ERANGE as c_int);
                return req;
            }
            req.util = (req.percent as u64).wrapping_shl(LUPOS_CORE_SCHED_CAPACITY_SHIFT);
            req.util = req
                .util
                .wrapping_add((LUPOS_CORE_UCLAMP_PERCENT_SCALE as u64) / 2)
                / LUPOS_CORE_UCLAMP_PERCENT_SCALE as u64;
        }
        req
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
unsafe fn cpu_uclamp_write(
    of: *mut kernfs_open_file,
    buf: *mut c_char,
    nbytes: usize,
    _off: loff_t,
    id: uclamp_id,
) -> isize {
    // SAFETY: The cgroup file pins of/CSS and supplies writable input.
    // Clamp mutex and RCU protect request publication and descendant updates.
    unsafe {
        let req = capacity_from_percent(buf);
        if req.ret != 0 {
            return req.ret as isize;
        }
        lupos_core_header_sched_uclamp_enable();
        lupos_core_mutex_lock(addr_of_mut!(lupos_core_uclamp_mutex));
        lupos_core_rcu_read_lock();
        let tg = lupos_core_css_tg(lupos_core_of_css(of));
        let idx = id as usize;
        if lupos_core_uclamp_se_value(addr_of!((*tg).uclamp_req[idx])) as u64 != req.util {
            lupos_core_uclamp_se_set(
                addr_of_mut!((*tg).uclamp_req[idx]),
                req.util as c_uint,
                false,
            );
        }
        (*tg).uclamp_pct[idx] = req.percent as _;
        cpu_util_update_eff(lupos_core_of_css(of));
        lupos_core_rcu_read_unlock();
        lupos_core_mutex_unlock(addr_of_mut!(lupos_core_uclamp_mutex));
        nbytes as isize
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
#[no_mangle]
pub unsafe extern "C" fn cpu_uclamp_min_write(
    of: *mut kernfs_open_file,
    buf: *mut c_char,
    n: usize,
    off: loff_t,
) -> isize {
    // SAFETY: The cgroup write callback supplies live file/input storage;
    // the shared writer receives the native minimum-clamp identifier.
    unsafe {
        cpu_uclamp_write(of, buf, n, off, LUPOS_CORE_UCLAMP_MIN as uclamp_id)
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
#[no_mangle]
pub unsafe extern "C" fn cpu_uclamp_max_write(
    of: *mut kernfs_open_file,
    buf: *mut c_char,
    n: usize,
    off: loff_t,
) -> isize {
    // SAFETY: The cgroup write callback supplies live file/input storage;
    // the shared writer receives the native maximum-clamp identifier.
    unsafe {
        cpu_uclamp_write(of, buf, n, off, LUPOS_CORE_UCLAMP_MAX as uclamp_id)
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
unsafe fn cpu_uclamp_print(sf: *mut seq_file, id: uclamp_id) {
    // SAFETY: The seq-file pins its CSS/group beyond the RCU snapshot; id
    // is a native clamp ID and output uses the live seq buffer.
    unsafe {
        lupos_core_rcu_read_lock();
        let tg = lupos_core_css_tg(lupos_core_seq_css(sf));
        let util = lupos_core_uclamp_se_value(addr_of!((*tg).uclamp_req[id as usize])) as u64;
        lupos_core_rcu_read_unlock();
        if util == LUPOS_CORE_SCHED_CAPACITY_SCALE as u64 {
            lupos_core_header_seq_puts(sf, b"max\n\0".as_ptr().cast());
            return;
        }
        // Preserve original lifetime: seq_css reference pins tg beyond RCU unlock.
        let percent = (*tg).uclamp_pct[id as usize] as u64;
        let scale = LUPOS_CORE_UCLAMP_PERCENT_POW10 as u64;
        let rem = (percent % scale) as u32;
        seq_printf(
            sf,
            b"%llu.%0*u\n\0".as_ptr().cast(),
            percent / scale,
            LUPOS_CORE_UCLAMP_PERCENT_SHIFT as c_int,
            rem,
        );
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
#[no_mangle]
pub unsafe extern "C" fn cpu_uclamp_min_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The cgroup seq callback pins sf and its group; the native
    // minimum-clamp identifier bounds the shared printing path.
    unsafe {
        cpu_uclamp_print(sf, LUPOS_CORE_UCLAMP_MIN as uclamp_id);
        0
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_UCLAMP_TASK_GROUP))]
#[no_mangle]
pub unsafe extern "C" fn cpu_uclamp_max_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The cgroup seq callback pins sf and its group; the native
    // maximum-clamp identifier bounds the shared printing path.
    unsafe {
        cpu_uclamp_print(sf, LUPOS_CORE_UCLAMP_MAX as uclamp_id);
        0
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
unsafe fn tg_weight(tg: *mut task_group) -> c_ulong {
    // SAFETY: The caller pins tg under the native cgroup read contract;
    // configured CFS or extension storage supplies its group weight.
    unsafe {
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        {
            lupos_core_scale_load_down((*tg).shares)
        }
        #[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
        {
            lupos_core_sched_weight_from_cgroup((*tg).scx.weight as c_ulong)
        }
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_shares_write_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    mut value: u64,
) -> c_int {
    // SAFETY: The cgroup callback pins css/tg; native share/extension
    // setters provide their own update serialization after value bounds handling.
    unsafe {
        if value > lupos_core_scale_load_down(c_ulong::MAX) as u64 {
            value = LUPOS_CORE_MAX_SHARES as u64;
        }
        let tg = lupos_core_css_tg(css);
        let ret = lupos_core_header_sched_group_set_shares(tg, lupos_core_scale_load(value as c_ulong));
        if ret == 0 {
            lupos_core_header_scx_group_set_weight(
                tg,
                lupos_core_sched_weight_to_cgroup(value as c_ulong),
            );
        }
        ret
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_shares_read_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> u64 {
    // SAFETY: The cgroup callback pins css/tg while the native configured
    // weight accessor supplies the reported value.
    unsafe {
        tg_weight(lupos_core_css_tg(css)) as u64
    }
}
