// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Load Analog Devices SigmaStudio firmware files
 *
 * Copyright 2009-2014 Analog Devices Inc.
 */

// C dependencies originally included from Linux/ALSA headers and "sigmadsp.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const SIGMA_MAGIC: &[u8; 7] = b"ADISIGM";

const SIGMA_FW_CHUNK_TYPE_DATA: u32 = 0;
const SIGMA_FW_CHUNK_TYPE_CONTROL: u32 = 1;
const SIGMA_FW_CHUNK_TYPE_SAMPLERATES: u32 = 2;

const READBACK_CTRL_NAME: &[u8; 8] = b"ReadBack";

const SIGMA_ACTION_WRITEXBYTES: u8 = 0;
const SIGMA_ACTION_WRITESINGLE: u8 = 1;
const SIGMA_ACTION_WRITESAFELOAD: u8 = 2;
const SIGMA_ACTION_END: u8 = 3;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_uint = 4;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 0x0100;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 10;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_ctl_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub bytes: snd_ctl_bytes,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub vd: [snd_kcontrol_volatile; 1],
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
}

#[repr(C)]
pub struct sigmadsp_ops {
    pub safeload: Option<
        unsafe extern "C" fn(*mut sigmadsp, c_uint, *const c_void, c_uint) -> c_int,
    >,
}

#[repr(C)]
pub struct sigmadsp {
    pub write: unsafe extern "C" fn(*mut c_void, c_uint, *const u8, usize) -> c_int,
    pub read: unsafe extern "C" fn(*mut c_void, c_uint, *mut u8, usize) -> c_int,
    pub control_data: *mut c_void,
    pub ops: *const sigmadsp_ops,
    pub dev: *mut device,
    pub ctrl_list: list_head,
    pub data_list: list_head,
    pub lock: mutex,
    pub component: *mut snd_soc_component,
    pub current_samplerate: c_uint,
    pub rate_constraints: snd_pcm_hw_constraint_list,
}

#[repr(C)]
struct sigmadsp_control {
    head: list_head,
    samplerates: u32,
    addr: c_uint,
    num_bytes: c_uint,
    name: *const c_char,
    kcontrol: *mut snd_kcontrol,
    is_readback: bool,
    cached: bool,
    cache: [u8; 0],
}

#[repr(C)]
struct sigmadsp_data {
    head: list_head,
    samplerates: u32,
    addr: c_uint,
    length: c_uint,
    data: [u8; 0],
}

#[repr(C, packed)]
struct sigma_fw_chunk {
    length: u32,
    tag: u32,
    samplerates: u32,
}

#[repr(C, packed)]
struct sigma_fw_chunk_data {
    chunk: sigma_fw_chunk,
    addr: u16,
    data: [u8; 0],
}

#[repr(C, packed)]
struct sigma_fw_chunk_control {
    chunk: sigma_fw_chunk,
    type_: u16,
    addr: u16,
    num_bytes: u16,
    name: [c_char; 0],
}

#[repr(C, packed)]
struct sigma_fw_chunk_samplerate {
    chunk: sigma_fw_chunk,
    samplerates: [u32; 0],
}

#[repr(C, packed)]
struct sigma_firmware_header {
    magic: [u8; 7],
    version: u8,
    crc: u32,
}

