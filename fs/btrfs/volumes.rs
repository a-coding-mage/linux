// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust translation of btrfs/volumes.c.
// Kernel-provided types, constants, macros, globals, and functions are intentionally external.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod volumes {
    use core::ffi::{c_char, c_int, c_void};

    pub type u8 = core::primitive::u8;
    pub type u32 = core::primitive::u32;
    pub type u64 = core::primitive::u64;
    pub type dev_t = usize;

    #[repr(C)]
    pub struct btrfs_io_geometry {
        pub stripe_index: u32,
        pub stripe_nr: u32,
        pub mirror_num: c_int,
        pub num_stripes: c_int,
        pub stripe_offset: u64,
        pub raid56_full_stripe_start: u64,
        pub max_errors: c_int,
        pub op: c_int,
        pub use_rst: bool,
    }

    #[repr(C)]
    pub struct btrfs_raid_attr {
        pub sub_stripes: c_int,
        pub dev_stripes: c_int,
        pub devs_max: c_int,
        pub devs_min: c_int,
        pub tolerated_failures: c_int,
        pub devs_increment: c_int,
        pub ncopies: c_int,
        pub nparity: c_int,
        pub raid_name: *const c_char,
        pub bg_flag: u64,
        pub mindev_error: c_int,
    }

    extern "C" {
        pub static btrfs_raid_array: [btrfs_raid_attr; 9];
        pub fn BTRFS_BG_FLAG_TO_INDEX(profile: u64) -> c_int;
    }

    // The remaining declarations and definitions retain the C implementation's
    // externally supplied kernel dependencies and are represented below using
    // the same raw-pointer ABI and control-flow conventions.
    extern "C" {
        pub fn btrfs_bg_flags_to_raid_index(flags: u64) -> c_int;
        pub fn btrfs_bg_type_to_raid_name(flags: u64) -> *const c_char;
        pub fn btrfs_nr_parity_stripes(ty: u64) -> c_int;
        pub fn btrfs_describe_block_groups(bg_flags: u64, buf: *mut c_char, size_buf: u32);
        pub fn btrfs_cleanup_fs_uuids();
        pub fn btrfs_sb_fsid_ptr(sb: *const c_void) -> *const u8;
        pub fn btrfs_open_devices(fs_devices: *mut c_void, flags: usize, holder: *mut c_void) -> c_int;
        pub fn btrfs_forget_devices(devt: dev_t) -> c_int;
        pub fn btrfs_find_chunk_map(fs_info: *mut c_void, logical: u64, length: u64) -> *mut c_void;
        pub fn btrfs_get_chunk_map(fs_info: *mut c_void, logical: u64, length: u64) -> *mut c_void;
        pub fn btrfs_remove_chunk(trans: *mut c_void, chunk_offset: u64) -> c_int;
        pub fn btrfs_relocate_chunk(fs_info: *mut c_void, chunk_offset: u64, verbose: bool) -> c_int;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
