/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * netprio_cgroup.h                  Control Group Priority set
 *
 * Authors:        Neil Horman <nhorman@tuxdriver.com>
 */

/* Dependencies supplied by the corresponding kernel headers are referenced here. */

#[cfg(CONFIG_CGROUP_NET_PRIO)]
#[repr(C)]
pub struct netprio_map {
    pub rcu: rcu_head,
    pub priomap_len: u32,
    pub priomap: [u32; 0],
}

#[cfg(CONFIG_CGROUP_NET_PRIO)]
#[inline]
pub unsafe fn task_netprioidx(p: *mut task_struct) -> u32 {
    let css: *mut cgroup_subsys_state;
    let idx: u32;

    rcu_read_lock();
    css = task_css(p, net_prio_cgrp_id);
    idx = (*css).id;
    rcu_read_unlock();
    idx
}

#[cfg(CONFIG_CGROUP_NET_PRIO)]
#[inline]
pub unsafe fn sock_update_netprioidx(skcd: *mut sock_cgroup_data) {
    if in_interrupt() {
        return;
    }

    sock_cgroup_set_prioidx(skcd, task_netprioidx(current));
}

#[cfg(not(CONFIG_CGROUP_NET_PRIO))]
#[inline]
pub unsafe fn task_netprioidx(_p: *mut task_struct) -> u32 {
    0
}

#[cfg(not(CONFIG_CGROUP_NET_PRIO))]
#[inline]
pub unsafe fn sock_update_netprioidx(_skcd: *mut sock_cgroup_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
