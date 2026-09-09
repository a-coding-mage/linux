// SPDX-License-Identifier: GPL-2.0
/*
 * Clock driver for twl device.
 *
 * inspired by the driver for the Palmas device
 */

// C dependencies supplied by the surrounding kernel translation.

const VREG_STATE: u32 = 2;
const VREG_GRP: u32 = 0;
const TWL6030_CFG_STATE_OFF: u8 = 0x00;
const TWL6030_CFG_STATE_ON: u8 = 0x01;
const TWL6030_CFG_STATE_MASK: u8 = 0x03;
const TWL6030_CFG_STATE_GRP_SHIFT: u32 = 5;
const TWL6030_CFG_STATE_APP_SHIFT: u32 = 2;
const TWL6030_CFG_STATE_APP_MASK: u8 = 0x03 << TWL6030_CFG_STATE_APP_SHIFT;

#[inline]
const fn twl6030_cfg_state_app(v: u8) -> u8 {
    (v & TWL6030_CFG_STATE_APP_MASK) >> TWL6030_CFG_STATE_APP_SHIFT
}

const P1_GRP: u32 = 1 << 0;
const P2_GRP: u32 = 1 << 1;
const P3_GRP: u32 = 1 << 2;
const ALL_GRP: u32 = P1_GRP | P2_GRP | P3_GRP;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum twl_type {
    TWL_TYPE_6030,
    TWL_TYPE_6032,
}

#[repr(C)]
struct twl_clock_info {
    dev: *mut device,
    type_: twl_type,
    base: u8,
    hw: clk_hw,
}

#[inline]
unsafe fn twlclk_read(info: *mut twl_clock_info, slave_subgp: u32, offset: u32) -> i32 {
    let mut value: u8 = 0;
    let status = twl_i2c_read_u8(slave_subgp, &mut value, (*info).base as u32 + offset);
    if status < 0 { status } else { value as i32 }
}

#[inline]
unsafe fn twlclk_write(info: *mut twl_clock_info, slave_subgp: u32, offset: u32, value: u8) -> i32 {
    twl_i2c_write_u8(slave_subgp, value, (*info).base as u32 + offset)
}

#[inline]
unsafe fn to_twl_clks_info(hw: *mut clk_hw) -> *mut twl_clock_info {
    (hw as *mut u8).sub(memoffset_of_twl_clock_info_hw()) as *mut twl_clock_info
}

unsafe fn memoffset_of_twl_clock_info_hw() -> usize {
    core::mem::offset_of!(twl_clock_info, hw)
}

unsafe fn twl_clks_recalc_rate(_hw: *mut clk_hw, _parent_rate: usize) -> usize {
    32768
}

unsafe fn twl6032_clks_prepare(hw: *mut clk_hw) -> i32 {
    let cinfo = to_twl_clks_info(hw);

    if (*cinfo).type_ == twl_type::TWL_TYPE_6030 {
        let grp = twlclk_read(cinfo, TWL_MODULE_PM_RECEIVER, VREG_GRP);
        if grp < 0 { return grp; }

        return twlclk_write(
            cinfo,
            TWL_MODULE_PM_RECEIVER,
            VREG_STATE,
            ((grp as u32) << TWL6030_CFG_STATE_GRP_SHIFT | TWL6030_CFG_STATE_ON as u32) as u8,
        );
    }

    twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE, TWL6030_CFG_STATE_ON)
}

unsafe fn twl6032_clks_unprepare(hw: *mut clk_hw) {
    let cinfo = to_twl_clks_info(hw);
    let ret;

    if (*cinfo).type_ == twl_type::TWL_TYPE_6030 {
        ret = twlclk_write(
            cinfo,
            TWL_MODULE_PM_RECEIVER,
            VREG_STATE,
            (ALL_GRP << TWL6030_CFG_STATE_GRP_SHIFT | TWL6030_CFG_STATE_OFF as u32) as u8,
        );
    } else {
        ret = twlclk_write(cinfo, TWL_MODULE_PM_RECEIVER, VREG_STATE, TWL6030_CFG_STATE_OFF);
    }

    if ret < 0 {
        dev_err((*cinfo).dev, "clk unprepare failed\n");
    }
}

static twl6032_clks_ops: clk_ops = clk_ops {
    prepare: Some(twl6032_clks_prepare),
    unprepare: Some(twl6032_clks_unprepare),
    recalc_rate: Some(twl_clks_recalc_rate),
};

#[repr(C)]
struct twl_clks_data {
    init: clk_init_data,
    base: u8,
}

static twl6032_clks: [twl_clks_data; 3] = [
    twl_clks_data {
        init: clk_init_data { name: "clk32kg", ops: &twl6032_clks_ops, flags: CLK_IGNORE_UNUSED },
        base: 0x8C,
    },
    twl_clks_data {
        init: clk_init_data { name: "clk32kaudio", ops: &twl6032_clks_ops, flags: CLK_IGNORE_UNUSED },
        base: 0x8F,
    },
    twl_clks_data { init: clk_init_data::sentinel(), base: 0 },
];

unsafe fn twl_clks_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let hw_data = twl6032_clks.as_ptr();
    let count = 2usize;

    clk_data = devm_kzalloc(&mut (*pdev).dev, struct_size_clk_hw_onecell_data(count), GFP_KERNEL);
    if clk_data.is_null() { return -ENOMEM; }

    (*clk_data).num = count;
    let cinfo = devm_kcalloc(&mut (*pdev).dev, count, core::mem::size_of::<twl_clock_info>(), GFP_KERNEL);
    if cinfo.is_null() { return -ENOMEM; }

    for i in 0..count {
        (*cinfo.add(i)).base = (*hw_data.add(i)).base;
        (*cinfo.add(i)).dev = &mut (*pdev).dev;
        (*cinfo.add(i)).type_ = platform_get_device_id(pdev).as_ref().unwrap().driver_data;
        (*cinfo.add(i)).hw.init = &(*hw_data.add(i)).init;
        let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*cinfo.add(i)).hw);
        if ret != 0 {
            return dev_err_probe(&mut (*pdev).dev, ret, "Fail to register clock %s\n", (*hw_data.add(i)).init.name);
        }
        (*clk_data).hws[i] = &mut (*cinfo.add(i)).hw;
    }

    let ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_onecell_get, clk_data);
    if ret < 0 { return dev_err_probe(&mut (*pdev).dev, ret, "Fail to add clock driver\n"); }
    0
}

static twl_clks_id: [platform_device_id; 3] = [
    platform_device_id { name: "twl6030-clk", driver_data: twl_type::TWL_TYPE_6030 },
    platform_device_id { name: "twl6032-clk", driver_data: twl_type::TWL_TYPE_6032 },
    platform_device_id::sentinel(),
];

static mut twl_clks_driver: platform_driver = platform_driver {
    driver: driver { name: "twl-clk" },
    probe: Some(twl_clks_probe),
    id_table: twl_clks_id.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, twl_clks_id);
// module_platform_driver(twl_clks_driver);
// MODULE_DESCRIPTION("Clock driver for TWL Series Devices");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
