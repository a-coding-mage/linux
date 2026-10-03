// SPDX-License-Identifier: GPL-2.0-only
// Native configured type/field checks; generated from make-layout.pl.
const _: () = {
    assert!(size_of::<softirq_action>() == RUST_SIRQ_SIZE_softirq_action as usize);
    assert!(core::mem::align_of::<softirq_action>() == RUST_SIRQ_ALIGN_softirq_action as usize);
    assert!(offset_of!(softirq_action, action) == RUST_SIRQ_OFFSET_softirq_action_action as usize);
};
const _: () = {
    assert!(size_of::<tasklet_head>() == RUST_SIRQ_SIZE_tasklet_head as usize);
    assert!(core::mem::align_of::<tasklet_head>() == RUST_SIRQ_ALIGN_tasklet_head as usize);
    assert!(offset_of!(tasklet_head, head) == RUST_SIRQ_OFFSET_tasklet_head_head as usize);
    assert!(offset_of!(tasklet_head, tail) == RUST_SIRQ_OFFSET_tasklet_head_tail as usize);
};
const _: () = {
    assert!(size_of::<tasklet_struct>() == RUST_SIRQ_SIZE_tasklet_struct as usize);
    assert!(core::mem::align_of::<tasklet_struct>() == RUST_SIRQ_ALIGN_tasklet_struct as usize);
    assert!(offset_of!(tasklet_struct, next) == RUST_SIRQ_OFFSET_tasklet_struct_next as usize);
    assert!(offset_of!(tasklet_struct, state) == RUST_SIRQ_OFFSET_tasklet_struct_state as usize);
    assert!(offset_of!(tasklet_struct, count) == RUST_SIRQ_OFFSET_tasklet_struct_count as usize);
    assert!(
        offset_of!(tasklet_struct, use_callback)
            == RUST_SIRQ_OFFSET_tasklet_struct_use_callback as usize
    );
    assert!(
        offset_of!(tasklet_struct, __bindgen_anon_1.func)
            == RUST_SIRQ_OFFSET_tasklet_struct_func as usize
    );
    assert!(
        offset_of!(tasklet_struct, __bindgen_anon_1.callback)
            == RUST_SIRQ_OFFSET_tasklet_struct_callback as usize
    );
    assert!(offset_of!(tasklet_struct, data) == RUST_SIRQ_OFFSET_tasklet_struct_data as usize);
};
const _: () = {
    assert!(size_of::<task_struct>() == RUST_SIRQ_SIZE_task_struct as usize);
    assert!(core::mem::align_of::<task_struct>() == RUST_SIRQ_ALIGN_task_struct as usize);
    assert!(offset_of!(task_struct, flags) == RUST_SIRQ_OFFSET_task_struct_flags as usize);
};
#[cfg(CONFIG_PREEMPT_RT)]
const _: () = {
    assert!(size_of::<task_struct>() == RUST_SIRQ_SIZE_task_struct_rt as usize);
    assert!(core::mem::align_of::<task_struct>() == RUST_SIRQ_ALIGN_task_struct_rt as usize);
    assert!(
        offset_of!(task_struct, softirq_disable_cnt)
            == RUST_SIRQ_OFFSET_task_struct_rt_softirq_disable_cnt as usize
    );
};
#[cfg(CONFIG_DEBUG_PREEMPT)]
const _: () = {
    assert!(size_of::<task_struct>() == RUST_SIRQ_SIZE_task_struct_debug as usize);
    assert!(core::mem::align_of::<task_struct>() == RUST_SIRQ_ALIGN_task_struct_debug as usize);
    assert!(
        offset_of!(task_struct, preempt_disable_ip)
            == RUST_SIRQ_OFFSET_task_struct_debug_preempt_disable_ip as usize
    );
};
const _: () = {
    assert!(size_of::<smp_hotplug_thread>() == RUST_SIRQ_SIZE_smp_hotplug_thread as usize);
    assert!(
        core::mem::align_of::<smp_hotplug_thread>() == RUST_SIRQ_ALIGN_smp_hotplug_thread as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, store) == RUST_SIRQ_OFFSET_smp_hotplug_thread_store as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, list) == RUST_SIRQ_OFFSET_smp_hotplug_thread_list as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, thread_should_run)
            == RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_should_run as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, thread_fn)
            == RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_fn as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, setup) == RUST_SIRQ_OFFSET_smp_hotplug_thread_setup as usize
    );
    assert!(
        offset_of!(smp_hotplug_thread, thread_comm)
            == RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_comm as usize
    );
};
const _: () = {
    assert!(size_of::<atomic_t>() == RUST_SIRQ_SIZE_atomic_t as usize);
    assert!(core::mem::align_of::<atomic_t>() == RUST_SIRQ_ALIGN_atomic_t as usize);
    assert!(offset_of!(atomic_t, counter) == RUST_SIRQ_OFFSET_atomic_t_counter as usize);
};
#[cfg(CONFIG_PREEMPT_RT)]
const _: () = {
    assert!(size_of::<softirq_ctrl>() == RUST_SIRQ_SIZE_softirq_ctrl as usize);
    assert!(core::mem::align_of::<softirq_ctrl>() == RUST_SIRQ_ALIGN_softirq_ctrl as usize);
    assert!(offset_of!(softirq_ctrl, lock) == RUST_SIRQ_OFFSET_softirq_ctrl_lock as usize);
    assert!(offset_of!(softirq_ctrl, cnt) == RUST_SIRQ_OFFSET_softirq_ctrl_cnt as usize);
};
#[cfg(CONFIG_PREEMPT_RT)]
const _: () = {
    assert!(size_of::<tasklet_sync_callback>() == RUST_SIRQ_SIZE_tasklet_sync_callback as usize);
    assert!(
        core::mem::align_of::<tasklet_sync_callback>()
            == RUST_SIRQ_ALIGN_tasklet_sync_callback as usize
    );
    assert!(
        offset_of!(tasklet_sync_callback, cb_lock)
            == RUST_SIRQ_OFFSET_tasklet_sync_callback_cb_lock as usize
    );
    assert!(
        offset_of!(tasklet_sync_callback, cb_waiters)
            == RUST_SIRQ_OFFSET_tasklet_sync_callback_cb_waiters as usize
    );
};
