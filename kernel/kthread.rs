// SPDX-License-Identifier: GPL-2.0-only
// Complete source translation of kernel/kthread.c (frozen baseline be59db3).
// All allocation, lifecycle, affinity and worker policy is owned by this file.
// Native types, constants, primitives and ABI adapters are declared by bindgen.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unused_mut,
    unused_unsafe,
    unreachable_pub
)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/kthread_generated.rs"
    ));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

include!("kthread_header_algorithms.rs");

static mut kthread_create_list: list_head = list_head {
    next: addr_of_mut!(kthread_create_list),
    prev: addr_of_mut!(kthread_create_list),
};
static mut kthread_affinity_list: list_head = list_head {
    next: addr_of_mut!(kthread_affinity_list),
    prev: addr_of_mut!(kthread_affinity_list),
};
#[no_mangle]
pub static mut kthreadd_task: *mut task_struct = null_mut();

#[inline]
unsafe fn current() -> *mut task_struct {
    lupos_kthread_current()
}
#[inline]
unsafe fn to_kthread(task: *mut task_struct) -> *mut kthread {
    lupos_kthread_warn((*task).flags & PF_KTHREAD == 0);
    (*task).worker_private.cast()
}
#[inline]
unsafe fn tsk_is_kthread(task: *mut task_struct) -> *mut kthread {
    if (*task).flags & PF_KTHREAD != 0 {
        (*task).worker_private.cast()
    } else {
        null_mut()
    }
}
#[inline]
fn error<T>(errno: c_int) -> *mut T {
    errno as isize as *mut T
}
#[inline]
fn is_err<T>(ptr: *const T) -> bool {
    ptr as usize >= (-(MAX_ERRNO as isize)) as usize
}
#[inline]
unsafe fn init_list(head: *mut list_head) {
    (*head).next = head;
    (*head).prev = head;
}

#[no_mangle]
pub unsafe extern "C" fn get_kthread_comm(buf: *mut c_char, size: usize, task: *mut task_struct) {
    let k = to_kthread(task);
    if k.is_null() || (*k).full_name.is_null() {
        lupos_kthread_strscpy(buf, addr_of!((*task).comm).cast(), size);
        return;
    }
    header_sized_strscpy_pad(buf, (*k).full_name, size);
}

#[no_mangle]
pub unsafe extern "C" fn set_kthread_struct(task: *mut task_struct) -> bool {
    if lupos_kthread_warn_struct_exists(!to_kthread(task).is_null()) {
        return false;
    }
    let k: *mut kthread = lupos_kthread_zalloc(size_of::<kthread>()).cast();
    if k.is_null() {
        return false;
    }
    lupos_kthread_init_completion(addr_of_mut!((*k).exited));
    lupos_kthread_init_completion(addr_of_mut!((*k).parked));
    init_list(addr_of_mut!((*k).affinity_node));
    (*task).vfork_done = addr_of_mut!((*k).exited);
    (*k).task = task;
    (*k).node = tsk_fork_get_node(current()) as c_uint;
    (*task).worker_private = k.cast();
    true
}

#[no_mangle]
pub unsafe extern "C" fn free_kthread_struct(task: *mut task_struct) {
    let k = to_kthread(task);
    if k.is_null() {
        return;
    }
    #[cfg(CONFIG_BLK_CGROUP)]
    lupos_kthread_warn_blkcg_attached(!(*k).blkcg_css.is_null());
    (*task).worker_private = null_mut();
    kfree((*k).full_name.cast());
    kfree(k.cast());
}

#[no_mangle]
pub unsafe extern "C" fn kthread_should_stop() -> bool {
    lupos_kthread_test_bit(
        KTHREAD_SHOULD_STOP,
        addr_of!((*to_kthread(current())).flags),
    )
}
unsafe fn __kthread_should_park(task: *mut task_struct) -> bool {
    lupos_kthread_test_bit(KTHREAD_SHOULD_PARK, addr_of!((*to_kthread(task)).flags))
}
#[no_mangle]
pub unsafe extern "C" fn kthread_should_park() -> bool {
    __kthread_should_park(current())
}
#[no_mangle]
pub unsafe extern "C" fn kthread_should_stop_or_park() -> bool {
    let k = tsk_is_kthread(current());
    !k.is_null() && ((*k).flags & ((1 << KTHREAD_SHOULD_STOP) | (1 << KTHREAD_SHOULD_PARK))) != 0
}
#[no_mangle]
pub unsafe extern "C" fn kthread_freezable_should_stop(was_frozen: *mut bool) -> bool {
    let mut frozen = false;
    lupos_kthread_might_sleep();
    if header_freezing(current()) {
        frozen = lupos_kthread_refrigerator(true);
    }
    if !was_frozen.is_null() {
        *was_frozen = frozen;
    }
    kthread_should_stop()
}
#[no_mangle]
pub unsafe extern "C" fn kthread_func(task: *mut task_struct) -> *mut c_void {
    let k = tsk_is_kthread(task);
    if k.is_null() {
        null_mut()
    } else {
        (*k).threadfn.map_or(null_mut(), |f| f as *mut c_void)
    }
}
#[no_mangle]
pub unsafe extern "C" fn kthread_data(task: *mut task_struct) -> *mut c_void {
    (*to_kthread(task)).data
}
#[no_mangle]
pub unsafe extern "C" fn kthread_probe_data(task: *mut task_struct) -> *mut c_void {
    let k = tsk_is_kthread(task);
    let mut data: *mut c_void = null_mut();
    if !k.is_null() {
        // worker_private may point outside any live allocation. Form its field
        // address with wrapping byte arithmetic, never an in-bounds projection.
        let source = k.cast::<u8>().wrapping_add(offset_of!(kthread, data));
        copy_from_kernel_nofault(
            addr_of_mut!(data).cast(),
            source.cast(),
            size_of::<*mut c_void>(),
        );
    }
    data
}

