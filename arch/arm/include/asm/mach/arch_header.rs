/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mach/arch.h
 *
 *  Copyright (C) 2000 Russell King
 */

// linux/types.h and linux/reboot.h are supplied by the surrounding tree.

pub struct tag;
pub struct pt_regs;
pub struct smp_operations;

#[cfg(feature = "CONFIG_SMP")]
macro_rules! smp_ops {
    ($ops:expr) => { &($ops) };
}
#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! smp_ops {
    ($ops:expr) => { core::ptr::null::<smp_operations>() };
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! smp_init_ops {
    ($ops:expr) => { &($ops) };
}
#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! smp_init_ops {
    ($ops:expr) => { None::<unsafe extern "C" fn() -> bool> };
}

#[repr(C)]
pub struct machine_desc {
    pub nr: core::ffi::c_uint,
    pub name: *const core::ffi::c_char,
    pub atag_offset: core::ffi::c_ulong,
    pub dt_compat: *const *const core::ffi::c_char,
    pub nr_irqs: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    pub dma_zone_size: phys_addr_t,
    pub video_start: core::ffi::c_uint,
    pub video_end: core::ffi::c_uint,
    // C unsigned-char bit-fields, each one bit wide.
    pub reserve_lp0: u8,
    pub reserve_lp1: u8,
    pub reserve_lp2: u8,
    pub reboot_mode: reboot_mode,
    pub l2c_aux_val: core::ffi::c_uint,
    pub l2c_aux_mask: core::ffi::c_uint,
    pub l2c_write_sec: Option<unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_uint)>,
    pub smp: *const smp_operations,
    pub smp_init: Option<unsafe extern "C" fn() -> bool>,
    pub fixup: Option<unsafe extern "C" fn(*mut tag, *mut *mut core::ffi::c_char)>,
    pub dt_fixup: Option<unsafe extern "C" fn()>,
    pub pv_fixup: Option<unsafe extern "C" fn() -> i64>,
    pub reserve: Option<unsafe extern "C" fn()>,
    pub map_io: Option<unsafe extern "C" fn()>,
    pub init_early: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub init_time: Option<unsafe extern "C" fn()>,
    pub init_machine: Option<unsafe extern "C" fn()>,
    pub init_late: Option<unsafe extern "C" fn()>,
    pub restart: Option<unsafe extern "C" fn(reboot_mode, *const core::ffi::c_char)>,
}

/* Current machine - only accessible during boot. */
extern "C" {
    pub static machine_desc: *const machine_desc;
}

/* Machine type table - also only accessible during boot. */
extern "C" {
    pub static __arch_info_begin: machine_desc;
    pub static __arch_info_end: machine_desc;
}

macro_rules! for_each_machine_desc {
    ($p:ident) => {
        for $p in unsafe {
            core::slice::from_raw_parts(
                &__arch_info_begin as *const machine_desc,
                (&__arch_info_end as *const machine_desc).offset_from(
                    &__arch_info_begin as *const machine_desc,
                ) as usize,
            )
        }
    };
}

/*
 * Set of macros to define architecture features. This is built into
 * a table by the linker.
 */
macro_rules! MACHINE_START {
    ($type:ident, $name:expr) => {
        static __mach_desc_$type: machine_desc = machine_desc {
            nr: MACH_TYPE_$type,
            name: $name,
    };
    };
}

macro_rules! MACHINE_END {
    () => {};
}

macro_rules! DT_MACHINE_START {
    ($name:ident, $namestr:expr) => {
        static __mach_desc_$name: machine_desc = machine_desc {
            nr: !0,
            name: $namestr,
    };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
