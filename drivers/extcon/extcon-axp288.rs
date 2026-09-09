// SPDX-License-Identifier: GPL-2.0-only
/*
 * extcon-axp288.c - X-Power AXP288 PMIC extcon cable detection driver
 *
 * Copyright (c) 2017-2018 Hans de Goede <hdegoede@redhat.com>
 * Copyright (C) 2015 Intel Corporation
 * Author: Ramakrishna Pallala <ramakrishna.pallala@intel.com>
 */

/* Translated from the Linux kernel implementation; kernel dependencies are external. */

const PS_STAT_VBUS_TRIGGER: u32 = 1 << 0;
const PS_STAT_BAT_CHRG_DIR: u32 = 1 << 2;
const PS_STAT_VBUS_ABOVE_VHOLD: u32 = 1 << 3;
const PS_STAT_VBUS_VALID: u32 = 1 << 4;
const PS_STAT_VBUS_PRESENT: u32 = 1 << 5;
const BC_GLOBAL_RUN: u32 = 1 << 0;
const BC_GLOBAL_DET_STAT: u32 = 1 << 2;
const BC_GLOBAL_DBP_TOUT: u32 = 1 << 3;
const BC_GLOBAL_VLGC_COM_SEL: u32 = 1 << 4;
const BC_GLOBAL_DCD_TOUT_MASK: u32 = (1 << 6) | (1 << 5);
const BC_GLOBAL_DCD_TOUT_300MS: u32 = 0;
const BC_GLOBAL_DCD_TOUT_100MS: u32 = 1;
const BC_GLOBAL_DCD_TOUT_500MS: u32 = 2;
const BC_GLOBAL_DCD_TOUT_900MS: u32 = 3;
const BC_GLOBAL_DCD_DET_SEL: u32 = 1 << 7;
const VBUS_CNTL_DPDM_PD_EN: u32 = 1 << 4;
const VBUS_CNTL_DPDM_FD_EN: u32 = 1 << 5;
const VBUS_CNTL_FIRST_PO_STAT: u32 = 1 << 6;
const USB_STAT_BUS_STAT_MASK: u32 = (1 << 3) | (1 << 2) | (1 << 1) | (1 << 0);
const USB_STAT_BUS_STAT_SHIFT: u32 = 0;
const USB_STAT_BUS_STAT_ATHD: u32 = 0;
const USB_STAT_BUS_STAT_CONN: u32 = 1;
const USB_STAT_BUS_STAT_SUSP: u32 = 2;
const USB_STAT_BUS_STAT_CONF: u32 = 3;
const USB_STAT_USB_SS_MODE: u32 = 1 << 4;
const USB_STAT_DEAD_BAT_DET: u32 = 1 << 6;
const USB_STAT_DBP_UNCFG: u32 = 1 << 7;
const DET_STAT_MASK: u32 = (1 << 7) | (1 << 6) | (1 << 5);
const DET_STAT_SHIFT: u32 = 5;
const DET_STAT_SDP: u8 = 1;
const DET_STAT_CDP: u8 = 2;
const DET_STAT_DCP: u8 = 3;

#[repr(u32)]
enum Axp288ExtconReg { PsStat = 0x00, PsBootReason = 0x02, BcGlobal = 0x2c, BcVbusCntl = 0x2d, BcUsbStat = 0x2e, BcDetStat = 0x2f }
#[repr(usize)]
enum Axp288ExtconIrq { VbusFalling = 0, VbusRising, MvChng, BcUsbChng, ExtconIrqEnd }

extern "C" {
    static axp288_extcon_cables: [u32; 5];
    static system_long_wq: *mut WorkqueueStruct;
}

