// SPDX-License-Identifier: GPL-2.0-only
/*
 * Light-weight single-linked queue.
 *
 * Entries are enqueued to the head of an llist, with no blocking.
 * This can happen in any context.
 *
 * Entries are dequeued using a spinlock to protect against multiple
 * access.  The llist is staged in reverse order, and refreshed
 * from the llist when it exhausts.
 *
 * This is particularly suitable when work items are queued in BH or
 * IRQ context, and where work items are handled one at a time by
 * dedicated threads.
 */

#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct llist_head {
    pub first: *mut llist_node,
}

#[repr(C)]
pub struct lwq {
    pub lock: spinlock_t,
    pub ready: *mut llist_node,
    pub new: llist_head,
}

extern "C" {
    fn lwq_empty(q: *const lwq) -> bool;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn smp_store_release(p: *mut *mut llist_node, v: *mut llist_node);
    fn llist_empty(head: *const llist_head) -> bool;
    fn llist_reverse_order(head: *mut llist_node) -> *mut llist_node;
    fn llist_del_all(head: *mut llist_head) -> *mut llist_node;
    fn llist_next(node: *mut llist_node) -> *mut llist_node;
}

#[no_mangle]
pub unsafe extern "C" fn __lwq_dequeue(q: *mut lwq) -> *mut llist_node {
    let mut this: *mut llist_node;

    if lwq_empty(q) {
        return core::ptr::null_mut();
    }
    spin_lock(&mut (*q).lock);
    this = (*q).ready;
    if this.is_null() && !llist_empty(&(*q).new) {
        /* ensure queue doesn't appear transiently lwq_empty */
        smp_store_release(&mut (*q).ready, 1usize as *mut llist_node);
        this = llist_reverse_order(llist_del_all(&mut (*q).new));
        if this.is_null() {
            (*q).ready = core::ptr::null_mut();
        }
    }
    if !this.is_null() {
        (*q).ready = llist_next(this);
    }
    spin_unlock(&mut (*q).lock);
    this
}

/* EXPORT_SYMBOL_GPL(__lwq_dequeue); */

/*
 * lwq_dequeue_all - dequeue all currently enqueued objects
 * @q: the queue to dequeue from
 *
 * Remove and return a linked list of llist_nodes of all the objects that were
 * in the queue. The first on the list will be the object that was least
 * recently enqueued.
 */
#[no_mangle]
pub unsafe extern "C" fn lwq_dequeue_all(q: *mut lwq) -> *mut llist_node {
    let (mut r, t, mut ep): (*mut llist_node, *mut llist_node, *mut *mut llist_node);

    if lwq_empty(q) {
        return core::ptr::null_mut();
    }

    spin_lock(&mut (*q).lock);
    r = (*q).ready;
    (*q).ready = core::ptr::null_mut();
    t = llist_del_all(&mut (*q).new);
    spin_unlock(&mut (*q).lock);
    ep = &mut r;
    while !(*ep).is_null() {
        ep = &mut (**ep).next;
    }
    *ep = llist_reverse_order(t);
    r
}

/* EXPORT_SYMBOL_GPL(lwq_dequeue_all); */

/* CONFIG_LWQ_TEST is a build-time condition from the Linux kernel environment. */
#[cfg(feature = "CONFIG_LWQ_TEST")]
mod lwq_test {
    use super::*;

    #[repr(C)]
    pub struct lwq_node {
        pub node: llist_node,
    }

    #[repr(C)]
    pub struct tnode {
        pub n: lwq_node,
        pub i: i32,
        pub c: i32,
    }

    extern "C" {
        fn lwq_init(q: *mut lwq);
        fn lwq_enqueue(node: *mut lwq_node, q: *mut lwq) -> bool;
        fn lwq_enqueue_batch(list: *mut llist_node, q: *mut lwq);
        fn lwq_dequeue_typed(q: *mut lwq) -> *mut tnode;
        fn wait_var_event(q: *mut lwq, condition: bool);
        fn wake_up_var(q: *mut lwq);
        fn kthread_should_stop() -> bool;
        fn schedule_timeout_idle(timeout: i32);
        fn kthread_run(f: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                        data: *mut core::ffi::c_void,
                        name: *const core::ffi::c_char,
                        ...) -> *mut core::ffi::c_void;
        fn kthread_stop(task: *mut core::ffi::c_void);
        fn kmalloc_tnode() -> *mut tnode;
        fn kfree(p: *mut core::ffi::c_void);
        fn printk_info(s: *const core::ffi::c_char, ...);
        fn printk_cont(s: *const core::ffi::c_char, ...);
    }

    unsafe extern "C" fn lwq_exercise(qv: *mut core::ffi::c_void) -> i32 {
        let q = qv as *mut lwq;
        let mut cnt = 0;
        while cnt < 10000 {
            let t = lwq_dequeue_typed(q);
            wait_var_event(q, !t.is_null());
            if !t.is_null() {
                (*t).c += 1;
                if lwq_enqueue(&mut (*t).n, q) {
                    wake_up_var(q);
                }
            }
            cnt += 1;
        }
        while !kthread_should_stop() {
            schedule_timeout_idle(1);
        }
        0
    }

    #[allow(dead_code)]
    unsafe fn lwq_test() -> i32 {
        let mut q: lwq = core::mem::zeroed();
        let mut threads: [*mut core::ffi::c_void; 8] = [core::ptr::null_mut(); 8];
        printk_info(b"testing lwq....\0".as_ptr() as *const _);
        lwq_init(&mut q);
        printk_info(b" lwq: run some threads\n\0".as_ptr() as *const _);
        for i in 0..threads.len() {
            threads[i] = kthread_run(lwq_exercise, &mut q as *mut _ as *mut _,
                                     b"lwq-test-%d\0".as_ptr() as *const _, i);
        }
        for i in 0..100 {
            let t = kmalloc_tnode();
            if t.is_null() { break; }
            (*t).i = i;
            (*t).c = 0;
            if lwq_enqueue(&mut (*t).n, &mut q) {
                wake_up_var(&mut q);
            }
        }
        for task in threads {
            if !task.is_null() { kthread_stop(task); }
        }
        printk_info(b" lwq: dequeue first 50:\0".as_ptr() as *const _);
        for i in 0..50 {
            let t = lwq_dequeue_typed(&mut q);
            if !t.is_null() { printk_cont(b" %d(%d)\0".as_ptr() as *const _, (*t).i, (*t).c); }
            kfree(t as *mut _);
            let _ = i;
        }
        printk_cont(b"\n\0".as_ptr() as *const _);
        let mut l = lwq_dequeue_all(&mut q);
        while !l.is_null() {
            let t = l as *mut tnode;
            let next = (*(*t).n.node.next).next;
            if (*t).i % 3 == 0 { (*t).i = -1; kfree(t as *mut _); }
            l = next;
        }
        if !l.is_null() { lwq_enqueue_batch(l, &mut q); }
        while !lwq_dequeue_typed(&mut q).is_null() {}
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
