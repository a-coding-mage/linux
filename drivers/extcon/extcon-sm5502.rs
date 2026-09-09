// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of extcon-sm5502.c. Kernel symbols and constants are supplied externally. */

const DELAY_MS_DEFAULT: u32 = 17000;

#[repr(C)]
pub struct muic_irq { pub irq: u32, pub name: *const core::ffi::c_char, pub virq: u32 }
#[repr(C)]
pub struct reg_data { pub reg: u8, pub val: u32, pub invert: bool }
#[repr(C)]
pub struct sm5502_muic_info {
    pub dev: *mut device, pub edev: *mut extcon_dev, pub i2c: *mut i2c_client,
    pub regmap: *mut regmap, pub r#type: *const sm5502_type, pub irq_data: *mut regmap_irq_chip_data,
    pub irq: i32, pub irq_attach: bool, pub irq_detach: bool, pub irq_work: work_struct,
    pub mutex: mutex, pub wq_detcable: delayed_work,
}
#[repr(C)]
pub struct sm5502_type {
    pub muic_irqs: *mut muic_irq, pub num_muic_irqs: u32,
    pub irq_chip: *const regmap_irq_chip, pub reg_data: *mut reg_data,
    pub num_reg_data: u32, pub otg_dev_type1: u32,
    pub parse_irq: Option<unsafe extern "C" fn(*mut sm5502_muic_info, i32) -> i32>,
}

extern "C" {
    type device; type extcon_dev; type i2c_client; type regmap; type regmap_irq_chip_data;
    type work_struct; type delayed_work; type mutex; type regmap_irq_chip; type regmap_irq;
    fn regmap_update_bits(*mut regmap, u32, u32, u32) -> i32;
    fn regmap_read(*mut regmap, u32, *mut u32) -> i32;
    fn regmap_write(*mut regmap, u8, u32) -> i32;
    fn extcon_set_state_sync(*mut extcon_dev, u32, bool) -> i32;
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex);
    fn schedule_work(*mut work_struct) -> bool;
    fn regmap_irq_get_virq(*mut regmap_irq_chip_data, u32) -> i32;
    fn enable_irq_wake(i32) -> i32; fn disable_irq_wake(i32) -> i32;
}

#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub val_bits: u32, pub volatile_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub max_register: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const core::ffi::c_char, pub data: *const core::ffi::c_void }
#[repr(C)] pub struct i2c_device_id { pub name: *const core::ffi::c_char, pub driver_data: usize }

// Accessory ADC values and all SM5502/SM5504 constants are defined by extcon-sm5502.h.
#[repr(C)] pub struct regmap_irq { pub reg_offset: u32, pub mask: u32 }

static mut sm5502_reg_data: [reg_data; 4] = [
    reg_data { reg: SM5502_REG_RESET as u8, val: SM5502_REG_RESET_MASK, invert: true },
    reg_data { reg: SM5502_REG_CONTROL as u8, val: SM5502_REG_CONTROL_MASK_INT_MASK, invert: false },
    reg_data { reg: SM5502_REG_INTMASK1 as u8, val: SM5502_REG_INTM1_KP_MASK | SM5502_REG_INTM1_LKP_MASK | SM5502_REG_INTM1_LKR_MASK, invert: true },
    reg_data { reg: SM5502_REG_INTMASK2 as u8, val: SM5502_REG_INTM2_VBUS_DET_MASK | SM5502_REG_INTM2_REV_ACCE_MASK | SM5502_REG_INTM2_ADC_CHG_MASK | SM5502_REG_INTM2_STUCK_KEY_MASK | SM5502_REG_INTM2_STUCK_KEY_RCV_MASK | SM5502_REG_INTM2_MHL_MASK, invert: true },
];
static mut sm5504_reg_data: [reg_data; 4] = [
    reg_data { reg: SM5502_REG_RESET as u8, val: SM5502_REG_RESET_MASK, invert: true },
    reg_data { reg: SM5502_REG_INTMASK1 as u8, val: SM5504_REG_INTM1_ATTACH_MASK | SM5504_REG_INTM1_DETACH_MASK, invert: false },
    reg_data { reg: SM5502_REG_INTMASK2 as u8, val: SM5504_REG_INTM2_RID_CHG_MASK | SM5504_REG_INTM2_UVLO_MASK | SM5504_REG_INTM2_POR_MASK, invert: true },
    reg_data { reg: SM5502_REG_CONTROL as u8, val: SM5502_REG_CONTROL_MANUAL_SW_MASK | SM5504_REG_CONTROL_CHGTYP_MASK | SM5504_REG_CONTROL_USBCHDEN_MASK | SM5504_REG_CONTROL_ADC_EN_MASK, invert: true },
];
static sm5502_extcon_cable: [u32; 5] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_DCP, EXTCON_NONE];

