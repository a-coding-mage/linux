// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Pinctrl Protocol */

// External kernel and SCMI dependencies are supplied by the surrounding tree.

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x10000;
pub const CONFIG_FLAG_MASK: u32 = 0x000c0000;
pub const SELECTOR_MASK: u32 = 0x00030000;
pub const SKIP_CONFIGS_MASK: u32 = 0x0000ff00;
pub const CONFIG_TYPE_MASK: u32 = 0x000000ff;

#[repr(u32)]
pub enum ScmiPinctrlProtocolCmd { PinctrlAttributes=0x3, PinctrlListAssociations=0x4,
    PinctrlSettingsGet=0x5, PinctrlSettingsConfigure=0x6, PinctrlRequest=0x7,
    PinctrlRelease=0x8, PinctrlNameGet=0x9, PinctrlSetPermissions=0xa }

#[repr(C)] pub struct ScmiMsgSettingsConf { pub identifier:u32, pub function_id:u32, pub attributes:u32, pub configs:[u32;0] }
#[repr(C)] pub struct ScmiMsgSettingsGet { pub identifier:u32, pub attributes:u32 }
#[repr(C)] pub struct ScmiRespSettingsGet { pub function_selected:u32, pub num_configs:u32, pub configs:[u32;0] }
#[repr(C)] pub struct ScmiMsgPinctrlProtocolAttributes { pub attributes_low:u32, pub attributes_high:u32 }
#[repr(C)] pub struct ScmiMsgPinctrlAttributes { pub identifier:u32, pub flags:u32 }
#[repr(C)] pub struct ScmiRespPinctrlAttributes { pub attributes:u32, pub name:[u8; SCMI_SHORT_NAME_MAX_SIZE] }
#[repr(C)] pub struct ScmiMsgPinctrlListAssoc { pub identifier:u32, pub flags:u32, pub index:u32 }
#[repr(C)] pub struct ScmiRespPinctrlListAssoc { pub flags:u32, pub array:[u16;0] }
#[repr(C)] pub struct ScmiMsgRequest { pub identifier:u32, pub flags:u32 }

#[repr(C)] pub struct ScmiGroupInfo { pub name:[i8;SCMI_MAX_STR_SIZE], pub present:bool, pub group_pins:*mut u32, pub nr_pins:u32 }
#[repr(C)] pub struct ScmiFunctionInfo { pub name:[i8;SCMI_MAX_STR_SIZE], pub present:bool, pub groups:*mut u32, pub nr_groups:u32 }
#[repr(C)] pub struct ScmiPinInfo { pub name:[i8;SCMI_MAX_STR_SIZE], pub present:bool }
#[repr(C)] pub struct ScmiPinctrlInfo { pub nr_groups:i32, pub nr_functions:i32, pub nr_pins:i32, pub groups:*mut ScmiGroupInfo, pub functions:*mut ScmiFunctionInfo, pub pins:*mut ScmiPinInfo }
#[repr(C)] pub struct ScmiPinctrlIpriv { pub selector:u32, pub r#type:ScmiPinctrlSelectorType, pub array:*mut u32 }
#[repr(C)] pub struct ScmiSettingsGetIpriv { pub selector:u32, pub r#type:ScmiPinctrlSelectorType, pub get_all:bool, pub nr_configs:*mut u32, pub config_types:*mut ScmiPinctrlConfType, pub config_values:*mut u32 }

extern "C" {
    fn scmi_pinctrl_count_get(ph:*const ScmiProtocolHandle, t:ScmiPinctrlSelectorType)->i32;
}

unsafe fn scmi_pinctrl_validate_id(ph:*const ScmiProtocolHandle, selector:u32, t:ScmiPinctrlSelectorType)->i32 {
    let value=scmi_pinctrl_count_get(ph,t); if value<0 { return value; }
    if selector >= value as u32 || value==0 { return -22; } 0
}

// The following declarations mirror the implementation entry points and retain
// the original protocol operations. Kernel-provided types and helpers are external.
pub unsafe fn scmi_pinctrl_attributes_get(ph:*const ScmiProtocolHandle, pi:*mut ScmiPinctrlInfo)->i32 {
    let mut t:*mut ScmiXfer=core::ptr::null_mut();
    let mut ret=((*(*ph).xops).xfer_get_init)(ph, PROTOCOL_ATTRIBUTES,0,core::mem::size_of::<ScmiMsgPinctrlProtocolAttributes>(),&mut t);
    if ret!=0{return ret;} let attr=(*t).rx.buf as *const ScmiMsgPinctrlProtocolAttributes;
    ret=((*(*ph).xops).do_xfer)(ph,t); if ret==0 { (*pi).nr_functions=(((*attr).attributes_high)&0xffff) as i32; (*pi).nr_groups=(((*attr).attributes_low)>>16) as i32; (*pi).nr_pins=((*attr).attributes_low&0xffff) as i32; if (*pi).nr_pins==0 { ret=-22; } } ((*(*ph).xops).xfer_put)(ph,t); ret
}

