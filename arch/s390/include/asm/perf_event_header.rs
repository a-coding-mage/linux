/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Performance event support - s390 specific definitions.
 *
 * Copyright IBM Corp. 2009, 2017
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 *           Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// C dependencies: linux/perf_event.h, linux/device.h, asm/stacktrace.h.

/* Per-CPU flags for PMU states */
pub const PMU_F_RESERVED: u32 = 0x1000;
pub const PMU_F_ENABLED: u32 = 0x2000;
pub const PMU_F_IN_USE: u32 = 0x4000;
pub const PMU_F_ERR_IBE: u32 = 0x0100;
pub const PMU_F_ERR_LSDA: u32 = 0x0200;
pub const PMU_F_ERR_MASK: u32 = PMU_F_ERR_IBE | PMU_F_ERR_LSDA;

/* Perf definitions for PMU event attributes in sysfs */
extern "C" {
    pub fn cpumf_cf_event_group() -> *const *const attribute_group;
    pub fn cpumf_events_sysfs_show(
        dev: *mut device,
        attr: *mut device_attribute,
        page: *mut core::ffi::c_char,
    ) -> isize;
}

/*
 * C token-pasting and PMU_EVENT_ATTR expansion are retained as source-level
 * macro intent; the referenced event attributes are supplied by dependencies.
 * EVENT_VAR(cat, name) => event_attr_cat_name
 * EVENT_PTR(cat, name) => &EVENT_VAR(cat, name).attr.attr
 * CPUMF_EVENT_ATTR(cat, name, id) => PMU_EVENT_ATTR(name, EVENT_VAR(cat, name), id, cpumf_events_sysfs_show)
 * CPUMF_EVENT_PTR(cat, name) => EVENT_PTR(cat, name)
 */

/* Perf callbacks */
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong;
    pub fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong;
}

/* perf_arch_misc_flags(regs) is the C self-referential macro wrapper. */
/* perf_arch_bpf_user_pt_regs(regs) => &regs->user_regs */

/* Perf pt_regs extension for sample-data-entry indicators */
#[repr(C)]
pub struct perf_sf_sde_regs {
    /* C bit-field: in_guest:1; */
    pub in_guest: u8,
    /* C bit-field: reserved:63; */
    pub reserved: c_ulong,
}

/* C dependencies: struct stack_frame and offsetof are supplied externally. */
/*
 * perf_arch_fetch_caller_regs(regs, __ip):
 *   (regs)->psw.mask = 0;
 *   (regs)->psw.addr = (__ip);
 *   (regs)->gprs[15] = (unsigned long)__builtin_frame_address(0) -
 *       offsetof(struct stack_frame, back_chain);
 */

pub type c_ulong = core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
