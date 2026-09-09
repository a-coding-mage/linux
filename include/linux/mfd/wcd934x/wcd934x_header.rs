/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header:
// linux/clk.h, linux/regulator/consumer.h, linux/regmap.h, linux/slimbus.h

pub const WCD934X_MAX_SUPPLY: usize = 5;

/**
 * struct wcd934x_ddata - wcd934x driver data
 *
 * @supplies:  wcd934x regulator supplies
 * @irq_data:  wcd934x irq_chip data
 * @regmap:    wcd934x regmap pointer
 * @extclk:    External clock
 * @dev:       device instance of wcd934x slim device
 * @irq:       irq for wcd934x.
 */
#[repr(C)]
pub struct wcd934x_ddata {
    pub supplies: [regulator_bulk_data; WCD934X_MAX_SUPPLY],
    pub irq_data: *mut regmap_irq_chip_data,
    pub regmap: *mut regmap,
    pub extclk: *mut clk,
    pub dev: *mut device,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
