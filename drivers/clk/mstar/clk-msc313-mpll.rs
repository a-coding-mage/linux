// SPDX-License-Identifier: GPL-2.0
/*
 * MStar MSC313 MPLL driver
 *
 * Copyright (C) 2020 Daniel Palmer <daniel@thingy.jp>
 */

// Linux kernel dependencies from the original implementation are supplied by
// the surrounding Rust environment.

const REG_CONFIG1: u32 = 0x8;
const REG_CONFIG2: u32 = 0xc;

static MSC313_MPLL_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 16,
    val_bits: 16,
    reg_stride: 4,
};

static CONFIG1_LOOP_DIV_FIRST: RegField = reg_field(REG_CONFIG1, 8, 9);
static CONFIG1_INPUT_DIV_FIRST: RegField = reg_field(REG_CONFIG1, 4, 5);
static CONFIG2_OUTPUT_DIV_FIRST: RegField = reg_field(REG_CONFIG2, 12, 13);
static CONFIG2_LOOP_DIV_SECOND: RegField = reg_field(REG_CONFIG2, 0, 7);

static OUTPUT_DIVIDERS: [u32; 7] = [2, 3, 4, 5, 6, 7, 10];

const NUMOUTPUTS: usize = OUTPUT_DIVIDERS.len() + 1;

#[repr(C)]
struct Msc313Mpll {
    clk_hw: ClkHw,
    input_div: *mut RegmapField,
    loop_div_first: *mut RegmapField,
    loop_div_second: *mut RegmapField,
    output_div: *mut RegmapField,
    clk_data: *mut ClkHwOnecellData,
}

#[inline]
unsafe fn to_mpll(hw: *mut ClkHw) -> *mut Msc313Mpll {
    container_of!(hw, Msc313Mpll, clk_hw)
}

unsafe extern "C" fn msc313_mpll_recalc_rate(
    hw: *mut ClkHw,
    parent_rate: c_ulong,
) -> c_ulong {
    let mpll = &mut *to_mpll(hw);
    let mut input_div: u32 = 0;
    let mut output_div: u32 = 0;
    let mut loop_first: u32 = 0;
    let mut loop_second: u32 = 0;

    regmap_field_read(mpll.input_div, &mut input_div);
    regmap_field_read(mpll.output_div, &mut output_div);
    regmap_field_read(mpll.loop_div_first, &mut loop_first);
    regmap_field_read(mpll.loop_div_second, &mut loop_second);

    let mut output_rate = parent_rate / (1u64 << input_div);
    output_rate *= (1u64 << loop_first) * core::cmp::max(loop_second, 1);
    output_rate /= core::cmp::max(output_div, 1) as u64;

    output_rate as c_ulong
}

static MSC313_MPLL_OPS: ClkOps = ClkOps {
    recalc_rate: Some(msc313_mpll_recalc_rate),
};

static MPLL_PARENT: ClkParentData = ClkParentData { index: 0 };

unsafe extern "C" fn msc313_mpll_probe(pdev: *mut PlatformDevice) -> c_int {
    let mut base: *mut core::ffi::c_void;
    let mpll: *mut Msc313Mpll;
    let mut clk_init: ClkInitData = core::mem::zeroed();
    let dev = &mut (*pdev).dev;
    let mut regmap: *mut Regmap;
    let mut outputname: *mut c_char;
    let mut divhw: *mut ClkHw;
    let mut ret: c_int;
    let mut i: usize;

    mpll = devm_kzalloc(dev, core::mem::size_of::<Msc313Mpll>(), GFP_KERNEL) as *mut Msc313Mpll;
    if mpll.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) {
        return ptr_err(base);
    }

    regmap = devm_regmap_init_mmio(dev, base, &MSC313_MPLL_REGMAP_CONFIG);
    if is_err(regmap) {
        return ptr_err(regmap);
    }

    (*mpll).input_div = devm_regmap_field_alloc(dev, regmap, CONFIG1_INPUT_DIV_FIRST);
    if is_err((*mpll).input_div) { return ptr_err((*mpll).input_div); }
    (*mpll).output_div = devm_regmap_field_alloc(dev, regmap, CONFIG2_OUTPUT_DIV_FIRST);
    if is_err((*mpll).output_div) { return ptr_err((*mpll).output_div); }
    (*mpll).loop_div_first = devm_regmap_field_alloc(dev, regmap, CONFIG1_LOOP_DIV_FIRST);
    if is_err((*mpll).loop_div_first) { return ptr_err((*mpll).loop_div_first); }
    (*mpll).loop_div_second = devm_regmap_field_alloc(dev, regmap, CONFIG2_LOOP_DIV_SECOND);
    if is_err((*mpll).loop_div_second) { return ptr_err((*mpll).loop_div_second); }

    (*mpll).clk_data = devm_kzalloc(dev, struct_size!(ClkHwOnecellData, hws, OUTPUT_DIVIDERS.len()), GFP_KERNEL)
        as *mut ClkHwOnecellData;
    if (*mpll).clk_data.is_null() { return -ENOMEM; }

    (*dev).name_into(&mut clk_init.name);
    clk_init.ops = &MSC313_MPLL_OPS;
    clk_init.parent_data = &MPLL_PARENT;
    clk_init.num_parents = 1;
    (*mpll).clk_hw.init = &clk_init;

    ret = devm_clk_hw_register(dev, &mut (*mpll).clk_hw);
    if ret != 0 { return ret; }

    (*(*mpll).clk_data).num = NUMOUTPUTS;
    (*(*mpll).clk_data).hws[0] = &mut (*mpll).clk_hw;

    i = 0;
    while i < OUTPUT_DIVIDERS.len() {
        outputname = devm_kasprintf(dev, GFP_KERNEL, "%s_div_%u", clk_init.name, OUTPUT_DIVIDERS[i]);
        if outputname.is_null() { return -ENOMEM; }
        divhw = devm_clk_hw_register_fixed_factor(dev, outputname, clk_init.name, 0, 1, OUTPUT_DIVIDERS[i]);
        if is_err(divhw) { return ptr_err(divhw); }
        (*(*mpll).clk_data).hws[i + 1] = divhw;
        i += 1;
    }

    platform_set_drvdata(pdev, mpll as *mut core::ffi::c_void);
    devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, (*mpll).clk_data)
}

static MSC313_MPLL_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: c"mstar,msc313-mpll" },
    OfDeviceId::default(),
];

static mut MSC313_MPLL_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: c"mstar-msc313-mpll",
        of_match_table: MSC313_MPLL_OF_MATCH.as_ptr(),
    },
    probe: Some(msc313_mpll_probe),
};

builtin_platform_driver!(MSC313_MPLL_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
