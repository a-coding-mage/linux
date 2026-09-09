/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct stat {
	pub st_dev: u32,       /* dev_t is 32 bits on parisc */
	pub st_ino: u32,       /* 32 bits */
	pub st_mode: u16,      /* 16 bits */
	pub st_nlink: u16,     /* 16 bits */
	pub st_reserved1: u16, /* old st_uid */
	pub st_reserved2: u16, /* old st_gid */
	pub st_rdev: u32,
	pub st_size: i32,
	pub st_atime: i32,
	pub st_atime_nsec: u32,
	pub st_mtime: i32,
	pub st_mtime_nsec: u32,
	pub st_ctime: i32,
	pub st_ctime_nsec: u32,
	pub st_blksize: i32,
	pub st_blocks: i32,
	pub __unused1: u32, /* ACL stuff */
	pub __unused2: u32, /* network */
	pub __unused3: u32, /* network */
	pub __unused4: u32, /* cnodes */
	pub __unused5: u16, /* netsite */
	pub st_fstype: i16,
	pub st_realdev: u32,
	pub st_basemode: u16,
	pub st_spareshort: u16,
	pub st_uid: u32,
	pub st_gid: u32,
	pub st_spare4: [u32; 3],
}

pub const STAT_HAVE_NSEC: bool = true;

/* This is the struct that 32-bit userspace applications are expecting.
 * How 64-bit apps are going to be compiled, I have no idea.  But at least
 * this way, we don't have a wrapper in the kernel.
 */
#[repr(C)]
pub struct stat64 {
	pub st_dev: u64,
	pub __pad1: u32,

	pub __st_ino: u32, /* Not actually filled in */
	pub st_mode: u32,
	pub st_nlink: u32,
	pub st_uid: u32,
	pub st_gid: u32,
	pub st_rdev: u64,
	pub __pad2: u32,
	pub st_size: i64,
	pub st_blksize: i32,

	pub st_blocks: i64,
	pub st_atime: i32,
	pub st_atime_nsec: u32,
	pub st_mtime: i32,
	pub st_mtime_nsec: u32,
	pub st_ctime: i32,
	pub st_ctime_nsec: u32,
	pub st_ino: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