unsafe fn __kthread_parkme(k: *mut kthread) {
    loop {
        // The special-state primitive serializes pending wakeups via pi_lock.
        lupos_kthread_set_special_state(TASK_PARKED as _);
        if !lupos_kthread_test_bit(KTHREAD_SHOULD_PARK, addr_of!((*k).flags)) {
            break;
        }
        lupos_kthread_preempt_disable();
        complete(addr_of_mut!((*k).parked));
        schedule_preempt_disabled();
        lupos_kthread_preempt_enable();
    }
    lupos_kthread_set_state_relaxed(TASK_RUNNING as _);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_parkme() {
    __kthread_parkme(to_kthread(current()));
}
#[no_mangle]
pub unsafe extern "C" fn kthread_do_exit(k: *mut kthread, result: c_long) {
    (*k).result = result as c_int;
    if !lupos_kthread_list_empty(addr_of!((*k).affinity_node)) {
        lupos_kthread_affinity_lock();
        lupos_kthread_list_del(addr_of_mut!((*k).affinity_node));
        lupos_kthread_affinity_unlock();
        if !(*k).preferred_affinity.is_null() {
            kfree((*k).preferred_affinity.cast());
            (*k).preferred_affinity = null_mut();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn kthread_complete_and_exit(comp: *mut completion, code: c_long) -> ! {
    if !comp.is_null() {
        complete(comp);
    }
    do_exit(code)
}

unsafe fn kthread_fetch_affinity(k: *mut kthread, mask: *mut cpumask) {
    lupos_kthread_rcu_lock();
    let pref = if !(*k).preferred_affinity.is_null() {
        (*k).preferred_affinity.cast_const()
    } else if (*k).node == NUMA_NO_NODE as c_uint {
        lupos_kthread_housekeeping_mask()
    } else {
        lupos_kthread_node_mask((*k).node as c_int)
    };
    lupos_kthread_cpumask_and(mask, pref, lupos_kthread_housekeeping_mask());
    if lupos_kthread_cpumask_empty(mask) {
        lupos_kthread_cpumask_copy(mask, lupos_kthread_housekeeping_mask());
    }
    lupos_kthread_rcu_unlock();
}
unsafe fn kthread_affine_node() {
    let task = current();
    let k = to_kthread(task);
    let mut affinity: lupos_kthread_cpumask = zeroed();
    if lupos_kthread_warn_affine_per_cpu(kthread_is_per_cpu(task)) {
        return;
    }
    if !lupos_kthread_alloc_cpumask(addr_of_mut!(affinity)) {
        lupos_kthread_warn_affine_alloc(true);
        return;
    }
    let mask = lupos_kthread_cpumask_ptr(addr_of_mut!(affinity));
    lupos_kthread_affinity_lock();
    lupos_kthread_warn_affine_linked(!lupos_kthread_list_empty(addr_of!((*k).affinity_node)));
    lupos_kthread_list_add_tail(
        addr_of_mut!((*k).affinity_node),
        addr_of_mut!(kthread_affinity_list),
    );
    kthread_fetch_affinity(k, mask);
    set_cpus_allowed_ptr(task, mask);
    lupos_kthread_affinity_unlock();
    lupos_kthread_free_cpumask(addr_of_mut!(affinity));
}

// C callback adapter preserves kernel_thread's native function-pointer ABI/CFI.
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_thread(arg: *mut c_void) -> c_int {
    let create: *mut kthread_create_info = arg.cast();
    let threadfn = (*create).threadfn;
    let data = (*create).data;
    let task = current();
    let k = to_kthread(task);
    let done = lupos_kthread_xchg_done(addr_of_mut!((*create).done));
    if done.is_null() {
        kfree((*create).full_name.cast());
        kfree(create.cast());
        do_exit(-(EINTR as c_long));
    }
    (*k).full_name = (*create).full_name;
    (*k).threadfn = threadfn;
    (*k).data = data;
    let param = sched_param { sched_priority: 0 };
    sched_setscheduler_nocheck(task, SCHED_NORMAL as _, addr_of!(param));
    lupos_kthread_set_state_relaxed(TASK_UNINTERRUPTIBLE as _);
    (*create).result = task;
    lupos_kthread_preempt_disable();
    complete(done);
    schedule_preempt_disabled();
    lupos_kthread_preempt_enable();
    (*k).started = 1;
    if (*task).flags & PF_NO_SETAFFINITY == 0 && (*k).preferred_affinity.is_null() {
        kthread_affine_node();
    }
    let mut result = -(EINTR as c_int);
    if !lupos_kthread_test_bit(KTHREAD_SHOULD_STOP, addr_of!((*k).flags)) {
        lupos_kthread_cgroup_ready();
        __kthread_parkme(k);
        // The C API requires a valid callback; no success substitute for NULL.
        result = threadfn.unwrap_unchecked()(data);
    }
    do_exit(result as c_long)
}
#[no_mangle]
pub unsafe extern "C" fn tsk_fork_get_node(task: *mut task_struct) -> c_int {
    #[cfg(not(CONFIG_NUMA))]
    let _ = task;
    #[cfg(CONFIG_NUMA)]
    if task == kthreadd_task {
        return (*task).pref_node_fork as c_int;
    }
    NUMA_NO_NODE
}
unsafe fn create_kthread(create: *mut kthread_create_info) {
    #[cfg(CONFIG_NUMA)]
    {
        (*current()).pref_node_fork = (*create).node as _;
    }
    let pid = kernel_thread(
        Some(lupos_kthread_thread_callback),
        create.cast(),
        (*create).full_name,
        (CLONE_FS | CLONE_FILES | SIGCHLD) as _,
    );
    if pid < 0 {
        let done = lupos_kthread_xchg_done(addr_of_mut!((*create).done));
        kfree((*create).full_name.cast());
        if done.is_null() {
            kfree(create.cast());
            return;
        }
        (*create).result = error(pid);
        complete(done);
    }
}

// The varargs entry only supplies &va_list. Allocation and the killable handoff
// stay here, including ownership transfer when a fatal signal wins the xchg.
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_create_on_node_v(
    threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    data: *mut c_void,
    node: c_int,
    namefmt: *const c_char,
    args: *mut c_void,
) -> *mut task_struct {
    let mut done: completion = zeroed();
    lupos_kthread_init_completion(addr_of_mut!(done));
    let create: *mut kthread_create_info =
        lupos_kthread_alloc(size_of::<kthread_create_info>()).cast();
    if create.is_null() {
        return error(-(ENOMEM as c_int));
    }
    (*create).threadfn = threadfn;
    (*create).data = data;
    (*create).node = node;
    (*create).done = addr_of_mut!(done);
    (*create).full_name = lupos_kthread_vasprintf(namefmt, args);
    if (*create).full_name.is_null() {
        kfree(create.cast());
        return error(-(ENOMEM as c_int));
    }
    lupos_kthread_create_lock();
    lupos_kthread_list_add_tail(
        addr_of_mut!((*create).list),
        addr_of_mut!(kthread_create_list),
    );
    lupos_kthread_create_unlock();
    wake_up_process(kthreadd_task);
    if wait_for_completion_killable(addr_of_mut!(done)) != 0 {
        if !lupos_kthread_xchg_done(addr_of_mut!((*create).done)).is_null() {
            return error(-(EINTR as c_int));
        }
        wait_for_completion(addr_of_mut!(done));
    }
    let task = (*create).result;
    kfree(create.cast());
    task
}

unsafe fn __kthread_bind_mask(task: *mut task_struct, mask: *const cpumask, state: c_uint) {
    if wait_task_inactive(task, state) == 0 {
        lupos_kthread_warn(true);
        return;
    }
    let flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*task).pi_lock));
    set_cpus_allowed_force(task, mask);
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*task).pi_lock), flags);
    (*task).flags |= PF_NO_SETAFFINITY;
}
unsafe fn __kthread_bind(task: *mut task_struct, cpu: c_uint, state: c_uint) {
    __kthread_bind_mask(task, lupos_kthread_cpu_mask(cpu), state);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_bind_mask(task: *mut task_struct, mask: *const cpumask) {
    let k = to_kthread(task);
    __kthread_bind_mask(task, mask, TASK_UNINTERRUPTIBLE as _);
    lupos_kthread_warn_bind_mask_started((*k).started != 0);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_bind(task: *mut task_struct, cpu: c_uint) {
    let k = to_kthread(task);
    __kthread_bind(task, cpu, TASK_UNINTERRUPTIBLE as _);
    lupos_kthread_warn_bind_started((*k).started != 0);
}
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_create_on_cpu(
    threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    data: *mut c_void,
    cpu: c_uint,
    namefmt: *const c_char,
) -> *mut task_struct {
    let task = kthread_create_on_node(
        threadfn,
        data,
        lupos_kthread_cpu_to_node(cpu as c_int),
        namefmt,
        cpu,
    );
    if is_err(task) {
        return task;
    }
    kthread_bind(task, cpu);
    (*to_kthread(task)).cpu = cpu;
    task
}
#[no_mangle]
pub unsafe extern "C" fn kthread_set_per_cpu(task: *mut task_struct, cpu: c_int) {
    let k = to_kthread(task);
    if k.is_null() {
        return;
    }
    lupos_kthread_warn_per_cpu_unbound((*task).flags & PF_NO_SETAFFINITY == 0);
    if cpu < 0 {
        lupos_kthread_clear_bit(KTHREAD_IS_PER_CPU, addr_of_mut!((*k).flags));
        return;
    }
    (*k).cpu = cpu as c_uint;
    lupos_kthread_set_bit(KTHREAD_IS_PER_CPU, addr_of_mut!((*k).flags));
}
#[no_mangle]
pub unsafe extern "C" fn kthread_is_per_cpu(task: *mut task_struct) -> bool {
    let k = tsk_is_kthread(task);
    !k.is_null() && lupos_kthread_test_bit(KTHREAD_IS_PER_CPU, addr_of!((*k).flags))
}
#[no_mangle]
pub unsafe extern "C" fn kthread_unpark(task: *mut task_struct) {
    let k = to_kthread(task);
    if !lupos_kthread_test_bit(KTHREAD_SHOULD_PARK, addr_of!((*k).flags)) {
        return;
    }
    if lupos_kthread_test_bit(KTHREAD_IS_PER_CPU, addr_of!((*k).flags)) {
        __kthread_bind(task, (*k).cpu, TASK_PARKED as _);
    }
    lupos_kthread_clear_bit(KTHREAD_SHOULD_PARK, addr_of_mut!((*k).flags));
    wake_up_state(task, TASK_PARKED as _);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_park(task: *mut task_struct) -> c_int {
    let k = to_kthread(task);
    if lupos_kthread_warn((*task).flags & PF_EXITING != 0) {
        return -(ENOSYS as c_int);
    }
    if lupos_kthread_warn_already_parked(lupos_kthread_test_bit(
        KTHREAD_SHOULD_PARK,
        addr_of!((*k).flags),
    )) {
        return -(EBUSY as c_int);
    }
    lupos_kthread_set_bit(KTHREAD_SHOULD_PARK, addr_of_mut!((*k).flags));
    if task != current() {
        wake_up_process(task);
        wait_for_completion(addr_of_mut!((*k).parked));
        lupos_kthread_warn_park_inactive(wait_task_inactive(task, TASK_PARKED as _) == 0);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn kthread_stop(task: *mut task_struct) -> c_int {
    lupos_kthread_trace_stop(task);
    lupos_kthread_get_task(task);
    let k = to_kthread(task);
    lupos_kthread_set_bit(KTHREAD_SHOULD_STOP, addr_of_mut!((*k).flags));
    kthread_unpark(task);
    lupos_kthread_notify_signal(task);
    wake_up_process(task);
    wait_for_completion(addr_of_mut!((*k).exited));
    let result = (*k).result;
    header_put_task_struct(task);
    lupos_kthread_trace_stop_ret(result);
    result
}
#[no_mangle]
pub unsafe extern "C" fn kthread_stop_put(task: *mut task_struct) -> c_int {
    let result = kthread_stop(task);
    header_put_task_struct(task);
    result
}
#[no_mangle]
pub unsafe extern "C" fn kthreadd(_unused: *mut c_void) -> c_int {
    let task = current();
    lupos_kthread_set_comm(task, b"kthreadd\0".as_ptr().cast());
    lupos_kthread_ignore_signals(task);
    header_set_mems_allowed();
    (*task).flags |= PF_NOFREEZE;
    lupos_kthread_cgroup_init();
    kthread_affine_node();
    loop {
        lupos_kthread_set_state(TASK_INTERRUPTIBLE as _);
        if lupos_kthread_list_empty(addr_of!(kthread_create_list)) {
            schedule();
        }
        lupos_kthread_set_state_relaxed(TASK_RUNNING as _);
        lupos_kthread_create_lock();
        while !lupos_kthread_list_empty(addr_of!(kthread_create_list)) {
            let create = (*addr_of!(kthread_create_list))
                .next
                .cast::<u8>()
                .sub(offset_of!(kthread_create_info, list))
                .cast::<kthread_create_info>();
            lupos_kthread_list_del_init(addr_of_mut!((*create).list));
            lupos_kthread_create_unlock();
            create_kthread(create);
            lupos_kthread_create_lock();
        }
        lupos_kthread_create_unlock();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kthread_affine_preferred(
    task: *mut task_struct,
    preferred: *const cpumask,
) -> c_int {
    let k = to_kthread(task);
    if wait_task_inactive(task, TASK_UNINTERRUPTIBLE as _) == 0 || (*k).started != 0 {
        lupos_kthread_warn(true);
        return -(EINVAL as c_int);
    }
    lupos_kthread_warn_preferred_exists(!(*k).preferred_affinity.is_null());
    let mut affinity: lupos_kthread_cpumask = zeroed();
    if !lupos_kthread_alloc_cpumask(addr_of_mut!(affinity)) {
        return -(ENOMEM as c_int);
    }
    let mask = lupos_kthread_cpumask_ptr(addr_of_mut!(affinity));
    (*k).preferred_affinity = lupos_kthread_zalloc(size_of::<cpumask>()).cast();
    if (*k).preferred_affinity.is_null() {
        lupos_kthread_free_cpumask(addr_of_mut!(affinity));
        return -(ENOMEM as c_int);
    }
    lupos_kthread_affinity_lock();
    lupos_kthread_cpumask_copy((*k).preferred_affinity, preferred);
    lupos_kthread_warn_preferred_linked(!lupos_kthread_list_empty(addr_of!((*k).affinity_node)));
    lupos_kthread_list_add_tail(
        addr_of_mut!((*k).affinity_node),
        addr_of_mut!(kthread_affinity_list),
    );
    kthread_fetch_affinity(k, mask);
    let flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*task).pi_lock));
    set_cpus_allowed_force(task, mask);
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*task).pi_lock), flags);
    lupos_kthread_affinity_unlock();
    lupos_kthread_free_cpumask(addr_of_mut!(affinity));
    0
}
unsafe fn kthreads_update_affinity(force: bool) -> c_int {
    lupos_kthread_affinity_lock();
    if lupos_kthread_list_empty(addr_of!(kthread_affinity_list)) {
        lupos_kthread_affinity_unlock();
        return 0;
    }
    let mut affinity: lupos_kthread_cpumask = zeroed();
    if !lupos_kthread_alloc_cpumask(addr_of_mut!(affinity)) {
        lupos_kthread_affinity_unlock();
        return -(ENOMEM as c_int);
    }
    let mask = lupos_kthread_cpumask_ptr(addr_of_mut!(affinity));
    let mut result = 0;
    let mut pos = (*addr_of!(kthread_affinity_list)).next;
    while pos != addr_of_mut!(kthread_affinity_list) {
        let k = pos
            .cast::<u8>()
            .sub(offset_of!(kthread, affinity_node))
            .cast::<kthread>();
        pos = (*pos).next;
        if lupos_kthread_warn_update_bound(
            (*(*k).task).flags & PF_NO_SETAFFINITY != 0 || kthread_is_per_cpu((*k).task),
        ) {
            result = -(EINVAL as c_int);
            continue;
        }
        if force || !(*k).preferred_affinity.is_null() || (*k).node != NUMA_NO_NODE as c_uint {
            kthread_fetch_affinity(k, mask);
            set_cpus_allowed_ptr((*k).task, mask);
        }
    }
    lupos_kthread_free_cpumask(addr_of_mut!(affinity));
    lupos_kthread_affinity_unlock();
    result
}
#[no_mangle]
pub unsafe extern "C" fn kthreads_update_housekeeping() -> c_int {
    kthreads_update_affinity(true)
}
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_online_cpu(_cpu: c_uint) -> c_int {
    kthreads_update_affinity(false)
}
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_init() -> c_int {
    lupos_kthread_cpuhp_setup(b"kthreads:online\0".as_ptr().cast())
}

#[no_mangle]
pub unsafe extern "C" fn __kthread_init_worker(
    worker: *mut kthread_worker,
    name: *const c_char,
    key: *mut lock_class_key,
) {
    core::ptr::write_bytes(worker, 0, 1);
    lupos_kthread_raw_lock_init(addr_of_mut!((*worker).lock));
    lupos_kthread_lockdep_class(addr_of_mut!((*worker).lock), key, name);
    init_list(addr_of_mut!((*worker).work_list));
    init_list(addr_of_mut!((*worker).delayed_work_list));
}
#[no_mangle]
pub unsafe extern "C" fn kthread_worker_fn(arg: *mut c_void) -> c_int {
    let worker: *mut kthread_worker = arg.cast();
    let task = current();
    lupos_kthread_warn(!(*worker).task.is_null() && (*worker).task != task);
    (*worker).task = task;
    if (*worker).flags & KTW_FREEZABLE != 0 {
        lupos_kthread_set_freezable();
    }
    loop {
        // Full state-store barrier pairs with kthread_stop's wakeup.
        lupos_kthread_set_state(TASK_INTERRUPTIBLE as _);
        if kthread_should_stop() {
            lupos_kthread_set_state_relaxed(TASK_RUNNING as _);
            lupos_kthread_raw_lock_irq(addr_of_mut!((*worker).lock));
            (*worker).task = null_mut();
            lupos_kthread_raw_unlock_irq(addr_of_mut!((*worker).lock));
            return 0;
        }
        let mut work: *mut kthread_work = null_mut();
        lupos_kthread_raw_lock_irq(addr_of_mut!((*worker).lock));
        if !lupos_kthread_list_empty(addr_of!((*worker).work_list)) {
            work = (*worker)
                .work_list
                .next
                .cast::<u8>()
                .sub(offset_of!(kthread_work, node))
                .cast();
            lupos_kthread_list_del_init(addr_of_mut!((*work).node));
        }
        (*worker).current_work = work;
        lupos_kthread_raw_unlock_irq(addr_of_mut!((*worker).lock));
        if !work.is_null() {
            let func = (*work).func;
            lupos_kthread_set_state_relaxed(TASK_RUNNING as _);
            lupos_kthread_trace_execute_start(work);
            (*work).func.unwrap_unchecked()(work);
            // Callback may free work. Only its address and saved func survive.
            lupos_kthread_trace_execute_end(work, func);
        } else if !header_freezing(task) {
            schedule();
        } else {
            lupos_kthread_set_state_relaxed(TASK_RUNNING as _);
        }
        header_try_to_freeze();
        lupos_kthread_cond_resched();
    }
}

#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_create_worker_on_node_v(
    flags: c_uint,
    node: c_int,
    namefmt: *const c_char,
    args: *mut c_void,
) -> *mut kthread_worker {
    let worker: *mut kthread_worker = lupos_kthread_zalloc(size_of::<kthread_worker>()).cast();
    if worker.is_null() {
        return error(-(ENOMEM as c_int));
    }
    // Native key/name adapter calls the Rust __kthread_init_worker body.
    lupos_kthread_init_worker_key(worker);
    let task =
        lupos_kthread_create_on_node_v(Some(kthread_worker_fn), worker.cast(), node, namefmt, args);
    if is_err(task) {
        kfree(worker.cast());
        return task.cast();
    }
    (*worker).flags = flags;
    (*worker).task = task;
    worker
}
#[no_mangle]
pub unsafe extern "C" fn kthread_create_worker_on_cpu(
    cpu: c_int,
    flags: c_uint,
    namefmt: *const c_char,
) -> *mut kthread_worker {
    let worker = kthread_create_worker_on_node(flags, lupos_kthread_cpu_to_node(cpu), namefmt, cpu);
    if !is_err(worker) {
        kthread_bind((*worker).task, cpu as c_uint);
    }
    worker
}