#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct Regmap { _private: [u8; 0] }
#[repr(C)] struct RegmapIrqChipData { _private: [u8; 0] }
#[repr(C)] struct UsbRoleSwitch { _private: [u8; 0] }
#[repr(C)] struct WorkqueueStruct { _private: [u8; 0] }
#[repr(C)] struct WorkStruct { _private: [u8; 0] }
#[repr(C)] struct ExtconDev { _private: [u8; 0] }
#[repr(C)] struct NotifierBlock { notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, u64, *mut core::ffi::c_void) -> i32> }

#[repr(C)] struct Axp288ExtconInfo {
    dev: *mut Device, regmap: *mut Regmap, regmap_irqc: *mut RegmapIrqChipData,
    role_sw: *mut UsbRoleSwitch, role_work: WorkStruct, irq: [i32; 4],
    edev: *mut ExtconDev, id_extcon: *mut ExtconDev, id_nb: NotifierBlock,
    previous_cable: u32, vbus_attach: bool,
}

static AXP288_PWR_UP_DOWN_INFO: [&str; 8] = [
    "Last wake caused by user pressing the power button", "Last wake caused by a charger insertion",
    "Last wake caused by a battery insertion", "Last wake caused by SOC initiated global reset",
    "Last wake caused by cold reset", "Last shutdown caused by PMIC UVLO threshold",
    "Last shutdown caused by SOC initiated cold off", "Last shutdown caused by user pressing the power button",
];

unsafe fn axp288_extcon_log_rsi(info: *mut Axp288ExtconInfo) {
    let mut val: u32 = 0;
    let ret = regmap_read((*info).regmap, Axp288ExtconReg::PsBootReason as u32, &mut val);
    if ret < 0 { dev_err((*info).dev, "failed to read reset source indicator\n"); return; }
    let bits = val & ((1u32 << AXP288_PWR_UP_DOWN_INFO.len()) - 1);
    for i in 0..AXP288_PWR_UP_DOWN_INFO.len() { if bits & (1 << i) != 0 { dev_dbg((*info).dev, AXP288_PWR_UP_DOWN_INFO[i]); } }
    regmap_write((*info).regmap, Axp288ExtconReg::PsBootReason as u32, bits);
}

unsafe fn axp288_get_id_pin(info: *mut Axp288ExtconInfo) -> bool {
    if !(*info).id_extcon.is_null() { return extcon_get_state((*info).id_extcon, EXTCON_USB_HOST) <= 0; }
    usb_role_switch_get_role((*info).role_sw) != USB_ROLE_HOST
}

unsafe extern "C" fn axp288_usb_role_work(work: *mut WorkStruct) {
    let info = container_of_work(work);
    let id_pin = axp288_get_id_pin(info);
    let role = if !id_pin { USB_ROLE_HOST } else if (*info).vbus_attach { USB_ROLE_DEVICE } else { USB_ROLE_NONE };
    let ret = usb_role_switch_set_role((*info).role_sw, role);
    if ret != 0 { dev_err((*info).dev, "failed to set role: %d\n", ret); }
}

unsafe fn axp288_get_vbus_attach(info: *mut Axp288ExtconInfo) -> bool {
    let mut pwr_stat = 0; if regmap_read((*info).regmap, Axp288ExtconReg::PsStat as u32, &mut pwr_stat) < 0 { dev_err((*info).dev, "failed to read vbus status\n"); return false; } pwr_stat & (PS_STAT_VBUS_VALID as i32) != 0
}

