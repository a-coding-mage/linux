// SPDX-License-Identifier: GPL-2.0-only
//! Shared configured native type authority and work-data encoding.
//!
//! Original identities: kernel/workqueue.c:726-936 at
//! 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Generated declarations are a
//! future native build input, not supplied or fabricated by this source pass.

pub(super) use kernel::bindings::workqueue_native::*;
pub(super) use kernel::bindings::init_work_with_key;
pub(super) use kernel::ffi::{
    c_char,
    c_int,
    c_long,
    c_uint,
    c_ulong,
    c_void, //
};
use kernel::sync::barrier::{
    smp_mb,
    Full,
    Write, //
};

/// Allocates a pool ID from the exact native IDR interval.
///
/// # Safety
/// The caller holds wq_pool_mutex; pool is live and unpublished or protected
/// against concurrent ID assignment. Native IDR allocation may sleep.
pub(super) unsafe fn worker_pool_assign_id(pool: *mut worker_pool) -> c_int {
    // SAFETY: The pool mutex serializes IDR writes and pool initialization.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        let ret = idr_alloc(
            lupos_wq_worker_pool_idr(),
            pool.cast::<c_void>(),
            0,
            WORK_OFFQ_POOL_NONE as c_int,
            GFP_KERNEL,
        );
        if ret >= 0 {
            (*pool).id = ret;
            0
        } else {
            ret
        }
    }
}

/// Selects the per-CPU or default unbound PWQ publication slot.
///
/// # Safety
/// wq and its configured per-CPU allocation are live. Nonnegative cpu denotes
/// an allocated possible CPU. The returned raw slot is not a lifetime grant.
pub(super) unsafe fn unbound_pwq_slot(
    wq: *mut workqueue_struct,
    cpu: c_int,
) -> *mut *mut pool_workqueue {
    // SAFETY: Only native per-CPU address calculation crosses the ABI leaf.
    unsafe {
        if cpu >= 0 {
            lupos_wq_percpu_pwq_slot(wq, cpu)
        } else {
            &raw mut (*wq).dfl_pwq
        }
    }
}

/// Loads the selected PWQ using the native RCU dereference contract.
///
/// # Safety
/// The caller holds the wq pool mutex, wq mutex, or the appropriate RCU read
/// lock, and satisfies unbound_pwq_slot's CPU and allocation requirements.
pub(super) unsafe fn unbound_pwq(
    wq: *mut workqueue_struct,
    cpu: c_int,
) -> *mut pool_workqueue {
    // SAFETY: The selected slot remains live through the protected RCU load.
    unsafe { lupos_wq_unbound_pwq_dereference(unbound_pwq_slot(wq, cpu), wq) }
}

/// Returns the default pool's effective unbound mask without copying it.
///
/// # Safety
/// The caller provides unbound_pwq's exclusion and retains the selected PWQ,
/// pool and immutable attributes for the entire use of the returned mask.
pub(super) unsafe fn unbound_effective_cpumask(
    wq: *mut workqueue_struct,
) -> *mut cpumask {
    // SAFETY: Successful unbound setup supplies the non-null default PWQ.
    unsafe { lupos_wq_attrs_pod_cpumask((*(*unbound_pwq(wq, -1)).pool).attrs) }
}

/// Encodes a valid native work color into the unsigned flag word.
pub(super) fn work_color_to_flags(color: c_int) -> c_uint {
    (color as c_uint) << WORK_STRUCT_COLOR_SHIFT
}

/// Decodes the native work-color field from a work-data snapshot.
pub(super) fn get_work_color(work_data: c_ulong) -> c_int {
    ((work_data >> WORK_STRUCT_COLOR_SHIFT)
        & (((1 as c_ulong) << WORK_STRUCT_COLOR_BITS) - 1)) as c_int
}

/// Advances one valid color in the native cyclic flush-color space.
pub(super) fn work_next_color(color: c_int) -> c_int {
    // Callers pass [0, WORK_NR_COLORS); the addition cannot overflow int.
    color.wrapping_add(1) % (WORK_NR_COLORS as c_int)
}