unsafe fn queuing_blocked(worker: *mut kthread_worker, work: *mut kthread_work) -> bool {
    lupos_kthread_assert_locked(addr_of_mut!((*worker).lock));
    !lupos_kthread_list_empty(addr_of!((*work).node)) || (*work).canceling != 0
}
unsafe fn kthread_insert_work_sanity_check(worker: *mut kthread_worker, work: *mut kthread_work) {
    lupos_kthread_assert_locked(addr_of_mut!((*worker).lock));
    lupos_kthread_warn_insert_pending(!lupos_kthread_list_empty(addr_of!((*work).node)));
    lupos_kthread_warn_insert_worker(!(*work).worker.is_null() && (*work).worker != worker);
}
unsafe fn kthread_insert_work(
    worker: *mut kthread_worker,
    work: *mut kthread_work,
    pos: *mut list_head,
) {
    kthread_insert_work_sanity_check(worker, work);
    lupos_kthread_trace_queue_work(worker, work);
    lupos_kthread_list_add_tail(addr_of_mut!((*work).node), pos);
    (*work).worker = worker;
    if (*worker).current_work.is_null() && !(*worker).task.is_null() {
        wake_up_process((*worker).task);
    }
}
#[no_mangle]
pub unsafe extern "C" fn kthread_queue_work(
    worker: *mut kthread_worker,
    work: *mut kthread_work,
) -> bool {
    let flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    let mut result = false;
    if !queuing_blocked(worker, work) {
        kthread_insert_work(worker, work, addr_of_mut!((*worker).work_list));
        result = true;
    }
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
    result
}

