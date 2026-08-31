// Translated from perf/util/rwsem.h
//
// C dependencies removed from executable Rust:
// - <pthread.h>
// - "mutex.h"

/*
 * Mutexes have additional error checking. Enable to use a mutex rather than a
 * rwlock for debugging.
 */
pub const RWS_ERRORCHECK: i32 = 0;

// C used:
//   struct LOCKABLE rw_semaphore {
//   #if RWS_ERRORCHECK
//       struct mutex mtx;
//   #else
//       pthread_rwlock_t lock;
//   #endif
//   };
//
// RWS_ERRORCHECK is 0 in this header, so the active layout contains
// pthread_rwlock_t. LOCKABLE is a lock-analysis annotation supplied externally.
#[repr(C)]
pub struct rw_semaphore {
    pub lock: pthread_rwlock_t,
}

unsafe extern "C" {
    pub fn init_rwsem(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;
    pub fn exit_rwsem(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;

    // C annotations:
    // - SHARED_LOCK_FUNCTION(sem)
    // - UNLOCK_FUNCTION(sem)
    // - EXCLUSIVE_LOCK_FUNCTION(sem)
    pub fn down_read(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;
    pub fn up_read(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;

    pub fn down_write(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;
    pub fn up_write(sem: *mut rw_semaphore) -> ::std::os::raw::c_int;
}
