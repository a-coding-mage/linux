// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2010-2011 EIA Electronics,
//                         Pieter Beyens <pieter.beyens@eia.be>
// Copyright (c) 2010-2011 EIA Electronics,
//                         Kurt Van Dijck <kurt.van.dijck@eia.be>
// Copyright (c) 2018 Protonic,
//                         Robin van der Gracht <robin@protonic.nl>
// Copyright (c) 2017-2019 Pengutronix,
//                         Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2017-2019 Pengutronix,
//                         Oleksij Rempel <kernel@pengutronix.de>

/* Core of can-j1939 that links j1939 to CAN. */

// Linux kernel includes and j1939-priv.h are supplied by the surrounding crate.

const J1939_CAN_HDR: usize = core::mem::offset_of!(CanFrame, data);
const J1939_CAN_ID: u32 = CAN_EFF_FLAG;
const J1939_CAN_MASK: u32 = CAN_EFF_FLAG | CAN_RTR_FLAG;

unsafe fn j1939_can_recv(iskb: *mut SkBuff, data: *mut core::ffi::c_void) {
    let priv_: *mut J1939Priv = data.cast();
    let skb: *mut SkBuff;
    let skcb: *mut J1939SkBuffCb;
    let iskcb: *mut J1939SkBuffCb;
    let cf: *mut CanFrame;

    if !can_is_can_skb(iskb) { return; }
    skb = skb_clone(iskb, GFP_ATOMIC);
    if skb.is_null() { return; }

    j1939_priv_get(priv_);
    can_skb_set_owner(skb, (*iskb).sk);
    cf = (*skb).data.cast();
    skb_pull(skb, J1939_CAN_HDR);
    skb_trim(skb, core::cmp::min((*cf).len as u8, 8));

    skcb = j1939_skb_to_cb(skb);
    core::ptr::write_bytes(skcb.cast::<u8>(), 0, core::mem::size_of::<J1939SkBuffCb>());
    iskcb = j1939_skb_to_cb(iskb);
    (*skcb).tskey = (*iskcb).tskey;
    (*skcb).priority = ((*cf).can_id >> 26) & 0x7;
    (*skcb).addr.sa = (*cf).can_id;
    (*skcb).addr.pgn = ((*cf).can_id >> 8) & J1939_PGN_MAX;
    (*skcb).addr.type_ = J1939_TP;

    if !j1939_address_is_valid((*skcb).addr.sa) {
        netdev_err_once((*priv_).ndev, "%s: sa is broadcast address, ignoring!\n", "j1939_can_recv");
        j1939_priv_put(priv_); kfree_skb(skb); return;
    }
    if j1939_pgn_is_pdu1((*skcb).addr.pgn) {
        (*skcb).addr.da = (*skcb).addr.pgn;
        (*skcb).addr.pgn &= 0x3ff00;
    } else { (*skcb).addr.da = J1939_NO_ADDR; }

    read_lock_bh(&(*priv_).lock);
    if j1939_address_is_unicast((*skcb).addr.sa) && (*priv_).ents[(*skcb).addr.sa as usize].nusers != 0 {
        (*skcb).flags |= J1939_ECU_LOCAL_SRC;
    }
    if j1939_address_is_unicast((*skcb).addr.da) && (*priv_).ents[(*skcb).addr.da as usize].nusers != 0 {
        (*skcb).flags |= J1939_ECU_LOCAL_DST;
    }
    read_unlock_bh(&(*priv_).lock);

    j1939_ac_recv(priv_, skb);
    if j1939_tp_recv(priv_, skb) { j1939_priv_put(priv_); kfree_skb(skb); return; }
    j1939_simple_recv(priv_, skb);
    j1939_sk_recv(priv_, skb);
    j1939_priv_put(priv_);
    kfree_skb(skb);
}

static mut J1939_NETDEV_LOCK: Mutex = Mutex::new();

unsafe fn j1939_priv_create(ndev: *mut NetDevice) -> *mut J1939Priv {
    let priv_ = kzalloc_obj::<J1939Priv>();
    if priv_.is_null() { return core::ptr::null_mut(); }
    rwlock_init(&mut (*priv_).lock);
    INIT_LIST_HEAD(&mut (*priv_).ecus);
    (*priv_).ndev = ndev;
    kref_init(&mut (*priv_).kref);
    kref_init(&mut (*priv_).rx_kref);
    netdev_hold(ndev, &mut (*priv_).dev_tracker, GFP_KERNEL);
    netdev_dbg((*priv_).ndev, "%s : 0x%p\n", "j1939_priv_create", priv_);
    priv_
}

