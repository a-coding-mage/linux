// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Google LLC
 */

// Translated from blk-crypto-profile.c.  Kernel-provided types, constants,
// functions, and synchronization primitives are supplied by other files.

#[repr(C)]
pub struct blk_crypto_keyslot {
    pub slot_refs: atomic_t,
    pub idle_slot_node: list_head,
    pub hash_node: hlist_node,
    pub key: *const blk_crypto_key,
    pub profile: *mut blk_crypto_profile,
}

#[inline]
unsafe fn blk_crypto_hw_enter(profile: *mut blk_crypto_profile) {
    if !(*profile).dev.is_null() {
        pm_runtime_get_sync((*profile).dev);
    }
    down_write(&mut (*profile).lock);
}

#[inline]
unsafe fn blk_crypto_hw_exit(profile: *mut blk_crypto_profile) {
    up_write(&mut (*profile).lock);
    if !(*profile).dev.is_null() {
        pm_runtime_put_sync((*profile).dev);
    }
}

pub unsafe fn blk_crypto_profile_init(
    profile: *mut blk_crypto_profile,
    num_slots: c_uint,
) -> c_int {
    let mut slot: c_uint;
    let mut i: c_uint;
    let mut slot_hashtable_size: c_uint;

    memset(profile as *mut c_void, 0, core::mem::size_of::<blk_crypto_profile>());
    lockdep_register_key(&mut (*profile).lockdep_key);
    __init_rwsem(&mut (*profile).lock, b"&profile->lock\0".as_ptr() as *const c_char,
                 &mut (*profile).lockdep_key);
    if num_slots == 0 { return 0; }

    (*profile).slots = kvzalloc_objs((*profile).slots.as_mut(), num_slots);
    if (*profile).slots.is_null() { blk_crypto_profile_destroy(profile); return -ENOMEM; }
    (*profile).num_slots = num_slots;
    init_waitqueue_head(&mut (*profile).idle_slots_wait_queue);
    INIT_LIST_HEAD(&mut (*profile).idle_slots);
    slot = 0;
    while slot < num_slots {
        (*(*profile).slots.add(slot as usize)).profile = profile;
        list_add_tail(&mut (*(*profile).slots.add(slot as usize)).idle_slot_node,
                      &mut (*profile).idle_slots);
        slot += 1;
    }
    spin_lock_init(&mut (*profile).idle_slots_lock);
    slot_hashtable_size = roundup_pow_of_two(num_slots);
    if slot_hashtable_size < 2 { slot_hashtable_size = 2; }
    (*profile).log_slot_ht_size = ilog2(slot_hashtable_size);
    (*profile).slot_hashtable = kvmalloc_objs((*profile).slot_hashtable.as_mut(), slot_hashtable_size);
    if (*profile).slot_hashtable.is_null() { blk_crypto_profile_destroy(profile); return -ENOMEM; }
    i = 0;
    while i < slot_hashtable_size {
        INIT_HLIST_HEAD(&mut *(*profile).slot_hashtable.add(i as usize));
        i += 1;
    }
    0
}

unsafe fn blk_crypto_profile_destroy_callback(profile: *mut c_void) {
    blk_crypto_profile_destroy(profile as *mut blk_crypto_profile);
}

pub unsafe fn devm_blk_crypto_profile_init(dev: *mut device, profile: *mut blk_crypto_profile,
                                           num_slots: c_uint) -> c_int {
    let err = blk_crypto_profile_init(profile, num_slots);
    if err != 0 { return err; }
    devm_add_action_or_reset(dev, blk_crypto_profile_destroy_callback, profile as *mut c_void)
}

unsafe fn blk_crypto_hash_bucket_for_key(profile: *mut blk_crypto_profile,
                                         key: *const blk_crypto_key) -> *mut hlist_head {
    (*profile).slot_hashtable.add(hash_ptr(key as *const c_void, (*profile).log_slot_ht_size) as usize)
}

