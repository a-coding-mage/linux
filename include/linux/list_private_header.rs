/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

/**
 * DOC: Private List Primitives
 *
 * Provides a set of list primitives identical in function to those in
 * ``<linux/list.h>``, but designed for cases where the embedded
 * ``&struct list_head`` is private member.
 */

/* The compiler and list declarations are supplied by the surrounding crate. */

#[macro_export]
macro_rules! __list_private_offset {
    ($type:ty, $member:tt) => {
        ::core::mem::offset_of!($type, $member)
    };
}

/** Get the struct for this entry. */
#[macro_export]
macro_rules! list_private_entry {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr = $ptr as *const _;
        unsafe {
            (__mptr as *const u8)
                .sub($crate::__list_private_offset!($type, $member)) as *mut $type
        }
    }};
}

/** Get the first element from a list. */
#[macro_export]
macro_rules! list_private_first_entry {
    ($ptr:expr, $type:ty, $member:tt) => {
        $crate::list_private_entry!(unsafe { (*($ptr as *const _)).next }, $type, $member)
    };
}

/** Get the last element from a list. */
#[macro_export]
macro_rules! list_private_last_entry {
    ($ptr:expr, $type:ty, $member:tt) => {
        $crate::list_private_entry!(unsafe { (*($ptr as *const _)).prev }, $type, $member)
    };
}

/** Get the next element in a list. */
#[macro_export]
macro_rules! list_private_next_entry {
    ($pos:expr, $member:tt) => {
        $crate::list_private_entry!(unsafe { (*::core::ptr::addr_of!((*$pos).$member)).next },
                                    _, $member)
    };
}

/** Get the next element in a list, wrapping around at the end. */
#[macro_export]
macro_rules! list_private_next_entry_circular {
    ($pos:expr, $head:expr, $member:tt) => {
        if list_is_last!(unsafe { &*::core::ptr::addr_of!((*$pos).$member) }, $head) {
            $crate::list_private_first_entry!($head, _, $member)
        } else {
            $crate::list_private_next_entry!($pos, $member)
        }
    };
}

/** Get the previous element in a list. */
#[macro_export]
macro_rules! list_private_prev_entry {
    ($pos:expr, $member:tt) => {
        $crate::list_private_entry!(unsafe { (*::core::ptr::addr_of!((*$pos).$member)).prev },
                                    _, $member)
    };
}

/** Get the previous element in a list, wrapping around at the beginning. */
#[macro_export]
macro_rules! list_private_prev_entry_circular {
    ($pos:expr, $head:expr, $member:tt) => {
        if list_is_first!(unsafe { &*::core::ptr::addr_of!((*$pos).$member) }, $head) {
            $crate::list_private_last_entry!($head, _, $member)
        } else {
            $crate::list_private_prev_entry!($pos, $member)
        }
    };
}

/** Test if an entry points to the head of the list. */
#[macro_export]
macro_rules! list_private_entry_is_head {
    ($pos:expr, $head:expr, $member:tt) => {
        list_is_head!(unsafe { &*::core::ptr::addr_of!((*$pos).$member) }, $head)
    };
}

/* The remaining iteration primitives preserve the C for-loop shape. */
#[macro_export]
macro_rules! list_private_for_each_entry {
    ($pos:ident, $head:expr, $member:tt) => {
        for $pos in $crate::list_private_first_entry!($head, _, $member) {
            if $crate::list_private_entry_is_head!($pos, $head, $member) { break; }
        }
    };
}

#[macro_export]
macro_rules! list_private_for_each_entry_reverse {
    ($pos:ident, $head:expr, $member:tt) => {
        for $pos in $crate::list_private_last_entry!($head, _, $member) {
            if $crate::list_private_entry_is_head!($pos, $head, $member) { break; }
        }
    };
}

#[macro_export]
macro_rules! list_private_for_each_entry_continue {
    ($pos:expr, $head:expr, $member:tt, $body:block) => {{
        while !$crate::list_private_entry_is_head!($pos, $head, $member) {
            $body
            $pos = $crate::list_private_next_entry!($pos, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_private_for_each_entry_continue_reverse {
    ($pos:expr, $head:expr, $member:tt, $body:block) => {{
        while !$crate::list_private_entry_is_head!($pos, $head, $member) {
            $body
            $pos = $crate::list_private_prev_entry!($pos, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_private_for_each_entry_from {
    ($pos:expr, $head:expr, $member:tt, $body:block) => {
        $crate::list_private_for_each_entry_continue!($pos, $head, $member, $body)
    };
}

#[macro_export]
macro_rules! list_private_for_each_entry_from_reverse {
    ($pos:expr, $head:expr, $member:tt, $body:block) => {
        $crate::list_private_for_each_entry_continue_reverse!($pos, $head, $member, $body)
    };
}

#[macro_export]
macro_rules! list_private_for_each_entry_safe {
    ($pos:expr, $n:expr, $head:expr, $member:tt, $body:block) => {{
        $pos = $crate::list_private_first_entry!($head, _, $member);
        while !$crate::list_private_entry_is_head!($pos, $head, $member) {
            $n = $crate::list_private_next_entry!($pos, $member);
            $body
            $pos = $n;
        }
    }};
}

#[macro_export]
macro_rules! list_private_for_each_entry_safe_continue {
    ($pos:expr, $n:expr, $head:expr, $member:tt, $body:block) => {{
        $pos = $crate::list_private_next_entry!($pos, $member);
        $crate::list_private_for_each_entry_safe!($pos, $n, $head, $member, $body)
    }};
}

#[macro_export]
macro_rules! list_private_for_each_entry_safe_from {
    ($pos:expr, $n:expr, $head:expr, $member:tt, $body:block) => {
        $crate::list_private_for_each_entry_safe!($pos, $n, $head, $member, $body)
    };
}

#[macro_export]
macro_rules! list_private_for_each_entry_safe_reverse {
    ($pos:expr, $n:expr, $head:expr, $member:tt, $body:block) => {{
        $pos = $crate::list_private_last_entry!($head, _, $member);
        while !$crate::list_private_entry_is_head!($pos, $head, $member) {
            $n = $crate::list_private_prev_entry!($pos, $member);
            $body
            $pos = $n;
        }
    }};
}

/* C iteration macros are exposed as expression-level helpers for callers. */
#[macro_export]
macro_rules! list_private_safe_reset_next {
    ($pos:expr, $n:expr, $member:tt) => {
        $n = $crate::list_private_next_entry!($pos, $member)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