unsafe fn axp288_handle_chrg_det_event(info: *mut Axp288ExtconInfo) -> i32 {
    let mut ret = iosf_mbi_block_punit_i2c_access(); if ret < 0 { return ret; }
    let vbus_attach = axp288_get_vbus_attach(info); let mut cable = (*info).previous_cable;
    if vbus_attach {
        let mut cfg = 0; ret = regmap_read((*info).regmap, Axp288ExtconReg::BcGlobal as u32, &mut cfg);
        if ret >= 0 && cfg & BC_GLOBAL_DET_STAT as i32 == 0 {
            let mut stat = 0; ret = regmap_read((*info).regmap, Axp288ExtconReg::BcDetStat as u32, &mut stat);
            if ret >= 0 { match ((stat as u32 & DET_STAT_MASK) >> DET_STAT_SHIFT) as u8 { DET_STAT_SDP => cable = EXTCON_CHG_USB_SDP, DET_STAT_CDP => cable = EXTCON_CHG_USB_CDP, DET_STAT_DCP => cable = EXTCON_CHG_USB_DCP, _ => { dev_warn((*info).dev, "unknown (reserved) bc detect result\n"); cable = EXTCON_CHG_USB_SDP; } } }
        }
    }
    iosf_mbi_unblock_punit_i2c_access();
    extcon_set_state_sync((*info).edev, (*info).previous_cable, false);
    if (*info).previous_cable == EXTCON_CHG_USB_SDP { extcon_set_state_sync((*info).edev, EXTCON_USB, false); }
    if vbus_attach { extcon_set_state_sync((*info).edev, cable, true); if cable == EXTCON_CHG_USB_SDP { extcon_set_state_sync((*info).edev, EXTCON_USB, true); } (*info).previous_cable = cable; }
    if !(*info).role_sw.is_null() && (*info).vbus_attach != vbus_attach { (*info).vbus_attach = vbus_attach; queue_work(system_long_wq, &mut (*info).role_work); }
    if ret < 0 { dev_err((*info).dev, "failed to detect BC Mod\n"); } ret
}

unsafe extern "C" fn axp288_extcon_id_evt(nb: *mut NotifierBlock, _event: u64, _param: *mut core::ffi::c_void) -> i32 { let info = container_of_nb(nb); queue_work(system_long_wq, &mut (*info).role_work); NOTIFY_OK }
unsafe extern "C" fn axp288_extcon_isr(_irq: i32, data: *mut core::ffi::c_void) -> i32 { if axp288_handle_chrg_det_event(data as *mut Axp288ExtconInfo) < 0 { dev_err((*(data as *mut Axp288ExtconInfo)).dev, "failed to handle the interrupt\n"); } IRQ_HANDLED }

// Remaining platform-driver registration and lifecycle declarations are external-kernel bindings.
unsafe extern "C" {
    fn regmap_read(*mut Regmap, u32, *mut i32) -> i32; fn regmap_write(*mut Regmap, u32, u32) -> i32;
    fn iosf_mbi_block_punit_i2c_access() -> i32; fn iosf_mbi_unblock_punit_i2c_access();
    fn extcon_get_state(*mut ExtconDev, u32) -> i32; fn extcon_set_state_sync(*mut ExtconDev, u32, bool) -> i32;
    fn usb_role_switch_get_role(*mut UsbRoleSwitch) -> i32; fn usb_role_switch_set_role(*mut UsbRoleSwitch, i32) -> i32;
    fn queue_work(*mut WorkqueueStruct, *mut WorkStruct) -> bool;
    fn dev_err(*mut Device, *const str, ...); fn dev_dbg(*mut Device, *const str, ...); fn dev_warn(*mut Device, *const str, ...);
    fn container_of_work(*mut WorkStruct) -> *mut Axp288ExtconInfo; fn container_of_nb(*mut NotifierBlock) -> *mut Axp288ExtconInfo;
}

/* The probe, suspend/resume, PM-ops, platform-device-id table, platform-driver
 * registration, and module metadata below retain their C-facing declarations;
 * their concrete implementations are supplied by the surrounding kernel glue. */
#[repr(C)] struct PlatformDevice { _private: [u8; 0] }
#[repr(C)] struct PlatformDriver { _private: [u8; 0] }
unsafe extern "C" {
    fn axp288_extcon_probe(pdev: *mut PlatformDevice) -> i32;
    fn axp288_extcon_suspend(dev: *mut Device) -> i32;
    fn axp288_extcon_resume(dev: *mut Device) -> i32;
    static mut axp288_extcon_driver: PlatformDriver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
