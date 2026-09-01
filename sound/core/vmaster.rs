// SPDX-License-Identifier: GPL-2.0-only
/*
 * Virtual master and follower controls
 *
 *  Copyright (c) 2008 by Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

/* Dependencies originally included from:
 * <linux/slab.h>, <linux/export.h>, <sound/core.h>, <sound/control.h>,
 * and <sound/tlv.h>.
 */

pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;

pub const SNDRV_CTL_ELEM_TYPE_BOOLEAN: snd_ctl_elem_type_t = 1;
pub const SNDRV_CTL_ELEM_TYPE_INTEGER: snd_ctl_elem_type_t = 2;
pub const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 1 << 6;
pub const SNDRV_CTL_TLVO_TYPE: usize = 0;
pub const SNDRV_CTL_TLVT_DB_SCALE: c_uint = 0x0001;
pub const SNDRV_CTL_TLVT_DB_MINMAX: c_uint = 0x0004;
pub const SNDRV_CTL_TLVT_DB_MINMAX_MUTE: c_uint = 0x0005;
pub const SND_CTL_FOLLOWER_NEED_UPDATE: c_uint = 1 << 0;

#[allow(non_camel_case_types)]
pub type snd_ctl_elem_type_t = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_id {
    pub _bindgen_opaque_blob: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_int,
    pub max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info {
    pub id: snd_ctl_elem_id,
    pub type_: snd_ctl_elem_type_t,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
    pub c: Option<
        unsafe extern "C" fn(
            kcontrol: *mut snd_kcontrol,
            op_flag: c_int,
            size: c_uint,
            tlv: *mut c_uint,
        ) -> c_int,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol {
    pub list: list_head,
    pub id: snd_ctl_elem_id,
    pub count: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub vd: *mut snd_kcontrol_volatile,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *mut c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_add_follower(master: *mut snd_kcontrol, follower: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *mut snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_entry<T>(ptr: *mut list_head, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}

const LINK_FOLLOWER_LIST_OFFSET: usize = 0;

unsafe fn list_for_each_follower<F>(head: *mut list_head, mut f: F) -> c_int
where
    F: FnMut(*mut link_follower) -> c_int,
{
    let mut pos = (*head).next;
    while pos != head {
        let follower = list_entry::<link_follower>(pos, LINK_FOLLOWER_LIST_OFFSET);
        let ret = f(follower);
        if ret < 0 {
            return ret;
        }
        pos = (*pos).next;
    }
    0
}

/*
 * a subset of information returned via ctl info callback
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_ctl_info {
    pub type_: snd_ctl_elem_type_t, /* value type */
    pub count: c_int,              /* item count */
    pub min_val: c_int,
    pub max_val: c_int, /* min, max values */
}

/*
 * link master - this contains a list of follower controls that are
 * identical types, i.e. info returns the same value type and value
 * ranges, but may have different number of counts.
 *
 * The master control is so far only mono volume/switch for simplicity.
 * The same value will be applied to all followers.
 */
#[repr(C)]
pub struct link_master {
    pub followers: list_head,
    pub info: link_ctl_info,
    pub val: c_int, /* the master value */
    pub tlv: [c_uint; 4],
    pub hook: Option<unsafe extern "C" fn(private_data: *mut c_void, val: c_int)>,
    pub hook_private_data: *mut c_void,
}

/*
 * link follower - this contains a follower control element
 *
 * It fakes the control callbacks with additional attenuation by the
 * master control.  A follower may have either one or two channels.
 */
#[repr(C)]
pub struct link_follower {
    pub list: list_head,
    pub master: *mut link_master,
    pub info: link_ctl_info,
    pub vals: [c_int; 2], /* current values */
    pub flags: c_uint,
    pub kctl: *mut snd_kcontrol,       /* original kcontrol pointer */
    pub follower: snd_kcontrol, /* the copy of original control entry */
}

unsafe fn follower_update(follower: *mut link_follower) -> c_int {
    let mut err: c_int;
    let mut ch: c_int;
    let uctl = kzalloc(mem::size_of::<snd_ctl_elem_value>(), 0) as *mut snd_ctl_elem_value;

    if uctl.is_null() {
        return -ENOMEM;
    }
    (*uctl).id = (*follower).follower.id;
    err = ((*follower).follower.get.unwrap())(&mut (*follower).follower, uctl);
    if err < 0 {
        kfree(uctl as *const c_void);
        return err;
    }
    ch = 0;
    while ch < (*follower).info.count {
        (*follower).vals[ch as usize] = (*uctl).value.integer.value[ch as usize];
        ch += 1;
    }
    kfree(uctl as *const c_void);
    0
}

/* get the follower ctl info and save the initial values */
unsafe fn follower_init(follower: *mut link_follower) -> c_int {
    let err: c_int;

    if (*follower).info.count != 0 {
        /* already initialized */
        if ((*follower).flags & SND_CTL_FOLLOWER_NEED_UPDATE) != 0 {
            return follower_update(follower);
        }
        return 0;
    }

    let uinfo = kmalloc(mem::size_of::<snd_ctl_elem_info>(), 0) as *mut snd_ctl_elem_info;
    if uinfo.is_null() {
        return -ENOMEM;
    }
    (*uinfo).id = (*follower).follower.id;
    err = ((*follower).follower.info.unwrap())(&mut (*follower).follower, uinfo);
    if err < 0 {
        kfree(uinfo as *const c_void);
        return err;
    }
    (*follower).info.type_ = (*uinfo).type_;
    (*follower).info.count = (*uinfo).count as c_int;
    if (*follower).info.count > 2
        || ((*follower).info.type_ != SNDRV_CTL_ELEM_TYPE_INTEGER
            && (*follower).info.type_ != SNDRV_CTL_ELEM_TYPE_BOOLEAN)
    {
        pr_err(c"ALSA: vmaster: invalid follower element\n".as_ptr());
        kfree(uinfo as *const c_void);
        return -EINVAL;
    }
    (*follower).info.min_val = (*uinfo).value.integer.min;
    (*follower).info.max_val = (*uinfo).value.integer.max;
    kfree(uinfo as *const c_void);

    follower_update(follower)
}

/* initialize master volume */
unsafe fn master_init(master: *mut link_master) -> c_int {
    if (*master).info.count != 0 {
        return 0; /* already initialized */
    }

    let ret = list_for_each_follower(&mut (*master).followers, |follower| {
        let err = follower_init(follower);
        if err < 0 {
            return err;
        }
        (*master).info = (*follower).info;
        (*master).info.count = 1; /* always mono */
        /* set full volume as default (= no attenuation) */
        (*master).val = (*master).info.max_val;
        if let Some(hook) = (*master).hook {
            hook((*master).hook_private_data, (*master).val);
        }
        1
    });
    if ret != 0 {
        return ret;
    }
    -ENOENT
}

unsafe fn follower_get_val(
    follower: *mut link_follower,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mut ch: c_int;

    let err = follower_init(follower);
    if err < 0 {
        return err;
    }
    ch = 0;
    while ch < (*follower).info.count {
        (*ucontrol).value.integer.value[ch as usize] = (*follower).vals[ch as usize];
        ch += 1;
    }
    0
}

unsafe fn follower_put_val(
    follower: *mut link_follower,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mut ch: c_int;
    let mut vol: c_int;

    let err = master_init((*follower).master);
    if err < 0 {
        return err;
    }

    match (*follower).info.type_ {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN => {
            ch = 0;
            while ch < (*follower).info.count {
                (*ucontrol).value.integer.value[ch as usize] &=
                    ((*(*follower).master).val != 0) as c_int;
                ch += 1;
            }
        }
        SNDRV_CTL_ELEM_TYPE_INTEGER => {
            ch = 0;
            while ch < (*follower).info.count {
                /* max master volume is supposed to be 0 dB */
                vol = (*ucontrol).value.integer.value[ch as usize];
                vol += (*(*follower).master).val - (*(*follower).master).info.max_val;
                if vol < (*follower).info.min_val {
                    vol = (*follower).info.min_val;
                } else if vol > (*follower).info.max_val {
                    vol = (*follower).info.max_val;
                }
                (*ucontrol).value.integer.value[ch as usize] = vol;
                ch += 1;
            }
        }
        _ => {}
    }
    ((*follower).follower.put.unwrap())(&mut (*follower).follower, ucontrol)
}

/*
 * ctl callbacks for followers
 */
unsafe extern "C" fn follower_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let follower = snd_kcontrol_chip(kcontrol) as *mut link_follower;
    ((*follower).follower.info.unwrap())(&mut (*follower).follower, uinfo)
}

unsafe extern "C" fn follower_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let follower = snd_kcontrol_chip(kcontrol) as *mut link_follower;
    follower_get_val(follower, ucontrol)
}

unsafe extern "C" fn follower_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let follower = snd_kcontrol_chip(kcontrol) as *mut link_follower;
    let mut changed: c_int = 0;

    let err = follower_init(follower);
    if err < 0 {
        return err;
    }
    let mut ch: c_int = 0;
    while ch < (*follower).info.count {
        if (*ucontrol).value.integer.value[ch as usize] < (*follower).info.min_val
            || (*ucontrol).value.integer.value[ch as usize] > (*follower).info.max_val
        {
            return -EINVAL;
        }
        ch += 1;
    }

    ch = 0;
    while ch < (*follower).info.count {
        if (*follower).vals[ch as usize] != (*ucontrol).value.integer.value[ch as usize] {
            changed = 1;
            (*follower).vals[ch as usize] = (*ucontrol).value.integer.value[ch as usize];
        }
        ch += 1;
    }
    if changed == 0 {
        return 0;
    }
    let err = follower_put_val(follower, ucontrol);
    if err < 0 {
        return err;
    }
    1
}

