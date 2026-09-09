/* SPDX-License-Identifier: GPL-2.0 */
/* HD-audio core stuff */

// C header dependencies are supplied by other translated units.

use core::ffi::{c_char, c_void};

pub type hda_nid_t = u16;

#[repr(C)] pub struct hdac_bus { _private: [u8; 0] }
#[repr(C)] pub struct hdac_stream { _private: [u8; 0] }
#[repr(C)] pub struct hda_device_id { _private: [u8; 0] }
#[repr(C)] pub struct hdac_widget_tree { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
pub type snd_pcm_format_t = u32;
pub type snd_pcm_subformat_t = u32;

#[repr(C)]
pub struct snd_array {
    pub used: u32,
    pub alloced: u32,
    pub elem_size: u32,
    pub alloc_align: u32,
    pub list: *mut c_void,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub type_: i32,
    pub bus: *mut hdac_bus,
    pub addr: u32,
    pub list: list_head,
    pub afg: hda_nid_t,
    pub mfg: hda_nid_t,
    pub vendor_id: u32,
    pub subsystem_id: u32,
    pub revision_id: u32,
    pub afg_function_id: u32,
    pub mfg_function_id: u32,
    pub afg_unsol: u32,
    pub mfg_unsol: u32,
    pub power_caps: u32,
    pub vendor_name: *const c_char,
    pub chip_name: *const c_char,
    pub exec_verb: Option<unsafe extern "C" fn(*mut hdac_device, u32, u32, *mut u32) -> i32>,
    pub num_nodes: u32,
    pub start_nid: hda_nid_t,
    pub end_nid: hda_nid_t,
    pub in_pm: atomic_t,
    pub widget_lock: mutex,
    pub widgets: *mut hdac_widget_tree,
    pub regmap: *mut regmap,
    pub regmap_lock: mutex,
    pub vendor_verbs: snd_array,
    pub lazy_cache: bool,
    pub caps_overwriting: bool,
    pub cache_coef: bool,
    pub registered: u32,
}

pub const HDA_DEV_CORE: i32 = 0;
pub const HDA_DEV_LEGACY: i32 = 1;
pub const HDA_DEV_ASOC: i32 = 2;
pub const SND_SKL_PCI_BIND_AUTO: i32 = 0;
pub const SND_SKL_PCI_BIND_LEGACY: i32 = 1;
pub const SND_SKL_PCI_BIND_ASOC: i32 = 2;
pub const HDA_INPUT: i32 = 0;
pub const HDA_OUTPUT: i32 = 1;

#[macro_export]
macro_rules! dev_to_hdac_dev { ($dev:expr) => { $dev as *mut $crate::hdac_device }; }

extern "C" {
    pub static snd_hda_bus_type: bus_type;
    pub fn snd_hdac_device_init(dev: *mut hdac_device, bus: *mut hdac_bus, name: *const c_char, addr: u32) -> i32;
    pub fn snd_hdac_device_exit(dev: *mut hdac_device);
    pub fn snd_hdac_device_register(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_device_unregister(codec: *mut hdac_device);
    pub fn snd_hdac_device_set_chip_name(codec: *mut hdac_device, name: *const c_char) -> i32;
    pub fn snd_hdac_codec_modalias(hdac: *const hdac_device, buf: *mut c_char, size: usize) -> i32;
    pub fn snd_hdac_refresh_widgets(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_read(codec: *mut hdac_device, nid: hda_nid_t, verb: u32, parm: u32, res: *mut u32) -> i32;
    pub fn _snd_hdac_read_parm(codec: *mut hdac_device, nid: hda_nid_t, parm: i32, res: *mut u32) -> i32;
    pub fn snd_hdac_read_parm_uncached(codec: *mut hdac_device, nid: hda_nid_t, parm: i32) -> i32;
    pub fn snd_hdac_override_parm(codec: *mut hdac_device, nid: hda_nid_t, parm: u32, val: u32) -> i32;
    pub fn snd_hdac_get_connections(codec: *mut hdac_device, nid: hda_nid_t, conn_list: *mut hda_nid_t, max_conns: i32) -> i32;
    pub fn snd_hdac_get_sub_nodes(codec: *mut hdac_device, nid: hda_nid_t, start_id: *mut hda_nid_t) -> i32;
    pub fn snd_hdac_stream_format_bits(format: snd_pcm_format_t, subformat: snd_pcm_subformat_t, maxbits: u32) -> u32;
    pub fn snd_hdac_stream_format(channels: u32, bits: u32, rate: u32) -> u32;
    pub fn snd_hdac_spdif_stream_format(channels: u32, bits: u32, rate: u32, spdif_ctls: u16) -> u32;
    pub fn snd_hdac_query_supported_pcm(codec: *mut hdac_device, nid: hda_nid_t, ratesp: *mut u32, formatsp: *mut u64, subformatsp: *mut u32, bpsp: *mut u32) -> i32;
    pub fn snd_hdac_is_supported_format(codec: *mut hdac_device, nid: hda_nid_t, format: u32) -> bool;
    pub fn snd_hdac_codec_read(hdac: *mut hdac_device, nid: hda_nid_t, flags: i32, verb: u32, parm: u32) -> i32;
    pub fn snd_hdac_codec_write(hdac: *mut hdac_device, nid: hda_nid_t, flags: i32, verb: u32, parm: u32) -> i32;
    pub fn snd_hdac_check_power_state(hdac: *mut hdac_device, nid: hda_nid_t, target_state: u32) -> bool;
    pub fn snd_hdac_sync_power_state(hdac: *mut hdac_device, nid: hda_nid_t, target_state: u32) -> u32;
    pub fn snd_hdac_power_up(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_power_down(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_power_up_pm(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_power_down_pm(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_keep_power_up(codec: *mut hdac_device) -> i32;
    pub fn snd_hdac_enter_pm(codec: *mut hdac_device);
    pub fn snd_hdac_leave_pm(codec: *mut hdac_device);
    pub fn snd_hdac_is_in_pm(codec: *mut hdac_device) -> bool;
    pub fn snd_hdac_is_power_on(codec: *mut hdac_device) -> bool;
}

#[inline]
pub unsafe fn snd_hdac_read_parm(codec: *mut hdac_device, nid: hda_nid_t, parm: i32) -> i32 {
    let mut val = 0u32;
    if _snd_hdac_read_parm(codec, nid, parm, &mut val) < 0 { -1 } else { val as i32 }
}

#[repr(C)]
pub struct hdac_driver {
    pub driver: device_driver,
    pub type_: i32,
    pub id_table: *const hda_device_id,
    pub match_: Option<unsafe extern "C" fn(*mut hdac_device, *const hdac_driver) -> i32>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hdac_device, u32)>,
    pub probe: Option<unsafe extern "C" fn(*mut hdac_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut hdac_device) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut hdac_device)>,
}

#[macro_export]
macro_rules! drv_to_hdac_driver { ($drv:expr) => { $drv as *mut $crate::hdac_driver }; }

extern "C" {
    pub fn hdac_get_device_id(hdev: *mut hdac_device, drv: *const hdac_driver) -> *const hda_device_id;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