#[repr(C, packed)]
struct sigma_action {
    instr: u8,
    len_hi: u8,
    len: u16,
    addr: u16,
    payload: [u8; 0],
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kmemdup_nul(src: *const c_void, len: usize, flags: c_uint) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn crc32(crc: u32, buf: *const u8, len: usize) -> u32;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn devres_alloc(
        release: Option<unsafe extern "C" fn(*mut device, *mut c_void)>,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut sigmadsp;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_activate_id(card: *mut snd_card, id: *mut snd_ctl_elem_id, active: bool) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn ERR_PTR(error: isize) -> *mut sigmadsp;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

#[inline]
fn le16_to_cpu(x: u16) -> u16 {
    u16::from_le(x)
}

#[inline]
fn le32_to_cpu(x: u32) -> u32 {
    u32::from_le(x)
}

#[inline]
fn be16_to_cpu(x: u16) -> u16 {
    u16::from_be(x)
}

#[inline]
fn align(value: usize, a: usize) -> usize {
    (value + a - 1) & !(a - 1)
}

#[inline]
fn bit(n: c_int) -> c_uint {
    1u32.wrapping_shl(n as u32)
}

#[inline]
unsafe fn list_entry<T>(ptr: *mut list_head, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}

#[inline]
fn offset_sigmadsp_control_head() -> usize {
    0
}

#[inline]
fn offset_sigmadsp_data_head() -> usize {
    0
}

#[inline]
fn flex_size<T>(extra: usize) -> usize {
    size_of::<T>() + extra
}

unsafe fn sigmadsp_write(sigmadsp: *mut sigmadsp, addr: c_uint, data: *const u8, len: usize) -> c_int {
    ((*sigmadsp).write)((*sigmadsp).control_data, addr, data, len)
}

unsafe fn sigmadsp_read(sigmadsp: *mut sigmadsp, addr: c_uint, data: *mut u8, len: usize) -> c_int {
    ((*sigmadsp).read)((*sigmadsp).control_data, addr, data, len)
}

unsafe extern "C" fn sigmadsp_ctrl_info(
    kcontrol: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    let ctrl = (*kcontrol).private_value as *mut sigmadsp_control;

    (*info).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*info).count = (*ctrl).num_bytes;

    0
}

unsafe fn sigmadsp_ctrl_write(
    sigmadsp: *mut sigmadsp,
    ctrl: *mut sigmadsp_control,
    data: *mut c_void,
) -> c_int {
    /* safeload loads up to 20 bytes in a atomic operation */
    if (*ctrl).num_bytes <= 20
        && !(*sigmadsp).ops.is_null()
        && (*(*sigmadsp).ops).safeload.is_some()
    {
        ((*(*sigmadsp).ops).safeload.unwrap())(sigmadsp, (*ctrl).addr, data, (*ctrl).num_bytes)
    } else {
        sigmadsp_write(sigmadsp, (*ctrl).addr, data as *const u8, (*ctrl).num_bytes as usize)
    }
}

unsafe extern "C" fn sigmadsp_ctrl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ctrl = (*kcontrol).private_value as *mut sigmadsp_control;
    let sigmadsp = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    mutex_lock(&mut (*sigmadsp).lock);

    let data = (*ucontrol).value.bytes.data.as_mut_ptr();

    if ((*kcontrol).vd[0].access & SNDRV_CTL_ELEM_ACCESS_INACTIVE) == 0 {
        ret = sigmadsp_ctrl_write(sigmadsp, ctrl, data as *mut c_void);
    }

    if ret == 0 {
        memcpy((*ctrl).cache.as_mut_ptr() as *mut c_void, data as *const c_void, (*ctrl).num_bytes as usize);
        if !(*ctrl).is_readback {
            (*ctrl).cached = true;
        }
    }

    mutex_unlock(&mut (*sigmadsp).lock);

    ret
}

unsafe extern "C" fn sigmadsp_ctrl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ctrl = (*kcontrol).private_value as *mut sigmadsp_control;
    let sigmadsp = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    mutex_lock(&mut (*sigmadsp).lock);

    if !(*ctrl).cached {
        ret = sigmadsp_read(sigmadsp, (*ctrl).addr, (*ctrl).cache.as_mut_ptr(), (*ctrl).num_bytes as usize);
    }

    if ret == 0 {
        if !(*ctrl).is_readback {
            (*ctrl).cached = true;
        }
        memcpy(
            (*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
            (*ctrl).cache.as_ptr() as *const c_void,
            (*ctrl).num_bytes as usize,
        );
    }

    mutex_unlock(&mut (*sigmadsp).lock);

    ret
}

unsafe extern "C" fn sigmadsp_control_free(kcontrol: *mut snd_kcontrol) {
    let ctrl = (*kcontrol).private_value as *mut sigmadsp_control;

    (*ctrl).kcontrol = ptr::null_mut();
}

