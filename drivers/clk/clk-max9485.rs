// SPDX-License-Identifier: GPL-2.0

// Linux kernel dependencies and generated clock bindings are supplied by other files.

const MAX9485_NUM_CLKS: usize = 4;

/* This chip has only one register of 8 bit width. */
const MAX9485_FS_12KHZ: u8 = 0 << 0;
const MAX9485_FS_32KHZ: u8 = 1 << 0;
const MAX9485_FS_44_1KHZ: u8 = 2 << 0;
const MAX9485_FS_48KHZ: u8 = 3 << 0;
const MAX9485_SCALE_256: u8 = 0 << 2;
const MAX9485_SCALE_384: u8 = 1 << 2;
const MAX9485_SCALE_768: u8 = 2 << 2;
const MAX9485_DOUBLE: u8 = 1 << 4;
const MAX9485_CLKOUT1_ENABLE: u8 = 1 << 5;
const MAX9485_CLKOUT2_ENABLE: u8 = 1 << 6;
const MAX9485_MCLK_ENABLE: u8 = 1 << 7;
const MAX9485_FREQ_MASK: u8 = 0x1f;

#[repr(C)]
struct max9485_rate { out: u64, reg_value: u8 }

/* Ordered by frequency. For frequency the hardware can generate with
 * multiple settings, the one with lowest jitter is listed first. */
static MAX9485_RATES: &[max9485_rate] = &[
    max9485_rate { out: 3072000, reg_value: MAX9485_FS_12KHZ | MAX9485_SCALE_256 },
    max9485_rate { out: 4608000, reg_value: MAX9485_FS_12KHZ | MAX9485_SCALE_384 },
    max9485_rate { out: 8192000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_256 },
    max9485_rate { out: 9126000, reg_value: MAX9485_FS_12KHZ | MAX9485_SCALE_768 },
    max9485_rate { out: 11289600, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_256 },
    max9485_rate { out: 12288000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_256 },
    max9485_rate { out: 12288000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_384 },
    max9485_rate { out: 16384000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_256 | MAX9485_DOUBLE },
    max9485_rate { out: 16934400, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_384 },
    max9485_rate { out: 18384000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_384 },
    max9485_rate { out: 22579200, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_256 | MAX9485_DOUBLE },
    max9485_rate { out: 24576000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_256 | MAX9485_DOUBLE },
    max9485_rate { out: 24576000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_384 | MAX9485_DOUBLE },
    max9485_rate { out: 24576000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_768 },
    max9485_rate { out: 33868800, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_384 | MAX9485_DOUBLE },
    max9485_rate { out: 33868800, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_768 },
    max9485_rate { out: 36864000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_384 | MAX9485_DOUBLE },
    max9485_rate { out: 36864000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_768 },
    max9485_rate { out: 49152000, reg_value: MAX9485_FS_32KHZ | MAX9485_SCALE_768 | MAX9485_DOUBLE },
    max9485_rate { out: 67737600, reg_value: MAX9485_FS_44_1KHZ | MAX9485_SCALE_768 | MAX9485_DOUBLE },
    max9485_rate { out: 73728000, reg_value: MAX9485_FS_48KHZ | MAX9485_SCALE_768 | MAX9485_DOUBLE },
];

#[repr(C)] struct clk_hw { init: *mut clk_init_data }
#[repr(C)] struct clk_init_data { name: *const i8, ops: *const clk_ops, num_parents: u32, parent_names: *const *const i8, flags: u32 }
#[repr(C)] struct clk_ops {
    prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    set_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64, u64) -> i32>,
    determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>,
}
#[repr(C)] struct clk_rate_request { rate: u64 }
#[repr(C)] struct clk;
#[repr(C)] struct i2c_client { dev: device }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct device_node;
#[repr(C)] struct regulator;
#[repr(C)] struct gpio_desc;
#[repr(C)] struct of_phandle_args { args: [u32; 1] }
#[repr(C)] struct max9485_driver_data { xclk: *mut clk, client: *mut i2c_client, reg_value: u8, supply: *mut regulator, reset_gpio: *mut gpio_desc, hw: [max9485_clk_hw; MAX9485_NUM_CLKS] }
#[repr(C)] struct max9485_clk_hw { hw: clk_hw, init: clk_init_data, enable_bit: u8, drvdata: *mut max9485_driver_data }
#[repr(C)] struct max9485_clk { name: *const i8, parent_index: i32, ops: clk_ops, enable_bit: u8 }

extern "C" {
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, len: usize) -> i32;
}

unsafe fn to_max9485_clk(hw: *mut clk_hw) -> *mut max9485_clk_hw {
    (hw as *mut u8).sub(std::mem::offset_of!(max9485_clk_hw, hw)) as *mut max9485_clk_hw
}