unsafe extern "C" fn follower_tlv_cmd(
    kcontrol: *mut snd_kcontrol,
    op_flag: c_int,
    size: c_uint,
    tlv: *mut c_uint,
) -> c_int {
    let follower = snd_kcontrol_chip(kcontrol) as *mut link_follower;
    /* FIXME: this assumes that the max volume is 0 dB */
    ((*follower).follower.tlv.c.unwrap())(&mut (*follower).follower, op_flag, size, tlv)
}

unsafe extern "C" fn follower_free(kcontrol: *mut snd_kcontrol) {
    let follower = snd_kcontrol_chip(kcontrol) as *mut link_follower;
    if let Some(private_free) = (*follower).follower.private_free {
        private_free(&mut (*follower).follower);
    }
    if !(*follower).master.is_null() {
        list_del(&mut (*follower).list);
    }
    kfree(follower as *const c_void);
}

/*
 * Add a follower control to the group with the given master control
 *
 * All followers must be the same type (returning the same information
 * via info callback).  The function doesn't check it, so it's your
 * responsibility.
 *
 * Also, some additional limitations:
 * - at most two channels
 * - logarithmic volume control (dB level), no linear volume
 * - master can only attenuate the volume, no gain
 */
#[no_mangle]
pub unsafe extern "C" fn _snd_ctl_add_follower(
    master: *mut snd_kcontrol,
    follower: *mut snd_kcontrol,
    flags: c_uint,
) -> c_int {
    let master_link = snd_kcontrol_chip(master) as *mut link_master;
    let size = mem::size_of::<link_follower>()
        + ((*follower).count as usize * mem::size_of::<snd_kcontrol_volatile>());
    let srec = kzalloc(size, 0) as *mut link_follower;
    if srec.is_null() {
        return -ENOMEM;
    }
    (*srec).kctl = follower;
    ptr::copy_nonoverlapping(follower, &mut (*srec).follower, 1);
    (*srec).follower.vd = (srec as *mut u8).add(mem::size_of::<link_follower>())
        as *mut snd_kcontrol_volatile;
    memcpy(
        (*srec).follower.vd as *mut c_void,
        (*follower).vd as *const c_void,
        (*follower).count as usize * mem::size_of::<snd_kcontrol_volatile>(),
    );
    (*srec).master = master_link;
    (*srec).flags = flags;

    /* override callbacks */
    (*follower).info = Some(follower_info);
    (*follower).get = Some(follower_get);
    (*follower).put = Some(follower_put);
    if ((*(*follower).vd).access & SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK) != 0 {
        (*follower).tlv.c = Some(follower_tlv_cmd);
    }
    (*follower).private_data = srec as *mut c_void;
    (*follower).private_free = Some(follower_free);

    list_add_tail(&mut (*srec).list, &mut (*master_link).followers);
    0
}

