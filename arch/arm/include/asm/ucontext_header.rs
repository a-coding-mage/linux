/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding architecture headers. */
use core::ffi::{c_int, c_ulong};

/*
 * struct sigcontext only has room for the basic registers, but struct
 * ucontext now has room for all registers which need to be saved and
 * restored. Coprocessor registers are stored in uc_regspace. Each
 * coprocessor's saved state should start with a documented 32-bit magic
 * number, followed by a 32-bit word giving the coprocessor's saved size.
 * uc_regspace may be expanded if necessary, although this takes some
 * coordination with glibc.
 */
#[repr(C)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: sigcontext,
    pub uc_sigmask: sigset_t,
    /* Allow for uc_sigmask growth. Glibc uses a 1024-bit sigset_t. */
    pub __unused: [c_int; 32 - (core::mem::size_of::<sigset_t>() / core::mem::size_of::<c_int>())],
    /* Last for extensibility. Eight byte aligned because some coprocessors require eight byte alignment. */
    pub uc_regspace: [c_ulong; 128],
}

/* The original field has __attribute__((__aligned__(8))). */
const _: () = assert!(core::mem::align_of::<ucontext>() >= 8);

/*
 * Coprocessor save state. The magic values and specific coprocessor's
 * layouts are part of the userspace ABI. Each one of these should be a
 * multiple of eight bytes and aligned to eight bytes, to prevent
 * unpredictable padding in the signal frame.
 */

/*
 * Dummy padding block: if this magic is encountered, the block should be
 * skipped using the corresponding size field.
 */
pub const DUMMY_MAGIC: c_ulong = 0xb0d9ed01;

/* CONFIG_IWMMXT is a kernel build-time condition. */
#[cfg(feature = "CONFIG_IWMMXT")]
pub const IWMMXT_MAGIC: c_ulong = 0x12ef842a;
#[cfg(feature = "CONFIG_IWMMXT")]
pub const IWMMXT_STORAGE_SIZE: usize = IWMMXT_SIZE as usize + 8;

#[cfg(feature = "CONFIG_IWMMXT")]
#[repr(C, align(8))]
pub struct iwmmxt_sigframe {
    pub magic: c_ulong,
    pub size: c_ulong,
    pub storage: iwmmxt_struct,
}

/* CONFIG_VFP is a kernel build-time condition. */
#[cfg(feature = "CONFIG_VFP")]
pub const VFP_MAGIC: c_ulong = 0x56465001;

#[cfg(feature = "CONFIG_VFP")]
#[repr(C, align(8))]
pub struct vfp_sigframe {
    pub magic: c_ulong,
    pub size: c_ulong,
    pub ufp: user_vfp,
    pub ufp_exc: user_vfp_exc,
}

/*
 * 8 byte for magic and size, 264 byte for ufp, 12 bytes for ufp_exc,
 * 4 bytes padding.
 */
#[cfg(feature = "CONFIG_VFP")]
pub const VFP_STORAGE_SIZE: usize = core::mem::size_of::<vfp_sigframe>();

/*
 * Auxiliary signal frame. This saves stuff like FP state.
 * The layout of this structure is not part of the user ABI,
 * because the config options aren't. uc_regspace is really one of these.
 */
#[repr(C, align(8))]
pub struct aux_sigframe {
    #[cfg(feature = "CONFIG_IWMMXT")]
    pub iwmmxt: iwmmxt_sigframe,
    #[cfg(feature = "CONFIG_VFP")]
    pub vfp: vfp_sigframe,
    /* Something that isn't a valid magic number for any coprocessor. */
    pub end_magic: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
