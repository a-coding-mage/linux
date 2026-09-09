/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Linux types and externally supplied symbols
// are intentionally referenced rather than reimplemented here.

pub const ASUS_WMI_MGMT_GUID: &str = "97845ED0-4E6D-11DE-8A39-0800200C9A66";
pub const ASUS_ACPI_UID_ASUSWMI: &str = "ASUSWMI";

/* WMI Methods */
pub const ASUS_WMI_METHODID_SPEC: u32 = 0x4345_5053; // BIOS SPECification
pub const ASUS_WMI_METHODID_SFBD: u32 = 0x4442_4653; // Set First Boot Device
pub const ASUS_WMI_METHODID_GLCD: u32 = 0x4443_4C47; // Get LCD status
pub const ASUS_WMI_METHODID_GPID: u32 = 0x4449_5047; // Get Panel ID?? (Resol)
pub const ASUS_WMI_METHODID_QMOD: u32 = 0x444F_4D51; // Quiet MODe
pub const ASUS_WMI_METHODID_SPLV: u32 = 0x4C42_5053; // Set Panel Light Value
pub const ASUS_WMI_METHODID_AGFN: u32 = 0x4E46_4741; // Atk Generic FuNction
pub const ASUS_WMI_METHODID_SFUN: u32 = 0x4E55_4653; // FUNCtionalities
pub const ASUS_WMI_METHODID_SDSP: u32 = 0x5053_4453; // Set DiSPlay output
pub const ASUS_WMI_METHODID_GDSP: u32 = 0x5053_4447; // Get DiSPlay output
pub const ASUS_WMI_METHODID_DEVP: u32 = 0x5056_4544; // DEVice Policy
pub const ASUS_WMI_METHODID_OSVR: u32 = 0x5256_534F; // OS VeRsion
pub const ASUS_WMI_METHODID_DCTS: u32 = 0x5354_4344; // Device status (DCTS)
pub const ASUS_WMI_METHODID_DSTS: u32 = 0x5354_5344; // Device status (DSTS)
pub const ASUS_WMI_METHODID_BSTS: u32 = 0x5354_5342; // Bios STatuS ?
pub const ASUS_WMI_METHODID_DEVS: u32 = 0x5356_4544; // DEVice Set
pub const ASUS_WMI_METHODID_CFVS: u32 = 0x5356_4643; // CPU Frequency Volt Set
pub const ASUS_WMI_METHODID_KBFT: u32 = 0x5446_424B; // KeyBoard FilTer
pub const ASUS_WMI_METHODID_INIT: u32 = 0x5449_4E49; // INITialize
pub const ASUS_WMI_METHODID_HKEY: u32 = 0x5945_4B48; // Hot KEY ??
pub const ASUS_WMI_METHODID_NOTIF: u32 = 0x0010_0021; // Notify method

pub const ASUS_WMI_UNSUPPORTED_METHOD: u32 = 0xFFFF_FFFE;

/* Wireless */
pub const ASUS_WMI_DEVID_HW_SWITCH: u32 = 0x0001_0001;
pub const ASUS_WMI_DEVID_WIRELESS_LED: u32 = 0x0001_0002;
pub const ASUS_WMI_DEVID_CWAP: u32 = 0x0001_0003;
pub const ASUS_WMI_DEVID_WLAN: u32 = 0x0001_0011;
pub const ASUS_WMI_DEVID_WLAN_LED: u32 = 0x0001_0012;
pub const ASUS_WMI_DEVID_BLUETOOTH: u32 = 0x0001_0013;
pub const ASUS_WMI_DEVID_GPS: u32 = 0x0001_0015;
pub const ASUS_WMI_DEVID_WIMAX: u32 = 0x0001_0017;
pub const ASUS_WMI_DEVID_WWAN3G: u32 = 0x0001_0019;
pub const ASUS_WMI_DEVID_UWB: u32 = 0x0001_0021;

