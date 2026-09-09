// Translated from pseudo_fs.h.
// Dependency intent: linux/fs_context.h supplies `fs_context` and the
// referenced filesystem operation types.

#[repr(C)]
pub struct pseudo_fs_context {
    pub ops: *const super_operations,
    pub eops: *const export_operations,
    pub xattr: *const *const xattr_handler,
    pub dops: *const dentry_operations,
    pub magic: ::core::ffi::c_ulong,
    pub s_d_flags: ::core::ffi::c_uint,
}

extern "C" {
    pub fn init_pseudo(
        fc: *mut fs_context,
        magic: ::core::ffi::c_ulong,
    ) -> *mut pseudo_fs_context;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
