/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from sound/control.h. */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct snd_kcontrol;
#[repr(C)]
pub struct snd_ctl_elem_info;
#[repr(C)]
pub struct snd_ctl_elem_value;
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub iface: snd_ctl_elem_iface_t,
    pub device: u32,
    pub subdevice: u32,
    pub name: [::std::os::raw::c_char; 44],
    pub index: u32,
    pub numid: u32,
}
pub type snd_ctl_elem_iface_t = u32;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct snd_ctl_file;
#[repr(C)] pub struct snd_card;
#[repr(C)] pub struct snd_fasync;
#[repr(C)] pub struct pid;
#[repr(C)] pub struct rw_semaphore;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct spinlock_t;

pub type snd_kcontrol_info_t = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32;
pub type snd_kcontrol_get_t = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32;
pub type snd_kcontrol_put_t = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32;
pub type snd_kcontrol_tlv_rw_t = unsafe extern "C" fn(*mut snd_kcontrol, i32, u32, *mut u32) -> i32;

pub const SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK: u32 = 1 << 24;
pub const SNDRV_CTL_ELEM_ACCESS_LED_SHIFT: u32 = 25;
pub const SNDRV_CTL_ELEM_ACCESS_LED_MASK: u32 = 7 << 25;
pub const SNDRV_CTL_ELEM_ACCESS_SPK_LED: u32 = 1 << 25;
pub const SNDRV_CTL_ELEM_ACCESS_MIC_LED: u32 = 2 << 25;

#[inline]
pub unsafe fn snd_ctl_skip_validation(info: *const snd_ctl_elem_info) -> bool {
    /* CONFIG_SND_CTL_DEBUG selects the access-bit implementation. */
    (info as *const u8) as usize != 0
}

pub const SNDRV_CTL_TLV_OP_READ: i32 = 0;
pub const SNDRV_CTL_TLV_OP_WRITE: i32 = 1;
pub const SNDRV_CTL_TLV_OP_CMD: i32 = -1;

#[repr(C)]
pub union snd_kcontrol_new_tlv { pub c: Option<snd_kcontrol_tlv_rw_t>, pub p: *const u32 }
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: snd_ctl_elem_iface_t, pub device: u32, pub subdevice: u32,
    pub name: *const ::std::os::raw::c_char, pub index: u32, pub access: u32,
    pub count: u32, pub info: Option<snd_kcontrol_info_t>, pub get: Option<snd_kcontrol_get_t>,
    pub put: Option<snd_kcontrol_put_t>, pub tlv: snd_kcontrol_new_tlv, pub private_value: usize,
}

#[repr(C)] pub struct snd_kcontrol_volatile { pub owner: *mut snd_ctl_file, pub access: u32 }
#[repr(C)]
pub struct snd_kcontrol {
    pub list: list_head, pub id: snd_ctl_elem_id, pub count: u32,
    pub info: Option<snd_kcontrol_info_t>, pub get: Option<snd_kcontrol_get_t>, pub put: Option<snd_kcontrol_put_t>,
    pub tlv: snd_kcontrol_new_tlv, pub private_value: usize, pub private_data: *mut ::std::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub vd: [snd_kcontrol_volatile; 0],
}

#[repr(C)] pub struct snd_kctl_event { pub list: list_head, pub id: snd_ctl_elem_id, pub mask: u32 }
pub const SND_CTL_SUBDEV_PCM: u32 = 0;
pub const SND_CTL_SUBDEV_RAWMIDI: u32 = 1;
pub const SND_CTL_SUBDEV_ITEMS: usize = 2;
#[repr(C)]
pub struct snd_ctl_file {
    pub list: list_head, pub card: *mut snd_card, pub pid: *mut pid,
    pub preferred_subdevice: [i32; SND_CTL_SUBDEV_ITEMS], pub change_sleep: wait_queue_head_t,
    pub read_lock: spinlock_t, pub fasync: *mut snd_fasync, pub subscribed: i32, pub events: list_head,
}
#[repr(C)]
pub struct snd_ctl_layer_ops {
    pub next: *mut snd_ctl_layer_ops, pub module_name: *const ::std::os::raw::c_char,
    pub lregister: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub ldisconnect: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub lnotify: Option<unsafe extern "C" fn(*mut snd_card, u32, *mut snd_kcontrol, u32)>,
}
pub type snd_kctl_ioctl_func_t = unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, u32, usize) -> i32;

