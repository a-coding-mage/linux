// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 Facebook
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

#[repr(C)]
pub struct reuseport_array {
    pub map: bpf_map,
    pub ptrs: *mut *mut sock,
}

unsafe fn reuseport_array(map: *mut bpf_map) -> *mut reuseport_array {
    map as *mut reuseport_array
}

/* The caller must hold the reuseport_lock */
pub unsafe fn bpf_sk_reuseport_detach(sk: *mut sock) {
    let mut socks: *mut *mut sock;

    write_lock_bh(&mut (*sk).sk_callback_lock);
    socks = __locked_read_sk_user_data_with_flags(sk, SK_USER_DATA_BPF);
    if !socks.is_null() {
        WRITE_ONCE(&mut (*sk).sk_user_data, core::ptr::null_mut());
        /*
         * Do not move this NULL assignment outside of
         * sk->sk_callback_lock because there is
         * a race with reuseport_array_free()
         * which does not hold the reuseport_lock.
         */
        RCU_INIT_POINTER(socks, core::ptr::null_mut());
    }
    write_unlock_bh(&mut (*sk).sk_callback_lock);
}

unsafe fn reuseport_array_alloc_check(attr: *mut bpf_attr) -> i32 {
    if (*attr).value_size != core::mem::size_of::<u32>() as u32
        && (*attr).value_size != core::mem::size_of::<u64>() as u32
    {
        return -EINVAL;
    }

    array_map_alloc_check(attr)
}

unsafe fn reuseport_array_lookup_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut sock {
    let array = reuseport_array(map);
    let index = *(key as *mut u32);

    if unlikely(index >= (*array).map.max_entries) {
        return core::ptr::null_mut();
    }

    rcu_dereference((*array).ptrs.add(index as usize))
}

/* Called from syscall only */
unsafe fn reuseport_array_delete_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> i64 {
    let array = reuseport_array(map);
    let index = *(key as *mut u32);
    let sk: *mut sock;
    let err: i32;

    if index >= (*map).max_entries {
        return -E2BIG as i64;
    }

    if rcu_access_pointer((*array).ptrs.add(index as usize)).is_null() {
        return -ENOENT as i64;
    }

    spin_lock_bh(&mut reuseport_lock);
    sk = rcu_dereference_protected((*array).ptrs.add(index as usize), lockdep_is_held(&reuseport_lock));
    if !sk.is_null() {
        write_lock_bh(&mut (*sk).sk_callback_lock);
        WRITE_ONCE(&mut (*sk).sk_user_data, core::ptr::null_mut());
        RCU_INIT_POINTER((*array).ptrs.add(index as usize), core::ptr::null_mut());
        write_unlock_bh(&mut (*sk).sk_callback_lock);
        err = 0;
    } else {
        err = -ENOENT;
    }
    spin_unlock_bh(&mut reuseport_lock);
    err as i64
}

unsafe fn reuseport_array_free(map: *mut bpf_map) {
    let array = reuseport_array(map);
    let sk: *mut sock;
    let mut i: u32;

    /*
     * ops->map_*_elem() will not be able to access this
     * array now. Hence, this function only races with
     * bpf_sk_reuseport_detach() which was triggered by
     * close() or disconnect().
     *
     * This function and bpf_sk_reuseport_detach() are
     * both removing sk from "array".  Who removes it
     * first does not matter.
     *
     * The only concern here is bpf_sk_reuseport_detach()
     * may access "array" which is being freed here.
     * bpf_sk_reuseport_detach() access this "array"
     * through sk->sk_user_data _and_ with sk->sk_callback_lock
     * held which is enough because this "array" is not freed
     * until all sk->sk_user_data has stopped referencing this "array".
     *
     * Hence, due to the above, taking "reuseport_lock" is not
     * needed here.
     */
    /* Since reuseport_lock is not taken, sk is accessed under rcu_read_lock() */
    rcu_read_lock();
    i = 0;
    while i < (*map).max_entries {
        sk = rcu_dereference((*array).ptrs.add(i as usize));
        if !sk.is_null() {
            write_lock_bh(&mut (*sk).sk_callback_lock);
            (*sk).sk_user_data = core::ptr::null_mut();
            write_unlock_bh(&mut (*sk).sk_callback_lock);
            RCU_INIT_POINTER((*array).ptrs.add(i as usize), core::ptr::null_mut());
        }
        i += 1;
    }
    rcu_read_unlock();
    bpf_map_area_free(array as *mut core::ffi::c_void);
}

unsafe fn reuseport_array_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let numa_node = bpf_map_attr_numa_node(attr);
    let array = bpf_map_area_alloc(struct_size_reuseport_array((*attr).max_entries), numa_node)
        as *mut reuseport_array;
    if array.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&mut (*array).map, attr);
    &mut (*array).map
}

pub unsafe fn bpf_fd_reuseport_array_lookup_elem(
    map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void,
) -> i32 {
    let sk: *mut sock;
    let err: i32;
    if (*map).value_size != core::mem::size_of::<u64>() as u32 { return -ENOSPC; }
    rcu_read_lock();
    sk = reuseport_array_lookup_elem(map, key);
    if !sk.is_null() { *(value as *mut u64) = __sock_gen_cookie(sk); err = 0; }
    else { err = -ENOENT; }
    rcu_read_unlock();
    err
}

