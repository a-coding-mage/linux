// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright Gavin Shan, IBM Corporation 2016.
 */

// Linux kernel headers and local headers are supplied by the surrounding crate.

unsafe fn ncsi_validate_aen_pkt(h: *mut ncsi_aen_pkt_hdr, payload: u16) -> i32 {
    let checksum: u32;
    let pchecksum: *mut u32;

    if (*h).common.revision != NCSI_PKT_REVISION { return -EINVAL; }
    if ntohs((*h).common.length) != payload { return -EINVAL; }

    /* Validate checksum, which might be zeroes if the sender doesn't support
     * checksum according to NCSI specification.
     */
    pchecksum = ((h.add(1) as *mut u8).add(payload as usize - 4)) as *mut u32;
    if ntohl(*pchecksum) == 0 { return 0; }

    checksum = ncsi_calculate_checksum(h as *mut u8,
                                       core::mem::size_of::<ncsi_aen_pkt_hdr>()
                                           + payload as usize - 4);
    if *pchecksum != htonl(checksum) { return -EINVAL; }
    0
}

unsafe fn ncsi_aen_handler_lsc(ndp: *mut ncsi_dev_priv,
                               h: *mut ncsi_aen_pkt_hdr) -> i32 {
    let mut nc: *mut ncsi_channel = core::ptr::null_mut();
    let mut ncm: *mut ncsi_channel_mode;
    let old_data: usize;
    let data: usize;
    let lsc: *mut ncsi_aen_lsc_pkt;
    let mut had_link: bool;
    let mut has_link: bool;
    let mut flags: c_ulong = 0;
    let chained: bool;
    let state: i32;

    ncsi_find_package_and_channel(ndp, (*h).common.channel, core::ptr::null_mut(), &mut nc);
    if nc.is_null() { return -ENODEV; }

    lsc = h as *mut ncsi_aen_lsc_pkt;
    spin_lock_irqsave(&mut (*nc).lock, &mut flags);
    ncm = &mut (*nc).modes[NCSI_MODE_LINK as usize];
    old_data = (*ncm).data[2];
    data = ntohl((*lsc).status) as usize;
    (*ncm).data[2] = data;
    (*ncm).data[4] = ntohl((*lsc).oem_status) as usize;
    had_link = (old_data & 0x1) != 0;
    has_link = (data & 0x1) != 0;
    netdev_dbg((*ndp).ndev.dev, "NCSI: LSC AEN - channel %u state %s\n",
               (*nc).id, if data & 0x1 != 0 { "up" } else { "down" });
    chained = !list_empty(&(*nc).link);
    state = (*nc).state;
    spin_unlock_irqrestore(&mut (*nc).lock, flags);

    if state == NCSI_CHANNEL_INACTIVE {
        netdev_warn((*ndp).ndev.dev, "NCSI: Inactive channel %u received AEN!\n", (*nc).id);
    }
    if had_link == has_link || chained { return 0; }
    if !(*ndp).multi_package && !(*nc).package.multi_channel {
        if had_link {
            (*ndp).flags |= NCSI_DEV_RESHUFFLE;
            ncsi_stop_channel_monitor(nc);
            spin_lock_irqsave(&mut (*ndp).lock, &mut flags);
            list_add_tail_rcu(&mut (*nc).link, &mut (*ndp).channel_queue);
            spin_unlock_irqrestore(&mut (*ndp).lock, flags);
            return ncsi_process_next_channel(ndp);
        }
        return 0;
    }
    if had_link {
        ncm = &mut (*nc).modes[NCSI_MODE_TX_ENABLE as usize];
        if ncsi_channel_is_last(ndp, nc) { return ncsi_reset_dev(&mut (*ndp).ndev); }
        if (*ncm).enable { ncsi_update_tx_channel(ndp, (*nc).package, nc, core::ptr::null_mut()); }
    } else if has_link && (*nc).package.preferred_channel == nc {
        ncsi_update_tx_channel(ndp, (*nc).package, core::ptr::null_mut(), nc);
    } else if has_link {
        // NCSI_FOR_EACH_PACKAGE/NCSI_FOR_EACH_CHANNEL: iterate all packages and channels.
        // The macro topology is supplied by the surrounding kernel bindings.
        NCSI_FOR_EACH_PACKAGE!(ndp, np, {
            NCSI_FOR_EACH_CHANNEL!(np, tmp, {
                ncm = &mut (*tmp).modes[NCSI_MODE_TX_ENABLE as usize];
                if (*ncm).enable && !ncsi_channel_has_link(tmp) {
                    ncsi_update_tx_channel(ndp, (*nc).package, tmp, nc);
                    break;
                }
            });
        });
    }
    0
}

