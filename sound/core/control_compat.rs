// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * compat ioctls for control API
 *
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* this file included from control.c */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;

type u32 = u32;
type s32 = i32;
type u64 = u64;
type s64 = i64;
type compat_caddr_t = u32;
type snd_ctl_elem_type_t = c_int;

const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const ENOIOCTLCMD: c_int = 515;

const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_int = 3;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_int = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 5;
const SNDRV_CTL_ELEM_TYPE_INTEGER64: c_int = 6;

extern "C" {
    static mut snd_ioctl_rwsem: c_void;
    static mut snd_control_compat_ioctls: list_head;

    static SNDRV_CTL_IOCTL_PVERSION: c_uint;
    static SNDRV_CTL_IOCTL_CARD_INFO: c_uint;
    static SNDRV_CTL_IOCTL_CARD_BYTES: c_uint;
    static SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS: c_uint;
    static SNDRV_CTL_IOCTL_POWER: c_uint;
    static SNDRV_CTL_IOCTL_POWER_STATE: c_uint;
    static SNDRV_CTL_IOCTL_ELEM_LOCK: c_uint;
    static SNDRV_CTL_IOCTL_ELEM_UNLOCK: c_uint;
    static SNDRV_CTL_IOCTL_ELEM_REMOVE: c_uint;
    static SNDRV_CTL_IOCTL_TLV_READ: c_uint;
    static SNDRV_CTL_IOCTL_TLV_WRITE: c_uint;
    static SNDRV_CTL_IOCTL_TLV_COMMAND: c_uint;

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn compat_ptr(ptr: c_ulong) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_ctl_elem_list(card: *mut snd_card, data: *mut snd_ctl_elem_list) -> c_int;
    fn snd_power_ref_and_wait(card: *mut snd_card) -> c_int;
    fn snd_power_unref(card: *mut snd_card);
    fn snd_ctl_elem_info(ctl: *mut snd_ctl_file, data: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_find_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> *mut snd_kcontrol;
    fn snd_ctl_elem_read(card: *mut snd_card, data: *mut snd_ctl_elem_value) -> c_int;
    fn snd_ctl_elem_write(
        card: *mut snd_card,
        file: *mut snd_ctl_file,
        data: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_ctl_elem_add(
        file: *mut snd_ctl_file,
        data: *mut snd_ctl_elem_info,
        replace: c_int,
    ) -> c_int;
    fn snd_ctl_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn rwsem_read_guard(sem: *mut c_void);
    fn rwsem_read_unguard(sem: *mut c_void);
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct file {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_card {
    dev: *mut c_void,
    controls_rwsem: c_void,
}

#[repr(C)]
struct snd_ctl_file {
    card: *mut snd_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_id {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_list {
    offset: u32,
    space: u32,
    used: u32,
    count: u32,
    pids: *mut c_void,
}

#[repr(C)]
struct snd_ctl_elem_list32 {
    offset: u32,
    space: u32,
    used: u32,
    count: u32,
    pids: u32,
    reserved: [u8; 50],
} /* don't set packed attribute here */

unsafe fn get_user<T: Copy>(dst: *mut T, src: *const T) -> c_int {
    copy_from_user(dst.cast(), src.cast(), size_of::<T>()) as c_int
}

unsafe fn put_user<T: Copy>(src: T, dst: *mut T) -> c_int {
    copy_to_user(dst.cast(), (&src as *const T).cast(), size_of::<T>()) as c_int
}

unsafe fn snd_ctl_elem_list_compat(
    card: *mut snd_card,
    data32: *mut snd_ctl_elem_list32,
) -> c_int {
    let mut data: snd_ctl_elem_list = core::mem::zeroed();
    let mut ptr: compat_caddr_t = 0;
    let err: c_int;

    /* offset, space, used, count */
    if copy_from_user(
        (&mut data as *mut snd_ctl_elem_list).cast(),
        data32.cast(),
        4 * size_of::<u32>(),
    ) != 0
    {
        return -EFAULT;
    }
    /* pids */
    if get_user(&mut ptr, &(*data32).pids) != 0 {
        return -EFAULT;
    }
    data.pids = compat_ptr(ptr as c_ulong);
    err = snd_ctl_elem_list(card, &mut data);
    if err < 0 {
        return err;
    }
    /* copy the result */
    if copy_to_user(
        data32.cast(),
        (&data as *const snd_ctl_elem_list).cast(),
        4 * size_of::<u32>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

/*
 * control element info
 * it uses union, so the things are not easy..
 */

#[repr(C, packed)]
struct snd_ctl_elem_info32_integer {
    min: s32,
    max: s32,
    step: s32,
}

#[repr(C, packed)]
struct snd_ctl_elem_info32_integer64 {
    min: u64,
    max: u64,
    step: u64,
}

#[repr(C, packed)]
struct snd_ctl_elem_info32_enumerated {
    items: u32,
    item: u32,
    name: [c_char; 64],
    names_ptr: u64,
    names_length: u32,
}

#[repr(C)]
union snd_ctl_elem_info32_value {
    integer: core::mem::ManuallyDrop<snd_ctl_elem_info32_integer>,
    integer64: core::mem::ManuallyDrop<snd_ctl_elem_info32_integer64>,
    enumerated: core::mem::ManuallyDrop<snd_ctl_elem_info32_enumerated>,
    reserved: [u8; 128],
}

#[repr(C, packed)]
struct snd_ctl_elem_info32 {
    id: snd_ctl_elem_id, // the size of struct is same
    type_: s32,
    access: u32,
    count: u32,
    owner: s32,
    value: snd_ctl_elem_info32_value,
    reserved: [u8; 64],
}

#[repr(C)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
    step: c_long,
}

#[repr(C)]
struct snd_ctl_elem_info_integer64 {
    min: s64,
    max: s64,
    step: s64,
}

#[repr(C)]
struct snd_ctl_elem_info_enumerated {
    items: u32,
    item: u32,
    name: [c_char; 64],
    names_ptr: u64,
    names_length: u32,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer>,
    integer64: core::mem::ManuallyDrop<snd_ctl_elem_info_integer64>,
    enumerated: core::mem::ManuallyDrop<snd_ctl_elem_info_enumerated>,
    reserved: [u8; 128],
}

#[repr(C)]
struct snd_ctl_elem_info {
    id: snd_ctl_elem_id,
    type_: c_int,
    access: c_uint,
    count: c_uint,
    owner: c_int,
    value: snd_ctl_elem_info_value,
}

unsafe fn alloc_zeroed<T>() -> *mut T {
    kzalloc(size_of::<T>(), 0).cast()
}

unsafe fn snd_ctl_elem_info_compat(
    ctl: *mut snd_ctl_file,
    data32: *mut snd_ctl_elem_info32,
) -> c_int {
    let card = (*ctl).card;
    let mut err: c_int;
    let data: *mut snd_ctl_elem_info = alloc_zeroed();

    if data.is_null() {
        return -ENOMEM;
    }

    /* copy id */
    if copy_from_user(
        (&mut (*data).id as *mut snd_ctl_elem_id).cast(),
        (&(*data32).id as *const snd_ctl_elem_id).cast(),
        size_of::<snd_ctl_elem_id>(),
    ) != 0
    {
        kfree(data.cast());
        return -EFAULT;
    }
    /* we need to copy the item index.
     * hope this doesn't break anything..
     */
    if get_user(
        &mut (*data).value.enumerated.item,
        &(*data32).value.enumerated.item,
    ) != 0
    {
        kfree(data.cast());
        return -EFAULT;
    }

    err = snd_power_ref_and_wait(card);
    if err < 0 {
        kfree(data.cast());
        return err;
    }
    err = snd_ctl_elem_info(ctl, data);
    snd_power_unref(card);
    if err < 0 {
        kfree(data.cast());
        return err;
    }
    /* restore info to 32bit */
    /* id, type, access, count */
    if copy_to_user(
        (&mut (*data32).id as *mut snd_ctl_elem_id).cast(),
        (&(*data).id as *const snd_ctl_elem_id).cast(),
        size_of::<snd_ctl_elem_id>(),
    ) != 0
        || copy_to_user(
            (&mut (*data32).type_ as *mut s32).cast(),
            (&(*data).type_ as *const c_int).cast(),
            3 * size_of::<u32>(),
        ) != 0
    {
        kfree(data.cast());
        return -EFAULT;
    }
    if put_user((*data).owner, &mut (*data32).owner) != 0 {
        kfree(data.cast());
        return -EFAULT;
    }
    match (*data).type_ {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN | SNDRV_CTL_ELEM_TYPE_INTEGER => {
            if put_user(
                (*data).value.integer.min as s32,
                &mut (*data32).value.integer.min,
            ) != 0
                || put_user(
                    (*data).value.integer.max as s32,
                    &mut (*data32).value.integer.max,
                ) != 0
                || put_user(
                    (*data).value.integer.step as s32,
                    &mut (*data32).value.integer.step,
                ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
        }
        SNDRV_CTL_ELEM_TYPE_INTEGER64 => {
            if copy_to_user(
                (&mut (*data32).value.integer64 as *mut _).cast(),
                (&(*data).value.integer64 as *const _).cast(),
                size_of::<snd_ctl_elem_info_integer64>(),
            ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
        }
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
            if copy_to_user(
                (&mut (*data32).value.enumerated as *mut _).cast(),
                (&(*data).value.enumerated as *const _).cast(),
                size_of::<snd_ctl_elem_info32_enumerated>(),
            ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
        }
        _ => {}
    }
    kfree(data.cast());
    0
}

/* read / write */
#[repr(C)]
union snd_ctl_elem_value32_value {
    integer: [s32; 128],
    data: [u8; 512],
    #[cfg(not(target_arch = "x86_64"))]
    integer64: [s64; 64],
}

#[repr(C)]
struct snd_ctl_elem_value32 {
    id: snd_ctl_elem_id,
    indirect: c_uint, /* bit-field causes misalignment */
    value: snd_ctl_elem_value32_value,
    reserved: [u8; 128],
}

/* CONFIG_X86_X32_ABI: x32 has a different alignment for 64bit values from ia32 */
#[cfg(target_env = "x32")]
#[repr(C)]
union snd_ctl_elem_value_x32_value {
    integer: [s32; 128],
    data: [u8; 512],
    integer64: [s64; 64],
}

#[cfg(target_env = "x32")]
#[repr(C)]
struct snd_ctl_elem_value_x32 {
    id: snd_ctl_elem_id,
    indirect: c_uint, /* bit-field causes misalignment */
    value: snd_ctl_elem_value_x32_value,
    reserved: [u8; 128],
}

#[repr(C)]
struct snd_kcontrol {
    info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_ctl_elem_value_bytes {
    data: [u8; 512],
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
    bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_bytes>,
}

#[repr(C)]
struct snd_ctl_elem_value {
    id: snd_ctl_elem_id,
    indirect: c_uint,
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_aes_iec958 {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
struct snd_kctl_ioctl {
    list: list_head,
    fioctl: Option<
        unsafe extern "C" fn(
            *mut snd_card,
            *mut snd_ctl_file,
            c_uint,
            c_ulong,
        ) -> c_int,
    >,
}

struct RwsemReadGuard {
    sem: *mut c_void,
}

impl RwsemReadGuard {
    unsafe fn new(sem: *mut c_void) -> Self {
        rwsem_read_guard(sem);
        Self { sem }
    }
}

impl Drop for RwsemReadGuard {
    fn drop(&mut self) {
        unsafe {
            rwsem_read_unguard(self.sem);
        }
    }
}

/* get the value type and count of the control */
unsafe fn get_ctl_type(
    card: *mut snd_card,
    id: *mut snd_ctl_elem_id,
    countp: *mut c_int,
) -> c_int {
    let kctl: *mut snd_kcontrol;
    let mut err: c_int;

    let _guard = RwsemReadGuard::new((&mut (*card).controls_rwsem as *mut c_void).cast());
    kctl = snd_ctl_find_id(card, id);
    if kctl.is_null() {
        return -ENOENT;
    }

    let info: *mut snd_ctl_elem_info = alloc_zeroed();
    if info.is_null() {
        return -ENOMEM;
    }
    (*info).id = *id;
    err = ((*kctl).info)(kctl, info);
    if err >= 0 {
        err = (*info).type_;
        *countp = (*info).count as c_int;
    }
    kfree(info.cast());
    err
}

unsafe fn get_elem_size(type_: snd_ctl_elem_type_t, count: c_int) -> c_int {
    match type_ {
        SNDRV_CTL_ELEM_TYPE_INTEGER64 => (size_of::<s64>() * count as usize) as c_int,
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => (size_of::<c_int>() * count as usize) as c_int,
        SNDRV_CTL_ELEM_TYPE_BYTES => 512,
        SNDRV_CTL_ELEM_TYPE_IEC958 => size_of::<snd_aes_iec958>() as c_int,
        _ => -1,
    }
}

unsafe fn copy_ctl_value_from_user(
    card: *mut snd_card,
    data: *mut snd_ctl_elem_value,
    userdata: *mut c_void,
    valuep: *mut c_void,
    typep: *mut c_int,
    countp: *mut c_int,
) -> c_int {
    let data32 = userdata as *mut snd_ctl_elem_value32;
    let mut i: c_int;
    let mut type_: c_int;
    let mut size: c_int;
    let mut count: c_int = 0;
    let mut indirect: c_uint = 0;

    if copy_from_user(
        (&mut (*data).id as *mut snd_ctl_elem_id).cast(),
        (&(*data32).id as *const snd_ctl_elem_id).cast(),
        size_of::<snd_ctl_elem_id>(),
    ) != 0
    {
        return -EFAULT;
    }
    if get_user(&mut indirect, &(*data32).indirect) != 0 {
        return -EFAULT;
    }
    if indirect != 0 {
        return -EINVAL;
    }
    type_ = get_ctl_type(card, &mut (*data).id, &mut count);
    if type_ < 0 {
        return type_;
    }

    if type_ == SNDRV_CTL_ELEM_TYPE_BOOLEAN || type_ == SNDRV_CTL_ELEM_TYPE_INTEGER {
        i = 0;
        while i < count {
            let intp = valuep as *mut s32;
            let mut val: c_int = 0;
            if get_user(&mut val, intp.add(i as usize) as *const c_int) != 0 {
                return -EFAULT;
            }
            (*data).value.integer.value[i as usize] = val as c_long;
            i += 1;
        }
    } else {
        size = get_elem_size(type_, count);
        if size < 0 {
            dev_err(
                (*card).dev,
                b"snd_ioctl32_ctl_elem_value: unknown type %d\n\0".as_ptr().cast(),
                type_,
            );
            return -EINVAL;
        }
        if copy_from_user(
            (*data).value.bytes.data.as_mut_ptr().cast(),
            valuep.cast(),
            size as usize,
        ) != 0
        {
            return -EFAULT;
        }
    }

    *typep = type_;
    *countp = count;
    0
}

/* restore the value to 32bit */
unsafe fn copy_ctl_value_to_user(
    userdata: *mut c_void,
    valuep: *mut c_void,
    data: *mut snd_ctl_elem_value,
    type_: c_int,
    count: c_int,
) -> c_int {
    let data32 = userdata as *mut snd_ctl_elem_value32;
    let mut i: c_int;
    let size: c_int;

    if type_ == SNDRV_CTL_ELEM_TYPE_BOOLEAN || type_ == SNDRV_CTL_ELEM_TYPE_INTEGER {
        i = 0;
        while i < count {
            let intp = valuep as *mut s32;
            let val: c_int;
            val = (*data).value.integer.value[i as usize] as c_int;
            if put_user(val, intp.add(i as usize) as *mut c_int) != 0 {
                return -EFAULT;
            }
            i += 1;
        }
    } else {
        size = get_elem_size(type_, count);
        if copy_to_user(
            valuep.cast(),
            (*data).value.bytes.data.as_ptr().cast(),
            size as usize,
        ) != 0
        {
            return -EFAULT;
        }
    }
    if copy_to_user(
        (&mut (*data32).id as *mut snd_ctl_elem_id).cast(),
        (&(*data).id as *const snd_ctl_elem_id).cast(),
        size_of::<snd_ctl_elem_id>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

unsafe fn __ctl_elem_read_user(
    card: *mut snd_card,
    userdata: *mut c_void,
    valuep: *mut c_void,
) -> c_int {
    let mut err: c_int;
    let mut type_: c_int = 0;
    let mut count: c_int = 0;
    let data: *mut snd_ctl_elem_value = alloc_zeroed();

    if data.is_null() {
        return -ENOMEM;
    }

    err = copy_ctl_value_from_user(card, data, userdata, valuep, &mut type_, &mut count);
    if err < 0 {
        kfree(data.cast());
        return err;
    }

    err = snd_ctl_elem_read(card, data);
    if err < 0 {
        kfree(data.cast());
        return err;
    }
    err = copy_ctl_value_to_user(userdata, valuep, data, type_, count);
    kfree(data.cast());
    err
}

unsafe fn ctl_elem_read_user(
    card: *mut snd_card,
    userdata: *mut c_void,
    valuep: *mut c_void,
) -> c_int {
    let mut err: c_int;

    err = snd_power_ref_and_wait(card);
    if err < 0 {
        return err;
    }
    err = __ctl_elem_read_user(card, userdata, valuep);
    snd_power_unref(card);
    err
}

unsafe fn __ctl_elem_write_user(
    file: *mut snd_ctl_file,
    userdata: *mut c_void,
    valuep: *mut c_void,
) -> c_int {
    let card = (*file).card;
    let mut err: c_int;
    let mut type_: c_int = 0;
    let mut count: c_int = 0;
    let data: *mut snd_ctl_elem_value = alloc_zeroed();

    if data.is_null() {
        return -ENOMEM;
    }

    err = copy_ctl_value_from_user(card, data, userdata, valuep, &mut type_, &mut count);
    if err < 0 {
        kfree(data.cast());
        return err;
    }

    err = snd_ctl_elem_write(card, file, data);
    if err < 0 {
        kfree(data.cast());
        return err;
    }
    err = copy_ctl_value_to_user(userdata, valuep, data, type_, count);
    kfree(data.cast());
    err
}

unsafe fn ctl_elem_write_user(
    file: *mut snd_ctl_file,
    userdata: *mut c_void,
    valuep: *mut c_void,
) -> c_int {
    let card = (*file).card;
    let mut err: c_int;

    err = snd_power_ref_and_wait(card);
    if err < 0 {
        return err;
    }
    err = __ctl_elem_write_user(file, userdata, valuep);
    snd_power_unref(card);
    err
}

unsafe fn snd_ctl_elem_read_user_compat(
    card: *mut snd_card,
    data32: *mut snd_ctl_elem_value32,
) -> c_int {
    ctl_elem_read_user(card, data32.cast(), (&mut (*data32).value as *mut _).cast())
}

unsafe fn snd_ctl_elem_write_user_compat(
    file: *mut snd_ctl_file,
    data32: *mut snd_ctl_elem_value32,
) -> c_int {
    ctl_elem_write_user(file, data32.cast(), (&mut (*data32).value as *mut _).cast())
}

#[cfg(target_env = "x32")]
unsafe fn snd_ctl_elem_read_user_x32(
    card: *mut snd_card,
    data32: *mut snd_ctl_elem_value_x32,
) -> c_int {
    ctl_elem_read_user(card, data32.cast(), (&mut (*data32).value as *mut _).cast())
}

#[cfg(target_env = "x32")]
unsafe fn snd_ctl_elem_write_user_x32(
    file: *mut snd_ctl_file,
    data32: *mut snd_ctl_elem_value_x32,
) -> c_int {
    ctl_elem_write_user(file, data32.cast(), (&mut (*data32).value as *mut _).cast())
}

/* add or replace a user control */
unsafe fn snd_ctl_elem_add_compat(
    file: *mut snd_ctl_file,
    data32: *mut snd_ctl_elem_info32,
    replace: c_int,
) -> c_int {
    let data: *mut snd_ctl_elem_info = alloc_zeroed();

    if data.is_null() {
        return -ENOMEM;
    }

    /* id, type, access, count */
    if copy_from_user(
        (&mut (*data).id as *mut snd_ctl_elem_id).cast(),
        (&(*data32).id as *const snd_ctl_elem_id).cast(),
        size_of::<snd_ctl_elem_id>(),
    ) != 0
        || copy_from_user(
            (&mut (*data).type_ as *mut c_int).cast(),
            (&(*data32).type_ as *const s32).cast(),
            3 * size_of::<u32>(),
        ) != 0
    {
        kfree(data.cast());
        return -EFAULT;
    }
    if get_user(&mut (*data).owner, &(*data32).owner as *const s32 as *const c_int) != 0 {
        kfree(data.cast());
        return -EFAULT;
    }
    match (*data).type_ {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN | SNDRV_CTL_ELEM_TYPE_INTEGER => {
            if get_user(
                &mut (*data).value.integer.min,
                &(*data32).value.integer.min as *const s32 as *const c_long,
            ) != 0
                || get_user(
                    &mut (*data).value.integer.max,
                    &(*data32).value.integer.max as *const s32 as *const c_long,
                ) != 0
                || get_user(
                    &mut (*data).value.integer.step,
                    &(*data32).value.integer.step as *const s32 as *const c_long,
                ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
        }
        SNDRV_CTL_ELEM_TYPE_INTEGER64 => {
            if copy_from_user(
                (&mut (*data).value.integer64 as *mut _).cast(),
                (&(*data32).value.integer64 as *const _).cast(),
                size_of::<snd_ctl_elem_info_integer64>(),
            ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
        }
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
            if copy_from_user(
                (&mut (*data).value.enumerated as *mut _).cast(),
                (&(*data32).value.enumerated as *const _).cast(),
                size_of::<snd_ctl_elem_info32_enumerated>(),
            ) != 0
            {
                kfree(data.cast());
                return -EFAULT;
            }
            (*data).value.enumerated.names_ptr =
                compat_ptr((*data).value.enumerated.names_ptr as c_ulong) as usize as u64;
        }
        _ => {}
    }
    let ret = snd_ctl_elem_add(file, data, replace);
    kfree(data.cast());
    ret
}

const fn _IOWR(_type: u8, _nr: u8, _size: usize) -> c_uint {
    ((_type as c_uint) << 8) | (_nr as c_uint)
}

const SNDRV_CTL_IOCTL_ELEM_LIST32: c_uint =
    _IOWR(b'U', 0x10, size_of::<snd_ctl_elem_list32>());
const SNDRV_CTL_IOCTL_ELEM_INFO32: c_uint =
    _IOWR(b'U', 0x11, size_of::<snd_ctl_elem_info32>());
const SNDRV_CTL_IOCTL_ELEM_READ32: c_uint =
    _IOWR(b'U', 0x12, size_of::<snd_ctl_elem_value32>());
const SNDRV_CTL_IOCTL_ELEM_WRITE32: c_uint =
    _IOWR(b'U', 0x13, size_of::<snd_ctl_elem_value32>());
const SNDRV_CTL_IOCTL_ELEM_ADD32: c_uint =
    _IOWR(b'U', 0x17, size_of::<snd_ctl_elem_info32>());
const SNDRV_CTL_IOCTL_ELEM_REPLACE32: c_uint =
    _IOWR(b'U', 0x18, size_of::<snd_ctl_elem_info32>());
#[cfg(target_env = "x32")]
const SNDRV_CTL_IOCTL_ELEM_READ_X32: c_uint =
    _IOWR(b'U', 0x12, size_of::<snd_ctl_elem_value_x32>());
#[cfg(target_env = "x32")]
const SNDRV_CTL_IOCTL_ELEM_WRITE_X32: c_uint =
    _IOWR(b'U', 0x13, size_of::<snd_ctl_elem_value_x32>());

unsafe fn snd_ctl_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let ctl: *mut snd_ctl_file;
    let mut p: *mut snd_kctl_ioctl;
    let argp: *mut c_void = compat_ptr(arg);
    let mut err: c_int;

    ctl = (*file).private_data.cast();
    if snd_BUG_ON(ctl.is_null() || (*ctl).card.is_null()) {
        return -ENXIO as c_long;
    }

    if cmd == SNDRV_CTL_IOCTL_PVERSION
        || cmd == SNDRV_CTL_IOCTL_CARD_INFO
        || cmd == SNDRV_CTL_IOCTL_CARD_BYTES
        || cmd == SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS
        || cmd == SNDRV_CTL_IOCTL_POWER
        || cmd == SNDRV_CTL_IOCTL_POWER_STATE
        || cmd == SNDRV_CTL_IOCTL_ELEM_LOCK
        || cmd == SNDRV_CTL_IOCTL_ELEM_UNLOCK
        || cmd == SNDRV_CTL_IOCTL_ELEM_REMOVE
        || cmd == SNDRV_CTL_IOCTL_TLV_READ
        || cmd == SNDRV_CTL_IOCTL_TLV_WRITE
        || cmd == SNDRV_CTL_IOCTL_TLV_COMMAND
    {
        return snd_ctl_ioctl(file, cmd, argp as c_ulong);
    }

    match cmd {
        SNDRV_CTL_IOCTL_ELEM_LIST32 => {
            return snd_ctl_elem_list_compat((*ctl).card, argp.cast()) as c_long;
        }
        SNDRV_CTL_IOCTL_ELEM_INFO32 => {
            return snd_ctl_elem_info_compat(ctl, argp.cast()) as c_long;
        }
        SNDRV_CTL_IOCTL_ELEM_READ32 => {
            return snd_ctl_elem_read_user_compat((*ctl).card, argp.cast()) as c_long;
        }
        SNDRV_CTL_IOCTL_ELEM_WRITE32 => {
            return snd_ctl_elem_write_user_compat(ctl, argp.cast()) as c_long;
        }
        SNDRV_CTL_IOCTL_ELEM_ADD32 => {
            return snd_ctl_elem_add_compat(ctl, argp.cast(), 0) as c_long;
        }
        SNDRV_CTL_IOCTL_ELEM_REPLACE32 => {
            return snd_ctl_elem_add_compat(ctl, argp.cast(), 1) as c_long;
        }
        #[cfg(target_env = "x32")]
        SNDRV_CTL_IOCTL_ELEM_READ_X32 => {
            return snd_ctl_elem_read_user_x32((*ctl).card, argp.cast()) as c_long;
        }
        #[cfg(target_env = "x32")]
        SNDRV_CTL_IOCTL_ELEM_WRITE_X32 => {
            return snd_ctl_elem_write_user_x32(ctl, argp.cast()) as c_long;
        }
        _ => {}
    }

    let _guard = RwsemReadGuard::new((&mut snd_ioctl_rwsem as *mut c_void).cast());
    p = snd_control_compat_ioctls.next.cast();
    while (p.cast::<list_head>()) != (&mut snd_control_compat_ioctls as *mut list_head) {
        if let Some(fioctl) = (*p).fioctl {
            err = fioctl((*ctl).card, ctl, cmd, arg);
            if err != -ENOIOCTLCMD {
                return err as c_long;
            }
        }
        p = (*p).list.next.cast();
    }
    -ENOIOCTLCMD as c_long
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