extern "C" {
    pub fn snd_ctl_notify(card: *mut snd_card, mask: u32, id: *mut snd_ctl_elem_id);
    pub fn snd_ctl_notify_one(card: *mut snd_card, mask: u32, kctl: *mut snd_kcontrol, ioff: u32);
    pub fn snd_ctl_new1(kcontrolnew: *const snd_kcontrol_new, private_data: *mut ::std::ffi::c_void) -> *mut snd_kcontrol;
    pub fn snd_ctl_free_one(kcontrol: *mut snd_kcontrol);
    pub fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    pub fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    pub fn snd_ctl_replace(card: *mut snd_card, kcontrol: *mut snd_kcontrol, add_on_replace: bool) -> i32;
    pub fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> i32;
    pub fn snd_ctl_rename_id(card: *mut snd_card, src_id: *mut snd_ctl_elem_id, dst_id: *mut snd_ctl_elem_id) -> i32;
    pub fn snd_ctl_rename(card: *mut snd_card, kctl: *mut snd_kcontrol, name: *const ::std::os::raw::c_char);
    pub fn snd_ctl_activate_id(card: *mut snd_card, id: *mut snd_ctl_elem_id, active: i32) -> i32;
    pub fn snd_ctl_find_numid(card: *mut snd_card, numid: u32) -> *mut snd_kcontrol;
    pub fn snd_ctl_find_id(card: *mut snd_card, id: *const snd_ctl_elem_id) -> *mut snd_kcontrol;
    pub fn snd_ctl_create(card: *mut snd_card) -> i32;
    pub static mut snd_ioctl_rwsem: rw_semaphore;
    pub fn snd_ctl_register_ioctl(fcn: snd_kctl_ioctl_func_t) -> i32;
    pub fn snd_ctl_unregister_ioctl(fcn: snd_kctl_ioctl_func_t) -> i32;
    pub fn snd_ctl_request_layer(module_name: *const ::std::os::raw::c_char) -> i32;
    pub fn snd_ctl_register_layer(lops: *mut snd_ctl_layer_ops);
    pub fn snd_ctl_disconnect_layer(lops: *mut snd_ctl_layer_ops);
    pub fn snd_ctl_get_preferred_subdevice(card: *mut snd_card, ty: i32) -> i32;
    pub fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
    pub fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
    pub fn snd_ctl_enum_info(info: *mut snd_ctl_elem_info, channels: u32, items: u32, names: *const *const ::std::os::raw::c_char) -> i32;
    pub fn snd_ctl_make_virtual_master(name: *mut ::std::os::raw::c_char, tlv: *const u32) -> *mut snd_kcontrol;
    pub fn _snd_ctl_add_follower(master: *mut snd_kcontrol, follower: *mut snd_kcontrol, flags: u32) -> i32;
    pub fn snd_ctl_add_followers(card: *mut snd_card, master: *mut snd_kcontrol, list: *const *const ::std::os::raw::c_char) -> i32;
    pub fn snd_ctl_add_vmaster_hook(kctl: *mut snd_kcontrol, hook: Option<unsafe extern "C" fn(*mut ::std::ffi::c_void, i32)>, private_data: *mut ::std::ffi::c_void) -> i32;
    pub fn snd_ctl_sync_vmaster(kctl: *mut snd_kcontrol, hook_only: bool);
    pub fn snd_ctl_apply_vmaster_followers(kctl: *mut snd_kcontrol, func: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_kcontrol, *mut ::std::ffi::c_void) -> i32>, arg: *mut ::std::ffi::c_void) -> i32;
    pub fn snd_kctl_jack_new(name: *const ::std::os::raw::c_char, card: *mut snd_card) -> *mut snd_kcontrol;
    pub fn snd_kctl_jack_report(card: *mut snd_card, kctl: *mut snd_kcontrol, status: bool);
}

#[inline]
pub unsafe fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const ::std::os::raw::c_char) -> *mut snd_kcontrol {
    let mut id: snd_ctl_elem_id = ::std::mem::zeroed();
    /* SNDRV_CTL_ELEM_IFACE_MIXER and strscpy are supplied by asound/kernel headers. */
    id.iface = 2;
    let mut p = id.name.as_mut_ptr();
    let mut s = name;
    while !s.is_null() && *s != 0 && (p as usize - id.name.as_ptr() as usize) < id.name.len() - 1 {
        *p = *s; p = p.add(1); s = s.add(1);
    }
    *p = 0;
    snd_ctl_find_id(card, &id)
}

extern "C" { pub fn array_index_nospec(index: u32, size: u32) -> u32; }
#[inline] pub unsafe fn snd_ctl_get_ioffnum(kctl: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> u32 {
    array_index_nospec((*id).numid.wrapping_sub((*kctl).id.numid), (*kctl).count)
}
#[inline] pub unsafe fn snd_ctl_get_ioffidx(kctl: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> u32 {
    array_index_nospec((*id).index.wrapping_sub((*kctl).id.index), (*kctl).count)
}
#[inline] pub unsafe fn snd_ctl_get_ioff(kctl: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> u32 {
    if (*id).numid != 0 { snd_ctl_get_ioffnum(kctl, id) } else { snd_ctl_get_ioffidx(kctl, id) }
}
#[inline] pub unsafe fn snd_ctl_build_ioff(dst_id: *mut snd_ctl_elem_id, src_kctl: *mut snd_kcontrol, offset: u32) -> *mut snd_ctl_elem_id {
    *dst_id = (*src_kctl).id;
    (*dst_id).index = (*dst_id).index.wrapping_add(offset);
    (*dst_id).numid = (*dst_id).numid.wrapping_add(offset);
    dst_id
}

pub const SND_CTL_FOLLOWER_NEED_UPDATE: u32 = 1 << 0;
pub const SND_CTL_LAYER_MODULE_LED: &[u8] = b"snd-ctl-led\0";

#[inline] pub unsafe fn snd_ctl_add_follower(master: *mut snd_kcontrol, follower: *mut snd_kcontrol) -> i32 { _snd_ctl_add_follower(master, follower, 0) }
#[inline] pub unsafe fn snd_ctl_add_follower_uncached(master: *mut snd_kcontrol, follower: *mut snd_kcontrol) -> i32 { _snd_ctl_add_follower(master, follower, SND_CTL_FOLLOWER_NEED_UPDATE) }
#[inline] pub unsafe fn snd_ctl_sync_vmaster_hook(kctl: *mut snd_kcontrol) { snd_ctl_sync_vmaster(kctl, true) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
