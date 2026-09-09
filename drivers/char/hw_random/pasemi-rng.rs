// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Driver for the PWRficient onchip rng
 */

// Dependencies supplied by the Linux kernel and other translation units.

const SDCRNG_CTL_REG: usize = 0x00;
const SDCRNG_CTL_FVLD_M: u32 = 0x0000f000;
const SDCRNG_CTL_FVLD_S: u32 = 12;
const SDCRNG_CTL_KSZ: u32 = 0x00000800;
const SDCRNG_CTL_RSRC_CRG: u32 = 0x00000010;
const SDCRNG_CTL_RSRC_RRG: u32 = 0x00000000;
const SDCRNG_CTL_CE: u32 = 0x00000004;
const SDCRNG_CTL_RE: u32 = 0x00000002;
const SDCRNG_CTL_DR: u32 = 0x00000001;
const SDCRNG_CTL_SELECT_RRG_RNG: u32 = SDCRNG_CTL_RE | SDCRNG_CTL_RSRC_RRG;
const SDCRNG_CTL_SELECT_CRG_RNG: u32 = SDCRNG_CTL_CE | SDCRNG_CTL_RSRC_CRG;
const SDCRNG_VAL_REG: usize = 0x20;

const MODULE_NAME: &str = "pasemi_rng";

unsafe fn pasemi_rng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let rng_regs = (*rng).r#priv as *mut u8;
    let mut data: i32 = 0;

    for _i in 0..20 {
        data = if (in_le32(rng_regs.add(SDCRNG_CTL_REG)) & SDCRNG_CTL_FVLD_M) != 0 {
            1
        } else {
            0
        };
        if data != 0 || wait == 0 {
            break;
        }
        udelay(10);
    }
    data
}

unsafe fn pasemi_rng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let rng_regs = (*rng).r#priv as *mut u8;
    *data = in_le32(rng_regs.add(SDCRNG_VAL_REG));
    4
}

unsafe fn pasemi_rng_init(rng: *mut hwrng) -> i32 {
    let rng_regs = (*rng).r#priv as *mut u8;
    let ctl: u32 = SDCRNG_CTL_DR | SDCRNG_CTL_SELECT_RRG_RNG | SDCRNG_CTL_KSZ;

    out_le32(rng_regs.add(SDCRNG_CTL_REG), ctl);
    out_le32(rng_regs.add(SDCRNG_CTL_REG), ctl & !SDCRNG_CTL_DR);

    0
}

unsafe fn pasemi_rng_cleanup(rng: *mut hwrng) {
    let rng_regs = (*rng).r#priv as *mut u8;
    let ctl: u32 = SDCRNG_CTL_RE | SDCRNG_CTL_CE;

    out_le32(
        rng_regs.add(SDCRNG_CTL_REG),
        in_le32(rng_regs.add(SDCRNG_CTL_REG)) & !ctl,
    );
}

static mut pasemi_rng: hwrng = hwrng {
    name: MODULE_NAME,
    init: Some(pasemi_rng_init),
    cleanup: Some(pasemi_rng_cleanup),
    data_present: Some(pasemi_rng_data_present),
    data_read: Some(pasemi_rng_data_read),
    ..hwrng::default()
};

unsafe fn rng_probe(pdev: *mut platform_device) -> i32 {
    let rng_regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(rng_regs) {
        return PTR_ERR(rng_regs);
    }

    pasemi_rng.r#priv = rng_regs as usize;

    pr_info!("Registering PA Semi RNG\n");
    devm_hwrng_register(&mut (*pdev).dev, &mut pasemi_rng)
}

static rng_match: [of_device_id; 3] = [
    of_device_id { compatible: "1682m-rng", ..of_device_id::default() },
    of_device_id { compatible: "pasemi,pwrficient-rng", ..of_device_id::default() },
    of_device_id::default(),
];

static mut rng_driver: platform_driver = platform_driver {
    driver: driver {
        name: "pasemi-rng",
        of_match_table: rng_match.as_ptr(),
        ..driver::default()
    },
    probe: Some(rng_probe),
    ..platform_driver::default()
};

// Equivalent of module_platform_driver(rng_driver).

const MODULE_LICENSE: &str = "GPL";
const MODULE_AUTHOR: &str = "Egor Martovetsky <egor@pasemi.com>";
const MODULE_DESCRIPTION: &str = "H/W RNG driver for PA Semi processor";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
