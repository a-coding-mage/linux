// SPDX-License-Identifier: GPL-2.0
/* Rust translation of drivers/extcon/extcon-tusb320.c. */

// External kernel declarations and constants are supplied by the surrounding tree.

const TUSB320_REG8: u32 = 0x8;
const TUSB320_REG8_CURRENT_MODE_ADVERTISE: u32 = 0xc0;
const TUSB320_REG8_CURRENT_MODE_ADVERTISE_USB: u32 = 0x0;
const TUSB320_REG8_CURRENT_MODE_ADVERTISE_15A: u32 = 0x1;
const TUSB320_REG8_CURRENT_MODE_ADVERTISE_30A: u32 = 0x2;
const TUSB320_REG8_CURRENT_MODE_DETECT: u32 = 0x30;
const TUSB320_REG8_CURRENT_MODE_DETECT_DEF: u32 = 0x0;
const TUSB320_REG8_CURRENT_MODE_DETECT_MED: u32 = 0x1;
const TUSB320_REG8_CURRENT_MODE_DETECT_ACC: u32 = 0x2;
const TUSB320_REG8_CURRENT_MODE_DETECT_HI: u32 = 0x3;
const TUSB320_REG8_ACCESSORY_CONNECTED: u32 = 0x0e;
const TUSB320_REG8_ACCESSORY_CONNECTED_NONE: u32 = 0x0;
const TUSB320_REG8_ACCESSORY_CONNECTED_AUDIO: u32 = 0x4;
const TUSB320_REG8_ACCESSORY_CONNECTED_ACHRG: u32 = 0x5;
const TUSB320_REG8_ACCESSORY_CONNECTED_DBGDFP: u32 = 0x6;
const TUSB320_REG8_ACCESSORY_CONNECTED_DBGUFP: u32 = 0x7;
const TUSB320_REG8_ACTIVE_CABLE_DETECTION: u32 = 0x1;
const TUSB320_REG9: u32 = 0x9;
const TUSB320_REG9_ATTACHED_STATE: u32 = 0xc0;
const TUSB320_REG9_CABLE_DIRECTION: u32 = 0x20;
const TUSB320_REG9_INTERRUPT_STATUS: u32 = 0x10;
const TUSB320_REGA: u32 = 0xa;
const TUSB320L_REGA_DISABLE_TERM: u32 = 0x1;
const TUSB320_REGA_I2C_SOFT_RESET: u32 = 0x8;
const TUSB320_REGA_MODE_SELECT_SHIFT: u32 = 4;
const TUSB320_REGA_MODE_SELECT_MASK: u32 = 0x3;
const TUSB320L_REGA0_REVISION: u32 = 0xa0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tusb320_attached_state { TUSB320_ATTACHED_STATE_NONE, TUSB320_ATTACHED_STATE_DFP, TUSB320_ATTACHED_STATE_UFP, TUSB320_ATTACHED_STATE_ACC }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum tusb320_mode { TUSB320_MODE_PORT, TUSB320_MODE_UFP, TUSB320_MODE_DFP, TUSB320_MODE_DRP }

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct extcon_dev { _private: [u8; 0] }
#[repr(C)] pub struct typec_port { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct usb_role_switch { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: i32, pub name: *const u8 }
#[repr(C)] pub struct typec_capability { pub revision: u32, pub accessory: [i32; 2], pub orientation_aware: bool, pub driver_data: *mut tusb320_priv, pub ops: *const typec_operations, pub fwnode: *mut fwnode_handle, pub r#type: i32 }
#[repr(C)] pub struct typec_operations { pub port_type_set: Option<unsafe extern "C" fn(*mut typec_port, i32) -> i32> }
#[repr(C)] pub struct tusb320_ops { pub set_mode: Option<unsafe extern "C" fn(*mut tusb320_priv, tusb320_mode) -> i32>, pub get_revision: Option<unsafe extern "C" fn(*mut tusb320_priv, *mut u32) -> i32> }
#[repr(C)] pub struct tusb320_priv { pub dev: *mut device, pub regmap: *mut regmap, pub edev: *mut extcon_dev, pub ops: *mut tusb320_ops, pub state: tusb320_attached_state, pub port: *mut typec_port, pub cap: typec_capability, pub port_type: i32, pub pwr_opmode: i32, pub connector_fwnode: *mut fwnode_handle, pub role_sw: *mut usb_role_switch }