unsafe fn j1939_priv_set(ndev: *mut NetDevice, priv_: *mut J1939Priv) {
    (*can_get_ml_priv(ndev)).j1939_priv = priv_;
}

unsafe extern "C" fn __j1939_priv_release(kref: *mut KRef) {
    let priv_ = container_of!(kref, J1939Priv, kref);
    let ndev = (*priv_).ndev;
    netdev_dbg((*priv_).ndev, "%s: 0x%p\n", "__j1939_priv_release", priv_);
    WARN_ON_ONCE(!list_empty(&(*priv_).active_session_list));
    WARN_ON_ONCE(!list_empty(&(*priv_).ecus));
    WARN_ON_ONCE(!list_empty(&(*priv_).j1939_socks));
    netdev_put(ndev, &mut (*priv_).dev_tracker);
    kfree(priv_);
}

pub unsafe fn j1939_priv_put(priv_: *mut J1939Priv) { kref_put(&mut (*priv_).kref, __j1939_priv_release); }
pub unsafe fn j1939_priv_get(priv_: *mut J1939Priv) { kref_get(&mut (*priv_).kref); }

unsafe fn j1939_can_rx_register(priv_: *mut J1939Priv) -> i32 {
    let ndev = (*priv_).ndev;
    j1939_priv_get(priv_);
    let ret = can_rx_register(dev_net(ndev), ndev, J1939_CAN_ID, J1939_CAN_MASK, j1939_can_recv, priv_.cast(), "j1939", core::ptr::null_mut());
    if ret < 0 { j1939_priv_put(priv_); return ret; }
    0
}

unsafe fn j1939_can_rx_unregister(priv_: *mut J1939Priv) {
    let ndev = (*priv_).ndev;
    can_rx_unregister(dev_net(ndev), ndev, J1939_CAN_ID, J1939_CAN_MASK, j1939_can_recv, priv_.cast());
    j1939_priv_put(priv_);
}

unsafe extern "C" fn __j1939_rx_release(kref: *mut KRef) {
    let priv_ = container_of!(kref, J1939Priv, rx_kref);
    j1939_can_rx_unregister(priv_);
    j1939_ecu_unmap_all(priv_);
    j1939_priv_set((*priv_).ndev, core::ptr::null_mut());
    mutex_unlock(&mut J1939_NETDEV_LOCK);
}

unsafe fn j1939_ndev_to_priv(ndev: *mut NetDevice) -> *mut J1939Priv { (*can_get_ml_priv(ndev)).j1939_priv }

unsafe fn j1939_priv_get_by_ndev_locked(ndev: *mut NetDevice) -> *mut J1939Priv {
    let priv_ = j1939_ndev_to_priv(ndev);
    if !priv_.is_null() { j1939_priv_get(priv_); }
    priv_
}

unsafe fn j1939_priv_get_by_ndev(ndev: *mut NetDevice) -> *mut J1939Priv {
    mutex_lock(&mut J1939_NETDEV_LOCK);
    let priv_ = j1939_priv_get_by_ndev_locked(ndev);
    mutex_unlock(&mut J1939_NETDEV_LOCK);
    priv_
}

