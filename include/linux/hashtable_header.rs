/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Statically sized hash table implementation
 * (C) 2012  Sasha Levin <levinsasha928@gmail.com>
 */

// Dependencies supplied by the surrounding Linux translation:
// linux/list.h, linux/types.h, linux/kernel.h, linux/hash.h, linux/rculist.h

macro_rules! DEFINE_HASHTABLE {
    ($name:ident, $bits:expr) => {
        let mut $name: [hlist_head; 1usize << ($bits)] =
            [HLIST_HEAD_INIT; 1usize << ($bits)];
    };
}

macro_rules! DEFINE_READ_MOSTLY_HASHTABLE {
    ($name:ident, $bits:expr) => {
        let mut $name: [hlist_head; 1usize << ($bits)] =
            [HLIST_HEAD_INIT; 1usize << ($bits)];
        // C __read_mostly annotation has no direct file-local Rust equivalent.
    };
}

macro_rules! DECLARE_HASHTABLE {
    ($name:ident, $bits:expr) => {
        let mut $name: [hlist_head; 1usize << ($bits)] =
            [HLIST_HEAD_INIT; 1usize << ($bits)];
    };
}

macro_rules! HASH_SIZE {
    ($name:expr) => { $name.len() };
}

macro_rules! HASH_BITS {
    ($name:expr) => { ilog2(HASH_SIZE!($name)) };
}

/* Use hash_32 when possible to allow for fast 32bit hashing in 64bit kernels. */
macro_rules! hash_min {
    ($val:expr, $bits:expr) => {{
        if core::mem::size_of_val(&$val) <= 4 {
            hash_32($val, $bits)
        } else {
            hash_long($val, $bits)
        }
    }};
}

#[inline]
pub unsafe fn __hash_init(ht: *mut hlist_head, sz: c_uint) {
    let mut i: c_uint = 0;
    while i < sz {
        INIT_HLIST_HEAD(unsafe { ht.add(i as usize) });
        i = i.wrapping_add(1);
    }
}

/**
 * hash_init - initialize a hash table
 * @hashtable: hashtable to be initialized
 *
 * Calculates the size of the hashtable from the given parameter, otherwise
 * same as hash_init_size.
 *
 * This has to be a macro since HASH_BITS() will not work on pointers since
 * it calculates the size during preprocessing.
 */
macro_rules! hash_init {
    ($hashtable:expr) => {
        unsafe { __hash_init($hashtable.as_mut_ptr(), HASH_SIZE!($hashtable) as c_uint) }
    };
}

/** hash_add - add an object to a hashtable */
macro_rules! hash_add {
    ($hashtable:expr, $node:expr, $key:expr) => {
        hlist_add_head($node, &mut $hashtable[hash_min!($key, HASH_BITS!($hashtable)) as usize])
    };
}

/** hash_add_rcu - add an object to a rcu enabled hashtable */
macro_rules! hash_add_rcu {
    ($hashtable:expr, $node:expr, $key:expr) => {
        hlist_add_head_rcu($node, &mut $hashtable[hash_min!($key, HASH_BITS!($hashtable)) as usize])
    };
}

#[inline]
pub unsafe fn hash_hashed(node: *mut hlist_node) -> bool {
    !hlist_unhashed(node)
}

#[inline]
pub unsafe fn __hash_empty(ht: *mut hlist_head, sz: c_uint) -> bool {
    let mut i: c_uint = 0;
    while i < sz {
        if !hlist_empty(unsafe { ht.add(i as usize) }) {
            return false;
        }
        i = i.wrapping_add(1);
    }
    true
}

/** hash_empty - check whether a hashtable is empty */
macro_rules! hash_empty {
    ($hashtable:expr) => {
        unsafe { __hash_empty($hashtable.as_mut_ptr(), HASH_SIZE!($hashtable) as c_uint) }
    };
}

#[inline]
pub unsafe fn hash_del(node: *mut hlist_node) {
    hlist_del_init(node);
}

#[inline]
pub unsafe fn hash_del_rcu(node: *mut hlist_node) {
    hlist_del_init_rcu(node);
}

macro_rules! hash_for_each {
    ($name:expr, $bkt:ident, $obj:ident, $member:ident) => {
        for $bkt in 0..HASH_SIZE!($name) {
            hlist_for_each_entry!($obj, &$name[$bkt], $member);
        }
    };
}

macro_rules! hash_for_each_rcu {
    ($name:expr, $bkt:ident, $obj:ident, $member:ident) => {
        for $bkt in 0..HASH_SIZE!($name) {
            hlist_for_each_entry_rcu!($obj, &$name[$bkt], $member);
        }
    };
}

macro_rules! hash_for_each_safe {
    ($name:expr, $bkt:ident, $tmp:ident, $obj:ident, $member:ident) => {
        for $bkt in 0..HASH_SIZE!($name) {
            hlist_for_each_entry_safe!($obj, $tmp, &$name[$bkt], $member);
        }
    };
}

macro_rules! hash_for_each_possible {
    ($name:expr, $obj:ident, $member:ident, $key:expr) => {
        hlist_for_each_entry!($obj, &$name[hash_min!($key, HASH_BITS!($name)) as usize], $member);
    };
}

macro_rules! hash_for_each_possible_rcu {
    ($name:expr, $obj:ident, $member:ident, $key:expr, $($cond:tt)*) => {
        hlist_for_each_entry_rcu!($obj, &$name[hash_min!($key, HASH_BITS!($name)) as usize], $member, $($cond)*);
    };
}

macro_rules! hash_for_each_possible_rcu_notrace {
    ($name:expr, $obj:ident, $member:ident, $key:expr) => {
        hlist_for_each_entry_rcu_notrace!($obj, &$name[hash_min!($key, HASH_BITS!($name)) as usize], $member);
    };
}

macro_rules! hash_for_each_possible_safe {
    ($name:expr, $obj:ident, $tmp:ident, $member:ident, $key:expr) => {
        hlist_for_each_entry_safe!($obj, $tmp, &$name[hash_min!($key, HASH_BITS!($name)) as usize], $member);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
