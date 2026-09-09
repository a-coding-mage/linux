// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM Specific GTDT table Support
 *
 * Copyright (C) 2016, Linaro Ltd.
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org>
 *         Fu Wei <fu.wei@linaro.org>
 *         Hanjun Guo <hanjun.guo@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct acpi_gtdt_descriptor {
    pub gtdt: *mut acpi_table_gtdt,
    pub gtdt_end: *mut core::ffi::c_void,
    pub platform_timer: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct gtdt_v3 {
    pub gtdt_v2: acpi_table_gtdt,
    pub el2_vtimer: acpi_gtdt_el2,
}

static mut acpi_gtdt_desc: acpi_gtdt_descriptor = acpi_gtdt_descriptor {
    gtdt: core::ptr::null_mut(),
    gtdt_end: core::ptr::null_mut(),
    platform_timer: core::ptr::null_mut(),
};

unsafe fn gtdt_to_el2_vtimer(gtdt: *mut acpi_table_gtdt) -> *mut acpi_gtdt_el2 {
    if (*gtdt).header.revision < 3 {
        return core::ptr::null_mut();
    }
    (&mut (*(gtdt as *mut gtdt_v3)).el2_vtimer) as *mut acpi_gtdt_el2
}

unsafe fn platform_timer_valid(platform_timer: *mut core::ffi::c_void) -> bool {
    let gh = platform_timer as *mut acpi_gtdt_header;
    let platform_timer_begin: *mut u8;
    if (*acpi_gtdt_desc.gtdt).header.revision >= 3 {
        platform_timer_begin = (acpi_gtdt_desc.gtdt as *mut gtdt_v3 as *mut u8)
            .add(core::mem::size_of::<gtdt_v3>());
    } else {
        platform_timer_begin = (acpi_gtdt_desc.gtdt as *mut u8)
            .add(core::mem::size_of::<acpi_table_gtdt>());
    }
    let p = platform_timer as *mut u8;
    p >= platform_timer_begin
        && p.add(core::mem::size_of::<acpi_gtdt_header>()) <= acpi_gtdt_desc.gtdt_end as *mut u8
        && (*gh).length != 0
        && p.add((*gh).length as usize) <= acpi_gtdt_desc.gtdt_end as *mut u8
}

unsafe fn next_platform_timer(platform_timer: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let gh = platform_timer as *mut acpi_gtdt_header;
    (platform_timer as *mut u8).add((*gh).length as usize) as *mut core::ffi::c_void
}

unsafe fn is_timer_block(platform_timer: *mut core::ffi::c_void) -> bool {
    (*(platform_timer as *mut acpi_gtdt_header)).type_ == ACPI_GTDT_TYPE_TIMER_BLOCK
}

unsafe fn is_non_secure_watchdog(platform_timer: *mut core::ffi::c_void) -> bool {
    let gh = platform_timer as *mut acpi_gtdt_header;
    let wd = platform_timer as *mut acpi_gtdt_watchdog;
    if (*gh).type_ != ACPI_GTDT_TYPE_WATCHDOG { return false; }
    ((*wd).timer_flags & ACPI_GTDT_WATCHDOG_SECURE) == 0
}

unsafe fn map_gt_gsi(interrupt: u32, flags: u32) -> i32 {
    let trigger = if flags & ACPI_GTDT_INTERRUPT_MODE != 0 { ACPI_EDGE_SENSITIVE } else { ACPI_LEVEL_SENSITIVE };
    let polarity = if flags & ACPI_GTDT_INTERRUPT_POLARITY != 0 { ACPI_ACTIVE_LOW } else { ACPI_ACTIVE_HIGH };
    acpi_register_gsi(core::ptr::null_mut(), interrupt, trigger, polarity)
}

pub unsafe fn acpi_gtdt_map_ppi(type_: i32) -> i32 {
    let gtdt = acpi_gtdt_desc.gtdt;
    let el2_vtimer = gtdt_to_el2_vtimer(gtdt);
    match type_ {
        ARCH_TIMER_PHYS_NONSECURE_PPI => map_gt_gsi((*gtdt).non_secure_el1_interrupt, (*gtdt).non_secure_el1_flags),
        ARCH_TIMER_VIRT_PPI => map_gt_gsi((*gtdt).virtual_timer_interrupt, (*gtdt).virtual_timer_flags),
        ARCH_TIMER_HYP_PPI => map_gt_gsi((*gtdt).non_secure_el2_interrupt, (*gtdt).non_secure_el2_flags),
        ARCH_TIMER_HYP_VIRT_PPI => if !el2_vtimer.is_null() && (*el2_vtimer).virtual_el2_timer_gsiv != 0 {
            map_gt_gsi((*el2_vtimer).virtual_el2_timer_gsiv, (*el2_vtimer).virtual_el2_timer_flags)
        } else { 0 },
        _ => { pr_err!("Failed to map timer interrupt: invalid type.\n"); 0 }
    }
}

pub unsafe fn acpi_gtdt_c3stop(type_: i32) -> bool {
    let gtdt = acpi_gtdt_desc.gtdt;
    let el2_vtimer = gtdt_to_el2_vtimer(gtdt);
    match type_ {
        ARCH_TIMER_PHYS_NONSECURE_PPI => (*gtdt).non_secure_el1_flags & ACPI_GTDT_ALWAYS_ON == 0,
        ARCH_TIMER_VIRT_PPI => (*gtdt).virtual_timer_flags & ACPI_GTDT_ALWAYS_ON == 0,
        ARCH_TIMER_HYP_PPI => (*gtdt).non_secure_el2_flags & ACPI_GTDT_ALWAYS_ON == 0,
        ARCH_TIMER_HYP_VIRT_PPI => !el2_vtimer.is_null() && (*el2_vtimer).virtual_el2_timer_gsiv != 0 && (*el2_vtimer).virtual_el2_timer_flags & ACPI_GTDT_ALWAYS_ON == 0,
        _ => { pr_err!("Failed to get c3stop info: invalid type.\n"); false }
    }
}

pub unsafe fn acpi_gtdt_init(table: *mut acpi_table_header, platform_timer_count: *mut i32) -> i32 {
    let gtdt = table as *mut acpi_table_gtdt;
    let mut cnt: u32 = 0;
    if ((*gtdt).header.revision >= 3 && (*gtdt).header.length < core::mem::size_of::<gtdt_v3>() as u32)
        || ((*gtdt).header.revision == 2 && (*gtdt).header.length < core::mem::size_of::<acpi_table_gtdt>() as u32) {
        pr_err!("GTDT with invalid size {}\n", (*gtdt).header.length);
        return -EINVAL;
    }
    acpi_gtdt_desc.gtdt = gtdt;
    acpi_gtdt_desc.gtdt_end = (table as *mut u8).add((*table).length as usize) as *mut core::ffi::c_void;
    acpi_gtdt_desc.platform_timer = core::ptr::null_mut();
    if !platform_timer_count.is_null() { *platform_timer_count = 0; }
    if (*table).revision < 2 { pr_warn!("Revision:{} doesn't support Platform Timers.\n", (*table).revision); return 0; }
    if (*gtdt).platform_timer_count == 0 { pr_debug!("No Platform Timer.\n"); return 0; }
    acpi_gtdt_desc.platform_timer = (gtdt as *mut u8).add((*gtdt).platform_timer_offset as usize) as *mut core::ffi::c_void;
    let mut platform_timer = acpi_gtdt_desc.platform_timer;
    while platform_timer_valid(platform_timer) { cnt += 1; platform_timer = next_platform_timer(platform_timer); }
    if cnt != (*gtdt).platform_timer_count { cnt = core::cmp::min(cnt, (*gtdt).platform_timer_count); pr_err!("limiting Platform Timer count to {}\n", cnt); }
    if cnt == 0 { acpi_gtdt_desc.platform_timer = core::ptr::null_mut(); return 0; }
    if !platform_timer_count.is_null() { *platform_timer_count = cnt as i32; }
    0
}

unsafe fn gtdt_parse_timer_block(block: *mut acpi_gtdt_timer_block, timer_mem: *mut arch_timer_mem) -> i32 {
    if (*block).timer_count == 0 { pr_err!("GT block present, but frame count is zero.\n"); return -ENODEV; }
    if (*block).timer_count > ARCH_TIMER_MEM_MAX_FRAMES { pr_err!("GT block lists {} frames, ACPI spec only allows 8\n", (*block).timer_count); return -EINVAL; }
    (*timer_mem).cntctlbase = (*block).block_address as phys_addr_t;
    (*timer_mem).size = SZ_4K;
    let mut gtdt_frame = (block as *mut u8).add((*block).timer_offset as usize) as *mut acpi_gtdt_timer_entry;
    if gtdt_frame.add((*block).timer_count as usize) as *mut u8 != (block as *mut u8).add((*block).header.length as usize) { return -EINVAL; }
    let mut i = 0;
    while i < (*block).timer_count {
        if (*gtdt_frame).common_flags & ACPI_GTDT_GT_IS_SECURE_TIMER != 0 { i += 1; gtdt_frame = gtdt_frame.add(1); continue; }
        if (*gtdt_frame).frame_number >= ARCH_TIMER_MEM_MAX_FRAMES || (*gtdt_frame).base_address == 0 || (*gtdt_frame).timer_interrupt == 0 { break; }
        let frame = &mut (*timer_mem).frame[(*gtdt_frame).frame_number as usize];
        if frame.valid { break; }
        frame.phys_irq = map_gt_gsi((*gtdt_frame).timer_interrupt, (*gtdt_frame).timer_flags);
        if frame.phys_irq <= 0 { pr_warn!("failed to map physical timer irq in frame {}.\n", (*gtdt_frame).frame_number); break; }
        if (*gtdt_frame).virtual_timer_interrupt != 0 { frame.virt_irq = map_gt_gsi((*gtdt_frame).virtual_timer_interrupt, (*gtdt_frame).virtual_timer_flags); if frame.virt_irq <= 0 { pr_warn!("failed to map virtual timer irq in frame {}.\n", (*gtdt_frame).frame_number); break; } } else { pr_debug!("virtual timer in frame {} not implemented.\n", (*gtdt_frame).frame_number); }
        frame.cntbase = (*gtdt_frame).base_address; frame.size = SZ_4K; frame.valid = true;
        i += 1; gtdt_frame = gtdt_frame.add(1);
    }
    if i == (*block).timer_count { return 0; }
    while i > 0 { i -= 1; gtdt_frame = gtdt_frame.sub(1); if (*gtdt_frame).common_flags & ACPI_GTDT_GT_IS_SECURE_TIMER != 0 || (*gtdt_frame).frame_number >= ARCH_TIMER_MEM_MAX_FRAMES { continue; } let frame = &mut (*timer_mem).frame[(*gtdt_frame).frame_number as usize]; if frame.phys_irq > 0 { acpi_unregister_gsi((*gtdt_frame).timer_interrupt); } frame.phys_irq = 0; if frame.virt_irq > 0 { acpi_unregister_gsi((*gtdt_frame).virtual_timer_interrupt); } frame.virt_irq = 0; }
    -EINVAL
}

unsafe fn gtdt_import_sbsa_gwdt(wd: *mut acpi_gtdt_watchdog, index: i32) -> i32 {
    if (*wd).refresh_frame_address == 0 || (*wd).control_frame_address == 0 { pr_err!("failed to get the Watchdog base address.\n"); return -EINVAL; }
    let irq = map_gt_gsi((*wd).timer_interrupt, (*wd).timer_flags);
    let mut res = [DEFINE_RES_MEM!((*wd).control_frame_address, SZ_4K), DEFINE_RES_MEM!((*wd).refresh_frame_address, SZ_4K), DEFINE_RES_IRQ!(irq)];
    let nr_res = if irq <= 0 { 2 } else { 3 };
    let pdev = platform_device_register_simple("sbsa-gwdt", index, res.as_mut_ptr(), nr_res);
    if IS_ERR!(pdev) { if irq > 0 { acpi_unregister_gsi((*wd).timer_interrupt); } return PTR_ERR!(pdev); }
    0
}

unsafe fn gtdt_platform_timer_init() -> i32 {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut timer_count = 0; let mut gwdt_count = 0; let mut mmio_timer_count = 0;
    if acpi_disabled { return 0; }
    if ACPI_FAILURE!(acpi_get_table!(ACPI_SIG_GTDT, 0, &mut table)) { return -EINVAL; }
    let mut ret = acpi_gtdt_init(table, &mut timer_count);
    if ret != 0 || timer_count == 0 { acpi_put_table(table); return ret; }
    let mut platform_timer = acpi_gtdt_desc.platform_timer;
    while platform_timer_valid(platform_timer) {
        ret = 0;
        if is_non_secure_watchdog(platform_timer) { ret = gtdt_import_sbsa_gwdt(platform_timer as *mut acpi_gtdt_watchdog, gwdt_count); if ret == 0 { gwdt_count += 1; } }
        else if is_timer_block(platform_timer) { let mut atm: arch_timer_mem = core::mem::zeroed(); ret = gtdt_parse_timer_block(platform_timer as *mut acpi_gtdt_timer_block, &mut atm); if ret == 0 { let pdev = platform_device_register_data(core::ptr::null_mut(), "gtdt-arm-mmio-timer", mmio_timer_count, &atm, core::mem::size_of::<arch_timer_mem>()); if IS_ERR!(pdev) { pr_err!("Can't register timer {}\n", mmio_timer_count); } else { mmio_timer_count += 1; } } }
        platform_timer = next_platform_timer(platform_timer);
    }
    if gwdt_count != 0 { pr_info!("found {} SBSA generic Watchdog(s).\n", gwdt_count); }
    if mmio_timer_count != 0 { pr_info!("found {} Generic MMIO timer(s).\n", mmio_timer_count); }
    acpi_put_table(table); ret
}

// device_initcall(gtdt_platform_timer_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
