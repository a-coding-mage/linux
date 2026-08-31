// SPDX-License-Identifier: GPL-2.0
/*
 * Regression1
 * Description:
 * Salman Qazi describes the following radix-tree bug:
 *
 * In the following case, we get can get a deadlock:
 *
 * 0.  The radix tree contains two items, one has the index 0.
 * 1.  The reader (in this case find_get_pages) takes the rcu_read_lock.
 * 2.  The reader acquires slot(s) for item(s) including the index 0 item.
 * 3.  The non-zero index item is deleted, and as a consequence the other item
 *     is moved to the root of the tree. The place where it used to be is queued
 *     for deletion after the readers finish.
 * 3b. The zero item is deleted, removing it from the direct slot, it remains in
 *     the rcu-delayed indirect node.
 * 4.  The reader looks at the index 0 slot, and finds that the page has 0 ref
 *     count
 * 5.  The reader looks at it again, hoping that the item will either be freed
 *     or the ref count will increase. This never happens, as the slot it is
 *     looking at will never be updated. Also, this slot can never be reclaimed
 *     because the reader is holding rcu_read_lock and is in an infinite loop.
 *
 * The fix is to re-use the same "indirect" pointer case that requires a slot
 * lookup retry into a general "retry the lookup" bit.
 *
 * Running:
 * This test should run to completion in a few seconds. The above bug would
 * cause it to hang indefinitely.
 *
 * Upstream commit:
 * Not yet
 */

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

const GFP_KERNEL: c_uint = 0;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [usize; 0],
}

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [usize; 0],
}

pub type pthread_t = c_ulong;

#[repr(C)]
pub struct rcu_head {
    _private: [usize; 0],
}

#[repr(C)]
pub struct radix_tree_root {
    _private: [usize; 0],
}

#[repr(C)]
pub struct xa_state {
    _private: [usize; 0],
}

// static RADIX_TREE(mt_tree, GFP_KERNEL);
static mut mt_tree: radix_tree_root = radix_tree_root { _private: [] };

#[repr(C)]
pub struct page {
    lock: pthread_mutex_t,
    rcu: rcu_head,
    count: c_int,
    index: c_ulong,
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn perror(s: *const i8);

    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn call_rcu(rcu: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_register_thread();
    fn rcu_unregister_thread();

    fn xa_lock(root: *mut radix_tree_root);
    fn xa_unlock(root: *mut radix_tree_root);
    fn radix_tree_insert(root: *mut radix_tree_root, index: c_ulong, item: *mut c_void) -> c_int;
    fn radix_tree_delete(root: *mut radix_tree_root, index: c_ulong) -> *mut c_void;

    fn xas_init(xas: *mut xa_state, root: *mut radix_tree_root, index: c_ulong);
    fn xas_next_entry(xas: *mut xa_state, max: c_ulong) -> *mut c_void;
    fn xas_retry(xas: *mut xa_state, entry: *mut c_void) -> bool;
    fn xas_reload(xas: *mut xa_state) -> *mut c_void;
    fn xas_reset(xas: *mut xa_state);

    fn printv(level: c_int, fmt: *const i8, ...);
}

unsafe fn page_alloc(index: c_int) -> *mut page {
    let p: *mut page = malloc(core::mem::size_of::<page>()) as *mut page;
    (*p).count = 1;
    (*p).index = index as c_ulong;
    pthread_mutex_init(core::ptr::addr_of_mut!((*p).lock), core::ptr::null());

    p
}

unsafe extern "C" fn page_rcu_free(rcu: *mut rcu_head) {
    let p: *mut page = (rcu as *mut u8).sub(core::mem::offset_of!(page, rcu)) as *mut page;
    assert!((*p).count == 0);
    pthread_mutex_destroy(core::ptr::addr_of_mut!((*p).lock));
    free(p as *mut c_void);
}

unsafe fn page_free(p: *mut page) {
    call_rcu(core::ptr::addr_of_mut!((*p).rcu), page_rcu_free);
}

unsafe fn find_get_pages(
    start: c_ulong,
    _nr_pages: c_uint,
    pages: *mut *mut page,
) -> c_uint {
    let mut xas: xa_state = core::mem::zeroed();
    let mut page: *mut page;
    let mut ret: c_uint = 0;

    xas_init(
        &mut xas,
        core::ptr::addr_of_mut!(mt_tree),
        start,
    );

    rcu_read_lock();
    loop {
        page = xas_next_entry(&mut xas, ULONG_MAX) as *mut page;
        if page.is_null() {
            break;
        }

        if xas_retry(&mut xas, page as *mut c_void) {
            continue;
        }

        pthread_mutex_lock(core::ptr::addr_of_mut!((*page).lock));
        if (*page).count == 0 {
            pthread_mutex_unlock(core::ptr::addr_of_mut!((*page).lock));
            xas_reset(&mut xas);
            continue;
        }

        /* don't actually update page refcount */
        pthread_mutex_unlock(core::ptr::addr_of_mut!((*page).lock));

        /* Has the page moved? */
        if page != xas_reload(&mut xas) as *mut page {
            xas_reset(&mut xas);
            continue;
        }

        *pages.add(ret as usize) = page;
        ret += 1;
        continue;
    }
    rcu_read_unlock();
    ret
}

static mut worker_barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };

