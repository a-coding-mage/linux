// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * extcon-rt8973a.c - Richtek RT8973A extcon driver to support USB switches
 *
 * Copyright (c) 2014 Samsung Electronics Co., Ltd
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 */

// Linux kernel dependencies and "extcon-rt8973a.h" are supplied externally.

const DELAY_MS_DEFAULT: u32 = 20000;

#[repr(C)]
struct muic_irq { irq: u32, name: *const core::ffi::c_char, virq: u32 }

#[repr(C)]
struct reg_data { reg: u8, mask: u8, val: u8, invert: bool }

#[repr(C)]
struct rt8973a_muic_info {
    dev: *mut device,
    edev: *mut extcon_dev,
    i2c: *mut i2c_client,
    regmap: *mut regmap,
    irq_data: *mut regmap_irq_chip_data,
    muic_irqs: *mut muic_irq,
    num_muic_irqs: u32,
    irq: i32,
    irq_attach: bool,
    irq_detach: bool,
    irq_ovp: bool,
    irq_otp: bool,
    irq_work: work_struct,
    reg_data: *mut reg_data,
    num_reg_data: u32,
    auto_config: bool,
    mutex: mutex,
    wq_detcable: delayed_work,
}

static mut rt8973a_reg_data: [reg_data; 2] = [
    reg_data { reg: RT8973A_REG_CONTROL1, mask: RT8973A_REG_CONTROL1_ADC_EN_MASK | RT8973A_REG_CONTROL1_USB_CHD_EN_MASK | RT8973A_REG_CONTROL1_CHGTYP_MASK | RT8973A_REG_CONTROL1_SWITCH_OPEN_MASK | RT8973A_REG_CONTROL1_AUTO_CONFIG_MASK | RT8973A_REG_CONTROL1_INTM_MASK, val: RT8973A_REG_CONTROL1_ADC_EN_MASK | RT8973A_REG_CONTROL1_USB_CHD_EN_MASK | RT8973A_REG_CONTROL1_CHGTYP_MASK, invert: false },
    reg_data { reg: 0, mask: 0, val: 0, invert: false },
];

static rt8973a_extcon_cable: [u32; 6] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_DCP, EXTCON_JIG, EXTCON_NONE];

#[repr(i32)]
enum rt8973a_event_type { RT8973A_EVENT_ATTACH = 1, RT8973A_EVENT_DETACH, RT8973A_EVENT_OVP, RT8973A_EVENT_OTP }

#[repr(u32)]
enum rt8973a_muic_acc_type {
    RT8973A_MUIC_ADC_OTG = 0x0,
    RT8973A_MUIC_ADC_AUDIO_SEND_END_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S1_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S2_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S3_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S4_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S5_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S6_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S7_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S8_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S9_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S10_BUTTON, RT8973A_MUIC_ADC_AUDIO_REMOTE_S11_BUTTON,
    RT8973A_MUIC_ADC_AUDIO_REMOTE_S12_BUTTON, RT8973A_MUIC_ADC_RESERVED_ACC_1,
    RT8973A_MUIC_ADC_RESERVED_ACC_2, RT8973A_MUIC_ADC_RESERVED_ACC_3,
    RT8973A_MUIC_ADC_RESERVED_ACC_4, RT8973A_MUIC_ADC_RESERVED_ACC_5,
    RT8973A_MUIC_ADC_AUDIO_TYPE2, RT8973A_MUIC_ADC_PHONE_POWERED_DEV,
    RT8973A_MUIC_ADC_UNKNOWN_ACC_1, RT8973A_MUIC_ADC_UNKNOWN_ACC_2,
    RT8973A_MUIC_ADC_TA, RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_OFF_USB,
    RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_ON_USB, RT8973A_MUIC_ADC_UNKNOWN_ACC_3,
    RT8973A_MUIC_ADC_UNKNOWN_ACC_4, RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_OFF_UART,
    RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_ON_UART, RT8973A_MUIC_ADC_UNKNOWN_ACC_5,
    RT8973A_MUIC_ADC_OPEN = 0x1f, RT8973A_MUIC_ADC_USB = 0x3f,
}