pub unsafe fn scmi_pinctrl_count_get_impl(ph:*const ScmiProtocolHandle,t:ScmiPinctrlSelectorType)->i32 { let p=((*ph).get_priv)(ph) as *mut ScmiPinctrlInfo; match t { ScmiPinctrlSelectorType::Pin=>(*p).nr_pins, ScmiPinctrlSelectorType::Group=>(*p).nr_groups, ScmiPinctrlSelectorType::Function=>(*p).nr_functions, _=>-22 } }

pub unsafe fn scmi_pinctrl_request_free(ph:*const ScmiProtocolHandle, identifier:u32, t:ScmiPinctrlSelectorType, cmd:ScmiPinctrlProtocolCmd)->i32 { if matches!(t,ScmiPinctrlSelectorType::Function)||!matches!(cmd,ScmiPinctrlProtocolCmd::PinctrlRequest|ScmiPinctrlProtocolCmd::PinctrlRelease){return -22;} let mut r=scmi_pinctrl_validate_id(ph,identifier,t); if r!=0{return r;} let mut x:*mut ScmiXfer=core::ptr::null_mut(); r=((*(*ph).xops).xfer_get_init)(ph,cmd as u32,8,0,&mut x); if r!=0{return r;} let q=(*x).tx.buf as *mut ScmiMsgRequest; (*q).identifier=identifier.to_le(); (*q).flags=(t as u32).to_le(); r=((*(*ph).xops).do_xfer)(ph,x); if r==-95{r=0;} ((*(*ph).xops).xfer_put)(ph,x); r }

pub unsafe fn scmi_pinctrl_pin_request(ph:*const ScmiProtocolHandle,pin:u32)->i32{scmi_pinctrl_request_free(ph,pin,ScmiPinctrlSelectorType::Pin,ScmiPinctrlProtocolCmd::PinctrlRequest)}
pub unsafe fn scmi_pinctrl_pin_free(ph:*const ScmiProtocolHandle,pin:u32)->i32{scmi_pinctrl_request_free(ph,pin,ScmiPinctrlSelectorType::Pin,ScmiPinctrlProtocolCmd::PinctrlRelease)}

// Remaining iterator, metadata, configuration, protocol-init, and registration
// symbols are intentionally declared for linkage with the SCMI implementation.
extern "C" { pub fn scmi_pinctrl_settings_get_one(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _:ScmiPinctrlConfType, _:*mut u32)->i32; pub fn scmi_pinctrl_settings_get_all(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _:*mut u32, _:*mut ScmiPinctrlConfType, _:*mut u32)->i32; pub fn scmi_pinctrl_settings_conf(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _:u32, _:*mut ScmiPinctrlConfType, _:*mut u32)->i32; }

extern "C" {
    pub fn scmi_pinctrl_attributes(_: *const ScmiProtocolHandle, _:ScmiPinctrlSelectorType, _:u32, _: *mut i8, _: *mut u32)->i32;
    pub fn scmi_pinctrl_list_associations(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _:u16, _: *mut u32)->i32;
    pub fn scmi_pinctrl_function_select(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _:u32)->i32;
    pub fn scmi_pinctrl_get_group_info(_: *const ScmiProtocolHandle, _:u32)->i32;
    pub fn scmi_pinctrl_get_group_name(_: *const ScmiProtocolHandle, _:u32, _: *mut *const i8)->i32;
    pub fn scmi_pinctrl_group_pins_get(_: *const ScmiProtocolHandle, _:u32, _: *mut *const u32, _: *mut u32)->i32;
    pub fn scmi_pinctrl_get_function_info(_: *const ScmiProtocolHandle, _:u32)->i32;
    pub fn scmi_pinctrl_get_function_name(_: *const ScmiProtocolHandle, _:u32, _: *mut *const i8)->i32;
    pub fn scmi_pinctrl_function_groups_get(_: *const ScmiProtocolHandle, _:u32, _: *mut u32, _: *mut *const u32)->i32;
    pub fn scmi_pinctrl_mux_set(_: *const ScmiProtocolHandle, _:u32, _:u32)->i32;
    pub fn scmi_pinctrl_get_pin_info(_: *const ScmiProtocolHandle, _:u32)->i32;
    pub fn scmi_pinctrl_get_pin_name(_: *const ScmiProtocolHandle, _:u32, _: *mut *const i8)->i32;
    pub fn scmi_pinctrl_name_get(_: *const ScmiProtocolHandle, _:u32, _:ScmiPinctrlSelectorType, _: *mut *const i8)->i32;
    pub fn scmi_pinctrl_protocol_init(_: *const ScmiProtocolHandle)->i32;
    pub fn scmi_pinctrl_protocol_deinit(_: *const ScmiProtocolHandle)->i32;
}

// Supplied by common SCMI headers.
pub enum ScmiPinctrlSelectorType { Pin=0, Group=1, Function=2 }
pub enum ScmiPinctrlConfType { Unknown=0 }
pub enum ScmiProtocolHandle {}
pub enum ScmiXfer {}
pub const SCMI_SHORT_NAME_MAX_SIZE:usize=32; pub const SCMI_MAX_STR_SIZE:usize=64; pub const PROTOCOL_ATTRIBUTES:u32=0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
