/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/vfsdebug.h. The C header's include and header guard
// are intentionally omitted; required symbols are supplied by dependencies.

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[cfg(CONFIG_DEBUG_VFS)]
extern "C" {
    pub fn dump_inode(inode: *mut inode, reason: *const ::core::ffi::c_char);
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_BUG_ON {
    ($cond:expr) => {{ BUG_ON!($cond) }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_WARN_ON {
    ($cond:expr) => {{ let _ = WARN_ON!($cond); }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_WARN_ON_ONCE {
    ($cond:expr) => {{ let _ = WARN_ON_ONCE!($cond); }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_WARN_ONCE {
    ($cond:expr, $($format:tt)*) => {{ let _ = WARN_ONCE!($cond, $($format)*); }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_WARN {
    ($cond:expr, $($format:tt)*) => {{ let _ = WARN!($cond, $($format)*); }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_BUG_ON_INODE {
    ($cond:expr, $inode:expr) => {{
        if ($cond) {
            let __reason = concat!("VFS_BUG_ON_INODE(", stringify!($cond), ")");
            unsafe {
                dump_inode($inode, concat!(__reason, "\0").as_ptr() as *const ::core::ffi::c_char);
            }
            BUG_ON!(true);
        }
    }};
}

#[cfg(CONFIG_DEBUG_VFS)]
macro_rules! VFS_WARN_ON_INODE {
    ($cond:expr, $inode:expr) => {{
        let __ret_warn: i32 = if $cond { 1 } else { 0 };
        if __ret_warn != 0 {
            let __reason = concat!("VFS_WARN_ON_INODE(", stringify!($cond), ")");
            unsafe {
                dump_inode($inode, concat!(__reason, "\0").as_ptr() as *const ::core::ffi::c_char);
            }
            WARN_ON!(true);
        }
        __ret_warn != 0
    }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_BUG_ON {
    ($cond:expr) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_WARN_ON {
    ($cond:expr) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_WARN_ON_ONCE {
    ($cond:expr) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_WARN_ONCE {
    ($cond:expr, $($format:tt)*) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_WARN {
    ($cond:expr, $($format:tt)*) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_BUG_ON_INODE {
    ($cond:expr, $inode:expr) => {{ VFS_BUG_ON!($cond) }};
}

#[cfg(not(CONFIG_DEBUG_VFS))]
macro_rules! VFS_WARN_ON_INODE {
    ($cond:expr, $inode:expr) => {{ BUILD_BUG_ON_INVALID!($cond) }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