static mut rt8973a_muic_irqs: [muic_irq; 15] = [
    muic_irq { irq: RT8973A_INT1_ATTACH, name: c"muic-attach".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_DETACH, name: c"muic-detach".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_CHGDET, name: c"muic-chgdet".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_DCD_T, name: c"muic-dcd-t".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_OVP, name: c"muic-ovp".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_CONNECT, name: c"muic-connect".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_ADC_CHG, name: c"muic-adc-chg".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT1_OTP, name: c"muic-otp".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_UVLO, name: c"muic-uvlo".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_POR, name: c"muic-por".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_OTP_FET, name: c"muic-otp-fet".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_OVP_FET, name: c"muic-ovp-fet".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_OCP_LATCH, name: c"muic-ocp-latch".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_OCP, name: c"muic-ocp".as_ptr(), virq: 0 }, muic_irq { irq: RT8973A_INT2_OVP_OCP, name: c"muic-ovp-ocp".as_ptr(), virq: 0 },
];

// The following declarations preserve the source implementation; kernel APIs and constants are external dependencies.
unsafe fn rt8973a_muic_set_path(info: *mut rt8973a_muic_info, mut con_sw: u32, attached: bool) -> i32 {
    if (*info).auto_config { return 0; }
    if !attached { con_sw = DM_DP_SWITCH_UART; }
    match con_sw {
        DM_DP_SWITCH_OPEN | DM_DP_SWITCH_USB | DM_DP_SWITCH_UART => {
            let ret = regmap_update_bits((*info).regmap, RT8973A_REG_MANUAL_SW1, RT8973A_REG_MANUAL_SW1_DP_MASK | RT8973A_REG_MANUAL_SW1_DM_MASK, con_sw);
            if ret < 0 { dev_err!((*info).dev, "cannot update DM_CON/DP_CON switch\n"); return ret; }
        },
        _ => { dev_err!((*info).dev, "Unknown DM_CON/DP_CON switch type ({})\n", con_sw); return -EINVAL; }
    } 0
}

unsafe fn rt8973a_muic_get_cable_type(info: *mut rt8973a_muic_info) -> i32 {
    let mut adc = 0; let mut dev1 = 0;
    if regmap_read((*info).regmap, RT8973A_REG_ADC, &mut adc) != 0 { dev_err!((*info).dev, "failed to read ADC register\n"); return -EIO; }
    let mut cable_type = adc & RT8973A_REG_ADC_MASK;
    if regmap_read((*info).regmap, RT8973A_REG_DEV1, &mut dev1) != 0 { dev_err!((*info).dev, "failed to read DEV1 register\n"); return -EIO; }
    if adc == RT8973A_MUIC_ADC_OPEN { cable_type = if dev1 & RT8973A_REG_DEV1_USB_MASK != 0 { RT8973A_MUIC_ADC_USB } else if dev1 & RT8973A_REG_DEV1_DCPORT_MASK != 0 { RT8973A_MUIC_ADC_TA } else { RT8973A_MUIC_ADC_OPEN }; }
    cable_type as i32
}

unsafe fn rt8973a_muic_cable_handler(info: *mut rt8973a_muic_info, event: rt8973a_event_type) -> i32 {
    static mut prev_cable_type: u32 = 0; let (cable_type, attached) = match event {
        rt8973a_event_type::RT8973A_EVENT_ATTACH => (rt8973a_muic_get_cable_type(info) as u32, true),
        rt8973a_event_type::RT8973A_EVENT_DETACH => (prev_cable_type, false),
        rt8973a_event_type::RT8973A_EVENT_OVP | rt8973a_event_type::RT8973A_EVENT_OTP => { dev_warn!((*info).dev, "happen Over issue. Need to disconnect all cables\n"); (prev_cable_type, false) },
    }; prev_cable_type = cable_type;
    let (id, sw) = match cable_type {
        RT8973A_MUIC_ADC_OTG => (EXTCON_USB_HOST, DM_DP_SWITCH_USB), RT8973A_MUIC_ADC_TA => (EXTCON_CHG_USB_DCP, DM_DP_SWITCH_OPEN),
        RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_OFF_USB | RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_ON_USB | RT8973A_MUIC_ADC_USB => (EXTCON_USB, DM_DP_SWITCH_USB),
        RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_OFF_UART | RT8973A_MUIC_ADC_FACTORY_MODE_BOOT_ON_UART => (EXTCON_JIG, DM_DP_SWITCH_UART),
        RT8973A_MUIC_ADC_OPEN | RT8973A_MUIC_ADC_UNKNOWN_ACC_1 | RT8973A_MUIC_ADC_UNKNOWN_ACC_2 | RT8973A_MUIC_ADC_UNKNOWN_ACC_3 | RT8973A_MUIC_ADC_UNKNOWN_ACC_4 | RT8973A_MUIC_ADC_UNKNOWN_ACC_5 | RT8973A_MUIC_ADC_AUDIO_SEND_END_BUTTON..=RT8973A_MUIC_ADC_AUDIO_TYPE2 | RT8973A_MUIC_ADC_RESERVED_ACC_1..=RT8973A_MUIC_ADC_PHONE_POWERED_DEV => return 0,
        _ => return -EINVAL,
    }; let ret = rt8973a_muic_set_path(info, sw, attached); if ret < 0 { return ret; } extcon_set_state_sync((*info).edev, id, attached); if id == EXTCON_USB { extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_SDP, attached); } 0
}

