/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/linux/random.h
 *
 * Include file for the random number generator.
 */

/* ioctl()'s for the random number generator */

/* Get the entropy count. */
pub const RNDGETENTCNT: u32 = 0x8004_5200;

/* Add to (or subtract from) the entropy count.  (Superuser only.) */
pub const RNDADDTOENTCNT: u32 = 0x4004_5201;

/* Get the contents of the entropy pool.  (Superuser only.) (Removed in 2.6.9-rc2.) */
pub const RNDGETPOOL: u32 = 0x8008_5202;

/*
 * Write bytes into the entropy pool and add to the entropy count.
 * (Superuser only.)
 */
pub const RNDADDENTROPY: u32 = 0x4008_5203;

/* Clear entropy count to 0.  (Superuser only.) */
pub const RNDZAPENTCNT: u32 = 0x0000_5204;

/* Clear the entropy pool and associated counters.  (Superuser only.) */
pub const RNDCLEARPOOL: u32 = 0x0000_5206;

/* Reseed CRNG.  (Superuser only.) */
pub const RNDRESEEDCRNG: u32 = 0x0000_5207;

#[repr(C)]
pub struct rand_pool_info {
    pub entropy_count: core::ffi::c_int,
    pub buf_size: core::ffi::c_int,
    pub buf: [u32; 0],
}

/*
 * Flags for getrandom(2)
 *
 * GRND_NONBLOCK	Don't block and return EAGAIN instead
 * GRND_RANDOM		No effect
 * GRND_INSECURE	Return non-cryptographic random bytes
 */
pub const GRND_NONBLOCK: u32 = 0x0001;
pub const GRND_RANDOM: u32 = 0x0002;
pub const GRND_INSECURE: u32 = 0x0004;

/**
 * struct vgetrandom_opaque_params - arguments for allocating memory for vgetrandom
 *
 * @size_per_opaque_state:	Size of each state that is to be passed to vgetrandom().
 * @mmap_prot:			Value of the prot argument in mmap(2).
 * @mmap_flags:			Value of the flags argument in mmap(2).
 * @reserved:			Reserved for future use.
 */
#[repr(C)]
pub struct vgetrandom_opaque_params {
    pub size_of_opaque_state: u32,
    pub mmap_prot: u32,
    pub mmap_flags: u32,
    pub reserved: [u32; 13],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