unsafe fn sigma_fw_validate_control_name(name: *const c_char, len: c_uint) -> bool {
    let mut i: c_uint = 0;

    while i < len {
        /* Normal ASCII characters are valid */
        let ch = *name.add(i as usize);
        if ch < b' ' as c_char || ch > b'~' as c_char {
            return false;
        }
        i += 1;
    }

    true
}

unsafe fn sigma_fw_load_control(
    sigmadsp: *mut sigmadsp,
    chunk: *const sigma_fw_chunk,
    length: c_uint,
) -> c_int {
    let ctrl_chunk: *const sigma_fw_chunk_control;
    let ctrl: *mut sigmadsp_control;
    let num_bytes: c_uint;
    let mut name_len: usize;
    let name: *mut c_char;
    let ret: c_int;

    if (length as usize) <= size_of::<sigma_fw_chunk_control>() {
        return -EINVAL;
    }

    ctrl_chunk = chunk as *const sigma_fw_chunk_control;

    name_len = length as usize - size_of::<sigma_fw_chunk_control>();
    if name_len >= SNDRV_CTL_ELEM_ID_NAME_MAXLEN {
        name_len = SNDRV_CTL_ELEM_ID_NAME_MAXLEN - 1;
    }

    /* Make sure there are no non-displayable characaters in the string */
    if !sigma_fw_validate_control_name((*ctrl_chunk).name.as_ptr(), name_len as c_uint) {
        return -EINVAL;
    }

    num_bytes = le16_to_cpu((*ctrl_chunk).num_bytes) as c_uint;
    ctrl = kzalloc(flex_size::<sigmadsp_control>(num_bytes as usize), GFP_KERNEL) as *mut sigmadsp_control;
    if ctrl.is_null() {
        return -ENOMEM;
    }

    name = kmemdup_nul((*ctrl_chunk).name.as_ptr() as *const c_void, name_len, GFP_KERNEL);
    if name.is_null() {
        ret = -ENOMEM;
        kfree(ctrl as *const c_void);
        return ret;
    }
    (*ctrl).name = name;

    /*
     * Readbacks doesn't work with non-volatile controls, since the
     * firmware updates the control value without driver interaction. Mark
     * the readbacks to ensure that the values are not cached.
     */
    if !(*ctrl).name.is_null()
        && strncmp(
            (*ctrl).name,
            READBACK_CTRL_NAME.as_ptr() as *const c_char,
            READBACK_CTRL_NAME.len() - 1,
        ) == 0
    {
        (*ctrl).is_readback = true;
    }

    (*ctrl).addr = le16_to_cpu((*ctrl_chunk).addr) as c_uint;
    (*ctrl).num_bytes = num_bytes;
    (*ctrl).samplerates = le32_to_cpu((*chunk).samplerates);

    list_add_tail(&mut (*ctrl).head, &mut (*sigmadsp).ctrl_list);

    0
}

unsafe fn sigma_fw_load_data(
    sigmadsp: *mut sigmadsp,
    chunk: *const sigma_fw_chunk,
    mut length: c_uint,
) -> c_int {
    let data_chunk: *const sigma_fw_chunk_data;
    let data: *mut sigmadsp_data;

    if (length as usize) <= size_of::<sigma_fw_chunk_data>() {
        return -EINVAL;
    }

    data_chunk = chunk as *const sigma_fw_chunk_data;

    length -= size_of::<sigma_fw_chunk_data>() as c_uint;

    data = kzalloc(flex_size::<sigmadsp_data>(length as usize), GFP_KERNEL) as *mut sigmadsp_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).addr = le16_to_cpu((*data_chunk).addr) as c_uint;
    (*data).length = length;
    (*data).samplerates = le32_to_cpu((*chunk).samplerates);
    memcpy(
        (*data).data.as_mut_ptr() as *mut c_void,
        (*data_chunk).data.as_ptr() as *const c_void,
        length as usize,
    );
    list_add_tail(&mut (*data).head, &mut (*sigmadsp).data_list);

    0
}