/* Leds */
/* 0x000200XX and 0x000400XX */
pub const ASUS_WMI_DEVID_LED1: u32 = 0x0002_0011;
pub const ASUS_WMI_DEVID_LED2: u32 = 0x0002_0012;
pub const ASUS_WMI_DEVID_LED3: u32 = 0x0002_0013;
pub const ASUS_WMI_DEVID_LED4: u32 = 0x0002_0014;
pub const ASUS_WMI_DEVID_LED5: u32 = 0x0002_0015;
pub const ASUS_WMI_DEVID_LED6: u32 = 0x0002_0016;
pub const ASUS_WMI_DEVID_MICMUTE_LED: u32 = 0x0004_0017;

/* Disable Camera LED */
pub const ASUS_WMI_DEVID_CAMERA_LED_NEG: u32 = 0x0006_0078; // 0 = on (unused)
pub const ASUS_WMI_DEVID_CAMERA_LED: u32 = 0x0006_0079; // 1 = on

/* Backlight and Brightness */
pub const ASUS_WMI_DEVID_ALS_ENABLE: u32 = 0x0005_0001; // Ambient Light Sensor
pub const ASUS_WMI_DEVID_BACKLIGHT: u32 = 0x0005_0011;
pub const ASUS_WMI_DEVID_BRIGHTNESS: u32 = 0x0005_0012;
pub const ASUS_WMI_DEVID_KBD_BACKLIGHT: u32 = 0x0005_0021;
pub const ASUS_WMI_DEVID_LIGHT_SENSOR: u32 = 0x0005_0022; // ??
pub const ASUS_WMI_DEVID_LIGHTBAR: u32 = 0x0005_0025;
pub const ASUS_WMI_DEVID_OOBE: u32 = 0x0005_002F;
/* This can only be used to disable the screen, not re-enable */
pub const ASUS_WMI_DEVID_SCREENPAD_POWER: u32 = 0x0005_0031;
/* Writing a brightness re-enables the screen if disabled */
pub const ASUS_WMI_DEVID_SCREENPAD_LIGHT: u32 = 0x0005_0032;
pub const ASUS_WMI_DEVID_FAN_BOOST_MODE: u32 = 0x0011_0018;
pub const ASUS_WMI_DEVID_THROTTLE_THERMAL_POLICY: u32 = 0x0012_0075;
pub const ASUS_WMI_DEVID_THROTTLE_THERMAL_POLICY_VIVO: u32 = 0x0011_0019;

