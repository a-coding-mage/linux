/* SPDX-License-Identifier: GPL-2.0 */

#[cfg(feature = "CONFIG_USER_RETURN_NOTIFIER")]
#[repr(C)]
pub struct UserReturnNotifier {
    pub on_user_return: Option<unsafe extern "C" fn(urn: *mut UserReturnNotifier)>,
    pub link: HListNode,
}

#[cfg(feature = "CONFIG_USER_RETURN_NOTIFIER")]
extern "C" {
    pub fn user_return_notifier_register(urn: *mut UserReturnNotifier);
    pub fn user_return_notifier_unregister(urn: *mut UserReturnNotifier);

    pub fn test_tsk_thread_flag(task: *mut TaskStruct, flag: i32) -> bool;
    pub fn clear_tsk_thread_flag(task: *mut TaskStruct, flag: i32);
    pub fn set_tsk_thread_flag(task: *mut TaskStruct, flag: i32);

    pub fn fire_user_return_notifiers();
}

#[cfg(feature = "CONFIG_USER_RETURN_NOTIFIER")]
#[inline]
pub unsafe fn propagate_user_return_notify(
    prev: *mut TaskStruct,
    next: *mut TaskStruct,
) {
    if test_tsk_thread_flag(prev, TIF_USER_RETURN_NOTIFY) {
        clear_tsk_thread_flag(prev, TIF_USER_RETURN_NOTIFY);
        set_tsk_thread_flag(next, TIF_USER_RETURN_NOTIFY);
    }
}

#[cfg(feature = "CONFIG_USER_RETURN_NOTIFIER")]
#[inline]
pub unsafe fn clear_user_return_notifier(p: *mut TaskStruct) {
    clear_tsk_thread_flag(p, TIF_USER_RETURN_NOTIFY);
}

#[cfg(not(feature = "CONFIG_USER_RETURN_NOTIFIER"))]
#[repr(C)]
pub struct UserReturnNotifier {}

#[cfg(not(feature = "CONFIG_USER_RETURN_NOTIFIER"))]
#[inline]
pub unsafe fn propagate_user_return_notify(
    _prev: *mut TaskStruct,
    _next: *mut TaskStruct,
) {
}

#[cfg(not(feature = "CONFIG_USER_RETURN_NOTIFIER"))]
#[inline]
pub fn fire_user_return_notifiers() {}

#[cfg(not(feature = "CONFIG_USER_RETURN_NOTIFIER"))]
#[inline]
pub unsafe fn clear_user_return_notifier(_p: *mut TaskStruct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