/// Encodes whether the pool runs in bottom-half context.
///
/// # Safety
/// pool is live; its flags are read with native pool-lock or initialization
/// protection. The caller does not rely on this helper for synchronization.
pub(super) unsafe fn pool_offq_flags(pool: *mut worker_pool) -> c_ulong {
    // SAFETY: The caller provides native flag-field read exclusion.
    unsafe {
        if (*pool).flags & POOL_BH as c_uint != 0 {
            WORK_OFFQ_BH as c_ulong
        } else {
            0
        }
    }
}

/// Writes an owned work-data word while retaining its static debug bit.
///
/// # Safety
/// The caller owns PENDING and keeps work live; static initialization bits are
/// stable. The native warning is diagnostic, not an alternative ownership path.
pub(super) unsafe fn set_work_data(work: *mut work_struct, data: c_ulong) {
    // SAFETY: Native leaves preserve atomic_long_set, work_static, and the
    // exact non-pending warning condition; no Rust reference aliases the word.
    unsafe {
        lupos_wq_warn_set_work_data_not_pending(work);
        let static_flag = lupos_wq_work_static(work) as c_ulong;
        lupos_wq_work_data_set(work, data | static_flag);
    }
}

/// Publishes the aligned PWQ pointer and its native pending/queued flags.
///
/// # Safety
/// The caller owns work's PENDING bit, holds the relevant pool lock, and
/// retains the aligned PWQ reference for the queued lifetime.
pub(super) unsafe fn set_work_pwq(
    work: *mut work_struct,
    pwq: *mut pool_workqueue,
    flags: c_ulong,
) {
    // SAFETY: Native configured PWQ alignment reserves every flag/color bit.
    unsafe {
        set_work_data(
            work,
            (pwq as c_ulong)
                | WORK_STRUCT_PENDING as c_ulong
                | WORK_STRUCT_PWQ as c_ulong
                | flags,
        );
    }
}

/// Changes work data to off-queue pool information while retaining ownership.
///
/// # Safety
/// The caller owns PENDING, retains work, and passes a representable pool ID
/// and off-queue flags, with native locking for the surrounding transition.
pub(super) unsafe fn set_work_pool_and_keep_pending(
    work: *mut work_struct,
    pool_id: c_int,
    flags: c_ulong,
) {
    // SAFETY: PENDING remains set across this atomic data-word replacement.
    unsafe {
        set_work_data(
            work,
            ((pool_id as c_ulong) << WORK_OFFQ_POOL_SHIFT)
                | WORK_STRUCT_PENDING as c_ulong
                | flags,
        );
    }
}

/// Releases PENDING between the original write barrier and full barrier.
///
/// # Safety
/// The caller owns PENDING, retains work, and obeys the original pool/IRQ
/// transition contract. pool_id and flags fit the native off-queue encoding.
pub(super) unsafe fn set_work_pool_and_clear_pending(
    work: *mut work_struct,
    pool_id: c_int,
    flags: c_ulong,
) {
    // Pair prior work updates with the next test_and_set_bit(PENDING) owner.
    smp_mb(Write);
    // SAFETY: The caller owns PENDING up to this atomic publication.
    unsafe {
        set_work_data(work, ((pool_id as c_ulong) << WORK_OFFQ_POOL_SHIFT) | flags);
    }
    // Prevent callback loads/stores from passing the preceding PENDING clear.
    // The existing kernel barrier API retains a compiler barrier on !SMP.
    smp_mb(Full);
}

/// Decodes an aligned PWQ pointer from a queued work-data snapshot.
pub(super) fn work_struct_pwq(data: c_ulong) -> *mut pool_workqueue {
    (data & WORK_STRUCT_PWQ_MASK as c_ulong) as *mut pool_workqueue
}

