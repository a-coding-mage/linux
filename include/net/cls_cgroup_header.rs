/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * cls_cgroup.h			Control Group Classifier
 *
 * Authors:	Thomas Graf <tgraf@suug.ch>
 */

/* The declarations below depend on the corresponding kernel types and
 * functions supplied by other translated files. */

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
#[repr(C)]
pub struct cgroup_cls_state {
    pub css: cgroup_subsys_state,
    pub classid: u32,
}

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
extern "C" {
    pub fn task_cls_state(p: *mut task_struct) -> *mut cgroup_cls_state;
}

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
#[inline]
pub unsafe fn task_cls_classid(p: *mut task_struct) -> u32 {
    let classid: u32;

    if in_interrupt() {
        return 0;
    }

    rcu_read_lock();
    classid = (*container_of(
        task_css(p, net_cls_cgrp_id),
        cgroup_cls_state,
        css,
    ))
    .classid;
    rcu_read_unlock();

    classid
}

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
#[inline]
pub unsafe fn sock_update_classid(skcd: *mut sock_cgroup_data) {
    let classid: u32;

    classid = task_cls_classid(current);
    sock_cgroup_set_classid(skcd, classid);
}

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
#[inline]
pub unsafe fn __task_get_classid(task: *mut task_struct) -> u32 {
    (*task_cls_state(task)).classid
}

#[cfg(CONFIG_CGROUP_NET_CLASSID)]
#[inline]
pub unsafe fn task_get_classid(skb: *const sk_buff) -> u32 {
    let mut classid: u32 = __task_get_classid(current);

    /* Due to the nature of the classifier it is required to ignore all
     * packets originating from softirq context as accessing `current'
     * would lead to false results.
     *
     * This test assumes that all callers of dev_queue_xmit() explicitly
     * disable bh. Knowing this, it is possible to detect softirq based
     * calls by looking at the number of nested bh disable calls because
     * softirqs always disables bh.
     */
    if softirq_count() != 0 {
        let sk: *mut sock = skb_to_full_sk(skb);

        /* If there is an sock_cgroup_classid we'll use that. */
        if sk.is_null() || !sk_fullsock(sk) {
            return 0;
        }

        classid = sock_cgroup_classid(&mut (*sk).sk_cgrp_data);
    }

    classid
}

/* !CONFIG_CGROUP_NET_CLASSID */
#[cfg(not(CONFIG_CGROUP_NET_CLASSID))]
#[inline]
pub unsafe fn sock_update_classid(_skcd: *mut sock_cgroup_data) {}

#[cfg(not(CONFIG_CGROUP_NET_CLASSID))]
#[inline]
pub unsafe fn task_get_classid(_skb: *const sk_buff) -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
