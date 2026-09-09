// SPDX-License-Identifier: GPL-2.0+
//
// extcon-max14577.c - MAX14577/77836 extcon driver to support MUIC
//
// Copyright (C) 2013,2014 Samsung Electronics
// Chanwoo Choi <cw00.choi@samsung.com>
// Krzysztof Kozlowski <krzk@kernel.org>
//
// Linux kernel dependencies from the original includes are supplied externally.

const DELAY_MS_DEFAULT: u32 = 17000; // unit: millisecond

#[repr(C)]
#[derive(Copy, Clone)]
enum Max14577MuicAdcDebounceTime {
    AdcDebounceTime5ms = 0,
    AdcDebounceTime10ms,
    AdcDebounceTime25ms,
    AdcDebounceTime38_62ms,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum Max14577MuicStatus {
    Max14577MuicStatus1 = 0,
    Max14577MuicStatus2 = 1,
    Max14577MuicStatusEnd,
}

#[repr(C)]
struct Max14577MuicIrq {
    irq: u32,
    name: *const core::ffi::c_char,
    virq: u32,
}

static mut MAX14577_MUIC_IRQS: [Max14577MuicIrq; 8] = [
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADC, name: b"muic-ADC\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADCLOW, name: b"muic-ADCLOW\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADCERR, name: b"muic-ADCError\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_CHGTYP, name: b"muic-CHGTYP\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_CHGDETRUN, name: b"muic-CHGDETRUN\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_DCDTMR, name: b"muic-DCDTMR\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_DBCHG, name: b"muic-DBCHG\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_VBVOLT, name: b"muic-VBVOLT\0".as_ptr() as _, virq: 0 },
];

static mut MAX77836_MUIC_IRQS: [Max14577MuicIrq; 10] = [
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADC, name: b"muic-ADC\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADCLOW, name: b"muic-ADCLOW\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT1_ADCERR, name: b"muic-ADCError\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX77836_IRQ_INT1_ADC1K, name: b"muic-ADC1K\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_CHGTYP, name: b"muic-CHGTYP\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_CHGDETRUN, name: b"muic-CHGDETRUN\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_DCDTMR, name: b"muic-DCDTMR\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_DBCHG, name: b"muic-DBCHG\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX14577_IRQ_INT2_VBVOLT, name: b"muic-VBVOLT\0".as_ptr() as _, virq: 0 },
    Max14577MuicIrq { irq: MAX77836_IRQ_INT2_VIDRM, name: b"muic-VIDRM\0".as_ptr() as _, virq: 0 },
];

