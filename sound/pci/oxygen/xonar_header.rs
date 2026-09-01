/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependency: "oxygen.h" */

#[repr(C)]
pub struct xonar_generic {
    pub anti_pop_delay: u32,
    pub output_enable_bit: u16,
    pub ext_power_reg: u8,
    pub ext_power_int_reg: u8,
    pub ext_power_bit: u8,
    pub has_power: u8,
}

#[repr(C)]
pub struct xonar_hdmi {
    pub params: [u8; 5],
}

/* generic helper functions */

unsafe extern "C" {
    pub fn xonar_enable_output(chip: *mut oxygen);
    pub fn xonar_disable_output(chip: *mut oxygen);
    pub fn xonar_init_ext_power(chip: *mut oxygen);
    pub fn xonar_init_cs53x1(chip: *mut oxygen);
    pub fn xonar_set_cs53x1_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params);
}

pub const XONAR_GPIO_BIT_INVERT: i32 = 1 << 16;

unsafe extern "C" {
    pub fn xonar_gpio_bit_switch_get(
        ctl: *mut snd_kcontrol,
        value: *mut snd_ctl_elem_value,
    ) -> i32;
    pub fn xonar_gpio_bit_switch_put(
        ctl: *mut snd_kcontrol,
        value: *mut snd_ctl_elem_value,
    ) -> i32;
}

/* model-specific card drivers */

unsafe extern "C" {
    pub fn get_xonar_pcm179x_model(chip: *mut oxygen, id: *const pci_device_id) -> i32;
    pub fn get_xonar_cs43xx_model(chip: *mut oxygen, id: *const pci_device_id) -> i32;
    pub fn get_xonar_wm87x6_model(chip: *mut oxygen, id: *const pci_device_id) -> i32;
}

/* HDMI helper functions */

unsafe extern "C" {
    pub fn xonar_hdmi_init(chip: *mut oxygen, data: *mut xonar_hdmi);
    pub fn xonar_hdmi_cleanup(chip: *mut oxygen);
    pub fn xonar_hdmi_resume(chip: *mut oxygen, hdmi: *mut xonar_hdmi);
    pub fn xonar_hdmi_pcm_hardware_filter(channel: u32, hardware: *mut snd_pcm_hardware);
    pub fn xonar_set_hdmi_params(
        chip: *mut oxygen,
        hdmi: *mut xonar_hdmi,
        params: *mut snd_pcm_hw_params,
    );
    pub fn xonar_hdmi_uart_input(chip: *mut oxygen);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
