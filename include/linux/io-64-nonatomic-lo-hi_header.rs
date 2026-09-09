/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/io-64-nonatomic-lo-hi.h.
// C header dependencies: linux/io.h and asm-generic/int-ll64.h.

extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(val: u32, addr: *mut core::ffi::c_void);
    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut core::ffi::c_void);
    fn ioread32(addr: *const core::ffi::c_void) -> u32;
    fn iowrite32(val: u32, addr: *mut core::ffi::c_void);
    fn ioread32be(addr: *const core::ffi::c_void) -> u32;
    fn iowrite32be(val: u32, addr: *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn lo_hi_readq(addr: *const core::ffi::c_void) -> u64 {
    let p = addr as *const u32;
    let low = readl(p as *const core::ffi::c_void);
    let high = readl(p.add(1) as *const core::ffi::c_void);
    (low as u64).wrapping_add((high as u64) << 32)
}

#[inline]
pub unsafe fn lo_hi_writeq(val: u64, addr: *mut core::ffi::c_void) {
    writel(val as u32, addr);
    writel((val >> 32) as u32, addr.add(4));
}

#[inline]
pub unsafe fn lo_hi_readq_relaxed(addr: *const core::ffi::c_void) -> u64 {
    let p = addr as *const u32;
    let low = readl_relaxed(p as *const core::ffi::c_void);
    let high = readl_relaxed(p.add(1) as *const core::ffi::c_void);
    (low as u64).wrapping_add((high as u64) << 32)
}

#[inline]
pub unsafe fn lo_hi_writeq_relaxed(val: u64, addr: *mut core::ffi::c_void) {
    writel_relaxed(val as u32, addr);
    writel_relaxed((val >> 32) as u32, addr.add(4));
}

// #ifndef readq / writeq / readq_relaxed / writeq_relaxed
#[macro_export]
macro_rules! readq { ($addr:expr) => { $crate::lo_hi_readq($addr) }; }
#[macro_export]
macro_rules! writeq { ($val:expr, $addr:expr) => { $crate::lo_hi_writeq($val, $addr) }; }
#[macro_export]
macro_rules! readq_relaxed { ($addr:expr) => { $crate::lo_hi_readq_relaxed($addr) }; }
#[macro_export]
macro_rules! writeq_relaxed { ($val:expr, $addr:expr) => { $crate::lo_hi_writeq_relaxed($val, $addr) }; }

#[inline]
pub unsafe fn ioread64_lo_hi(addr: *const core::ffi::c_void) -> u64 {
    let low = ioread32(addr);
    let high = ioread32(addr.add(core::mem::size_of::<u32>()));
    (low as u64).wrapping_add((high as u64) << 32)
}

#[inline]
pub unsafe fn iowrite64_lo_hi(val: u64, addr: *mut core::ffi::c_void) {
    iowrite32(val as u32, addr);
    iowrite32((val >> 32) as u32, addr.add(core::mem::size_of::<u32>()));
}

#[inline]
pub unsafe fn ioread64be_lo_hi(addr: *const core::ffi::c_void) -> u64 {
    let low = ioread32be(addr.add(core::mem::size_of::<u32>()));
    let high = ioread32be(addr);
    (low as u64).wrapping_add((high as u64) << 32)
}

#[inline]
pub unsafe fn iowrite64be_lo_hi(val: u64, addr: *mut core::ffi::c_void) {
    iowrite32be(val as u32, addr.add(core::mem::size_of::<u32>()));
    iowrite32be((val >> 32) as u32, addr);
}

// The following build-time conditions preserve the original CONFIG_GENERIC_IOMAP
// and CONFIG_64BIT selection; dependent implementations are supplied elsewhere.
// ioread64_is_nonatomic, iowrite64_is_nonatomic, ioread64be_is_nonatomic,
// and iowrite64be_is_nonatomic are marker macros in the C header.
// Under CONFIG_GENERIC_IOMAP && CONFIG_64BIT, the corresponding double-underscore
// implementations are selected; otherwise the local lo_hi implementations apply.

// C preprocessor aliases, represented as Rust invocation macros. The cfg branch
// stands for CONFIG_GENERIC_IOMAP && CONFIG_64BIT.
#[cfg(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT"))]
#[macro_export]
macro_rules! ioread64 { ($addr:expr) => { __ioread64_lo_hi($addr) }; }
#[cfg(not(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT")))]
#[macro_export]
macro_rules! ioread64 { ($addr:expr) => { $crate::ioread64_lo_hi($addr) }; }

#[cfg(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT"))]
#[macro_export]
macro_rules! iowrite64 { ($val:expr, $addr:expr) => { __iowrite64_lo_hi($val, $addr) }; }
#[cfg(not(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT")))]
#[macro_export]
macro_rules! iowrite64 { ($val:expr, $addr:expr) => { $crate::iowrite64_lo_hi($val, $addr) }; }

#[cfg(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT"))]
#[macro_export]
macro_rules! ioread64be { ($addr:expr) => { __ioread64be_lo_hi($addr) }; }
#[cfg(not(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT")))]
#[macro_export]
macro_rules! ioread64be { ($addr:expr) => { $crate::ioread64be_lo_hi($addr) }; }

#[cfg(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT"))]
#[macro_export]
macro_rules! iowrite64be { ($val:expr, $addr:expr) => { __iowrite64be_lo_hi($val, $addr) }; }
#[cfg(not(all(feature = "CONFIG_GENERIC_IOMAP", feature = "CONFIG_64BIT")))]
#[macro_export]
macro_rules! iowrite64be { ($val:expr, $addr:expr) => { $crate::iowrite64be_lo_hi($val, $addr) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
