/* SPDX-License-Identifier: GPL-2.0 */

/*
 * SCHED_DEADLINE tasks has negative priorities, reflecting
 * the fact that any of them has higher prio than RT and
 * NORMAL/BATCH tasks.
 *
 * The declarations below depend on names supplied by linux/sched.h.
 */

#[inline]
pub unsafe fn dl_prio(prio: i32) -> bool {
    unlikely(prio < MAX_DL_PRIO)
}

/*
 * Returns true if a task has a priority that belongs to DL class. PI-boosted
 * tasks will return true. Use dl_policy() to ignore PI-boosted tasks.
 */
#[inline]
pub unsafe fn dl_task(p: *mut task_struct) -> bool {
    dl_prio((*p).prio)
}

#[inline]
pub fn dl_time_before(a: u64, b: u64) -> bool {
    (a.wrapping_sub(b) as i64) < 0
}

pub struct root_domain;

extern "C" {
    pub fn dl_add_task_root_domain(p: *mut task_struct);
    pub fn dl_clear_root_domain(rd: *mut root_domain);
    pub fn dl_clear_root_domain_cpu(cpu: i32);

    /*
     * Return whether moving DL task @p to @new_mask requires moving DL
     * bandwidth accounting between root domains. This helper is specific to
     * DL bandwidth move accounting semantics and is shared by
     * cpuset_can_attach() and set_cpus_allowed_dl() so both paths use the
     * same source root-domain test.
     */
    pub fn dl_task_needs_bw_move(
        p: *mut task_struct,
        new_mask: *const cpumask,
    ) -> bool;

    pub static mut dl_cookie: u64;
    pub fn dl_bw_visited(cpu: i32, cookie: u64) -> bool;
}

#[inline]
pub unsafe fn dl_server(dl_se: *mut sched_dl_entity) -> bool {
    (*dl_se).dl_server
}

#[inline]
pub unsafe fn dl_task_of(dl_se: *mut sched_dl_entity) -> *mut task_struct {
    BUG_ON(dl_server(dl_se));
    container_of(dl_se, task_struct, dl)
}

/*
 * Regarding the deadline, a task with implicit deadline has a relative
 * deadline == relative period. A task with constrained deadline has a
 * relative deadline <= relative period.
 *
 * We support constrained deadline tasks. However, there are some restrictions
 * applied only for tasks which do not have an implicit deadline. See
 * update_dl_entity() to know more about such restrictions.
 *
 * The dl_is_implicit() returns true if the task has an implicit deadline.
 */
#[inline]
pub unsafe fn dl_is_implicit(dl_se: *mut sched_dl_entity) -> bool {
    (*dl_se).dl_deadline == (*dl_se).dl_period
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