pub unsafe fn j1939_netdev_start(ndev: *mut NetDevice) -> *mut J1939Priv {
    mutex_lock(&mut J1939_NETDEV_LOCK);
    let mut priv_ = j1939_priv_get_by_ndev_locked(ndev);
    if !priv_.is_null() { kref_get(&mut (*priv_).rx_kref); mutex_unlock(&mut J1939_NETDEV_LOCK); return priv_; }
    mutex_unlock(&mut J1939_NETDEV_LOCK);
    priv_ = j1939_priv_create(ndev);
    if priv_.is_null() { return ERR_PTR(-ENOMEM); }
    j1939_tp_init(priv_);
    rwlock_init(&mut (*priv_).j1939_socks_lock);
    INIT_LIST_HEAD(&mut (*priv_).j1939_socks);
    mutex_lock(&mut J1939_NETDEV_LOCK);
    let priv_new = j1939_priv_get_by_ndev_locked(ndev);
    if !priv_new.is_null() {
        kref_get(&mut (*priv_new).rx_kref); mutex_unlock(&mut J1939_NETDEV_LOCK);
        netdev_put(ndev, &mut (*priv_).dev_tracker); kfree(priv_); return priv_new;
    }
    j1939_priv_set(ndev, priv_);
    let ret = j1939_can_rx_register(priv_);
    if ret < 0 { j1939_priv_set(ndev, core::ptr::null_mut()); mutex_unlock(&mut J1939_NETDEV_LOCK); netdev_put(ndev, &mut (*priv_).dev_tracker); kfree(priv_); return ERR_PTR(ret); }
    mutex_unlock(&mut J1939_NETDEV_LOCK); priv_
}

pub unsafe fn j1939_netdev_stop(priv_: *mut J1939Priv) { kref_put_mutex(&mut (*priv_).rx_kref, __j1939_rx_release, &mut J1939_NETDEV_LOCK); j1939_priv_put(priv_); }

pub unsafe fn j1939_send_one(priv_: *mut J1939Priv, skb: *mut SkBuff) -> i32 {
    let skcb = j1939_skb_to_cb(skb);
    if j1939_pgn_is_pdu1((*skcb).addr.pgn) { (*skcb).addr.pgn &= J1939_PGN_PDU1_MAX; } else { (*skcb).addr.pgn &= J1939_PGN_MAX; }
    if (*skcb).priority > 7 { (*skcb).priority = 6; }
    let ret = j1939_ac_fixup(priv_, skb); if ret != 0 { kfree_skb(skb); return ret; }
    let dlc = (*skb).len;
    let cf = skb_push(skb, J1939_CAN_HDR).cast::<CanFrame>();
    core::ptr::write_bytes(cf.cast::<u8>(), 0, J1939_CAN_HDR);
    skb_put_zero(skb, 8 - dlc);
    let mut canid = CAN_EFF_FLAG | ((*skcb).priority << 26) | ((*skcb).addr.pgn << 8) | (*skcb).addr.sa;
    if j1939_pgn_is_pdu1((*skcb).addr.pgn) { canid |= (*skcb).addr.da << 8; }
    (*cf).can_id = canid; (*cf).len = dlc as u8;
    can_send(skb, 1)
}

unsafe fn j1939_netdev_notify(_nb: *mut NotifierBlock, msg: usize, data: *mut core::ffi::c_void) -> i32 {
    let ndev = netdev_notifier_info_to_dev(data);
    if can_get_ml_priv(ndev).is_null() { return NOTIFY_DONE; }
    let priv_ = j1939_priv_get_by_ndev(ndev); if priv_.is_null() { return NOTIFY_DONE; }
    match msg {
        NETDEV_DOWN => { j1939_cancel_active_session(priv_, core::ptr::null_mut()); j1939_sk_netdev_event_netdown(priv_); j1939_ecu_unmap_all(priv_); },
        NETDEV_UNREGISTER => { j1939_cancel_active_session(priv_, core::ptr::null_mut()); j1939_sk_netdev_event_netdown(priv_); j1939_sk_netdev_event_unregister(priv_); },
        _ => {}
    }
    j1939_priv_put(priv_); NOTIFY_DONE
}

static mut J1939_NETDEV_NOTIFIER: NotifierBlock = NotifierBlock { notifier_call: j1939_netdev_notify };

pub unsafe fn j1939_module_init() -> i32 {
    pr_info!("can: SAE J1939\n");
    let ret = register_netdevice_notifier(&mut J1939_NETDEV_NOTIFIER);
    if ret != 0 { return ret; }
    let ret = can_proto_register(&mut j1939_can_proto);
    if ret < 0 { pr_err!("can: registration of j1939 protocol failed\n"); unregister_netdevice_notifier(&mut J1939_NETDEV_NOTIFIER); }
    ret
}

pub unsafe fn j1939_module_exit() {
    can_proto_unregister(&mut j1939_can_proto);
    unregister_netdevice_notifier(&mut J1939_NETDEV_NOTIFIER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
