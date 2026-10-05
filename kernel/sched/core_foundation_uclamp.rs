// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_idle_value(rq: *mut rq, id: uclamp_id, value: c_uint) -> c_uint {
    // SAFETY: The caller holds the live rq's lock and supplies a valid
    // clamp ID; the native constants define the idle-retention policy.
    unsafe {
        if id == LUPOS_CORE_UCLAMP_MAX {
            (*rq).uclamp_flags |= LUPOS_CORE_UCLAMP_FLAG_IDLE;
            return value;
        }
        lupos_core_uclamp_none(LUPOS_CORE_UCLAMP_MIN)
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_idle_reset(rq: *mut rq, id: uclamp_id, value: c_uint) {
    // SAFETY: The caller holds the live rq's lock with a valid clamp ID and
    // value; native clamp storage remains valid through the update.
    unsafe {
        if (*rq).uclamp_flags & LUPOS_CORE_UCLAMP_FLAG_IDLE == 0 {
            return;
        }
        lupos_core_uclamp_rq_set(rq, id, value);
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_max_value(rq: *mut rq, id: uclamp_id, value: c_uint) -> c_uint {
    // SAFETY: The caller holds rq's lock and supplies a valid clamp ID.
    // The configured bucket count bounds every native bucket access.
    unsafe {
        let bucket = addr_of!((*rq).uclamp[id as usize].bucket).cast::<uclamp_bucket>();
        let mut i = LUPOS_CORE_UCLAMP_BUCKETS as c_int - 1;
        while i >= 0 {
            let b = bucket.add(i as usize);
            if (*b).tasks() != 0 {
                return (*b).value() as c_uint;
            }
            i -= 1;
        }
        uclamp_idle_value(rq, id, value)
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn __uclamp_update_util_min_rt_default(p: *mut task_struct) {
    // SAFETY: The caller keeps p live and holds its pi lock, serializing
    // request updates against task migration and explicit user clamp settings.
    unsafe {
        lupos_core_assert_pi_lock(p);
        let uc = addr_of_mut!((*p).uclamp_req[LUPOS_CORE_UCLAMP_MIN as usize]);
        if (*uc).user_defined() != 0 {
            return;
        }
        lupos_core_uclamp_se_set(uc, sysctl_sched_uclamp_util_min_rt_default, false);
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_update_util_min_rt_default(p: *mut task_struct) {
    // SAFETY: The caller keeps p live; the acquired task-rq locks protect
    // the RT default update and are released before returning.
    unsafe {
        if !lupos_core_rt_task(p) {
            return;
        }
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = _task_rq_lock(p, rf.as_mut_ptr());
        __uclamp_update_util_min_rt_default(p);
        lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_tg_restrict(p: *mut task_struct, id: uclamp_id) -> uclamp_se {
    // SAFETY: The caller keeps p and its task group live under the native
    // clamp-read contract; id indexes a configured clamp and only a copy changes.
    unsafe {
        let uc = (*p).uclamp_req[id as usize];
        #[cfg(CONFIG_UCLAMP_TASK_GROUP)]
        {
            let mut uc = uc;
            let tg = lupos_core_task_group(p);
            if lupos_core_task_group_is_autogroup(tg) || tg == addr_of_mut!(root_task_group) {
                return uc;
            }
            let min = (*tg).uclamp[LUPOS_CORE_UCLAMP_MIN as usize].value();
            let max = (*tg).uclamp[LUPOS_CORE_UCLAMP_MAX as usize].value();
            let value = core::cmp::min(core::cmp::max(uc.value(), min), max);
            lupos_core_uclamp_se_set(addr_of_mut!(uc), value, false);
            uc
        }
        #[cfg(not(CONFIG_UCLAMP_TASK_GROUP))]
        {
            uc
        }
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_eff_get(p: *mut task_struct, id: uclamp_id) -> uclamp_se {
    // SAFETY: The caller keeps p/group state live under native clamp
    // serialization, and id is valid for task and global clamp arrays.
    unsafe {
        let requested = uclamp_tg_restrict(p, id);
        let max = lupos_core_uclamp_default[id as usize];
        if requested.value() > max.value() {
            max
        } else {
            requested
        }
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
#[no_mangle]
pub unsafe extern "C" fn uclamp_eff_value(p: *mut task_struct, id: uclamp_id) -> c_ulong {
    // SAFETY: The caller keeps p live and supplies a valid clamp ID under
    // the native effective-clamp read contract, including group lifetime.
    unsafe {
        if (*p).uclamp[id as usize].active() != 0 {
            return (*p).uclamp[id as usize].value() as c_ulong;
        }
        uclamp_eff_get(p, id).value() as c_ulong
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_inc_id(rq: *mut rq, p: *mut task_struct, id: uclamp_id) {
    // SAFETY: The caller holds the live rq's lock and keeps p live with a
    // valid clamp ID; configured bucket IDs stay within the native bucket array.
    unsafe {
        let uc_rq = addr_of_mut!((*rq).uclamp[id as usize]);
        let uc_se = addr_of_mut!((*p).uclamp[id as usize]);
        lupos_core_assert_rq_held(rq);
        *uc_se = uclamp_eff_get(p, id);
        let bucket = addr_of_mut!((*uc_rq).bucket)
            .cast::<uclamp_bucket>()
            .add((*uc_se).bucket_id() as usize);
        (*bucket).set_tasks((*bucket).tasks().wrapping_add(1));
        (*uc_se).set_active(1);
        uclamp_idle_reset(rq, id, (*uc_se).value());
        if (*bucket).tasks() == 1 || (*uc_se).value() as c_ulong > (*bucket).value() {
            (*bucket).set_value((*uc_se).value() as c_ulong);
        }
        if (*uc_se).value() > lupos_core_uclamp_rq_get(rq, id) {
            lupos_core_uclamp_rq_set(rq, id, (*uc_se).value());
        }
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_dec_id(rq: *mut rq, p: *mut task_struct, id: uclamp_id) {
    // SAFETY: The caller holds rq's lock and keeps p live; only active
    // clamps own bucket references, and their native bucket IDs remain valid.
    unsafe {
        let uc_rq = addr_of_mut!((*rq).uclamp[id as usize]);
        let uc_se = addr_of_mut!((*p).uclamp[id as usize]);
        lupos_core_assert_rq_held(rq);
        if (*uc_se).active() == 0 {
            return;
        }
        let bucket = addr_of_mut!((*uc_rq).bucket)
            .cast::<uclamp_bucket>()
            .add((*uc_se).bucket_id() as usize);
        lupos_core_warn_uclamp_bucket_empty((*bucket).tasks() == 0);
        if (*bucket).tasks() != 0 {
            (*bucket).set_tasks((*bucket).tasks().wrapping_sub(1));
        }
        (*uc_se).set_active(0);
        if (*bucket).tasks() != 0 {
            return;
        }
        let rq_clamp = lupos_core_uclamp_rq_get(rq, id);
        lupos_core_warn_uclamp_bucket_value((*bucket).value() > rq_clamp as c_ulong);
        if (*bucket).value() >= rq_clamp as c_ulong {
            let value = uclamp_rq_max_value(rq, id, (*uc_se).value());
            lupos_core_uclamp_rq_set(rq, id, value);
        }
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_inc(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    // SAFETY: The enqueue caller holds rq's lock and keeps p/class state
    // live; native clamp IDs bound iteration and delayed-task flags gate updates.
    unsafe {
        if !lupos_core_uclamp_is_used() || (*(*p).sched_class).uclamp_enabled == 0 {
            return;
        }
        if (*p).se.sched_delayed != 0 && flags & LUPOS_CORE_ENQUEUE_DELAYED == 0 {
            return;
        }
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            uclamp_rq_inc_id(rq, p, id);
            id += 1;
        }
        if (*rq).uclamp_flags & LUPOS_CORE_UCLAMP_FLAG_IDLE != 0 {
            (*rq).uclamp_flags &= !LUPOS_CORE_UCLAMP_FLAG_IDLE;
        }
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn uclamp_rq_inc(_rq: *mut rq, _p: *mut task_struct, _flags: c_int) {}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_dec(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The dequeue caller holds rq's lock and keeps p live; inactive
    // and delayed tasks do not release clamp references they do not own.
    unsafe {
        if !lupos_core_uclamp_is_used()
            || (*(*p).sched_class).uclamp_enabled == 0
            || (*p).se.sched_delayed != 0
        {
            return;
        }
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            uclamp_rq_dec_id(rq, p, id);
            id += 1;
        }
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn uclamp_rq_dec(_rq: *mut rq, _p: *mut task_struct) {}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_rq_reinc_id(rq: *mut rq, p: *mut task_struct, id: uclamp_id) {
    // SAFETY: The caller holds rq's lock and keeps p live with a valid
    // clamp ID; the paired decrement/increment preserves active bucket ownership.
    unsafe {
        if (*p).uclamp[id as usize].active() == 0 {
            return;
        }
        uclamp_rq_dec_id(rq, p, id);
        uclamp_rq_inc_id(rq, p, id);
        if id == LUPOS_CORE_UCLAMP_MAX && (*rq).uclamp_flags & LUPOS_CORE_UCLAMP_FLAG_IDLE != 0 {
            (*rq).uclamp_flags &= !LUPOS_CORE_UCLAMP_FLAG_IDLE;
        }
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_update_active(p: *mut task_struct) {
    // SAFETY: The caller keeps p live; task-rq locking serializes clamp
    // reaccounting against enqueue/dequeue and migration through every native ID.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = _task_rq_lock(p, rf.as_mut_ptr());
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            uclamp_rq_reinc_id(rq, p, id);
            id += 1;
        }
        lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
    }
}
#[cfg(all(CONFIG_UCLAMP_TASK, CONFIG_UCLAMP_TASK_GROUP))]
unsafe fn uclamp_update_active_tasks(css: *mut cgroup_subsys_state) {
    // SAFETY: css remains live under caller cgroup synchronization. The
    // native iterator keeps each task valid through its locked clamp update.
    unsafe {
        let mut it = MaybeUninit::<css_task_iter>::uninit();
        css_task_iter_start(css, 0, it.as_mut_ptr());
        loop {
            let p = css_task_iter_next(it.as_mut_ptr());
            if p.is_null() {
                break;
            }
            uclamp_update_active(p);
        }
        css_task_iter_end(it.as_mut_ptr());
    }
}
#[cfg(all(CONFIG_UCLAMP_TASK, CONFIG_SYSCTL))]
#[cfg(CONFIG_UCLAMP_TASK_GROUP)]
unsafe fn uclamp_update_root_tg() {
    // SAFETY: The sysctl caller holds the clamp mutex; root group storage
    // is permanent and RCU protects descendant effective-value propagation.
    unsafe {
        lupos_core_uclamp_se_set(
            addr_of_mut!(root_task_group.uclamp_req[LUPOS_CORE_UCLAMP_MIN as usize]),
            lupos_core_sysctl_sched_uclamp_util_min,
            false,
        );
        lupos_core_uclamp_se_set(
            addr_of_mut!(root_task_group.uclamp_req[LUPOS_CORE_UCLAMP_MAX as usize]),
            lupos_core_sysctl_sched_uclamp_util_max,
            false,
        );
        lupos_core_rcu_read_lock();
        cpu_util_update_eff(addr_of_mut!(root_task_group.css));
        lupos_core_rcu_read_unlock();
    }
}
#[cfg(all(CONFIG_UCLAMP_TASK, CONFIG_SYSCTL, not(CONFIG_UCLAMP_TASK_GROUP)))]
unsafe fn uclamp_update_root_tg() {}
#[cfg(all(CONFIG_UCLAMP_TASK, CONFIG_SYSCTL))]
unsafe fn uclamp_sync_util_min_rt_default() {
    // SAFETY: The caller serializes global clamp changes. The tasklist
    // barrier pairs with fork publication; RCU protects native process iteration.
    unsafe {
        lupos_core_read_lock_tasklist();
        lupos_core_smp_mb_after_spinlock();
        lupos_core_read_unlock_tasklist();
        lupos_core_rcu_read_lock();
        let mut g = lupos_core_next_task(addr_of_mut!(init_task));
        while g != addr_of_mut!(init_task) {
            let mut p = lupos_core_first_thread(g);
            while !p.is_null() {
                uclamp_update_util_min_rt_default(p);
                p = lupos_core_next_thread_in_group(g, p);
            }
            g = lupos_core_next_task(g);
        }
        lupos_core_rcu_read_unlock();
    }
}
#[cfg(all(CONFIG_UCLAMP_TASK, CONFIG_SYSCTL))]
#[no_mangle]
pub unsafe extern "C" fn lupos_core_sysctl_sched_uclamp_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    len: *mut usize,
    pos: *mut loff_t,
) -> c_int {
    // SAFETY: The sysctl caller supplies valid table/buffer/length/position
    // pointers. The clamp mutex covers validation, propagation and rollback.
    unsafe {
        lupos_core_mutex_lock(addr_of_mut!(lupos_core_uclamp_mutex));
        let old_min = lupos_core_sysctl_sched_uclamp_util_min;
        let old_max = lupos_core_sysctl_sched_uclamp_util_max;
        let old_rt = sysctl_sched_uclamp_util_min_rt_default;
        let mut result = proc_dointvec(table, write, buffer, len, pos);
        if result == 0 && write != 0 {
            if lupos_core_sysctl_sched_uclamp_util_min > lupos_core_sysctl_sched_uclamp_util_max
                || lupos_core_sysctl_sched_uclamp_util_max > LUPOS_CORE_SCHED_CAPACITY_SCALE
                || sysctl_sched_uclamp_util_min_rt_default > LUPOS_CORE_SCHED_CAPACITY_SCALE
            {
                result = -LUPOS_CORE_EINVAL;
            } else {
                let mut update_root = false;
                if old_min != lupos_core_sysctl_sched_uclamp_util_min {
                    lupos_core_uclamp_se_set(
                        addr_of_mut!(lupos_core_uclamp_default[LUPOS_CORE_UCLAMP_MIN as usize]),
                        lupos_core_sysctl_sched_uclamp_util_min,
                        false,
                    );
                    update_root = true;
                }
                if old_max != lupos_core_sysctl_sched_uclamp_util_max {
                    lupos_core_uclamp_se_set(
                        addr_of_mut!(lupos_core_uclamp_default[LUPOS_CORE_UCLAMP_MAX as usize]),
                        lupos_core_sysctl_sched_uclamp_util_max,
                        false,
                    );
                    update_root = true;
                }
                if update_root {
                    lupos_core_sched_uclamp_enable();
                    uclamp_update_root_tg();
                }
                if old_rt != sysctl_sched_uclamp_util_min_rt_default {
                    lupos_core_sched_uclamp_enable();
                    uclamp_sync_util_min_rt_default();
                }
            }
        }
        if result != 0 {
            lupos_core_sysctl_sched_uclamp_util_min = old_min;
            lupos_core_sysctl_sched_uclamp_util_max = old_max;
            sysctl_sched_uclamp_util_min_rt_default = old_rt;
        }
        lupos_core_mutex_unlock(addr_of_mut!(lupos_core_uclamp_mutex));
        result
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_fork(p: *mut task_struct) {
    // SAFETY: The fork caller owns the unpublished child, so its configured
    // clamp arrays can be reset without task-rq locking.
    unsafe {
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            (*p).uclamp[id as usize].set_active(0);
            id += 1;
        }
        if lupos_core_task_reset_on_fork(p) == 0 {
            return;
        }
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            lupos_core_uclamp_se_set(
                addr_of_mut!((*p).uclamp_req[id as usize]),
                lupos_core_uclamp_none(id),
                false,
            );
            id += 1;
        }
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn uclamp_fork(_p: *mut task_struct) {}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_post_fork(p: *mut task_struct) {
    // SAFETY: The fork caller keeps p live while the task-rq-locked helper
    // synchronizes the applicable RT default clamp request.
    unsafe {
        uclamp_update_util_min_rt_default(p);
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn uclamp_post_fork(_p: *mut task_struct) {}
#[cfg(CONFIG_UCLAMP_TASK)]
#[link_section = ".init.text"]
unsafe fn init_uclamp_rq(rq: *mut rq) {
    // SAFETY: The boot caller exclusively initializes a live rq before
    // scheduling uses its configured clamp arrays and bucket state.
    unsafe {
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            let mut uc: uclamp_rq = core::mem::zeroed();
            uc.value = lupos_core_uclamp_none(id);
            (*rq).uclamp[id as usize] = uc;
            id += 1;
        }
        (*rq).uclamp_flags = LUPOS_CORE_UCLAMP_FLAG_IDLE;
    }
}
#[cfg_attr(CONFIG_UCLAMP_TASK, link_section = ".init.text")]
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn init_uclamp() {
    // SAFETY: The boot caller exclusively initializes permanent runqueue,
    // initial-task and default/root-group clamp state before normal use.
    unsafe {
        let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
        while cpu < lupos_core_nr_cpu_ids() {
            init_uclamp_rq(lupos_core_cpu_rq(cpu));
            cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
        }
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            lupos_core_uclamp_se_set(
                addr_of_mut!(init_task.uclamp_req[id as usize]),
                lupos_core_uclamp_none(id),
                false,
            );
            id += 1;
        }
        let mut uc_max: uclamp_se = core::mem::zeroed();
        lupos_core_uclamp_se_set(
            addr_of_mut!(uc_max),
            lupos_core_uclamp_none(LUPOS_CORE_UCLAMP_MAX),
            false,
        );
        let mut id = 0;
        while id < LUPOS_CORE_UCLAMP_CNT {
            lupos_core_uclamp_default[id as usize] = uc_max;
            #[cfg(CONFIG_UCLAMP_TASK_GROUP)]
            {
                root_task_group.uclamp_req[id as usize] = uc_max;
                root_task_group.uclamp[id as usize] = uc_max;
            }
            id += 1;
        }
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn init_uclamp() {}
