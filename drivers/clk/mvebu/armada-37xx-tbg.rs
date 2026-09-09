// SPDX-License-Identifier: GPL-2.0+
/*
 * Marvell Armada 37xx SoC Time Base Generator clocks
 *
 * Copyright (C) 2016 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 */

// Linux kernel dependencies supplied by the surrounding repository.

const NUM_TBG: usize = 4;

const TBG_CTRL0: usize = 0x4;
const TBG_CTRL1: usize = 0x8;
const TBG_CTRL7: usize = 0x20;
const TBG_CTRL8: usize = 0x30;

const TBG_DIV_MASK: u32 = 0x1FF;

const TBG_A_REFDIV: u32 = 0;
const TBG_B_REFDIV: u32 = 16;

const TBG_A_FBDIV: u32 = 2;
const TBG_B_FBDIV: u32 = 18;

const TBG_A_VCODIV_SE: u32 = 0;
const TBG_B_VCODIV_SE: u32 = 16;

const TBG_A_VCODIV_DIFF: u32 = 1;
const TBG_B_VCODIV_DIFF: u32 = 17;

#[repr(C)]
struct tbg_def {
    name: *mut i8,
    refdiv_offset: u32,
    fbdiv_offset: u32,
    vcodiv_reg: u32,
    vcodiv_offset: u32,
}

static mut tbg: [tbg_def; NUM_TBG] = [
    tbg_def { name: b"TBG-A-P\0" as *const u8 as *mut i8, refdiv_offset: TBG_A_REFDIV, fbdiv_offset: TBG_A_FBDIV, vcodiv_reg: TBG_CTRL8 as u32, vcodiv_offset: TBG_A_VCODIV_DIFF },
    tbg_def { name: b"TBG-B-P\0" as *const u8 as *mut i8, refdiv_offset: TBG_B_REFDIV, fbdiv_offset: TBG_B_FBDIV, vcodiv_reg: TBG_CTRL8 as u32, vcodiv_offset: TBG_B_VCODIV_DIFF },
    tbg_def { name: b"TBG-A-S\0" as *const u8 as *mut i8, refdiv_offset: TBG_A_REFDIV, fbdiv_offset: TBG_A_FBDIV, vcodiv_reg: TBG_CTRL1 as u32, vcodiv_offset: TBG_A_VCODIV_SE },
    tbg_def { name: b"TBG-B-S\0" as *const u8 as *mut i8, refdiv_offset: TBG_B_REFDIV, fbdiv_offset: TBG_B_FBDIV, vcodiv_reg: TBG_CTRL1 as u32, vcodiv_offset: TBG_B_VCODIV_SE },
];

unsafe fn tbg_get_mult(reg: *mut u8, ptbg: *const tbg_def) -> u32 {
    let val: u32 = readl(reg.add(TBG_CTRL0));
    ((val >> (*ptbg).fbdiv_offset) & TBG_DIV_MASK) << 2
}

unsafe fn tbg_get_div(reg: *mut u8, ptbg: *const tbg_def) -> u32 {
    let mut val: u32 = readl(reg.add(TBG_CTRL7));
    let mut div = (val >> (*ptbg).refdiv_offset) & TBG_DIV_MASK;
    if div == 0 {
        div = 1;
    }
    val = readl(reg.add((*ptbg).vcodiv_reg as usize));
    div *= 1 << ((val >> (*ptbg).vcodiv_offset) & TBG_DIV_MASK);
    div
}

unsafe fn armada_3700_tbg_clock_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let mut hw_tbg_data = devm_kzalloc(&mut (*pdev).dev, struct_size(NUM_TBG), GFP_KERNEL);
    if hw_tbg_data.is_null() {
        return -ENOMEM;
    }
    (*hw_tbg_data).num = NUM_TBG;
    platform_set_drvdata(pdev, hw_tbg_data);

    let parent = clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR(parent) {
        dev_err(&mut (*pdev).dev, "Could get the clock parent\n");
        return -EINVAL;
    }
    let parent_name = __clk_get_name(parent);
    clk_put(parent);

    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) {
        return PTR_ERR(reg);
    }

    for i in 0..NUM_TBG {
        let name = tbg[i].name;
        let mult = tbg_get_mult(reg, &tbg[i]);
        let div = tbg_get_div(reg, &tbg[i]);
        (*hw_tbg_data).hws[i] = clk_hw_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, div);
        if IS_ERR((*hw_tbg_data).hws[i]) {
            dev_err(&mut (*pdev).dev, "Can't register TBG clock %s\n", name);
        }
    }

    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, hw_tbg_data)
}

unsafe fn armada_3700_tbg_clock_remove(pdev: *mut platform_device) {
    let hw_tbg_data = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    for i in 0..(*hw_tbg_data).num {
        clk_hw_unregister_fixed_factor((*hw_tbg_data).hws[i]);
    }
}

static armada_3700_tbg_clock_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"marvell,armada-3700-tbg-clock\0" },
    of_device_id { compatible: core::ptr::null() },
];

static mut armada_3700_tbg_clock_driver: platform_driver = platform_driver {
    probe: Some(armada_3700_tbg_clock_probe),
    remove: Some(armada_3700_tbg_clock_remove),
    driver: driver {
        name: b"marvell-armada-3700-tbg-clock\0" as *const u8 as *const i8,
        of_match_table: armada_3700_tbg_clock_of_match.as_ptr(),
    },
};

builtin_platform_driver!(armada_3700_tbg_clock_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
