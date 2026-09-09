/* SPDX-License-Identifier: GPL-2.0 */
/* Intel INT3472 ACPI camera sensor power-management support. */

/* C header dependencies are supplied by the surrounding kernel translation. */

#[repr(C)] pub struct acpi_device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct acpi_object { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_lookup { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct regulator_consumer_supply { _private: [u8; 0] }
#[repr(C)] pub struct regulator_dev { _private: [u8; 0] }
#[repr(C)] pub struct regulator_desc { _private: [u8; 0] }
#[repr(C)] pub struct led_classdev { _private: [u8; 0] }
#[repr(C)] pub struct led_lookup_data { _private: [u8; 0] }
#[repr(C)] pub struct gpiod_lookup_table { _private: [u8; 0] }
#[repr(C)] pub struct dmi_system_id { _private: [u8; 0] }

/* FIXME: retained from the C header; used when I2C_DEV_NAME_FORMAT is absent. */
pub const I2C_DEV_NAME_FORMAT: &str = "i2c-%s";

pub const INT3472_GPIO_TYPE_RESET: u8 = 0x00;
pub const INT3472_GPIO_TYPE_POWERDOWN: u8 = 0x01;
pub const INT3472_GPIO_TYPE_STROBE: u8 = 0x02;
pub const INT3472_GPIO_TYPE_POWER_ENABLE: u8 = 0x0b;
pub const INT3472_GPIO_TYPE_CLK_ENABLE: u8 = 0x0c;
pub const INT3472_GPIO_TYPE_PRIVACY_LED: u8 = 0x0d;
pub const INT3472_GPIO_TYPE_DOVDD: u8 = 0x10;
pub const INT3472_GPIO_TYPE_HANDSHAKE: u8 = 0x12;
pub const INT3472_GPIO_TYPE_HOTPLUG_DETECT: u8 = 0x13;

pub const INT3472_PDEV_MAX_NAME_LEN: usize = 23;
pub const INT3472_MAX_SENSOR_GPIOS: usize = 3;
pub const INT3472_MAX_LEDS: usize = 2;
pub const INT3472_MAX_REGULATORS: usize = 3;
pub const GPIO_SUPPLY_NAME_LENGTH: usize = 6;
pub const GPIO_REGULATOR_NAME_LENGTH: usize = 12 + GPIO_SUPPLY_NAME_LENGTH;
pub const GPIO_REGULATOR_SUPPLY_MAP_COUNT: usize = 2;
pub const GPIO_REGULATOR_ENABLE_TIME: u32 = 2 * 1000;
pub const GPIO_REGULATOR_OFF_ON_DELAY: u32 = 2 * 1000;
pub const INT3472_LED_MAX_NAME_LEN: usize = 32;
pub const CIO2_SENSOR_SSDB_MCLKSPEED_OFFSET: usize = 86;

#[repr(C)]
pub struct int3472_cldb {
    pub version: u8,
    /* control logic type: 0 UNKNOWN, 1 DISCRETE(CRD-D), 2 PMIC TPS68470, 3 PMIC uP6641 */
    pub control_logic_type: u8,
    pub control_logic_id: u8,
    pub sensor_card_sku: u8,
    pub reserved: [u8; 10],
    pub clock_source: u8,
    pub reserved2: [u8; 17],
}

#[repr(C)]
pub struct int3472_discrete_quirks {
    /* For models where AVDD GPIO is shared between sensors */
    pub avdd_second_sensor: *const core::ffi::c_char,
}

#[repr(C)]
pub struct int3472_gpio_regulator {
    pub supply_map: [regulator_consumer_supply; GPIO_REGULATOR_SUPPLY_MAP_COUNT * 2],
    pub supply_name_upper: [core::ffi::c_char; GPIO_SUPPLY_NAME_LENGTH],
    pub regulator_name: [core::ffi::c_char; GPIO_REGULATOR_NAME_LENGTH],
    pub rdev: *mut regulator_dev,
    pub rdesc: regulator_desc,
}

#[repr(C)]
pub struct int3472_clock {
    pub clk: *mut clk,
    pub clk_hw: clk_hw,
    pub cl: *mut clk_lookup,
    pub ena_gpio: *mut gpio_desc,
    pub frequency: u32,
    pub imgclk_index: u8,
}

#[repr(C)]
pub struct int3472_led {
    pub classdev: led_classdev,
    pub lookup: led_lookup_data,
    pub name: [core::ffi::c_char; INT3472_LED_MAX_NAME_LEN],
    pub gpio: *mut gpio_desc,
}

#[repr(C)]
pub struct int3472_discrete_device {
    pub adev: *mut acpi_device,
    pub dev: *mut device,
    pub sensor: *mut acpi_device,
    pub sensor_name: *const core::ffi::c_char,
    pub regulators: [int3472_gpio_regulator; INT3472_MAX_REGULATORS],
    pub clock: int3472_clock,
    pub leds: [int3472_led; INT3472_MAX_LEDS],
    pub quirks: int3472_discrete_quirks,
    pub ngpios: core::ffi::c_uint,
    pub n_leds: core::ffi::c_uint,
    pub n_sensor_gpios: core::ffi::c_uint,
    pub n_regulator_gpios: core::ffi::c_uint,
    pub gpios: gpiod_lookup_table,
}

extern "C" {
    pub static skl_int3472_discrete_quirks: [dmi_system_id; 0];
    pub fn skl_int3472_get_acpi_buffer(adev: *mut acpi_device, id: *mut core::ffi::c_char) -> *mut acpi_object;
    pub fn skl_int3472_fill_cldb(adev: *mut acpi_device, cldb: *mut int3472_cldb) -> core::ffi::c_int;
    pub fn skl_int3472_get_sensor_adev_and_name(dev: *mut device, sensor_adev_ret: *mut *mut acpi_device, name_ret: *mut *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn int3472_discrete_parse_crs(int3472: *mut int3472_discrete_device) -> core::ffi::c_int;
    pub fn int3472_discrete_cleanup(int3472: *mut int3472_discrete_device);
    pub fn skl_int3472_register_gpio_clock(int3472: *mut int3472_discrete_device, gpio: *mut gpio_desc) -> core::ffi::c_int;
    pub fn skl_int3472_register_dsm_clock(int3472: *mut int3472_discrete_device) -> core::ffi::c_int;
    pub fn skl_int3472_unregister_clock(int3472: *mut int3472_discrete_device);
    pub fn skl_int3472_register_regulator(int3472: *mut int3472_discrete_device, gpio: *mut gpio_desc, enable_time: core::ffi::c_uint, supply_name: *const core::ffi::c_char, second_sensor: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn skl_int3472_unregister_regulator(int3472: *mut int3472_discrete_device);
    pub fn skl_int3472_register_led(int3472: *mut int3472_discrete_device, gpio: *mut gpio_desc, con_id: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn skl_int3472_unregister_leds(int3472: *mut int3472_discrete_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
