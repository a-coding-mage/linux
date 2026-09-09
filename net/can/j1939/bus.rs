// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2010-2011 EIA Electronics,
//                         Kurt Van Dijck <kurt.van.dijck@eia.be>
// Copyright (c) 2017-2019 Pengutronix,
//                         Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2017-2019 Pengutronix,
//                         Oleksij Rempel <kernel@pengutronix.de>

/* bus for j1939 remote devices
 * Since rtnetlink, no real bus is used.
 */

// C dependencies: <net/sock.h> and "j1939-priv.h".

unsafe fn __j1939_ecu_release(kref: *mut kref) {
    let ecu = container_of!(kref, j1939_ecu, kref);
    let priv_ = (*ecu).priv_;

    list_del(&mut (*ecu).list);
    netdev_put((*priv_).ndev, &mut (*ecu).priv_dev_tracker);
    kfree(ecu);
    j1939_priv_put(priv_);
}

pub unsafe fn j1939_ecu_put(ecu: *mut j1939_ecu) {
    kref_put(&mut (*ecu).kref, __j1939_ecu_release);
}

unsafe fn j1939_ecu_get(ecu: *mut j1939_ecu) {
    kref_get(&mut (*ecu).kref);
}

unsafe fn j1939_ecu_is_mapped_locked(ecu: *mut j1939_ecu) -> bool {
    let priv_ = (*ecu).priv_;

    lockdep_assert_held(&(*priv_).lock);

    j1939_ecu_find_by_addr_locked(priv_, (*ecu).addr) == ecu
}

/* ECU device interface */
/* map ECU to a bus address space */
unsafe fn j1939_ecu_map_locked(ecu: *mut j1939_ecu) {
    let priv_ = (*ecu).priv_;
    let ent: *mut j1939_addr_ent;

    lockdep_assert_held(&(*priv_).lock);

    if !j1939_address_is_unicast((*ecu).addr) {
        return;
    }

    ent = &mut (*priv_).ents[(*ecu).addr as usize];

    if !(*ent).ecu.is_null() {
        netdev_warn((*priv_).ndev, "Trying to map already mapped ECU, addr: 0x%02x, name: 0x%016llx. Skip it.\n",
                    (*ecu).addr, (*ecu).name);
        return;
    }

    j1939_ecu_get(ecu);
    (*ent).ecu = ecu;
    (*ent).nusers += (*ecu).nusers;
}

/* unmap ECU from a bus address space */
pub unsafe fn j1939_ecu_unmap_locked(ecu: *mut j1939_ecu) {
    let priv_ = (*ecu).priv_;
    let ent: *mut j1939_addr_ent;

    lockdep_assert_held(&(*priv_).lock);

    if !j1939_address_is_unicast((*ecu).addr) {
        return;
    }

    if !j1939_ecu_is_mapped_locked(ecu) {
        return;
    }

    ent = &mut (*priv_).ents[(*ecu).addr as usize];
    (*ent).ecu = core::ptr::null_mut();
    (*ent).nusers -= (*ecu).nusers;
    j1939_ecu_put(ecu);
}

pub unsafe fn j1939_ecu_unmap(ecu: *mut j1939_ecu) {
    write_lock_bh(&mut (*(*ecu).priv_).lock);
    j1939_ecu_unmap_locked(ecu);
    write_unlock_bh(&mut (*(*ecu).priv_).lock);
}

pub unsafe fn j1939_ecu_unmap_all(priv_: *mut j1939_priv) {
    let mut i: i32;

    write_lock_bh(&mut (*priv_).lock);
    i = 0;
    while (i as usize) < ARRAY_SIZE!((*priv_).ents) {
        if !(*priv_).ents[i as usize].ecu.is_null() {
            j1939_ecu_unmap_locked((*priv_).ents[i as usize].ecu);
        }
        i += 1;
    }
    write_unlock_bh(&mut (*priv_).lock);
}

pub unsafe fn j1939_ecu_timer_start(ecu: *mut j1939_ecu) {
    /* The ECU is held here and released in the
     * j1939_ecu_timer_handler() or j1939_ecu_timer_cancel().
     */
    j1939_ecu_get(ecu);

    /* Schedule timer in 250 msec to commit address change. */
    hrtimer_start(&mut (*ecu).ac_timer, ms_to_ktime(250), HRTIMER_MODE_REL_SOFT);
}

pub unsafe fn j1939_ecu_timer_cancel(ecu: *mut j1939_ecu) {
    if hrtimer_cancel(&mut (*ecu).ac_timer) {
        j1939_ecu_put(ecu);
    }
}

unsafe fn j1939_ecu_timer_handler(hrtimer: *mut hrtimer) -> hrtimer_restart {
    let ecu = container_of!(hrtimer, j1939_ecu, ac_timer);
    let priv_ = (*ecu).priv_;

    write_lock_bh(&mut (*priv_).lock);
    /* TODO: can we test if ecu->addr is unicast before starting
     * the timer?
     */
    j1939_ecu_map_locked(ecu);

    /* The corresponding j1939_ecu_get() is in
     * j1939_ecu_timer_start().
     */
    j1939_ecu_put(ecu);
    write_unlock_bh(&mut (*priv_).lock);

    HRTIMER_NORESTART
}

