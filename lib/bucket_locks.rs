// Dependencies supplied by the surrounding kernel translation unit:
// spinlock_t, gfp_t, lock_class_key, num_possible_cpus, kvmalloc_objs,
// kvfree, spin_lock_init, lockdep_init_map, and ENOMEM.

/* Allocate an array of spinlocks to be accessed by a hash. Two arguments
 * indicate the number of elements to allocate in the array. max_size
 * gives the maximum number of elements to allocate. cpu_mult gives the
 * number of locks per CPU to allocate. The size is rounded up
 * to a power of 2 to be suitable as a hash table.
 */

#[allow(non_camel_case_types)]
pub unsafe fn __alloc_bucket_spinlocks(
    locks: *mut *mut spinlock_t,
    locks_mask: *mut ::core::ffi::c_uint,
    max_size: usize,
    cpu_mult: ::core::ffi::c_uint,
    gfp: gfp_t,
    name: *const ::core::ffi::c_char,
    key: *mut lock_class_key,
) -> ::core::ffi::c_int {
    let mut tlocks: *mut spinlock_t = core::ptr::null_mut();
    let mut i: ::core::ffi::c_uint;
    let size: ::core::ffi::c_uint;

    // CONFIG_PROVE_LOCKING selects the fixed value in the original build.
    #[cfg(CONFIG_PROVE_LOCKING)]
    let mut nr_pcpus: ::core::ffi::c_uint = 2;
    #[cfg(not(CONFIG_PROVE_LOCKING))]
    let mut nr_pcpus: ::core::ffi::c_uint = num_possible_cpus();

    if cpu_mult != 0 {
        nr_pcpus = core::cmp::min(nr_pcpus, 64u32);
        size = core::cmp::min(
            nr_pcpus.wrapping_mul(cpu_mult),
            max_size as ::core::ffi::c_uint,
        );
    } else {
        size = max_size as ::core::ffi::c_uint;
    }

    if core::mem::size_of::<spinlock_t>() != 0 {
        tlocks = kvmalloc_objs::<spinlock_t>(size, gfp);
        if tlocks.is_null() {
            return -ENOMEM;
        }
        i = 0;
        while i < size {
            spin_lock_init(tlocks.add(i as usize));
            lockdep_init_map(
                &mut (*tlocks.add(i as usize)).dep_map,
                name,
                key,
                0,
            );
            i = i.wrapping_add(1);
        }
    }

    *locks = tlocks;
    *locks_mask = size.wrapping_sub(1);

    0
}

pub unsafe fn free_bucket_spinlocks(locks: *mut spinlock_t) {
    kvfree(locks);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
