// SPDX-License-Identifier: GPL-2.0
/* Deferred user space unwinding */

// Kernel dependencies are supplied by the surrounding translation.

const CAN_USE_IN_NMI: bool = cfg!(feature = "CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG");
const RESERVED_BITS: c_ulong = UNWIND_PENDING | UNWIND_USED;
const UNWIND_MAX_ENTRIES: usize = (SZ_4K - core::mem::size_of::<unwind_cache>()) / core::mem::size_of::<c_long>();

static mut CALLBACK_MUTEX: mutex = mutex::new();
static mut CALLBACKS: list_head = list_head::new();
static mut UNWIND_MASK: c_ulong = RESERVED_BITS;
static mut UNWIND_CTX_CTR: percpu<u32> = percpu::new();

#[inline]
unsafe fn try_assign_cnt(info: *mut unwind_task_info, cnt: u32) -> bool {
    if CAN_USE_IN_NMI {
        let mut old: u32 = 0;
        try_cmpxchg(&mut (*info).id.cnt, &mut old, cnt)
    } else {
        (*info).id.cnt = cnt;
        true
    }
}

#[inline]
unsafe fn unwind_pending(info: *mut unwind_task_info) -> bool {
    atomic_long_read(&(*info).unwind_mask) & UNWIND_PENDING != 0
}

unsafe fn get_cookie(info: *mut unwind_task_info) -> u64 {
    let mut cnt: u32 = 1;

    lockdep_assert_irqs_disabled();
    if (*info).id.cpu != 0 {
        return (*info).id.id;
    }

    cnt |= this_cpu_read(&UNWIND_CTX_CTR).wrapping_add(2);
    if try_assign_cnt(info, cnt) {
        this_cpu_write(&mut UNWIND_CTX_CTR, cnt);
    }
    (*info).id.cpu = smp_processor_id() + 1;
    (*info).id.id
}

pub unsafe fn unwind_user_faultable(trace: *mut unwind_stacktrace) -> c_int {
    let info = &mut (*current).unwind_info as *mut unwind_task_info;
    let mut cache: *mut unwind_cache;

    might_fault();
    if (*current).mm.is_null() {
        return -EINVAL;
    }
    if (*info).cache.is_null() {
        (*info).cache = kzalloc_flex::<unwind_cache>(UNWIND_MAX_ENTRIES);
        if (*info).cache.is_null() {
            return -ENOMEM;
        }
    }
    cache = (*info).cache;
    (*trace).entries = (*cache).entries;
    (*trace).nr = (*cache).nr_entries;
    if (*trace).nr != 0 {
        return 0;
    }
    unwind_user(trace, UNWIND_MAX_ENTRIES);
    (*cache).nr_entries = (*trace).nr;
    atomic_long_or(UNWIND_USED, &mut (*info).unwind_mask);
    0
}

unsafe fn process_unwind_deferred(task: *mut task_struct) {
    let info = &mut (*task).unwind_info as *mut unwind_task_info;
    let mut trace = unwind_stacktrace { nr: 0, entries: core::ptr::null_mut() };
    let mut bits: c_ulong;
    let cookie: u64;

    if WARN_ON_ONCE(!unwind_pending(info)) { return; }
    bits = atomic_long_fetch_andnot(UNWIND_PENDING, &mut (*info).unwind_mask);
    unwind_user_faultable(&mut trace);
    if !(*info).cache.is_null() { bits &= !(*info).cache.unwind_completed; }
    cookie = (*info).id.id;
    let _srcu = guard_srcu(&UNWIND_SRCU);
    list_for_each_entry_srcu(work, &CALLBACKS, list, srcu_read_lock_held(&UNWIND_SRCU)) {
        if test_bit(work.bit, &bits) {
            (work.func)(work, &trace, cookie);
            if !(*info).cache.is_null() { (*info).cache.unwind_completed |= BIT(work.bit); }
        }
    }
}