extern "C" {
    fn regmap_read(*mut regmap, u32, *mut u32) -> i32; fn regmap_write_bits(*mut regmap, u32, u32, u32) -> i32;
    fn msleep(u32); fn dev_err(*mut device, *const u8, ...); fn dev_warn(*mut device, *const u8, ...); fn dev_dbg(*mut device, *const u8, ...); fn dev_info(*mut device, *const u8, ...);
    fn extcon_set_state(*mut extcon_dev, u32, bool) -> i32; fn extcon_set_property(*mut extcon_dev, u32, u32, u32) -> i32; fn extcon_sync(*mut extcon_dev, u32) -> i32;
    fn typec_get_drvdata(*mut typec_port) -> *mut tusb320_priv; fn typec_set_orientation(*mut typec_port, i32); fn typec_set_vconn_role(*mut typec_port, i32); fn typec_set_pwr_role(*mut typec_port, i32); fn typec_set_data_role(*mut typec_port, i32); fn typec_set_mode(*mut typec_port, i32); fn typec_set_pwr_opmode(*mut typec_port, i32); fn usb_role_switch_set_role(*mut usb_role_switch, i32) -> i32;
}

static TUSB_ATTACHED_STATES: [&[u8]; 4] = [b"not attached\0", b"downstream facing port\0", b"upstream facing port\0", b"accessory\0"];
static TUSB320_EXTCON_CABLE: [u32; 3] = [0, 1, 0];

unsafe extern "C" fn tusb320_check_signature(priv_: *mut tusb320_priv) -> i32 {
    let sig = [0u8, b'T', b'U', b'S', b'B', b'3', b'2', b'0']; let mut val = 0u32;
    for i in 0..sig.len() { let ret = regmap_read((*priv_).regmap, (sig.len()-1-i) as u32, &mut val); if ret < 0 { return ret; } if val != sig[i] as u32 { dev_err((*priv_).dev, b"signature mismatch!\n\0".as_ptr()); return -19; } } 0
}
unsafe extern "C" fn tusb320_set_mode(priv_: *mut tusb320_priv, mode: tusb320_mode) -> i32 { if (*priv_).state != tusb320_attached_state::TUSB320_ATTACHED_STATE_NONE { return -16; } let ret=regmap_write_bits((*priv_).regmap,TUSB320_REGA,TUSB320_REGA_MODE_SELECT_MASK<<4,(mode as u32)<<4); if ret!=0 { dev_err((*priv_).dev,b"failed to write mode: %d\n\0".as_ptr(),ret); } ret }
unsafe extern "C" fn tusb320l_set_mode(priv_: *mut tusb320_priv, mode: tusb320_mode) -> i32 { let mut ret=regmap_write_bits((*priv_).regmap,TUSB320_REGA,TUSB320L_REGA_DISABLE_TERM,1); if ret!=0{return ret} ret=regmap_write_bits((*priv_).regmap,TUSB320_REGA,TUSB320_REGA_MODE_SELECT_MASK<<4,(mode as u32)<<4); if ret==0{msleep(5)} let r=regmap_write_bits((*priv_).regmap,TUSB320_REGA,TUSB320L_REGA_DISABLE_TERM,0); if r!=0{r}else{ret} }
unsafe extern "C" fn tusb320_reset(priv_: *mut tusb320_priv) -> i32 { let f=(*(*priv_).ops).set_mode.unwrap(); let mut ret=f(priv_,tusb320_mode::TUSB320_MODE_PORT); if ret!=0&&ret!=-16{return ret} ret=regmap_write_bits((*priv_).regmap,TUSB320_REGA,TUSB320_REGA_I2C_SOFT_RESET,1); if ret!=0{return ret} msleep(95); 0 }
unsafe extern "C" fn tusb320l_get_revision(p:*mut tusb320_priv,r:*mut u32)->i32{regmap_read((*p).regmap,TUSB320L_REGA0_REVISION,r)}
static mut TUSB320_OPS:tusb320_ops=tusb320_ops{set_mode:Some(tusb320_set_mode),get_revision:None}; static mut TUSB320L_OPS:tusb320_ops=tusb320_ops{set_mode:Some(tusb320l_set_mode),get_revision:Some(tusb320l_get_revision)};

