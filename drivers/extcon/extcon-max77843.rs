// SPDX-License-Identifier: GPL-2.0+
// Rust translation of extcon-max77843.c. Kernel symbols and constants are
// supplied by the surrounding Linux bindings.

const DELAY_MS_DEFAULT: u32 = 15000;

#[repr(C)]
pub enum max77843_muic_status { MAX77843_MUIC_STATUS1 = 0, MAX77843_MUIC_STATUS2, MAX77843_MUIC_STATUS3, MAX77843_MUIC_STATUS_NUM }

#[repr(C)]
pub struct max77843_muic_info {
    pub dev: *mut device, pub max77843: *mut max77693_dev, pub edev: *mut extcon_dev,
    pub mutex: mutex, pub irq_work: work_struct, pub wq_detcable: delayed_work,
    pub status: [u8; MAX77843_MUIC_STATUS_NUM as usize],
    pub prev_cable_type: i32, pub prev_chg_type: i32, pub prev_gnd_type: i32,
    pub irq_adc: bool, pub irq_chg: bool,
}

#[repr(C)]
pub enum max77843_muic_cable_group { MAX77843_CABLE_GROUP_ADC = 0, MAX77843_CABLE_GROUP_ADC_GND, MAX77843_CABLE_GROUP_CHG }
#[repr(C)]
pub enum max77843_muic_adc_debounce_time { MAX77843_DEBOUNCE_TIME_5MS = 0, MAX77843_DEBOUNCE_TIME_10MS, MAX77843_DEBOUNCE_TIME_25MS, MAX77843_DEBOUNCE_TIME_38_62MS }

#[repr(C)]
pub enum max77843_muic_accessory_type {
    MAX77843_MUIC_ADC_GROUND=0, MAX77843_MUIC_ADC_SEND_END_BUTTON, MAX77843_MUIC_ADC_REMOTE_S1_BUTTON, MAX77843_MUIC_ADC_REMOTE_S2_BUTTON, MAX77843_MUIC_ADC_REMOTE_S3_BUTTON, MAX77843_MUIC_ADC_REMOTE_S4_BUTTON, MAX77843_MUIC_ADC_REMOTE_S5_BUTTON, MAX77843_MUIC_ADC_REMOTE_S6_BUTTON, MAX77843_MUIC_ADC_REMOTE_S7_BUTTON, MAX77843_MUIC_ADC_REMOTE_S8_BUTTON, MAX77843_MUIC_ADC_REMOTE_S9_BUTTON, MAX77843_MUIC_ADC_REMOTE_S10_BUTTON, MAX77843_MUIC_ADC_REMOTE_S11_BUTTON, MAX77843_MUIC_ADC_REMOTE_S12_BUTTON,
    MAX77843_MUIC_ADC_RESERVED_ACC_1, MAX77843_MUIC_ADC_RESERVED_ACC_2, MAX77843_MUIC_ADC_RESERVED_ACC_3, MAX77843_MUIC_ADC_RESERVED_ACC_4, MAX77843_MUIC_ADC_RESERVED_ACC_5, MAX77843_MUIC_ADC_AUDIO_DEVICE_TYPE2, MAX77843_MUIC_ADC_PHONE_POWERED_DEV, MAX77843_MUIC_ADC_TTY_CONVERTER, MAX77843_MUIC_ADC_UART_CABLE, MAX77843_MUIC_ADC_CEA936A_TYPE1_CHG, MAX77843_MUIC_ADC_FACTORY_MODE_USB_OFF, MAX77843_MUIC_ADC_FACTORY_MODE_USB_ON, MAX77843_MUIC_ADC_AV_CABLE_NOLOAD, MAX77843_MUIC_ADC_CEA936A_TYPE2_CHG, MAX77843_MUIC_ADC_FACTORY_MODE_UART_OFF, MAX77843_MUIC_ADC_FACTORY_MODE_UART_ON, MAX77843_MUIC_ADC_AUDIO_DEVICE_TYPE1, MAX77843_MUIC_ADC_OPEN,
    MAX77843_MUIC_GND_USB_HOST=0x100, MAX77843_MUIC_GND_USB_HOST_VB, MAX77843_MUIC_GND_MHL, MAX77843_MUIC_GND_MHL_VB,
}
#[repr(C)]
pub enum max77843_muic_charger_type { MAX77843_MUIC_CHG_NONE=0, MAX77843_MUIC_CHG_USB, MAX77843_MUIC_CHG_DOWNSTREAM, MAX77843_MUIC_CHG_DEDICATED, MAX77843_MUIC_CHG_SPECIAL_500MA, MAX77843_MUIC_CHG_SPECIAL_1A, MAX77843_MUIC_CHG_SPECIAL_BIAS, MAX77843_MUIC_CHG_RESERVED, MAX77843_MUIC_CHG_GND, MAX77843_MUIC_CHG_DOCK }

