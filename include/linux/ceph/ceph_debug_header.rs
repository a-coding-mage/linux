/* SPDX-License-Identifier: GPL-2.0 */

// Translated from ceph_debug.h. C header guards and includes are omitted;
// referenced kernel symbols are supplied by the surrounding translation.

#[macro_export]
macro_rules! pr_fmt {
    ($fmt:expr) => {
        concat!(KBUILD_MODNAME, ": ", $fmt)
    };
}

// CONFIG_CEPH_LIB_PRETTYDEBUG selects the corresponding branch at build time.
#[cfg(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG")]
#[cfg(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG"))]
#[macro_export]
macro_rules! dout {
    ($fmt:expr $(, $arg:expr)*) => {
        pr_debug!(
            concat!("%.*s %12.12s:%-4d : ", $fmt),
            8 - core::mem::size_of::<decltype!(KBUILD_MODNAME)>() as i32,
            "    ",
            kbasename!(file!()),
            line!()
            $(, $arg)*
        )
    };
}

#[cfg(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG")]
#[cfg(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG"))]
#[macro_export]
macro_rules! doutc {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_debug!(
            concat!("%.*s %12.12s:%-4d : [%pU %llu] ", $fmt),
            8 - core::mem::size_of::<decltype!(KBUILD_MODNAME)>() as i32,
            "    ",
            kbasename!(file!()),
            line!(),
            &$client->fsid,
            $client->monc.auth->global_id
            $(, $arg)*
        )
    };
}

// Faux printk calls retain compiler-warning behavior when debug output is disabled.
#[cfg(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG")]
#[cfg(not(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG")))]
#[macro_export]
macro_rules! dout {
    ($fmt:expr $(, $arg:expr)*) => {
        no_printk!(concat!(KERN_DEBUG, $fmt) $(, $arg)*)
    };
}

#[cfg(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG")]
#[cfg(not(any(feature = "DEBUG", feature = "CONFIG_DYNAMIC_DEBUG")))]
#[macro_export]
macro_rules! doutc {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        no_printk!(concat!(KERN_DEBUG, "[%pU %llu] ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

// Otherwise, simply wrap pr_debug.
#[cfg(not(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG"))]
#[macro_export]
macro_rules! dout {
    ($fmt:expr $(, $arg:expr)*) => {
        pr_debug!(concat!(" ", $fmt) $(, $arg)*)
    };
}

#[cfg(not(feature = "CONFIG_CEPH_LIB_PRETTYDEBUG"))]
#[macro_export]
macro_rules! doutc {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_debug!(concat!(" [%pU %llu] %s: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id, module_path!() $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_notice_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_notice!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_info_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_info!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_warn_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_warn!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_warn_once_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_warn_once!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_err_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_err!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_warn_ratelimited_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_warn_ratelimited!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

#[macro_export]
macro_rules! pr_err_ratelimited_client {
    ($client:expr, $fmt:expr $(, $arg:expr)*) => {
        pr_err_ratelimited!(concat!("[%pU %llu]: ", $fmt),
            &$client->fsid, $client->monc.auth->global_id $(, $arg)*)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
