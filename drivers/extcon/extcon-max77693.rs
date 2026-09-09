// SPDX-License-Identifier: GPL-2.0+
// extcon-max77693.c - MAX77693 extcon driver to support MAX77693 MUIC
// Copyright (C) 2012 Samsung Electrnoics
// Chanwoo Choi <cw00.choi@samsung.com>

// Kernel dependencies supplied by the surrounding translation unit.

const DEV_NAME: &str = "max77693-muic";
const DELAY_MS_DEFAULT: u32 = 20000;

static mut DEFAULT_INIT_DATA: [max77693_reg_data; 5] = [
    max77693_reg_data { addr: MAX77693_MUIC_REG_STATUS2, data: MAX77693_STATUS2_CHGDETRUN_MASK },
    max77693_reg_data { addr: MAX77693_MUIC_REG_INTMASK1, data: INTMASK1_ADC1K_MASK | INTMASK1_ADC_MASK },
    max77693_reg_data { addr: MAX77693_MUIC_REG_INTMASK2, data: INTMASK2_CHGTYP_MASK },
    max77693_reg_data { addr: MAX77693_MUIC_REG_INTMASK3, data: 0 },
    max77693_reg_data { addr: MAX77693_MUIC_REG_CDETCTRL2, data: CDETCTRL2_VIDRMEN_MASK | CDETCTRL2_DXOVPEN_MASK },
];

#[repr(C)]
enum max77693_muic_adc_debounce_time { ADC_DEBOUNCE_TIME_5MS = 0, ADC_DEBOUNCE_TIME_10MS, ADC_DEBOUNCE_TIME_25MS, ADC_DEBOUNCE_TIME_38_62MS }

#[repr(C)]
struct max77693_muic_info {
    dev: *mut device, max77693: *mut max77693_dev, edev: *mut extcon_dev,
    prev_cable_type: i32, prev_cable_type_gnd: i32, prev_chg_type: i32, prev_button_type: i32,
    status: [u8; 2], irq: i32, irq_work: work_struct, mutex: mutex, wq_detcable: delayed_work,
    dock: *mut input_dev, path_usb: i32, path_uart: i32,
}

#[repr(C)]
enum max77693_muic_cable_group { MAX77693_CABLE_GROUP_ADC = 0, MAX77693_CABLE_GROUP_ADC_GND, MAX77693_CABLE_GROUP_CHG, MAX77693_CABLE_GROUP_VBVOLT }
#[repr(C)]
enum max77693_muic_charger_type { MAX77693_CHARGER_TYPE_NONE = 0, MAX77693_CHARGER_TYPE_USB, MAX77693_CHARGER_TYPE_DOWNSTREAM_PORT, MAX77693_CHARGER_TYPE_DEDICATED_CHG, MAX77693_CHARGER_TYPE_APPLE_500MA, MAX77693_CHARGER_TYPE_APPLE_1A_2A, MAX77693_CHARGER_TYPE_DEAD_BATTERY = 7 }

#[repr(C)]
struct max77693_muic_irq { irq: u32, name: *const i8, virq: u32 }
static mut MUIC_IRQS: [max77693_muic_irq; 16] = [
    max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT1_ADC,name:b"muic-ADC\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT1_ADC_LOW,name:b"muic-ADCLOW\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT1_ADC_ERR,name:b"muic-ADCError\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT1_ADC1K,name:b"muic-ADC1K\0".as_ptr() as _,virq:0},
    max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_CHGTYP,name:b"muic-CHGTYP\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_CHGDETREUN,name:b"muic-CHGDETREUN\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_DCDTMR,name:b"muic-DCDTMR\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_DXOVP,name:b"muic-DXOVP\0".as_ptr() as _,virq:0},
    max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_VBVOLT,name:b"muic-VBVOLT\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT2_VIDRM,name:b"muic-VIDRM\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_EOC,name:b"muic-EOC\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_CGMBC,name:b"muic-CGMBC\0".as_ptr() as _,virq:0},
    max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_OVP,name:b"muic-OVP\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_MBCCHG_ERR,name:b"muic-MBCCHG_ERR\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_CHG_ENABLED,name:b"muic-CHG_ENABLED\0".as_ptr() as _,virq:0}, max77693_muic_irq{irq:MAX77693_MUIC_IRQ_INT3_BAT_DET,name:b"muic-BAT_DET\0".as_ptr() as _,virq:0},
];