unsafe fn reuseport_array_update_check(
    _array: *const reuseport_array, nsk: *const sock, osk: *const sock,
    nsk_reuse: *const sock_reuseport, map_flags: u32,
) -> i32 {
    if !osk.is_null() && map_flags == BPF_NOEXIST { return -EEXIST; }
    if osk.is_null() && map_flags == BPF_EXIST { return -ENOENT; }
    if (*nsk).sk_protocol != IPPROTO_UDP && (*nsk).sk_protocol != IPPROTO_TCP { return -ENOTSUPP; }
    if (*nsk).sk_family != AF_INET && (*nsk).sk_family != AF_INET6 { return -ENOTSUPP; }
    if (*nsk).sk_type != SOCK_STREAM && (*nsk).sk_type != SOCK_DGRAM { return -ENOTSUPP; }
    if !sock_flag(nsk, SOCK_RCU_FREE) || !sk_hashed(nsk) || nsk_reuse.is_null() { return -EINVAL; }
    if !READ_ONCE((*nsk).sk_user_data).is_null() { return -EBUSY; }
    0
}

/* Called from syscall only. */
pub unsafe fn bpf_fd_reuseport_array_update_elem(
    map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, map_flags: u64,
) -> i32 {
    let array = reuseport_array(map);
    let index = *(key as *mut u32);
    let mut free_osk: *mut sock = core::ptr::null_mut();
    let mut osk: *mut sock;
    let mut nsk: *mut sock;
    let mut reuse: *mut sock_reuseport;
    let sk_user_data: usize;
    let socket: *mut socket;
    let mut err: i32;
    let fd: i32;

    if map_flags > BPF_EXIST as u64 { return -EINVAL; }
    if index >= (*map).max_entries { return -E2BIG; }
    if (*map).value_size == core::mem::size_of::<u64>() as u32 {
        let fd64 = *(value as *mut u64);
        if fd64 > S32_MAX as u64 { return -EINVAL; }
        fd = fd64 as i32;
    } else { fd = *(value as *mut i32); }
    socket = sockfd_lookup(fd, &mut err);
    if socket.is_null() { return err; }
    nsk = (*socket).sk;
    if nsk.is_null() { err = -EINVAL; goto_put_file(socket, err); }
    err = reuseport_array_update_check(array, nsk, rcu_access_pointer((*array).ptrs.add(index as usize)), rcu_access_pointer((*nsk).sk_reuseport_cb), map_flags as u32);
    if err != 0 { goto_put_file(socket, err); }
    spin_lock_bh(&mut reuseport_lock);
    write_lock_bh(&mut (*nsk).sk_callback_lock);
    osk = rcu_dereference_protected((*array).ptrs.add(index as usize), lockdep_is_held(&reuseport_lock));
    reuse = rcu_dereference_protected((*nsk).sk_reuseport_cb, lockdep_is_held(&reuseport_lock));
    err = reuseport_array_update_check(array, nsk, osk, reuse, map_flags as u32);
    if err == 0 {
        sk_user_data = (&mut *(*array).ptrs.add(index as usize) as *mut *mut sock as usize) | SK_USER_DATA_NOCOPY | SK_USER_DATA_BPF;
        WRITE_ONCE(&mut (*nsk).sk_user_data, sk_user_data as *mut core::ffi::c_void);
        rcu_assign_pointer((*array).ptrs.add(index as usize), nsk);
        free_osk = osk;
    }
    write_unlock_bh(&mut (*nsk).sk_callback_lock);
    if !free_osk.is_null() { write_lock_bh(&mut (*free_osk).sk_callback_lock); WRITE_ONCE(&mut (*free_osk).sk_user_data, core::ptr::null_mut()); write_unlock_bh(&mut (*free_osk).sk_callback_lock); }
    spin_unlock_bh(&mut reuseport_lock);
    sockfd_put(socket);
    err
}

unsafe fn goto_put_file(socket: *mut socket, err: i32) -> i32 { sockfd_put(socket); err }

/* Called from syscall */
unsafe fn reuseport_array_get_next_key(map: *mut bpf_map, key: *mut core::ffi::c_void, next_key: *mut core::ffi::c_void) -> i32 {
    let array = reuseport_array(map);
    let index = if key.is_null() { U32_MAX } else { *(key as *mut u32) };
    let next = next_key as *mut u32;
    if index >= (*array).map.max_entries { *next = 0; return 0; }
    if index == (*array).map.max_entries - 1 { return -ENOENT; }
    *next = index + 1;
    0
}

unsafe fn reuseport_array_mem_usage(map: *const bpf_map) -> u64 {
    struct_size_reuseport_array((*map).max_entries) as u64
}

// BTF_ID_LIST_SINGLE(reuseport_array_map_btf_ids, struct, reuseport_array)
#[no_mangle]
pub static reuseport_array_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal),
    map_alloc_check: Some(reuseport_array_alloc_check),
    map_alloc: Some(reuseport_array_alloc),
    map_free: Some(reuseport_array_free),
    map_lookup_elem: Some(reuseport_array_lookup_elem),
    map_get_next_key: Some(reuseport_array_get_next_key),
    map_delete_elem: Some(reuseport_array_delete_elem),
    map_mem_usage: Some(reuseport_array_mem_usage),
    map_btf_id: core::ptr::null(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