/// Returns a queued PWQ from one atomic work-data snapshot, or null.
///
/// # Safety
/// work is live. The caller provides any pool/RCU exclusion needed to use the
/// returned PWQ; the observation alone does not pin the object.
pub(super) unsafe fn get_work_pwq(work: *mut work_struct) -> *mut pool_workqueue {
    // SAFETY: The native atomic load accesses a live work-data word once.
    unsafe {
        let data = lupos_wq_work_data_read(work);
        if data & WORK_STRUCT_PWQ as c_ulong != 0 {
            work_struct_pwq(data)
        } else {
            core::ptr::null_mut()
        }
    }
}

/// Resolves queued PWQ or off-queue IDR data to the associated worker pool.
///
/// # Safety
/// work is live. The caller holds wq_pool_mutex or an RCU read lock covering
/// this call and every use of the returned pool, unless separately retained.
pub(super) unsafe fn get_work_pool(work: *mut work_struct) -> *mut worker_pool {
    // SAFETY: Native IDR and queued PWQ destruction follow the caller's RCU
    // protection. Preserve the single load before the diagnostic assertion.
    unsafe {
        let data = lupos_wq_work_data_read(work);
        lupos_wq_assert_rcu_or_pool_mutex();
        if data & WORK_STRUCT_PWQ as c_ulong != 0 {
            return (*work_struct_pwq(data)).pool;
        }
        let pool_id = (data >> WORK_OFFQ_POOL_SHIFT) as c_int;
        if pool_id == WORK_OFFQ_POOL_NONE as c_int {
            return core::ptr::null_mut();
        }
        idr_find(lupos_wq_worker_pool_idr(), pool_id as c_ulong).cast::<worker_pool>()
    }
}

/// Extracts a native off-queue field whose mask has C unsigned-int width.
fn shift_and_mask(value: c_ulong, shift: u32, bits: u32) -> c_ulong {
    (value >> shift) & (((1 as c_uint) << bits) - 1) as c_ulong
}

/// Unpacks native off-queue pool, disable depth and flags into caller storage.
///
/// # Safety
/// offqd points to writable caller-owned storage; data is an off-queue native
/// snapshot. The original warning still reports a queued-word misuse.
pub(super) unsafe fn work_offqd_unpack(offqd: *mut work_offq_data, data: c_ulong) {
    // SAFETY: Only the caller's output fields are written; no pointer hidden
    // in the data word is dereferenced on this path.
    unsafe {
        lupos_wq_warn_work_offqd_unpack_pwq(data);
        (*offqd).pool_id = shift_and_mask(
            data,
            WORK_OFFQ_POOL_SHIFT as u32,
            WORK_OFFQ_POOL_BITS as u32,
        ) as u32;
        (*offqd).disable = shift_and_mask(
            data,
            WORK_OFFQ_DISABLE_SHIFT as u32,
            WORK_OFFQ_DISABLE_BITS as u32,
        ) as u32;
        (*offqd).flags = (data & WORK_OFFQ_FLAG_MASK as c_ulong) as u32;
    }
}

/// Packs the disable-depth and off-queue flag fields without a pool ID.
///
/// # Safety
/// offqd points to initialized readable native off-queue data; its fields obey
/// the original bounded disable-depth and flag invariants.
pub(super) unsafe fn work_offqd_pack_flags(offqd: *mut work_offq_data) -> c_ulong {
    // SAFETY: The caller retains the input through both plain field reads.
    unsafe {
        (((*offqd).disable as c_ulong) << WORK_OFFQ_DISABLE_SHIFT)
            | (*offqd).flags as c_ulong
    }
}

/// Exposes the native work-data bit-operation address, without accessing it.
///
/// # Safety
/// work is live and aligned as native work_struct. Any use of the returned
/// pointer follows the original work_data_bits and atomic/bitop locking rules.
pub(super) unsafe fn work_data_bits(work: *mut work_struct) -> *mut c_ulong {
    // SAFETY: This is the original address-only work_data_bits macro cast.
    unsafe { (&raw mut (*work).data).cast::<c_ulong>() }
}
