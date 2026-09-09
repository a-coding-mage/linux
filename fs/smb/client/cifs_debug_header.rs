/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2000,2002
 *   Modified by Steve French (sfrench@us.ibm.com)
 */

/* C header guard: _H_CIFS_DEBUG */

/* C preprocessor pr_fmt override. */
#[macro_export]
macro_rules! pr_fmt {
    ($fmt:expr) => { concat!("CIFS: ", $fmt) };
}

extern "C" {
    pub fn cifs_dump_mem(label: *mut ::core::ffi::c_char,
                         data: *mut ::core::ffi::c_void,
                         length: ::core::ffi::c_int);
    pub fn cifs_dump_mids(server: *mut TCP_Server_Info);
    pub static mut traceSMB: bool;
    pub fn dump_smb(buf: *mut ::core::ffi::c_void,
                    smb_buf_length: ::core::ffi::c_int);
    pub static mut cifsFYI: ::core::ffi::c_int;
}

/* Supplied by the surrounding implementation. */
#[repr(C)]
pub struct TCP_Server_Info {
    _private: [u8; 0],
}

pub const CIFS_INFO: ::core::ffi::c_int = 0x01;
pub const CIFS_RC: ::core::ffi::c_int = 0x02;
pub const CIFS_TIMER: ::core::ffi::c_int = 0x04;
pub const VFS: ::core::ffi::c_int = 1;
pub const FYI: ::core::ffi::c_int = 2;

#[cfg(CONFIG_CIFS_DEBUG2)]
pub const NOISY: ::core::ffi::c_int = 4;
#[cfg(not(CONFIG_CIFS_DEBUG2))]
pub const NOISY: ::core::ffi::c_int = 0;
pub const ONCE: ::core::ffi::c_int = 8;

/* The following macros correspond to the CONFIG_CIFS_DEBUG branch. */
#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_info_func {
    ($ratefunc:ident, $fmt:expr $(, $args:expr)*) => {
        $crate::paste_pr_info!($ratefunc, $fmt $(, $args)*)
    };
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_info {
    ($fmt:expr $(, $args:expr)*) => {
        $crate::cifs_info_func!(ratelimited, $fmt $(, $args)*)
    };
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_dbg_func {
    ($ratefunc:ident, $ty:expr, $fmt:expr $(, $args:expr)*) => {{
        if (($ty) & $crate::FYI) != 0 && unsafe { ($crate::cifsFYI & $crate::CIFS_INFO) != 0 } {
            $crate::paste_pr_debug!($ratefunc, concat!("%s: ", $fmt), file!() $(, $args)*)
        } else if (($ty) & $crate::VFS) != 0 {
            $crate::paste_pr_err!($ratefunc, concat!("VFS: ", $fmt) $(, $args)*)
        } else if (($ty) & $crate::NOISY) != 0 && $crate::NOISY != 0 {
            $crate::paste_pr_debug!($ratefunc, $fmt $(, $args)*)
        }
    }};
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_dbg {
    ($ty:expr, $fmt:expr $(, $args:expr)*) => {{
        if (($ty) & $crate::ONCE) != 0 {
            $crate::cifs_dbg_func!(once, $ty, $fmt $(, $args)*)
        } else {
            $crate::cifs_dbg_func!(ratelimited, $ty, $fmt $(, $args)*)
        }
    }};
}

/* Server/tcon debug macros retain their C-side locking and context semantics. */
#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_server_dbg {
    ($ty:expr, $fmt:expr $(, $args:expr)*) => {
        $crate::cifs_server_dbg_func!(if (($ty) & $crate::ONCE) != 0 { once } else { ratelimited }, $ty, $fmt $(, $args)*)
    };
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_server_dbg_func {
    ($ratefunc:ident, $ty:expr, $fmt:expr $(, $args:expr)*) => {{
        /* spin_lock(&server->srv_lock) / spin_unlock(&server->srv_lock) are external C operations. */
        if (($ty) & $crate::FYI) != 0 && unsafe { ($crate::cifsFYI & $crate::CIFS_INFO) != 0 } {
            $crate::paste_pr_debug!($ratefunc, concat!("%s: \\\\%s ", $fmt), file!(), unsafe { server.hostname } $(, $args)*)
        } else if (($ty) & $crate::VFS) != 0 {
            $crate::paste_pr_err!($ratefunc, concat!("VFS: \\\\%s ", $fmt), unsafe { server.hostname } $(, $args)*)
        } else if (($ty) & $crate::NOISY) != 0 && $crate::NOISY != 0 {
            $crate::paste_pr_debug!($ratefunc, concat!("\\\\%s ", $fmt), unsafe { server.hostname } $(, $args)*)
        }
    }};
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_tcon_dbg {
    ($ty:expr, $fmt:expr $(, $args:expr)*) => {
        /* Direct translation of cifs_tcon_dbg_func; tcon/tree_name are external C context. */
        if (($ty) & $crate::ONCE) != 0 { $crate::cifs_tcon_dbg_func!(once, $ty, $fmt $(, $args)*) }
        else { $crate::cifs_tcon_dbg_func!(ratelimited, $ty, $fmt $(, $args)*) }
    };
}

#[cfg(CONFIG_CIFS_DEBUG)]
#[macro_export]
macro_rules! cifs_tcon_dbg_func {
    ($ratefunc:ident, $ty:expr, $fmt:expr $(, $args:expr)*) => {{
        /* const char *tn = tcon && tcon->tree_name ? tcon->tree_name : ""; */
        /* Logging and tcon context are supplied by the surrounding C-compatible code. */
        if (($ty) & $crate::FYI) != 0 && unsafe { ($crate::cifsFYI & $crate::CIFS_INFO) != 0 } { $crate::paste_pr_debug!($ratefunc, concat!("%s: %s ", $fmt), file!(), tn $(, $args)*) }
        else if (($ty) & $crate::VFS) != 0 { $crate::paste_pr_err!($ratefunc, concat!("VFS: %s ", $fmt), tn $(, $args)*) }
        else if (($ty) & $crate::NOISY) != 0 && $crate::NOISY != 0 { $crate::paste_pr_debug!($ratefunc, concat!("%s ", $fmt), tn $(, $args)*) }
    }};
}

/* CONFIG_CIFS_DEBUG disabled: retain the intentionally unreachable logging calls. */
#[cfg(not(CONFIG_CIFS_DEBUG))]
#[macro_export]
macro_rules! cifs_dbg { ($ty:expr, $fmt:expr $(, $args:expr)*) => {{ if false { $crate::pr_debug!($fmt $(, $args)*) } }}; }
#[cfg(not(CONFIG_CIFS_DEBUG))]
#[macro_export]
macro_rules! cifs_server_dbg { ($ty:expr, $fmt:expr $(, $args:expr)*) => {{ if false { $crate::pr_debug!(concat!("\\\\%s ", $fmt), server.hostname $(, $args)*) } }}; }
#[cfg(not(CONFIG_CIFS_DEBUG))]
#[macro_export]
macro_rules! cifs_tcon_dbg { ($ty:expr, $fmt:expr $(, $args:expr)*) => {{ if false { $crate::pr_debug!(concat!("%s ", $fmt), tcon.tree_name $(, $args)*) } }}; }
#[cfg(not(CONFIG_CIFS_DEBUG))]
#[macro_export]
macro_rules! cifs_info { ($fmt:expr $(, $args:expr)*) => { $crate::pr_info!($fmt $(, $args)*) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
