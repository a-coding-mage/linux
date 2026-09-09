// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Round-Robin Scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *              Peter Kese <peter.kese@ijs.si>
 *
 * Fixes/Changes:
 *     Wensong Zhang            :     changed the ip_vs_rr_schedule to return dest
 *     Julian Anastasov         :     fixed the NULL pointer access bug in debugging
 *     Wensong Zhang            :     changed some comestics things for debugging
 *     Wensong Zhang            :     changed for the d-linked destination list
 *     Wensong Zhang            :     added the ip_vs_rr_update_svc
 *     Wensong Zhang            :     added any dest with weight=0 is quiesced
 */

// C dependencies supplied by the surrounding IPVS/kernel environment.

unsafe fn ip_vs_rr_init_svc(svc: *mut ip_vs_service) -> i32 {
    (*svc).sched_data = &mut (*svc).destinations as *mut list_head as *mut core::ffi::c_void;
    0
}

unsafe fn ip_vs_rr_del_dest(
    svc: *mut ip_vs_service,
    dest: *mut ip_vs_dest,
) -> i32 {
    let p: *mut list_head;

    spin_lock_bh(&mut (*svc).sched_lock);
    p = (*svc).sched_data as *mut list_head;
    /* dest is already unlinked, so p->prev is not valid but
     * p->next is valid, use it to reach previous entry.
     */
    if p == &mut (*dest).n_list {
        (*svc).sched_data = (*(*p).next).prev as *mut core::ffi::c_void;
    }
    spin_unlock_bh(&mut (*svc).sched_lock);
    0
}

/*
 * Round-Robin Scheduling
 */
unsafe fn ip_vs_rr_schedule(
    svc: *mut ip_vs_service,
    _skb: *const sk_buff,
    _iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let p: *mut list_head;
    let mut dest: *mut ip_vs_dest;
    let mut last: *mut ip_vs_dest;
    let mut pass: i32 = 0;

    IP_VS_DBG!(6, "{}(): Scheduling...\n", "ip_vs_rr_schedule");

    spin_lock_bh(&mut (*svc).sched_lock);
    p = (*svc).sched_data as *mut list_head;
    last = list_entry!(p, ip_vs_dest, n_list);
    dest = last;

    loop {
        list_for_each_entry_continue_rcu!(dest, &mut (*svc).destinations, n_list, {
            if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0
                && atomic_read(&(*dest).weight) > 0
            {
                break 'out;
            }
            if dest == last {
                break 'stop;
            }
        });
        pass += 1;
        /* Previous dest could be unlinked, do not loop forever.
         * If we stay at head there is no need for 2nd pass.
         */
        if !(pass < 2 && p != &mut (*svc).destinations) {
            break;
        }
    }

    'stop: {
        spin_unlock_bh(&mut (*svc).sched_lock);
        ip_vs_scheduler_err(svc, "no destination available");
        return core::ptr::null_mut();
    }

    'out: {
        (*svc).sched_data = &mut (*dest).n_list as *mut list_head as *mut core::ffi::c_void;
        spin_unlock_bh(&mut (*svc).sched_lock);
        IP_VS_DBG_BUF!(
            6,
            "RR: server %s:%u activeconns %d refcnt %d weight %d\n",
            IP_VS_DBG_ADDR!((*dest).af, &(*dest).addr),
            ntohs((*dest).port),
            atomic_read(&(*dest).activeconns),
            refcount_read(&(*dest).refcnt),
            atomic_read(&(*dest).weight)
        );
        return dest;
    }
}

static mut ip_vs_rr_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "rr",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_rr_scheduler.n_list),
    init_service: Some(ip_vs_rr_init_svc),
    add_dest: None,
    del_dest: Some(ip_vs_rr_del_dest),
    schedule: Some(ip_vs_rr_schedule),
};

unsafe extern "C" fn ip_vs_rr_init() -> i32 {
    register_ip_vs_scheduler(&mut ip_vs_rr_scheduler)
}

unsafe extern "C" fn ip_vs_rr_cleanup() {
    unregister_ip_vs_scheduler(&mut ip_vs_rr_scheduler);
    synchronize_rcu();
}

module_init!(ip_vs_rr_init);
module_exit!(ip_vs_rr_cleanup);
MODULE_DESCRIPTION!("ipvs round-robin scheduler");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
