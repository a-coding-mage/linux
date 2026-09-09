// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Generation 2 support
 *
 * Copyright (C) 2013  Renesas Solutions Corp.
 * Copyright (C) 2013  Magnus Damm
 * Copyright (C) 2014  Ulrich Hecht
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

extern "C" {
    fn of_find_matching_node_and_match(
        from: *mut DeviceNode,
        matches: *const OfDeviceId,
        match_out: *mut *const OfDeviceId,
    ) -> *mut DeviceNode;
    fn of_property_match_string(
        node: *mut DeviceNode,
        property: *const core::ffi::c_char,
        string: *const core::ffi::c_void,
    ) -> i32;
    fn of_parse_phandle(node: *mut DeviceNode, property: *const core::ffi::c_char, index: i32) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn of_property_read_u32(node: *mut DeviceNode, property: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn of_machine_compatible_match(matches: *const *const core::ffi::c_char) -> bool;
    fn secure_cntvoff_init();
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn ioread32(addr: *const u8) -> u32;
    fn iowrite32(value: u32, addr: *mut u8);
    fn iounmap(addr: *mut u8);
    fn of_clk_init(data: *const core::ffi::c_void);
    fn timer_probe();
    fn shmobile_init_late();
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

static CPG_MATCHES: &[OfDeviceId] = &[
    OfDeviceId { compatible: b"renesas,r8a7742-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"renesas,r8a7743-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"renesas,r8a7744-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"renesas,r8a7790-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"renesas,r8a7791-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"renesas,r8a7793-cpg-mssr\0".as_ptr() as *const _, data: b"extal\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn get_extal_freq() -> u32 {
    let mut matched: *const OfDeviceId = core::ptr::null();
    let mut freq: u32 = 20_000_000;
    let cpg = of_find_matching_node_and_match(core::ptr::null_mut(), CPG_MATCHES.as_ptr(), &mut matched);
    if cpg.is_null() { return freq; }
    let idx = if !(*matched).data.is_null() {
        of_property_match_string(cpg, b"clock-names\0".as_ptr() as *const _, (*matched).data)
    } else { 0 };
    let extal = of_parse_phandle(cpg, b"clocks\0".as_ptr() as *const _, idx);
    of_node_put(cpg);
    if extal.is_null() { return freq; }
    of_property_read_u32(extal, b"clock-frequency\0".as_ptr() as *const _, &mut freq);
    of_node_put(extal);
    freq
}

const CNTCR: usize = 0;
const CNTFID0: usize = 0x20;

pub unsafe fn rcar_gen2_timer_init() {
    let fixed_freq_socs: [*const core::ffi::c_char; 5] = [
        b"renesas,r8a7745\0".as_ptr() as *const _, b"renesas,r8a77470\0".as_ptr() as *const _,
        b"renesas,r8a7792\0".as_ptr() as *const _, b"renesas,r8a7794\0".as_ptr() as *const _, core::ptr::null(),
    ];
    // CONFIG_ARM_PSCI_FW: when PSCI is active, firmware has already updated the counter.
    let mut need_update = true;
    if !need_update { of_clk_init(core::ptr::null()); timer_probe(); return; }
    secure_cntvoff_init();
    let freq = if of_machine_compatible_match(fixed_freq_socs.as_ptr()) { 260_000_000 / 8 } else { get_extal_freq() / 2 };
    let base = ioremap(0xe6080000, 4096);
    if ioread32(base.add(CNTCR)) & 1 == 0 || ioread32(base.add(CNTFID0)) != freq {
        iowrite32(freq, base.add(CNTFID0));
        // Corresponds to: asm volatile("mcr p15, 0, %0, c14, c0, 0").
        core::arch::asm!("mcr p15, 0, {0}, c14, c0, 0", in(reg) freq);
        iowrite32(1, base.add(CNTCR));
    }
    iounmap(base);
    of_clk_init(core::ptr::null());
    timer_probe();
}

pub static RCAR_GEN2_BOARDS_COMPAT_DT: [*const core::ffi::c_char; 6] = [
    b"renesas,r8a7790\0".as_ptr() as *const _, b"renesas,r8a7791\0".as_ptr() as *const _,
    b"renesas,r8a7792\0".as_ptr() as *const _, b"renesas,r8a7793\0".as_ptr() as *const _,
    b"renesas,r8a7794\0".as_ptr() as *const _, core::ptr::null(),
];

pub static RZ_G1_BOARDS_COMPAT_DT: [*const core::ffi::c_char; 6] = [
    b"renesas,r8a7742\0".as_ptr() as *const _, b"renesas,r8a7743\0".as_ptr() as *const _,
    b"renesas,r8a7744\0".as_ptr() as *const _, b"renesas,r8a7745\0".as_ptr() as *const _,
    b"renesas,r8a77470\0".as_ptr() as *const _, core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
