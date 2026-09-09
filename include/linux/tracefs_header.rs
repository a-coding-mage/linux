/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tracefs.h - a pseudo file system for activating tracing
 *
 * Based on debugfs by: 2004 Greg Kroah-Hartman <greg@kroah.com>
 *
 * Copyright (C) 2014 Red Hat Inc, author: Steven Rostedt <srostedt@redhat.com>
 *
 * tracefs is the file system that is used by the tracing infrastructure.
 */

// C dependencies: linux/fs.h, linux/seq_file.h, and linux/types.h.

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eventfs_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eventfs_inode {
    _private: [u8; 0],
}

pub type umode_t = u16;

/* CONFIG_TRACING controls the declarations below in the original header. */

/**
 * eventfs_callback - A callback function to create dynamic files in eventfs
 * @name: The name of the file that is to be created
 * @mode: return the file mode for the file (RW access, etc)
 * @data: data to pass to the created file ops
 * @fops: the file operations of the created file
 *
 * The eventfs files are dynamically created. The struct eventfs_entry array
 * is passed to eventfs_create_dir() or eventfs_create_events_dir() that will
 * be used to create the files within those directories. When a lookup
 * or access to a file within the directory is made, the struct eventfs_entry
 * array is used to find a callback() with the matching name that is being
 * referenced (for lookups, the entire array is iterated and each callback
 * will be called).
 *
 * The callback will be called with @name for the name of the file to create.
 * The callback can return less than 1 to indicate  that no file should be
 * created.
 *
 * If a file is to be created, then @mode should be populated with the file
 * mode (permissions) for which the file is created for. This would be
 * used to set the created inode i_mode field.
 *
 * The @data should be set to the data passed to the other file operations
 * (read, write, etc). Note, @data will also point to the data passed in
 * to eventfs_create_dir() or eventfs_create_events_dir(), but the callback
 * can replace the data if it chooses to. Otherwise, the original data
 * will be used for the file operation functions.
 *
 * The @fops should be set to the file operations that will be used to create
 * the inode.
 *
 * NB. This callback is called while holding internal locks of the eventfs
 *     system. The callback must not call any code that might also call into
 *     the tracefs or eventfs system or it will risk creating a deadlock.
 */
pub type eventfs_callback = unsafe extern "C" fn(
    name: *const core::ffi::c_char,
    mode: *mut umode_t,
    data: *mut *mut core::ffi::c_void,
    fops: *mut *const file_operations,
) -> core::ffi::c_int;

pub type eventfs_release = unsafe extern "C" fn(
    name: *const core::ffi::c_char,
    data: *mut core::ffi::c_void,
);

/**
 * struct eventfs_entry - dynamically created eventfs file call back handler
 * @name: Then name of the dynamic file in an eventfs directory
 * @callback: The callback to get the fops of the file when it is created
 *
 * See evenfs_callback() typedef for how to set up @callback.
 */
#[repr(C)]
pub struct eventfs_entry {
    pub name: *const core::ffi::c_char,
    pub callback: eventfs_callback,
    pub release: eventfs_release,
}

unsafe extern "C" {
    pub fn eventfs_create_events_dir(
        name: *const core::ffi::c_char,
        parent: *mut dentry,
        entries: *const eventfs_entry,
        size: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> *mut eventfs_inode;

    pub fn eventfs_create_dir(
        name: *const core::ffi::c_char,
        parent: *mut eventfs_inode,
        entries: *const eventfs_entry,
        size: core::ffi::c_int,
        data: *mut core::ffi::c_void,
    ) -> *mut eventfs_inode;

    pub fn eventfs_remove_events_dir(ei: *mut eventfs_inode);
    pub fn eventfs_remove_dir(ei: *mut eventfs_inode);

    pub fn tracefs_create_file(
        name: *const core::ffi::c_char,
        mode: umode_t,
        parent: *mut dentry,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;

    pub fn tracefs_create_dir(
        name: *const core::ffi::c_char,
        parent: *mut dentry,
    ) -> *mut dentry;

    pub fn tracefs_remove(dentry: *mut dentry);

    pub fn tracefs_create_instance_dir(
        name: *const core::ffi::c_char,
        parent: *mut dentry,
        mkdir: Option<unsafe extern "C" fn(name: *const core::ffi::c_char) -> core::ffi::c_int>,
        rmdir: Option<unsafe extern "C" fn(name: *const core::ffi::c_char) -> core::ffi::c_int>,
    ) -> *mut dentry;

    pub fn tracefs_initialized() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
