// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/memory.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

// Linux kernel headers and "common.h" are supplied by the surrounding crate.

pub unsafe fn tomoyo_warn_oom(function: *const libc::c_char) {
    /* Reduce error messages. */
    static mut TOMOYO_LAST_PID: libc::pid_t = 0;
    let pid = (*current).pid;

    if TOMOYO_LAST_PID != pid {
        pr_warn(b"ERROR: Out of memory at %s.\n\0".as_ptr() as *const libc::c_char, function);
        TOMOYO_LAST_PID = pid;
    }
    if !tomoyo_policy_loaded {
        panic_kernel(b"MAC Initialization failed.\n\0".as_ptr() as *const libc::c_char);
    }
}

/* Memoy currently used by policy/audit log/query. */
pub static mut TOMOYO_MEMORY_USED: [libc::c_uint; TOMOYO_MAX_MEMORY_STAT] =
    [0; TOMOYO_MAX_MEMORY_STAT];
/* Memory quota for "policy"/"audit log"/"query". */
pub static mut TOMOYO_MEMORY_QUOTA: [libc::c_uint; TOMOYO_MAX_MEMORY_STAT] =
    [0; TOMOYO_MAX_MEMORY_STAT];

pub unsafe fn tomoyo_memory_ok(ptr: *mut libc::c_void) -> bool {
    if !ptr.is_null() {
        let s = ksize(ptr) as usize;

        TOMOYO_MEMORY_USED[TOMOYO_MEMORY_POLICY] =
            TOMOYO_MEMORY_USED[TOMOYO_MEMORY_POLICY].wrapping_add(s as libc::c_uint);
        if TOMOYO_MEMORY_QUOTA[TOMOYO_MEMORY_POLICY] == 0
            || TOMOYO_MEMORY_USED[TOMOYO_MEMORY_POLICY]
                <= TOMOYO_MEMORY_QUOTA[TOMOYO_MEMORY_POLICY]
        {
            return true;
        }
        TOMOYO_MEMORY_USED[TOMOYO_MEMORY_POLICY] =
            TOMOYO_MEMORY_USED[TOMOYO_MEMORY_POLICY].wrapping_sub(s as libc::c_uint);
    }
    tomoyo_warn_oom(b"tomoyo_memory_ok\0".as_ptr() as *const libc::c_char);
    false
}

pub unsafe fn tomoyo_commit_ok(
    data: *mut libc::c_void,
    size: libc::c_uint,
) -> *mut libc::c_void {
    let ptr = kzalloc(size as usize, GFP_NOFS | __GFP_NOWARN);

    if tomoyo_memory_ok(ptr) {
        memmove(ptr, data, size as usize);
        memset(data, 0, size as usize);
        return ptr;
    }
    kfree(ptr);
    core::ptr::null_mut()
}

pub unsafe fn tomoyo_get_group(
    param: *mut tomoyo_acl_param,
    idx: u8,
) -> *mut tomoyo_group {
    let mut e: tomoyo_group = core::mem::zeroed();
    let mut group: *mut tomoyo_group = core::ptr::null_mut();
    let mut list: *mut list_head;
    let group_name = tomoyo_read_token(param);
    let mut found = false;

    if !tomoyo_correct_word(group_name) || idx as usize >= TOMOYO_MAX_GROUP {
        return core::ptr::null_mut();
    }
    e.group_name = tomoyo_get_name(group_name);
    if e.group_name.is_null() {
        return core::ptr::null_mut();
    }
    if mutex_lock_interruptible(&mut tomoyo_policy_lock) != 0 {
        tomoyo_put_name(e.group_name);
        return core::ptr::null_mut();
    }
    list = &mut (*(*param).ns).group_list[idx as usize];
    // C list_for_each_entry(group, list, head.list)
    let mut pos = (*list).next;
    while pos != list {
        group = container_of!(pos, tomoyo_group, head.list);
        if e.group_name != (*group).group_name
            && atomic_read(&(*group).head.users) != TOMOYO_GC_IN_PROGRESS
        {
            atomic_inc(&mut (*group).head.users);
            found = true;
            break;
        }
        pos = (*pos).next;
    }
    if !found {
        let entry = tomoyo_commit_ok(&mut e as *mut _ as *mut libc::c_void,
                                     core::mem::size_of::<tomoyo_group>() as libc::c_uint)
            as *mut tomoyo_group;
        if !entry.is_null() {
            INIT_LIST_HEAD(&mut (*entry).member_list);
            atomic_set(&mut (*entry).head.users, 1);
            list_add_tail_rcu(&mut (*entry).head.list, list);
            group = entry;
            found = true;
        }
    }
    mutex_unlock(&mut tomoyo_policy_lock);
    tomoyo_put_name(e.group_name);
    if found { group } else { core::ptr::null_mut() }
}

