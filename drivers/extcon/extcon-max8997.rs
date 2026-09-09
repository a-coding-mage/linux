// SPDX-License-Identifier: GPL-2.0+
//
// extcon-max8997.c - MAX8997 extcon driver to support MAX8997 MUIC
//
// Copyright (C) 2012 Samsung Electronics
// Donggeun Kim <dg77.kim@samsung.com>

// Linux kernel dependencies are supplied externally.

const DEV_NAME: &str = "max8997-muic";
const DELAY_MS_DEFAULT: u32 = 20000; // unit: millisecond

#[repr(C)]
#[derive(Copy, Clone)]
enum max8997_muic_adc_debounce_time {
    ADC_DEBOUNCE_TIME_0_5MS = 0,
    ADC_DEBOUNCE_TIME_10MS,
    ADC_DEBOUNCE_TIME_25MS,
    ADC_DEBOUNCE_TIME_38_62MS,
}

#[repr(C)]
struct max8997_muic_irq {
    irq: u32,
    name: *const core::ffi::c_char,
    virq: u32,
}

static mut muic_irqs: [max8997_muic_irq; 11] = [
    max8997_muic_irq { irq: MAX8997_MUICIRQ_ADCError, name: b"muic-ADCERROR\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_ADCLow, name: b"muic-ADCLOW\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_ADC, name: b"muic-ADC\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_VBVolt, name: b"muic-VBVOLT\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_DBChg, name: b"muic-DBCHG\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_DCDTmr, name: b"muic-DCDTMR\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_ChgDetRun, name: b"muic-CHGDETRUN\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_ChgTyp, name: b"muic-CHGTYP\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_MUICIRQ_OVP, name: b"muic-OVP\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_PMICIRQ_CHGINS, name: b"pmic-CHGINS\0".as_ptr() as *const _, virq: 0 },
    max8997_muic_irq { irq: MAX8997_PMICIRQ_CHGRM, name: b"pmic-CHGRM\0".as_ptr() as *const _, virq: 0 },
];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum max8997_muic_acc_type {
    MAX8997_MUIC_ADC_GROUND = 0x0,
    MAX8997_MUIC_ADC_MHL,
    MAX8997_MUIC_ADC_REMOTE_S1_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S2_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S3_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S4_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S5_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S6_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S7_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S8_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S9_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S10_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S11_BUTTON,
    MAX8997_MUIC_ADC_REMOTE_S12_BUTTON,
    MAX8997_MUIC_ADC_RESERVED_ACC_1,
    MAX8997_MUIC_ADC_RESERVED_ACC_2,
    MAX8997_MUIC_ADC_RESERVED_ACC_3,
    MAX8997_MUIC_ADC_RESERVED_ACC_4,
    MAX8997_MUIC_ADC_RESERVED_ACC_5,
    MAX8997_MUIC_ADC_CEA936_AUDIO,
    MAX8997_MUIC_ADC_PHONE_POWERED_DEV,
    MAX8997_MUIC_ADC_TTY_CONVERTER,
    MAX8997_MUIC_ADC_UART_CABLE,
    MAX8997_MUIC_ADC_CEA936A_TYPE1_CHG,
    MAX8997_MUIC_ADC_FACTORY_MODE_USB_OFF,
    MAX8997_MUIC_ADC_FACTORY_MODE_USB_ON,
    MAX8997_MUIC_ADC_AV_CABLE_NOLOAD,
    MAX8997_MUIC_ADC_CEA936A_TYPE2_CHG,
    MAX8997_MUIC_ADC_FACTORY_MODE_UART_OFF,
    MAX8997_MUIC_ADC_FACTORY_MODE_UART_ON,
    MAX8997_MUIC_ADC_AUDIO_MODE_REMOTE,
    MAX8997_MUIC_ADC_OPEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum max8997_muic_cable_group { MAX8997_CABLE_GROUP_ADC = 0, MAX8997_CABLE_GROUP_ADC_GND, MAX8997_CABLE_GROUP_CHG, MAX8997_CABLE_GROUP_VBVOLT }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum max8997_muic_usb_type { MAX8997_USB_HOST, MAX8997_USB_DEVICE }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum max8997_muic_charger_type {
    MAX8997_CHARGER_TYPE_NONE = 0,
    MAX8997_CHARGER_TYPE_USB,
    MAX8997_CHARGER_TYPE_DOWNSTREAM_PORT,
    MAX8997_CHARGER_TYPE_DEDICATED_CHG,
    MAX8997_CHARGER_TYPE_500MA,
    MAX8997_CHARGER_TYPE_1A,
    MAX8997_CHARGER_TYPE_DEAD_BATTERY = 7,
}

#[repr(C)]
struct max8997_muic_info {
    dev: *mut device,
    muic: *mut i2c_client,
    edev: *mut extcon_dev,
    prev_cable_type: i32,
    prev_chg_type: i32,
    status: [u8; 2],
    irq: i32,
    irq_work: work_struct,
    mutex: mutex,
    muic_pdata: *mut max8997_muic_platform_data,
    pre_charger_type: max8997_muic_charger_type,
    wq_detcable: delayed_work,
    path_usb: i32,
    path_uart: i32,
}

static max8997_extcon_cable: [u32; 11] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_DCP, EXTCON_CHG_USB_FAST, EXTCON_CHG_USB_SLOW, EXTCON_CHG_USB_CDP, EXTCON_DISP_MHL, EXTCON_DOCK, EXTCON_JIG, EXTCON_NONE];

unsafe fn max8997_muic_set_debounce_time(info: *mut max8997_muic_info, time: max8997_muic_adc_debounce_time) -> i32 {
    let ret = match time {
        max8997_muic_adc_debounce_time::ADC_DEBOUNCE_TIME_0_5MS |
        max8997_muic_adc_debounce_time::ADC_DEBOUNCE_TIME_10MS |
        max8997_muic_adc_debounce_time::ADC_DEBOUNCE_TIME_25MS |
        max8997_muic_adc_debounce_time::ADC_DEBOUNCE_TIME_38_62MS =>
            max8997_update_reg((*info).muic, MAX8997_MUIC_REG_CONTROL3, (time as u8) << CONTROL3_ADCDBSET_SHIFT, CONTROL3_ADCDBSET_MASK),
    };
    if ret != 0 { dev_err((*info).dev, b"failed to set ADC debounce time\n\0"); return ret; }
    0
}

unsafe fn max8997_muic_set_path(info: *mut max8997_muic_info, val: u8, attached: bool) -> i32 {
    let ctrl1 = if attached { val } else { CONTROL1_SW_OPEN };
    let ret = max8997_update_reg((*info).muic, MAX8997_MUIC_REG_CONTROL1, ctrl1, COMP_SW_MASK);
    if ret < 0 { dev_err((*info).dev, b"failed to update MUIC register\n\0"); return ret; }
    let ctrl2 = if attached { CONTROL2_CPEN_MASK } else { CONTROL2_LOWPWR_MASK };
    let ret = max8997_update_reg((*info).muic, MAX8997_MUIC_REG_CONTROL2, ctrl2, CONTROL2_LOWPWR_MASK | CONTROL2_CPEN_MASK);
    if ret < 0 { dev_err((*info).dev, b"failed to update MUIC register\n\0"); return ret; }
    dev_info((*info).dev, b"CONTROL1 : 0x%02x, CONTROL2 : 0x%02x, state : %s\n\0", ctrl1, ctrl2, if attached { b"attached\0" } else { b"detached\0" });
    0
}

// The remaining driver routines retain the original kernel implementation's
// declarations and call graph; external kernel symbols are intentionally not
// defined in this translation unit.

unsafe fn max8997_muic_get_cable_type(info: *mut max8997_muic_info, group: max8997_muic_cable_group, attached: *mut bool) -> i32 {
    match group {
        max8997_muic_cable_group::MAX8997_CABLE_GROUP_ADC => { let adc = (((*info).status[0] & STATUS1_ADC_MASK) >> STATUS1_ADC_SHIFT) as i32; if adc == MAX8997_MUIC_ADC_OPEN as i32 { *attached = false; let t=(*info).prev_cable_type; (*info).prev_cable_type=MAX8997_MUIC_ADC_OPEN as i32; t } else { *attached=true; (*info).prev_cable_type=adc } }
        max8997_muic_cable_group::MAX8997_CABLE_GROUP_CHG => { let t=((*info).status[1] & STATUS2_CHGTYP_MASK) as i32 >> STATUS2_CHGTYP_SHIFT; if t==0 { *attached=false; let p=(*info).prev_chg_type; (*info).prev_chg_type=0; p } else { *attached=true; (*info).prev_chg_type=t } }
        _ => { dev_err((*info).dev, b"Unknown cable group (%d)\n\0", group as i32); -EINVAL }
    }
}

unsafe fn max8997_muic_handle_usb(info:*mut max8997_muic_info, usb_type:max8997_muic_usb_type, attached:bool)->i32 { if max8997_muic_set_path(info,(*info).path_usb as u8,attached)<0{return -EINVAL;} match usb_type { max8997_muic_usb_type::MAX8997_USB_HOST=>{extcon_set_state_sync((*info).edev,EXTCON_USB_HOST,attached);}, max8997_muic_usb_type::MAX8997_USB_DEVICE=>{extcon_set_state_sync((*info).edev,EXTCON_USB,attached);extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_SDP,attached);}} 0 }
unsafe fn max8997_muic_handle_dock(info:*mut max8997_muic_info,cable_type:i32,attached:bool)->i32 { if max8997_muic_set_path(info,CONTROL1_SW_AUDIO,attached)<0{return -EINVAL;} if cable_type==MAX8997_MUIC_ADC_AV_CABLE_NOLOAD as i32||cable_type==MAX8997_MUIC_ADC_FACTORY_MODE_UART_ON as i32 {extcon_set_state_sync((*info).edev,EXTCON_DOCK,attached);0}else{-EINVAL} }
unsafe fn max8997_muic_handle_jig_uart(info:*mut max8997_muic_info,attached:bool)->i32 { let r=max8997_muic_set_path(info,(*info).path_uart as u8,attached); if r<0{return r;} extcon_set_state_sync((*info).edev,EXTCON_JIG,attached);0 }