#[repr(u32)] enum sm5502_muic_acc_type {
    SM5502_MUIC_ADC_GROUND=0, SM5502_MUIC_ADC_SEND_END_BUTTON, SM5502_MUIC_ADC_REMOTE_S1_BUTTON,
    SM5502_MUIC_ADC_REMOTE_S2_BUTTON, SM5502_MUIC_ADC_REMOTE_S3_BUTTON, SM5502_MUIC_ADC_REMOTE_S4_BUTTON,
    SM5502_MUIC_ADC_REMOTE_S5_BUTTON, SM5502_MUIC_ADC_REMOTE_S6_BUTTON, SM5502_MUIC_ADC_REMOTE_S7_BUTTON,
    SM5502_MUIC_ADC_REMOTE_S8_BUTTON, SM5502_MUIC_ADC_REMOTE_S9_BUTTON, SM5502_MUIC_ADC_REMOTE_S10_BUTTON,
    SM5502_MUIC_ADC_REMOTE_S11_BUTTON, SM5502_MUIC_ADC_REMOTE_S12_BUTTON, SM5502_MUIC_ADC_RESERVED_ACC_1,
    SM5502_MUIC_ADC_RESERVED_ACC_2, SM5502_MUIC_ADC_RESERVED_ACC_3, SM5502_MUIC_ADC_RESERVED_ACC_4,
    SM5502_MUIC_ADC_RESERVED_ACC_5, SM5502_MUIC_ADC_AUDIO_TYPE2, SM5502_MUIC_ADC_PHONE_POWERED_DEV,
    SM5502_MUIC_ADC_TTY_CONVERTER, SM5502_MUIC_ADC_UART_CABLE, SM5502_MUIC_ADC_TYPE1_CHARGER,
    SM5502_MUIC_ADC_FACTORY_MODE_BOOT_OFF_USB, SM5502_MUIC_ADC_FACTORY_MODE_BOOT_ON_USB,
    SM5502_MUIC_ADC_AUDIO_VIDEO_CABLE, SM5502_MUIC_ADC_TYPE2_CHARGER,
    SM5502_MUIC_ADC_FACTORY_MODE_BOOT_OFF_UART, SM5502_MUIC_ADC_FACTORY_MODE_BOOT_ON_UART,
    SM5502_MUIC_ADC_AUDIO_TYPE1, SM5502_MUIC_ADC_OPEN=0x1f,
    SM5502_MUIC_ADC_AUDIO_TYPE1_FULL_REMOTE=0x3e, SM5502_MUIC_ADC_AUDIO_TYPE1_SEND_END=0x5e,
    SM5502_MUIC_ADC_GROUND_USB_OTG=0x80, SM5502_MUIC_ADC_OPEN_USB=0x5f,
    SM5502_MUIC_ADC_OPEN_TA=0xdf, SM5502_MUIC_ADC_OPEN_USB_OTG=0xff,
}

unsafe extern "C" fn sm5502_muic_volatile_reg(_: *mut device, reg: u32) -> bool { reg == SM5502_REG_INTMASK1 || reg == SM5502_REG_INTMASK2 }
unsafe extern "C" fn sm5502_muic_set_path(info: *mut sm5502_muic_info, mut con_sw: u32, mut vbus_sw: u32, attached: bool) -> i32 {
    if !attached { con_sw=DM_DP_SWITCH_OPEN; vbus_sw=VBUSIN_SWITCH_OPEN; }
    match con_sw { DM_DP_SWITCH_OPEN|DM_DP_SWITCH_USB|DM_DP_SWITCH_AUDIO|DM_DP_SWITCH_UART => {}, _ => return -22 }
    let mut ret=regmap_update_bits((*info).regmap,SM5502_REG_MANUAL_SW1,SM5502_REG_MANUAL_SW1_DP_MASK|SM5502_REG_MANUAL_SW1_DM_MASK,con_sw); if ret<0{return ret;}
    match vbus_sw { VBUSIN_SWITCH_OPEN|VBUSIN_SWITCH_VBUSOUT|VBUSIN_SWITCH_MIC|VBUSIN_SWITCH_VBUSOUT_WITH_USB => {}, _ => return -22 }
    ret=regmap_update_bits((*info).regmap,SM5502_REG_MANUAL_SW1,SM5502_REG_MANUAL_SW1_VBUSIN_MASK,vbus_sw); if ret<0{return ret;} 0
}