// Native public timer entry provides the callback identity; policy is Rust.
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_delayed_work_timer(t: *mut timer_list) {
    let delayed = t
        .cast::<u8>()
        .sub(offset_of!(kthread_delayed_work, timer))
        .cast::<kthread_delayed_work>();
    let work = addr_of_mut!((*delayed).work);
    let worker = (*work).worker;
    if lupos_kthread_warn_timer_no_worker(worker.is_null()) {
        return;
    }
    let flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    lupos_kthread_warn_timer_worker((*work).worker != worker);
    lupos_kthread_warn_timer_unlisted(lupos_kthread_list_empty(addr_of!((*work).node)));
    lupos_kthread_list_del_init(addr_of_mut!((*work).node));
    if (*work).canceling == 0 {
        kthread_insert_work(worker, work, addr_of_mut!((*worker).work_list));
    }
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
}
unsafe fn __kthread_queue_delayed_work(
    worker: *mut kthread_worker,
    delayed: *mut kthread_delayed_work,
    delay: c_ulong,
) {
    let timer = addr_of_mut!((*delayed).timer);
    let work = addr_of_mut!((*delayed).work);
    lupos_kthread_warn_timer_callback(!lupos_kthread_timer_callback_matches(timer));
    if delay == 0 {
        kthread_insert_work(worker, work, addr_of_mut!((*worker).work_list));
        return;
    }
    kthread_insert_work_sanity_check(worker, work);
    lupos_kthread_list_add(
        addr_of_mut!((*work).node),
        addr_of_mut!((*worker).delayed_work_list),
    );
    (*work).worker = worker;
    (*timer).expires = lupos_kthread_jiffies().wrapping_add(delay);
    add_timer(timer);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_queue_delayed_work(
    worker: *mut kthread_worker,
    delayed: *mut kthread_delayed_work,
    delay: c_ulong,
) -> bool {
    let work = addr_of_mut!((*delayed).work);
    let flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    let mut result = false;
    if !queuing_blocked(worker, work) {
        __kthread_queue_delayed_work(worker, delayed, delay);
        result = true;
    }
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
    result
}
#[no_mangle]
pub unsafe extern "C" fn lupos_kthread_flush_work_fn(work: *mut kthread_work) {
    let fwork = work
        .cast::<u8>()
        .sub(offset_of!(lupos_kthread_flush_work, work))
        .cast::<lupos_kthread_flush_work>();
    complete(addr_of_mut!((*fwork).done));
}
unsafe fn init_flush(fwork: *mut lupos_kthread_flush_work) {
    // Zero the ordinary fields and initialize the self-link only after the
    // on-stack object is at its final address; moving it would break the list.
    core::ptr::write_bytes(fwork, 0, 1);
    init_list(addr_of_mut!((*fwork).work.node));
    (*fwork).work.func = Some(lupos_kthread_flush_callback);
    lupos_kthread_init_completion(addr_of_mut!((*fwork).done));
}
#[no_mangle]
pub unsafe extern "C" fn kthread_flush_work(work: *mut kthread_work) {
    let mut fwork: lupos_kthread_flush_work = zeroed();
    init_flush(addr_of_mut!(fwork));
    let worker = (*work).worker;
    if worker.is_null() {
        return;
    }
    let mut noop = false;
    lupos_kthread_raw_lock_irq(addr_of_mut!((*worker).lock));
    lupos_kthread_warn_flush_worker((*work).worker != worker);
    if !lupos_kthread_list_empty(addr_of!((*work).node)) {
        kthread_insert_work(worker, addr_of_mut!(fwork.work), (*work).node.next);
    } else if (*worker).current_work == work {
        kthread_insert_work(worker, addr_of_mut!(fwork.work), (*worker).work_list.next);
    } else {
        noop = true;
    }
    lupos_kthread_raw_unlock_irq(addr_of_mut!((*worker).lock));
    if !noop {
        wait_for_completion(addr_of_mut!(fwork.done));
    }
}
unsafe fn kthread_cancel_delayed_work_timer(work: *mut kthread_work, flags: *mut c_ulong) {
    let delayed = work
        .cast::<u8>()
        .sub(offset_of!(kthread_delayed_work, work))
        .cast::<kthread_delayed_work>();
    let worker = (*work).worker;
    // Timer cancellation must drop the lock, while canceling prevents requeue.
    (*work).canceling += 1;
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), *flags);
    timer_delete_sync(addr_of_mut!((*delayed).timer));
    *flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    (*work).canceling -= 1;
}
unsafe fn __kthread_cancel_work(work: *mut kthread_work) -> bool {
    if !lupos_kthread_list_empty(addr_of!((*work).node)) {
        lupos_kthread_list_del_init(addr_of_mut!((*work).node));
        true
    } else {
        false
    }
}
#[no_mangle]
pub unsafe extern "C" fn kthread_mod_delayed_work(
    worker: *mut kthread_worker,
    delayed: *mut kthread_delayed_work,
    delay: c_ulong,
) -> bool {
    let work = addr_of_mut!((*delayed).work);
    let mut flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    let result;
    if (*work).worker.is_null() {
        result = false;
    } else {
        lupos_kthread_warn_mod_worker((*work).worker != worker);
        kthread_cancel_delayed_work_timer(work, addr_of_mut!(flags));
        if (*work).canceling != 0 {
            // A concurrent modifier/canceller wins. Keep its queue ownership,
            // and report true so the caller's reference count is unchanged.
            lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
            return true;
        }
        result = __kthread_cancel_work(work);
    }
    __kthread_queue_delayed_work(worker, delayed, delay);
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
    result
}
unsafe fn __kthread_cancel_work_sync(work: *mut kthread_work, is_delayed: bool) -> bool {
    let worker = (*work).worker;
    if worker.is_null() {
        return false;
    }
    let mut flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
    lupos_kthread_warn_cancel_worker((*work).worker != worker);
    if is_delayed {
        kthread_cancel_delayed_work_timer(work, addr_of_mut!(flags));
    }
    let result = __kthread_cancel_work(work);
    if (*worker).current_work == work {
        (*work).canceling += 1;
        lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
        kthread_flush_work(work);
        flags = lupos_kthread_raw_lock_irqsave(addr_of_mut!((*worker).lock));
        (*work).canceling -= 1;
    }
    lupos_kthread_raw_unlock_irqrestore(addr_of_mut!((*worker).lock), flags);
    result
}
#[no_mangle]
pub unsafe extern "C" fn kthread_cancel_work_sync(work: *mut kthread_work) -> bool {
    __kthread_cancel_work_sync(work, false)
}
#[no_mangle]
pub unsafe extern "C" fn kthread_cancel_delayed_work_sync(
    delayed: *mut kthread_delayed_work,
) -> bool {
    __kthread_cancel_work_sync(addr_of_mut!((*delayed).work), true)
}
#[no_mangle]
pub unsafe extern "C" fn kthread_flush_worker(worker: *mut kthread_worker) {
    let mut fwork: lupos_kthread_flush_work = zeroed();
    init_flush(addr_of_mut!(fwork));
    kthread_queue_work(worker, addr_of_mut!(fwork.work));
    wait_for_completion(addr_of_mut!(fwork.done));
}
#[no_mangle]
pub unsafe extern "C" fn kthread_destroy_worker(worker: *mut kthread_worker) {
    let task = (*worker).task;
    if lupos_kthread_warn(task.is_null()) {
        return;
    }
    kthread_flush_worker(worker);
    kthread_stop(task);
    lupos_kthread_warn(!lupos_kthread_list_empty(addr_of!(
        (*worker).delayed_work_list
    )));
    lupos_kthread_warn(!lupos_kthread_list_empty(addr_of!((*worker).work_list)));
    kfree(worker.cast());
}