/**
 * snd_ctl_add_followers - add multiple followers to vmaster
 * @card: card instance
 * @master: the target vmaster kcontrol object
 * @list: NULL-terminated list of name strings of followers to be added
 *
 * Adds the multiple follower kcontrols with the given names.
 * Returns 0 for success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_add_followers(
    card: *mut snd_card,
    master: *mut snd_kcontrol,
    mut list: *const *const c_char,
) -> c_int {
    while !(*list).is_null() {
        let follower = snd_ctl_find_id_mixer(card, *list);
        if !follower.is_null() {
            let err = snd_ctl_add_follower(master, follower);
            if err < 0 {
                return err;
            }
        }
        list = list.add(1);
    }

    0
}

/*
 * ctl callbacks for master controls
 */
unsafe extern "C" fn master_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;

    let ret = master_init(master);
    if ret < 0 {
        return ret;
    }
    (*uinfo).type_ = (*master).info.type_;
    (*uinfo).count = (*master).info.count as c_uint;
    (*uinfo).value.integer.min = (*master).info.min_val;
    (*uinfo).value.integer.max = (*master).info.max_val;
    0
}

unsafe extern "C" fn master_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;
    let err = master_init(master);
    if err < 0 {
        return err;
    }
    (*ucontrol).value.integer.value[0] = (*master).val;
    0
}

