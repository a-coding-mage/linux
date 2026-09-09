/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Generic watchdog defines. Derived from..
 *
 * Berkshire PC Watchdog Defines
 * by Ken Hollis <khollis@bitgate.com>
 */

/* Dependency intent: Linux ioctl encoding macros and integer types are supplied externally. */

pub const WATCHDOG_IOCTL_BASE: u8 = b'W';

#[repr(C)]
pub struct watchdog_info {
    pub options: u32,
    pub firmware_version: u32,
    pub identity: [u8; 32],
}

pub const WDIOC_GETSUPPORT: usize = _IOR(WATCHDOG_IOCTL_BASE, 0, watchdog_info);
pub const WDIOC_GETSTATUS: usize = _IOR(WATCHDOG_IOCTL_BASE, 1, i32);
pub const WDIOC_GETBOOTSTATUS: usize = _IOR(WATCHDOG_IOCTL_BASE, 2, i32);
pub const WDIOC_GETTEMP: usize = _IOR(WATCHDOG_IOCTL_BASE, 3, i32);
pub const WDIOC_SETOPTIONS: usize = _IOR(WATCHDOG_IOCTL_BASE, 4, i32);
pub const WDIOC_KEEPALIVE: usize = _IOR(WATCHDOG_IOCTL_BASE, 5, i32);
pub const WDIOC_SETTIMEOUT: usize = _IOWR(WATCHDOG_IOCTL_BASE, 6, i32);
pub const WDIOC_GETTIMEOUT: usize = _IOR(WATCHDOG_IOCTL_BASE, 7, i32);
pub const WDIOC_SETPRETIMEOUT: usize = _IOWR(WATCHDOG_IOCTL_BASE, 8, i32);
pub const WDIOC_GETPRETIMEOUT: usize = _IOR(WATCHDOG_IOCTL_BASE, 9, i32);
pub const WDIOC_GETTIMELEFT: usize = _IOR(WATCHDOG_IOCTL_BASE, 10, i32);

pub const WDIOF_UNKNOWN: i32 = -1;
pub const WDIOS_UNKNOWN: i32 = -1;

/* Bit masks for watchdog_info.options, GETSTATUS and GETBOOTSTATUS ioctls */
pub const WDIOF_OVERHEAT: u32 = 0x0001;
pub const WDIOF_FANFAULT: u32 = 0x0002;
pub const WDIOF_EXTERN1: u32 = 0x0004;
pub const WDIOF_EXTERN2: u32 = 0x0008;
pub const WDIOF_POWERUNDER: u32 = 0x0010;
pub const WDIOF_CARDRESET: u32 = 0x0020;
pub const WDIOF_POWEROVER: u32 = 0x0040;
pub const WDIOF_SETTIMEOUT: u32 = 0x0080;
pub const WDIOF_MAGICCLOSE: u32 = 0x0100;
pub const WDIOF_PRETIMEOUT: u32 = 0x0200;
pub const WDIOF_ALARMONLY: u32 = 0x0400;
pub const WDIOF_KEEPALIVEPING: u32 = 0x8000;

/* Bit masks for WDIOC_SETOPTIONS ioctl */
pub const WDIOS_DISABLECARD: u32 = 0x0001;
pub const WDIOS_ENABLECARD: u32 = 0x0002;
pub const WDIOS_TEMPPANIC: u32 = 0x0004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
