/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the Linux filesystem headers:
// #include <linux/fs_context.h>
// #include <linux/fs_parser.h>

use core::ffi::{c_char, c_int};

pub enum ovl_fs {}
pub enum ovl_config {}
pub enum fs_parameter_spec {}
pub enum constant_table {}
pub enum fs_context {}
pub enum seq_file {}
pub enum dentry {}
pub enum path {}

unsafe extern "C" {
    pub static ovl_parameter_spec: [fs_parameter_spec; 0];
    pub static ovl_parameter_redirect_dir: [constant_table; 0];
}

/// The set of options that user requested explicitly via mount options
#[repr(C)]
pub struct ovl_opt_set {
    pub metacopy: bool,
    pub redirect: bool,
    pub nfs_export: bool,
    pub index: bool,
}

pub const OVL_MAX_STACK: usize = 500;

#[repr(C)]
pub struct ovl_fs_context_layer {
    pub name: *mut c_char,
    pub path: path,
}

#[repr(C)]
pub struct ovl_fs_context {
    pub upper: path,
    pub work: path,
    pub capacity: usize,
    /// includes nr_data
    pub nr: usize,
    pub nr_data: usize,
    pub set: ovl_opt_set,
    pub lower: *mut ovl_fs_context_layer,
    /// user provided lowerdir string
    pub lowerdir_all: *mut c_char,
    pub casefold_set: bool,
}

unsafe extern "C" {
    pub fn ovl_init_fs_context(fc: *mut fs_context) -> c_int;
    pub fn ovl_free_fs(ofs: *mut ovl_fs);
    pub fn ovl_fs_params_verify(ctx: *const ovl_fs_context, config: *mut ovl_config) -> c_int;
    pub fn ovl_show_options(m: *mut seq_file, dentry: *mut dentry) -> c_int;
    pub fn ovl_xino_mode(config: *mut ovl_config) -> *const c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
