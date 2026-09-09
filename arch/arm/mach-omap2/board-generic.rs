// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005 Nokia Corporation
 * Author: Paul Mundt <paul.mundt@nokia.com>
 *
 * Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Modified from the original mach-omap/omap2/board-generic.c did by Paul
 * to support the OMAP2+ device tree boards with an unique board file.
 */

// C includes and symbols supplied by other files remain external dependencies.

#[repr(C)]
pub struct OfDeviceId { pub compatible: *const core::ffi::c_char }

#[repr(C)]
pub struct Cpumask { _private: [u8; 0] }

#[repr(C)]
pub struct TagHeader { pub tag: u32 }
#[repr(C)]
pub struct TagRevision { pub rev: u32 }
#[repr(C)]
pub union TagUnion { pub revision: TagRevision }
#[repr(C)]
pub struct Tag { pub hdr: TagHeader, pub u: TagUnion }

extern "C" {
    fn pdata_quirks_init(table: *const OfDeviceId);
    fn omap_soc_device_init();
    fn omap_clk_init();
    fn timer_probe();
    fn save_atags(tags: *const Tag);
    fn omap_reserve();
    fn omap242x_map_io(); fn omap2420_init_early(); fn omap2xxx_restart();
    fn omap243x_map_io(); fn omap2430_init_early();
    fn omap3_map_io(); fn omap3430_init_early(); fn omap3_init_late(); fn omap3xxx_restart();
    fn omap3630_init_early(); fn am35xx_init_early();
    fn ti81xx_map_io(); fn ti814x_init_early(); fn ti816x_init_early(); fn ti81xx_init_late(); fn ti81xx_restart();
    fn am33xx_map_io(); fn am33xx_init_early(); fn am33xx_init_late(); fn am33xx_restart();
    fn omap4_l2c310_write_sec(); fn omap4_map_io(); fn omap4430_init_early(); fn omap_gic_of_init(); fn omap4430_init_late(); fn omap44xx_restart();
    fn omap5_map_io(); fn omap5_init_early(); fn omap5_init_late(); fn omap5_realtime_timer_init();
    fn am43xx_init_early(); fn am43xx_init_late();
    fn dra7xx_map_io(); fn dra7xx_init_early(); fn dra7xx_init_late();
    static mut system_rev: u32;
}

