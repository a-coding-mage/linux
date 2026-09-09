// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Weighted Round-Robin Scheduling module
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *
 * Changes:
 *     Wensong Zhang            :     changed the ip_vs_wrr_schedule to return dest
 *     Wensong Zhang            :     changed some comestics things for debugging
 *     Wensong Zhang            :     changed for the d-linked destination list
 *     Wensong Zhang            :     added the ip_vs_wrr_update_svc
 *     Julian Anastasov         :     fixed the bug of returning destination
 *                                    with weight 0 when all weights are zero
 */

/* Kernel/IPVS dependencies are supplied by the surrounding translation unit. */

/* The WRR algorithm depends on some caclulations:
 * - mw: maximum weight
 * - di: weight step, greatest common divisor from all weights
 * - cw: current required weight
 * As result, all weights are in the [di..mw] range with a step=di.
 *
 * First, we start with cw = mw and select dests with weight >= cw.
 * Then cw is reduced with di and all dests are checked again.
 * Last pass should be with cw = di. We have mw/di passes in total:
 *
 * pass 1: cw = max weight
 * pass 2: cw = max weight - di
 * pass 3: cw = max weight - 2 * di
 * ...
 * last pass: cw = di
 *
 * Weights are supposed to be >= di but we run in parallel with
 * weight changes, it is possible some dest weight to be reduced
 * below di, bad if it is the only available dest.
 *
 * So, we modify how mw is calculated, now it is reduced with (di - 1),
 * so that last cw is 1 to catch such dests with weight below di:
 * pass 1: cw = max weight - (di - 1)
 * pass 2: cw = max weight - di - (di - 1)
 * pass 3: cw = max weight - 2 * di - (di - 1)
 * ...
 * last pass: cw = 1
 */

#[repr(C)]
pub struct ip_vs_wrr_mark {
    pub cl: *mut ip_vs_dest,
    pub cw: i32,
    pub mw: i32,
    pub di: i32,
    pub rcu_head: rcu_head,
}

pub unsafe fn ip_vs_wrr_gcd_weight(svc: *mut ip_vs_service) -> i32 {
    let mut g: i32 = 0;
    list_for_each_entry!(dest, svc.as_ref().unwrap().destinations, n_list, {
        let weight = atomic_read!((*dest).weight);
        if weight > 0 {
            if g > 0 { g = gcd(weight, g); } else { g = weight; }
        }
    });
    if g != 0 { g } else { 1 }
}

/* Get the maximum weight of the service destinations. */
pub unsafe fn ip_vs_wrr_max_weight(svc: *mut ip_vs_service) -> i32 {
    let mut weight: i32 = 0;
    list_for_each_entry!(dest, svc.as_ref().unwrap().destinations, n_list, {
        let new_weight = atomic_read!((*dest).weight);
        if new_weight > weight { weight = new_weight; }
    });
    weight
}

pub unsafe fn ip_vs_wrr_init_svc(svc: *mut ip_vs_service) -> i32 {
    let mark = kmalloc_obj::<ip_vs_wrr_mark>();
    if mark.is_null() { return -ENOMEM; }
    (*mark).cl = list_entry!(&(*svc).destinations, ip_vs_dest, n_list);
    (*mark).di = ip_vs_wrr_gcd_weight(svc);
    (*mark).mw = ip_vs_wrr_max_weight(svc) - ((*mark).di - 1);
    (*mark).cw = (*mark).mw;
    (*svc).sched_data = mark as *mut _;
    0
}

pub unsafe fn ip_vs_wrr_done_svc(svc: *mut ip_vs_service) {
    let mark = (*svc).sched_data as *mut ip_vs_wrr_mark;
    kfree_rcu!(mark, rcu_head);
}

pub unsafe fn ip_vs_wrr_dest_changed(svc: *mut ip_vs_service, _dest: *mut ip_vs_dest) -> i32 {
    let mark = (*svc).sched_data as *mut ip_vs_wrr_mark;
    spin_lock_bh!((*svc).sched_lock);
    (*mark).cl = list_entry!(&(*svc).destinations, ip_vs_dest, n_list);
    (*mark).di = ip_vs_wrr_gcd_weight(svc);
    (*mark).mw = ip_vs_wrr_max_weight(svc) - ((*mark).di - 1);
    if (*mark).cw > (*mark).mw || (*mark).cw == 0 {
        (*mark).cw = (*mark).mw;
    } else if (*mark).di > 1 {
        (*mark).cw = ((*mark).cw / (*mark).di) * (*mark).di + 1;
    }
    spin_unlock_bh!((*svc).sched_lock);
    0
}

/* Weighted Round-Robin Scheduling */
pub unsafe fn ip_vs_wrr_schedule(
    svc: *mut ip_vs_service, _skb: *const sk_buff, _iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mark = (*svc).sched_data as *mut ip_vs_wrr_mark;
    let mut dest = (*mark).cl;
    let mut stop: *mut ip_vs_dest = core::ptr::null_mut();
    let mut last_pass = false;
    let mut restarted = false;
    spin_lock_bh!((*svc).sched_lock);
    if (*mark).mw == 0 { goto_err_noavail!(svc, dest); }
    let last = dest;
    loop {
        list_for_each_entry_continue_rcu!(dest, (*svc).destinations, n_list, {
            if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0 && atomic_read!((*dest).weight) >= (*mark).cw {
                (*mark).cl = dest;
                spin_unlock_bh!((*svc).sched_lock);
                return dest;
            }
            if dest == stop { goto_err_over!(svc, dest); }
        });
        (*mark).cw -= (*mark).di;
        if (*mark).cw <= 0 {
            (*mark).cw = (*mark).mw;
            if last_pass || list_is_head!((*last).n_list, (*svc).destinations) { goto_err_over!(svc, dest); }
            restarted = true;
        }
        last_pass = (*mark).cw <= (*mark).di;
        if last_pass && restarted && !list_is_head!((*last).n_list, (*svc).destinations) { stop = last; }
    }
}

/* External scheduler registration and module entry points. */
static mut ip_vs_wrr_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "wrr",
    refcnt: ATOMIC_INIT!(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT!(ip_vs_wrr_scheduler.n_list),
    init_service: Some(ip_vs_wrr_init_svc),
    done_service: Some(ip_vs_wrr_done_svc),
    add_dest: Some(ip_vs_wrr_dest_changed),
    del_dest: Some(ip_vs_wrr_dest_changed),
    upd_dest: Some(ip_vs_wrr_dest_changed),
    schedule: Some(ip_vs_wrr_schedule),
};

pub unsafe fn ip_vs_wrr_init() -> i32 { register_ip_vs_scheduler!(&mut ip_vs_wrr_scheduler) }
pub unsafe fn ip_vs_wrr_cleanup() {
    unregister_ip_vs_scheduler!(&mut ip_vs_wrr_scheduler);
    synchronize_rcu!();
}

module_init!(ip_vs_wrr_init);
module_exit!(ip_vs_wrr_cleanup);
MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("ipvs weighted round-robin scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