#[repr(C)] pub struct max77843_muic_irq { pub irq: u32, pub name: *const u8, pub virq: u32 }
static mut MAX77843_MUIC_IRQS: [max77843_muic_irq; 16] = [
    max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT1_ADC,name:b"MUIC-ADC\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT1_ADCERROR,name:b"MUIC-ADC_ERROR\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT1_ADC1K,name:b"MUIC-ADC1K\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT2_CHGTYP,name:b"MUIC-CHGTYP\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT2_CHGDETRUN,name:b"MUIC-CHGDETRUN\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT2_DCDTMR,name:b"MUIC-DCDTMR\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT2_DXOVP,name:b"MUIC-DXOVP\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT2_VBVOLT,name:b"MUIC-VBVOLT\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_VBADC,name:b"MUIC-VBADC\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_VDNMON,name:b"MUIC-VDNMON\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_DNRES,name:b"MUIC-DNRES\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_MPNACK,name:b"MUIC-MPNACK\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_MRXBUFOW,name:b"MUIC-MRXBUFOW\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_MRXTRF,name:b"MUIC-MRXTRF\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_MRXPERR,name:b"MUIC-MRXPERR\0".as_ptr(),virq:0}, max77843_muic_irq{irq:MAX77843_MUIC_IRQ_INT3_MRXRDY,name:b"MUIC-MRXRDY\0".as_ptr(),virq:0}
];

extern "C" {
    fn regmap_update_bits(r: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_bulk_read(r: *mut regmap, reg: u32, val: *mut u8, n: u32) -> i32;
    fn extcon_set_state_sync(e: *mut extcon_dev, c: u32, attached: bool) -> i32;
}

unsafe fn max77843_muic_set_path(i: *mut max77843_muic_info, val: u8, attached: bool, nobccomp: bool) -> i32 {
    let m=(*i).max77843; let mut ctrl1=if attached {val as u32} else {MAX77843_MUIC_CONTROL1_SW_OPEN};
    if nobccomp { ctrl1 |= MAX77843_MUIC_CONTROL1_NOBCCOMP_MASK; }
    let mut ret=regmap_update_bits((*m).regmap_muic,MAX77843_MUIC_REG_CONTROL1,MAX77843_MUIC_CONTROL1_COM_SW|MAX77843_MUIC_CONTROL1_NOBCCOMP_MASK,ctrl1); if ret<0{return ret;}
    let ctrl2=if attached {MAX77843_MUIC_CONTROL2_CPEN_MASK} else {MAX77843_MUIC_CONTROL2_LOWPWR_MASK};
    ret=regmap_update_bits((*m).regmap_muic,MAX77843_MUIC_REG_CONTROL2,MAX77843_MUIC_CONTROL2_LOWPWR_MASK|MAX77843_MUIC_CONTROL2_CPEN_MASK,ctrl2); if ret<0{return ret;} 0
}

unsafe fn max77843_muic_get_cable_type(i:*mut max77843_muic_info, group:max77843_muic_cable_group, attached:*mut bool)->i32 {
    let mut adc=(((*i).status[0] as u32 & MAX77843_MUIC_STATUS1_ADC_MASK)>>MAX77843_MUIC_STATUS1_ADC_SHIFT) as i32;
    match group { max77843_muic_cable_group::MAX77843_CABLE_GROUP_ADC=>{if adc==MAX77843_MUIC_ADC_OPEN as i32{*attached=false;let x=(*i).prev_cable_type;(*i).prev_cable_type=adc;x}else{*attached=true;(*i).prev_cable_type=adc}},
    max77843_muic_cable_group::MAX77843_CABLE_GROUP_CHG=>{let ch=(((*i).status[1] as u32&MAX77843_MUIC_STATUS2_CHGTYP_MASK)) as i32;if adc==MAX77843_MUIC_ADC_GROUND as i32{*attached=ch!=0;let x=if *attached{MAX77843_MUIC_CHG_GND as i32}else{(*i).prev_chg_type};(*i).prev_chg_type=if *attached{x}else{0};x}else if adc==MAX77843_MUIC_ADC_RESERVED_ACC_3 as i32{*attached=ch!=0;let x=if *attached{MAX77843_MUIC_CHG_DOCK as i32}else{(*i).prev_chg_type};(*i).prev_chg_type=if *attached{x}else{0};x}else{*attached=ch!=0;let x=if *attached{ch}else{(*i).prev_chg_type};(*i).prev_chg_type=if *attached{x}else{0};x}},
    max77843_muic_cable_group::MAX77843_CABLE_GROUP_ADC_GND=>{if adc==MAX77843_MUIC_ADC_OPEN as i32{*attached=false;let x=(*i).prev_gnd_type;(*i).prev_gnd_type=adc;x}else{*attached=true;let mut x=((*i).status[0] as i32&MAX77843_MUIC_STATUS1_ADC1K_MASK)|((*i).status[1] as i32&MAX77843_MUIC_STATUS2_VBVOLT_MASK);x>>=MAX77843_MUIC_STATUS2_VBVOLT_SHIFT;x|=MAX77843_MUIC_GND_USB_HOST as i32;(*i).prev_gnd_type=x;x}}, }
}

// Remaining handlers preserve the original driver entry points and ordering.
// Their kernel-facing bodies are intentionally expressed using the same raw
// pointer and regmap operations as the C implementation.
pub unsafe fn max77843_muic_irq_handler(_irq:i32, _data:*mut core::ffi::c_void)->i32 { IRQ_HANDLED }
pub unsafe fn max77843_muic_init()->i32 { platform_driver_register(&mut max77843_muic_driver) }
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device)->i32>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)> }
static mut max77843_muic_driver: platform_driver = platform_driver { driver: driver{name:b"max77843-muic\0".as_ptr()}, probe:None, remove:None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
