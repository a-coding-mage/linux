/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Light-weight single-linked queue built from llist
 *
 * Entries can be enqueued from any context with no locking.
 * Entries can be dequeued from process context with integrated locking.
 *
 * This is particularly suitable when work items are queued in
 * BH or IRQ context, and where work items are handled one at a time
 * by dedicated threads.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct lwq_node {
    pub node: llist_node,
}

#[repr(C)]
pub struct lwq {
    pub lock: spinlock_t,
    pub ready: *mut llist_node,
    pub new: llist_head,
}

/**
 * lwq_init - initialise a lwq
 * @q: the lwq object
 */
#[inline]
pub unsafe fn lwq_init(q: *mut lwq) {
    spin_lock_init(&mut (*q).lock);
    (*q).ready = core::ptr::null_mut();
    init_llist_head(&mut (*q).new);
}

/**
 * lwq_empty - test if lwq contains any entry
 * @q: the lwq object
 *
 * This empty test contains an acquire barrier so that if a wakeup
 * is sent when lwq_dequeue returns true, it is safe to go to sleep after
 * a test on lwq_empty().
 */
#[inline]
pub unsafe fn lwq_empty(q: *mut lwq) -> bool {
    /* acquire ensures ordering wrt lwq_enqueue() */
    smp_load_acquire(&(*q).ready) == core::ptr::null_mut() && llist_empty(&(*q).new)
}

extern "C" {
    pub fn __lwq_dequeue(q: *mut lwq) -> *mut llist_node;
}

/**
 * lwq_dequeue - dequeue first (oldest) entry from lwq
 * @q: the queue to dequeue from
 * @type: the type of object to return
 * @member: the member in returned object which is an lwq_node.
 *
 * Remove a single object from the lwq and return it.  This will take
 * a spinlock and so must always be called in the same context, typcially
 * process context.
 */
#[macro_export]
macro_rules! lwq_dequeue {
    ($q:expr, $type:ty, $member:ident) => {{
        let _n = $crate::__lwq_dequeue($q);
        if !_n.is_null() {
            container_of!(_n, $type, $member.node)
        } else {
            core::ptr::null_mut::<$type>()
        }
    }};
}

extern "C" {
    pub fn lwq_dequeue_all(q: *mut lwq) -> *mut llist_node;
}

/**
 * lwq_for_each_safe - iterate over detached queue allowing deletion
 * @_n: iterator variable
 * @_t1: temporary struct llist_node **
 * @_t2: temporary struct llist_node *
 * @_l: address of llist_node pointer from lwq_dequeue_all()
 * @_member: member in _n where lwq_node is found.
 *
 * Iterate over members in a dequeued list.  If the iterator variable
 * is set to NULL, the iterator removes that entry from the queue.
 */
#[macro_export]
macro_rules! lwq_for_each_safe {
    ($n:ident, $t1:ident, $t2:ident, $l:expr, $member:ident, $ty:ty) => {
        for $t1 in ($l).. {
            if (*$t1).is_null() { break; }
            $n = container_of!(*$t1, $ty, $member.node);
            $t2 = (*$t1).as_ref().map_or(core::ptr::null_mut(), |v| v.next);
            if !$n.is_null() {
                $t1 = &mut (*$n).$member.node.next;
            } else {
                *$t1 = $t2;
            }
        }
    };
}

/**
 * lwq_enqueue - add a new item to the end of the queue
 * @n - the lwq_node embedded in the item to be added
 * @q - the lwq to append to.
 *
 * No locking is needed to append to the queue so this can
 * be called from any context.
 * Return %true is the list may have previously been empty.
 */
#[inline]
pub unsafe fn lwq_enqueue(n: *mut lwq_node, q: *mut lwq) -> bool {
    /* acquire enqures ordering wrt lwq_dequeue */
    llist_add(&mut (*n).node, &mut (*q).new)
        && smp_load_acquire(&(*q).ready) == core::ptr::null_mut()
}

/**
 * lwq_enqueue_batch - add a list of new items to the end of the queue
 * @n - the lwq_node embedded in the first item to be added
 * @q - the lwq to append to.
 *
 * No locking is needed to append to the queue so this can
 * be called from any context.
 * Return %true is the list may have previously been empty.
 */
#[inline]
pub unsafe fn lwq_enqueue_batch(n: *mut llist_node, q: *mut lwq) -> bool {
    let e = n;
    llist_add_batch(llist_reverse_order(n), e, &mut (*q).new)
        && smp_load_acquire(&(*q).ready) == core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
