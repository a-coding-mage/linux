/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/* Dependency intent from C source: #include "bpf_experimental.h" */

/* list helpers from include/linux/list.h */
#[inline]
pub unsafe fn list_is_head(list: *const list_head, head: *const list_head) -> ::core::ffi::c_int {
    (list == head) as ::core::ffi::c_int
}

macro_rules! list_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

macro_rules! list_first_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        list_entry!((*($ptr)).next, $type, $member)
    };
}

macro_rules! list_next_entry {
    ($pos:expr, $type:ty, $member:ident) => {
        list_entry!((*($pos)).$member.next, $type, $member)
    };
}

macro_rules! list_entry_is_head {
    ($pos:expr, $head:expr, $member:ident) => {
        list_is_head(::core::ptr::addr_of!((*($pos)).$member), $head)
    };
}

/* small difference: 'can_loop' has been added in the conditions */
macro_rules! list_for_each_entry {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_first_entry!($head, $type, $member);
        while !list_entry_is_head!($pos, $head, $member) && can_loop {
            $body
            $pos = list_next_entry!($pos, $type, $member);
        }
    }};
}

/* mptcp helpers from protocol.h */
macro_rules! mptcp_for_each_subflow {
    ($__msk:expr, $__subflow:ident, $type:ty, $body:block) => {
        list_for_each_entry!(
            $__subflow,
            ::core::ptr::addr_of_mut!((*($__msk)).conn_list),
            $type,
            node,
            $body
        )
    };
}

#[inline]
pub unsafe fn mptcp_subflow_tcp_sock(subflow: *const mptcp_subflow_context) -> *mut sock {
    (*subflow).tcp_sock
}
