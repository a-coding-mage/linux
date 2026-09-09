/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding UAPI header: dev_t.

pub const MINORBITS: u32 = 20;
pub const MINORMASK: u32 = (1u32 << MINORBITS) - 1;

#[macro_export]
macro_rules! MAJOR {
    ($dev:expr) => {
        (($dev >> $crate::MINORBITS) as u32)
    };
}

#[macro_export]
macro_rules! MINOR {
    ($dev:expr) => {
        (($dev & $crate::MINORMASK as _) as u32)
    };
}

#[macro_export]
macro_rules! MKDEV {
    ($ma:expr, $mi:expr) => {
        (($ma << $crate::MINORBITS) | $mi)
    };
}

// C macro preserved as a Rust macro; sprintf is supplied by the surrounding
// translated codebase.
#[macro_export]
macro_rules! print_dev_t {
    ($buffer:expr, $dev:expr) => {
        sprintf($buffer, "%u:%u\n", MAJOR!($dev), MINOR!($dev))
    };
}

#[macro_export]
macro_rules! format_dev_t {
    ($buffer:expr, $dev:expr) => {{
        sprintf($buffer, "%u:%u", MAJOR!($dev), MINOR!($dev));
        $buffer
    }};
}

/* acceptable for old filesystems */
#[inline(always)]
pub unsafe fn old_valid_dev(dev: dev_t) -> bool {
    MAJOR!(dev) < 256 && MINOR!(dev) < 256
}

#[inline(always)]
pub unsafe fn old_encode_dev(dev: dev_t) -> u16 {
    ((MAJOR!(dev) << 8) | MINOR!(dev)) as u16
}

#[inline(always)]
pub unsafe fn old_decode_dev(val: u16) -> dev_t {
    MKDEV!(((val >> 8) & 255), (val & 255)) as dev_t
}

#[inline(always)]
pub unsafe fn new_encode_dev(dev: dev_t) -> u32 {
    let major: u32 = MAJOR!(dev);
    let minor: u32 = MINOR!(dev);
    (minor & 0xff) | (major << 8) | ((minor & !0xff) << 12)
}

#[inline(always)]
pub unsafe fn new_decode_dev(dev: u32) -> dev_t {
    let major: u32 = (dev & 0xfff00) >> 8;
    let minor: u32 = (dev & 0xff) | ((dev >> 12) & 0xfff00);
    MKDEV!(major, minor) as dev_t
}

#[inline(always)]
pub unsafe fn huge_encode_dev(dev: dev_t) -> u64 {
    new_encode_dev(dev) as u64
}

#[inline(always)]
pub unsafe fn huge_decode_dev(dev: u64) -> dev_t {
    new_decode_dev(dev as u32)
}

#[inline(always)]
pub unsafe fn sysv_valid_dev(dev: dev_t) -> i32 {
    (MAJOR!(dev) < (1 << 14) && MINOR!(dev) < (1 << 18)) as i32
}

#[inline(always)]
pub unsafe fn sysv_encode_dev(dev: dev_t) -> u32 {
    MINOR!(dev) | (MAJOR!(dev) << 18)
}

#[inline(always)]
pub unsafe fn sysv_major(dev: u32) -> u32 {
    (dev >> 18) & 0x3fff
}

#[inline(always)]
pub unsafe fn sysv_minor(dev: u32) -> u32 {
    dev & 0x3ffff
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