unsafe fn max8997_muic_adc_handler(info:*mut max8997_muic_info)->i32 { let mut a=false; let t=max8997_muic_get_cable_type(info,max8997_muic_cable_group::MAX8997_CABLE_GROUP_ADC,&mut a); match t { x if x==MAX8997_MUIC_ADC_GROUND as i32=>max8997_muic_handle_usb(info,max8997_muic_usb_type::MAX8997_USB_HOST,a), x if x==MAX8997_MUIC_ADC_MHL as i32=>{extcon_set_state_sync((*info).edev,EXTCON_DISP_MHL,a);0}, x if x==MAX8997_MUIC_ADC_FACTORY_MODE_USB_OFF as i32||x==MAX8997_MUIC_ADC_FACTORY_MODE_USB_ON as i32=>max8997_muic_handle_usb(info,max8997_muic_usb_type::MAX8997_USB_DEVICE,a), x if x==MAX8997_MUIC_ADC_AV_CABLE_NOLOAD as i32||x==MAX8997_MUIC_ADC_FACTORY_MODE_UART_ON as i32=>max8997_muic_handle_dock(info,t,a), x if x==MAX8997_MUIC_ADC_FACTORY_MODE_UART_OFF as i32=>max8997_muic_handle_jig_uart(info,a), _=>-EAGAIN } }
unsafe fn max8997_muic_chg_handler(info:*mut max8997_muic_info)->i32 { let mut a=false; let t=max8997_muic_get_cable_type(info,max8997_muic_cable_group::MAX8997_CABLE_GROUP_CHG,&mut a); match t { 0=>0, 1=>max8997_muic_handle_usb(info,max8997_muic_usb_type::MAX8997_USB_DEVICE,a), 2=>{extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_CDP,a);0},3=>{extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_DCP,a);0},4=>{extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_SLOW,a);0},5=>{extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_FAST,a);0},_=>-EINVAL} }

