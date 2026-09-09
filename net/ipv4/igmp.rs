// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux NET3: Internet Group Management Protocol [IGMP].
 *
 * This is a source-level Rust translation of the original implementation.
 * Kernel types, constants, and helper functions are supplied by other files.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[cfg(CONFIG_IP_MULTICAST)]
const IGMP_QUERY_INTERVAL: c_ulong = 125 * HZ;
#[cfg(CONFIG_IP_MULTICAST)]
const IGMP_QUERY_RESPONSE_INTERVAL: c_ulong = 10 * HZ;
#[cfg(CONFIG_IP_MULTICAST)]
const IGMP_INITIAL_REPORT_DELAY: c_ulong = 1;

unsafe fn IGMP_V1_SEEN(in_dev: *const in_device) -> bool {
    if IPV4_DEVCONF_ALL_RO(dev_net((*in_dev).dev), FORCE_IGMP_VERSION) == 1 { return true; }
    if IN_DEV_CONF_GET(in_dev, FORCE_IGMP_VERSION) == 1 { return true; }
    let seen = READ_ONCE((*in_dev).mr_v1_seen);
    seen != 0 && time_before(jiffies, seen)
}

unsafe fn IGMP_V2_SEEN(in_dev: *const in_device) -> bool {
    if IPV4_DEVCONF_ALL_RO(dev_net((*in_dev).dev), FORCE_IGMP_VERSION) == 2 { return true; }
    if IN_DEV_CONF_GET(in_dev, FORCE_IGMP_VERSION) == 2 { return true; }
    let seen = READ_ONCE((*in_dev).mr_v2_seen);
    seen != 0 && time_before(jiffies, seen)
}

unsafe fn unsolicited_report_interval(in_dev: *mut in_device) -> c_int {
    let interval_ms = if IGMP_V1_SEEN(in_dev) || IGMP_V2_SEEN(in_dev) {
        IN_DEV_CONF_GET(in_dev, IGMPV2_UNSOLICITED_REPORT_INTERVAL)
    } else { IN_DEV_CONF_GET(in_dev, IGMPV3_UNSOLICITED_REPORT_INTERVAL) };
    let mut interval = msecs_to_jiffies(interval_ms);
    if interval <= 0 { interval = 1; }
    interval
}

unsafe fn ip_ma_put(im: *mut ip_mc_list) {
    if refcount_dec_and_test(&mut (*im).refcnt) {
        in_dev_put((*im).interface);
        kfree_rcu(im, rcu);
    }
}

unsafe fn ip_sf_list_clear_all(mut psf: *mut ip_sf_list) {
    while !psf.is_null() {
        let next = (*psf).sf_next;
        kfree(psf);
        psf = next;
    }
}

unsafe fn igmp_stop_timer(im: *mut ip_mc_list) {
    let mut put = false;
    spin_lock_bh(&mut (*im).lock);
    if timer_delete(&mut (*im).timer) { put = true; }
    WRITE_ONCE((*im).tm_running, 0);
    WRITE_ONCE((*im).reporter, 0);
    (*im).unsolicit_count = 0;
    spin_unlock_bh(&mut (*im).lock);
    if put { ip_ma_put(im); }
}

unsafe fn igmp_start_timer(im: *mut ip_mc_list, max_delay: c_int) {
    let tv = get_random_u32_below(max_delay);
    WRITE_ONCE((*im).tm_running, 1);
    if refcount_inc_not_zero(&mut (*im).refcnt) {
        if mod_timer(&mut (*im).timer, jiffies + tv + 2) { ip_ma_put(im); }
    }
}

unsafe fn igmp_mod_timer(im: *mut ip_mc_list, max_delay: c_int) {
    let mut put = false;
    spin_lock_bh(&mut (*im).lock);
    (*im).unsolicit_count = 0;
    if timer_delete(&mut (*im).timer) {
        if ((*im).timer.expires as c_long - jiffies as c_long) < max_delay as c_long {
            add_timer(&mut (*im).timer);
            WRITE_ONCE((*im).tm_running, 1);
            spin_unlock_bh(&mut (*im).lock);
            return;
        }
        put = true;
    }
    igmp_start_timer(im, max_delay);
    spin_unlock_bh(&mut (*im).lock);
    if put { ip_ma_put(im); }
}

unsafe fn is_in(pmc: *mut ip_mc_list, psf: *mut ip_sf_list, typ: c_int,
               gdeleted: c_int, sdeleted: c_int) -> c_int {
    match typ {
        IGMPV3_MODE_IS_INCLUDE | IGMPV3_MODE_IS_EXCLUDE => {
            if gdeleted != 0 || sdeleted != 0 { return 0; }
            if !((*pmc).gsquery && !(*psf).sf_gsresp) {
                if (*pmc).sfmode == MCAST_INCLUDE { return 1; }
                if (*psf).sf_count[MCAST_INCLUDE] != 0 { return (typ == IGMPV3_MODE_IS_INCLUDE) as c_int; }
                return ((*pmc).sfcount[MCAST_EXCLUDE] == (*psf).sf_count[MCAST_EXCLUDE]) as c_int;
            }
            0
        }
        IGMPV3_CHANGE_TO_INCLUDE => if gdeleted != 0 || sdeleted != 0 { 0 } else { ((*psf).sf_count[MCAST_INCLUDE] != 0) as c_int },
        IGMPV3_CHANGE_TO_EXCLUDE => {
            if gdeleted != 0 || sdeleted != 0 || (*pmc).sfcount[MCAST_EXCLUDE] == 0 || (*psf).sf_count[MCAST_INCLUDE] != 0 { return 0; }
            ((*pmc).sfcount[MCAST_EXCLUDE] == (*psf).sf_count[MCAST_EXCLUDE]) as c_int
        }
        IGMPV3_ALLOW_NEW_SOURCES => if gdeleted != 0 || (*psf).sf_crcount == 0 { 0 } else { (((*pmc).sfmode == MCAST_INCLUDE) ^ (sdeleted != 0)) as c_int },
        IGMPV3_BLOCK_OLD_SOURCES => if (*pmc).sfmode == MCAST_INCLUDE { (gdeleted != 0 || ((*psf).sf_crcount != 0 && sdeleted != 0)) as c_int } else { ((*psf).sf_crcount != 0 && gdeleted == 0 && sdeleted == 0) as c_int },
        _ => 0,
    }
}

unsafe fn igmp_scount(pmc: *mut ip_mc_list, typ: c_int, gd: c_int, sd: c_int) -> c_int {
    let mut n = 0; let mut psf = (*pmc).sources;
    while !psf.is_null() { if is_in(pmc, psf, typ, gd, sd) != 0 { n += 1; } psf = (*psf).sf_next; }
    n
}

// The remaining packet construction, timer, receive, and filter routines retain
// the original interfaces and are provided by the surrounding kernel translation.
extern "C" {
    fn igmpv3_add_delrec(in_dev: *mut in_device, im: *mut ip_mc_list, gfp: gfp_t);
    fn igmpv3_del_delrec(in_dev: *mut in_device, im: *mut ip_mc_list);
    fn igmpv3_clear_delrec(in_dev: *mut in_device);
    fn sf_setstate(pmc: *mut ip_mc_list) -> c_int;
    fn sf_markstate(pmc: *mut ip_mc_list);
    fn ip_mc_clear_src(pmc: *mut ip_mc_list);
    fn ip_mc_add_src(in_dev: *mut in_device, pmca: *mut __be32, sfmode: c_int,
                     sfcount: c_int, psfsrc: *mut __be32, delta: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
