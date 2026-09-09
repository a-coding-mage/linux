// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

/* The Serial To Parallel (STP) is found on MIPS based Lantiq socs. */

const XWAY_STP_CON0: u32 = 0x00;
const XWAY_STP_CON1: u32 = 0x04;
const XWAY_STP_CPU0: u32 = 0x08;
const XWAY_STP_CPU1: u32 = 0x0C;
const XWAY_STP_AR: u32 = 0x10;

const XWAY_STP_CON_SWU: u32 = 1u32 << 31;
const XWAY_STP_2HZ: u32 = 0;
const XWAY_STP_4HZ: u32 = 1u32 << 23;
const XWAY_STP_8HZ: u32 = 1u32 << 24;
const XWAY_STP_10HZ: u32 = (1u32 << 24) | (1u32 << 23);
const XWAY_STP_SPEED_MASK: u32 = (1u32 << 23) | (1u32 << 24) | (1u32 << 25) | (1u32 << 26) | (1u32 << 27);
const XWAY_STP_FPIS_VALUE: u32 = 1u32 << 21;
const XWAY_STP_FPIS_MASK: u32 = (1u32 << 20) | (1u32 << 21);
const XWAY_STP_UPD_FPI: u32 = 1u32 << 31;
const XWAY_STP_UPD_MASK: u32 = (1u32 << 31) | (1u32 << 30);
const XWAY_STP_ADSL_SHIFT: u32 = 24;
const XWAY_STP_ADSL_MASK: u32 = 0x3;
const XWAY_STP_PHY_MASK: u32 = 0x7;
const XWAY_STP_PHY1_SHIFT: u32 = 27;
const XWAY_STP_PHY2_SHIFT: u32 = 3;
const XWAY_STP_PHY3_SHIFT: u32 = 6;
const XWAY_STP_PHY4_SHIFT: u32 = 15;
const XWAY_STP_GROUP0: u32 = 1u32 << 0;
const XWAY_STP_GROUP1: u32 = 1u32 << 1;
const XWAY_STP_GROUP2: u32 = 1u32 << 2;
const XWAY_STP_GROUP_MASK: u32 = 0x7;
const XWAY_STP_FALLING: u32 = 1u32 << 26;
const XWAY_STP_EDGE_MASK: u32 = 1u32 << 26;

#[repr(C)]
struct xway_stp {
    gc: gpio_chip,
    virt: *mut core::ffi::c_void,
    edge: u32,
    shadow: u32,
    groups: u8,
    dsl: u8,
    phy1: u8,
    phy2: u8,
    phy3: u8,
    phy4: u8,
    reserved: u8,
}

unsafe fn xway_stp_r32(m: *mut core::ffi::c_void, reg: u32) -> u32 {
    core::ptr::read_volatile((m as *mut u8).add(reg as usize) as *const u32)
}

unsafe fn xway_stp_w32(m: *mut core::ffi::c_void, val: u32, reg: u32) {
    core::ptr::write_volatile((m as *mut u8).add(reg as usize) as *mut u32, val);
}

unsafe fn xway_stp_w32_mask(m: *mut core::ffi::c_void, clear: u32, set: u32, reg: u32) {
    xway_stp_w32(m, (xway_stp_r32(m, reg) & !clear) | set, reg);
}

unsafe fn xway_stp_get(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut xway_stp;
    ((xway_stp_r32((*chip).virt, XWAY_STP_CPU0) & (1u32 << gpio)) != 0) as i32
}

unsafe fn xway_stp_set(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut xway_stp;
    if val != 0 { (*chip).shadow |= 1u32 << gpio; } else { (*chip).shadow &= !(1u32 << gpio); }
    xway_stp_w32((*chip).virt, (*chip).shadow, XWAY_STP_CPU0);
    if (*chip).reserved == 0 { xway_stp_w32_mask((*chip).virt, 0, XWAY_STP_CON_SWU, XWAY_STP_CON0); }
    0
}

