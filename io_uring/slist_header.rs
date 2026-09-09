/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: io_wq_work_node, io_wq_work_list, and io_wq_work are
// supplied by the translated io_uring types header.

#[macro_export]
macro_rules! __wq_list_for_each {
    ($pos:ident, $head:expr) => {
        for $pos in unsafe { (*($head)).first } {
            if $pos.is_null() {
                break;
            }
            $pos = unsafe { (*$pos).next };
        }
    };
}

#[macro_export]
macro_rules! wq_list_for_each {
    ($pos:ident, $prv:ident, $head:expr) => {
        for ($pos, $prv) in [(unsafe { (*($head)).first }, std::ptr::null_mut())] {
            if $pos.is_null() {
                break;
            }
            $prv = $pos;
            $pos = unsafe { (*$pos).next };
        }
    };
}

#[macro_export]
macro_rules! wq_list_empty {
    ($list:expr) => {
        unsafe { (*($list)).first == std::ptr::null_mut() }
    };
}

#[macro_export]
macro_rules! INIT_WQ_LIST {
    ($list:expr) => {{
        unsafe { (*($list)).first = std::ptr::null_mut(); }
    }};
}

#[inline]
pub unsafe fn wq_list_add_after(
    node: *mut io_wq_work_node,
    pos: *mut io_wq_work_node,
    list: *mut io_wq_work_list,
) {
    let next = (*pos).next;

    (*pos).next = node;
    (*node).next = next;
    if next.is_null() {
        (*list).last = node;
    }
}

#[inline]
pub unsafe fn wq_list_add_tail(node: *mut io_wq_work_node, list: *mut io_wq_work_list) {
    (*node).next = std::ptr::null_mut();
    if (*list).first.is_null() {
        (*list).last = node;
        (*list).first = node;
    } else {
        (*(*list).last).next = node;
        (*list).last = node;
    }
}

#[inline]
pub unsafe fn wq_list_cut(
    list: *mut io_wq_work_list,
    last: *mut io_wq_work_node,
    prev: *mut io_wq_work_node,
) {
    /* first in the list, if prev==NULL */
    if prev.is_null() {
        (*list).first = (*last).next;
    } else {
        (*prev).next = (*last).next;
    }

    if last == (*list).last {
        (*list).last = prev;
    }
    (*last).next = std::ptr::null_mut();
}

#[inline]
pub unsafe fn wq_stack_add_head(node: *mut io_wq_work_node, stack: *mut io_wq_work_node) {
    (*node).next = (*stack).next;
    (*stack).next = node;
}

#[inline]
pub unsafe fn wq_list_del(
    list: *mut io_wq_work_list,
    node: *mut io_wq_work_node,
    prev: *mut io_wq_work_node,
) {
    wq_list_cut(list, node, prev);
}

#[inline]
pub unsafe fn wq_stack_extract(stack: *mut io_wq_work_node) -> *mut io_wq_work_node {
    let node = (*stack).next;

    (*stack).next = (*node).next;
    node
}

#[inline]
pub unsafe fn wq_next_work(work: *mut io_wq_work) -> *mut io_wq_work {
    if (*work).list.next.is_null() {
        return std::ptr::null_mut();
    }

    let list = std::ptr::addr_of!((*work).list.next) as *mut u8;
    list.sub(std::mem::offset_of!(io_wq_work, list)) as *mut io_wq_work
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
