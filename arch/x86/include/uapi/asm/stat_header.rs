/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the surrounding translated code: asm/posix_types.h

pub const STAT_HAVE_NSEC: i32 = 1;

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct stat {
    pub st_dev: ::core::ffi::c_ulong,
    pub st_ino: ::core::ffi::c_ulong,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: ::core::ffi::c_ulong,
    pub st_size: ::core::ffi::c_ulong,
    pub st_blksize: ::core::ffi::c_ulong,
    pub st_blocks: ::core::ffi::c_ulong,
    pub st_atime: ::core::ffi::c_ulong,
    pub st_atime_nsec: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_ulong,
    pub st_mtime_nsec: ::core::ffi::c_ulong,
    pub st_ctime: ::core::ffi::c_ulong,
    pub st_ctime_nsec: ::core::ffi::c_ulong,
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

#[cfg(target_arch = "x86")]
pub unsafe fn init_struct_stat_padding(st: &mut stat) {
    st.__unused4 = 0;
    st.__unused5 = 0;
}

pub const STAT64_HAS_BROKEN_ST_INO: i32 = 1;

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct stat64 {
    pub st_dev: u64,
    pub __pad0: [u8; 4],
    pub __st_ino: ::core::ffi::c_ulong,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: ::core::ffi::c_ulong,
    pub st_gid: ::core::ffi::c_ulong,
    pub st_rdev: u64,
    pub __pad3: [u8; 4],
    pub st_size: i64,
    pub st_blksize: ::core::ffi::c_ulong,
    pub st_blocks: u64,
    pub st_atime: ::core::ffi::c_ulong,
    pub st_atime_nsec: ::core::ffi::c_ulong,
    pub st_mtime: ::core::ffi::c_ulong,
    pub st_mtime_nsec: u32,
    pub st_ctime: ::core::ffi::c_ulong,
    pub st_ctime_nsec: ::core::ffi::c_ulong,
    pub st_ino: u64,
}

#[cfg(target_arch = "x86")]
pub unsafe fn init_struct_stat64_padding(st: &mut stat64) {
    st.__pad0 = [0; 4];
    st.__pad3 = [0; 4];
}

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub struct stat {
    pub st_dev: ::kernel_types::__kernel_ulong_t,
    pub st_ino: ::kernel_types::__kernel_ulong_t,
    pub st_nlink: ::kernel_types::__kernel_ulong_t,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: u32,
    pub st_rdev: ::kernel_types::__kernel_ulong_t,
    pub st_size: ::kernel_types::__kernel_long_t,
    pub st_blksize: ::kernel_types::__kernel_long_t,
    pub st_blocks: ::kernel_types::__kernel_long_t,
    pub st_atime: ::kernel_types::__kernel_ulong_t,
    pub st_atime_nsec: ::kernel_types::__kernel_ulong_t,
    pub st_mtime: ::kernel_types::__kernel_ulong_t,
    pub st_mtime_nsec: ::kernel_types::__kernel_ulong_t,
    pub st_ctime: ::kernel_types::__kernel_ulong_t,
    pub st_ctime_nsec: ::kernel_types::__kernel_ulong_t,
    pub __unused: [::kernel_types::__kernel_long_t; 3],
}

#[cfg(not(target_arch = "x86"))]
pub unsafe fn init_struct_stat_padding(st: &mut stat) {
    st.__pad0 = 0;
    st.__unused[0] = 0;
    st.__unused[1] = 0;
    st.__unused[2] = 0;
}

#[repr(C)]
pub struct __old_kernel_stat {
    pub st_dev: u16,
    pub st_ino: u16,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: u16,
    #[cfg(target_arch = "x86")]
    pub st_size: ::core::ffi::c_ulong,
    #[cfg(target_arch = "x86")]
    pub st_atime: ::core::ffi::c_ulong,
    #[cfg(target_arch = "x86")]
    pub st_mtime: ::core::ffi::c_ulong,
    #[cfg(target_arch = "x86")]
    pub st_ctime: ::core::ffi::c_ulong,
    #[cfg(not(target_arch = "x86"))]
    pub st_size: u32,
    #[cfg(not(target_arch = "x86"))]
    pub st_atime: u32,
    #[cfg(not(target_arch = "x86"))]
    pub st_mtime: u32,
    #[cfg(not(target_arch = "x86"))]
    pub st_ctime: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
