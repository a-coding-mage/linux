/* SPDX-License-Identifier: GPL-2.0 */

/*
 * task->stack (kernel stack) handling interfaces.
 *
 * C dependencies supplied by other headers are intentionally not implemented
 * here: task_struct, THREAD_SIZE, STACK_END_MAGIC, current, kasan_reset_tag,
 * and refcount_inc_not_zero are external dependencies.
 */

#[cfg(CONFIG_THREAD_INFO_IN_TASK)]
#[inline(always)]
pub unsafe fn task_stack_page(task: *const task_struct) -> *mut core::ffi::c_void {
    (*task).stack
}

#[cfg(CONFIG_THREAD_INFO_IN_TASK)]
#[inline(always)]
pub unsafe fn setup_thread_stack(_new: *mut task_struct, _old: *mut task_struct) {}

#[cfg(all(CONFIG_THREAD_INFO_IN_TASK, CONFIG_STACK_GROWSUP))]
#[inline(always)]
pub unsafe fn end_of_stack(task: *const task_struct) -> *mut usize {
    ((((*task).stack as usize).wrapping_add(THREAD_SIZE)) as *mut usize).wrapping_sub(1)
}

#[cfg(all(CONFIG_THREAD_INFO_IN_TASK, not(CONFIG_STACK_GROWSUP)))]
#[inline(always)]
pub unsafe fn end_of_stack(task: *const task_struct) -> *mut usize {
    (*task).stack as *mut usize
}

#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
#[inline(always)]
pub unsafe fn task_stack_page(task: *const task_struct) -> *mut core::ffi::c_void {
    (*task).stack as *mut core::ffi::c_void
}

#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
#[inline]
pub unsafe fn setup_thread_stack(p: *mut task_struct, org: *mut task_struct) {
    *task_thread_info(p) = *task_thread_info(org);
    (*task_thread_info(p)).task = p;
}

#[cfg(all(not(CONFIG_THREAD_INFO_IN_TASK), CONFIG_STACK_GROWSUP))]
#[inline]
pub unsafe fn end_of_stack(p: *const task_struct) -> *mut usize {
    (((task_thread_info(p) as usize).wrapping_add(THREAD_SIZE)) as *mut usize).wrapping_sub(1)
}

#[cfg(all(not(CONFIG_THREAD_INFO_IN_TASK), not(CONFIG_STACK_GROWSUP)))]
#[inline]
pub unsafe fn end_of_stack(p: *const task_struct) -> *mut usize {
    (task_thread_info(p).wrapping_add(1)) as *mut usize
}

#[cfg(CONFIG_THREAD_INFO_IN_TASK)]
#[inline]
pub unsafe fn try_get_task_stack(tsk: *mut task_struct) -> *mut core::ffi::c_void {
    if refcount_inc_not_zero(&mut (*tsk).stack_refcount) {
        task_stack_page(tsk)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(CONFIG_THREAD_INFO_IN_TASK)]
extern "C" {
    pub fn put_task_stack(tsk: *mut task_struct);
}

#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
#[inline]
pub unsafe fn try_get_task_stack(tsk: *mut task_struct) -> *mut core::ffi::c_void {
    task_stack_page(tsk)
}

#[cfg(not(CONFIG_THREAD_INFO_IN_TASK))]
#[inline]
pub unsafe fn put_task_stack(_tsk: *mut task_struct) {}

extern "C" {
    pub fn exit_task_stack_account(tsk: *mut task_struct);
}

#[inline]
pub unsafe fn task_stack_end_corrupted(task: *const task_struct) -> bool {
    *end_of_stack(task) != STACK_END_MAGIC
}

#[inline]
pub unsafe fn object_is_on_stack(obj: *const core::ffi::c_void) -> core::ffi::c_int {
    let stack = task_stack_page(current);
    let obj = kasan_reset_tag(obj);
    ((obj as usize >= stack as usize)
        && (obj as usize < (stack as usize).wrapping_add(THREAD_SIZE))) as core::ffi::c_int
}

extern "C" {
    pub fn thread_stack_cache_init();
}

#[cfg(CONFIG_DEBUG_STACK_USAGE)]
extern "C" {
    pub fn stack_not_used(p: *mut task_struct) -> usize;
}

#[cfg(not(CONFIG_DEBUG_STACK_USAGE))]
#[inline]
pub unsafe fn stack_not_used(_p: *mut task_struct) -> usize {
    0
}

extern "C" {
    pub fn set_task_stack_end_magic(tsk: *mut task_struct);
}

#[inline]
pub unsafe fn kstack_end(addr: *mut core::ffi::c_void) -> core::ffi::c_int {
    (!((addr as usize)
        .wrapping_add(core::mem::size_of::<*mut core::ffi::c_void>())
        .wrapping_sub(1)
        & (THREAD_SIZE - core::mem::size_of::<*mut core::ffi::c_void>()))) as core::ffi::c_int
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
