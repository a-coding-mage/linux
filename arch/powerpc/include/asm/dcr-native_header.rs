/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * (c) Copyright 2006 Benjamin Herrenschmidt, IBM Corp.
 *                    <benh@kernel.crashing.org>
 */

/* The C header's __KERNEL__ and !__ASSEMBLER__ guards are build-time
 * conditions; this Rust translation contains the guarded declarations. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcr_host_native_t {
    pub base: core::ffi::c_uint,
}

#[inline]
pub fn dcr_map_ok_native(_host: dcr_host_native_t) -> bool {
    true
}

#[macro_export]
macro_rules! dcr_map_native {
    ($dev:expr, $dcr_n:expr, $dcr_c:expr) => {{
        let _ = &$dev;
        let _ = &$dcr_c;
        $crate::dcr_host_native_t { base: $dcr_n }
    }};
}

#[macro_export]
macro_rules! dcr_unmap_native {
    ($host:expr, $dcr_c:expr) => {{
        let _ = &$host;
        let _ = &$dcr_c;
    }};
}

#[macro_export]
macro_rules! dcr_read_native {
    ($host:expr, $dcr_n:expr) => { mfdcr!($dcr_n + ($host).base) };
}

#[macro_export]
macro_rules! dcr_write_native {
    ($host:expr, $dcr_n:expr, $value:expr) => { mtdcr!($dcr_n + ($host).base, $value) };
}

/* Table based DCR accessors. */
extern "C" {
    pub fn __mtdcr(reg: core::ffi::c_uint, val: core::ffi::c_uint);
    pub fn __mfdcr(reg: core::ffi::c_uint) -> core::ffi::c_uint;
}

/* mfdcrx/mtdcrx instruction based accessors. */
#[inline]
pub unsafe fn mfdcrx(reg: core::ffi::c_uint) -> core::ffi::c_uint {
    let ret: core::ffi::c_uint;
    core::arch::asm!(
        ".long 0x7c000206 | ({0} << 21) | ({1} << 16)",
        out(reg) ret,
        in(reg) reg,
    );
    ret
}

#[inline]
pub unsafe fn mtdcrx(reg: core::ffi::c_uint, val: core::ffi::c_uint) {
    core::arch::asm!(
        ".long 0x7c000306 | ({0} << 21) | ({1} << 16)",
        in(reg) val,
        in(reg) reg,
    );
}

/* cpu_has_feature(CPU_FTR_INDEXED_DCR) and likely() are supplied by the
 * surrounding PowerPC translation. */
#[macro_export]
macro_rules! mfdcr {
    ($rn:expr) => {{
        let rn: core::ffi::c_uint = $rn;
        if cpu_has_feature(CPU_FTR_INDEXED_DCR) {
            unsafe { $crate::mfdcrx(rn) }
        } else {
            unsafe { $crate::__mfdcr(rn) }
        }
    }};
}

#[macro_export]
macro_rules! mtdcr {
    ($rn:expr, $v:expr) => {{
        let rn: core::ffi::c_uint = $rn;
        let v: core::ffi::c_uint = $v;
        if cpu_has_feature(CPU_FTR_INDEXED_DCR) {
            unsafe { $crate::mtdcrx(rn, v) }
        } else {
            unsafe { $crate::__mtdcr(rn, v) }
        }
    }};
}

/* R/W of indirect DCRs make use of standard naming conventions for DCRs. */
extern "C" {
    pub static mut dcr_ind_lock: spinlock_t;
}

#[inline]
pub unsafe fn __mfdcri(base_addr: i32, base_data: i32, reg: i32) -> core::ffi::c_uint {
    let mut flags: core::ffi::c_ulong = 0;
    let val;
    spin_lock_irqsave(&mut dcr_ind_lock, &mut flags);
    if cpu_has_feature(CPU_FTR_INDEXED_DCR) {
        mtdcrx(base_addr as _, reg as _);
        val = mfdcrx(base_data as _);
    } else {
        __mtdcr(base_addr as _, reg as _);
        val = __mfdcr(base_data as _);
    }
    spin_unlock_irqrestore(&mut dcr_ind_lock, flags);
    val
}

#[inline]
pub unsafe fn __mtdcri(base_addr: i32, base_data: i32, reg: i32, val: core::ffi::c_uint) {
    let mut flags: core::ffi::c_ulong = 0;
    spin_lock_irqsave(&mut dcr_ind_lock, &mut flags);
    if cpu_has_feature(CPU_FTR_INDEXED_DCR) {
        mtdcrx(base_addr as _, reg as _);
        mtdcrx(base_data as _, val);
    } else {
        __mtdcr(base_addr as _, reg as _);
        __mtdcr(base_data as _, val);
    }
    spin_unlock_irqrestore(&mut dcr_ind_lock, flags);
}

#[inline]
pub unsafe fn __dcri_clrset(base_addr: i32, base_data: i32, reg: i32,
                             clr: core::ffi::c_uint, set: core::ffi::c_uint) {
    let mut flags: core::ffi::c_ulong = 0;
    let val;
    spin_lock_irqsave(&mut dcr_ind_lock, &mut flags);
    if cpu_has_feature(CPU_FTR_INDEXED_DCR) {
        mtdcrx(base_addr as _, reg as _);
        val = (mfdcrx(base_data as _) & !clr) | set;
        mtdcrx(base_data as _, val);
    } else {
        __mtdcr(base_addr as _, reg as _);
        val = (__mfdcr(base_data as _) & !clr) | set;
        __mtdcr(base_data as _, val);
    }
    spin_unlock_irqrestore(&mut dcr_ind_lock, flags);
}

/* The C token-pasting forms require the corresponding DCRN_* constants from
 * the including translation; Rust callers provide the two resolved addresses. */
#[macro_export]
macro_rules! mfdcri { ($base_addr:expr, $base_data:expr, $reg:expr) => {
    unsafe { $crate::__mfdcri($base_addr, $base_data, $reg) }
} }
#[macro_export]
macro_rules! mtdcri { ($base_addr:expr, $base_data:expr, $reg:expr, $data:expr) => {
    unsafe { $crate::__mtdcri($base_addr, $base_data, $reg, $data) }
} }
#[macro_export]
macro_rules! dcri_clrset { ($base_addr:expr, $base_data:expr, $reg:expr, $clr:expr, $set:expr) => {
    unsafe { $crate::__dcri_clrset($base_addr, $base_data, $reg, $clr, $set) }
} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