pub unsafe fn j1939_ecu_create_locked(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu {
    let ecu = kzalloc_obj!(j1939_ecu, gfp_any());

    lockdep_assert_held(&(*priv_).lock);

    if ecu.is_null() {
        return ERR_PTR!(-ENOMEM);
    }
    kref_init(&mut (*ecu).kref);
    netdev_hold((*priv_).ndev, &mut (*ecu).priv_dev_tracker, gfp_any());
    (*ecu).addr = J1939_IDLE_ADDR;
    (*ecu).name = name;

    hrtimer_setup(&mut (*ecu).ac_timer, j1939_ecu_timer_handler, CLOCK_MONOTONIC,
                  HRTIMER_MODE_REL_SOFT);
    INIT_LIST_HEAD(&mut (*ecu).list);

    j1939_priv_get(priv_);
    (*ecu).priv_ = priv_;
    list_add_tail(&mut (*ecu).list, &mut (*priv_).ecus);

    ecu
}

pub unsafe fn j1939_ecu_find_by_addr_locked(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu {
    lockdep_assert_held(&(*priv_).lock);
    (*priv_).ents[addr as usize].ecu
}

pub unsafe fn j1939_ecu_get_by_addr_locked(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu {
    lockdep_assert_held(&(*priv_).lock);
    if !j1939_address_is_unicast(addr) {
        return core::ptr::null_mut();
    }
    let ecu = j1939_ecu_find_by_addr_locked(priv_, addr);
    if !ecu.is_null() { j1939_ecu_get(ecu); }
    ecu
}

pub unsafe fn j1939_ecu_get_by_addr(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu {
    read_lock_bh(&mut (*priv_).lock);
    let ecu = j1939_ecu_get_by_addr_locked(priv_, addr);
    read_unlock_bh(&mut (*priv_).lock);
    ecu
}

/* get pointer to ecu without increasing ref counter */
unsafe fn j1939_ecu_find_by_name_locked(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu {
    lockdep_assert_held(&(*priv_).lock);
    let mut ecu = (*priv_).ecus.next as *mut j1939_ecu;
    while !ecu.is_null() {
        if (*ecu).name == name { return ecu; }
        ecu = (*ecu).list.next as *mut j1939_ecu;
    }
    core::ptr::null_mut()
}

pub unsafe fn j1939_ecu_get_by_name_locked(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu {
    lockdep_assert_held(&(*priv_).lock);
    if name == 0 { return core::ptr::null_mut(); }
    let ecu = j1939_ecu_find_by_name_locked(priv_, name);
    if !ecu.is_null() { j1939_ecu_get(ecu); }
    ecu
}

pub unsafe fn j1939_ecu_get_by_name(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu {
    read_lock_bh(&mut (*priv_).lock);
    let ecu = j1939_ecu_get_by_name_locked(priv_, name);
    read_unlock_bh(&mut (*priv_).lock);
    ecu
}

pub unsafe fn j1939_name_to_addr(priv_: *mut j1939_priv, name: name_t) -> u8 {
    let mut addr = J1939_IDLE_ADDR;
    if name == 0 { return J1939_NO_ADDR; }
    read_lock_bh(&mut (*priv_).lock);
    let ecu = j1939_ecu_find_by_name_locked(priv_, name);
    if !ecu.is_null() && j1939_ecu_is_mapped_locked(ecu) { addr = (*ecu).addr; }
    read_unlock_bh(&mut (*priv_).lock);
    addr
}

/* TX addr/name accounting
 * Transport protocol needs to know if a SA is local or not
 * These functions originate from userspace manipulating sockets,
 * so locking is straigforward
 */
pub unsafe fn j1939_local_ecu_get(priv_: *mut j1939_priv, name: name_t, sa: u8) -> i32 {
    let mut err = 0;
    write_lock_bh(&mut (*priv_).lock);
    if j1939_address_is_unicast(sa) { (*priv_).ents[sa as usize].nusers += 1; }
    if name == 0 { write_unlock_bh(&mut (*priv_).lock); return err; }
    let mut ecu = j1939_ecu_get_by_name_locked(priv_, name);
    if ecu.is_null() { ecu = j1939_ecu_create_locked(priv_, name); }
    err = PTR_ERR_OR_ZERO!(ecu);
    if err != 0 {
        if j1939_address_is_unicast(sa) { (*priv_).ents[sa as usize].nusers -= 1; }
        write_unlock_bh(&mut (*priv_).lock); return err;
    }
    (*ecu).nusers += 1;
    if j1939_ecu_is_mapped_locked(ecu) { (*priv_).ents[(*ecu).addr as usize].nusers += 1; }
    write_unlock_bh(&mut (*priv_).lock);
    err
}

pub unsafe fn j1939_local_ecu_put(priv_: *mut j1939_priv, name: name_t, sa: u8) {
    write_lock_bh(&mut (*priv_).lock);
    if j1939_address_is_unicast(sa) { (*priv_).ents[sa as usize].nusers -= 1; }
    if name == 0 { write_unlock_bh(&mut (*priv_).lock); return; }
    let ecu = j1939_ecu_find_by_name_locked(priv_, name);
    if WARN_ON_ONCE!(ecu.is_null()) { write_unlock_bh(&mut (*priv_).lock); return; }
    (*ecu).nusers -= 1;
    if j1939_ecu_is_mapped_locked(ecu) { (*priv_).ents[(*ecu).addr as usize].nusers -= 1; }
    j1939_ecu_put(ecu);
    write_unlock_bh(&mut (*priv_).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
