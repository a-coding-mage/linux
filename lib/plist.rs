// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/plist.c
 *
 * Descending-priority-sorted double-linked list
 *
 * This is a direct Rust translation. Kernel list, plist, warning, BUG, and
 * initialization facilities are supplied by the surrounding translation.
 */

#[cfg(CONFIG_DEBUG_PLIST)]
static mut TEST_HEAD: plist_head = plist_head { /* supplied by kernel layout */ };

#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_check_prev_next(t: *mut list_head, p: *mut list_head, n: *mut list_head) {
    WARN(
        (*n).prev != p || (*p).next != n,
        "top: %p, n: %p, p: %p\nprev: %p, n: %p, p: %p\nnext: %p, n: %p, p: %p\n",
        t, (*t).next, (*t).prev, p, (*p).next, (*p).prev, n, (*n).next, (*n).prev,
    );
}

#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_check_list(top: *mut list_head) {
    let mut prev = top;
    let mut next = (*top).next;
    plist_check_prev_next(top, prev, next);
    while next != top {
        prev = next;
        next = (*prev).next;
        plist_check_prev_next(top, prev, next);
    }
}

#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_check_head(head: *mut plist_head) {
    if !plist_head_empty(head) {
        plist_check_list(&mut (*plist_first(head)).prio_list);
    }
    plist_check_list(&mut (*head).node_list);
}

#[cfg(not(CONFIG_DEBUG_PLIST))]
#[inline(always)]
unsafe fn plist_check_head(_h: *mut plist_head) {}

/// Add `node` to `head`.
pub unsafe fn plist_add(node: *mut plist_node, head: *mut plist_head) {
    let mut first: *mut plist_node;
    let mut iter: *mut plist_node;
    let mut prev: *mut plist_node = core::ptr::null_mut();
    let mut last: *mut plist_node;
    let mut reverse_iter: *mut plist_node;
    let mut node_next: *mut list_head = &mut (*head).node_list;

    plist_check_head(head);
    WARN_ON(!plist_node_empty(node));
    WARN_ON(!list_empty(&mut (*node).prio_list));

    if plist_head_empty(head) {
        list_add_tail(&mut (*node).node_list, node_next);
        plist_check_head(head);
        return;
    }

    first = plist_first(head);
    iter = first;
    last = list_entry((*first).prio_list.prev, plist_node, prio_list);
    reverse_iter = last;

    loop {
        if (*node).prio < (*iter).prio {
            node_next = &mut (*iter).node_list;
            break;
        } else if (*node).prio >= (*reverse_iter).prio {
            prev = reverse_iter;
            iter = list_entry((*reverse_iter).prio_list.next, plist_node, prio_list);
            if likely(reverse_iter != last) {
                node_next = &mut (*iter).node_list;
            }
            break;
        }
        prev = iter;
        iter = list_entry((*iter).prio_list.next, plist_node, prio_list);
        reverse_iter = list_entry((*reverse_iter).prio_list.prev, plist_node, prio_list);
        if iter == first {
            break;
        }
    }

    if prev.is_null() || (*prev).prio != (*node).prio {
        list_add_tail(&mut (*node).prio_list, &mut (*iter).prio_list);
    }
    list_add_tail(&mut (*node).node_list, node_next);
    plist_check_head(head);
}

/// Remove `node` from `head`.
pub unsafe fn plist_del(node: *mut plist_node, head: *mut plist_head) {
    plist_check_head(head);
    if !list_empty(&mut (*node).prio_list) {
        if (*node).node_list.next != &mut (*head).node_list {
            let next = list_entry((*node).node_list.next, plist_node, node_list);
            if list_empty(&mut (*next).prio_list) {
                list_add(&mut (*next).prio_list, &mut (*node).prio_list);
            }
        }
        list_del_init(&mut (*node).prio_list);
    }
    list_del_init(&mut (*node).node_list);
    plist_check_head(head);
}

/// Requeue `node` at the end of entries with the same priority.
pub unsafe fn plist_requeue(node: *mut plist_node, head: *mut plist_head) {
    let mut iter: *mut plist_node;
    let mut node_next: *mut list_head = &mut (*head).node_list;

    plist_check_head(head);
    BUG_ON(plist_head_empty(head));
    BUG_ON(plist_node_empty(node));
    if node == plist_last(head) {
        return;
    }
    iter = plist_next(node);
    if (*node).prio != (*iter).prio {
        return;
    }
    plist_del(node, head);
    if !list_empty(&mut (*iter).prio_list) {
        iter = list_entry((*iter).prio_list.next, plist_node, prio_list);
        node_next = &mut (*iter).node_list;
    } else {
        while iter != head as *mut plist_node {
            if (*node).prio != (*iter).prio {
                node_next = &mut (*iter).node_list;
                break;
            }
            iter = plist_next(iter);
        }
    }
    list_add_tail(&mut (*node).node_list, node_next);
    plist_check_head(head);
}