unsafe fn sigma_fw_load_samplerates(
    sigmadsp: *mut sigmadsp,
    chunk: *const sigma_fw_chunk,
    length: c_uint,
) -> c_int {
    let rate_chunk: *const sigma_fw_chunk_samplerate;
    let num_rates: c_uint;
    let rates: *mut c_uint;
    let mut i: c_uint;

    rate_chunk = chunk as *const sigma_fw_chunk_samplerate;

    num_rates = ((length as usize - size_of::<sigma_fw_chunk_samplerate>()) / size_of::<u32>()) as c_uint;

    if num_rates > 32 || num_rates == 0 {
        return -EINVAL;
    }

    /* We only allow one samplerates block per file */
    if (*sigmadsp).rate_constraints.count != 0 {
        return -EINVAL;
    }

    rates = kcalloc(num_rates as usize, size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if rates.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < num_rates {
        *rates.add(i as usize) = le32_to_cpu(*(*rate_chunk).samplerates.as_ptr().add(i as usize));
        i += 1;
    }

    (*sigmadsp).rate_constraints.count = num_rates;
    (*sigmadsp).rate_constraints.list = rates;

    0
}

unsafe fn sigmadsp_fw_load_v2(sigmadsp: *mut sigmadsp, fw: *const firmware) -> c_int {
    let mut chunk: *mut sigma_fw_chunk;
    let mut length: c_uint;
    let mut pos: c_uint;
    let mut ret: c_int;

    /*
     * Make sure that there is at least one chunk to avoid integer
     * underflows later on. Empty firmware is still valid though.
     */
    if (*fw).size < size_of::<sigma_fw_chunk>() + size_of::<sigma_firmware_header>() {
        return 0;
    }

    pos = size_of::<sigma_firmware_header>() as c_uint;

    while (pos as usize) < (*fw).size - size_of::<sigma_fw_chunk>() {
        chunk = (*fw).data.add(pos as usize) as *mut sigma_fw_chunk;

        length = le32_to_cpu((*chunk).length);

        if (length as usize) > (*fw).size - pos as usize || (length as usize) < size_of::<sigma_fw_chunk>() {
            return -EINVAL;
        }

        match le32_to_cpu((*chunk).tag) {
            SIGMA_FW_CHUNK_TYPE_DATA => {
                ret = sigma_fw_load_data(sigmadsp, chunk, length);
            }
            SIGMA_FW_CHUNK_TYPE_CONTROL => {
                ret = sigma_fw_load_control(sigmadsp, chunk, length);
            }
            SIGMA_FW_CHUNK_TYPE_SAMPLERATES => {
                ret = sigma_fw_load_samplerates(sigmadsp, chunk, length);
            }
            _ => {
                dev_warn(
                    (*sigmadsp).dev,
                    b"Unknown chunk type: %d\n\0".as_ptr() as *const c_char,
                    (*chunk).tag,
                );
                ret = 0;
            }
        }

        if ret != 0 {
            return ret;
        }

        /*
         * This can not overflow since if length is larger than the
         * maximum firmware size (0x4000000) we'll error out earilier.
         */
        pos = pos.wrapping_add(align(length as usize, size_of::<u32>()) as c_uint);
    }

    0
}

#[inline]
unsafe fn sigma_action_len(sa: *mut sigma_action) -> u32 {
    ((*sa).len_hi as u32) << 16 | le16_to_cpu((*sa).len) as u32
}

unsafe fn sigma_action_size(sa: *mut sigma_action) -> usize {
    let mut payload: usize = 0;

    match (*sa).instr {
        SIGMA_ACTION_WRITEXBYTES | SIGMA_ACTION_WRITESINGLE | SIGMA_ACTION_WRITESAFELOAD => {
            payload = sigma_action_len(sa) as usize;
        }
        _ => {}
    }

    payload = align(payload, 2);

    payload + size_of::<sigma_action>()
}

/*
 * Returns a negative error value in case of an error, 0 if processing of
 * the firmware should be stopped after this action, 1 otherwise.
 */
unsafe fn process_sigma_action(sigmadsp: *mut sigmadsp, sa: *mut sigma_action) -> c_int {
    let len = sigma_action_len(sa) as usize;
    let data: *mut sigmadsp_data;

    pr_debug(
        b"%s: instr:%i addr:%#x len:%zu\n\0".as_ptr() as *const c_char,
        b"process_sigma_action\0".as_ptr() as *const c_char,
        (*sa).instr as c_int,
        (*sa).addr as c_uint,
        len,
    );

    match (*sa).instr {
        SIGMA_ACTION_WRITEXBYTES | SIGMA_ACTION_WRITESINGLE | SIGMA_ACTION_WRITESAFELOAD => {
            if len < 3 {
                return -EINVAL;
            }

            data = kzalloc(flex_size::<sigmadsp_data>(len.wrapping_sub(2)), GFP_KERNEL) as *mut sigmadsp_data;
            if data.is_null() {
                return -ENOMEM;
            }

            (*data).addr = be16_to_cpu((*sa).addr) as c_uint;
            (*data).length = (len - 2) as c_uint;
            memcpy(
                (*data).data.as_mut_ptr() as *mut c_void,
                (*sa).payload.as_ptr() as *const c_void,
                (*data).length as usize,
            );
            list_add_tail(&mut (*data).head, &mut (*sigmadsp).data_list);
        }
        SIGMA_ACTION_END => {
            return 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    1
}

unsafe fn sigmadsp_fw_load_v1(sigmadsp: *mut sigmadsp, fw: *const firmware) -> c_int {
    let mut sa: *mut sigma_action;
    let mut size: usize;
    let mut pos: usize;
    let mut ret: c_int;

    pos = size_of::<sigma_firmware_header>();

    while pos + size_of::<sigma_action>() <= (*fw).size {
        sa = (*fw).data.add(pos) as *mut sigma_action;

        size = sigma_action_size(sa);
        pos += size;
        if pos > (*fw).size || size == 0 {
            break;
        }

        ret = process_sigma_action(sigmadsp, sa);

        pr_debug(
            b"%s: action returned %i\n\0".as_ptr() as *const c_char,
            b"sigmadsp_fw_load_v1\0".as_ptr() as *const c_char,
            ret,
        );

        if ret <= 0 {
            return ret;
        }
    }

    if pos != (*fw).size {
        return -EINVAL;
    }

    0
}

unsafe fn sigmadsp_firmware_release(sigmadsp: *mut sigmadsp) {
    let mut pos: *mut list_head;
    let mut n: *mut list_head;

    pos = (*sigmadsp).ctrl_list.next;
    while pos != &mut (*sigmadsp).ctrl_list {
        n = (*pos).next;
        let ctrl: *mut sigmadsp_control = list_entry(pos, offset_sigmadsp_control_head());
        kfree((*ctrl).name as *const c_void);
        kfree(ctrl as *const c_void);
        pos = n;
    }

    pos = (*sigmadsp).data_list.next;
    while pos != &mut (*sigmadsp).data_list {
        n = (*pos).next;
        let data: *mut sigmadsp_data = list_entry(pos, offset_sigmadsp_data_head());
        kfree(data as *const c_void);
        pos = n;
    }

    INIT_LIST_HEAD(&mut (*sigmadsp).ctrl_list);
    INIT_LIST_HEAD(&mut (*sigmadsp).data_list);
}

unsafe extern "C" fn devm_sigmadsp_release(_dev: *mut device, res: *mut c_void) {
    sigmadsp_firmware_release(res as *mut sigmadsp);
}

unsafe fn sigmadsp_firmware_load(sigmadsp: *mut sigmadsp, name: *const c_char) -> c_int {
    let mut ssfw_head: *const sigma_firmware_header;
    let mut fw: *const firmware = ptr::null();
    let mut ret: c_int;
    let crc: u32;

    /* first load the blob */
    ret = request_firmware(&mut fw, name, (*sigmadsp).dev);
    if ret != 0 {
        pr_debug(
            b"%s: request_firmware() failed with %i\n\0".as_ptr() as *const c_char,
            b"sigmadsp_firmware_load\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* then verify the header */
    ret = -EINVAL;

    /*
     * Reject too small or unreasonable large files. The upper limit has been
     * chosen a bit arbitrarily, but it should be enough for all practical
     * purposes and having the limit makes it easier to avoid integer
     * overflows later in the loading process.
     */
    if (*fw).size < size_of::<sigma_firmware_header>() || (*fw).size >= 0x4000000 {
        dev_err(
            (*sigmadsp).dev,
            b"Failed to load firmware: Invalid size\n\0".as_ptr() as *const c_char,
        );
        release_firmware(fw);
        return -EINVAL;
    }

    ssfw_head = (*fw).data as *const sigma_firmware_header;
    if memcmp(
        (*ssfw_head).magic.as_ptr() as *const c_void,
        SIGMA_MAGIC.as_ptr() as *const c_void,
        (*ssfw_head).magic.len(),
    ) != 0
    {
        dev_err(
            (*sigmadsp).dev,
            b"Failed to load firmware: Invalid magic\n\0".as_ptr() as *const c_char,
        );
        release_firmware(fw);
        return -EINVAL;
    }

    crc = crc32(
        0,
        (*fw).data.add(size_of::<sigma_firmware_header>()),
        (*fw).size - size_of::<sigma_firmware_header>(),
    );
    pr_debug(
        b"%s: crc=%x\n\0".as_ptr() as *const c_char,
        b"sigmadsp_firmware_load\0".as_ptr() as *const c_char,
        crc,
    );
    if crc != le32_to_cpu((*ssfw_head).crc) {
        dev_err(
            (*sigmadsp).dev,
            b"Failed to load firmware: Wrong crc checksum: expected %x got %x\n\0".as_ptr()
                as *const c_char,
            le32_to_cpu((*ssfw_head).crc),
            crc,
        );
        release_firmware(fw);
        return -EINVAL;
    }

    match (*ssfw_head).version {
        1 => {
            ret = sigmadsp_fw_load_v1(sigmadsp, fw);
        }
        2 => {
            ret = sigmadsp_fw_load_v2(sigmadsp, fw);
        }
        _ => {
            dev_err(
                (*sigmadsp).dev,
                b"Failed to load firmware: Invalid version %d. Supported firmware versions: 1, 2\n\0"
                    .as_ptr() as *const c_char,
                (*ssfw_head).version as c_int,
            );
            ret = -EINVAL;
        }
    }

    if ret != 0 {
        sigmadsp_firmware_release(sigmadsp);
    }

    release_firmware(fw);
    ret
}

unsafe fn sigmadsp_init(
    sigmadsp: *mut sigmadsp,
    dev: *mut device,
    ops: *const sigmadsp_ops,
    firmware_name: *const c_char,
) -> c_int {
    (*sigmadsp).ops = ops;
    (*sigmadsp).dev = dev;

    INIT_LIST_HEAD(&mut (*sigmadsp).ctrl_list);
    INIT_LIST_HEAD(&mut (*sigmadsp).data_list);
    mutex_init(&mut (*sigmadsp).lock);

    sigmadsp_firmware_load(sigmadsp, firmware_name)
}

/**
 * devm_sigmadsp_init() - Initialize SigmaDSP instance
 * @dev: The parent device
 * @ops: The sigmadsp_ops to use for this instance
 * @firmware_name: Name of the firmware file to load
 *
 * Allocates a SigmaDSP instance and loads the specified firmware file.
 *
 * Returns a pointer to a struct sigmadsp on success, or a PTR_ERR() on error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_sigmadsp_init(
    dev: *mut device,
    ops: *const sigmadsp_ops,
    firmware_name: *const c_char,
) -> *mut sigmadsp {
    let sigmadsp: *mut sigmadsp;
    let ret: c_int;

    sigmadsp = devres_alloc(Some(devm_sigmadsp_release), size_of::<sigmadsp>(), GFP_KERNEL)
        as *mut sigmadsp;
    if sigmadsp.is_null() {
        return ERR_PTR(-ENOMEM as isize);
    }

    ret = sigmadsp_init(sigmadsp, dev, ops, firmware_name);
    if ret != 0 {
        devres_free(sigmadsp as *mut c_void);
        return ERR_PTR(ret as isize);
    }

    devres_add(dev, sigmadsp as *mut c_void);

    sigmadsp
}
// EXPORT_SYMBOL_GPL(devm_sigmadsp_init);

unsafe fn sigmadsp_rate_to_index(sigmadsp: *mut sigmadsp, rate: c_uint) -> c_int {
    let mut i: c_uint = 0;

    while i < (*sigmadsp).rate_constraints.count {
        if *(*sigmadsp).rate_constraints.list.add(i as usize) == rate {
            return i as c_int;
        }
        i += 1;
    }

    -EINVAL
}

unsafe fn sigmadsp_get_samplerate_mask(sigmadsp: *mut sigmadsp, samplerate: c_uint) -> c_uint {
    let samplerate_index: c_int;

    if samplerate == 0 {
        return 0;
    }

    if (*sigmadsp).rate_constraints.count != 0 {
        samplerate_index = sigmadsp_rate_to_index(sigmadsp, samplerate);
        if samplerate_index < 0 {
            return 0;
        }

        bit(samplerate_index)
    } else {
        !0
    }
}

fn sigmadsp_samplerate_valid(supported: c_uint, requested: c_uint) -> bool {
    /* All samplerates are supported */
    if supported == 0 {
        return true;
    }

    (supported & requested) != 0
}

unsafe fn sigmadsp_alloc_control(
    sigmadsp: *mut sigmadsp,
    ctrl: *mut sigmadsp_control,
    samplerate_mask: c_uint,
) -> c_int {
    let mut template: snd_kcontrol_new = zeroed();
    let kcontrol: *mut snd_kcontrol;

    memset(
        &mut template as *mut snd_kcontrol_new as *mut c_void,
        0,
        size_of::<snd_kcontrol_new>(),
    );
    template.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    template.name = (*ctrl).name;
    template.info = Some(sigmadsp_ctrl_info);
    template.get = Some(sigmadsp_ctrl_get);
    template.put = Some(sigmadsp_ctrl_put);
    template.private_value = ctrl as c_ulong;
    template.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    if !sigmadsp_samplerate_valid((*ctrl).samplerates, samplerate_mask) {
        template.access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }

    kcontrol = snd_ctl_new1(&template, sigmadsp as *mut c_void);
    if kcontrol.is_null() {
        return -ENOMEM;
    }

    (*kcontrol).private_free = Some(sigmadsp_control_free);
    (*ctrl).kcontrol = kcontrol;

    snd_ctl_add((*(*(*sigmadsp).component).card).snd_card, kcontrol)
}

unsafe fn sigmadsp_activate_ctrl(
    sigmadsp: *mut sigmadsp,
    ctrl: *mut sigmadsp_control,
    samplerate_mask: c_uint,
) {
    let card = (*(*(*sigmadsp).component).card).snd_card;
    let active: bool;
    let changed: c_int;

    active = sigmadsp_samplerate_valid((*ctrl).samplerates, samplerate_mask);
    if (*ctrl).kcontrol.is_null() {
        return;
    }
    changed = snd_ctl_activate_id(card, &mut (*(*ctrl).kcontrol).id, active);
    if active && changed > 0 {
        mutex_lock(&mut (*sigmadsp).lock);
        if (*ctrl).cached {
            sigmadsp_ctrl_write(sigmadsp, ctrl, (*ctrl).cache.as_mut_ptr() as *mut c_void);
        }
        mutex_unlock(&mut (*sigmadsp).lock);
    }
}

/**
 * sigmadsp_attach() - Attach a sigmadsp instance to a ASoC component
 * @sigmadsp: The sigmadsp instance to attach
 * @component: The component to attach to
 *
 * Typically called in the components probe callback.
 *
 * Note, once this function has been called the firmware must not be released
 * until after the ALSA snd_card that the component belongs to has been
 * disconnected, even if sigmadsp_attach() returns an error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigmadsp_attach(
    sigmadsp: *mut sigmadsp,
    component: *mut snd_soc_component,
) -> c_int {
    let mut ctrl: *mut sigmadsp_control;
    let samplerate_mask: c_uint;
    let mut ret: c_int;

    (*sigmadsp).component = component;

    samplerate_mask = sigmadsp_get_samplerate_mask(sigmadsp, (*sigmadsp).current_samplerate);

    ctrl = list_entry((*sigmadsp).ctrl_list.next, offset_sigmadsp_control_head());
    while &mut (*ctrl).head != &mut (*sigmadsp).ctrl_list {
        ret = sigmadsp_alloc_control(sigmadsp, ctrl, samplerate_mask);
        if ret != 0 {
            return ret;
        }
        ctrl = list_entry((*ctrl).head.next, offset_sigmadsp_control_head());
    }

    0
}
// EXPORT_SYMBOL_GPL(sigmadsp_attach);

/**
 * sigmadsp_setup() - Setup the DSP for the specified samplerate
 * @sigmadsp: The sigmadsp instance to configure
 * @samplerate: The samplerate the DSP should be configured for
 *
 * Loads the appropriate firmware program and parameter memory (if not already
 * loaded) and enables the controls for the specified samplerate. Any control
 * parameter changes that have been made previously will be restored.
 *
 * Returns 0 on success, a negative error code otherwise.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigmadsp_setup(sigmadsp: *mut sigmadsp, samplerate: c_uint) -> c_int {
    let mut ctrl: *mut sigmadsp_control;
    let samplerate_mask: c_uint;
    let mut data: *mut sigmadsp_data;
    let mut ret: c_int;

    if (*sigmadsp).current_samplerate == samplerate {
        return 0;
    }

    samplerate_mask = sigmadsp_get_samplerate_mask(sigmadsp, samplerate);
    if samplerate_mask == 0 {
        return -EINVAL;
    }

    data = list_entry((*sigmadsp).data_list.next, offset_sigmadsp_data_head());
    while &mut (*data).head != &mut (*sigmadsp).data_list {
        if !sigmadsp_samplerate_valid((*data).samplerates, samplerate_mask) {
            data = list_entry((*data).head.next, offset_sigmadsp_data_head());
            continue;
        }
        ret = sigmadsp_write(sigmadsp, (*data).addr, (*data).data.as_ptr(), (*data).length as usize);
        if ret != 0 {
            sigmadsp_reset(sigmadsp);
            return ret;
        }
        data = list_entry((*data).head.next, offset_sigmadsp_data_head());
    }

    ctrl = list_entry((*sigmadsp).ctrl_list.next, offset_sigmadsp_control_head());
    while &mut (*ctrl).head != &mut (*sigmadsp).ctrl_list {
        sigmadsp_activate_ctrl(sigmadsp, ctrl, samplerate_mask);
        ctrl = list_entry((*ctrl).head.next, offset_sigmadsp_control_head());
    }

    (*sigmadsp).current_samplerate = samplerate;

    0
}
// EXPORT_SYMBOL_GPL(sigmadsp_setup);

/**
 * sigmadsp_reset() - Notify the sigmadsp instance that the DSP has been reset
 * @sigmadsp: The sigmadsp instance to reset
 *
 * Should be called whenever the DSP has been reset and parameter and program
 * memory need to be re-loaded.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigmadsp_reset(sigmadsp: *mut sigmadsp) {
    let mut ctrl: *mut sigmadsp_control;

    ctrl = list_entry((*sigmadsp).ctrl_list.next, offset_sigmadsp_control_head());
    while &mut (*ctrl).head != &mut (*sigmadsp).ctrl_list {
        sigmadsp_activate_ctrl(sigmadsp, ctrl, false as c_uint);
        ctrl = list_entry((*ctrl).head.next, offset_sigmadsp_control_head());
    }

    (*sigmadsp).current_samplerate = 0;
}
// EXPORT_SYMBOL_GPL(sigmadsp_reset);

/**
 * sigmadsp_restrict_params() - Applies DSP firmware specific constraints
 * @sigmadsp: The sigmadsp instance
 * @substream: The substream to restrict
 *
 * Applies samplerate constraints that may be required by the firmware Should
 * typically be called from the CODEC/component drivers startup callback.
 *
 * Returns 0 on success, a negative error code otherwise.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigmadsp_restrict_params(
    sigmadsp: *mut sigmadsp,
    substream: *mut snd_pcm_substream,
) -> c_int {
    if (*sigmadsp).rate_constraints.count == 0 {
        return 0;
    }

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*sigmadsp).rate_constraints,
    )
}
// EXPORT_SYMBOL_GPL(sigmadsp_restrict_params);

// MODULE_DESCRIPTION("Analog Devices SigmaStudio firmware helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
