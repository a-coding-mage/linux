/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// C header dependency: "hda_controller.h"
use crate::{
    azx, azx_dev, completion, delayed_work, list_head, work_struct,
};

pub const HDA_INTEL_IRQ_PENDING_WARNED: u32 = 1 << 0;
pub const HDA_INTEL_PROBE_CONTINUED: u32 = 1 << 1;
pub const HDA_INTEL_RUNTIME_PM_DISABLED: u32 = 1 << 2;
pub const HDA_INTEL_USE_VGA_SWITCHEROO: u32 = 1 << 3;
pub const HDA_INTEL_VGA_SWITCHEROO_REGISTERED: u32 = 1 << 4;
pub const HDA_INTEL_INIT_FAILED: u32 = 1 << 5;
pub const HDA_INTEL_FREED: u32 = 1 << 6;
pub const HDA_INTEL_NEED_I915_POWER: u32 = 1 << 7;

#[repr(C)]
pub struct hda_intel {
    pub chip: azx,

    /* sync probing */
    pub probe_wait: completion,
    pub probe_work: delayed_work,

    /* card list (for power_save trigger) */
    pub list: list_head,

    /*
     * extra flags
     *
     * C bitfields translated to flag bits in this storage word:
     * irq_pending_warned:1, probe_continued:1, runtime_pm_disabled:1,
     * use_vga_switcheroo:1, vga_switcheroo_registered:1,
     * init_failed:1, freed:1, need_i915_power:1.
     */
    pub flags: u32,

    pub probe_retry: i32, /* being probe-retry */
}

#[repr(C)]
pub struct hda_intel_stream {
    pub azx_dev: azx_dev,

    /* for pending irqs */
    pub hda: *mut hda_intel,
    pub irq_pending_work: work_struct,
    pub irq_pending: bool,
}

#[inline]
pub unsafe fn azx_dev_to_istream(azx_dev: *mut azx_dev) -> *mut hda_intel_stream {
    (azx_dev as *mut u8).sub(core::mem::offset_of!(hda_intel_stream, azx_dev))
        as *mut hda_intel_stream
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