#[repr(C)]
enum max77693_muic_acc_type {
    MAX77693_MUIC_ADC_GROUND=0, MAX77693_MUIC_ADC_SEND_END_BUTTON, MAX77693_MUIC_ADC_REMOTE_S1_BUTTON, MAX77693_MUIC_ADC_REMOTE_S2_BUTTON, MAX77693_MUIC_ADC_REMOTE_S3_BUTTON, MAX77693_MUIC_ADC_REMOTE_S4_BUTTON, MAX77693_MUIC_ADC_REMOTE_S5_BUTTON, MAX77693_MUIC_ADC_REMOTE_S6_BUTTON, MAX77693_MUIC_ADC_REMOTE_S7_BUTTON, MAX77693_MUIC_ADC_REMOTE_S8_BUTTON, MAX77693_MUIC_ADC_REMOTE_S9_BUTTON, MAX77693_MUIC_ADC_REMOTE_S10_BUTTON, MAX77693_MUIC_ADC_REMOTE_S11_BUTTON, MAX77693_MUIC_ADC_REMOTE_S12_BUTTON, MAX77693_MUIC_ADC_RESERVED_ACC_1, MAX77693_MUIC_ADC_RESERVED_ACC_2, MAX77693_MUIC_ADC_RESERVED_ACC_3, MAX77693_MUIC_ADC_RESERVED_ACC_4, MAX77693_MUIC_ADC_RESERVED_ACC_5, MAX77693_MUIC_ADC_CEA936_AUDIO, MAX77693_MUIC_ADC_PHONE_POWERED_DEV, MAX77693_MUIC_ADC_TTY_CONVERTER, MAX77693_MUIC_ADC_UART_CABLE, MAX77693_MUIC_ADC_CEA936A_TYPE1_CHG, MAX77693_MUIC_ADC_FACTORY_MODE_USB_OFF, MAX77693_MUIC_ADC_FACTORY_MODE_USB_ON, MAX77693_MUIC_ADC_AV_CABLE_NOLOAD, MAX77693_MUIC_ADC_CEA936A_TYPE2_CHG, MAX77693_MUIC_ADC_FACTORY_MODE_UART_OFF, MAX77693_MUIC_ADC_FACTORY_MODE_UART_ON, MAX77693_MUIC_ADC_AUDIO_MODE_REMOTE, MAX77693_MUIC_ADC_OPEN,
    MAX77693_MUIC_GND_USB_HOST=0x100, MAX77693_MUIC_GND_USB_HOST_VB=0x104, MAX77693_MUIC_GND_AV_CABLE_LOAD=0x102, MAX77693_MUIC_GND_MHL=0x103, MAX77693_MUIC_GND_MHL_VB=0x107,
}

static MAX77693_EXTCON_CABLE: [u32; 11] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_DCP, EXTCON_CHG_USB_FAST, EXTCON_CHG_USB_SLOW, EXTCON_CHG_USB_CDP, EXTCON_DISP_MHL, EXTCON_JIG, EXTCON_DOCK, EXTCON_NONE];

unsafe fn max77693_muic_set_debounce_time(info: *mut max77693_muic_info, time: max77693_muic_adc_debounce_time) -> i32 {
    match time { ADC_DEBOUNCE_TIME_5MS|ADC_DEBOUNCE_TIME_10MS|ADC_DEBOUNCE_TIME_25MS|ADC_DEBOUNCE_TIME_38_62MS => { let ret=regmap_write((*(*info).max77693).regmap_muic,MAX77693_MUIC_REG_CTRL3,(time as u32)<<MAX77693_CONTROL3_ADCDBSET_SHIFT); if ret != 0 { dev_err((*info).dev,"failed to set ADC debounce time\n"); return ret; } }, _ => { dev_err((*info).dev,"invalid ADC debounce time\n"); return -EINVAL; } } 0
}

