/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the Linux errno and proc_fs dependencies.

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[cfg(feature = "CONFIG_INOTIFY_USER")]
unsafe extern "C" {
    pub fn inotify_show_fdinfo(m: *mut seq_file, f: *mut file);
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[cfg(feature = "CONFIG_FANOTIFY")]
unsafe extern "C" {
    pub fn fanotify_show_fdinfo(m: *mut seq_file, f: *mut file);
}

// When CONFIG_PROC_FS is disabled, the C header defines these names as NULL.
#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub const inotify_show_fdinfo: Option<unsafe extern "C" fn(*mut seq_file, *mut file)> = None;

#[cfg(not(feature = "CONFIG_PROC_FS"))]
pub const fanotify_show_fdinfo: Option<unsafe extern "C" fn(*mut seq_file, *mut file)> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