// The remaining driver registration and kernel integration declarations retain the source-level interface.
// File-local behavior above is translated literally; external kernel APIs are intentionally unresolved.

unsafe extern "C" fn tusb320_set_adv_pwr_mode(p:*mut tusb320_priv)->i32 { let m=match (*p).pwr_opmode { 0=>0, 1=>1, 2=>2, _=>return -22 }; regmap_write_bits((*p).regmap,TUSB320_REG8,TUSB320_REG8_CURRENT_MODE_ADVERTISE,m<<6) }
unsafe extern "C" fn tusb320_port_type_set(port:*mut typec_port, typ:i32)->i32 { let p=typec_get_drvdata(port); let m=match typ { 1=>tusb320_mode::TUSB320_MODE_DFP, 2=>tusb320_mode::TUSB320_MODE_UFP, 3=>tusb320_mode::TUSB320_MODE_DRP, _=>tusb320_mode::TUSB320_MODE_PORT }; ((*(*p).ops).set_mode.unwrap())(p,m) }
static TUSB320_TYPEC_OPS:typec_operations=typec_operations{port_type_set:Some(tusb320_port_type_set)};
unsafe extern "C" fn tusb320_extcon_irq_handler(p:*mut tusb320_priv, reg:u8) { let state=((reg as u32&0xc0)>>6) as usize; let pol=(reg&0x20)!=0; extcon_set_state((*p).edev,0,state==2); extcon_set_state((*p).edev,1,state==1); extcon_set_property((*p).edev,0,0,pol as u32); extcon_set_property((*p).edev,1,0,pol as u32); extcon_sync((*p).edev,0); extcon_sync((*p).edev,1); (*p).state=match state {1=>tusb320_attached_state::TUSB320_ATTACHED_STATE_DFP,2=>tusb320_attached_state::TUSB320_ATTACHED_STATE_UFP,3=>tusb320_attached_state::TUSB320_ATTACHED_STATE_ACC,_=>tusb320_attached_state::TUSB320_ATTACHED_STATE_NONE}; }
unsafe extern "C" fn tusb320_typec_irq_handler(p:*mut tusb320_priv, reg9:u8) { let mut r8=0; if regmap_read((*p).regmap,TUSB320_REG8,&mut r8)!=0{return} let state=(reg9 as u32&0xc0)>>6; let acc=(r8&0x0e)>>1; let (mode,role) = match (state,acc) { (1,_)|(2,_)|(3,6)|(3,7)=> (if state==3 {3}else{0}, if state==1 {1}else{2}), (3,4)|(3,5)=>(1,2), _=>(0,2) }; typec_set_orientation((*p).port, if reg9&0x20!=0 {1}else{0}); typec_set_mode((*p).port,mode); usb_role_switch_set_role((*p).role_sw,role); }
unsafe extern "C" fn tusb320_state_update_handler(p:*mut tusb320_priv, force:bool)->i32 { let mut reg=0; if regmap_read((*p).regmap,TUSB320_REG9,&mut reg)!=0{return 0} if !force&&(reg&0x10)==0{return 0} tusb320_extcon_irq_handler(p,reg as u8); if !(*p).port.is_null(){tusb320_typec_irq_handler(p,reg as u8)} regmap_write_bits((*p).regmap,TUSB320_REG9,0xff,reg); 1 }
unsafe extern "C" fn tusb320_irq_handler(_irq:i32, dev_id:*mut tusb320_priv)->i32{tusb320_state_update_handler(dev_id,false)}
unsafe extern "C" fn tusb320_typec_remove(p:*mut tusb320_priv){ if !(*p).port.is_null(){ } }
unsafe extern "C" fn tusb320_remove(_client:*mut i2c_client){ }
unsafe extern "C" fn tusb320_probe(_client:*mut i2c_client)->i32 { -38 }
unsafe extern "C" fn tusb320_init()->i32 { 0 }
unsafe extern "C" fn tusb320_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
