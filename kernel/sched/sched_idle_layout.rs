// SPDX-License-Identifier: GPL-2.0-only
// Binding drift is fatal; do not substitute guessed fields/padding to pass.
macro_rules! native_type {
    ($ty:ty, $size:ident, $align:ident) => {
        assert!(size_of::<$ty>() == $size as usize);
        assert!(align_of::<$ty>() == $align as usize);
    };
}
macro_rules! native_field {
    ($ty:ty, $($field:ident).+, $offset:ident) => {
        assert!(offset_of!($ty, $($field).+) == $offset as usize);
    };
}
const _: () = {
    native_type!(rq, LUPOS_IDLE_SIZE_rq, LUPOS_IDLE_ALIGN_rq);
    native_type!(
        task_struct,
        LUPOS_IDLE_SIZE_task_struct,
        LUPOS_IDLE_ALIGN_task_struct
    );
    native_type!(
        sched_entity,
        LUPOS_IDLE_SIZE_sched_entity,
        LUPOS_IDLE_ALIGN_sched_entity
    );
    native_type!(
        sched_avg,
        LUPOS_IDLE_SIZE_sched_avg,
        LUPOS_IDLE_ALIGN_sched_avg
    );
    native_type!(cfs_rq, LUPOS_IDLE_SIZE_cfs_rq, LUPOS_IDLE_ALIGN_cfs_rq);
    native_type!(
        cpuidle_device,
        LUPOS_IDLE_SIZE_cpuidle_device,
        LUPOS_IDLE_ALIGN_cpuidle_device
    );
    native_type!(
        cpuidle_driver,
        LUPOS_IDLE_SIZE_cpuidle_driver,
        LUPOS_IDLE_ALIGN_cpuidle_driver
    );
    native_type!(
        idle_timer,
        LUPOS_IDLE_SIZE_idle_timer,
        LUPOS_IDLE_ALIGN_idle_timer
    );
    native_type!(hrtimer, LUPOS_IDLE_SIZE_hrtimer, LUPOS_IDLE_ALIGN_hrtimer);
    native_field!(rq, idle, LUPOS_IDLE_OFF_rq_idle);
    #[cfg(CONFIG_SCHED_PROXY_EXEC)]
    native_field!(rq, curr, LUPOS_IDLE_OFF_rq_curr);
    #[cfg(not(CONFIG_SCHED_PROXY_EXEC))]
    native_field!(rq, __bindgen_anon_1.curr, LUPOS_IDLE_OFF_rq_curr);
    native_field!(rq, cfs, LUPOS_IDLE_OFF_rq_cfs);
    native_field!(rq, avg_rt, LUPOS_IDLE_OFF_rq_avg_rt);
    native_field!(rq, avg_dl, LUPOS_IDLE_OFF_rq_avg_dl);
    native_field!(rq, lost_idle_time, LUPOS_IDLE_OFF_rq_lost_idle_time);
    native_field!(rq, clock_pelt, LUPOS_IDLE_OFF_rq_clock_pelt);
    native_field!(rq, fair_server, LUPOS_IDLE_OFF_rq_fair_server);
    #[cfg(CONFIG_CPU_IDLE)]
    native_field!(rq, idle_state, LUPOS_IDLE_OFF_rq_idle_state);
    #[cfg(CONFIG_SCHEDSTATS)]
    native_field!(rq, sched_goidle, LUPOS_IDLE_OFF_rq_sched_goidle);
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    native_field!(rq, ext_server, LUPOS_IDLE_OFF_rq_ext_server);
    native_field!(task_struct, flags, LUPOS_IDLE_OFF_task_struct_flags);
    native_field!(task_struct, policy, LUPOS_IDLE_OFF_task_struct_policy);
    native_field!(
        task_struct,
        nr_cpus_allowed,
        LUPOS_IDLE_OFF_task_struct_nr_cpus_allowed
    );
    native_field!(task_struct, mm, LUPOS_IDLE_OFF_task_struct_mm);
    native_field!(task_struct, se, LUPOS_IDLE_OFF_task_struct_se);
    native_field!(task_struct, prio, LUPOS_IDLE_OFF_task_struct_prio);
    native_field!(
        sched_entity,
        exec_start,
        LUPOS_IDLE_OFF_sched_entity_exec_start
    );
    native_field!(cfs_rq, avg, LUPOS_IDLE_OFF_cfs_rq_avg);
    native_field!(sched_avg, util_sum, LUPOS_IDLE_OFF_sched_avg_util_sum);
    native_field!(
        cpuidle_device,
        last_residency_ns,
        LUPOS_IDLE_OFF_cpuidle_device_last_residency_ns
    );
    native_field!(
        cpuidle_device,
        forced_idle_latency_limit_ns,
        LUPOS_IDLE_OFF_cpuidle_device_forced_idle_latency_limit_ns
    );
    native_field!(
        cpuidle_driver,
        state_count,
        LUPOS_IDLE_OFF_cpuidle_driver_state_count
    );
    native_field!(idle_timer, timer, LUPOS_IDLE_OFF_idle_timer_timer);
    native_field!(idle_timer, done, LUPOS_IDLE_OFF_idle_timer_done);
};
