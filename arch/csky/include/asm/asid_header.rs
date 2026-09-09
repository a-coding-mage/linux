/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  The Linux types and helper operations used
// below are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct asid_info {
    pub generation: atomic64_t,
    pub map: *mut ::core::ffi::c_ulong,
    pub active: *mut atomic64_t,
    pub reserved: *mut u64,
    pub bits: u32,
    /* Lock protecting the structure */
    pub lock: raw_spinlock_t,
    /* Which CPU requires context flush on next call */
    pub flush_pending: cpumask_t,
    /* Number of ASID allocated by context (shift value) */
    pub ctxt_shift: ::core::ffi::c_uint,
    /* Callback to locally flush the context. */
    pub flush_cpu_ctxt_cb: Option<unsafe extern "C" fn()>,
}

#[inline]
pub const unsafe fn NUM_ASIDS(info: *const asid_info) -> ::core::ffi::c_ulong {
    1 as ::core::ffi::c_ulong << (*info).bits
}

#[inline]
pub const unsafe fn NUM_CTXT_ASIDS(info: *const asid_info) -> ::core::ffi::c_ulong {
    NUM_ASIDS(info) >> (*info).ctxt_shift
}

// C: #define active_asid(info, cpu) *per_cpu_ptr((info)->active, cpu)
#[inline]
pub unsafe fn active_asid(info: *mut asid_info, cpu: ::core::ffi::c_uint) -> atomic64_t {
    *per_cpu_ptr((*info).active, cpu)
}

extern "C" {
    pub fn per_cpu_ptr<T>(ptr: *mut T, cpu: ::core::ffi::c_uint) -> *mut T;

    pub fn asid_new_context(
        info: *mut asid_info,
        pasid: *mut atomic64_t,
        cpu: ::core::ffi::c_uint,
        mm: *mut mm_struct,
    );

    pub fn asid_allocator_init(
        info: *mut asid_info,
        bits: u32,
        asid_per_ctxt: ::core::ffi::c_uint,
        flush_cpu_ctxt_cb: Option<unsafe extern "C" fn()>,
    ) -> ::core::ffi::c_int;
}

/*
 * Check the ASID is still valid for the context. If not generate a new ASID.
 *
 * @pasid: Pointer to the current ASID batch
 * @cpu: current CPU ID. Must have been acquired through get_cpu()
 */
#[inline]
pub unsafe fn asid_check_context(
    info: *mut asid_info,
    pasid: *mut atomic64_t,
    cpu: ::core::ffi::c_uint,
    mm: *mut mm_struct,
) {
    let asid: u64;
    let old_active_asid: u64;

    asid = atomic64_read(pasid);

    /*
     * The memory ordering here is subtle.
     * If our active_asid is non-zero and the ASID matches the current
     * generation, then we update the active_asid entry with a relaxed
     * cmpxchg. Racing with a concurrent rollover means that either:
     *
     * - We get a zero back from the cmpxchg and end up waiting on the
     *   lock. Taking the lock synchronises with the rollover and so
     *   we are forced to see the updated generation.
     *
     * - We get a valid ASID back from the cmpxchg, which means the
     *   relaxed xchg in flush_context will treat us as reserved
     *   because atomic RmWs are totally ordered for a given location.
     */
    old_active_asid = atomic64_read(per_cpu_ptr((*info).active, cpu));
    if old_active_asid != 0
        && ((asid ^ atomic64_read(&(*info).generation)) >> (*info).bits) == 0
        && atomic64_cmpxchg_relaxed(per_cpu_ptr((*info).active, cpu), old_active_asid, asid) != 0
    {
        return;
    }

    asid_new_context(info, pasid, cpu, mm);
}

// External Linux kernel types and atomic helpers.
extern "C" {
    pub type atomic64_t;
    pub type raw_spinlock_t;
    pub type cpumask_t;
    pub type mm_struct;
    pub fn atomic64_read(v: *const atomic64_t) -> u64;
    pub fn atomic64_cmpxchg_relaxed(v: *mut atomic64_t, old: u64, new: u64) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
