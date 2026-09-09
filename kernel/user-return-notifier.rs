// SPDX-License-Identifier: GPL-2.0-only

// Kernel declarations supplied by the corresponding Linux headers.
#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_return_notifier {
    pub link: hlist_node,
    pub on_user_return: unsafe extern "C" fn(*mut user_return_notifier),
}

extern "C" {
    static mut current: *mut core::ffi::c_void;

    fn set_tsk_thread_flag(task: *mut core::ffi::c_void, flag: i32);
    fn clear_tsk_thread_flag(task: *mut core::ffi::c_void, flag: i32);
    fn hlist_add_head(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del(node: *mut hlist_node);
    fn hlist_empty(head: *mut hlist_head) -> bool;
    fn this_cpu_ptr(head: *mut hlist_head) -> *mut hlist_head;
    fn get_cpu_var(head: *mut hlist_head) -> *mut hlist_head;
    fn put_cpu_var(head: *mut hlist_head);
    fn hlist_for_each_entry_safe(
        notifier: *mut user_return_notifier,
        tmp: *mut hlist_node,
        head: *mut hlist_head,
        member: *mut hlist_node,
    );
}

// TIF_USER_RETURN_NOTIFY is supplied by linux/sched.h.
extern "C" {
    static TIF_USER_RETURN_NOTIFY: i32;
}

static mut return_notifier_list: hlist_head = hlist_head { _private: [] };

/*
 * Request a notification when the current cpu returns to userspace.  Must be
 * called in atomic context.  The notifier will also be called in atomic
 * context.
 */
#[no_mangle]
pub unsafe extern "C" fn user_return_notifier_register(urn: *mut user_return_notifier) {
    set_tsk_thread_flag(current, TIF_USER_RETURN_NOTIFY);
    hlist_add_head(
        core::ptr::addr_of_mut!((*urn).link),
        this_cpu_ptr(core::ptr::addr_of_mut!(return_notifier_list)),
    );
}

/*
 * Removes a registered user return notifier.  Must be called from atomic
 * context, and from the same cpu registration occurred in.
 */
#[no_mangle]
pub unsafe extern "C" fn user_return_notifier_unregister(urn: *mut user_return_notifier) {
    hlist_del(core::ptr::addr_of_mut!((*urn).link));
    let head = this_cpu_ptr(core::ptr::addr_of_mut!(return_notifier_list));
    if hlist_empty(head) {
        clear_tsk_thread_flag(current, TIF_USER_RETURN_NOTIFY);
    }
}

/* Calls registered user return notifiers */
#[no_mangle]
pub unsafe extern "C" fn fire_user_return_notifiers() {
    let mut urn: *mut user_return_notifier = core::ptr::null_mut();
    let mut tmp2: *mut hlist_node = core::ptr::null_mut();
    let head = get_cpu_var(core::ptr::addr_of_mut!(return_notifier_list));

    // hlist_for_each_entry_safe(urn, tmp2, head, link)
    hlist_for_each_entry_safe(
        urn,
        tmp2,
        head,
        core::ptr::addr_of_mut!((*urn).link),
    );
    if !urn.is_null() {
        ((*urn).on_user_return)(urn);
    }
    put_cpu_var(core::ptr::addr_of_mut!(return_notifier_list));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