/* Misc */
pub const ASUS_WMI_DEVID_PANEL_HD: u32 = 0x0005_001C;
pub const ASUS_WMI_DEVID_PANEL_OD: u32 = 0x0005_0019;
pub const ASUS_WMI_DEVID_CAMERA: u32 = 0x0006_0013;
pub const ASUS_WMI_DEVID_LID_FLIP: u32 = 0x0006_0062;
pub const ASUS_WMI_DEVID_LID_FLIP_ROG: u32 = 0x0006_0077;
pub const ASUS_WMI_DEVID_MINI_LED_MODE: u32 = 0x0005_001E;
pub const ASUS_WMI_DEVID_MINI_LED_MODE2: u32 = 0x0005_002E;
pub const ASUS_WMI_DEVID_SCREEN_AUTO_BRIGHTNESS: u32 = 0x0005_002A;
/* Storage */
pub const ASUS_WMI_DEVID_CARDREADER: u32 = 0x0008_0013;
/* Input */
pub const ASUS_WMI_DEVID_TOUCHPAD: u32 = 0x0010_0011;
pub const ASUS_WMI_DEVID_TOUCHPAD_LED: u32 = 0x0010_0012;
pub const ASUS_WMI_DEVID_FNLOCK: u32 = 0x0010_0023;
/* Fan, Thermal */
pub const ASUS_WMI_DEVID_THERMAL_CTRL: u32 = 0x0011_0011;
pub const ASUS_WMI_DEVID_FAN_CTRL: u32 = 0x0011_0012; // deprecated
pub const ASUS_WMI_DEVID_CPU_FAN_CTRL: u32 = 0x0011_0013;
pub const ASUS_WMI_DEVID_GPU_FAN_CTRL: u32 = 0x0011_0014;
pub const ASUS_WMI_DEVID_MID_FAN_CTRL: u32 = 0x0011_0031;
pub const ASUS_WMI_DEVID_CPU_FAN_CURVE: u32 = 0x0011_0024;
pub const ASUS_WMI_DEVID_GPU_FAN_CURVE: u32 = 0x0011_0025;
pub const ASUS_WMI_DEVID_MID_FAN_CURVE: u32 = 0x0011_0032;
/* Tunables for AUS ROG laptops */
pub const ASUS_WMI_DEVID_PPT_PL2_SPPT: u32 = 0x0012_00A0;
pub const ASUS_WMI_DEVID_PPT_PL1_SPL: u32 = 0x0012_00A3;
pub const ASUS_WMI_DEVID_PPT_APU_SPPT: u32 = 0x0012_00B0;
pub const ASUS_WMI_DEVID_PPT_PLAT_SPPT: u32 = 0x0012_00B1;
pub const ASUS_WMI_DEVID_PPT_PL3_FPPT: u32 = 0x0012_00C1;
pub const ASUS_WMI_DEVID_NV_DYN_BOOST: u32 = 0x0012_00C0;
pub const ASUS_WMI_DEVID_NV_THERM_TARGET: u32 = 0x0012_00C2;
/* Power */
pub const ASUS_WMI_DEVID_PROCESSOR_STATE: u32 = 0x0012_0012;
/* Deep S3 / Resume on LID open */
pub const ASUS_WMI_DEVID_LID_RESUME: u32 = 0x0012_0031;
/* Maximum charging percentage */
pub const ASUS_WMI_DEVID_RSOC: u32 = 0x0012_0057;
/* Keyboard dock */
pub const ASUS_WMI_DEVID_KBD_DOCK: u32 = 0x0012_0063;
/* Charging mode - 1=Barrel, 2=USB */
pub const ASUS_WMI_DEVID_CHARGE_MODE: u32 = 0x0012_006C;
/* MCU powersave mode */
pub const ASUS_WMI_DEVID_MCU_POWERSAVE: u32 = 0x0012_00E2;
/* epu is connected? 1 == true */
pub const ASUS_WMI_DEVID_EGPU_CONNECTED: u32 = 0x0009_0018;
/* egpu on/off */
pub const ASUS_WMI_DEVID_EGPU: u32 = 0x0009_0019;
/* dgpu on/off */
pub const ASUS_WMI_DEVID_DGPU: u32 = 0x0009_0020;
pub const ASUS_WMI_DEVID_APU_MEM: u32 = 0x0006_00C1;
pub const ASUS_WMI_DEVID_DGPU_BASE_TGP: u32 = 0x0012_0099;
pub const ASUS_WMI_DEVID_DGPU_SET_TGP: u32 = 0x0012_0098;
/* gpu mux switch, 0 = dGPU, 1 = Optimus */
pub const ASUS_WMI_DEVID_GPU_MUX: u32 = 0x0009_0016;
pub const ASUS_WMI_DEVID_GPU_MUX_VIVO: u32 = 0x0009_0026;
/* Keystone dongle insert/remove state.
 * PRESENCE_BIT (0x00010000) encodes insert state:
 * 0x00010000 = inserted, 0x00000000 = absent. STATUS_BIT is never set.
 * 0xFFFFFFFE means no keystone slot on this machine.
 */
