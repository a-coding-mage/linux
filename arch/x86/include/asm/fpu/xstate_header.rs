/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/uaccess.h, linux/types.h, asm/processor.h, asm/fpu/api.h, asm/user.h

/* Bit 63 of XCR0 is reserved for future expansion */
pub const XFEATURE_MASK_EXTEND: u64 = !(XFEATURE_MASK_FPSSE | (1u64 << 63));

pub const FXSAVE_SIZE: usize = 512;

pub const XSAVE_HDR_SIZE: usize = 64;
pub const XSAVE_HDR_OFFSET: usize = FXSAVE_SIZE;

pub const XSAVE_YMM_SIZE: usize = 256;
pub const XSAVE_YMM_OFFSET: usize = XSAVE_HDR_SIZE + XSAVE_HDR_OFFSET;

pub const XSAVE_ALIGNMENT: usize = 64;

/* All currently supported user features */
pub const XFEATURE_MASK_USER_SUPPORTED: u64 = XFEATURE_MASK_FP |
    XFEATURE_MASK_SSE |
    XFEATURE_MASK_YMM |
    XFEATURE_MASK_OPMASK |
    XFEATURE_MASK_ZMM_HI256 |
    XFEATURE_MASK_HI16_ZMM |
    XFEATURE_MASK_PKRU |
    XFEATURE_MASK_BNDREGS |
    XFEATURE_MASK_BNDCSR |
    XFEATURE_MASK_XTILE |
    XFEATURE_MASK_APX;

/*
 * Features which are restored when returning to user space.
 * PKRU is not restored on return to user space because PKRU
 * is switched eagerly in switch_to() and flush_thread()
 */
pub const XFEATURE_MASK_USER_RESTORE: u64 =
    XFEATURE_MASK_USER_SUPPORTED & !XFEATURE_MASK_PKRU;

/* Features which are dynamically enabled for a process on request */
pub const XFEATURE_MASK_USER_DYNAMIC: u64 = XFEATURE_MASK_XTILE_DATA;

/* Supervisor features which are enabled only in guest FPUs */
pub const XFEATURE_MASK_GUEST_SUPERVISOR: u64 = XFEATURE_MASK_CET_KERNEL;

/* All currently supported supervisor features */
pub const XFEATURE_MASK_SUPERVISOR_SUPPORTED: u64 = XFEATURE_MASK_PASID |
    XFEATURE_MASK_CET_USER |
    XFEATURE_MASK_GUEST_SUPERVISOR;

/*
 * A supervisor state component may not always contain valuable information,
 * and its size may be huge. Saving/restoring such supervisor state components
 * at each context switch can cause high CPU and space overhead, which should
 * be avoided. Such supervisor state components should only be saved/restored
 * on demand. The on-demand supervisor features are set in this mask.
 *
 * Unlike the existing supported supervisor features, an independent supervisor
 * feature does not allocate a buffer in task->fpu, and the corresponding
 * supervisor state component cannot be saved/restored at each context switch.
 *
 * To support an independent supervisor feature, a developer should follow the
 * dos and don'ts as below:
 * - Do dynamically allocate a buffer for the supervisor state component.
 * - Do manually invoke the XSAVES/XRSTORS instruction to save/restore the
 *   state component to/from the buffer.
 * - Don't set the bit corresponding to the independent supervisor feature in
 *   IA32_XSS at run time, since it has been set at boot time.
 */
pub const XFEATURE_MASK_INDEPENDENT: u64 = XFEATURE_MASK_LBR;

/*
 * Unsupported supervisor features. When a supervisor feature in this mask is
 * supported in the future, move it to the supported supervisor feature mask.
 */
pub const XFEATURE_MASK_SUPERVISOR_UNSUPPORTED: u64 = XFEATURE_MASK_PT;

/* All supervisor states including supported and unsupported states. */
pub const XFEATURE_MASK_SUPERVISOR_ALL: u64 = XFEATURE_MASK_SUPERVISOR_SUPPORTED |
    XFEATURE_MASK_INDEPENDENT |
    XFEATURE_MASK_SUPERVISOR_UNSUPPORTED;

/*
 * The feature mask required to restore FPU state:
 * - All user states which are not eagerly switched in switch_to()/exec()
 * - The suporvisor states
 */
pub const XFEATURE_MASK_FPSTATE: u64 = XFEATURE_MASK_USER_RESTORE |
    XFEATURE_MASK_SUPERVISOR_SUPPORTED;

/*
 * Features in this mask have space allocated in the signal frame, but may not
 * have that space initialized when the feature is in its init state.
 */
pub const XFEATURE_MASK_SIGFRAME_INITOPT: u64 = XFEATURE_MASK_XTILE |
    XFEATURE_MASK_USER_DYNAMIC;

extern "C" {
    pub static mut xstate_fx_sw_bytes: [u64; USER_XSTATE_FX_SW_WORDS];

    pub fn update_regset_xstate_info(size: u32, xstate_mask: u64);

    pub fn xfeature_size(xfeature_nr: i32) -> i32;

    pub fn xsaves(xsave: *mut xregs_state, mask: u64);
    pub fn xrstors(xsave: *mut xregs_state, mask: u64);

    pub fn xfd_enable_feature(xfd_err: u64) -> i32;
}

// CONFIG_X86_64 controls this declaration in the original header.
#[cfg(target_arch = "x86_64")]
extern "C" {
    pub static mut __fpu_state_size_dynamic: core::ffi::c_void;
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn fpu_state_size_dynamic() -> bool {
    static_branch_unlikely(&__fpu_state_size_dynamic)
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub const fn fpu_state_size_dynamic() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