unsafe fn blk_crypto_remove_slot_from_lru_list(slot: *mut blk_crypto_keyslot) {
    let profile = (*slot).profile;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*profile).idle_slots_lock, &mut flags);
    list_del(&mut (*slot).idle_slot_node);
    spin_unlock_irqrestore(&mut (*profile).idle_slots_lock, flags);
}

unsafe fn blk_crypto_find_keyslot(profile: *mut blk_crypto_profile,
                                  key: *const blk_crypto_key) -> *mut blk_crypto_keyslot {
    let head = blk_crypto_hash_bucket_for_key(profile, key);
    let mut slotp: *mut blk_crypto_keyslot = core::ptr::null_mut();
    hlist_for_each_entry(&mut slotp, head, hash_node) {
        if (*slotp).key == key { return slotp; }
    }
    core::ptr::null_mut()
}

unsafe fn blk_crypto_find_and_grab_keyslot(profile: *mut blk_crypto_profile,
                                           key: *const blk_crypto_key) -> *mut blk_crypto_keyslot {
    let slot = blk_crypto_find_keyslot(profile, key);
    if slot.is_null() { return core::ptr::null_mut(); }
    if atomic_inc_return(&mut (*slot).slot_refs) == 1 { blk_crypto_remove_slot_from_lru_list(slot); }
    slot
}

pub unsafe fn blk_crypto_keyslot_index(slot: *mut blk_crypto_keyslot) -> c_uint {
    slot.offset_from((*slot).profile.as_ref().unwrap().slots) as c_uint
}

pub unsafe fn blk_crypto_get_keyslot(profile: *mut blk_crypto_profile,
                                     key: *const blk_crypto_key,
                                     slot_ptr: *mut *mut blk_crypto_keyslot) -> blk_status_t {
    *slot_ptr = core::ptr::null_mut();
    if (*profile).num_slots == 0 { return BLK_STS_OK; }
    down_read(&mut (*profile).lock);
    let mut slot = blk_crypto_find_and_grab_keyslot(profile, key);
    up_read(&mut (*profile).lock);
    if !slot.is_null() { *slot_ptr = slot; return BLK_STS_OK; }
    loop {
        blk_crypto_hw_enter(profile);
        slot = blk_crypto_find_and_grab_keyslot(profile, key);
        if !slot.is_null() { blk_crypto_hw_exit(profile); *slot_ptr = slot; return BLK_STS_OK; }
        if !list_empty(&(*profile).idle_slots) { break; }
        blk_crypto_hw_exit(profile);
        wait_event(&mut (*profile).idle_slots_wait_queue, !list_empty(&(*profile).idle_slots));
    }
    slot = list_first_entry(&mut (*profile).idle_slots, blk_crypto_keyslot, idle_slot_node);
    let slot_idx = blk_crypto_keyslot_index(slot) as c_int;
    let err = ((*profile).ll_ops.keyslot_program.unwrap())(profile, key, slot_idx);
    if err != 0 { wake_up(&mut (*profile).idle_slots_wait_queue); blk_crypto_hw_exit(profile); return errno_to_blk_status(err); }
    if !(*slot).key.is_null() { hlist_del(&mut (*slot).hash_node); }
    (*slot).key = key;
    hlist_add_head(&mut (*slot).hash_node, blk_crypto_hash_bucket_for_key(profile, key));
    atomic_set(&mut (*slot).slot_refs, 1);
    blk_crypto_remove_slot_from_lru_list(slot);
    blk_crypto_hw_exit(profile);
    *slot_ptr = slot;
    BLK_STS_OK
}

pub unsafe fn blk_crypto_put_keyslot(slot: *mut blk_crypto_keyslot) {
    let profile = (*slot).profile;
    let mut flags: c_ulong = 0;
    if atomic_dec_and_lock_irqsave(&mut (*slot).slot_refs, &mut (*profile).idle_slots_lock, &mut flags) {
        list_add_tail(&mut (*slot).idle_slot_node, &mut (*profile).idle_slots);
        spin_unlock_irqrestore(&mut (*profile).idle_slots_lock, flags);
        wake_up(&mut (*profile).idle_slots_wait_queue);
    }
}