pub const ASUS_WMI_DEVID_KEYSTONE: u32 = 0x0012_0091;
/* TUF laptop RGB modes/colours */
pub const ASUS_WMI_DEVID_TUF_RGB_MODE: u32 = 0x0010_0056;
pub const ASUS_WMI_DEVID_TUF_RGB_MODE2: u32 = 0x0010_005A;
/* TUF laptop RGB power/state */
pub const ASUS_WMI_DEVID_TUF_RGB_STATE: u32 = 0x0010_0057;
/* Bootup sound control */
pub const ASUS_WMI_DEVID_BOOT_SOUND: u32 = 0x0013_0022;
/* DSTS masks */
pub const ASUS_WMI_DSTS_STATUS_BIT: u32 = 0x0000_0001;
pub const ASUS_WMI_DSTS_UNKNOWN_BIT: u32 = 0x0000_0002;
pub const ASUS_WMI_DSTS_PRESENCE_BIT: u32 = 0x0001_0000;
pub const ASUS_WMI_DSTS_USER_BIT: u32 = 0x0002_0000;
pub const ASUS_WMI_DSTS_BIOS_BIT: u32 = 0x0004_0000;
pub const ASUS_WMI_DSTS_BRIGHTNESS_MASK: u32 = 0x0000_00FF;
pub const ASUS_WMI_DSTS_MAX_BRIGTH_MASK: u32 = 0x0000_FF00;
pub const ASUS_WMI_DSTS_LIGHTBAR_MASK: u32 = 0x0000_000F;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum asus_ally_mcu_hack {
    ASUS_WMI_ALLY_MCU_HACK_INIT,
    ASUS_WMI_ALLY_MCU_HACK_ENABLED,
    ASUS_WMI_ALLY_MCU_HACK_DISABLED,
}

/* Used to notify hid-asus when asus-wmi changes keyboard backlight */
#[repr(C)]
pub struct asus_hid_listener {
    pub list: list_head,
    pub brightness_set: Option<unsafe extern "C" fn(listener: *mut asus_hid_listener, brightness: i32)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum asus_hid_event {
    ASUS_EV_BRTUP,
    ASUS_EV_BRTDOWN,
    ASUS_EV_BRTTOGGLE,
}

pub const ASUS_EV_MAX_BRIGHTNESS: u32 = 3;

// `IS_REACHABLE(CONFIG_ASUS_WMI)` is a kernel build-time condition and is
// preserved here as a Rust feature condition.
#[cfg(feature = "CONFIG_ASUS_WMI")]
extern "C" {
    pub fn set_ally_mcu_hack(status: asus_ally_mcu_hack);
    pub fn set_ally_mcu_powersave(enabled: bool);
    pub fn asus_wmi_get_devstate_dsts(dev_id: u32, retval: *mut u32) -> i32;
    pub fn asus_wmi_set_devstate(dev_id: u32, ctrl_param: u32, retval: *mut u32) -> i32;
    pub fn asus_wmi_evaluate_method(method_id: u32, arg0: u32, arg1: u32, retval: *mut u32) -> i32;
    pub fn asus_hid_register_listener(cdev: *mut asus_hid_listener) -> i32;
    pub fn asus_hid_unregister_listener(cdev: *mut asus_hid_listener);
    pub fn asus_hid_event(event: asus_hid_event) -> i32;
    pub fn asus_wmi_custom_fan_curve_is_enabled() -> bool;
}

#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn set_ally_mcu_hack(_status: asus_ally_mcu_hack) {}
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn set_ally_mcu_powersave(_enabled: bool) {}
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_wmi_set_devstate(_dev_id: u32, _ctrl_param: u32, _retval: *mut u32) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_wmi_get_devstate_dsts(_dev_id: u32, _retval: *mut u32) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_wmi_evaluate_method(_method_id: u32, _arg0: u32, _arg1: u32, _retval: *mut u32) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_hid_register_listener(_bdev: *mut asus_hid_listener) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_hid_unregister_listener(_bdev: *mut asus_hid_listener) {}
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_hid_event(_event: asus_hid_event) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_ASUS_WMI"))]
pub unsafe fn asus_wmi_custom_fan_curve_is_enabled() -> bool { false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