unsafe fn sync_followers(master: *mut link_master, old_val: c_int, new_val: c_int) -> c_int {
    let uval = kmalloc(mem::size_of::<snd_ctl_elem_value>(), 0) as *mut snd_ctl_elem_value;

    if uval.is_null() {
        return -ENOMEM;
    }
    let ret = list_for_each_follower(&mut (*master).followers, |follower| {
        (*master).val = old_val;
        (*uval).id = (*follower).follower.id;
        follower_get_val(follower, uval);
        (*master).val = new_val;
        follower_put_val(follower, uval)
    });
    kfree(uval as *const c_void);
    ret
}

unsafe extern "C" fn master_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;
    let err = master_init(master);
    if err < 0 {
        return err;
    }
    let first_init = err != 0;
    let old_val = (*master).val;
    let new_val = (*ucontrol).value.integer.value[0];
    if new_val == old_val {
        return 0;
    }
    if new_val < (*master).info.min_val || new_val > (*master).info.max_val {
        return -EINVAL;
    }

    let err = sync_followers(master, old_val, new_val);
    if err < 0 {
        return err;
    }
    if let Some(hook) = (*master).hook {
        if !first_init {
            hook((*master).hook_private_data, (*master).val);
        }
    }
    1
}

unsafe extern "C" fn master_free(kcontrol: *mut snd_kcontrol) {
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;

    /* free all follower links and retore the original follower kctls */
    let head = &mut (*master).followers as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let follower = list_entry::<link_follower>(pos, LINK_FOLLOWER_LIST_OFFSET);
        let sctl = (*follower).kctl;
        let olist = (*sctl).list;
        memcpy(
            sctl as *mut c_void,
            &mut (*follower).follower as *mut snd_kcontrol as *const c_void,
            mem::size_of::<snd_kcontrol>(),
        );
        memcpy(
            (*sctl).vd as *mut c_void,
            (*follower).follower.vd as *const c_void,
            (*sctl).count as usize * mem::size_of::<snd_kcontrol_volatile>(),
        );
        (*sctl).list = olist; /* keep the current linked-list */
        kfree(follower as *const c_void);
        pos = next;
    }
    kfree(master as *const c_void);
}