unsafe fn xway_stp_dir_out(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 { xway_stp_set(gc, gpio, val) }

unsafe fn xway_stp_request(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut xway_stp;
    if gpio < 8 && ((*chip).reserved & (1u8 << gpio)) != 0 { dev_err((*gc).parent, "GPIO %d is driven by hardware\n", gpio); return -ENODEV; }
    0
}

unsafe fn xway_stp_hw_init(chip: *mut xway_stp) {
    xway_stp_w32((*chip).virt, 0, XWAY_STP_AR); xway_stp_w32((*chip).virt, 0, XWAY_STP_CPU0);
    xway_stp_w32((*chip).virt, 0, XWAY_STP_CPU1); xway_stp_w32((*chip).virt, XWAY_STP_CON_SWU, XWAY_STP_CON0);
    xway_stp_w32((*chip).virt, 0, XWAY_STP_CON1);
    xway_stp_w32_mask((*chip).virt, XWAY_STP_EDGE_MASK, (*chip).edge, XWAY_STP_CON0);
    xway_stp_w32_mask((*chip).virt, XWAY_STP_GROUP_MASK, (*chip).groups as u32, XWAY_STP_CON1);
    xway_stp_w32_mask((*chip).virt, XWAY_STP_ADSL_MASK << XWAY_STP_ADSL_SHIFT, (*chip).dsl as u32 << XWAY_STP_ADSL_SHIFT, XWAY_STP_CON0);
    xway_stp_w32_mask((*chip).virt, XWAY_STP_PHY_MASK << XWAY_STP_PHY1_SHIFT, (*chip).phy1 as u32 << XWAY_STP_PHY1_SHIFT, XWAY_STP_CON0);
    xway_stp_w32_mask((*chip).virt, XWAY_STP_PHY_MASK << XWAY_STP_PHY2_SHIFT, (*chip).phy2 as u32 << XWAY_STP_PHY2_SHIFT, XWAY_STP_CON1);
    if of_machine_is_compatible("lantiq,grx390") || of_machine_is_compatible("lantiq,ar10") { xway_stp_w32_mask((*chip).virt, XWAY_STP_PHY_MASK << XWAY_STP_PHY3_SHIFT, (*chip).phy3 as u32 << XWAY_STP_PHY3_SHIFT, XWAY_STP_CON1); }
    if of_machine_is_compatible("lantiq,grx390") { xway_stp_w32_mask((*chip).virt, XWAY_STP_PHY_MASK << XWAY_STP_PHY4_SHIFT, (*chip).phy4 as u32 << XWAY_STP_PHY4_SHIFT, XWAY_STP_CON1); }
    (*chip).reserved = (((*chip).phy4 as u16 << 11) | ((*chip).phy3 as u16 << 8) | ((*chip).phy2 as u16 << 5) | ((*chip).phy1 as u16 << 2) | (*chip).dsl as u16) as u8;
    if (*chip).reserved != 0 {
        xway_stp_w32_mask((*chip).virt, XWAY_STP_UPD_MASK, XWAY_STP_UPD_FPI, XWAY_STP_CON1);
        xway_stp_w32_mask((*chip).virt, XWAY_STP_SPEED_MASK, XWAY_STP_10HZ, XWAY_STP_CON1);
        xway_stp_w32_mask((*chip).virt, XWAY_STP_FPIS_MASK, XWAY_STP_FPIS_VALUE, XWAY_STP_CON1);
    }
}

// External kernel declarations and platform-driver registration are supplied by the surrounding kernel bindings.
unsafe fn xway_stp_probe(pdev: *mut platform_device) -> i32 {
    let mut shadow: u32 = 0;
    let mut groups: u32 = 0;
    let mut dsl: u32 = 0;
    let mut phy: u32 = 0;
    let mut chip = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<xway_stp>(), GFP_KERNEL) as *mut xway_stp;
    if chip.is_null() { return -ENOMEM; }
    (*chip).virt = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*chip).virt) { return ptr_err((*chip).virt); }
    (*chip).gc.parent = &mut (*pdev).dev;
    (*chip).gc.label = "stp-xway";
    (*chip).gc.direction_output = Some(xway_stp_dir_out);
    (*chip).gc.get = Some(xway_stp_get);
    (*chip).gc.set = Some(xway_stp_set);
    (*chip).gc.request = Some(xway_stp_request);
    (*chip).gc.base = -1;
    (*chip).gc.owner = THIS_MODULE;
    if of_property_read_u32((*pdev).dev.of_node, "lantiq,shadow", &mut shadow) == 0 { (*chip).shadow = shadow; }
    if of_property_read_u32((*pdev).dev.of_node, "lantiq,groups", &mut groups) == 0 { (*chip).groups = (groups & XWAY_STP_GROUP_MASK) as u8; } else { (*chip).groups = XWAY_STP_GROUP0 as u8; }
    (*chip).gc.ngpio = fls((*chip).groups as u32) * 8;
    if of_property_read_u32((*pdev).dev.of_node, "lantiq,dsl", &mut dsl) == 0 { (*chip).dsl = (dsl & XWAY_STP_ADSL_MASK) as u8; }
    if of_machine_is_compatible("lantiq,ar9") || of_machine_is_compatible("lantiq,gr9") || of_machine_is_compatible("lantiq,vr9") || of_machine_is_compatible("lantiq,ar10") || of_machine_is_compatible("lantiq,grx390") {
        if of_property_read_u32((*pdev).dev.of_node, "lantiq,phy1", &mut phy) == 0 { (*chip).phy1 = (phy & XWAY_STP_PHY_MASK) as u8; }
        if of_property_read_u32((*pdev).dev.of_node, "lantiq,phy2", &mut phy) == 0 { (*chip).phy2 = (phy & XWAY_STP_PHY_MASK) as u8; }
    }
    if of_machine_is_compatible("lantiq,ar10") || of_machine_is_compatible("lantiq,grx390") { if of_property_read_u32((*pdev).dev.of_node, "lantiq,phy3", &mut phy) == 0 { (*chip).phy3 = (phy & XWAY_STP_PHY_MASK) as u8; } }
    if of_machine_is_compatible("lantiq,grx390") { if of_property_read_u32((*pdev).dev.of_node, "lantiq,phy4", &mut phy) == 0 { (*chip).phy4 = (phy & XWAY_STP_PHY_MASK) as u8; } }
    if !of_property_read_bool((*pdev).dev.of_node, "lantiq,rising") { (*chip).edge = XWAY_STP_FALLING; }
    let clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if is_err(clk) { dev_err(&mut (*pdev).dev, "Failed to get clock\n"); return ptr_err(clk); }
    xway_stp_hw_init(chip);
    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*chip).gc, chip as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    dev_info(&mut (*pdev).dev, "Init done\n");
    0
}

static XWAY_STP_MATCH: [of_device_id; 2] = [of_device_id { compatible: "lantiq,gpio-stp-xway" }, of_device_id { compatible: "" }];
static mut XWAY_STP_DRIVER: platform_driver = platform_driver { probe: Some(xway_stp_probe), driver: driver { name: "gpio-stp-xway", of_match_table: XWAY_STP_MATCH.as_ptr() } };

unsafe fn xway_stp_init() -> i32 { platform_driver_register(&mut XWAY_STP_DRIVER) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
