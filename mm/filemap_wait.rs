// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:1072-1790. Table alignment comes from the native type.
const PAGE_WAIT_TABLE_BITS: c_uint = 8;
const PAGE_WAIT_TABLE_SIZE: usize = 1 << PAGE_WAIT_TABLE_BITS;
#[link_section = ".data..cacheline_aligned"]
static mut FOLIO_WAIT_TABLE: rust_filemap_wait_table = unsafe { zeroed() };
static mut SYSCTL_PAGE_LOCK_UNFAIRNESS: c_int = 5;
static mut FILEMAP_SYSCTL_TABLE: [ctl_table; 1] = unsafe { zeroed() };
unsafe fn folio_waitqueue(folio: *mut folio) -> *mut wait_queue_head_t {
    addr_of_mut!(FOLIO_WAIT_TABLE.queues)
        .cast::<wait_queue_head_t>()
        .add(hash_ptr(folio.cast(), PAGE_WAIT_TABLE_BITS) as usize)
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_FILEMAP_INIT_COLD, cold)]
pub unsafe extern "C" fn pagecache_init() {
    for i in 0..PAGE_WAIT_TABLE_SIZE {
        init_waitqueue_head(
            addr_of_mut!(FOLIO_WAIT_TABLE.queues)
                .cast::<wait_queue_head_t>()
                .add(i),
        );
    }
    page_writeback_init();
    let table = addr_of_mut!(FILEMAP_SYSCTL_TABLE).cast::<ctl_table>();
    sysctl_set_procname(
        table,
        kernel::str::as_char_ptr_in_const_context(c"page_lock_unfairness"),
    );
    sysctl_set_data(table, addr_of_mut!(SYSCTL_PAGE_LOCK_UNFAIRNESS).cast());
    sysctl_set_maxlen(table, size_of::<c_int>() as c_int);
    sysctl_set_mode(table, 0o644);
    sysctl_set_proc_handler(table, Some(proc_dointvec_minmax));
    sysctl_set_extra1(table, rust_filemap_sysctl_zero());
    rust_filemap_register_sysctl_init(kernel::str::as_char_ptr_in_const_context(c"vm"), table, 1);
}
unsafe extern "C" fn wake_page_function(
    wait: *mut wait_queue_entry_t,
    mode: c_uint,
    _sync: c_int,
    arg: *mut c_void,
) -> c_int {
    let key = arg.cast::<wait_page_key>();
    let wait_page = rust_filemap_wait_page_from_wait(wait);
    if !wake_page_match(wait_page, key) {
        return 0;
    }
    let mut flags = (*wait).flags;
    if flags & RUST_FILEMAP_WQ_FLAG_EXCLUSIVE != 0 {
        if test_bit((*key).bit_nr as c_ulong, folio_flags_field((*key).folio)) {
            return -1;
        }
        if flags & RUST_FILEMAP_WQ_FLAG_CUSTOM != 0 {
            if test_and_set_bit((*key).bit_nr as c_ulong, folio_flags_field((*key).folio)) {
                return -1;
            }
            flags |= RUST_FILEMAP_WQ_FLAG_DONE;
        }
    }
    rust_filemap_wait_flags_store_release(wait, flags | RUST_FILEMAP_WQ_FLAG_WOKEN);
    wake_up_state((*wait).private.cast(), mode);
    // No dereference of wait after removal: its owner may immediately free it.
    list_del_init_careful(addr_of_mut!((*wait).entry));
    (flags & RUST_FILEMAP_WQ_FLAG_EXCLUSIVE != 0) as c_int
}
unsafe fn folio_wake_bit(folio: *mut folio, bit_nr: c_int) {
    let q = folio_waitqueue(folio);
    let mut key = wait_page_key {
        folio,
        bit_nr,
        page_match: 0,
    };
    let flags = spin_lock_irqsave(addr_of_mut!((*q).lock));
    __wake_up_locked_key(
        q,
        RUST_FILEMAP_TASK_NORMAL as c_uint,
        (&mut key as *mut wait_page_key).cast(),
    );
    if waitqueue_active(q) == 0 || key.page_match == 0 {
        folio_clear_waiters(folio);
    }
    spin_unlock_irqrestore(addr_of_mut!((*q).lock), flags);
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Behavior {
    Exclusive,
    Shared,
    Drop,
}
unsafe fn folio_trylock_flag(
    folio: *mut folio,
    bit_nr: c_int,
    wait: *mut wait_queue_entry,
) -> bool {
    if (*wait).flags & RUST_FILEMAP_WQ_FLAG_EXCLUSIVE != 0 {
        if test_and_set_bit(bit_nr as c_ulong, folio_flags_field(folio)) {
            return false;
        }
    } else if test_bit(bit_nr as c_ulong, folio_flags_field(folio)) {
        return false;
    }
    (*wait).flags |= RUST_FILEMAP_WQ_FLAG_WOKEN | RUST_FILEMAP_WQ_FLAG_DONE;
    true
}
unsafe fn folio_wait_bit_common(
    folio: *mut folio,
    bit_nr: c_int,
    state: c_int,
    behavior: Behavior,
) -> c_int {
    let q = folio_waitqueue(folio);
    let mut unfairness = SYSCTL_PAGE_LOCK_UNFAIRNESS;
    let mut wait_page = MaybeUninit::<wait_page_queue>::uninit();
    let wait_page = wait_page.as_mut_ptr();
    let wait = addr_of_mut!((*wait_page).wait);
    let mut thrashing = false;
    let mut pflags = MaybeUninit::<c_ulong>::uninit();
    let mut in_thrashing = MaybeUninit::<bool>::uninit();
    if bit_nr == PG_locked as c_int && !folio_test_uptodate(folio) && folio_test_workingset(folio) {
        delayacct_thrashing_start(in_thrashing.as_mut_ptr());
        psi_memstall_enter(pflags.as_mut_ptr());
        thrashing = true;
    }
    init_wait(wait);
    (*wait).func = Some(wake_page_function);
    (*wait_page).folio = folio;
    (*wait_page).bit_nr = bit_nr;
    'repeat: loop {
        (*wait).flags = 0;
        if behavior == Behavior::Exclusive {
            (*wait).flags = RUST_FILEMAP_WQ_FLAG_EXCLUSIVE;
            unfairness = unfairness.wrapping_sub(1);
            if unfairness < 0 {
                (*wait).flags |= RUST_FILEMAP_WQ_FLAG_CUSTOM;
            }
        }
        spin_lock_irq(addr_of_mut!((*q).lock));
        folio_set_waiters(folio);
        if !folio_trylock_flag(folio, bit_nr, wait) {
            __add_wait_queue_entry_tail(q, wait);
        }
        spin_unlock_irq(addr_of_mut!((*q).lock));
        if behavior == Behavior::Drop {
            folio_put(folio);
        }
        loop {
            set_current_state(state);
            let flags = rust_filemap_wait_flags_load_acquire(wait);
            if flags & RUST_FILEMAP_WQ_FLAG_WOKEN == 0 {
                if signal_pending_state(state as c_uint, current()) != 0 {
                    break;
                }
                io_schedule();
                continue;
            }
            if behavior != Behavior::Exclusive || flags & RUST_FILEMAP_WQ_FLAG_DONE != 0 {
                break;
            }
            if test_and_set_bit(bit_nr as c_ulong, folio_flags(folio, 0)) {
                continue 'repeat;
            }
            (*wait).flags |= RUST_FILEMAP_WQ_FLAG_DONE;
            break;
        }
        break;
    }
    finish_wait(q, wait);
    if thrashing {
        delayacct_thrashing_end(in_thrashing.as_mut_ptr());
        psi_memstall_leave(pflags.as_mut_ptr());
    }
    let mask = if behavior == Behavior::Exclusive {
        RUST_FILEMAP_WQ_FLAG_DONE
    } else {
        RUST_FILEMAP_WQ_FLAG_WOKEN
    };
    if (*wait).flags & mask != 0 {
        0
    } else {
        -(EINTR as c_int)
    }
}
#[cfg(CONFIG_MIGRATION)]
#[no_mangle]
pub unsafe extern "C" fn softleaf_entry_wait_on_locked(entry: softleaf_t, ptl: *mut spinlock_t) {
    let mut wait_page = MaybeUninit::<wait_page_queue>::uninit();
    let wait_page = wait_page.as_mut_ptr();
    let wait = addr_of_mut!((*wait_page).wait);
    let mut thrashing = false;
    let mut pflags = MaybeUninit::<c_ulong>::uninit();
    let mut in_thrashing = MaybeUninit::<bool>::uninit();
    let folio = softleaf_to_folio(entry);
    let q = folio_waitqueue(folio);
    if !folio_test_uptodate(folio) && folio_test_workingset(folio) {
        delayacct_thrashing_start(in_thrashing.as_mut_ptr());
        psi_memstall_enter(pflags.as_mut_ptr());
        thrashing = true;
    }
    init_wait(wait);
    (*wait).func = Some(wake_page_function);
    (*wait_page).folio = folio;
    (*wait_page).bit_nr = PG_locked as c_int;
    (*wait).flags = 0;
    spin_lock_irq(addr_of_mut!((*q).lock));
    folio_set_waiters(folio);
    if !folio_trylock_flag(folio, PG_locked as c_int, wait) {
        __add_wait_queue_entry_tail(q, wait);
    }
    spin_unlock_irq(addr_of_mut!((*q).lock));
    // Migration/device-private ownership keeps folio alive until this unlock.
    spin_unlock(ptl);
    loop {
        set_current_state(RUST_FILEMAP_TASK_UNINTERRUPTIBLE as c_int);
        let flags = rust_filemap_wait_flags_load_acquire(wait);
        if flags & RUST_FILEMAP_WQ_FLAG_WOKEN == 0 {
            if signal_pending_state(RUST_FILEMAP_TASK_UNINTERRUPTIBLE as c_uint, current()) != 0 {
                break;
            }
            io_schedule();
            continue;
        }
        break;
    }
    finish_wait(q, wait);
    if thrashing {
        delayacct_thrashing_end(in_thrashing.as_mut_ptr());
        psi_memstall_leave(pflags.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_wait_bit(folio: *mut folio, bit_nr: c_int) {
    folio_wait_bit_common(
        folio,
        bit_nr,
        RUST_FILEMAP_TASK_UNINTERRUPTIBLE as c_int,
        Behavior::Shared,
    );
}
#[no_mangle]
pub unsafe extern "C" fn folio_wait_bit_killable(folio: *mut folio, bit_nr: c_int) -> c_int {
    folio_wait_bit_common(
        folio,
        bit_nr,
        RUST_FILEMAP_TASK_KILLABLE as c_int,
        Behavior::Shared,
    )
}
unsafe fn folio_put_wait_locked(folio: *mut folio, state: c_int) -> c_int {
    folio_wait_bit_common(folio, PG_locked as c_int, state, Behavior::Drop)
}
#[no_mangle]
pub unsafe extern "C" fn folio_unlock(folio: *mut folio) {
    const {
        assert!(PG_waiters == 7);
        assert!(PG_locked <= 7);
    }
    vm_bug_folio!((!folio_test_locked(folio)) as c_int, folio);
    if folio_xor_flags_has_waiters(folio, 1 << PG_locked) {
        folio_wake_bit(folio, PG_locked as c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_end_read(folio: *mut folio, success: bool) {
    let mut mask: c_ulong = 1 << PG_locked;
    const {
        assert!(PG_uptodate <= 7);
    }
    vm_bug_folio!((!folio_test_locked(folio)) as c_int, folio);
    vm_bug_folio!((success && folio_test_uptodate(folio)) as c_int, folio);
    if success {
        mask |= 1 << PG_uptodate;
    }
    if folio_xor_flags_has_waiters(folio, mask) {
        folio_wake_bit(folio, PG_locked as c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_end_private_2(folio: *mut folio) {
    vm_bug_folio!((!folio_test_private_2(folio)) as c_int, folio);
    clear_bit_unlock(PG_private_2 as c_ulong, folio_flags(folio, 0));
    folio_wake_bit(folio, PG_private_2 as c_int);
    folio_put(folio);
}
#[no_mangle]
pub unsafe extern "C" fn folio_wait_private_2(folio: *mut folio) {
    while folio_test_private_2(folio) {
        folio_wait_bit(folio, PG_private_2 as c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_wait_private_2_killable(folio: *mut folio) -> c_int {
    let mut ret = 0;
    while folio_test_private_2(folio) {
        ret = folio_wait_bit_killable(folio, PG_private_2 as c_int);
        if ret < 0 {
            break;
        }
    }
    ret
}
unsafe fn filemap_end_dropbehind(folio: *mut folio) {
    let mapping = folio_mapping_field(folio);
    vm_bug_folio!((!folio_test_locked(folio)) as c_int, folio);
    if folio_test_writeback(folio) || folio_test_dirty(folio) {
        return;
    }
    if !folio_test_clear_dropbehind(folio) {
        return;
    }
    if !mapping.is_null() {
        folio_unmap_invalidate(mapping, folio, 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_end_dropbehind(folio: *mut folio) {
    if !folio_test_dropbehind(folio) {
        return;
    }
    if in_task() && folio_trylock(folio) {
        filemap_end_dropbehind(folio);
        folio_unlock(folio);
    }
}
#[no_mangle]
pub unsafe extern "C" fn folio_end_writeback_no_dropbehind(folio: *mut folio) {
    vm_bug_folio!((!folio_test_writeback(folio)) as c_int, folio);
    if folio_test_reclaim(folio) {
        folio_clear_reclaim(folio);
        folio_rotate_reclaimable(folio);
    }
    if __folio_end_writeback(folio) {
        folio_wake_bit(folio, PG_writeback as c_int);
    }
    acct_reclaim_writeback(folio);
}
#[no_mangle]
pub unsafe extern "C" fn folio_end_writeback(folio: *mut folio) {
    vm_bug_folio!((!folio_test_writeback(folio)) as c_int, folio);
    folio_get(folio);
    folio_end_writeback_no_dropbehind(folio);
    folio_end_dropbehind(folio);
    folio_put(folio);
}
#[no_mangle]
pub unsafe extern "C" fn __folio_lock(folio: *mut folio) {
    folio_wait_bit_common(
        folio,
        PG_locked as c_int,
        RUST_FILEMAP_TASK_UNINTERRUPTIBLE as c_int,
        Behavior::Exclusive,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __folio_lock_killable(folio: *mut folio) -> c_int {
    folio_wait_bit_common(
        folio,
        PG_locked as c_int,
        RUST_FILEMAP_TASK_KILLABLE as c_int,
        Behavior::Exclusive,
    )
}
unsafe fn __folio_lock_async(folio: *mut folio, wait: *mut wait_page_queue) -> c_int {
    let q = folio_waitqueue(folio);
    (*wait).folio = folio;
    (*wait).bit_nr = PG_locked as c_int;
    spin_lock_irq(addr_of_mut!((*q).lock));
    __add_wait_queue_entry_tail(q, addr_of_mut!((*wait).wait));
    folio_set_waiters(folio);
    let ret = if folio_trylock(folio) {
        __remove_wait_queue(q, addr_of_mut!((*wait).wait));
        0
    } else {
        -(EIOCBQUEUED as c_int)
    };
    spin_unlock_irq(addr_of_mut!((*q).lock));
    ret
}
#[no_mangle]
pub unsafe extern "C" fn __folio_lock_or_retry(
    folio: *mut folio,
    vmf: *mut vm_fault,
) -> vm_fault_t {
    let flags = (*vmf).flags;
    if fault_flag_allow_retry_first(flags) {
        if flags & RUST_FILEMAP_FAULT_FLAG_RETRY_NOWAIT != 0 {
            return RUST_FILEMAP_VM_FAULT_RETRY;
        }
        release_fault_lock(vmf);
        if flags & RUST_FILEMAP_FAULT_FLAG_KILLABLE != 0 {
            folio_wait_locked_killable(folio);
        } else {
            folio_wait_locked(folio);
        }
        return RUST_FILEMAP_VM_FAULT_RETRY;
    }
    if flags & RUST_FILEMAP_FAULT_FLAG_KILLABLE != 0 {
        if __folio_lock_killable(folio) != 0 {
            release_fault_lock(vmf);
            return RUST_FILEMAP_VM_FAULT_RETRY;
        }
    } else {
        __folio_lock(folio);
    }
    0
}
