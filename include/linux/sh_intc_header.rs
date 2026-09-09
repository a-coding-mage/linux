/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong};

// #include <linux/ioport.h>

#[cfg(feature = "CONFIG_SUPERH")]
pub const INTC_NR_IRQS: usize = 512;
#[cfg(not(feature = "CONFIG_SUPERH"))]
pub const INTC_NR_IRQS: usize = 1024;

/*
 * Convert back and forth between INTEVT and IRQ values.
 */
#[cfg(feature = "CONFIG_CPU_HAS_INTEVT")]
#[inline]
pub const fn evt2irq(evt: u32) -> u32 { evt >> 5 }
#[cfg(feature = "CONFIG_CPU_HAS_INTEVT")]
#[inline]
pub const fn irq2evt(irq: u32) -> u32 { irq << 5 }
#[cfg(not(feature = "CONFIG_CPU_HAS_INTEVT"))]
#[inline]
pub const fn evt2irq(evt: u32) -> u32 { evt }
#[cfg(not(feature = "CONFIG_CPU_HAS_INTEVT"))]
#[inline]
pub const fn irq2evt(irq: u32) -> u32 { irq }

pub type intc_enum = u8;

#[repr(C)]
pub struct intc_vect {
    pub enum_id: intc_enum,
    pub vect: u16,
}

#[macro_export]
macro_rules! INTC_VECT {
    ($enum_id:expr, $vect:expr) => { $crate::intc_vect { enum_id: $enum_id, vect: $vect } };
}

#[macro_export]
macro_rules! INTC_IRQ {
    ($enum_id:expr, $irq:expr) => { $crate::INTC_VECT!($enum_id, $crate::irq2evt($irq)) };
}

#[repr(C)]
pub struct intc_group {
    pub enum_id: intc_enum,
    pub enum_ids: [intc_enum; 32],
}

#[macro_export]
macro_rules! INTC_GROUP {
    ($enum_id:expr $(, $id:expr)* $(,)?) => {
        $crate::intc_group { enum_id: $enum_id, enum_ids: [$($id,)* 0; 32] }
    };
}

#[repr(C)]
pub struct intc_subgroup {
    pub reg: c_ulong,
    pub reg_width: c_ulong,
    pub parent_id: intc_enum,
    pub enum_ids: [intc_enum; 32],
}

#[repr(C)]
pub struct intc_mask_reg {
    pub set_reg: c_ulong,
    pub clr_reg: c_ulong,
    pub reg_width: c_ulong,
    pub enum_ids: [intc_enum; 32],
    #[cfg(feature = "CONFIG_INTC_BALANCING")]
    pub dist_reg: c_ulong,
    #[cfg(feature = "CONFIG_SMP")]
    pub smp: c_ulong,
}

#[repr(C)]
pub struct intc_prio_reg {
    pub set_reg: c_ulong,
    pub clr_reg: c_ulong,
    pub reg_width: c_ulong,
    pub field_width: c_ulong,
    pub enum_ids: [intc_enum; 16],
    #[cfg(feature = "CONFIG_SMP")]
    pub smp: c_ulong,
}

#[repr(C)]
pub struct intc_sense_reg {
    pub reg: c_ulong,
    pub reg_width: c_ulong,
    pub field_width: c_ulong,
    pub enum_ids: [intc_enum; 16],
}

#[cfg(feature = "CONFIG_INTC_BALANCING")]
#[macro_export]
macro_rules! INTC_SMP_BALANCING { ($reg:expr) => { dist_reg: $reg }; }
#[cfg(not(feature = "CONFIG_INTC_BALANCING"))]
#[macro_export]
macro_rules! INTC_SMP_BALANCING { ($reg:expr) => {}; }

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! INTC_SMP { ($stride:expr, $nr:expr) => { smp: ($stride) | (($nr) << 8) }; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! INTC_SMP { ($stride:expr, $nr:expr) => {}; }

#[repr(C)]
pub struct intc_hw_desc {
    pub vectors: *mut intc_vect,
    pub nr_vectors: u32,
    pub groups: *mut intc_group,
    pub nr_groups: u32,
    pub mask_regs: *mut intc_mask_reg,
    pub nr_mask_regs: u32,
    pub prio_regs: *mut intc_prio_reg,
    pub nr_prio_regs: u32,
    pub sense_regs: *mut intc_sense_reg,
    pub nr_sense_regs: u32,
    pub ack_regs: *mut intc_mask_reg,
    pub nr_ack_regs: u32,
    pub subgroups: *mut intc_subgroup,
    pub nr_subgroups: u32,
}

// C _Generic/sizeof helpers are represented by the Rust macro below.
#[macro_export]
macro_rules! _INTC_ARRAY { ($a:expr) => { ($a, core::mem::size_of_val(&$a) / core::mem::size_of_val(&$a[0])) }; }

#[repr(C)]
pub struct resource;

#[repr(C)]
pub struct intc_desc {
    pub name: *mut c_char,
    pub resource: *mut resource,
    pub num_resources: u32,
    pub force_enable: intc_enum,
    pub force_disable: intc_enum,
    pub skip_syscore_suspend: bool,
    pub hw: intc_hw_desc,
}

extern "C" {
    pub fn register_intc_controller(desc: *mut intc_desc) -> c_int;
    pub fn intc_set_priority(irq: u32, prio: u32) -> c_int;
    pub fn intc_irq_lookup(chipname: *const c_char, enum_id: intc_enum) -> c_int;
    pub fn intc_finalize();
}

#[cfg(feature = "CONFIG_INTC_USERIMASK")]
extern "C" {
    pub fn register_intc_userimask(addr: c_ulong) -> c_int;
}

#[cfg(not(feature = "CONFIG_INTC_USERIMASK"))]
#[inline]
pub unsafe fn register_intc_userimask(_addr: c_ulong) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
