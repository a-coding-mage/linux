// SPDX-License-Identifier: GPL-2.0
/*
 * Freescale SAI BCLK as a generic clock driver
 *
 * Copyright 2020 Michael Walle <michael@walle.cc>
 */

// Linux kernel dependencies supplied by other translation units.

const I2S_CSR: usize = 0x00;
const I2S_CR2: usize = 0x08;
const I2S_MCR: usize = 0x100;
const CSR_BCE_BIT: u32 = 28;
const CSR_TE_BIT: u32 = 31;
const CR2_BCD: u32 = 1 << 24;
const CR2_DIV_SHIFT: u32 = 0;
const CR2_DIV_WIDTH: u32 = 8;
const MCR_MOE: u32 = 1 << 30;

#[repr(C)]
struct fsl_sai_data {
    offset: u32,             // Register offset
    have_mclk: bool,         // Have MCLK control
}

#[repr(C)]
struct fsl_sai_clk {
    data: *const fsl_sai_data,
    bclk_div: clk_divider,
    mclk_div: clk_divider,
    bclk_gate: clk_gate,
    mclk_gate: clk_gate,
    bclk_hw: *mut clk_hw,
    mclk_hw: *mut clk_hw,
    lock: spinlock_t,
}

unsafe fn fsl_sai_of_clk_get(
    clkspec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let sai_clk = data as *mut fsl_sai_clk;

    if (*clkspec).args_count == 0 {
        return (*sai_clk).bclk_hw;
    }

    if (*clkspec).args_count == 1 {
        if (*clkspec).args[0] == 0 {
            return (*sai_clk).bclk_hw;
        }
        if (*(*sai_clk).data).have_mclk && (*clkspec).args[0] == 1 {
            return (*sai_clk).mclk_hw;
        }
    }

    ERR_PTR(-EINVAL)
}

unsafe fn fsl_sai_clk_register(
    dev: *mut device,
    base: *mut core::ffi::c_void,
    lock: *mut spinlock_t,
    div: *mut clk_divider,
    gate: *mut clk_gate,
    hw: *mut *mut clk_hw,
    gate_bit: i32,
    dir_bit: i32,
    div_reg: usize,
    name: *mut i8,
) -> i32 {
    let data = device_get_match_data(dev) as *const fsl_sai_data;
    let mut pdata = clk_parent_data { index: 0 };
    let mut chw: *mut clk_hw;
    let cname: *mut i8;

    (*gate).reg = (base as *mut u8).add((*data).offset as usize + I2S_CSR) as *mut u32;
    (*gate).bit_idx = gate_bit as u8;
    (*gate).lock = lock;

    (*div).reg = (base as *mut u8).add(div_reg) as *mut u32;
    (*div).shift = CR2_DIV_SHIFT as u8;
    (*div).width = CR2_DIV_WIDTH as u8;
    (*div).lock = lock;

    cname = devm_kasprintf(dev, GFP_KERNEL, "%s.%s", of_node_full_name((*dev).of_node), name);
    if cname.is_null() {
        return -ENOMEM;
    }

    /* Set clock direction */
    writel(dir_bit as u32, (base as *mut u8).add(div_reg) as *mut u32);

    chw = devm_clk_hw_register_composite_pdata(
        dev, cname, &mut pdata, 1, core::ptr::null_mut(), core::ptr::null(),
        &mut (*div).hw, &clk_divider_ops, &mut (*gate).hw, &clk_gate_ops,
        CLK_SET_RATE_GATE,
    );
    if IS_ERR(chw) {
        return PTR_ERR(chw);
    }

    *hw = chw;
    0
}

unsafe fn fsl_sai_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let data = device_get_match_data(dev) as *const fsl_sai_data;
    let sai_clk: *mut fsl_sai_clk;
    let mut clk_bus: *mut clk;
    let base: *mut core::ffi::c_void;
    let mut ret: i32;

    sai_clk = devm_kzalloc(dev, core::mem::size_of::<fsl_sai_clk>(), GFP_KERNEL)
        as *mut fsl_sai_clk;
    if sai_clk.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    clk_bus = devm_clk_get_optional_enabled(dev, "bus\0".as_ptr() as *const i8);
    if IS_ERR(clk_bus) {
        return PTR_ERR(clk_bus);
    }

    (*sai_clk).data = data;
    spin_lock_init(&mut (*sai_clk).lock);

    ret = fsl_sai_clk_register(dev, base, &mut (*sai_clk).lock,
        &mut (*sai_clk).bclk_div, &mut (*sai_clk).bclk_gate,
        &mut (*sai_clk).bclk_hw, CSR_BCE_BIT as i32, CR2_BCD as i32,
        (*data).offset as usize + I2S_CR2, "BCLK\0".as_ptr() as *mut i8);
    if ret != 0 { return ret; }

    if (*data).have_mclk {
        ret = fsl_sai_clk_register(dev, base, &mut (*sai_clk).lock,
            &mut (*sai_clk).mclk_div, &mut (*sai_clk).mclk_gate,
            &mut (*sai_clk).mclk_hw, CSR_TE_BIT as i32, MCR_MOE as i32,
            I2S_MCR, "MCLK\0".as_ptr() as *mut i8);
        if ret != 0 { return ret; }
    }

    devm_of_clk_add_hw_provider(dev, fsl_sai_of_clk_get, sai_clk as *mut core::ffi::c_void)
}

static fsl_sai_vf610_data: fsl_sai_data = fsl_sai_data { offset: 0, have_mclk: false };
static fsl_sai_imx8mq_data: fsl_sai_data = fsl_sai_data { offset: 8, have_mclk: true };

static of_fsl_sai_clk_ids: [of_device_id; 3] = [
    of_device_id { compatible: "fsl,vf610-sai-clock", data: &fsl_sai_vf610_data },
    of_device_id { compatible: "fsl,imx8mq-sai-clock", data: &fsl_sai_imx8mq_data },
    of_device_id { compatible: "", data: core::ptr::null() },
];

static mut fsl_sai_clk_driver: platform_driver = platform_driver {
    probe: Some(fsl_sai_clk_probe),
    driver: driver {
        name: "fsl-sai-clk",
        of_match_table: of_fsl_sai_clk_ids.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_fsl_sai_clk_ids);
// module_platform_driver(fsl_sai_clk_driver);
// MODULE_DESCRIPTION("Freescale SAI bitclock-as-a-clock driver");
// MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
// MODULE_ALIAS("platform:fsl-sai-clk");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