unsafe extern "C" fn sm5502_muic_get_cable_type(info:*mut sm5502_muic_info)->u32 {
    let mut adc=0; if regmap_read((*info).regmap,SM5502_REG_ADC,&mut adc)!=0{return (-22i32) as u32;} let mut cable=adc&SM5502_REG_ADC_MASK;
    if cable==SM5502_MUIC_ADC_GROUND as u32 || cable==SM5502_MUIC_ADC_OPEN as u32 { let mut d=0; if regmap_read((*info).regmap,SM5502_REG_DEV_TYPE1,&mut d)!=0{return (-22i32) as u32;}; if d==(*(*info).r#type).otg_dev_type1 { return if cable==0 {SM5502_MUIC_ADC_GROUND_USB_OTG as u32} else {SM5502_MUIC_ADC_OPEN_USB_OTG as u32}; } if cable==SM5502_MUIC_ADC_OPEN as u32 { return match d { SM5502_REG_DEV_TYPE1_USB_SDP_MASK=>SM5502_MUIC_ADC_OPEN_USB as u32, SM5502_REG_DEV_TYPE1_DEDICATED_CHG_MASK=>SM5502_MUIC_ADC_OPEN_TA as u32, _=>(-22i32) as u32 }; } return (-22i32) as u32; } cable
}

unsafe extern "C" fn sm5502_muic_cable_handler(info:*mut sm5502_muic_info, attached:bool)->i32 { static mut prev:u32=0; let cable=if attached{sm5502_muic_get_cable_type(info)}else{prev}; prev=cable; let (id,con,vbus)=match cable { x if x==SM5502_MUIC_ADC_OPEN_USB as u32=>(EXTCON_USB,DM_DP_SWITCH_USB,VBUSIN_SWITCH_VBUSOUT_WITH_USB), x if x==SM5502_MUIC_ADC_OPEN_TA as u32=>(EXTCON_CHG_USB_DCP,DM_DP_SWITCH_OPEN,VBUSIN_SWITCH_VBUSOUT), x if x==SM5502_MUIC_ADC_GROUND_USB_OTG as u32||x==SM5502_MUIC_ADC_OPEN_USB_OTG as u32=>(EXTCON_USB_HOST,DM_DP_SWITCH_USB,VBUSIN_SWITCH_OPEN), _=>return 0}; let r=sm5502_muic_set_path(info,con,vbus,attached); if r<0{return r;} extcon_set_state_sync((*info).edev,id,attached); if id==EXTCON_USB{extcon_set_state_sync((*info).edev,EXTCON_CHG_USB_SDP,attached);} 0 }

unsafe extern "C" fn sm5502_parse_irq(info:*mut sm5502_muic_info, irq:i32)->i32 { if irq==SM5502_IRQ_INT1_ATTACH {(*info).irq_attach=true;} else if irq==SM5502_IRQ_INT1_DETACH {(*info).irq_detach=true;} 0 }
unsafe extern "C" fn sm5504_parse_irq(info:*mut sm5502_muic_info, irq:i32)->i32 { if irq==SM5504_IRQ_INT1_ATTACH {(*info).irq_attach=true;} else if irq==SM5504_IRQ_INT1_DETACH {(*info).irq_detach=true;} 0 }

unsafe extern "C" fn sm5502_muic_irq_handler(_irq:i32, data:*mut core::ffi::c_void)->i32 {
    let info=data as *mut sm5502_muic_info;
    if (*info).r#type.is_null() { return 1; }
    schedule_work(&mut (*info).irq_work); 1
}
unsafe extern "C" fn sm5502_muic_irq_work(work:*mut work_struct) {
    let info=work as *mut sm5502_muic_info;
    if info.is_null() || (*info).edev.is_null() { return; }
    mutex_lock(&mut (*info).mutex);
    if (*info).irq_attach { sm5502_muic_cable_handler(info,true); (*info).irq_attach=false; }
    if (*info).irq_detach { sm5502_muic_cable_handler(info,false); (*info).irq_detach=false; }
    mutex_unlock(&mut (*info).mutex);
}
unsafe extern "C" fn sm5502_muic_detect_cable_wq(work:*mut work_struct) {
    let info=work as *mut sm5502_muic_info; if !info.is_null(){sm5502_muic_cable_handler(info,true);}
}
unsafe extern "C" fn sm5502_init_dev_type(info:*mut sm5502_muic_info) {
    let mut reg=0; if regmap_read((*info).regmap,SM5502_REG_DEVICE_ID,&mut reg)!=0{return;}
    let n=(*(*info).r#type).num_reg_data; for i in 0..n { let d=&*(*(*info).r#type).reg_data.add(i as usize); let v=if d.invert{d.val}else{!d.val}; regmap_write((*info).regmap,d.reg,v); }
}

// The following declarations preserve the externally visible kernel-driver interfaces.
extern "C" {
    fn sm5022_muic_i2c_probe(i2c:*mut i2c_client)->i32;
    fn sm5502_muic_suspend(dev:*mut device)->i32;
    fn sm5502_muic_resume(dev:*mut device)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
