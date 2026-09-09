/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/bug.h and linux/kern_levels.h provide the referenced
// warning/build-check macros and KERN_* constants.

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn netdev_printk(level: *const ::core::ffi::c_char,
                         dev: *const net_device,
                         format: *const ::core::ffi::c_char,
                         ...);
    pub fn netdev_emerg(dev: *const net_device,
                        format: *const ::core::ffi::c_char,
                        ...);
    pub fn netdev_alert(dev: *const net_device,
                        format: *const ::core::ffi::c_char,
                        ...);
    pub fn netdev_crit(dev: *const net_device,
                       format: *const ::core::ffi::c_char,
                       ...);
    pub fn netdev_err(dev: *const net_device,
                      format: *const ::core::ffi::c_char,
                      ...);
    pub fn netdev_warn(dev: *const net_device,
                       format: *const ::core::ffi::c_char,
                       ...);
    pub fn netdev_notice(dev: *const net_device,
                         format: *const ::core::ffi::c_char,
                         ...);
    pub fn netdev_info(dev: *const net_device,
                       format: *const ::core::ffi::c_char,
                       ...);
}

#[macro_export]
macro_rules! netdev_level_once {
    ($level:expr, $dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        static mut __PRINT_ONCE: bool = false;
        unsafe {
            if !__PRINT_ONCE {
                __PRINT_ONCE = true;
                $crate::netdev_printk($level, $dev, $fmt $(, $args)*);
            }
        }
    }};
}

#[macro_export]
macro_rules! netdev_emerg_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_EMERG, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_alert_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_ALERT, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_crit_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_CRIT, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_err_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_ERR, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_warn_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_WARNING, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_notice_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_NOTICE, $dev, $fmt $(, $args)*); }; }
#[macro_export]
macro_rules! netdev_info_once { ($dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => { $crate::netdev_level_once!(KERN_INFO, $dev, $fmt $(, $args)*); }; }

// CONFIG_DYNAMIC_DEBUG / DEBUG branches are preserved by these direct macro
// forms; dynamic_netdev_dbg and the KERN_* constants are external dependencies.
#[macro_export]
macro_rules! netdev_dbg {
    ($dev:expr, $format:expr $(, $args:expr)* $(,)?) => {
        $crate::netdev_printk(KERN_DEBUG, $dev, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! netdev_vdbg { ($dev:expr, $format:expr $(, $args:expr)* $(,)?) => { $crate::netdev_dbg!($dev, $format $(, $args)*); }; }

// netif printk helpers, similar to netdev_printk. The token-pasted
// netif_msg_* and netdev_* symbols remain external dependencies.
#[macro_export]
macro_rules! netif_printk {
    ($priv:expr, $type:ident, $level:expr, $dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        if netif_msg_$type($priv) {
            $crate::netdev_printk($level, $dev, $fmt $(, $args)*);
        }
    }};
}

#[macro_export]
macro_rules! netif_level {
    ($level:ident, $priv:expr, $type:ident, $dev:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        if netif_msg_$type($priv) {
            netdev_$level($dev, $fmt $(, $args)*);
        }
    }};
}

#[macro_export] macro_rules! netif_emerg { ($($x:tt)*) => { $crate::netif_level!(emerg, $($x)*); }; }
#[macro_export] macro_rules! netif_alert { ($($x:tt)*) => { $crate::netif_level!(alert, $($x)*); }; }
#[macro_export] macro_rules! netif_crit { ($($x:tt)*) => { $crate::netif_level!(crit, $($x)*); }; }
#[macro_export] macro_rules! netif_err { ($($x:tt)*) => { $crate::netif_level!(err, $($x)*); }; }
#[macro_export] macro_rules! netif_warn { ($($x:tt)*) => { $crate::netif_level!(warn, $($x)*); }; }
#[macro_export] macro_rules! netif_notice { ($($x:tt)*) => { $crate::netif_level!(notice, $($x)*); }; }
#[macro_export] macro_rules! netif_info { ($($x:tt)*) => { $crate::netif_level!(info, $($x)*); }; }

#[macro_export]
macro_rules! netif_dbg {
    ($priv:expr, $type:ident, $dev:expr, $format:expr $(, $args:expr)* $(,)?) => {{
        if netif_msg_$type($priv) {
            $crate::netdev_printk(KERN_DEBUG, $dev, $format $(, $args)*);
        }
    }};
}

#[macro_export]
macro_rules! netif_cond_dbg {
    ($priv:expr, $type:ident, $netdev:expr, $cond:expr, $level:ident, $fmt:expr $(, $args:expr)* $(,)?) => {{
        if $cond { $crate::netif_dbg!($priv, $type, $netdev, $fmt $(, $args)*); }
        else { $crate::netif_$level!($priv, $type, $netdev, $fmt $(, $args)*); }
    }};
}

#[macro_export]
macro_rules! netif_vdbg { ($($x:tt)*) => { $crate::netif_dbg!($($x)*); }; }

// CONFIG_DEBUG_NET selects WARN_ON_ONCE/WARN_ONCE; otherwise these map to
// BUILD_BUG_ON_INVALID, supplied by linux/bug.h.
#[macro_export] macro_rules! DEBUG_NET_WARN_ON_ONCE { ($cond:expr) => { WARN_ON_ONCE($cond) }; }
#[macro_export] macro_rules! DEBUG_NET_WARN_ONCE { ($cond:expr $(, $format:expr $(, $args:expr)*)?) => { WARN_ONCE($cond $(, $format $(, $args)*)?) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