/**
 * snd_ctl_make_virtual_master - Create a virtual master control
 * @name: name string of the control element to create
 * @tlv: optional TLV int array for dB information
 *
 * Creates a virtual master control with the given name string.
 *
 * After creating a vmaster element, you can add the follower controls
 * via snd_ctl_add_follower() or snd_ctl_add_follower_uncached().
 *
 * The optional argument @tlv can be used to specify the TLV information
 * for dB scale of the master control.  It should be a single element
 * with #SNDRV_CTL_TLVT_DB_SCALE, #SNDRV_CTL_TLV_DB_MINMAX or
 * #SNDRV_CTL_TLVT_DB_MINMAX_MUTE type, and should be the max 0dB.
 *
 * Return: The created control element, or %NULL for errors (ENOMEM).
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_make_virtual_master(
    name: *mut c_char,
    tlv: *const c_uint,
) -> *mut snd_kcontrol {
    let mut knew: snd_kcontrol_new = mem::zeroed();

    memset(
        &mut knew as *mut snd_kcontrol_new as *mut c_void,
        0,
        mem::size_of::<snd_kcontrol_new>(),
    );
    knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    knew.name = name;
    knew.info = Some(master_info);

    let master = kzalloc(mem::size_of::<link_master>(), 0) as *mut link_master;
    if master.is_null() {
        return ptr::null_mut();
    }
    init_list_head(&mut (*master).followers);

    let kctl = snd_ctl_new1(&mut knew, master as *mut c_void);
    if kctl.is_null() {
        kfree(master as *const c_void);
        return ptr::null_mut();
    }
    /* override some callbacks */
    (*kctl).info = Some(master_info);
    (*kctl).get = Some(master_get);
    (*kctl).put = Some(master_put);
    (*kctl).private_free = Some(master_free);

    /* additional (constant) TLV read */
    if !tlv.is_null() {
        let type_ = *tlv.add(SNDRV_CTL_TLVO_TYPE);
        if type_ == SNDRV_CTL_TLVT_DB_SCALE
            || type_ == SNDRV_CTL_TLVT_DB_MINMAX
            || type_ == SNDRV_CTL_TLVT_DB_MINMAX_MUTE
        {
            (*(*kctl).vd).access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
            memcpy(
                (*master).tlv.as_mut_ptr() as *mut c_void,
                tlv as *const c_void,
                mem::size_of_val(&(*master).tlv),
            );
            (*kctl).tlv.p = (*master).tlv.as_ptr();
        }
    }

    kctl
}

/**
 * snd_ctl_add_vmaster_hook - Add a hook to a vmaster control
 * @kcontrol: vmaster kctl element
 * @hook: the hook function
 * @private_data: the private_data pointer to be saved
 *
 * Adds the given hook to the vmaster control element so that it's called
 * at each time when the value is changed.
 *
 * Return: Zero.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_add_vmaster_hook(
    kcontrol: *mut snd_kcontrol,
    hook: Option<unsafe extern "C" fn(private_data: *mut c_void, val: c_int)>,
    private_data: *mut c_void,
) -> c_int {
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;
    (*master).hook = hook;
    (*master).hook_private_data = private_data;
    0
}

/**
 * snd_ctl_sync_vmaster - Sync the vmaster followers and hook
 * @kcontrol: vmaster kctl element
 * @hook_only: sync only the hook
 *
 * Forcibly call the put callback of each follower and call the hook function
 * to synchronize with the current value of the given vmaster element.
 * NOP when NULL is passed to @kcontrol.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_sync_vmaster(kcontrol: *mut snd_kcontrol, hook_only: bool) {
    let mut first_init = false;

    if kcontrol.is_null() {
        return;
    }
    let master = snd_kcontrol_chip(kcontrol) as *mut link_master;
    if !hook_only {
        let mut err = master_init(master);
        if err < 0 {
            return;
        }
        first_init = err != 0;
        err = sync_followers(master, (*master).val, (*master).val);
        if err < 0 {
            return;
        }
    }

    if let Some(hook) = (*master).hook {
        if !first_init {
            hook((*master).hook_private_data, (*master).val);
        }
    }
}

/**
 * snd_ctl_apply_vmaster_followers - Apply function to each vmaster follower
 * @kctl: vmaster kctl element
 * @func: function to apply
 * @arg: optional function argument
 *
 * Apply the function @func to each follower kctl of the given vmaster kctl.
 *
 * Return: 0 if successful, or a negative error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_apply_vmaster_followers(
    kctl: *mut snd_kcontrol,
    func: Option<
        unsafe extern "C" fn(
            vfollower: *mut snd_kcontrol,
            follower: *mut snd_kcontrol,
            arg: *mut c_void,
        ) -> c_int,
    >,
    arg: *mut c_void,
) -> c_int {
    let master = snd_kcontrol_chip(kctl) as *mut link_master;
    let err = master_init(master);
    if err < 0 {
        return err;
    }
    list_for_each_follower(&mut (*master).followers, |follower| {
        func.unwrap()((*follower).kctl, &mut (*follower).follower, arg)
    })
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
