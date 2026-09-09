// SPDX-License-Identifier: GPL-2.0
/* Protective Load Balancing (PLB)
 *
 * PLB was designed to reduce link load imbalance across datacenter
 * switches. PLB is a host-based optimization; it leverages congestion
 * signals from the transport layer to randomly change the path of the
 * connection experiencing sustained congestion. PLB prefers to repath
 * after idle periods to minimize packet reordering. It repaths by
 * changing the IPv6 Flow Label on the packets of a connection, which
 * datacenter switches include as part of ECMP/WCMP hashing.
 *
 * PLB is described in detail in:
 *
 *	Mubashir Adnan Qureshi, Yuchung Cheng, Qianwen Yin, Qiaobin Fu,
 *	Gautam Kumar, Masoud Moshref, Junhua Yan, Van Jacobson,
 *	David Wetherall,Abdul Kabbani:
 *	"PLB: Congestion Signals are Simple and Effective for
 *	 Network Load Balancing"
 *	In ACM SIGCOMM 2022, Amsterdam Netherlands.
 *
 */

// Dependency declarations and macros are supplied by the surrounding kernel bindings.

/* Called once per round-trip to update PLB state for a connection. */
pub unsafe fn tcp_plb_update_state(
    sk: *const sock,
    plb: *mut tcp_plb_state,
    cong_ratio: i32,
) {
    let net: *mut net = sock_net(sk);

    if !READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_enabled) {
        return;
    }

    if cong_ratio >= 0 {
        if cong_ratio < READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_cong_thresh) {
            (*plb).consec_cong_rounds = 0;
        } else if (*plb).consec_cong_rounds
            < READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_rehash_rounds)
        {
            (*plb).consec_cong_rounds += 1;
        }
    }
}

/* Check whether recent congestion has been persistent enough to warrant
 * a load balancing decision that switches the connection to another path.
 */
pub unsafe fn tcp_plb_check_rehash(sk: *mut sock, plb: *mut tcp_plb_state) {
    let net: *mut net = sock_net(sk);
    let mut max_suspend: u32;
    let mut forced_rehash = false;
    let mut idle_rehash = false;

    if !READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_enabled) {
        return;
    }

    forced_rehash = (*plb).consec_cong_rounds
        >= READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_rehash_rounds);
    /* If sender goes idle then we check whether to rehash. */
    idle_rehash = READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_idle_rehash_rounds) != 0
        && (*tcp_sk(sk)).packets_out == 0
        && (*plb).consec_cong_rounds
            >= READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_idle_rehash_rounds);

    if !forced_rehash && !idle_rehash {
        return;
    }

    /* Note that tcp_jiffies32 can wrap; we detect wraps by checking for
     * cases where the max suspension end is before the actual suspension
     * end. We clear pause_until to 0 to indicate there is no recent
     * RTO event that constrains PLB rehashing.
     */
    max_suspend = 2u32
        .wrapping_mul(READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_suspend_rto_sec))
        .wrapping_mul(HZ);
    if (*plb).pause_until != 0
        && (!before(tcp_jiffies32, (*plb).pause_until)
            || before(tcp_jiffies32.wrapping_add(max_suspend), (*plb).pause_until))
    {
        (*plb).pause_until = 0;
    }

    if (*plb).pause_until != 0 {
        return;
    }

    __sk_rethink_txhash_reset_dst(sk);
    (*plb).consec_cong_rounds = 0;
    (*tcp_sk(sk)).plb_rehash = (*tcp_sk(sk)).plb_rehash.wrapping_add(1);
    NET_INC_STATS(sock_net(sk), LINUX_MIB_TCPPLBREHASH);
}

/* Upon RTO, disallow load balancing for a while, to avoid having load
 * balancing decisions switch traffic to a black-holed path that was
 * previously avoided with a sk_rethink_txhash() call at RTO time.
 */
pub unsafe fn tcp_plb_update_state_upon_rto(
    sk: *mut sock,
    plb: *mut tcp_plb_state,
) {
    let net: *mut net = sock_net(sk);
    let mut pause: u32;

    if !READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_enabled) {
        return;
    }

    pause = READ_ONCE((*(*net).ipv4).sysctl_tcp_plb_suspend_rto_sec).wrapping_mul(HZ);
    pause = pause.wrapping_add(get_random_u32_below(pause));
    (*plb).pause_until = tcp_jiffies32.wrapping_add(pause);

    /* Reset PLB state upon RTO, since an RTO causes a sk_rethink_txhash() call
     * that may switch this connection to a path with completely different
     * congestion characteristics.
     */
    (*plb).consec_cong_rounds = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
