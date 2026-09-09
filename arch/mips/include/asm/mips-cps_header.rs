/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Translated from mips-cps.h.  The included kernel definitions and symbols
// are supplied by other translation units.

extern "C" {
    pub fn __cps_access_bad_size() -> usize;
    pub static mut mips_cm_is64: bool;
}

#[macro_export]
macro_rules! cps_accessor_a {
    ($unit:ident, $off:expr, $name:ident) => {
        #[inline]
        pub unsafe fn addr_$unit\_$name() -> *mut core::ffi::c_void {
            mips_$unit\_base.add($off as usize)
        }
    };
}

// C's token-pasting accessors are retained as declarative macro templates;
// concrete expansions require the corresponding unit/base symbols.
#[macro_export]
macro_rules! cps_accessor_r {
    ($unit:ident, 32, $name:ident) => {
        #[inline]
        pub unsafe fn read_$unit\_$name() -> u32 {
            __raw_readl(addr_$unit\_$name())
        }
    };
    ($unit:ident, 64, $name:ident) => {
        #[inline]
        pub unsafe fn read_$unit\_$name() -> u64 {
            if mips_cm_is64 { __raw_readq(addr_$unit\_$name()) }
            else {
                let mut val64 = __raw_readl(addr_$unit\_$name().add(4)) as u64;
                val64 <<= 32;
                val64 |= __raw_readl(addr_$unit\_$name()) as u64;
                val64
            }
        }
    };
}

#[macro_export]
macro_rules! cps_accessor_w {
    ($unit:ident, 32, $name:ident) => {
        #[inline]
        pub unsafe fn write_$unit\_$name(val: u32) { __raw_writel(val, addr_$unit\_$name()); }
    };
    ($unit:ident, 64, $name:ident) => {
        #[inline]
        pub unsafe fn write_$unit\_$name(val: u64) {
            if mips_cm_is64 { __raw_writeq(val, addr_$unit\_$name()); }
            else {
                __raw_writel((val >> 32) as u32, addr_$unit\_$name().add(4));
                __raw_writel(val as u32, addr_$unit\_$name());
            }
        }
    };
}

#[macro_export]
macro_rules! cps_accessor_m {
    ($unit:ident, $sz:tt, $name:ident) => {
        #[inline]
        pub unsafe fn change_$unit\_$name(mask: u$sz, val: u$sz) {
            let mut reg_val = read_$unit\_$name();
            reg_val &= !mask;
            reg_val |= val;
            write_$unit\_$name(reg_val);
        }
        #[inline]
        pub unsafe fn set_$unit\_$name(val: u$sz) { change_$unit\_$name(val, val); }
        #[inline]
        pub unsafe fn clear_$unit\_$name(val: u$sz) { change_$unit\_$name(val, 0); }
    };
}

// CPS_ACCESSOR_RO/WO/RW are represented by composing the accessor macros.
#[macro_export]
macro_rules! cps_accessor_ro { ($unit:ident, $sz:tt, $off:expr, $name:ident) => {
    cps_accessor_a!($unit, $off, $name); cps_accessor_r!($unit, $sz, $name);
} }
#[macro_export]
macro_rules! cps_accessor_wo { ($unit:ident, $sz:tt, $off:expr, $name:ident) => {
    cps_accessor_a!($unit, $off, $name); cps_accessor_w!($unit, $sz, $name);
} }
#[macro_export]
macro_rules! cps_accessor_rw { ($unit:ident, $sz:tt, $off:expr, $name:ident) => {
    cps_accessor_a!($unit, $off, $name); cps_accessor_r!($unit, $sz, $name);
    cps_accessor_w!($unit, $sz, $name); cps_accessor_m!($unit, $sz, $name);
} }

#[inline]
pub unsafe fn mips_cps_numclusters() -> u32 {
    if mips_cm_revision() < CM_REV_CM3_5 { 1 }
    else { field_get(CM_GCR_CONFIG_NUM_CLUSTERS, read_gcr_config()) }
}

#[inline]
pub unsafe fn mips_cps_cluster_config(cluster: u32) -> u64 {
    let config;
    if mips_cm_revision() < CM_REV_CM3_5 {
        warn_on(cluster != 0);
        config = read_gcr_config();
    } else {
        mips_cm_lock_other(cluster, 0, 0, CM_GCR_Cx_OTHER_BLOCK_GLOBAL);
        config = read_cpc_redir_config();
        mips_cm_unlock_other();
    }
    config
}

#[inline]
pub unsafe fn mips_cps_numcores(cluster: u32) -> u32 {
    if !mips_cm_present() { return 0; }
    field_get(CM_GCR_CONFIG_PCORES, mips_cps_cluster_config(cluster) + 1)
}

#[inline]
pub unsafe fn mips_cps_numiocu(cluster: u32) -> u32 {
    if !mips_cm_present() { return 0; }
    field_get(CM_GCR_CONFIG_NUMIOCU, mips_cps_cluster_config(cluster))
}

#[inline]
pub unsafe fn mips_cps_numvps(cluster: u32, core: u32) -> u32 {
    if !mips_cm_present() { return 1; }
    if ((!is_enabled_config_mips_mt_smp() || !cpu_has_mipsmt)
        && (!is_enabled_config_cpu_mipsr6() || !cpu_has_vp)) { return 1; }
    mips_cm_lock_other(cluster, core, 0, CM_GCR_Cx_OTHER_BLOCK_LOCAL);
    let cfg = if mips_cm_revision() < CM_REV_CM3_5 { read_gcr_co_config() }
              else { read_cpc_co_config() };
    mips_cm_unlock_other();
    field_get(CM_GCR_Cx_CONFIG_PVPE, cfg + 1)
}

#[inline]
pub unsafe fn mips_cps_multicluster_cpus() -> bool {
    let first_cl = cpu_cluster(&boot_cpu_data);
    let last_cl = cpu_cluster(&cpu_data[nr_cpu_ids as usize - 1]);
    first_cl != last_cl
}

extern "C" {
    pub fn mips_cps_first_online_in_cluster(first_cpu: *mut i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