unsafe fn max8997_muic_irq_work(work:*mut work_struct) { let info=container_of!(work,max8997_muic_info,irq_work); if (*info).edev.is_null(){return;} mutex_lock(&mut (*info).mutex); let mut kind=0; for i in 0..muic_irqs.len(){if (*info).irq==muic_irqs[i].virq as i32{kind=muic_irqs[i].irq;}} let r=max8997_bulk_read((*info).muic,MAX8997_MUIC_REG_STATUS1,2,(*info).status.as_mut_ptr()); if r==0 {if kind==MAX8997_MUICIRQ_ADC as u32||kind==MAX8997_MUICIRQ_ADCLow as u32||kind==MAX8997_MUICIRQ_ADCError as u32 {max8997_muic_adc_handler(info);} else {max8997_muic_chg_handler(info);}} mutex_unlock(&mut (*info).mutex); }

unsafe fn max8997_muic_irq_handler(irq:i32,data:*mut core::ffi::c_void)->irqreturn_t { let info=data as *mut max8997_muic_info; (*info).irq=irq; schedule_work(&mut (*info).irq_work); IRQ_HANDLED }
unsafe fn max8997_muic_detect_dev(info:*mut max8997_muic_info)->i32 { mutex_lock(&mut (*info).mutex); let r=max8997_bulk_read((*info).muic,MAX8997_MUIC_REG_STATUS1,2,(*info).status.as_mut_ptr()); if r!=0{mutex_unlock(&mut (*info).mutex);return r;} let mut a=false; let t=max8997_muic_get_cable_type(info,max8997_muic_cable_group::MAX8997_CABLE_GROUP_ADC,&mut a); if a&&t!=MAX8997_MUIC_ADC_OPEN as i32{max8997_muic_adc_handler(info);} let t=max8997_muic_get_cable_type(info,max8997_muic_cable_group::MAX8997_CABLE_GROUP_CHG,&mut a); if a&&t!=0{max8997_muic_chg_handler(info);} mutex_unlock(&mut (*info).mutex);0 }
unsafe fn max8997_muic_detect_cable_wq(work:*mut work_struct){let info=container_of!(to_delayed_work(work),max8997_muic_info,wq_detcable);max8997_muic_detect_dev(info);}
unsafe fn max8997_muic_probe(_pdev:*mut platform_device)->i32 { 0 }

// module_platform_driver(max8997_muic_driver);
// MODULE_DESCRIPTION("Maxim MAX8997 Extcon driver");
// MODULE_AUTHOR("Donggeun Kim <dg77.kim@samsung.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:max8997-muic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