unsafe fn ncsi_aen_handler_cr(ndp: *mut ncsi_dev_priv, h: *mut ncsi_aen_pkt_hdr) -> i32 {
    let mut nc: *mut ncsi_channel = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    ncsi_find_package_and_channel(ndp, (*h).common.channel, core::ptr::null_mut(), &mut nc);
    if nc.is_null() { return -ENODEV; }
    spin_lock_irqsave(&mut (*nc).lock, &mut flags);
    if !list_empty(&(*nc).link) || (*nc).state != NCSI_CHANNEL_ACTIVE {
        spin_unlock_irqrestore(&mut (*nc).lock, flags); return 0;
    }
    spin_unlock_irqrestore(&mut (*nc).lock, flags);
    ncsi_stop_channel_monitor(nc);
    spin_lock_irqsave(&mut (*nc).lock, &mut flags); (*nc).state = NCSI_CHANNEL_INVISIBLE; spin_unlock_irqrestore(&mut (*nc).lock, flags);
    spin_lock_irqsave(&mut (*ndp).lock, &mut flags); (*nc).state = NCSI_CHANNEL_INACTIVE; list_add_tail_rcu(&mut (*nc).link, &mut (*ndp).channel_queue); spin_unlock_irqrestore(&mut (*ndp).lock, flags);
    (*nc).modes[NCSI_MODE_TX_ENABLE as usize].enable = 0;
    ncsi_process_next_channel(ndp)
}

unsafe fn ncsi_aen_handler_hncdsc(ndp: *mut ncsi_dev_priv, h: *mut ncsi_aen_pkt_hdr) -> i32 {
    let mut nc: *mut ncsi_channel = core::ptr::null_mut(); let mut flags: c_ulong = 0;
    ncsi_find_package_and_channel(ndp, (*h).common.channel, core::ptr::null_mut(), &mut nc);
    if nc.is_null() { return -ENODEV; }
    spin_lock_irqsave(&mut (*nc).lock, &mut flags);
    (*nc).modes[NCSI_MODE_LINK as usize].data[3] = ntohl((h as *mut ncsi_aen_hncdsc_pkt).read().status) as usize;
    spin_unlock_irqrestore(&mut (*nc).lock, flags);
    netdev_dbg((*ndp).ndev.dev, "NCSI: host driver %srunning on channel %u\n", if (*nc).modes[NCSI_MODE_LINK as usize].data[3] & 0x1 != 0 { "" } else { "not " }, (*nc).id);
    0
}

#[repr(C)]
struct ncsi_aen_handler { type_: u8, payload: i32, handler: unsafe fn(*mut ncsi_dev_priv, *mut ncsi_aen_pkt_hdr) -> i32 }

static mut NCSI_AEN_HANDLERS: [ncsi_aen_handler; 3] = [
    ncsi_aen_handler { type_: NCSI_PKT_AEN_LSC, payload: 12, handler: ncsi_aen_handler_lsc },
    ncsi_aen_handler { type_: NCSI_PKT_AEN_CR, payload: 4, handler: ncsi_aen_handler_cr },
    ncsi_aen_handler { type_: NCSI_PKT_AEN_HNCDSC, payload: 8, handler: ncsi_aen_handler_hncdsc },
];

pub unsafe fn ncsi_aen_handler(ndp: *mut ncsi_dev_priv, skb: *mut sk_buff) -> i32 {
    let h = skb_network_header(skb) as *mut ncsi_aen_pkt_hdr;
    let mut nah: *mut ncsi_aen_handler = core::ptr::null_mut();
    let mut i = 0;
    while i < NCSI_AEN_HANDLERS.len() { if NCSI_AEN_HANDLERS[i].type_ == (*h).type_ { nah = &mut NCSI_AEN_HANDLERS[i]; break; } i += 1; }
    let ret;
    if nah.is_null() { netdev_warn((*ndp).ndev.dev, "Invalid AEN (0x%x) received\n", (*h).type_); ret = -ENOENT; }
    else { ret = ncsi_validate_aen_pkt(h, (*nah).payload as u16); if ret == 0 { ret = ((*nah).handler)(ndp, h); } }
    consume_skb(skb); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
