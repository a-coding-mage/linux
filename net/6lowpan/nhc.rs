// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN next header compression
 *
 *	Authors:
 *	Alexander Aring		<aar@pengutronix.de>
 */

// Dependencies supplied by the corresponding kernel/networking headers.
use crate::{ipv6hdr, lowpan_nhc, net_device, sk_buff};

static mut LOWPAN_NEXTHDR_NHCS: [*const lowpan_nhc; (NEXTHDR_MAX + 1) as usize] =
    [core::ptr::null(); (NEXTHDR_MAX + 1) as usize];
static mut LOWPAN_NHC_LOCK: spinlock_t = spinlock_t::new();

unsafe fn lowpan_nhc_by_nhcid(skb: *mut sk_buff) -> *const lowpan_nhc {
    let mut nhc: *const lowpan_nhc;
    let id: u8;

    if !pskb_may_pull(skb, 1) {
        return core::ptr::null();
    }

    id = *(*skb).data;

    for i in 0..(NEXTHDR_MAX + 1) {
        nhc = LOWPAN_NEXTHDR_NHCS[i as usize];
        if nhc.is_null() {
            continue;
        }

        if (id & (*nhc).idmask) == (*nhc).id {
            return nhc;
        }
    }

    core::ptr::null()
}

pub unsafe fn lowpan_nhc_check_compression(
    skb: *mut sk_buff,
    hdr: *const ipv6hdr,
    hc_ptr: *mut *mut u8,
) -> i32 {
    let nhc: *const lowpan_nhc;
    let mut ret: i32 = 0;

    spin_lock_bh(&mut LOWPAN_NHC_LOCK);

    nhc = LOWPAN_NEXTHDR_NHCS[(*hdr).nexthdr as usize];
    if nhc.is_null() || (*nhc).compress.is_none() {
        ret = -ENOENT;
    }

    spin_unlock_bh(&mut LOWPAN_NHC_LOCK);

    ret
}

pub unsafe fn lowpan_nhc_do_compression(
    skb: *mut sk_buff,
    hdr: *const ipv6hdr,
    hc_ptr: *mut *mut u8,
) -> i32 {
    let ret: i32;
    let nhc: *const lowpan_nhc;

    spin_lock_bh(&mut LOWPAN_NHC_LOCK);

    nhc = LOWPAN_NEXTHDR_NHCS[(*hdr).nexthdr as usize];
    /* check if the nhc module was removed in unlocked part. */
    if nhc.is_null() || (*nhc).compress.is_none() {
        ret = -EINVAL;
        spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
        return ret;
    }

    /* In the case of RAW sockets the transport header is not set by
     * the ip6 stack so we must set it ourselves
     */
    if (*skb).transport_header == (*skb).network_header {
        skb_set_transport_header(skb, core::mem::size_of::<ipv6hdr>());
    }

    ret = ((*nhc).compress.unwrap())(skb, hc_ptr);
    if ret >= 0 {
        /* skip the transport header */
        skb_pull(skb, (*nhc).nexthdrlen);
    }

    spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
    ret
}

pub unsafe fn lowpan_nhc_do_uncompression(
    skb: *mut sk_buff,
    dev: *const net_device,
    hdr: *mut ipv6hdr,
) -> i32 {
    let nhc: *const lowpan_nhc;
    let ret: i32;

    spin_lock_bh(&mut LOWPAN_NHC_LOCK);

    nhc = lowpan_nhc_by_nhcid(skb);
    if !nhc.is_null() {
        if let Some(uncompress) = (*nhc).uncompress {
            ret = uncompress(skb, core::mem::size_of::<ipv6hdr>() + (*nhc).nexthdrlen);
            if ret < 0 {
                spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
                return ret;
            }
        } else {
            netdev_warn(dev, "received nhc id for %s which is not implemented.\n", (*nhc).name);
            spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
            return -ENOTSUPP;
        }
    } else {
        spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
        netdev_warn(dev, "received unknown nhc id which was not found.\n");
        return -ENOENT;
    }

    (*hdr).nexthdr = (*nhc).nexthdr;
    skb_reset_transport_header(skb);
    raw_dump_table(
        "lowpan_nhc_do_uncompression",
        "raw transport header dump",
        skb_transport_header(skb),
        (*nhc).nexthdrlen,
    );

    spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
    0
}

pub unsafe fn lowpan_nhc_add(nhc: *const lowpan_nhc) -> i32 {
    let mut ret: i32 = 0;

    spin_lock_bh(&mut LOWPAN_NHC_LOCK);

    if !LOWPAN_NEXTHDR_NHCS[(*nhc).nexthdr as usize].is_null() {
        ret = -EEXIST;
    } else {
        LOWPAN_NEXTHDR_NHCS[(*nhc).nexthdr as usize] = nhc;
    }

    spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
    ret
}

pub unsafe fn lowpan_nhc_del(nhc: *const lowpan_nhc) {
    spin_lock_bh(&mut LOWPAN_NHC_LOCK);
    LOWPAN_NEXTHDR_NHCS[(*nhc).nexthdr as usize] = core::ptr::null();
    spin_unlock_bh(&mut LOWPAN_NHC_LOCK);
    synchronize_net();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
