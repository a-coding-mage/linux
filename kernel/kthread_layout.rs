// SPDX-License-Identifier: GPL-2.0-only
// Assertions use the same configured native headers as the object.
const _: () = {
    assert!(core::mem::size_of::<task_struct>() == LUPOS_KTHREAD_SIZE_task_struct as usize);
    assert!(core::mem::align_of::<task_struct>() == LUPOS_KTHREAD_ALIGN_task_struct as usize);
    assert!(
        core::mem::offset_of!(task_struct, flags)
            == LUPOS_KTHREAD_OFFSET_task_struct_flags as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, worker_private)
            == LUPOS_KTHREAD_OFFSET_task_struct_worker_private as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, vfork_done)
            == LUPOS_KTHREAD_OFFSET_task_struct_vfork_done as usize
    );
    assert!(core::mem::offset_of!(task_struct, mm) == LUPOS_KTHREAD_OFFSET_task_struct_mm as usize);
    assert!(
        core::mem::offset_of!(task_struct, active_mm)
            == LUPOS_KTHREAD_OFFSET_task_struct_active_mm as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, pi_lock)
            == LUPOS_KTHREAD_OFFSET_task_struct_pi_lock as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, comm) == LUPOS_KTHREAD_OFFSET_task_struct_comm as usize
    );
    assert!(core::mem::size_of::<completion>() == LUPOS_KTHREAD_SIZE_completion as usize);
    assert!(core::mem::align_of::<completion>() == LUPOS_KTHREAD_ALIGN_completion as usize);
    assert!(
        core::mem::offset_of!(completion, done) == LUPOS_KTHREAD_OFFSET_completion_done as usize
    );
    assert!(
        core::mem::offset_of!(completion, wait) == LUPOS_KTHREAD_OFFSET_completion_wait as usize
    );
    assert!(core::mem::size_of::<list_head>() == LUPOS_KTHREAD_SIZE_list_head as usize);
    assert!(core::mem::align_of::<list_head>() == LUPOS_KTHREAD_ALIGN_list_head as usize);
    assert!(core::mem::offset_of!(list_head, next) == LUPOS_KTHREAD_OFFSET_list_head_next as usize);
    assert!(core::mem::offset_of!(list_head, prev) == LUPOS_KTHREAD_OFFSET_list_head_prev as usize);
    assert!(core::mem::size_of::<cpumask>() == LUPOS_KTHREAD_SIZE_cpumask as usize);
    assert!(core::mem::align_of::<cpumask>() == LUPOS_KTHREAD_ALIGN_cpumask as usize);
    assert!(core::mem::size_of::<raw_spinlock_t>() == LUPOS_KTHREAD_SIZE_raw_spinlock_t as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == LUPOS_KTHREAD_ALIGN_raw_spinlock_t as usize);
    assert!(core::mem::size_of::<lock_class_key>() == LUPOS_KTHREAD_SIZE_lock_class_key as usize);
    assert!(core::mem::align_of::<lock_class_key>() == LUPOS_KTHREAD_ALIGN_lock_class_key as usize);
    assert!(core::mem::size_of::<sched_param>() == LUPOS_KTHREAD_SIZE_sched_param as usize);
    assert!(core::mem::align_of::<sched_param>() == LUPOS_KTHREAD_ALIGN_sched_param as usize);
    assert!(
        core::mem::offset_of!(sched_param, sched_priority)
            == LUPOS_KTHREAD_OFFSET_sched_param_sched_priority as usize
    );
    assert!(
        core::mem::size_of::<kthread_create_info>()
            == LUPOS_KTHREAD_SIZE_kthread_create_info as usize
    );
    assert!(
        core::mem::align_of::<kthread_create_info>()
            == LUPOS_KTHREAD_ALIGN_kthread_create_info as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, full_name)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_full_name as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, threadfn)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_threadfn as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, data)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_data as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, node)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_node as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, result)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_result as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, done)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_done as usize
    );
    assert!(
        core::mem::offset_of!(kthread_create_info, list)
            == LUPOS_KTHREAD_OFFSET_kthread_create_info_list as usize
    );
    assert!(core::mem::size_of::<kthread>() == LUPOS_KTHREAD_SIZE_kthread as usize);
    assert!(core::mem::align_of::<kthread>() == LUPOS_KTHREAD_ALIGN_kthread as usize);
    assert!(core::mem::offset_of!(kthread, flags) == LUPOS_KTHREAD_OFFSET_kthread_flags as usize);
    assert!(core::mem::offset_of!(kthread, cpu) == LUPOS_KTHREAD_OFFSET_kthread_cpu as usize);
    assert!(core::mem::offset_of!(kthread, node) == LUPOS_KTHREAD_OFFSET_kthread_node as usize);
    assert!(
        core::mem::offset_of!(kthread, started) == LUPOS_KTHREAD_OFFSET_kthread_started as usize
    );
    assert!(core::mem::offset_of!(kthread, result) == LUPOS_KTHREAD_OFFSET_kthread_result as usize);
    assert!(
        core::mem::offset_of!(kthread, threadfn) == LUPOS_KTHREAD_OFFSET_kthread_threadfn as usize
    );
    assert!(core::mem::offset_of!(kthread, data) == LUPOS_KTHREAD_OFFSET_kthread_data as usize);
    assert!(core::mem::offset_of!(kthread, parked) == LUPOS_KTHREAD_OFFSET_kthread_parked as usize);
    assert!(core::mem::offset_of!(kthread, exited) == LUPOS_KTHREAD_OFFSET_kthread_exited as usize);
    assert!(
        core::mem::offset_of!(kthread, full_name)
            == LUPOS_KTHREAD_OFFSET_kthread_full_name as usize
    );
    assert!(core::mem::offset_of!(kthread, task) == LUPOS_KTHREAD_OFFSET_kthread_task as usize);
    assert!(
        core::mem::offset_of!(kthread, affinity_node)
            == LUPOS_KTHREAD_OFFSET_kthread_affinity_node as usize
    );
    assert!(
        core::mem::offset_of!(kthread, preferred_affinity)
            == LUPOS_KTHREAD_OFFSET_kthread_preferred_affinity as usize
    );
    assert!(core::mem::size_of::<kthread_worker>() == LUPOS_KTHREAD_SIZE_kthread_worker as usize);
    assert!(core::mem::align_of::<kthread_worker>() == LUPOS_KTHREAD_ALIGN_kthread_worker as usize);
    assert!(
        core::mem::offset_of!(kthread_worker, flags)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_flags as usize
    );
    assert!(
        core::mem::offset_of!(kthread_worker, lock)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_lock as usize
    );
    assert!(
        core::mem::offset_of!(kthread_worker, work_list)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_work_list as usize
    );
    assert!(
        core::mem::offset_of!(kthread_worker, delayed_work_list)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_delayed_work_list as usize
    );
    assert!(
        core::mem::offset_of!(kthread_worker, task)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_task as usize
    );
    assert!(
        core::mem::offset_of!(kthread_worker, current_work)
            == LUPOS_KTHREAD_OFFSET_kthread_worker_current_work as usize
    );
    assert!(core::mem::size_of::<kthread_work>() == LUPOS_KTHREAD_SIZE_kthread_work as usize);
    assert!(core::mem::align_of::<kthread_work>() == LUPOS_KTHREAD_ALIGN_kthread_work as usize);
    assert!(
        core::mem::offset_of!(kthread_work, node)
            == LUPOS_KTHREAD_OFFSET_kthread_work_node as usize
    );
    assert!(
        core::mem::offset_of!(kthread_work, func)
            == LUPOS_KTHREAD_OFFSET_kthread_work_func as usize
    );
    assert!(
        core::mem::offset_of!(kthread_work, worker)
            == LUPOS_KTHREAD_OFFSET_kthread_work_worker as usize
    );
    assert!(
        core::mem::offset_of!(kthread_work, canceling)
            == LUPOS_KTHREAD_OFFSET_kthread_work_canceling as usize
    );
    assert!(
        core::mem::size_of::<kthread_delayed_work>()
            == LUPOS_KTHREAD_SIZE_kthread_delayed_work as usize
    );
    assert!(
        core::mem::align_of::<kthread_delayed_work>()
            == LUPOS_KTHREAD_ALIGN_kthread_delayed_work as usize
    );
    assert!(
        core::mem::offset_of!(kthread_delayed_work, work)
            == LUPOS_KTHREAD_OFFSET_kthread_delayed_work_work as usize
    );
    assert!(
        core::mem::offset_of!(kthread_delayed_work, timer)
            == LUPOS_KTHREAD_OFFSET_kthread_delayed_work_timer as usize
    );
    assert!(core::mem::size_of::<timer_list>() == LUPOS_KTHREAD_SIZE_timer_list as usize);
    assert!(core::mem::align_of::<timer_list>() == LUPOS_KTHREAD_ALIGN_timer_list as usize);
    assert!(
        core::mem::offset_of!(timer_list, expires)
            == LUPOS_KTHREAD_OFFSET_timer_list_expires as usize
    );
    assert!(
        core::mem::offset_of!(timer_list, function)
            == LUPOS_KTHREAD_OFFSET_timer_list_function as usize
    );
    assert!(
        core::mem::size_of::<lupos_kthread_cpumask>()
            == LUPOS_KTHREAD_SIZE_lupos_kthread_cpumask as usize
    );
    assert!(
        core::mem::align_of::<lupos_kthread_cpumask>()
            == LUPOS_KTHREAD_ALIGN_lupos_kthread_cpumask as usize
    );
    assert!(
        core::mem::offset_of!(lupos_kthread_cpumask, mask)
            == LUPOS_KTHREAD_OFFSET_lupos_kthread_cpumask_mask as usize
    );
    assert!(
        core::mem::size_of::<lupos_kthread_flush_work>()
            == LUPOS_KTHREAD_SIZE_lupos_kthread_flush_work as usize
    );
    assert!(
        core::mem::align_of::<lupos_kthread_flush_work>()
            == LUPOS_KTHREAD_ALIGN_lupos_kthread_flush_work as usize
    );
    assert!(
        core::mem::offset_of!(lupos_kthread_flush_work, work)
            == LUPOS_KTHREAD_OFFSET_lupos_kthread_flush_work_work as usize
    );
    assert!(
        core::mem::offset_of!(lupos_kthread_flush_work, done)
            == LUPOS_KTHREAD_OFFSET_lupos_kthread_flush_work_done as usize
    );
};
#[cfg(CONFIG_NUMA)]
const _: () = assert!(
    core::mem::offset_of!(task_struct, pref_node_fork)
        == LUPOS_KTHREAD_OFFSET_task_struct_pref_node_fork as usize
);
#[cfg(CONFIG_BLK_CGROUP)]
const _: () = assert!(
    core::mem::offset_of!(kthread, blkcg_css) == LUPOS_KTHREAD_OFFSET_kthread_blkcg_css as usize
);
