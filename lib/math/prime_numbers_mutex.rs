// SPDX-License-Identifier: GPL-2.0
//! The prime cache's statically initialized native writer mutex.
//!
//! This mirrors `DEFINE_MUTEX(lock)` from `include/linux/mutex.h`. In particular,
//! it is ready before any initcall and does not depend on module initialization.
//! The constructor is unsafe because the debug initializer is self-referential.

use kernel::{bindings, sync::Mutex, sync::MutexGuard, types::Opaque};

// Every architecture currently selecting HAVE_RUST has been checked against its
// actual C raw-spinlock initializer. A future port needs that review too; do not
// silently assume its unlocked representation is all zero.
#[cfg(all(
    CONFIG_SMP,
    not(any(
        CONFIG_X86,
        CONFIG_ARM,
        CONFIG_ARM64,
        CONFIG_RISCV,
        CONFIG_LOONGARCH,
        CONFIG_UML,
        CONFIG_S390,
        CONFIG_PPC
    ))
))]
compile_error!("prime static mutex: audit this architecture's raw-spinlock initializer");

/// Permanent native mutex storage, initialized exactly as `DEFINE_MUTEX(lock)`.
#[repr(transparent)]
pub(crate) struct StaticMutex(Opaque<bindings::mutex>);

// SAFETY: Storage is permanently addressed and initialized before publication.
// The underlying mutex provides synchronization; all interior mutation goes
// through the existing kernel mutex abstraction, not ordinary Rust references.
unsafe impl Sync for StaticMutex {}

impl StaticMutex {
    /// Builds the original prime-cache mutex at its final static address.
    ///
    /// # Safety
    ///
    /// `address` must identify the final allocation containing this returned
    /// object. That allocation must have static storage, must never be moved or
    /// reinitialized, and must remain alive until every guard and user is gone.
    /// This constructor must only initialize the private prime-cache writer
    /// mutex; the lockdep name intentionally remains the original `lock`.
    pub(crate) const unsafe fn new_at(address: *const Self) -> Self {
        // SAFETY: The actual binding consists of C integers, nullable raw
        // pointers and integer unions. Zero is valid storage for all of them.
        // All nonzero configuration-specific C initializer fields follow.
        let mut value: bindings::mutex = unsafe { core::mem::zeroed() };

        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            value.wait_lock = raw_spinlock();
            #[cfg(CONFIG_DEBUG_MUTEXES)]
            {
                // repr(transparent) and Opaque preserve the mutex's address.
                value.magic = address.cast_mut().cast();
            }
            // owner, optional osq.tail, and first_waiter are zero in C.
        }

        #[cfg(CONFIG_PREEMPT_RT)]
        {
            value.rtmutex.wait_lock = raw_spinlock();
            // RB_ROOT_CACHED and the RT owner are null in C.
        }

        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            value.dep_map.name = c"lock".as_ptr().cast();
            value.dep_map.wait_type_inner = bindings::lockdep_wait_type_LD_WAIT_SLEEP as u8;
            // key, class caches, other wait types and optional lock statistics
            // remain zero exactly as the C designated initializer specifies.
        }

        let _ = address;
        Self(Opaque::new(value))
    }

    /// Acquires the statically initialized mutex using the existing RAII guard.
    ///
    /// This may sleep and must not be called from an atomic context. The real
    /// MutexGuard is non-Send and unlocks through the normal mutex backend.
    pub(crate) fn lock(&'static self) -> MutexGuard<'static, ()> {
        // SAFETY: new_at's contract ensures valid initialized native mutex
        // storage at this permanent address. Lock<(), _>::from_raw explicitly
        // supports mutexes initialized externally, including C static initializers.
        unsafe { Mutex::<()>::from_raw(self.0.get()) }.lock()
    }
}

const fn raw_spinlock() -> bindings::raw_spinlock_t {
    // SAFETY: These real C fields have zero-valid scalar/union representations.
    // For SMP, every current HAVE_RUST architecture's original initializer is
    // all zero: generic qspin (x86/ARM64/LoongArch/RISC-V/UML), ARM tickets,
    // s390 .lock, and both PowerPC queued/simple configurations. Endianness does
    // not affect the all-zero initializer. The generic UP debug exception follows.
    #[cfg_attr(
        not(any(CONFIG_DEBUG_SPINLOCK, CONFIG_DEBUG_LOCK_ALLOC)),
        allow(unused_mut)
    )]
    let mut value: bindings::raw_spinlock_t = unsafe { core::mem::zeroed() };

    #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
    {
        // include/linux/spinlock_types_up.h: __ARCH_SPIN_LOCK_UNLOCKED { 1 }.
        value.raw_lock.slock = 1;
    }

    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    {
        value.magic = bindings::SPINLOCK_MAGIC;
        value.owner_cpu = u32::MAX;
        value.owner = usize::MAX as *mut kernel::ffi::c_void;
    }

    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    {
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            value.dep_map.name = c"lock.wait_lock".as_ptr().cast();
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            value.dep_map.name = c"lock.rtmutex.wait_lock".as_ptr().cast();
        }
        value.dep_map.wait_type_inner = bindings::lockdep_wait_type_LD_WAIT_SPIN as u8;
    }

    value
}