unsafe fn unwind_deferred_task_work(_head: *mut callback_head) { process_unwind_deferred(current); }

pub unsafe fn unwind_deferred_task_exit(task: *mut task_struct) {
    let info = &mut (*current).unwind_info as *mut unwind_task_info;
    if !unwind_pending(info) { return; }
    process_unwind_deferred(task);
    task_work_cancel(task, &mut (*info).work);
}

pub unsafe fn unwind_deferred_request(work: *mut unwind_work, cookie: *mut u64) -> c_int {
    let info = &mut (*current).unwind_info as *mut unwind_task_info;
    let mut twa_mode = TWA_RESUME;
    let mut old: c_ulong;
    let mut bits: c_ulong;
    let bit: c_ulong;
    *cookie = 0;
    if ((*current).flags & (PF_KTHREAD | PF_EXITING)) != 0 || !user_mode(task_pt_regs(current)) { return -EINVAL; }
    if in_nmi() { if WARN_ON_ONCE(!CAN_USE_IN_NMI) { return -EINVAL; } twa_mode = TWA_NMI_CURRENT; }
    let work_bit = READ_ONCE((*work).bit);
    if WARN_ON_ONCE(work_bit < 0) { return -EINVAL; }
    bit = BIT(work_bit);
    let _irq = guard_irqsave();
    *cookie = get_cookie(info);
    old = atomic_long_read(&(*info).unwind_mask);
    if old & bit != 0 { return 1; }
    bits = UNWIND_PENDING | bit;
    old = atomic_long_fetch_or(bits, &mut (*info).unwind_mask);
    if old & bits != 0 { WARN_ON_ONCE(old & UNWIND_PENDING == 0); return if old & bit != 0 { 1 } else { 0 }; }
    let ret = task_work_add(current, &mut (*info).work, twa_mode);
    if WARN_ON_ONCE(ret != 0) { atomic_long_set(&mut (*info).unwind_mask, 0); }
    ret
}

pub unsafe fn unwind_deferred_cancel(work: *mut unwind_work) {
    if work.is_null() { return; }
    let bit = (*work).bit;
    if WARN_ON_ONCE(BIT(bit) & RESERVED_BITS != 0) { return; }
    let _mutex = guard_mutex(&CALLBACK_MUTEX);
    list_del_rcu(&mut (*work).list);
    (*work).bit = -1;
    clear_bit(bit, &mut UNWIND_MASK);
    synchronize_srcu(&UNWIND_SRCU);
    let _rcu = guard_rcu();
    for_each_process_thread(g, t) {
        atomic_long_andnot(BIT(bit), &mut (*t).unwind_info.unwind_mask);
        if !(*t).unwind_info.cache.is_null() { clear_bit(bit, &mut (*t).unwind_info.cache.unwind_completed); }
    }
}

pub unsafe fn unwind_deferred_init(work: *mut unwind_work, func: unwind_callback_t) -> c_int {
    memset(work, 0, core::mem::size_of::<unwind_work>());
    let _mutex = guard_mutex(&CALLBACK_MUTEX);
    if UNWIND_MASK == !0 { return -EBUSY; }
    (*work).bit = ffz(UNWIND_MASK);
    set_bit((*work).bit, &mut UNWIND_MASK);
    list_add_rcu(&mut (*work).list, &mut CALLBACKS);
    (*work).func = func;
    0
}

pub unsafe fn unwind_task_init(task: *mut task_struct) {
    let info = &mut (*task).unwind_info;
    memset(info, 0, core::mem::size_of::<unwind_task_info>());
    init_task_work(&mut info.work, Some(unwind_deferred_task_work));
    atomic_long_set(&mut info.unwind_mask, 0);
}

pub unsafe fn unwind_task_free(task: *mut task_struct) {
    let info = &mut (*task).unwind_info;
    kfree(info.cache);
    task_work_cancel(task, &mut info.work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