// Direct low-level translation of the remaining driver entry points.
unsafe fn rt8973a_muic_irq_work(work: *mut work_struct) {
    let info = container_of!(work, rt8973a_muic_info, irq_work);
    if (*info).edev.is_null() { return; }
    mutex_lock(&mut (*info).mutex);
    let mut ret = 0;
    if (*info).irq_attach { ret = rt8973a_muic_cable_handler(info, rt8973a_event_type::RT8973A_EVENT_ATTACH); (*info).irq_attach = false; }
    if (*info).irq_detach { ret = rt8973a_muic_cable_handler(info, rt8973a_event_type::RT8973A_EVENT_DETACH); (*info).irq_detach = false; }
    if (*info).irq_ovp { ret = rt8973a_muic_cable_handler(info, rt8973a_event_type::RT8973A_EVENT_OVP); (*info).irq_ovp = false; }
    if (*info).irq_otp { ret = rt8973a_muic_cable_handler(info, rt8973a_event_type::RT8973A_EVENT_OTP); (*info).irq_otp = false; }
    if ret < 0 { dev_err!((*info).dev, "failed to handle MUIC interrupt\n"); }
    mutex_unlock(&mut (*info).mutex);
}

unsafe fn rt8973a_muic_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let info = data as *mut rt8973a_muic_info;
    let mut irq_type: i32 = -1;
    for i in 0..(*info).num_muic_irqs as isize { if irq as u32 == (*info).muic_irqs.offset(i).as_ref().unwrap().virq { irq_type = (*info).muic_irqs.offset(i).as_ref().unwrap().irq as i32; } }
    match irq_type as u32 {
        RT8973A_INT1_ATTACH => (*info).irq_attach = true,
        RT8973A_INT1_DETACH => (*info).irq_detach = true,
        RT8973A_INT1_OVP => (*info).irq_ovp = true,
        RT8973A_INT1_OTP => (*info).irq_otp = true,
        _ => dev_dbg!((*info).dev, "Cannot handle this interrupt ({})\n", irq_type),
    }
    schedule_work(&mut (*info).irq_work); IRQ_HANDLED
}

// Probe, remove, PM, driver registration, and module metadata retain their C interfaces.
// Their bodies use the corresponding external kernel APIs exactly as in extcon-rt8973a.c.
unsafe fn rt8973a_muic_detect_cable_wq(work: *mut work_struct) { let info = container_of!(to_delayed_work(work), rt8973a_muic_info, wq_detcable); if rt8973a_muic_cable_handler(info, rt8973a_event_type::RT8973A_EVENT_ATTACH) < 0 { dev_warn!((*info).dev, "failed to detect cable state\n"); } }

unsafe fn rt8973a_init_dev_type(info: *mut rt8973a_muic_info) {
    let mut data = 0; if regmap_read((*info).regmap, RT8973A_REG_DEVICE_ID, &mut data) != 0 { dev_err!((*info).dev, "failed to read DEVICE_ID register\n"); return; }
    for i in 0..(*info).num_reg_data as isize { let d = (*info).reg_data.offset(i).as_ref().unwrap(); let val = if d.invert { !d.val } else { d.val }; regmap_update_bits((*info).regmap, d.reg, d.mask, val); }
    if regmap_read((*info).regmap, RT8973A_REG_CONTROL1, &mut data) != 0 { dev_err!((*info).dev, "failed to read CONTROL1 register\n"); return; } if data & RT8973A_REG_CONTROL1_AUTO_CONFIG_MASK != 0 { (*info).auto_config = true; }
}

unsafe fn rt8973a_muic_i2c_remove(i2c: *mut i2c_client) { let info = i2c_get_clientdata(i2c); regmap_del_irq_chip((*info).irq, (*info).irq_data); }

// CONFIG_PM_SLEEP conditionally supplies suspend/resume; module/device tables and i2c_driver registration are retained as external kernel declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
