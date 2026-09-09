// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * Alexander Aring <aar@pengutronix.de>
 *
 * Based on: net/mac80211/util.c
 */

// Dependencies supplied by the surrounding translation unit/build.

/* privid for wpan_phys to determine whether they belong to us or not */
pub static mac802154_wpan_phy_privid: *const core::ffi::c_void =
    &mac802154_wpan_phy_privid as *const _ as *const core::ffi::c_void;

/**
 * ieee802154_wake_queue - wake ieee802154 queue
 * @hw: main hardware object
 *
 * Tranceivers usually have either one transmit framebuffer or one framebuffer
 * for both transmitting and receiving. Hence, the core currently only handles
 * one frame at a time for each phy, which means we had to stop the queue to
 * avoid new skb to come during the transmission. The queue then needs to be
 * woken up after the operation.
 */
unsafe fn ieee802154_wake_queue(hw: *mut ieee802154_hw) {
    let local = hw_to_local(hw);
    let mut sdata: *mut ieee802154_sub_if_data;

    rcu_read_lock();
    clear_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &mut (*(*local).phy).flags);
    list_for_each_entry_rcu!(sdata, &(*local).interfaces, list, {
        if (*sdata).dev.is_null() {
            continue;
        }
        netif_wake_queue((*sdata).dev);
    });
    rcu_read_unlock();
}

/**
 * ieee802154_stop_queue - stop ieee802154 queue
 * @hw: main hardware object
 *
 * Tranceivers usually have either one transmit framebuffer or one framebuffer
 * for both transmitting and receiving. Hence, the core currently only handles
 * one frame at a time for each phy, which means we need to tell upper layers to
 * stop giving us new skbs while we are busy with the transmitted one. The queue
 * must then be stopped before transmitting.
 */
unsafe fn ieee802154_stop_queue(hw: *mut ieee802154_hw) {
    let local = hw_to_local(hw);
    let mut sdata: *mut ieee802154_sub_if_data;

    rcu_read_lock();
    list_for_each_entry_rcu!(sdata, &(*local).interfaces, list, {
        if (*sdata).dev.is_null() {
            continue;
        }
        netif_stop_queue((*sdata).dev);
    });
    rcu_read_unlock();
}

pub unsafe fn ieee802154_hold_queue(local: *mut ieee802154_local) {
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave!(&mut (*(*local).phy).queue_lock, flags);
    if !atomic_fetch_inc(&mut (*(*local).phy).hold_txs) {
        ieee802154_stop_queue(&mut (*local).hw);
    }
    spin_unlock_irqrestore!(&mut (*(*local).phy).queue_lock, flags);
}

pub unsafe fn ieee802154_release_queue(local: *mut ieee802154_local) {
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave!(&mut (*(*local).phy).queue_lock, flags);
    if atomic_dec_and_test(&mut (*(*local).phy).hold_txs) {
        ieee802154_wake_queue(&mut (*local).hw);
    }
    spin_unlock_irqrestore!(&mut (*(*local).phy).queue_lock, flags);
}

pub unsafe fn ieee802154_disable_queue(local: *mut ieee802154_local) {
    let mut sdata: *mut ieee802154_sub_if_data;

    rcu_read_lock();
    list_for_each_entry_rcu!(sdata, &(*local).interfaces, list, {
        if (*sdata).dev.is_null() {
            continue;
        }
        netif_tx_disable((*sdata).dev);
    });
    rcu_read_unlock();
}

pub unsafe fn ieee802154_xmit_ifs_timer(timer: *mut hrtimer) -> hrtimer_restart {
    let local = container_of!(timer, ieee802154_local, ifs_timer);

    ieee802154_release_queue(local);
    HRTIMER_NORESTART
}

pub unsafe fn ieee802154_xmit_complete(
    hw: *mut ieee802154_hw,
    skb: *mut sk_buff,
    ifs_handling: bool,
) {
    let local = hw_to_local(hw);

    (*local).tx_result = IEEE802154_SUCCESS;

    if ifs_handling {
        let max_sifs_size: u8;

        /* If transceiver sets CRC on his own we need to use lifs
         * threshold len above 16 otherwise 18, because it's not
         * part of skb->len.
         */
        if (*hw).flags & IEEE802154_HW_TX_OMIT_CKSUM != 0 {
            max_sifs_size = IEEE802154_MAX_SIFS_FRAME_SIZE - IEEE802154_FCS_LEN;
        } else {
            max_sifs_size = IEEE802154_MAX_SIFS_FRAME_SIZE;
        }

        if (*skb).len > max_sifs_size {
            hrtimer_start(
                &mut (*local).ifs_timer,
                (*(*hw).phy).lifs_period * NSEC_PER_USEC,
                HRTIMER_MODE_REL,
            );
        } else {
            hrtimer_start(
                &mut (*local).ifs_timer,
                (*(*hw).phy).sifs_period * NSEC_PER_USEC,
                HRTIMER_MODE_REL,
            );
        }
    } else {
        ieee802154_release_queue(local);
    }

    dev_consume_skb_any(skb);
    if atomic_dec_and_test(&mut (*(*hw).phy).ongoing_txs) {
        wake_up(&mut (*(*hw).phy).sync_txq);
    }
}

pub unsafe fn ieee802154_xmit_error(
    hw: *mut ieee802154_hw,
    skb: *mut sk_buff,
    reason: core::ffi::c_int,
) {
    let local = hw_to_local(hw);

    (*local).tx_result = reason;
    ieee802154_release_queue(local);
    dev_kfree_skb_any(skb);
    if atomic_dec_and_test(&mut (*(*hw).phy).ongoing_txs) {
        wake_up(&mut (*(*hw).phy).sync_txq);
    }
}

pub unsafe fn ieee802154_xmit_hw_error(hw: *mut ieee802154_hw, skb: *mut sk_buff) {
    ieee802154_xmit_error(hw, skb, IEEE802154_SYSTEM_ERROR);
}

pub unsafe fn ieee802154_stop_device(local: *mut ieee802154_local) {
    flush_workqueue((*local).workqueue);
    hrtimer_cancel(&mut (*local).ifs_timer);
    drv_stop(local);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
