/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_MMU_H
// Dependencies supplied by the surrounding translation unit:
// linux/atomic.h, linux/spinlock.h, and linux/wait.h.

#[repr(C)]
pub union mm_context_t__bindgen_ty_1 {
    pub asid: [u64; NR_CPUS],
    pub mmid: atomic64_t,
}

#[repr(C)]
pub struct mm_context_t {
    pub _bindgen_data_: mm_context_t__bindgen_ty_1,

    pub vdso: *mut core::ffi::c_void,

    /* lock to be held whilst modifying fp_bd_emupage_allocmap */
    pub bd_emupage_lock: spinlock_t,
    /* bitmap tracking allocation of fp_bd_emupage */
    pub bd_emupage_allocmap: *mut c_ulong,
    /* wait queue for threads requiring an emuframe */
    pub bd_emupage_queue: wait_queue_head_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