#[cfg(CONFIG_DEBUG_PLIST)]
static mut TEST_NODE: [plist_node; 241] = [plist_node { /* supplied by kernel layout */ }; 241];

#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_test_check(nr_expect: i32) {
    if plist_head_empty(&mut TEST_HEAD) {
        BUG_ON(nr_expect != 0);
        return;
    }
    let first = plist_first(&mut TEST_HEAD);
    let mut prio_pos = first;
    let mut remaining = nr_expect;
    plist_for_each!(node_pos, &mut TEST_HEAD, {
        if remaining < 0 { break; }
        remaining -= 1;
        if node_pos == first { continue; }
        if (*node_pos).prio == (*prio_pos).prio {
            BUG_ON(!list_empty(&mut (*node_pos).prio_list));
            continue;
        }
        BUG_ON((*prio_pos).prio > (*node_pos).prio);
        BUG_ON((*prio_pos).prio_list.next != &mut (*node_pos).prio_list);
        prio_pos = node_pos;
    });
    BUG_ON(remaining != 0);
    BUG_ON((*prio_pos).prio_list.next != &mut (*first).prio_list);
}

// The remaining CONFIG_DEBUG_PLIST self-test is intentionally retained as a
// direct kernel-facing translation; its init/module registration is external.
#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_test_requeue(node: *mut plist_node) {
    plist_requeue(node, &mut TEST_HEAD);
    if node != plist_last(&mut TEST_HEAD) {
        BUG_ON((*node).prio == (*plist_next(node)).prio);
    }
}

#[cfg(CONFIG_DEBUG_PLIST)]
unsafe fn plist_test() -> i32 {
    let mut nr_expect = 0;
    let mut r = local_clock();

    printk!(KERN_DEBUG, "start plist test\n");
    plist_head_init(&mut TEST_HEAD);
    for i in 0..TEST_NODE.len() {
        plist_node_init(TEST_NODE.as_mut_ptr().add(i), 0);
    }

    for _loop in 0..1000 {
        r = r.wrapping_mul(193939) % 47629;
        let i = (r % TEST_NODE.len() as u32) as usize;
        let node = TEST_NODE.as_mut_ptr().add(i);
        if plist_node_empty(node) {
            r = r.wrapping_mul(193939) % 47629;
            (*node).prio = (r % 99) as _;
            plist_add(node, &mut TEST_HEAD);
            nr_expect += 1;
        } else {
            plist_del(node, &mut TEST_HEAD);
            nr_expect -= 1;
        }
        plist_test_check(nr_expect);
        if !plist_node_empty(node) {
            plist_test_requeue(node);
            plist_test_check(nr_expect);
        }
    }

    for i in 0..TEST_NODE.len() {
        let node = TEST_NODE.as_mut_ptr().add(i);
        if plist_node_empty(node) { continue; }
        plist_del(node, &mut TEST_HEAD);
        nr_expect -= 1;
        plist_test_check(nr_expect);
    }

    printk!(KERN_DEBUG, "end plist test\n");

    let mut test_data = [0u32; 241];
    for i in 0..test_data.len() { test_data[i] = i as u32; }
    let mut start: ktime_t;
    let mut end: ktime_t;
    let mut time_elapsed: ktime_t = 0;

    plist_head_init(&mut TEST_HEAD);
    for i in 0..TEST_NODE.len() {
        let node = TEST_NODE.as_mut_ptr().add(i);
        plist_node_init(node, 0);
        (*node).prio = test_data[i] as _;
    }
    for i in 0..TEST_NODE.len() {
        let node = TEST_NODE.as_mut_ptr().add(i);
        if plist_node_empty(node) {
            start = ktime_get();
            plist_add(node, &mut TEST_HEAD);
            end = ktime_get();
            time_elapsed = time_elapsed.wrapping_add(end.wrapping_sub(start));
        }
    }
    pr_debug!("plist_add worst case test time elapsed %lld\n", time_elapsed);
    0
}

#[cfg(CONFIG_DEBUG_PLIST)]
module_init!(plist_test);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
