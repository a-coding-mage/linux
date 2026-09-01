// SPDX-License-Identifier: GPL-2.0
/*
 * Vortex Mixer support.
 *
 * There is much more than just the AC97 mixer...
 *
 */

// C dependencies: <linux/time.h>, <linux/init.h>, <sound/core.h>, "au88x0.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97 {
    pub ext_id: c_uint,
}

#[repr(C)]
pub struct vortex_t {
    pub card: *mut snd_card,
    pub codec: *mut snd_ac97,
    pub isquad: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub scaps: c_uint,
}

pub type snd_ac97_bus_write_t =
    unsafe extern "C" fn(*mut snd_ac97, c_uchar, c_ushort);
pub type snd_ac97_bus_read_t = unsafe extern "C" fn(*mut snd_ac97, c_uchar) -> c_ushort;

use core::ffi::{c_uchar, c_ushort};

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<snd_ac97_bus_write_t>,
    pub read: Option<snd_ac97_bus_read_t>,
}

pub const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
pub const AC97_SCAP_NO_SPDIF: c_uint = 1 << 10;

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const snd_ac97_bus_ops,
        private_data: *mut c_void,
        rbus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        rac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn vortex_codec_write(ac97: *mut snd_ac97, reg: c_uchar, val: c_ushort);
    fn vortex_codec_read(ac97: *mut snd_ac97, reg: c_uchar) -> c_ushort;
}

unsafe fn remove_ctl(card: *mut snd_card, name: *const c_char) -> c_int {
    let mut id: snd_ctl_elem_id = core::mem::zeroed();
    memset(
        &mut id as *mut snd_ctl_elem_id as *mut c_void,
        0,
        core::mem::size_of::<snd_ctl_elem_id>(),
    );
    strscpy(id.name.as_mut_ptr(), name);
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    snd_ctl_remove_id(card, &mut id)
}

unsafe fn snd_vortex_mixer(vortex: *mut vortex_t) -> c_int {
    let mut pbus: *mut snd_ac97_bus = core::ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut err: c_int;
    static OPS: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(vortex_codec_write),
        read: Some(vortex_codec_read),
    };

    err = snd_ac97_bus(
        (*vortex).card,
        0,
        &OPS,
        core::ptr::null_mut(),
        &mut pbus,
    );
    if err < 0 {
        return err;
    }
    memset(
        &mut ac97 as *mut snd_ac97_template as *mut c_void,
        0,
        core::mem::size_of::<snd_ac97_template>(),
    );
    // Initialize AC97 codec stuff.
    ac97.private_data = vortex as *mut c_void;
    ac97.scaps = AC97_SCAP_NO_SPDIF;
    err = snd_ac97_mixer(pbus, &mut ac97, &mut (*vortex).codec);
    (*vortex).isquad = if (*vortex).codec.is_null() {
        0
    } else {
        (*(*vortex).codec).ext_id & 0x80
    };
    remove_ctl(
        (*vortex).card,
        b"Master Mono Playback Volume\0".as_ptr() as *const c_char,
    );
    remove_ctl(
        (*vortex).card,
        b"Master Mono Playback Switch\0".as_ptr() as *const c_char,
    );
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
