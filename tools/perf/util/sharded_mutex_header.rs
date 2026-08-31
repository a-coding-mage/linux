/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: "mutex.h" provides `struct mutex`; "hashmap.h" provides `hash_bits`.
use core::ffi::c_uint;

pub type size_t = usize;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sharded_mutex {
    /* mutexes array is 1<<cap_bits in size. */
    pub cap_bits: c_uint,
    pub mutexes: [mutex; 0],
}

unsafe extern "C" {
    pub fn sharded_mutex__new(num_shards: size_t) -> *mut sharded_mutex;
    pub fn sharded_mutex__delete(sm: *mut sharded_mutex);
    pub fn hash_bits(hash: size_t, bits: c_uint) -> size_t;
}

/*
 * In a situation where a lock is needed per object, having a mutex can be
 * relatively memory expensive (40 bytes on x86-64). If the object can be
 * constantly hashed, a sharded mutex is an alternative global pool of mutexes
 * where the mutex is looked up from a hash value. This can lead to collisions
 * if the number of shards isn't large enough.
 */
#[inline]
pub unsafe fn sharded_mutex__get_mutex(sm: *mut sharded_mutex, hash: size_t) -> *mut mutex {
    unsafe {
        (*sm)
            .mutexes
            .as_mut_ptr()
            .add(hash_bits(hash, (*sm).cap_bits))
    }
}
