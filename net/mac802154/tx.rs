// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007-2012 Siemens AG
 *
 * Written by:
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Sergey Lapin <slapin@ossfans.org>
 * Maxim Gorbachyov <maxim.gorbachev@siemens.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub unsafe fn ieee802154_xmit_sync_worker(work: *mut work_struct) {
    let local: *mut ieee802154_local = container_of!(work, ieee802154_local, sync_tx_work);
    let skb = (*local).tx_skb;
    let dev = (*skb).dev;
    let res = drv_xmit_sync(local, skb);
    if res != 0 {
        ieee802154_release_queue(local);
        if atomic_dec_and_test(&mut (*(*local).phy).ongoing_txs) {
            wake_up(&mut (*(*local).phy).sync_txq);
        }
        kfree_skb(skb);
        netdev_dbg(dev, "transmission failed\n");
        return;
    }

    DEV_STATS_INC(dev, tx_packets);
    DEV_STATS_ADD(dev, tx_bytes, (*skb).len);
    ieee802154_xmit_complete(&mut (*local).hw, skb, false);
}

unsafe fn ieee802154_tx(local: *mut ieee802154_local, mut skb: *mut sk_buff) -> netdev_tx_t {
    let dev = (*skb).dev;
    let ret: i32;

    if ((*local).hw.flags & IEEE802154_HW_TX_OMIT_CKSUM) == 0 {
        if unlikely(skb_tailroom(skb) < IEEE802154_FCS_LEN) {
            let nskb = skb_copy_expand(skb, 0, IEEE802154_FCS_LEN, GFP_ATOMIC);
            if likely(!nskb.is_null()) {
                consume_skb(skb);
                skb = nskb;
            } else {
                kfree_skb(skb);
                return NETDEV_TX_OK;
            }
        }
        let crc: u16 = crc_ccitt(0, (*skb).data, (*skb).len);
        put_unaligned_le16(crc, skb_put(skb, 2));
    }

    ieee802154_hold_queue(local);
    atomic_inc(&mut (*(*local).phy).ongoing_txs);

    if !(*local).ops.xmit_async.is_none() {
        let len = (*skb).len;
        ret = drv_xmit_async(local, skb);
        if ret != 0 {
            ieee802154_release_queue(local);
            if atomic_dec_and_test(&mut (*(*local).phy).ongoing_txs) {
                wake_up(&mut (*(*local).phy).sync_txq);
            }
            kfree_skb(skb);
            return NETDEV_TX_OK;
        }
        DEV_STATS_INC(dev, tx_packets);
        DEV_STATS_ADD(dev, tx_bytes, len);
    } else {
        (*local).tx_skb = skb;
        queue_work((*local).workqueue, &mut (*local).sync_tx_work);
    }
    NETDEV_TX_OK
}

unsafe fn ieee802154_sync_queue(local: *mut ieee802154_local) -> i32 {
    ieee802154_hold_queue(local);
    ieee802154_disable_queue(local);
    wait_event!((*(*local).phy).sync_txq, atomic_read(&(*(*local).phy).ongoing_txs) == 0);
    let ret = (*local).tx_result;
    ieee802154_release_queue(local);
    ret
}

pub unsafe fn ieee802154_sync_and_hold_queue(local: *mut ieee802154_local) -> i32 {
    ieee802154_hold_queue(local);
    let ret = ieee802154_sync_queue(local);
    set_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &mut (*(*local).phy).flags);
    ret
}

pub unsafe fn ieee802154_mlme_op_pre(local: *mut ieee802154_local) -> i32 {
    ieee802154_sync_and_hold_queue(local)
}

pub unsafe fn ieee802154_mlme_tx_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32 {
    ASSERT_RTNL!();
    if (*local).open_count == 0 { return -ENETDOWN; }
    if WARN_ON_ONCE!(!netif_running((*sdata).dev)) { return -ENETDOWN; }
    ieee802154_tx(local, skb);
    ieee802154_sync_queue(local)
}

pub unsafe fn ieee802154_mlme_tx(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32 {
    rtnl_lock();
    let ret = ieee802154_mlme_tx_locked(local, sdata, skb);
    rtnl_unlock();
    ret
}

pub unsafe fn ieee802154_mlme_op_post(local: *mut ieee802154_local) {
    ieee802154_release_queue(local);
}

pub unsafe fn ieee802154_mlme_tx_one_locked(local: *mut ieee802154_local, sdata: *mut ieee802154_sub_if_data, skb: *mut sk_buff) -> i32 {
    ieee802154_mlme_op_pre(local);
    let ret = ieee802154_mlme_tx_locked(local, sdata, skb);
    ieee802154_mlme_op_post(local);
    ret
}

unsafe fn ieee802154_queue_is_stopped(local: *mut ieee802154_local) -> bool {
    test_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &(*(*local).phy).flags)
}

unsafe fn ieee802154_hot_tx(local: *mut ieee802154_local, skb: *mut sk_buff) -> netdev_tx_t {
    WARN_ON_ONCE!(ieee802154_queue_is_stopped(local));
    ieee802154_tx(local, skb)
}

pub unsafe fn ieee802154_monitor_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let sdata = IEEE802154_DEV_TO_SUB_IF!(dev);
    (*skb).skb_iif = (*dev).ifindex;
    ieee802154_hot_tx((*sdata).local, skb)
}

pub unsafe fn ieee802154_subif_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let sdata = IEEE802154_DEV_TO_SUB_IF!(dev);
    let rc = mac802154_llsec_encrypt(&mut (*sdata).sec, skb);
    if rc != 0 {
        netdev_warn(dev, "encryption failed: %i\n", rc);
        kfree_skb(skb);
        return NETDEV_TX_OK;
    }
    (*skb).skb_iif = (*dev).ifindex;
    ieee802154_hot_tx((*sdata).local, skb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