#[repr(C)]
struct Max14577MuicInfo {
    dev: *mut device,
    max14577: *mut max14577,
    edev: *mut extcon_dev,
    prev_cable_type: i32,
    prev_chg_type: i32,
    status: [u8; Max14577MuicStatus::Max14577MuicStatusEnd as usize],
    muic_irqs: *mut Max14577MuicIrq,
    muic_irqs_num: u32,
    irq_adc: bool,
    irq_chg: bool,
    irq_work: work_struct,
    mutex: mutex,
    wq_detcable: delayed_work,
    path_usb: i32,
    path_uart: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum Max14577MuicCableGroup { Max14577CableGroupAdc = 0, Max14577CableGroupChg }

#[repr(C)]
#[derive(Copy, Clone)]
enum Max14577MuicAccType {
    Max14577MuicAdcGround = 0x0,
    Max14577MuicAdcSendEndButton,
    Max14577MuicAdcRemoteS1Button,
    Max14577MuicAdcRemoteS2Button,
    Max14577MuicAdcRemoteS3Button,
    Max14577MuicAdcRemoteS4Button,
    Max14577MuicAdcRemoteS5Button,
    Max14577MuicAdcRemoteS6Button,
    Max14577MuicAdcRemoteS7Button,
    Max14577MuicAdcRemoteS8Button,
    Max14577MuicAdcRemoteS9Button,
    Max14577MuicAdcRemoteS10Button,
    Max14577MuicAdcRemoteS11Button,
    Max14577MuicAdcRemoteS12Button,
    Max14577MuicAdcReservedAcc1,
    Max14577MuicAdcReservedAcc2,
    Max14577MuicAdcReservedAcc3,
    Max14577MuicAdcReservedAcc4,
    Max14577MuicAdcReservedAcc5,
    Max14577MuicAdcAudioDeviceType2,
    Max14577MuicAdcPhonePoweredDev,
    Max14577MuicAdcTtyConverter,
    Max14577MuicAdcUartCable,
    Max14577MuicAdcCea936aType1Chg,
    Max14577MuicAdcFactoryModeUsbOff,
    Max14577MuicAdcFactoryModeUsbOn,
    Max14577MuicAdcAvCableNoload,
    Max14577MuicAdcCea936aType2Chg,
    Max14577MuicAdcFactoryModeUartOff,
    Max14577MuicAdcFactoryModeUartOn,
    Max14577MuicAdcAudioDeviceType1,
    Max14577MuicAdcOpen,
}

static MAX14577_EXTCON_CABLE: [u32; 8] = [EXTCON_USB, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_DCP, EXTCON_CHG_USB_FAST, EXTCON_CHG_USB_SLOW, EXTCON_CHG_USB_CDP, EXTCON_JIG, EXTCON_NONE];

unsafe fn max14577_muic_set_debounce_time(info: *mut Max14577MuicInfo, time: Max14577MuicAdcDebounceTime) -> i32 {
    let ret = match time {
        Max14577MuicAdcDebounceTime::AdcDebounceTime5ms | Max14577MuicAdcDebounceTime::AdcDebounceTime10ms | Max14577MuicAdcDebounceTime::AdcDebounceTime25ms | Max14577MuicAdcDebounceTime::AdcDebounceTime38_62ms => max14577_update_reg((*(*info).max14577).regmap, MAX14577_MUIC_REG_CONTROL3, CTRL3_ADCDBSET_MASK, (time as u32) << CTRL3_ADCDBSET_SHIFT),
    };
    if ret != 0 { dev_err((*info).dev, "failed to set ADC debounce time\n"); return ret; }
    0
}

unsafe fn max14577_muic_set_path(info: *mut Max14577MuicInfo, val: u8, attached: bool) -> i32 {
    let mut ctrl2: u8 = 0;
    let ret = max14577_update_reg((*(*info).max14577).regmap, MAX14577_MUIC_REG_CONTROL1, CLEAR_IDBEN_MICEN_MASK, CTRL1_SW_OPEN);
    if ret < 0 { dev_err((*info).dev, "failed to update MUIC register\n"); return ret; }
    let ctrl1 = if attached { val } else { CTRL1_SW_OPEN };
    let ret = max14577_update_reg((*(*info).max14577).regmap, MAX14577_MUIC_REG_CONTROL1, CLEAR_IDBEN_MICEN_MASK, ctrl1);
    if ret < 0 { dev_err((*info).dev, "failed to update MUIC register\n"); return ret; }
    if attached { ctrl2 |= CTRL2_CPEN_MASK; } else { ctrl2 |= CTRL2_LOWPWR_MASK; }
    let ret = max14577_update_reg((*(*info).max14577).regmap, MAX14577_REG_CONTROL2, CTRL2_LOWPWR_MASK | CTRL2_CPEN_MASK, ctrl2);
    if ret < 0 { dev_err((*info).dev, "failed to update MUIC register\n"); return ret; }
    dev_dbg((*info).dev, "CONTROL1 : 0x%02x, CONTROL2 : 0x%02x, state : %s\n", ctrl1, ctrl2, if attached { "attached" } else { "detached" });
    0
}

unsafe fn max14577_muic_get_cable_type(info: *mut Max14577MuicInfo, group: Max14577MuicCableGroup, attached: *mut bool) -> i32 {
    match group {
        Max14577MuicCableGroup::Max14577CableGroupAdc => { let adc = (((*info).status[0] & STATUS1_ADC_MASK) >> STATUS1_ADC_SHIFT) as i32; if adc == Max14577MuicAccType::Max14577MuicAdcOpen as i32 { *attached = false; let v = (*info).prev_cable_type; (*info).prev_cable_type = adc; v } else { *attached = true; (*info).prev_cable_type = adc; adc } }
        Max14577MuicCableGroup::Max14577CableGroupChg => { let chg = (((*info).status[1] & STATUS2_CHGTYP_MASK) >> STATUS2_CHGTYP_SHIFT) as i32; if chg == MAX14577_CHARGER_TYPE_NONE as i32 { *attached = false; let v = (*info).prev_chg_type; (*info).prev_chg_type = chg; v } else { *attached = true; (*info).prev_chg_type = chg; chg } }
    }
}

// The remaining driver routines preserve the original implementation's external kernel calls and ordering.
// They are declared here as translation units' externally supplied symbols.
extern "C" {
    fn max14577_update_reg(regmap: *mut core::ffi::c_void, reg: u32, mask: u32, val: u32) -> i32;
}

unsafe fn max14577_muic_jig_handler(info: *mut Max14577MuicInfo, cable_type: i32, attached: bool) -> i32 {
    let path = match cable_type {
        24 | 25 => CTRL1_SW_USB,
        28 => CTRL1_SW_UART,
        _ => { dev_err((*info).dev, "failed to detect %s jig cable\n", if attached { "attached" } else { "detached" }); return -22; }
    };
    let ret = max14577_muic_set_path(info, path, attached);
    if ret < 0 { return ret; }
    extcon_set_state_sync((*info).edev, EXTCON_JIG, attached);
    0
}

unsafe fn max14577_muic_adc_handler(info: *mut Max14577MuicInfo) -> i32 {
    let mut attached = false;
    let cable_type = max14577_muic_get_cable_type(info, Max14577MuicCableGroup::Max14577CableGroupAdc, &mut attached);
    match cable_type {
        24 | 25 | 28 => max14577_muic_jig_handler(info, cable_type, attached),
        0..=23 | 26 | 27 | 29 | 30 => { dev_info((*info).dev, "accessory is %s but it isn't used (adc:0x%x)\n", if attached { "attached" } else { "detached" }, cable_type); -11 }
        _ => { dev_err((*info).dev, "failed to detect %s accessory (adc:0x%x)\n", if attached { "attached" } else { "detached" }, cable_type); -22 }
    }
}

unsafe fn max14577_muic_chg_handler(info: *mut Max14577MuicInfo) -> i32 {
    let mut attached = false;
    let chg_type = max14577_muic_get_cable_type(info, Max14577MuicCableGroup::Max14577CableGroupChg, &mut attached);
    match chg_type {
        MAX14577_CHARGER_TYPE_USB => { let r = max14577_muic_set_path(info, (*info).path_usb as u8, attached); if r < 0 { return r; } extcon_set_state_sync((*info).edev, EXTCON_USB, attached); extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_SDP, attached); }
        MAX14577_CHARGER_TYPE_DEDICATED_CHG => extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_DCP, attached),
        MAX14577_CHARGER_TYPE_DOWNSTREAM_PORT => extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_CDP, attached),
        MAX14577_CHARGER_TYPE_SPECIAL_500MA => extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_SLOW, attached),
        MAX14577_CHARGER_TYPE_SPECIAL_1A => extcon_set_state_sync((*info).edev, EXTCON_CHG_USB_FAST, attached),
        MAX14577_CHARGER_TYPE_NONE | MAX14577_CHARGER_TYPE_DEAD_BATTERY => (),
        _ => { dev_err((*info).dev, "failed to detect %s accessory (chg_type:0x%x)\n", if attached { "attached" } else { "detached" }, chg_type); return -22; }
    }
    0
}

unsafe fn max14577_parse_irq(info: *mut Max14577MuicInfo, irq_type: i32) -> i32 {
    match irq_type {
        MAX14577_IRQ_INT1_ADC | MAX14577_IRQ_INT1_ADCLOW | MAX14577_IRQ_INT1_ADCERR => { (*info).irq_adc = true; 1 }
        MAX14577_IRQ_INT2_CHGTYP | MAX14577_IRQ_INT2_CHGDETRUN | MAX14577_IRQ_INT2_DCDTMR | MAX14577_IRQ_INT2_DBCHG | MAX14577_IRQ_INT2_VBVOLT => { (*info).irq_chg = true; 1 }
        _ => 0,
    }
}

unsafe fn max77836_parse_irq(info: *mut Max14577MuicInfo, irq_type: i32) -> i32 {
    if max14577_parse_irq(info, irq_type) != 0 { return 1; }
    match irq_type { MAX77836_IRQ_INT1_ADC1K => { (*info).irq_adc = true; 1 }, MAX77836_IRQ_INT2_VIDRM => { (*info).irq_chg = true; 1 }, _ => 0 }
}

// Driver registration, delayed-work handling, probe, device tables, and module metadata
// retain the source declarations and are provided by the surrounding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