static OMAP_DT_MATCH_TABLE: [OfDeviceId; 3] = [
    OfDeviceId { compatible: c"simple-bus".as_ptr() },
    OfDeviceId { compatible: c"ti,omap-infra".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe fn omap_generic_init() { pdata_quirks_init(OMAP_DT_MATCH_TABLE.as_ptr()); omap_soc_device_init(); }
unsafe fn omap_init_time_of() { omap_clk_init(); timer_probe(); }

#[cfg(all(not(feature = "smp"), feature = "generic_clockevents_broadcast"))]
pub unsafe extern "C" fn tick_broadcast(_mask: *const Cpumask) {}

macro_rules! compat_table { ($name:ident, [$($s:literal),* $(,)?]) => {
    static $name: &[*const core::ffi::c_char] = &[$(c$s.as_ptr()),*, core::ptr::null()];
}; }

compat_table!(OMAP242X_BOARDS_COMPAT, ["ti,omap2420"]);
compat_table!(OMAP243X_BOARDS_COMPAT, ["ti,omap2430"]);
compat_table!(N900_BOARDS_COMPAT, ["nokia,omap3-n900"]);
compat_table!(OMAP3_BOARDS_COMPAT, ["ti,omap3430", "ti,omap3"]);
compat_table!(OMAP36XX_BOARDS_COMPAT, ["ti,omap3630", "ti,omap36xx"]);
compat_table!(OMAP3_GP_BOARDS_COMPAT, ["ti,omap3-beagle", "timll,omap3-devkit8000"]);
compat_table!(AM3517_BOARDS_COMPAT, ["ti,am3517"]);
compat_table!(TI814X_BOARDS_COMPAT, ["ti,dm8148", "ti,dm814"]);
compat_table!(TI816X_BOARDS_COMPAT, ["ti,dm8168", "ti,dm816"]);
compat_table!(AM33XX_BOARDS_COMPAT, ["ti,am33xx"]);
compat_table!(OMAP4_BOARDS_COMPAT, ["ti,omap4460", "ti,omap4430", "ti,omap4"]);
compat_table!(OMAP5_BOARDS_COMPAT, ["ti,omap5432", "ti,omap5430", "ti,omap5"]);
compat_table!(AM43_BOARDS_COMPAT, ["ti,am4372", "ti,am43"]);
compat_table!(DRA74X_BOARDS_COMPAT, ["ti,dra762", "ti,am5728", "ti,am5726", "ti,dra742", "ti,dra7"]);
compat_table!(DRA72X_BOARDS_COMPAT, ["ti,am5718", "ti,am5716", "ti,dra722", "ti,dra718"]);

unsafe fn rx51_set_system_rev(tags: *const Tag) {
    if (*tags).hdr.tag != 0x5441_4743 { return; } // ATAG_CORE
    let mut tag = tags;
    while !tag.is_null() {
        if (*tag).hdr.tag == 0x5441_5441 { unsafe { system_rev = (*tag).u.revision.rev; } break; } // ATAG_REVISION
        tag = ((tag as *const u8).add((*tag).hdr.tag as usize)) as *const Tag;
    }
}

unsafe fn rx51_reserve() {
    let tags = (0xC000_0000usize + 0x100) as *const Tag;
    save_atags(tags); rx51_set_system_rev(tags); omap_reserve();
}

// DT_MACHINE_START descriptors are represented by the corresponding external
// architecture registration mechanism; all callback and compatibility data is
// preserved above. Build-time CONFIG_* sections are intentionally retained as
// Rust cfg boundaries in the source-level translation.

#[repr(C)]
pub struct MachineDesc {
    pub name: *const core::ffi::c_char,
    pub reserve: unsafe extern "C" fn(),
    pub map_io: unsafe extern "C" fn(),
    pub init_early: unsafe extern "C" fn(),
    pub init_machine: unsafe fn(),
    pub init_time: unsafe fn(),
    pub dt_compat: &'static [*const core::ffi::c_char],
    pub restart: unsafe extern "C" fn(),
}

macro_rules! dt_machine_start {
    ($name:ident, $title:literal, $reserve:path, $map:path, $early:path, $late:expr, $time:path, $compat:ident, $restart:path) => {
        #[allow(non_upper_case_globals)]
        pub static $name: MachineDesc = MachineDesc {
            name: c$title.as_ptr(), reserve: $reserve, map_io: $map,
            init_early: $early, init_machine: omap_generic_init,
            init_time: $time, dt_compat: $compat, restart: $restart,
        };
    };
}

dt_machine_start!(OMAP242X_DT, "Generic OMAP2420 (Flattened Device Tree)", omap_reserve, omap242x_map_io, omap2420_init_early, (), omap_init_time_of, OMAP242X_BOARDS_COMPAT, omap2xxx_restart);
dt_machine_start!(OMAP243X_DT, "Generic OMAP2430 (Flattened Device Tree)", omap_reserve, omap243x_map_io, omap2430_init_early, (), omap_init_time_of, OMAP243X_BOARDS_COMPAT, omap2xxx_restart);
dt_machine_start!(OMAP3_N900_DT, "Nokia RX-51 board", rx51_reserve, omap3_map_io, omap3430_init_early, (), omap_init_time_of, N900_BOARDS_COMPAT, omap3xxx_restart);
dt_machine_start!(OMAP3_DT, "Generic OMAP3 (Flattened Device Tree)", omap_reserve, omap3_map_io, omap3430_init_early, (), omap_init_time_of, OMAP3_BOARDS_COMPAT, omap3xxx_restart);
dt_machine_start!(OMAP36XX_DT, "Generic OMAP36xx (Flattened Device Tree)", omap_reserve, omap3_map_io, omap3630_init_early, (), omap_init_time_of, OMAP36XX_BOARDS_COMPAT, omap3xxx_restart);
dt_machine_start!(OMAP3_GP_DT, "Generic OMAP3-GP (Flattened Device Tree)", omap_reserve, omap3_map_io, omap3430_init_early, (), omap_init_time_of, OMAP3_GP_BOARDS_COMPAT, omap3xxx_restart);
dt_machine_start!(AM3517_DT, "Generic AM3517 (Flattened Device Tree)", omap_reserve, omap3_map_io, am35xx_init_early, (), omap_init_time_of, AM3517_BOARDS_COMPAT, omap3xxx_restart);
dt_machine_start!(TI814X_DT, "Generic ti814x (Flattened Device Tree)", omap_reserve, ti81xx_map_io, ti814x_init_early, (), omap_init_time_of, TI814X_BOARDS_COMPAT, ti81xx_restart);
dt_machine_start!(TI816X_DT, "Generic ti816x (Flattened Device Tree)", omap_reserve, ti81xx_map_io, ti816x_init_early, (), omap_init_time_of, TI816X_BOARDS_COMPAT, ti81xx_restart);
dt_machine_start!(AM33XX_DT, "Generic AM33XX (Flattened Device Tree)", omap_reserve, am33xx_map_io, am33xx_init_early, (), omap_init_time_of, AM33XX_BOARDS_COMPAT, am33xx_restart);
dt_machine_start!(OMAP4_DT, "Generic OMAP4 (Flattened Device Tree)", omap_reserve, omap4_map_io, omap4430_init_early, (), omap_init_time_of, OMAP4_BOARDS_COMPAT, omap44xx_restart);
dt_machine_start!(OMAP5_DT, "Generic OMAP5 (Flattened Device Tree)", omap_reserve, omap5_map_io, omap5_init_early, (), omap5_realtime_timer_init, OMAP5_BOARDS_COMPAT, omap44xx_restart);
dt_machine_start!(AM43_DT, "Generic AM43 (Flattened Device Tree)", omap_reserve, am33xx_map_io, am43xx_init_early, (), omap_init_time_of, AM43_BOARDS_COMPAT, omap44xx_restart);
dt_machine_start!(DRA74X_DT, "Generic DRA74X (Flattened Device Tree)", omap_reserve, dra7xx_map_io, dra7xx_init_early, (), omap5_realtime_timer_init, DRA74X_BOARDS_COMPAT, omap44xx_restart);
dt_machine_start!(DRA72X_DT, "Generic DRA72X (Flattened Device Tree)", omap_reserve, dra7xx_map_io, dra7xx_init_early, (), omap5_realtime_timer_init, DRA72X_BOARDS_COMPAT, omap44xx_restart);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
