/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_SOFTIRQ_LAYOUT_H
#define RUST_SOFTIRQ_LAYOUT_H
enum { RUST_SIRQ_SIZE_softirq_action = sizeof(struct softirq_action), RUST_SIRQ_ALIGN_softirq_action = __alignof__(struct softirq_action) };
enum { RUST_SIRQ_OFFSET_softirq_action_action = offsetof(struct softirq_action, action) };
enum { RUST_SIRQ_SIZE_tasklet_head = sizeof(struct tasklet_head), RUST_SIRQ_ALIGN_tasklet_head = __alignof__(struct tasklet_head) };
enum { RUST_SIRQ_OFFSET_tasklet_head_head = offsetof(struct tasklet_head, head) };
enum { RUST_SIRQ_OFFSET_tasklet_head_tail = offsetof(struct tasklet_head, tail) };
enum { RUST_SIRQ_SIZE_tasklet_struct = sizeof(struct tasklet_struct), RUST_SIRQ_ALIGN_tasklet_struct = __alignof__(struct tasklet_struct) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_next = offsetof(struct tasklet_struct, next) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_state = offsetof(struct tasklet_struct, state) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_count = offsetof(struct tasklet_struct, count) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_use_callback = offsetof(struct tasklet_struct, use_callback) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_func = offsetof(struct tasklet_struct, func) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_callback = offsetof(struct tasklet_struct, callback) };
enum { RUST_SIRQ_OFFSET_tasklet_struct_data = offsetof(struct tasklet_struct, data) };
enum { RUST_SIRQ_SIZE_task_struct = sizeof(struct task_struct), RUST_SIRQ_ALIGN_task_struct = __alignof__(struct task_struct) };
enum { RUST_SIRQ_OFFSET_task_struct_flags = offsetof(struct task_struct, flags) };
#ifdef CONFIG_PREEMPT_RT
enum { RUST_SIRQ_SIZE_task_struct_rt = sizeof(struct task_struct), RUST_SIRQ_ALIGN_task_struct_rt = __alignof__(struct task_struct) };
enum { RUST_SIRQ_OFFSET_task_struct_rt_softirq_disable_cnt = offsetof(struct task_struct, softirq_disable_cnt) };
#endif
#ifdef CONFIG_DEBUG_PREEMPT
enum { RUST_SIRQ_SIZE_task_struct_debug = sizeof(struct task_struct), RUST_SIRQ_ALIGN_task_struct_debug = __alignof__(struct task_struct) };
enum { RUST_SIRQ_OFFSET_task_struct_debug_preempt_disable_ip = offsetof(struct task_struct, preempt_disable_ip) };
#endif
enum { RUST_SIRQ_SIZE_smp_hotplug_thread = sizeof(struct smp_hotplug_thread), RUST_SIRQ_ALIGN_smp_hotplug_thread = __alignof__(struct smp_hotplug_thread) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_store = offsetof(struct smp_hotplug_thread, store) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_list = offsetof(struct smp_hotplug_thread, list) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_should_run = offsetof(struct smp_hotplug_thread, thread_should_run) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_fn = offsetof(struct smp_hotplug_thread, thread_fn) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_setup = offsetof(struct smp_hotplug_thread, setup) };
enum { RUST_SIRQ_OFFSET_smp_hotplug_thread_thread_comm = offsetof(struct smp_hotplug_thread, thread_comm) };
enum { RUST_SIRQ_SIZE_atomic_t = sizeof(atomic_t), RUST_SIRQ_ALIGN_atomic_t = __alignof__(atomic_t) };
enum { RUST_SIRQ_OFFSET_atomic_t_counter = offsetof(atomic_t, counter) };
#ifdef CONFIG_PREEMPT_RT
enum { RUST_SIRQ_SIZE_softirq_ctrl = sizeof(struct softirq_ctrl), RUST_SIRQ_ALIGN_softirq_ctrl = __alignof__(struct softirq_ctrl) };
enum { RUST_SIRQ_OFFSET_softirq_ctrl_lock = offsetof(struct softirq_ctrl, lock) };
enum { RUST_SIRQ_OFFSET_softirq_ctrl_cnt = offsetof(struct softirq_ctrl, cnt) };
#endif
#ifdef CONFIG_PREEMPT_RT
enum { RUST_SIRQ_SIZE_tasklet_sync_callback = sizeof(struct tasklet_sync_callback), RUST_SIRQ_ALIGN_tasklet_sync_callback = __alignof__(struct tasklet_sync_callback) };
enum { RUST_SIRQ_OFFSET_tasklet_sync_callback_cb_lock = offsetof(struct tasklet_sync_callback, cb_lock) };
enum { RUST_SIRQ_OFFSET_tasklet_sync_callback_cb_waiters = offsetof(struct tasklet_sync_callback, cb_waiters) };
#endif
#endif
