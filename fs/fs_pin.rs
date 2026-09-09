// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

static mut PIN_LOCK: SpinLock = DEFINE_SPINLOCK!();

pub unsafe fn pin_remove(pin: *mut fs_pin) {
    spin_lock(&mut PIN_LOCK);
    hlist_del_init(&mut (*pin).m_list);
    hlist_del_init(&mut (*pin).s_list);
    spin_unlock(&mut PIN_LOCK);
    spin_lock_irq(&mut (*pin).wait.lock);
    (*pin).done = 1;
    wake_up_locked(&mut (*pin).wait);
    spin_unlock_irq(&mut (*pin).wait.lock);
}

pub unsafe fn pin_insert(pin: *mut fs_pin, m: *mut vfsmount) {
    spin_lock(&mut PIN_LOCK);
    hlist_add_head(&mut (*pin).s_list, &mut (*(*m).mnt_sb).s_pins);
    hlist_add_head(&mut (*pin).m_list, &mut (*real_mount(m)).mnt_pins);
    spin_unlock(&mut PIN_LOCK);
}

pub unsafe fn pin_kill(p: *mut fs_pin) {
    let mut wait: wait_queue_entry_t;

    if p.is_null() {
        rcu_read_unlock();
        return;
    }
    init_wait(&mut wait);
    spin_lock_irq(&mut (*p).wait.lock);
    if likely((*p).done == 0) {
        (*p).done = -1;
        spin_unlock_irq(&mut (*p).wait.lock);
        rcu_read_unlock();
        ((*p).kill)(p);
        return;
    }
    if (*p).done > 0 {
        spin_unlock_irq(&mut (*p).wait.lock);
        rcu_read_unlock();
        return;
    }
    __add_wait_queue(&mut (*p).wait, &mut wait);
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        spin_unlock_irq(&mut (*p).wait.lock);
        rcu_read_unlock();
        schedule();
        rcu_read_lock();
        if likely(list_empty(&wait.entry)) {
            break;
        }
        /* OK, we know p couldn't have been freed yet */
        spin_lock_irq(&mut (*p).wait.lock);
        if (*p).done > 0 {
            spin_unlock_irq(&mut (*p).wait.lock);
            break;
        }
    }
    rcu_read_unlock();
}

pub unsafe fn mnt_pin_kill(m: *mut mount) {
    loop {
        let p: *mut hlist_node;
        rcu_read_lock();
        p = READ_ONCE((*m).mnt_pins.first);
        if p.is_null() {
            rcu_read_unlock();
            break;
        }
        pin_kill(hlist_entry(p, fs_pin, m_list));
    }
}

pub unsafe fn group_pin_kill(p: *mut hlist_head) {
    loop {
        let q: *mut hlist_node;
        rcu_read_lock();
        q = READ_ONCE((*p).first);
        if q.is_null() {
            rcu_read_unlock();
            break;
        }
        pin_kill(hlist_entry(q, fs_pin, s_list));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
