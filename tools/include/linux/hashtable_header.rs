/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Statically sized hash table implementation
 * (C) 2012  Sasha Levin <levinsasha928@gmail.com>
 */

// C dependencies: <linux/list.h>, <linux/types.h>, <linux/kernel.h>,
// <linux/bitops.h>, <linux/hash.h>, <linux/log2.h>

pub const HLIST_HEAD_INIT: hlist_head = hlist_head {
    first: core::ptr::null_mut(),
};

#[macro_export]
macro_rules! DEFINE_HASHTABLE {
    ($name:ident, $bits:expr) => {
        let mut $name: [hlist_head; 1usize << ($bits)] =
            [HLIST_HEAD_INIT; 1usize << ($bits)];
    };
}

#[macro_export]
macro_rules! DECLARE_HASHTABLE {
    ($name:ident, $bits:expr) => {
        let mut $name: [hlist_head; 1usize << ($bits)];
    };
}

#[macro_export]
macro_rules! HASH_SIZE {
    ($name:expr) => {
        ARRAY_SIZE($name)
    };
}

#[macro_export]
macro_rules! HASH_BITS {
    ($name:expr) => {
        ilog2(HASH_SIZE!($name))
    };
}

/* Use hash_32 when possible to allow for fast 32bit hashing in 64bit kernels. */
#[macro_export]
macro_rules! hash_min {
    ($val:expr, $bits:expr) => {
        if core::mem::size_of_val(&$val) <= 4 {
            hash_32($val, $bits)
        } else {
            hash_long($val, $bits)
        }
    };
}

#[inline]
pub unsafe fn __hash_init(ht: *mut hlist_head, sz: c_uint) {
    let mut i: c_uint;

    i = 0;
    while i < sz {
        INIT_HLIST_HEAD(ht.add(i as usize));
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
#[macro_export]
macro_rules! hash_init {
    ($hashtable:expr) => {
        __hash_init($hashtable, HASH_SIZE!($hashtable))
    };
}

/**
 * hash_add - add an object to a hashtable
 * @hashtable: hashtable to add to
 * @node: the &struct hlist_node of the object to be added
 * @key: the key of the object to be added
 */
#[macro_export]
macro_rules! hash_add {
    ($hashtable:expr, $node:expr, $key:expr) => {
        hlist_add_head(
            $node,
            &mut $hashtable[hash_min!($key, HASH_BITS!($hashtable)) as usize],
        )
    };
}

/**
 * hash_hashed - check whether an object is in any hashtable
 * @node: the &struct hlist_node of the object to be checked
 */
#[inline]
pub unsafe fn hash_hashed(node: *mut hlist_node) -> bool {
    !hlist_unhashed(node)
}

#[inline]
pub unsafe fn __hash_empty(ht: *mut hlist_head, sz: c_uint) -> bool {
    let mut i: c_uint;

    i = 0;
    while i < sz {
        if !hlist_empty(ht.add(i as usize)) {
            return false;
        }
        i = i.wrapping_add(1);
    }

    true
}

/**
 * hash_empty - check whether a hashtable is empty
 * @hashtable: hashtable to check
 *
 * This has to be a macro since HASH_BITS() will not work on pointers since
 * it calculates the size during preprocessing.
 */
#[macro_export]
macro_rules! hash_empty {
    ($hashtable:expr) => {
        __hash_empty($hashtable, HASH_SIZE!($hashtable))
    };
}

/**
 * hash_del - remove an object from a hashtable
 * @node: &struct hlist_node of the object to remove
 */
#[inline]
pub unsafe fn hash_del(node: *mut hlist_node) {
    hlist_del_init(node);
}

/**
 * hash_for_each - iterate over a hashtable
 * @name: hashtable to iterate
 * @bkt: integer to use as bucket loop cursor
 * @obj: the type * to use as a loop cursor for each entry
 * @member: the name of the hlist_node within the struct
 */
#[macro_export]
macro_rules! hash_for_each {
    ($name:expr, $bkt:expr, $obj:expr, $member:ident) => {
        for {
            $bkt = 0;
            $obj = core::ptr::null_mut();
        }; $obj.is_null() && $bkt < HASH_SIZE!($name); $bkt += 1 {
            hlist_for_each_entry!($obj, &$name[$bkt], $member)
        }
    };
}

/**
 * hash_for_each_safe - iterate over a hashtable safe against removal of
 * hash entry
 * @name: hashtable to iterate
 * @bkt: integer to use as bucket loop cursor
 * @tmp: a &struct used for temporary storage
 * @obj: the type * to use as a loop cursor for each entry
 * @member: the name of the hlist_node within the struct
 */
#[macro_export]
macro_rules! hash_for_each_safe {
    ($name:expr, $bkt:expr, $tmp:expr, $obj:expr, $member:ident) => {
        for {
            $bkt = 0;
            $obj = core::ptr::null_mut();
        }; $obj.is_null() && $bkt < HASH_SIZE!($name); $bkt += 1 {
            hlist_for_each_entry_safe!($obj, $tmp, &$name[$bkt], $member)
        }
    };
}

/**
 * hash_for_each_possible - iterate over all possible objects hashing to the
 * same bucket
 * @name: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @member: the name of the hlist_node within the struct
 * @key: the key of the objects to iterate over
 */
#[macro_export]
macro_rules! hash_for_each_possible {
    ($name:expr, $obj:expr, $member:ident, $key:expr) => {
        hlist_for_each_entry!(
            $obj,
            &$name[hash_min!($key, HASH_BITS!($name)) as usize],
            $member
        )
    };
}

/**
 * hash_for_each_possible_safe - iterate over all possible objects hashing to the
 * same bucket safe against removals
 * @name: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @tmp: a &struct used for temporary storage
 * @member: the name of the hlist_node within the struct
 * @key: the key of the objects to iterate over
 */
#[macro_export]
macro_rules! hash_for_each_possible_safe {
    ($name:expr, $obj:expr, $tmp:expr, $member:ident, $key:expr) => {
        hlist_for_each_entry_safe!(
            $obj,
            $tmp,
            &$name[hash_min!($key, HASH_BITS!($name)) as usize],
            $member
        )
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