pub unsafe fn __blk_crypto_evict_key(profile: *mut blk_crypto_profile, key: *const blk_crypto_key) -> c_int {
    if (*profile).num_slots == 0 {
        if let Some(evict) = (*profile).ll_ops.keyslot_evict { blk_crypto_hw_enter(profile); let err = evict(profile, key, -1); blk_crypto_hw_exit(profile); return err; }
        return 0;
    }
    blk_crypto_hw_enter(profile);
    let slot = blk_crypto_find_keyslot(profile, key);
    if slot.is_null() { blk_crypto_hw_exit(profile); return 0; }
    let mut err = 0;
    if atomic_read(&(*slot).slot_refs) != 0 { err = -EBUSY; }
    else if let Some(evict) = (*profile).ll_ops.keyslot_evict { err = evict(profile, key, blk_crypto_keyslot_index(slot) as c_int); }
    hlist_del(&mut (*slot).hash_node); (*slot).key = core::ptr::null();
    blk_crypto_hw_exit(profile); err
}

pub unsafe fn blk_crypto_reprogram_all_keys(profile: *mut blk_crypto_profile) {
    if (*profile).num_slots == 0 { return; }
    down_write(&mut (*profile).lock);
    let mut slot = 0;
    while slot < (*profile).num_slots {
        let key = (*(*profile).slots.add(slot as usize)).key;
        if !key.is_null() { ((*profile).ll_ops.keyslot_program.unwrap())(profile, key, slot as c_int); }
        slot += 1;
    }
    up_write(&mut (*profile).lock);
}

pub unsafe fn blk_crypto_profile_destroy(profile: *mut blk_crypto_profile) {
    if profile.is_null() { return; }
    lockdep_unregister_key(&mut (*profile).lockdep_key);
    kvfree((*profile).slot_hashtable as *mut c_void);
    kvfree_sensitive((*profile).slots as *mut c_void,
                     core::mem::size_of::<blk_crypto_keyslot>() * (*profile).num_slots as usize);
    memzero_explicit(profile as *mut c_void, core::mem::size_of::<blk_crypto_profile>());
}

pub unsafe fn blk_crypto_register(profile: *mut blk_crypto_profile, q: *mut request_queue) -> bool {
    if blk_integrity_queue_supports_integrity(q) { pr_warn!("Integrity and hardware inline encryption are not supported together. Disabling hardware inline encryption.\n"); return false; }
    (*q).crypto_profile = profile; true
}

pub unsafe fn blk_crypto_derive_sw_secret(bdev: *mut block_device, eph_key: *const u8,
                                          eph_key_size: size_t, sw_secret: *mut u8) -> c_int {
    let profile = bdev_get_queue(bdev).as_mut().unwrap().crypto_profile;
    if profile.is_null() || (*profile).key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED == 0 { return -EOPNOTSUPP; }
    let Some(f) = (*profile).ll_ops.derive_sw_secret else { return -EOPNOTSUPP; };
    blk_crypto_hw_enter(profile); let err = f(profile, eph_key, eph_key_size, sw_secret); blk_crypto_hw_exit(profile); err
}

pub unsafe fn blk_crypto_import_key(profile: *mut blk_crypto_profile, raw_key: *const u8,
                                    raw_key_size: size_t, lt_key: *mut u8) -> c_int {
    if profile.is_null() || (*profile).key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED == 0 { return -EOPNOTSUPP; }
    let Some(f) = (*profile).ll_ops.import_key else { return -EOPNOTSUPP; };
    blk_crypto_hw_enter(profile); let ret = f(profile, raw_key, raw_key_size, lt_key); blk_crypto_hw_exit(profile); ret
}

pub unsafe fn blk_crypto_generate_key(profile: *mut blk_crypto_profile, lt_key: *mut u8) -> c_int {
    if profile.is_null() || (*profile).key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED == 0 { return -EOPNOTSUPP; }
    let Some(f) = (*profile).ll_ops.generate_key else { return -EOPNOTSUPP; };
    blk_crypto_hw_enter(profile); let ret = f(profile, lt_key); blk_crypto_hw_exit(profile); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