unsafe extern "C" fn regression1_fn(arg: *mut c_void) -> *mut c_void {
    rcu_register_thread();

    if pthread_barrier_wait(core::ptr::addr_of_mut!(worker_barrier))
        == PTHREAD_BARRIER_SERIAL_THREAD
    {
        let mut j: c_int = 0;

        while j < 1000000 {
            let mut p: *mut page;

            p = page_alloc(0);
            xa_lock(core::ptr::addr_of_mut!(mt_tree));
            radix_tree_insert(core::ptr::addr_of_mut!(mt_tree), 0, p as *mut c_void);
            xa_unlock(core::ptr::addr_of_mut!(mt_tree));

            p = page_alloc(1);
            xa_lock(core::ptr::addr_of_mut!(mt_tree));
            radix_tree_insert(core::ptr::addr_of_mut!(mt_tree), 1, p as *mut c_void);
            xa_unlock(core::ptr::addr_of_mut!(mt_tree));

            xa_lock(core::ptr::addr_of_mut!(mt_tree));
            p = radix_tree_delete(core::ptr::addr_of_mut!(mt_tree), 1) as *mut page;
            pthread_mutex_lock(core::ptr::addr_of_mut!((*p).lock));
            (*p).count -= 1;
            pthread_mutex_unlock(core::ptr::addr_of_mut!((*p).lock));
            xa_unlock(core::ptr::addr_of_mut!(mt_tree));
            page_free(p);

            xa_lock(core::ptr::addr_of_mut!(mt_tree));
            p = radix_tree_delete(core::ptr::addr_of_mut!(mt_tree), 0) as *mut page;
            pthread_mutex_lock(core::ptr::addr_of_mut!((*p).lock));
            (*p).count -= 1;
            pthread_mutex_unlock(core::ptr::addr_of_mut!((*p).lock));
            xa_unlock(core::ptr::addr_of_mut!(mt_tree));
            page_free(p);

            j += 1;
        }
    } else {
        let mut j: c_int = 0;

        while j < 100000000 {
            let mut pages: [*mut page; 10] = [core::ptr::null_mut(); 10];

            find_get_pages(0, 10, pages.as_mut_ptr());
            j += 1;
        }
    }

    rcu_unregister_thread();

    core::ptr::null_mut()
}

static mut threads: *mut pthread_t = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regression1_test() {
    let mut nr_threads: c_int;
    let mut i: c_int;
    let mut arg: c_long;

    /* Regression #1 */
    printv(
        1,
        c"running regression test 1, should finish in under a minute\n".as_ptr(),
    );
    nr_threads = 2;
    pthread_barrier_init(
        core::ptr::addr_of_mut!(worker_barrier),
        core::ptr::null(),
        nr_threads as c_uint,
    );

    threads = malloc((nr_threads as usize) * core::mem::size_of::<pthread_t>()) as *mut pthread_t;

    i = 0;
    while i < nr_threads {
        arg = i as c_long;
        if pthread_create(
            threads.add(i as usize),
            core::ptr::null(),
            regression1_fn,
            arg as *mut c_void,
        ) != 0
        {
            perror(c"pthread_create".as_ptr());
            exit(1);
        }
        i += 1;
    }

    i = 0;
    while i < nr_threads {
        if pthread_join(*threads.add(i as usize), core::ptr::null_mut()) != 0 {
            perror(c"pthread_join".as_ptr());
            exit(1);
        }
        i += 1;
    }

    free(threads as *mut c_void);

    printv(1, c"regression test 1, done\n".as_ptr());
}