#[no_mangle]
pub unsafe extern "C" fn kthread_use_mm(mm: *mut mm_struct) {
    let task = current();
    lupos_kthread_warn_use_not_kthread((*task).flags & PF_KTHREAD == 0);
    lupos_kthread_warn_use_has_mm(!(*task).mm.is_null());
    // mmgrab and the lazy-TLB reference are deliberately distinct, even when
    // mm == active_mm. The final drop supplies the membarrier ordering.
    lupos_kthread_mmgrab(mm);
    lupos_kthread_task_lock(task);
    lupos_kthread_irq_disable();
    let active_mm = (*task).active_mm;
    (*task).active_mm = mm;
    (*task).mm = mm;
    lupos_kthread_membarrier_update(mm);
    lupos_kthread_switch_mm(active_mm, mm, task);
    lupos_kthread_irq_enable();
    lupos_kthread_task_unlock(task);
    lupos_kthread_finish_arch_post_lock_switch();
    header_mmdrop_lazy_tlb(active_mm);
}
#[no_mangle]
pub unsafe extern "C" fn kthread_unuse_mm(mm: *mut mm_struct) {
    let task = current();
    lupos_kthread_warn_unuse_not_kthread((*task).flags & PF_KTHREAD == 0);
    lupos_kthread_warn_unuse_no_mm((*task).mm.is_null());
    lupos_kthread_task_lock(task);
    lupos_kthread_mb_after_spinlock();
    lupos_kthread_irq_disable();
    (*task).mm = null_mut();
    lupos_kthread_membarrier_update(null_mut());
    header_mmgrab_lazy_tlb(mm);
    lupos_kthread_enter_lazy_tlb(mm, task);
    lupos_kthread_irq_enable();
    lupos_kthread_task_unlock(task);
    header_mmdrop(mm);
}
#[cfg(CONFIG_BLK_CGROUP)]
#[no_mangle]
pub unsafe extern "C" fn kthread_associate_blkcg(css: *mut cgroup_subsys_state) {
    let task = current();
    if (*task).flags & PF_KTHREAD == 0 {
        return;
    }
    let k = to_kthread(task);
    if k.is_null() {
        return;
    }
    if !(*k).blkcg_css.is_null() {
        header_css_put((*k).blkcg_css);
        (*k).blkcg_css = null_mut();
    }
    if !css.is_null() {
        header_css_get(css);
        (*k).blkcg_css = css;
    }
}
#[cfg(CONFIG_BLK_CGROUP)]
#[no_mangle]
pub unsafe extern "C" fn kthread_blkcg() -> *mut cgroup_subsys_state {
    let task = current();
    if (*task).flags & PF_KTHREAD != 0 {
        let k = to_kthread(task);
        if !k.is_null() {
            return (*k).blkcg_css;
        }
    }
    null_mut()
}

include!("kthread_layout.rs");
