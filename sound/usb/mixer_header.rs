// SPDX-License-Identifier: GPL-2.0
// Requires external: sound/info.h

pub struct media_mixer_ctl;

#[repr(C)]
pub struct usbmix_connector_map {
    pub id: u8,
    pub delegated_id: u8,
    pub control: u8,
    pub channel: u8,
}

#[repr(C)]
pub struct usb_mixer_interface {
    pub chip: *mut snd_usb_audio,
    pub hostif: *mut usb_host_interface,
    pub list: list_head,
    pub ignore_ctl_error: core::ffi::c_uint,
    // UAC2 status interrupt endpoint; owned by mixer.c
    pub urb: *mut urb,
    // array[MAX_ID_ELEMS], indexed by unit id
    pub id_elems: *mut *mut usb_mixer_elem_list,
    // the usb audio specification version this interface complies to
    pub protocol: core::ffi::c_int,
    // optional connector delegation map
    pub connector_map: *const usbmix_connector_map,
    // Sound Blaster remote control stuff
    pub rc_cfg: *const rc_config,
    pub rc_code: u32,
    pub rc_waitq: wait_queue_head_t,
    pub rc_urb: *mut urb,
    pub rc_setup_packet: *mut usb_ctrlrequest,
    pub rc_buffer: [u8; 6],
    pub media_mixer_ctl: *mut media_mixer_ctl,
    pub disconnected: bool,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<extern "C" fn(*mut usb_mixer_interface)>,
    pub private_suspend: Option<extern "C" fn(*mut usb_mixer_interface)>,
    pub private_resume: Option<extern "C" fn(*mut usb_mixer_interface) -> core::ffi::c_int>,
}

pub const MAX_CHANNELS: usize = 64;

pub const USB_MIXER_BOOLEAN: u32 = 0;
pub const USB_MIXER_INV_BOOLEAN: u32 = 1;
pub const USB_MIXER_S8: u32 = 2;
pub const USB_MIXER_U8: u32 = 3;
pub const USB_MIXER_S16: u32 = 4;
pub const USB_MIXER_U16: u32 = 5;
pub const USB_MIXER_S32: u32 = 6;
pub const USB_MIXER_U32: u32 = 7;
pub const USB_MIXER_BESPOKEN: u32 = 8;

pub type usb_mixer_elem_dump_func_t = extern "C" fn(*mut snd_info_buffer, *mut usb_mixer_elem_list) -> ();
pub type usb_mixer_elem_resume_func_t = extern "C" fn(*mut usb_mixer_elem_list) -> core::ffi::c_int;

#[repr(C)]
pub struct usb_mixer_elem_list {
    pub mixer: *mut usb_mixer_interface,
    pub next_id_elem: *mut usb_mixer_elem_list,
    pub kctl: *mut snd_kcontrol,
    pub id: core::ffi::c_uint,
    pub is_std_info: bool,
    pub dump: Option<usb_mixer_elem_dump_func_t>,
    pub resume: Option<usb_mixer_elem_resume_func_t>,
}

// C macro: for_each_mixer_elem(list, mixer, id)
// for ((list) = (mixer)->id_elems[id]; (list); (list) = (list)->next_id_elem)

// C macro: mixer_elem_list_to_info(list)
// container_of(list, struct usb_mixer_elem_info, head)

#[repr(C)]
pub struct usb_mixer_elem_info {
    pub head: usb_mixer_elem_list,
    pub control: core::ffi::c_uint,
    pub cmask: u64,
    pub idx_off: core::ffi::c_uint,
    pub ch_readonly: core::ffi::c_uint,
    pub master_readonly: core::ffi::c_uint,
    pub channels: core::ffi::c_int,
    pub val_type: core::ffi::c_int,
    pub min: core::ffi::c_int,
    pub max: core::ffi::c_int,
    pub res: core::ffi::c_int,
    pub max_exposed: core::ffi::c_int,
    pub dBmin: core::ffi::c_int,
    pub dBmax: core::ffi::c_int,
    pub cached: core::ffi::c_int,
    pub cache_val: [core::ffi::c_int; MAX_CHANNELS],
    pub initialized: u8,
    pub min_mute: u8,
    pub get_cur_broken: u8,
    pub private_data: *mut core::ffi::c_void,
}

extern "C" {
    pub fn snd_usb_create_mixer(chip: *mut snd_usb_audio, ctrlif: core::ffi::c_int) -> core::ffi::c_int;
    pub fn snd_usb_mixer_disconnect(mixer: *mut usb_mixer_interface);
    pub fn snd_usb_mixer_notify_id(mixer: *mut usb_mixer_interface, unitid: core::ffi::c_int);
    pub fn snd_usb_mixer_set_ctl_value(cval: *mut usb_mixer_elem_info, request: core::ffi::c_int, validx: core::ffi::c_int, value_set: core::ffi::c_int) -> core::ffi::c_int;
    pub fn snd_usb_mixer_add_list(list: *mut usb_mixer_elem_list, kctl: *mut snd_kcontrol, is_std_info: bool) -> core::ffi::c_int;
    pub fn snd_usb_mixer_elem_init_std(list: *mut usb_mixer_elem_list, mixer: *mut usb_mixer_interface, unitid: core::ffi::c_int);
    pub fn snd_usb_mixer_vol_tlv(kcontrol: *mut snd_kcontrol, op_flag: core::ffi::c_int, size: core::ffi::c_uint, _tlv: *mut core::ffi::c_uint) -> core::ffi::c_int;
    pub fn snd_usb_mixer_suspend(mixer: *mut usb_mixer_interface) -> core::ffi::c_int;
    pub fn snd_usb_mixer_resume(mixer: *mut usb_mixer_interface) -> core::ffi::c_int;
    pub fn snd_usb_set_cur_mix_value(cval: *mut usb_mixer_elem_info, channel: core::ffi::c_int, index: core::ffi::c_int, value: core::ffi::c_int) -> core::ffi::c_int;
    pub fn snd_usb_get_cur_mix_value(cval: *mut usb_mixer_elem_info, channel: core::ffi::c_int, index: core::ffi::c_int, value: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn snd_usb_mixer_elem_free(kctl: *mut snd_kcontrol);
    pub static snd_usb_feature_unit_ctl: *const snd_kcontrol_new;
}

// C macro: snd_usb_mixer_add_control(list, kctl)
// expands to: snd_usb_mixer_add_list(list, kctl, true)
#[inline]
pub fn snd_usb_mixer_add_control(list: *mut usb_mixer_elem_list, kctl: *mut snd_kcontrol) -> core::ffi::c_int {
    unsafe { snd_usb_mixer_add_list(list, kctl, true) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