unsafe fn max77693_muic_set_path(info:*mut max77693_muic_info,val:u8,attached:bool)->i32 { let ctrl1=if attached {val as u32}else{MAX77693_CONTROL1_SW_OPEN}; let mut ctrl2=0; let mut ret=regmap_update_bits((*(*info).max77693).regmap_muic,MAX77693_MUIC_REG_CTRL1,COMP_SW_MASK,ctrl1); if ret<0{return ret}; ctrl2=if attached{MAX77693_CONTROL2_CPEN_MASK}else{MAX77693_CONTROL2_LOWPWR_MASK}; ret=regmap_update_bits((*(*info).max77693).regmap_muic,MAX77693_MUIC_REG_CTRL2,MAX77693_CONTROL2_LOWPWR_MASK|MAX77693_CONTROL2_CPEN_MASK,ctrl2); if ret<0{return ret}; 0 }

unsafe fn max77693_muic_get_cable_type(info:*mut max77693_muic_info,group:max77693_muic_cable_group,attached:*mut bool)->i32 { let mut cable_type=0; let mut adc; let mut adc1k; let mut adclow; let mut vbvolt; let mut chg_type; match group { max77693_muic_cable_group::MAX77693_CABLE_GROUP_ADC=>{adc=(((*info).status[0] as u32&MAX77693_STATUS1_ADC_MASK)>>MAX77693_STATUS1_ADC_SHIFT) as i32;if adc==MAX77693_MUIC_ADC_OPEN as i32{*attached=false;cable_type=(*info).prev_cable_type;(*info).prev_cable_type=MAX77693_MUIC_ADC_OPEN as i32}else{*attached=true;cable_type=adc;(*info).prev_cable_type=adc}}, max77693_muic_cable_group::MAX77693_CABLE_GROUP_ADC_GND=>{adc=(((*info).status[0] as u32&MAX77693_STATUS1_ADC_MASK)>>MAX77693_STATUS1_ADC_SHIFT) as i32;if adc==MAX77693_MUIC_ADC_OPEN as i32{*attached=false;cable_type=(*info).prev_cable_type_gnd;(*info).prev_cable_type_gnd=MAX77693_MUIC_ADC_OPEN as i32}else{*attached=true;adclow=(((*info).status[0] as u32&MAX77693_STATUS1_ADCLOW_MASK)>>MAX77693_STATUS1_ADCLOW_SHIFT) as i32;adc1k=(((*info).status[0] as u32&MAX77693_STATUS1_ADC1K_MASK)>>MAX77693_STATUS1_ADC1K_SHIFT) as i32;vbvolt=(((*info).status[1] as u32&MAX77693_STATUS2_VBVOLT_MASK)>>MAX77693_STATUS2_VBVOLT_SHIFT) as i32;cable_type=(0x100|(vbvolt<<2)|(adclow<<1)|adc1k);(*info).prev_cable_type=adc;(*info).prev_cable_type_gnd=cable_type}}, max77693_muic_cable_group::MAX77693_CABLE_GROUP_CHG=>{chg_type=(((*info).status[1] as u32&MAX77693_STATUS2_CHGTYP_MASK)>>MAX77693_STATUS2_CHGTYP_SHIFT) as i32;if chg_type==MAX77693_CHARGER_TYPE_NONE as i32{*attached=false;cable_type=(*info).prev_chg_type;(*info).prev_chg_type=0}else{*attached=true;cable_type=chg_type;(*info).prev_chg_type=chg_type}}, max77693_muic_cable_group::MAX77693_CABLE_GROUP_VBVOLT=>{adc=(((*info).status[0] as u32&MAX77693_STATUS1_ADC_MASK)>>MAX77693_STATUS1_ADC_SHIFT) as i32;chg_type=(((*info).status[1] as u32&MAX77693_STATUS2_CHGTYP_MASK)>>MAX77693_STATUS2_CHGTYP_SHIFT) as i32;*attached=!(adc==MAX77693_MUIC_ADC_OPEN as i32&&chg_type==0);vbvolt=(((*info).status[1] as u32&MAX77693_STATUS2_VBVOLT_MASK)>>MAX77693_STATUS2_VBVOLT_SHIFT) as i32;cable_type=vbvolt}}, _=>{*attached=false;cable_type=-EINVAL}} cable_type }

// Remaining driver entry points retain the original kernel implementation contract.
extern "C" { fn max77693_muic_probe(pdev:*mut platform_device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