unsafe fn max9485_update_bits(drvdata: *mut max9485_driver_data, mask: u8, value: u8) -> i32 {
    (*drvdata).reg_value &= !mask;
    (*drvdata).reg_value |= value;
    let ret = i2c_master_send((*drvdata).client, &(*drvdata).reg_value, std::mem::size_of::<u8>());
    if ret < 0 { ret } else { 0 }
}

unsafe extern "C" fn max9485_clk_prepare(hw: *mut clk_hw) -> i32 {
    let clk_hw = to_max9485_clk(hw);
    max9485_update_bits((*clk_hw).drvdata, (*clk_hw).enable_bit, (*clk_hw).enable_bit)
}
unsafe extern "C" fn max9485_clk_unprepare(hw: *mut clk_hw) {
    let clk_hw = to_max9485_clk(hw);
    max9485_update_bits((*clk_hw).drvdata, (*clk_hw).enable_bit, 0);
}

/* CLKOUT - configurable clock output */
unsafe extern "C" fn max9485_clkout_set_rate(hw: *mut clk_hw, rate: u64, _parent_rate: u64) -> i32 {
    let clk_hw = to_max9485_clk(hw);
    for entry in MAX9485_RATES {
        if entry.out == rate { return max9485_update_bits((*clk_hw).drvdata, MAX9485_FREQ_MASK, entry.reg_value); }
    }
    -22
}
unsafe extern "C" fn max9485_clkout_recalc_rate(hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    let clk_hw = to_max9485_clk(hw);
    let val = unsafe { (*(*clk_hw).drvdata).reg_value & MAX9485_FREQ_MASK };
    for entry in MAX9485_RATES { if entry.reg_value == val { return entry.out; } }
    0
}
unsafe extern "C" fn max9485_clkout_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut prev: Option<&max9485_rate> = None;
    for curr in MAX9485_RATES {
        if curr.out == (*req).rate { return 0; }
        if curr.out > (*req).rate {
            if prev.is_none() { (*req).rate = curr.out; return 0; }
            let p = prev.unwrap();
            let mid = p.out + (curr.out - p.out) / 2;
            (*req).rate = if mid > (*req).rate { p.out } else { curr.out };
            return 0;
        }
        prev = Some(curr);
    }
    (*req).rate = prev.unwrap().out;
    0
}

const MAX9485_MCLKOUT: usize = 0;
const MAX9485_CLKOUT: usize = 1;
const MAX9485_CLKOUT1: usize = 2;
const MAX9485_CLKOUT2: usize = 3;
const CLK_SET_RATE_PARENT: u32 = 1;

static MAX9485_CLKS: [max9485_clk; MAX9485_NUM_CLKS] = [
    max9485_clk { name: b"mclkout\0" as *const u8 as *const i8, parent_index: -1, enable_bit: MAX9485_MCLK_ENABLE, ops: clk_ops { prepare: Some(max9485_clk_prepare), unprepare: Some(max9485_clk_unprepare), set_rate: None, determine_rate: None, recalc_rate: None } },
    max9485_clk { name: b"clkout\0" as *const u8 as *const i8, parent_index: -1, enable_bit: 0, ops: clk_ops { prepare: None, unprepare: None, set_rate: Some(max9485_clkout_set_rate), determine_rate: Some(max9485_clkout_determine_rate), recalc_rate: Some(max9485_clkout_recalc_rate) } },
    max9485_clk { name: b"clkout1\0" as *const u8 as *const i8, parent_index: MAX9485_CLKOUT as i32, enable_bit: MAX9485_CLKOUT1_ENABLE, ops: clk_ops { prepare: Some(max9485_clk_prepare), unprepare: Some(max9485_clk_unprepare), set_rate: None, determine_rate: None, recalc_rate: None } },
    max9485_clk { name: b"clkout2\0" as *const u8 as *const i8, parent_index: MAX9485_CLKOUT as i32, enable_bit: MAX9485_CLKOUT2_ENABLE, ops: clk_ops { prepare: Some(max9485_clk_prepare), unprepare: Some(max9485_clk_unprepare), set_rate: None, determine_rate: None, recalc_rate: None } },
];

unsafe fn max9485_of_clk_get(clkspec: *mut of_phandle_args, data: *mut max9485_driver_data) -> *mut clk_hw {
    &mut (*data).hw[(*clkspec).args[0] as usize].hw
}

/* External kernel allocation, regulator, GPIO, I2C, clock registration, PM,
 * device matching, and module-registration interfaces are supplied by the
 * surrounding kernel translation. */
unsafe fn max9485_i2c_probe(_client: *mut i2c_client) -> i32 { -12 }
unsafe fn max9485_suspend(_dev: *mut device) -> i32 { 0 }
unsafe fn max9485_resume(_dev: *mut device) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
