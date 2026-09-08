/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from the C header: array_size.h and list.h provide
// ARRAY_SIZE, hlist_head, hlist_node, and the hlist operations used below.

#[macro_export]
macro_rules! hash_size {
    ($name:expr) => {
        ::core::mem::size_of_val(&$name) / ::core::mem::size_of_val(&$name[0])
    };
}

#[macro_export]
macro_rules! hashtable_declare {
    ($name:ident, $size:expr) => {
        let mut $name: [hlist_head; $size] = [HLIST_HEAD_INIT; $size];
    };
}

// C's designated range initializer is represented by an array initializer;
// callers must provide the corresponding hlist_head initialization constant.
#[macro_export]
macro_rules! hashtable_define {
    ($name:ident, $size:expr) => {
        static mut $name: [hlist_head; $size] = [HLIST_HEAD_INIT; $size];
    };
}

#[macro_export]
macro_rules! hash_head {
    ($table:expr, $key:expr) => {
        &mut $table[($key as usize) % hash_size!($table)]
    };
}

pub unsafe fn __hash_init(ht: *mut hlist_head, sz: ::core::ffi::c_uint) {
    let mut i: ::core::ffi::c_uint = 0;
    while i < sz {
        INIT_HLIST_HEAD(unsafe { ht.add(i as usize) });
        i = i.wrapping_add(1);
    }
}

/**
 * hash_init - initialize a hash table
 * @table: hashtable to be initialized
 *
 * This has to be a macro since HASH_SIZE() will not work on pointers since
 * it calculates the size during preprocessing.
 */
#[macro_export]
macro_rules! hash_init {
    ($table:expr) => {
        unsafe { __hash_init($table.as_mut_ptr(), hash_size!($table) as ::core::ffi::c_uint) }
    };
}

/**
 * hash_add - add an object to a hashtable
 * @table: hashtable to add to
 * @node: the &struct hlist_node of the object to be added
 * @key: the key of the object to be added
 */
#[macro_export]
macro_rules! hash_add {
    ($table:expr, $node:expr, $key:expr) => {
        hlist_add_head($node, hash_head!($table, $key))
    };
}

/**
 * hash_del - remove an object from a hashtable
 * @node: &struct hlist_node of the object to remove
 */
pub unsafe fn hash_del(node: *mut hlist_node) {
    hlist_del_init(node);
}

/**
 * hash_for_each - iterate over a hashtable
 * @table: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @member: the name of the hlist_node within the struct
 */
#[macro_export]
macro_rules! hash_for_each {
    ($table:expr, $obj:expr, $member:ident) => {
        for _bkt in 0..hash_size!($table) {
            hlist_for_each_entry!($obj, &mut $table[_bkt], $member)
        }
    };
}

/**
 * hash_for_each_safe - iterate over a hashtable safe against removal of
 * hash entry
 * @table: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @tmp: a &struct hlist_node used for temporary storage
 * @member: the name of the hlist_node within the struct
 */
#[macro_export]
macro_rules! hash_for_each_safe {
    ($table:expr, $obj:expr, $tmp:expr, $member:ident) => {
        for _bkt in 0..hash_size!($table) {
            hlist_for_each_entry_safe!($obj, $tmp, &mut $table[_bkt], $member)
        }
    };
}

/**
 * hash_for_each_possible - iterate over all possible objects hashing to the
 * same bucket
 * @table: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @member: the name of the hlist_node within the struct
 * @key: the key of the objects to iterate over
 */
#[macro_export]
macro_rules! hash_for_each_possible {
    ($table:expr, $obj:expr, $member:ident, $key:expr) => {
        hlist_for_each_entry!($obj, hash_head!($table, $key), $member)
    };
}

/**
 * hash_for_each_possible_safe - iterate over all possible objects hashing to the
 * same bucket safe against removals
 * @table: hashtable to iterate
 * @obj: the type * to use as a loop cursor for each entry
 * @tmp: a &struct hlist_node used for temporary storage
 * @member: the name of the hlist_node within the struct
 * @key: the key of the objects to iterate over
 */
#[macro_export]
macro_rules! hash_for_each_possible_safe {
    ($table:expr, $obj:expr, $tmp:expr, $member:ident, $key:expr) => {
        hlist_for_each_entry_safe!($obj, $tmp, hash_head!($table, $key), $member)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
