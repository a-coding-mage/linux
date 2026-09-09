/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * {read,write}{b,w,l,q} based on arch/arm64/include/asm/io.h
 * which was based on arch/arm/include/io.h
 */

/* linux/types.h, asm/fence.h, and asm/mmiowb.h are supplied dependencies. */

/* Generic IO read/write. These perform native-endian accesses. */
#[inline]
pub unsafe fn __raw_writeb(val: u8, addr: *mut core::ffi::c_void) {
    core::arch::asm!("sb {0}, 0({1})", in(reg) val, in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn __raw_writew(val: u16, addr: *mut core::ffi::c_void) {
    core::arch::asm!("sh {0}, 0({1})", in(reg) val, in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn __raw_writel(val: u32, addr: *mut core::ffi::c_void) {
    core::arch::asm!("sw {0}, 0({1})", in(reg) val, in(reg) addr, options(nostack, preserves_flags));
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn __raw_writeq(val: u64, addr: *mut core::ffi::c_void) {
    core::arch::asm!("sd {0}, 0({1})", in(reg) val, in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 {
    let val: u8;
    core::arch::asm!("lb {0}, 0({1})", out(reg) val, in(reg) addr, options(nostack, preserves_flags));
    val
}

#[inline]
pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 {
    let val: u16;
    core::arch::asm!("lh {0}, 0({1})", out(reg) val, in(reg) addr, options(nostack, preserves_flags));
    val
}

#[inline]
pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 {
    let val: u32;
    core::arch::asm!("lw {0}, 0({1})", out(reg) val, in(reg) addr, options(nostack, preserves_flags));
    val
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn __raw_readq(addr: *const core::ffi::c_void) -> u64 {
    let val: u64;
    core::arch::asm!("ld {0}, 0({1})", out(reg) val, in(reg) addr, options(nostack, preserves_flags));
    val
}

/* Unordered I/O memory access primitives. */
#[macro_export]
macro_rules! readb_cpu { ($c:expr) => {{ unsafe { $crate::__raw_readb($c) } }}; }
#[macro_export]
macro_rules! readw_cpu { ($c:expr) => {{ u16::from_le(unsafe { $crate::__raw_readw($c) }) }}; }
#[macro_export]
macro_rules! readl_cpu { ($c:expr) => {{ u32::from_le(unsafe { $crate::__raw_readl($c) }) }}; }
#[macro_export]
macro_rules! writeb_cpu { ($v:expr, $c:expr) => {{ unsafe { $crate::__raw_writeb($v, $c) } }}; }
#[macro_export]
macro_rules! writew_cpu { ($v:expr, $c:expr) => {{ unsafe { $crate::__raw_writew(u16::to_le($v), $c) } }}; }
#[macro_export]
macro_rules! writel_cpu { ($v:expr, $c:expr) => {{ unsafe { $crate::__raw_writel(u32::to_le($v), $c) } }}; }

#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! readq_cpu { ($c:expr) => {{ u64::from_le(unsafe { $crate::__raw_readq($c) }) }}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! writeq_cpu { ($v:expr, $c:expr) => {{ unsafe { $crate::__raw_writeq(u64::to_le($v), $c) } }}; }

#[macro_export] macro_rules! __io_rbr { () => {}; }
#[macro_export] macro_rules! __io_rar { () => {}; }
#[macro_export] macro_rules! __io_rbw { () => {}; }
#[macro_export] macro_rules! __io_raw { () => {}; }

#[macro_export]
macro_rules! readb_relaxed { ($c:expr) => {{ $crate::__io_rbr!(); let v = $crate::readb_cpu!($c); $crate::__io_rar!(); v }}; }
#[macro_export]
macro_rules! readw_relaxed { ($c:expr) => {{ $crate::__io_rbr!(); let v = $crate::readw_cpu!($c); $crate::__io_rar!(); v }}; }
#[macro_export]
macro_rules! readl_relaxed { ($c:expr) => {{ $crate::__io_rbr!(); let v = $crate::readl_cpu!($c); $crate::__io_rar!(); v }}; }
#[macro_export]
macro_rules! writeb_relaxed { ($v:expr, $c:expr) => {{ $crate::__io_rbw!(); $crate::writeb_cpu!($v, $c); $crate::__io_raw!(); }}; }
#[macro_export]
macro_rules! writew_relaxed { ($v:expr, $c:expr) => {{ $crate::__io_rbw!(); $crate::writew_cpu!($v, $c); $crate::__io_raw!(); }}; }
#[macro_export]
macro_rules! writel_relaxed { ($v:expr, $c:expr) => {{ $crate::__io_rbw!(); $crate::writel_cpu!($v, $c); $crate::__io_raw!(); }}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! readq_relaxed { ($c:expr) => {{ $crate::__io_rbr!(); let v = $crate::readq_cpu!($c); $crate::__io_rar!(); v }}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! writeq_relaxed { ($v:expr, $c:expr) => {{ $crate::__io_rbw!(); $crate::writeq_cpu!($v, $c); $crate::__io_raw!(); }}; }

/* RISC-V ordering primitives; RISCV_FENCE and mmiowb_set_pending are external dependencies. */
#[macro_export] macro_rules! __io_br { () => {}; }
#[macro_export] macro_rules! __io_ar { ($v:expr) => { RISCV_FENCE!(i, ir); }; }
#[macro_export] macro_rules! __io_bw { () => { RISCV_FENCE!(w, o); }; }
#[macro_export] macro_rules! __io_aw { () => { unsafe { mmiowb_set_pending() }; }; }

#[macro_export]
macro_rules! readb { ($c:expr) => {{ $crate::__io_br!(); let v = $crate::readb_cpu!($c); $crate::__io_ar!(v); v }}; }
#[macro_export]
macro_rules! readw { ($c:expr) => {{ $crate::__io_br!(); let v = $crate::readw_cpu!($c); $crate::__io_ar!(v); v }}; }
#[macro_export]
macro_rules! readl { ($c:expr) => {{ $crate::__io_br!(); let v = $crate::readl_cpu!($c); $crate::__io_ar!(v); v }}; }
#[macro_export]
macro_rules! writeb { ($v:expr, $c:expr) => {{ $crate::__io_bw!(); $crate::writeb_cpu!($v, $c); $crate::__io_aw!(); }}; }
#[macro_export]
macro_rules! writew { ($v:expr, $c:expr) => {{ $crate::__io_bw!(); $crate::writew_cpu!($v, $c); $crate::__io_aw!(); }}; }
#[macro_export]
macro_rules! writel { ($v:expr, $c:expr) => {{ $crate::__io_bw!(); $crate::writel_cpu!($v, $c); $crate::__io_aw!(); }}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! readq { ($c:expr) => {{ $crate::__io_br!(); let v = $crate::readq_cpu!($c); $crate::__io_ar!(v); v }}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! writeq { ($v:expr, $c:expr) => {{ $crate::__io_bw!(); $crate::writeq_cpu!($v, $c); $crate::__io_aw!(); }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
