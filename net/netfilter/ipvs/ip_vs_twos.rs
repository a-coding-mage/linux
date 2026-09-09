// SPDX-License-Identifier: GPL-2.0-or-later
/* IPVS:        Power of Twos Choice Scheduling module
 *
 * Authors:     Darby Payne <darby.payne@applovin.com>
 */

// #define pr_fmt(fmt) "IPVS: " fmt
// C dependencies: linux/kernel.h, linux/module.h, linux/random.h, net/ip_vs.h

/*    Power of Twos Choice scheduling, algorithm originally described by
 *    Michael Mitzenmacher.
 *
 *    Randomly picks two destinations and picks the one with the least
 *    amount of connections
 *
 *    The algorithm calculates a few variables
 *    - total_weight = sum of all weights
 *    - rweight1 = random number between [0,total_weight]
 *    - rweight2 = random number between [0,total_weight]
 *
 *    For each destination
 *      decrement rweight1 and rweight2 by the destination weight
 *      pick choice1 when rweight1 is <= 0
 *      pick choice2 when rweight2 is <= 0
 *
 *    Return choice2 if choice2 has less connections than choice 1 normalized
 *    by weight
 *
 *    References
 *    ----------
 *
 *    [Mitzenmacher 2016]
 *       The Power of Two Random Choices: A Survey of Techniques and Results
 *       Michael Mitzenmacher, Andrea W. Richa y, Ramesh Sitaraman
 *       http://www.eecs.harvard.edu/~michaelm/NEWWORK/postscripts/twosurvey.pdf
 */

unsafe fn ip_vs_twos_schedule(
    svc: *mut ip_vs_service,
    skb: *const sk_buff,
    iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mut dest: *mut ip_vs_dest;
    let mut choice1: *mut ip_vs_dest = core::ptr::null_mut();
    let mut choice2: *mut ip_vs_dest = core::ptr::null_mut();
    let mut rweight1: i32;
    let mut rweight2: i32;
    let mut weight1: i32 = -1;
    let mut weight2: i32 = -1;
    let mut overhead1: i32 = 0;
    let mut overhead2: i32 = 0;
    let mut total_weight: i32 = 0;
    let mut weight: i32;

    // IP_VS_DBG(6, "%s(): Scheduling...\n", __func__);
    let _ = (skb, iph);

    // list_for_each_entry_rcu(dest, &svc->destinations, n_list)
    for dest in ip_vs_destinations_rcu(svc) {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) == 0 {
            weight = atomic_read(&(*dest).weight);
            if weight > 0 {
                total_weight += weight;
                choice1 = dest;
            }
        }
    }

    if choice1.is_null() {
        ip_vs_scheduler_err(svc, "no destination available");
        return core::ptr::null_mut();
    }

    total_weight += 1;
    rweight1 = get_random_u32_below(total_weight as u32) as i32;
    rweight2 = get_random_u32_below(total_weight as u32) as i32;

    // list_for_each_entry_rcu(dest, &svc->destinations, n_list)
    for dest in ip_vs_destinations_rcu(svc) {
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0 {
            continue;
        }

        weight = atomic_read(&(*dest).weight);
        if weight <= 0 {
            continue;
        }

        rweight1 -= weight;
        rweight2 -= weight;

        if rweight1 <= 0 && weight1 == -1 {
            choice1 = dest;
            weight1 = weight;
            overhead1 = ip_vs_dest_conn_overhead(dest);
        }

        if rweight2 <= 0 && weight2 == -1 {
            choice2 = dest;
            weight2 = weight;
            overhead2 = ip_vs_dest_conn_overhead(dest);
        }

        if weight1 != -1 && weight2 != -1 {
            break;
        }
    }

    if !choice2.is_null() && (weight2 * overhead1) > (weight1 * overhead2) {
        choice1 = choice2;
    }

    // IP_VS_DBG_BUF(6, "twos: server %s:%u conns %d refcnt %d weight %d\n", ...);
    return choice1;
}

static mut ip_vs_twos_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "twos" as *const str,
    refcnt: ATOMIC_INIT(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT,
    schedule: Some(ip_vs_twos_schedule),
};

unsafe fn ip_vs_twos_init() -> i32 {
    register_ip_vs_scheduler(&raw mut ip_vs_twos_scheduler)
}

unsafe fn ip_vs_twos_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_twos_scheduler);
    synchronize_rcu();
}

// module_init(ip_vs_twos_init);
// module_exit(ip_vs_twos_cleanup);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ipvs power of twos choice scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