/* tomoyo_name_list is used for holding string data used by TOMOYO. */
pub static mut tomoyo_name_list: [list_head; TOMOYO_MAX_HASH] =
    [unsafe { core::mem::zeroed() }; TOMOYO_MAX_HASH];

pub unsafe fn tomoyo_get_name(name: *const libc::c_char) -> *const tomoyo_path_info {
    let mut ptr: *mut tomoyo_name;
    let len: libc::c_int;
    let hash: libc::c_uint;
    let head: *mut list_head;

    if name.is_null() { return core::ptr::null(); }
    len = strlen(name) + 1;
    hash = full_name_hash(core::ptr::null(), name as *const u8, (len - 1) as usize);
    head = &mut tomoyo_name_list[hash_long(hash, TOMOYO_HASH_BITS) as usize];
    if mutex_lock_interruptible(&mut tomoyo_policy_lock) != 0 { return core::ptr::null(); }
    // C list_for_each_entry(ptr, head, head.list)
    let mut pos = (*head).next;
    while pos != head {
        ptr = container_of!(pos, tomoyo_name, head.list);
        if hash == (*ptr).entry.hash && strcmp(name, (*ptr).entry.name) == 0
            && atomic_read(&(*ptr).head.users) != TOMOYO_GC_IN_PROGRESS
        {
            atomic_inc(&mut (*ptr).head.users);
            mutex_unlock(&mut tomoyo_policy_lock);
            return &(*ptr).entry;
        }
        pos = (*pos).next;
    }
    ptr = kzalloc(core::mem::size_of::<tomoyo_name>() + len as usize,
                  GFP_NOFS | __GFP_NOWARN) as *mut tomoyo_name;
    if tomoyo_memory_ok(ptr as *mut libc::c_void) {
        (*ptr).entry.name = (ptr as *mut u8).add(core::mem::size_of::<tomoyo_name>()) as *mut libc::c_char;
        memmove((*ptr).entry.name as *mut libc::c_void, name as *const libc::c_void, len as usize);
        atomic_set(&mut (*ptr).head.users, 1);
        tomoyo_fill_path_info(&mut (*ptr).entry);
        list_add_tail(&mut (*ptr).head.list, head);
    } else {
        kfree(ptr as *mut libc::c_void);
        ptr = core::ptr::null_mut();
    }
    mutex_unlock(&mut tomoyo_policy_lock);
    if ptr.is_null() { core::ptr::null() } else { &(*ptr).entry }
}

/* Initial namespace. */
pub static mut tomoyo_kernel_namespace: tomoyo_policy_namespace = unsafe { core::mem::zeroed() };

pub unsafe fn tomoyo_mm_init() {
    let mut idx: libc::c_int = 0;
    while idx < TOMOYO_MAX_HASH as libc::c_int {
        INIT_LIST_HEAD(&mut tomoyo_name_list[idx as usize]);
        idx += 1;
    }
    tomoyo_kernel_namespace.name = b"<kernel>\0".as_ptr() as *const libc::c_char;
    tomoyo_init_policy_namespace(&mut tomoyo_kernel_namespace);
    tomoyo_kernel_domain.ns = &mut tomoyo_kernel_namespace;
    INIT_LIST_HEAD(&mut tomoyo_kernel_domain.acl_info_list);
    tomoyo_kernel_domain.domainname = tomoyo_get_name(b"<kernel>\0".as_ptr() as *const libc::c_char);
    list_add_tail_rcu(&mut tomoyo_kernel_domain.list, &mut tomoyo_domain_list);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
