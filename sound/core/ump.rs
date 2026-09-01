// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Universal MIDI Packet (UMP) support
 *
 * Source-level Rust translation of core/ump.c.  Linux/ALSA types, constants,
 * list helpers, locking helpers, user-copy helpers, rawmidi helpers, UMP
 * accessors, and logging/export/module macros are external dependencies from
 * the original C translation unit.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u32 = u32;
type size_t = usize;
type bool_t = bool;

const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOTTY: c_int = 25;
const ETIMEDOUT: c_int = 110;
const EFAULT: c_int = 14;
const ENOENT: c_int = 2;

const SNDRV_UMP_MAX_BLOCKS: c_uint = 32;
const SNDRV_UMP_MAX_GROUPS: c_int = 16;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_UMP: c_uint = 0x0008;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x0001;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x0002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x0004;
const SNDRV_RAWMIDI_LFLG_OUTPUT: c_int = 1;
const SNDRV_RAWMIDI_LFLG_APPEND: c_int = 2;
const SINGLE_DEPTH_NESTING: c_int = 0;
const SNDRV_SEQ_DEV_ID_UMP: c_int = 0;
const SNDRV_UMP_IOCTL_ENDPOINT_INFO: c_uint = 0;
const SNDRV_UMP_IOCTL_BLOCK_INFO: c_uint = 1;
const SNDRV_UMP_DIR_INPUT: c_int = 1;
const SNDRV_UMP_DIR_OUTPUT: c_int = 2;
const SNDRV_UMP_DIR_BIDIRECTION: c_int = 3;
const SNDRV_UMP_BLOCK_UI_HINT_RECEIVER: c_int = 1;
const SNDRV_UMP_BLOCK_UI_HINT_SENDER: c_int = 2;
const SNDRV_UMP_BLOCK_UI_HINT_BOTH: c_int = 3;
const SNDRV_UMP_EP_INFO_STATIC_BLOCKS: c_uint = 1 << 0;
const SNDRV_UMP_BLOCK_IS_MIDI1: c_uint = 1 << 0;
const SNDRV_UMP_BLOCK_IS_LOWSPEED: c_uint = 1 << 1;
const SNDRV_UMP_EP_INFO_PROTO_MIDI1: c_uint = 1 << 8;
const SNDRV_UMP_EP_INFO_PROTO_MIDI2: c_uint = 1 << 9;
const SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK: c_uint =
    SNDRV_UMP_EP_INFO_PROTO_MIDI1 | SNDRV_UMP_EP_INFO_PROTO_MIDI2;
const UMP_MSG_TYPE_STREAM: c_uint = 0xf;
const UMP_STREAM_MSG_FORMAT_SINGLE: c_uchar = 0;
const UMP_STREAM_MSG_FORMAT_START: c_uchar = 1;
const UMP_STREAM_MSG_FORMAT_END: c_uchar = 3;
const UMP_STREAM_MSG_STATUS_EP_DISCOVERY: c_uint = 0x00;
const UMP_STREAM_MSG_STATUS_EP_INFO: c_uint = 0x01;
const UMP_STREAM_MSG_STATUS_DEVICE_INFO: c_uint = 0x02;
const UMP_STREAM_MSG_STATUS_EP_NAME: c_uint = 0x03;
const UMP_STREAM_MSG_STATUS_PRODUCT_ID: c_uint = 0x04;
const UMP_STREAM_MSG_STATUS_STREAM_CFG: c_uint = 0x05;
const UMP_STREAM_MSG_STATUS_FB_DISCOVERY: c_uint = 0x10;
const UMP_STREAM_MSG_STATUS_FB_INFO: c_uint = 0x11;
const UMP_STREAM_MSG_STATUS_FB_NAME: c_uint = 0x12;
const UMP_STREAM_MSG_REQUEST_EP_INFO: c_uint = 1 << 8;
const UMP_STREAM_MSG_REQUEST_DEVICE_INFO: c_uint = 1 << 9;
const UMP_STREAM_MSG_REQUEST_EP_NAME: c_uint = 1 << 10;
const UMP_STREAM_MSG_REQUEST_PRODUCT_ID: c_uint = 1 << 11;
const UMP_STREAM_MSG_REQUEST_STREAM_CFG: c_uint = 1 << 12;
const UMP_STREAM_MSG_REQUEST_FB_INFO: c_uint = 1 << 0;
const UMP_STREAM_MSG_REQUEST_FB_NAME: c_uint = 1 << 1;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    number: c_int,
}

#[repr(C)]
pub struct snd_info_entry {
    private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_device {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
}

#[repr(C)]
pub struct snd_rawmidi_file {
    output: *mut snd_rawmidi_substream,
}

#[repr(C)]
pub struct snd_rawmidi_stream {
    substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    ops: *const snd_rawmidi_global_ops,
    card: *mut snd_card,
    device: c_int,
    info_flags: c_uint,
    name: [c_char; 80],
    streams: [snd_rawmidi_stream; 2],
    dev: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    rmidi: *mut snd_rawmidi,
    stream: c_int,
    number: c_int,
    list: list_head,
    name: [c_char; 32],
    inactive: bool_t,
}

#[repr(C)]
pub struct snd_rawmidi_global_ops {
    dev_register: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    dev_unregister: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    ioctl: Option<unsafe extern "C" fn(*mut snd_rawmidi, c_uint, *mut c_void) -> c_long>,
    proc_read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_ump_ops {
    open: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, c_int) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, c_int)>,
    trigger: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, c_int, c_int)>,
    drain: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, c_int)>,
}

#[repr(C)]
pub struct snd_ump_seq_ops {
    input_receive: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, *const u32, c_int)>,
    notify_ep_change: Option<unsafe extern "C" fn(*mut snd_ump_endpoint)>,
    switch_protocol: Option<unsafe extern "C" fn(*mut snd_ump_endpoint)>,
    notify_fb_change: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, *mut snd_ump_block)>,
}

#[repr(C)]
pub struct snd_ump_endpoint_info {
    card: c_int,
    device: c_int,
    flags: c_uint,
    protocol_caps: c_uint,
    protocol: c_uint,
    version: c_uint,
    num_blocks: c_uint,
    manufacturer_id: c_uint,
    family_id: c_uint,
    model_id: c_uint,
    sw_revision: [c_uchar; 4],
    name: [c_char; 128],
    product_id: [c_char; 128],
}

#[repr(C)]
pub struct snd_ump_block_info {
    card: c_int,
    device: c_int,
    block_id: c_uint,
    direction: c_uint,
    active: c_uint,
    first_group: c_uint,
    num_groups: c_uint,
    flags: c_uint,
    midi_ci_version: c_uint,
    sysex8_streams: c_uint,
    ui_hint: c_uint,
    name: [c_char; 128],
}

#[repr(C)]
pub struct snd_ump_group {
    name: [c_char; 128],
    dir_bits: c_uint,
    active: c_int,
    group: c_int,
    valid: bool_t,
    is_midi1: bool_t,
}

#[repr(C)]
pub struct ump_cvt_to_ump {
    ump_bytes: c_int,
    ump: [u8; 16],
}

#[repr(C)]
pub struct snd_ump_block {
    list: list_head,
    ump: *mut snd_ump_endpoint,
    info: snd_ump_block_info,
    private_free: Option<unsafe extern "C" fn(*mut snd_ump_block)>,
}

#[repr(C)]
pub struct snd_ump_endpoint {
    core: snd_rawmidi,
    block_list: list_head,
    open_mutex: mutex,
    stream_wait: wait_queue_head_t,
    legacy_locks: [spinlock_t; 2],
    ops: *mut snd_ump_ops,
    seq_ops: *mut snd_ump_seq_ops,
    seq_dev: *mut snd_seq_device,
    substreams: [*mut snd_rawmidi_substream; 2],
    private_free: Option<unsafe extern "C" fn(*mut snd_ump_endpoint)>,
    info: snd_ump_endpoint_info,
    input_buf: [u32; 4],
    input_buf_head: c_int,
    input_pending: c_int,
    stream_rfile: snd_rawmidi_file,
    stream_wait_for: c_uint,
    stream_finished: c_int,
    parsed: bool_t,
    no_process_stream: bool_t,
    groups: [snd_ump_group; 16],
    legacy_substreams: [[*mut snd_rawmidi_substream; 16]; 2],
    legacy_mapping: [c_int; 16],
    legacy_out_opens: c_int,
    legacy_out_rfile: snd_rawmidi_file,
    legacy_rmidi: *mut snd_rawmidi,
    out_cvts: *mut ump_cvt_to_ump,
}

#[repr(C)]
pub struct snd_ump_stream_msg_ep_info {
    raw0: u32,
    ump_version_major: c_uint,
    ump_version_minor: c_uint,
    num_function_blocks: c_uint,
    static_function_block: c_uint,
    protocol: c_uint,
    jrts: c_uint,
}

#[repr(C)]
pub struct snd_ump_stream_msg_device_info {
    raw0: u32,
    manufacture_id: c_uint,
    family_msb: c_uint,
    family_lsb: c_uint,
    model_msb: c_uint,
    model_lsb: c_uint,
    sw_revision: c_uint,
}

#[repr(C)]
pub struct snd_ump_stream_msg_stream_cfg {
    raw0: u32,
    protocol: c_uint,
    jrts: c_uint,
}

#[repr(C)]
pub struct snd_ump_stream_msg_fb_info {
    raw0: u32,
    status: c_uint,
    function_block_id: c_uchar,
    direction: c_uchar,
    ui_hint: c_uchar,
    first_group: c_uchar,
    num_groups: c_uchar,
    midi_10: c_uchar,
    active: c_uchar,
    midi_ci_version: c_uchar,
    sysex8_streams: c_uchar,
}

#[repr(C)]
pub struct snd_ump_stream_msg_fb_name {
    raw0: u32,
    function_block_id: c_uchar,
}

#[repr(C)]
pub union snd_ump_stream_msg {
    raw: [u32; 4],
    ep_info: core::mem::ManuallyDrop<snd_ump_stream_msg_ep_info>,
    device_info: core::mem::ManuallyDrop<snd_ump_stream_msg_device_info>,
    stream_cfg: core::mem::ManuallyDrop<snd_ump_stream_msg_stream_cfg>,
    fb_info: core::mem::ManuallyDrop<snd_ump_stream_msg_fb_info>,
    fb_name: core::mem::ManuallyDrop<snd_ump_stream_msg_fb_name>,
}

unsafe extern "C" {
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(list: *const list_head) -> c_int;
    fn list_del(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn snd_rawmidi_init(
        rmidi: *mut snd_rawmidi,
        card: *mut snd_card,
        id: *mut c_char,
        device: c_int,
        output: c_int,
        input: c_int,
        info_flags: c_uint,
    ) -> c_int;
    fn snd_rawmidi_free(rmidi: *mut snd_rawmidi);
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *const c_char, count: c_int) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: c_int) -> c_int;
    fn snd_rawmidi_kernel_open(rmidi: *mut snd_rawmidi, subdevice: c_int, mode: c_int, rfile: *mut snd_rawmidi_file) -> c_int;
    fn snd_rawmidi_kernel_open_nested(rmidi: *mut snd_rawmidi, subdevice: c_int, mode: c_int, rfile: *mut snd_rawmidi_file, subclass: c_int) -> c_int;
    fn snd_rawmidi_kernel_release(rfile: *mut snd_rawmidi_file);
    fn snd_rawmidi_kernel_release_nested(rfile: *mut snd_rawmidi_file, subclass: c_int);
    fn snd_rawmidi_kernel_write(substream: *mut snd_rawmidi_substream, buf: *mut c_uchar, count: c_int) -> c_int;
    fn snd_rawmidi_new(card: *mut snd_card, id: *mut c_char, device: c_int, output_count: c_int, input_count: c_int, rmidi_ret: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_tie_devices(rmidi: *mut snd_rawmidi, rmidi2: *mut snd_rawmidi);
    fn snd_seq_device_new(card: *mut snd_card, device: c_int, id: c_int, arg: c_int, result: *mut *mut snd_seq_device) -> c_int;
    fn snd_device_register(card: *mut snd_card, device: *mut snd_seq_device) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn copy_to_user(dst: *mut c_void, src: *const c_void, size: size_t) -> c_int;
    fn get_user_u8(dst: *mut c_uchar, src: *const c_uchar) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strlcat(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: c_int, timeout: c_long) -> c_long;
    fn msecs_to_jiffies(msecs: c_uint) -> c_long;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn ump_message_type(val: u32) -> c_uint;
    fn ump_stream_message_format(val: u32) -> c_uchar;
    fn ump_stream_message_status(val: u32) -> c_uint;
    fn ump_stream_compose(status: c_uint, format: c_uint) -> u32;
    fn snd_ump_convert_reset(ctx: *mut ump_cvt_to_ump);
    fn snd_ump_convert_to_ump(ctx: *mut ump_cvt_to_ump, group: c_int, protocol: c_uint, c: c_uchar);
    fn snd_ump_convert_from_ump(src: *const u32, buf: *mut c_uchar, group: *mut c_uchar) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const CONFIG_SND_SEQUENCER_ENABLED: bool = true;
const CONFIG_SND_UMP_LEGACY_RAWMIDI_ENABLED: bool = true;

unsafe fn rawmidi_to_ump(rmidi: *mut snd_rawmidi) -> *mut snd_ump_endpoint {
    rmidi as *mut snd_ump_endpoint
}

unsafe fn container_of_block_list(ptr: *mut list_head) -> *mut snd_ump_block {
    (ptr as *mut u8).sub(offset_of!(snd_ump_block, list)) as *mut snd_ump_block
}

unsafe fn container_of_substream_list(ptr: *mut list_head) -> *mut snd_rawmidi_substream {
    (ptr as *mut u8).sub(offset_of!(snd_rawmidi_substream, list)) as *mut snd_rawmidi_substream
}

unsafe fn isascii(c: c_uchar) -> bool {
    c <= 0x7f
}

unsafe fn isprint(c: c_uchar) -> bool {
    c >= 0x20 && c < 0x7f
}

unsafe fn safe_copy_string(dst: *mut c_void, mut max_dst_size: size_t, src: *const c_void, mut max_src_size: size_t) {
    let mut s = src as *const c_uchar;
    let mut d = dst as *mut c_uchar;

    if max_dst_size == 0 {
        return;
    }
    max_dst_size -= 1;
    while max_dst_size != 0 && *s != 0 && {
        let old = max_src_size;
        max_src_size = max_src_size.wrapping_sub(1);
        old != 0
    } {
        if !isascii(*s) || !isprint(*s) {
            s = s.add(1);
            continue;
        }
        *d = *s;
        d = d.add(1);
        max_dst_size -= 1;
        s = s.add(1);
    }
    *d = 0;
}

unsafe fn safe_append_string(dst: *mut c_void, max_dst_size: size_t, src: *const c_void, max_src_size: size_t) {
    let d = dst as *mut c_uchar;
    let len = strlen(d as *const c_char);
    safe_copy_string(d.add(len) as *mut c_void, max_dst_size - len, src, max_src_size);
}

static snd_ump_rawmidi_ops: snd_rawmidi_global_ops = snd_rawmidi_global_ops {
    dev_register: Some(snd_ump_dev_register),
    dev_unregister: Some(snd_ump_dev_unregister),
    ioctl: Some(snd_ump_ioctl),
    proc_read: Some(snd_ump_proc_read),
};

static snd_ump_rawmidi_input_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_ump_rawmidi_open),
    close: Some(snd_ump_rawmidi_close),
    trigger: Some(snd_ump_rawmidi_trigger),
    drain: None,
};

static snd_ump_rawmidi_output_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_ump_rawmidi_open),
    close: Some(snd_ump_rawmidi_close),
    trigger: Some(snd_ump_rawmidi_trigger),
    drain: Some(snd_ump_rawmidi_drain),
};

unsafe extern "C" fn snd_ump_endpoint_free(rmidi: *mut snd_rawmidi) {
    let ump = rawmidi_to_ump(rmidi);
    while list_empty(&mut (*ump).block_list) == 0 {
        let fb = container_of_block_list((*ump).block_list.next);
        list_del(&mut (*fb).list);
        if let Some(private_free) = (*fb).private_free {
            private_free(fb);
        }
        kfree(fb as *mut c_void);
    }
    if let Some(private_free) = (*ump).private_free {
        private_free(ump);
    }
    if CONFIG_SND_UMP_LEGACY_RAWMIDI_ENABLED {
        kfree((*ump).out_cvts as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_endpoint_new(
    card: *mut snd_card,
    id: *mut c_char,
    device: c_int,
    output: c_int,
    input: c_int,
    ump_ret: *mut *mut snd_ump_endpoint,
) -> c_int {
    let mut info_flags = SNDRV_RAWMIDI_INFO_UMP;
    let mut err: c_int;

    if input != 0 {
        info_flags |= SNDRV_RAWMIDI_INFO_INPUT;
    }
    if output != 0 {
        info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;
    }
    if input != 0 && output != 0 {
        info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
    }

    let ump = kzalloc(size_of::<snd_ump_endpoint>(), GFP_KERNEL) as *mut snd_ump_endpoint;
    if ump.is_null() {
        return -ENOMEM;
    }
    INIT_LIST_HEAD(&mut (*ump).block_list);
    mutex_init(&mut (*ump).open_mutex);
    init_waitqueue_head(&mut (*ump).stream_wait);
    if CONFIG_SND_UMP_LEGACY_RAWMIDI_ENABLED {
        spin_lock_init(&mut (*ump).legacy_locks[0]);
        spin_lock_init(&mut (*ump).legacy_locks[1]);
    }
    err = snd_rawmidi_init(&mut (*ump).core, card, id, device, output, input, info_flags);
    if err < 0 {
        snd_rawmidi_free(&mut (*ump).core);
        return err;
    }
    (*ump).info.card = (*card).number;
    (*ump).info.device = device;
    (*ump).core.private_free = Some(snd_ump_endpoint_free);
    (*ump).core.ops = &snd_ump_rawmidi_ops;
    if input != 0 {
        snd_rawmidi_set_ops(&mut (*ump).core, SNDRV_RAWMIDI_STREAM_INPUT, &snd_ump_rawmidi_input_ops);
    }
    if output != 0 {
        snd_rawmidi_set_ops(&mut (*ump).core, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_ump_rawmidi_output_ops);
    }
    *ump_ret = ump;
    0
}

unsafe extern "C" fn snd_ump_dev_seq_free(device: *mut snd_seq_device) {
    let ump = (*device).private_data as *mut snd_ump_endpoint;
    (*ump).seq_dev = ptr::null_mut();
}

unsafe extern "C" fn snd_ump_dev_register(rmidi: *mut snd_rawmidi) -> c_int {
    if CONFIG_SND_SEQUENCER_ENABLED {
        let ump = rawmidi_to_ump(rmidi);
        let err = snd_seq_device_new((*ump).core.card, (*ump).core.device, SNDRV_SEQ_DEV_ID_UMP, 0, &mut (*ump).seq_dev);
        if err < 0 {
            return err;
        }
        (*(*ump).seq_dev).private_data = ump as *mut c_void;
        (*(*ump).seq_dev).private_free = Some(snd_ump_dev_seq_free);
        snd_device_register((*ump).core.card, (*ump).seq_dev);
    }
    0
}

unsafe extern "C" fn snd_ump_dev_unregister(_rmidi: *mut snd_rawmidi) -> c_int {
    0
}

unsafe fn snd_ump_get_block(ump: *mut snd_ump_endpoint, id: c_uchar) -> *mut snd_ump_block {
    let mut pos = (*ump).block_list.next;
    while pos != &mut (*ump).block_list {
        let fb = container_of_block_list(pos);
        if (*fb).info.block_id == id as c_uint {
            return fb;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

unsafe extern "C" fn snd_ump_rawmidi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let ump = rawmidi_to_ump((*substream).rmidi);
    let dir = (*substream).stream;
    if !(*ump).substreams[dir as usize].is_null() {
        return -EBUSY;
    }
    let err = ((*(*ump).ops).open.unwrap())(ump, dir);
    if err < 0 {
        return err;
    }
    (*ump).substreams[dir as usize] = substream;
    0
}

unsafe extern "C" fn snd_ump_rawmidi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let ump = rawmidi_to_ump((*substream).rmidi);
    let dir = (*substream).stream;
    (*ump).substreams[dir as usize] = ptr::null_mut();
    ((*(*ump).ops).close.unwrap())(ump, dir);
    0
}

unsafe extern "C" fn snd_ump_rawmidi_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let ump = rawmidi_to_ump((*substream).rmidi);
    let dir = (*substream).stream;
    ((*(*ump).ops).trigger.unwrap())(ump, dir, up);
}

unsafe extern "C" fn snd_ump_rawmidi_drain(substream: *mut snd_rawmidi_substream) {
    let ump = rawmidi_to_ump((*substream).rmidi);
    if let Some(drain) = (*(*ump).ops).drain {
        drain(ump, SNDRV_RAWMIDI_STREAM_OUTPUT);
    }
}

static mut ump_packet_words: [c_uchar; 0x10] = [1, 1, 1, 2, 2, 4, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_receive_ump_val(ump: *mut snd_ump_endpoint, val: u32) -> c_int {
    if (*ump).input_pending == 0 {
        (*ump).input_pending = ump_packet_words[ump_message_type(val) as usize] as c_int;
    }
    (*ump).input_buf[(*ump).input_buf_head as usize] = val;
    (*ump).input_buf_head += 1;
    (*ump).input_pending -= 1;
    if (*ump).input_pending == 0 {
        let words = (*ump).input_buf_head;
        (*ump).input_buf_head = 0;
        return words;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_receive(ump: *mut snd_ump_endpoint, buffer: *const u32, count: c_int) -> c_int {
    let mut p = buffer;
    let mut words = count >> 2;
    while words != 0 {
        words -= 1;
        let n = snd_ump_receive_ump_val(ump, *p);
        p = p.add(1);
        if n == 0 {
            continue;
        }
        ump_handle_stream_msg(ump, (*ump).input_buf.as_ptr(), n);
        if CONFIG_SND_SEQUENCER_ENABLED && !(*ump).seq_ops.is_null() {
            if let Some(input_receive) = (*(*ump).seq_ops).input_receive {
                input_receive(ump, (*ump).input_buf.as_ptr(), n);
            }
        }
        process_legacy_input(ump, (*ump).input_buf.as_ptr(), n);
    }
    let substream = (*ump).substreams[SNDRV_RAWMIDI_STREAM_INPUT as usize];
    if substream.is_null() {
        return 0;
    }
    snd_rawmidi_receive(substream, buffer as *const c_char, count)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_transmit(ump: *mut snd_ump_endpoint, buffer: *mut u32, count: c_int) -> c_int {
    let substream = (*ump).substreams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize];
    if substream.is_null() {
        return -ENODEV;
    }
    let err = snd_rawmidi_transmit(substream, buffer as *mut c_uchar, count);
    if err != 0 {
        return err;
    }
    process_legacy_output(ump, buffer, count)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_block_new(
    ump: *mut snd_ump_endpoint,
    blk: c_uint,
    direction: c_uint,
    first_group: c_uint,
    num_groups: c_uint,
    blk_ret: *mut *mut snd_ump_block,
) -> c_int {
    if blk >= SNDRV_UMP_MAX_BLOCKS {
        return -EINVAL;
    }
    if !snd_ump_get_block(ump, blk as c_uchar).is_null() {
        return -EBUSY;
    }
    let fb = kzalloc(size_of::<snd_ump_block>(), GFP_KERNEL) as *mut snd_ump_block;
    if fb.is_null() {
        return -ENOMEM;
    }
    (*fb).ump = ump;
    (*fb).info.card = (*ump).info.card;
    (*fb).info.device = (*ump).info.device;
    (*fb).info.block_id = blk;
    if blk >= (*ump).info.num_blocks {
        (*ump).info.num_blocks = blk + 1;
    }
    (*fb).info.direction = direction;
    (*fb).info.active = 1;
    (*fb).info.first_group = first_group;
    (*fb).info.num_groups = num_groups;
    snprintf((*fb).info.name.as_mut_ptr(), (*fb).info.name.len(), c"Group %u-%u".as_ptr(), first_group + 1, first_group + num_groups);

    let mut pos = (*ump).block_list.next;
    while pos != &mut (*ump).block_list {
        let p = container_of_block_list(pos);
        if (*p).info.block_id > blk {
            list_add_tail(&mut (*fb).list, &mut (*p).list);
            *blk_ret = fb;
            return 0;
        }
        pos = (*pos).next;
    }
    list_add_tail(&mut (*fb).list, &mut (*ump).block_list);
    *blk_ret = fb;
    0
}

unsafe fn snd_ump_ioctl_block(ump: *mut snd_ump_endpoint, argp: *mut snd_ump_block_info) -> c_int {
    let mut id: c_uchar = 0;
    if get_user_u8(&mut id, &mut (*argp).block_id as *mut c_uint as *const c_uchar) != 0 {
        return -EFAULT;
    }
    let fb = snd_ump_get_block(ump, id);
    if fb.is_null() {
        return -ENOENT;
    }
    if copy_to_user(argp as *mut c_void, &(*fb).info as *const _ as *const c_void, size_of::<snd_ump_block_info>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe extern "C" fn snd_ump_ioctl(rmidi: *mut snd_rawmidi, cmd: c_uint, argp: *mut c_void) -> c_long {
    let ump = rawmidi_to_ump(rmidi);
    match cmd {
        SNDRV_UMP_IOCTL_ENDPOINT_INFO => {
            if copy_to_user(argp, &(*ump).info as *const _ as *const c_void, size_of::<snd_ump_endpoint_info>()) != 0 {
                return -EFAULT as c_long;
            }
            0
        }
        SNDRV_UMP_IOCTL_BLOCK_INFO => snd_ump_ioctl_block(ump, argp as *mut snd_ump_block_info) as c_long,
        _ => -ENOTTY as c_long,
    }
}

unsafe fn ump_direction_string(dir: c_int) -> *const c_char {
    match dir {
        SNDRV_UMP_DIR_INPUT => c"input".as_ptr(),
        SNDRV_UMP_DIR_OUTPUT => c"output".as_ptr(),
        SNDRV_UMP_DIR_BIDIRECTION => c"bidirection".as_ptr(),
        _ => c"unknown".as_ptr(),
    }
}

unsafe fn ump_ui_hint_string(dir: c_int) -> *const c_char {
    match dir {
        SNDRV_UMP_BLOCK_UI_HINT_RECEIVER => c"receiver".as_ptr(),
        SNDRV_UMP_BLOCK_UI_HINT_SENDER => c"sender".as_ptr(),
        SNDRV_UMP_BLOCK_UI_HINT_BOTH => c"both".as_ptr(),
        _ => c"unknown".as_ptr(),
    }
}

unsafe extern "C" fn snd_ump_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let rmidi = (*entry).private_data as *mut snd_rawmidi;
    let ump = rawmidi_to_ump(rmidi);
    snd_iprintf(buffer, c"EP Name: %s\n".as_ptr(), (*ump).info.name.as_ptr());
    snd_iprintf(buffer, c"EP Product ID: %s\n".as_ptr(), (*ump).info.product_id.as_ptr());
    snd_iprintf(buffer, c"UMP Version: 0x%04x\n".as_ptr(), (*ump).info.version);
    snd_iprintf(buffer, c"Protocol Caps: 0x%08x\n".as_ptr(), (*ump).info.protocol_caps);
    snd_iprintf(buffer, c"Protocol: 0x%08x\n".as_ptr(), (*ump).info.protocol);
    if (*ump).info.version != 0 {
        snd_iprintf(buffer, c"Manufacturer ID: 0x%08x\n".as_ptr(), (*ump).info.manufacturer_id);
        snd_iprintf(buffer, c"Family ID: 0x%04x\n".as_ptr(), (*ump).info.family_id);
        snd_iprintf(buffer, c"Model ID: 0x%04x\n".as_ptr(), (*ump).info.model_id);
        snd_iprintf(buffer, c"SW Revision: 0x%4phN\n".as_ptr(), (*ump).info.sw_revision.as_ptr());
    }
    snd_iprintf(buffer, c"Static Blocks: %s\n".as_ptr(), if ((*ump).info.flags & SNDRV_UMP_EP_INFO_STATIC_BLOCKS) != 0 { c"Yes".as_ptr() } else { c"No".as_ptr() });
    snd_iprintf(buffer, c"Num Blocks: %d\n\n".as_ptr(), (*ump).info.num_blocks);
    let mut pos = (*ump).block_list.next;
    while pos != &mut (*ump).block_list {
        let fb = container_of_block_list(pos);
        snd_iprintf(buffer, c"Block %d (%s)\n".as_ptr(), (*fb).info.block_id, (*fb).info.name.as_ptr());
        snd_iprintf(buffer, c"  Direction: %s\n".as_ptr(), ump_direction_string((*fb).info.direction as c_int));
        snd_iprintf(buffer, c"  Active: %s\n".as_ptr(), if (*fb).info.active != 0 { c"Yes".as_ptr() } else { c"No".as_ptr() });
        snd_iprintf(buffer, c"  Groups: %d-%d\n".as_ptr(), (*fb).info.first_group + 1, (*fb).info.first_group + (*fb).info.num_groups);
        snd_iprintf(buffer, c"  Is MIDI1: %s%s\n".as_ptr(), if ((*fb).info.flags & SNDRV_UMP_BLOCK_IS_MIDI1) != 0 { c"Yes".as_ptr() } else { c"No".as_ptr() }, if ((*fb).info.flags & SNDRV_UMP_BLOCK_IS_LOWSPEED) != 0 { c" (Low Speed)".as_ptr() } else { c"".as_ptr() });
        if (*ump).info.version != 0 {
            snd_iprintf(buffer, c"  MIDI-CI Version: %d\n".as_ptr(), (*fb).info.midi_ci_version);
            snd_iprintf(buffer, c"  Sysex8 Streams: %d\n".as_ptr(), (*fb).info.sysex8_streams);
            snd_iprintf(buffer, c"  UI Hint: %s\n".as_ptr(), ump_ui_hint_string((*fb).info.ui_hint as c_int));
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        pos = (*pos).next;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_update_group_attrs(ump: *mut snd_ump_endpoint) {
    for i in 0..SNDRV_UMP_MAX_GROUPS as usize {
        let group = &mut (*ump).groups[i];
        group.name[0] = 0;
        group.dir_bits = 0;
        group.active = 0;
        group.group = i as c_int;
        group.valid = false;
        group.is_midi1 = false;
    }
    let mut pos = (*ump).block_list.next;
    while pos != &mut (*ump).block_list {
        let fb = container_of_block_list(pos);
        if (*fb).info.first_group + (*fb).info.num_groups > SNDRV_UMP_MAX_GROUPS as c_uint {
            break;
        }
        for i in 0..(*fb).info.num_groups as usize {
            let idx = (*fb).info.first_group as usize + i;
            let group = &mut (*ump).groups[idx];
            group.valid = true;
            if (*fb).info.active != 0 {
                group.active = 1;
            }
            if ((*fb).info.flags & SNDRV_UMP_BLOCK_IS_MIDI1) != 0 {
                group.is_midi1 = true;
            }
            match (*fb).info.direction as c_int {
                SNDRV_UMP_DIR_INPUT => group.dir_bits |= 1 << SNDRV_RAWMIDI_STREAM_INPUT,
                SNDRV_UMP_DIR_OUTPUT => group.dir_bits |= 1 << SNDRV_RAWMIDI_STREAM_OUTPUT,
                SNDRV_UMP_DIR_BIDIRECTION => {
                    group.dir_bits |= (1 << SNDRV_RAWMIDI_STREAM_INPUT) | (1 << SNDRV_RAWMIDI_STREAM_OUTPUT);
                }
                _ => {}
            }
            if (*fb).info.name[0] == 0 {
                continue;
            }
            if group.name[0] != 0 {
                strlcat(group.name.as_mut_ptr(), c", ".as_ptr(), group.name.len());
            }
            safe_append_string(group.name.as_mut_ptr() as *mut c_void, group.name.len(), (*fb).info.name.as_ptr() as *const c_void, (*fb).info.name.len());
        }
        pos = (*pos).next;
    }
}

unsafe fn ump_request_open(ump: *mut snd_ump_endpoint) -> c_int {
    snd_rawmidi_kernel_open(&mut (*ump).core, 0, SNDRV_RAWMIDI_LFLG_OUTPUT, &mut (*ump).stream_rfile)
}

unsafe fn ump_request_close(ump: *mut snd_ump_endpoint) {
    snd_rawmidi_kernel_release(&mut (*ump).stream_rfile);
}

unsafe fn ump_req_msg(ump: *mut snd_ump_endpoint, req1: u32, req2: u32, reply: u32) -> c_int {
    let mut buf: [u32; 4] = [0; 4];
    memset(buf.as_mut_ptr() as *mut c_void, 0, size_of::<[u32; 4]>());
    buf[0] = req1;
    buf[1] = req2;
    (*ump).stream_finished = 0;
    (*ump).stream_wait_for = reply;
    snd_rawmidi_kernel_write((*ump).stream_rfile.output, buf.as_mut_ptr() as *mut c_uchar, 16);
    wait_event_timeout(&mut (*ump).stream_wait, (*ump).stream_finished, msecs_to_jiffies(500));
    if ptr::read_volatile(&(*ump).stream_finished) == 0 {
        return -ETIMEDOUT;
    }
    (*ump).stream_finished = 0;
    0
}

unsafe fn ump_append_string(_ump: *mut snd_ump_endpoint, dest: *mut c_char, maxsize: c_int, buf: *const u32, mut offset: c_int) -> c_int {
    let format = ump_stream_message_format(*buf);
    let mut c: c_int;
    if format == UMP_STREAM_MSG_FORMAT_SINGLE || format == UMP_STREAM_MSG_FORMAT_START {
        c = 0;
    } else {
        c = strlen(dest) as c_int;
        if c >= maxsize - 1 {
            return 1;
        }
    }
    while offset < 16 {
        *dest.add(c as usize) = ((*buf.add((offset / 4) as usize) >> ((3 - (offset % 4)) * 8)) & 0xff) as c_char;
        if *dest.add(c as usize) == 0 {
            break;
        }
        c += 1;
        if c >= maxsize - 1 {
            break;
        }
        offset += 1;
    }
    *dest.add(c as usize) = 0;
    (format == UMP_STREAM_MSG_FORMAT_SINGLE || format == UMP_STREAM_MSG_FORMAT_END) as c_int
}

unsafe fn choose_default_protocol(ump: *mut snd_ump_endpoint) {
    if ((*ump).info.protocol & SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK) != 0 {
        return;
    }
    if ((*ump).info.protocol_caps & SNDRV_UMP_EP_INFO_PROTO_MIDI2) != 0 {
        (*ump).info.protocol |= SNDRV_UMP_EP_INFO_PROTO_MIDI2;
    } else {
        (*ump).info.protocol |= SNDRV_UMP_EP_INFO_PROTO_MIDI1;
    }
}

unsafe fn seq_notify_ep_change(ump: *mut snd_ump_endpoint) {
    if CONFIG_SND_SEQUENCER_ENABLED && (*ump).parsed && !(*ump).seq_ops.is_null() {
        if let Some(notify_ep_change) = (*(*ump).seq_ops).notify_ep_change {
            notify_ep_change(ump);
        }
    }
}

unsafe fn ump_handle_ep_info_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    (*ump).info.version = ((*buf).ep_info.ump_version_major << 8) | (*buf).ep_info.ump_version_minor;
    (*ump).info.num_blocks = (*buf).ep_info.num_function_blocks;
    if (*ump).info.num_blocks > SNDRV_UMP_MAX_BLOCKS {
        (*ump).info.num_blocks = 1;
    }
    if (*buf).ep_info.static_function_block != 0 {
        (*ump).info.flags |= SNDRV_UMP_EP_INFO_STATIC_BLOCKS;
    }
    (*ump).info.protocol_caps = ((*buf).ep_info.protocol << 8) | (*buf).ep_info.jrts;
    (*ump).info.protocol &= (*ump).info.protocol_caps;
    choose_default_protocol(ump);
    seq_notify_ep_change(ump);
    1
}

unsafe fn ump_handle_device_info_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    (*ump).info.manufacturer_id = (*buf).device_info.manufacture_id & 0x7f7f7f;
    (*ump).info.family_id = ((*buf).device_info.family_msb << 8) | (*buf).device_info.family_lsb;
    (*ump).info.model_id = ((*buf).device_info.model_msb << 8) | (*buf).device_info.model_lsb;
    (*ump).info.sw_revision[0] = (((*buf).device_info.sw_revision >> 24) & 0x7f) as c_uchar;
    (*ump).info.sw_revision[1] = (((*buf).device_info.sw_revision >> 16) & 0x7f) as c_uchar;
    (*ump).info.sw_revision[2] = (((*buf).device_info.sw_revision >> 8) & 0x7f) as c_uchar;
    (*ump).info.sw_revision[3] = ((*buf).device_info.sw_revision & 0x7f) as c_uchar;
    seq_notify_ep_change(ump);
    1
}

unsafe fn ump_set_rawmidi_name(ump: *mut snd_ump_endpoint) {
    safe_copy_string((*ump).core.name.as_mut_ptr() as *mut c_void, (*ump).core.name.len(), (*ump).info.name.as_ptr() as *const c_void, (*ump).info.name.len());
}

unsafe fn ump_handle_ep_name_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    let ret = ump_append_string(ump, (*ump).info.name.as_mut_ptr(), (*ump).info.name.len() as c_int, (*buf).raw.as_ptr(), 2);
    if ret != 0 && (*ump).parsed {
        ump_set_rawmidi_name(ump);
        ump_legacy_set_rawmidi_name(ump);
        seq_notify_ep_change(ump);
    }
    ret
}

unsafe fn ump_handle_product_id_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    let ret = ump_append_string(ump, (*ump).info.product_id.as_mut_ptr(), (*ump).info.product_id.len() as c_int, (*buf).raw.as_ptr(), 2);
    if ret != 0 {
        seq_notify_ep_change(ump);
    }
    ret
}

unsafe fn seq_notify_protocol(ump: *mut snd_ump_endpoint) {
    if CONFIG_SND_SEQUENCER_ENABLED && !(*ump).seq_ops.is_null() {
        if let Some(switch_protocol) = (*(*ump).seq_ops).switch_protocol {
            switch_protocol(ump);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_switch_protocol(ump: *mut snd_ump_endpoint, mut protocol: c_uint) -> c_int {
    protocol &= (*ump).info.protocol_caps;
    if protocol == (*ump).info.protocol {
        return 0;
    }
    let type_ = protocol & SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK;
    if type_ != SNDRV_UMP_EP_INFO_PROTO_MIDI1 && type_ != SNDRV_UMP_EP_INFO_PROTO_MIDI2 {
        return 0;
    }
    (*ump).info.protocol = protocol;
    seq_notify_protocol(ump);
    1
}

unsafe fn ump_handle_stream_cfg_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    let protocol = ((*buf).stream_cfg.protocol << 8) | (*buf).stream_cfg.jrts;
    snd_ump_switch_protocol(ump, protocol);
    1
}

unsafe fn fill_fb_info(_ump: *mut snd_ump_endpoint, info: *mut snd_ump_block_info, buf: *const snd_ump_stream_msg) {
    (*info).direction = (*buf).fb_info.direction as c_uint;
    (*info).ui_hint = (*buf).fb_info.ui_hint as c_uint;
    (*info).first_group = (*buf).fb_info.first_group as c_uint;
    (*info).num_groups = (*buf).fb_info.num_groups as c_uint;
    if (*buf).fb_info.midi_10 < 2 {
        (*info).flags = (*buf).fb_info.midi_10 as c_uint;
    } else {
        (*info).flags = SNDRV_UMP_BLOCK_IS_MIDI1 | SNDRV_UMP_BLOCK_IS_LOWSPEED;
    }
    (*info).active = (*buf).fb_info.active as c_uint;
    (*info).midi_ci_version = (*buf).fb_info.midi_ci_version as c_uint;
    (*info).sysex8_streams = (*buf).fb_info.sysex8_streams as c_uint;
    if ((*info).flags & SNDRV_UMP_BLOCK_IS_MIDI1) != 0 && (*info).num_groups != 1 {
        (*info).num_groups = 1;
    }
}

unsafe fn is_fb_info_updated(ump: *mut snd_ump_endpoint, fb: *mut snd_ump_block, buf: *const snd_ump_stream_msg) -> bool_t {
    let size = offset_of!(snd_ump_block_info, name);
    let mut tmpbuf = [0u8; 128];
    if ((*ump).info.flags & SNDRV_UMP_EP_INFO_STATIC_BLOCKS) != 0 {
        return false;
    }
    memcpy(tmpbuf.as_mut_ptr() as *mut c_void, &(*fb).info as *const _ as *const c_void, size);
    fill_fb_info(ump, tmpbuf.as_mut_ptr() as *mut snd_ump_block_info, buf);
    memcmp(&(*fb).info as *const _ as *const c_void, tmpbuf.as_ptr() as *const c_void, size) != 0
}

unsafe fn seq_notify_fb_change(ump: *mut snd_ump_endpoint, fb: *mut snd_ump_block) {
    if CONFIG_SND_SEQUENCER_ENABLED && !(*ump).seq_ops.is_null() {
        if let Some(notify_fb_change) = (*(*ump).seq_ops).notify_fb_change {
            notify_fb_change(ump, fb);
        }
    }
}

unsafe fn ump_handle_fb_info_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    let blk = (*buf).fb_info.function_block_id;
    let fb = snd_ump_get_block(ump, blk);
    if fb.is_null() && (*ump).parsed {
        return -ENODEV;
    }
    if (*ump).parsed && !is_fb_info_updated(ump, fb, buf) {
        return 1;
    }
    if !fb.is_null() {
        fill_fb_info(ump, &mut (*fb).info, buf);
        if (*ump).parsed {
            snd_ump_update_group_attrs(ump);
            update_legacy_names(ump);
            seq_notify_fb_change(ump, fb);
        }
    }
    1
}

unsafe fn ump_handle_fb_name_msg(ump: *mut snd_ump_endpoint, buf: *const snd_ump_stream_msg) -> c_int {
    let blk = (*buf).fb_name.function_block_id;
    let fb = snd_ump_get_block(ump, blk);
    if fb.is_null() {
        return -ENODEV;
    }
    if (*ump).parsed && ((*ump).info.flags & SNDRV_UMP_EP_INFO_STATIC_BLOCKS) != 0 {
        return 0;
    }
    let ret = ump_append_string(ump, (*fb).info.name.as_mut_ptr(), (*fb).info.name.len() as c_int, (*buf).raw.as_ptr(), 3);
    if ret > 0 && (*ump).parsed {
        snd_ump_update_group_attrs(ump);
        update_legacy_names(ump);
        seq_notify_fb_change(ump, fb);
    }
    ret
}

unsafe fn create_block_from_fb_info(ump: *mut snd_ump_endpoint, blk: c_int) -> c_int {
    let mut fb: *mut snd_ump_block = ptr::null_mut();
    let buf = (*ump).input_buf.as_ptr() as *const snd_ump_stream_msg;
    let mut msg = ump_stream_compose(UMP_STREAM_MSG_STATUS_FB_DISCOVERY, 0) | ((blk as u32) << 8) | UMP_STREAM_MSG_REQUEST_FB_INFO;
    let mut err = ump_req_msg(ump, msg, 0, UMP_STREAM_MSG_STATUS_FB_INFO);
    if err < 0 {
        return err;
    }
    if (*buf).fb_info.status != UMP_STREAM_MSG_STATUS_FB_INFO {
        return -EINVAL;
    }
    let direction = (*buf).fb_info.direction as c_uint;
    let first_group = (*buf).fb_info.first_group as c_uint;
    let num_groups = (*buf).fb_info.num_groups as c_uint;
    err = snd_ump_block_new(ump, blk as c_uint, direction, first_group, num_groups, &mut fb);
    if err < 0 {
        return err;
    }
    fill_fb_info(ump, &mut (*fb).info, buf);
    msg = ump_stream_compose(UMP_STREAM_MSG_STATUS_FB_DISCOVERY, 0) | ((blk as u32) << 8) | UMP_STREAM_MSG_REQUEST_FB_NAME;
    err = ump_req_msg(ump, msg, 0, UMP_STREAM_MSG_STATUS_FB_NAME);
    0
}

unsafe fn ump_handle_stream_msg(ump: *mut snd_ump_endpoint, buf: *const u32, size: c_int) {
    if (*ump).no_process_stream {
        return;
    }
    if size_of::<snd_ump_stream_msg>() != 16 {
        panic!("BUILD_BUG_ON(sizeof(*msg) != 16)");
    }
    if size != 4 || ump_message_type(*buf) != UMP_MSG_TYPE_STREAM {
        return;
    }
    let msg = buf as *const snd_ump_stream_msg;
    let status = ump_stream_message_status(*buf);
    let ret = match status {
        UMP_STREAM_MSG_STATUS_EP_INFO => ump_handle_ep_info_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_DEVICE_INFO => ump_handle_device_info_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_EP_NAME => ump_handle_ep_name_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_PRODUCT_ID => ump_handle_product_id_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_STREAM_CFG => ump_handle_stream_cfg_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_FB_INFO => ump_handle_fb_info_msg(ump, msg),
        UMP_STREAM_MSG_STATUS_FB_NAME => ump_handle_fb_name_msg(ump, msg),
        _ => return,
    };
    if ret > 0 && (*ump).stream_wait_for == status {
        ptr::write_volatile(&mut (*ump).stream_finished, 1);
        wake_up(&mut (*ump).stream_wait);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_parse_endpoint(ump: *mut snd_ump_endpoint) -> c_int {
    if ((*ump).core.info_flags & SNDRV_RAWMIDI_INFO_DUPLEX) == 0 {
        return -ENODEV;
    }
    let mut err = ump_request_open(ump);
    if err < 0 {
        return err;
    }
    let msg = ump_stream_compose(UMP_STREAM_MSG_STATUS_EP_DISCOVERY, 0) | 0x0101;
    err = ump_req_msg(ump, msg, UMP_STREAM_MSG_REQUEST_EP_INFO, UMP_STREAM_MSG_STATUS_EP_INFO);
    if err < 0 {
        (*ump).parsed = true;
        ump_request_close(ump);
        if err == -ETIMEDOUT {
            return -ENODEV;
        }
        return err;
    }
    err = ump_req_msg(ump, msg, UMP_STREAM_MSG_REQUEST_DEVICE_INFO, UMP_STREAM_MSG_STATUS_DEVICE_INFO);
    err = ump_req_msg(ump, msg, UMP_STREAM_MSG_REQUEST_EP_NAME, UMP_STREAM_MSG_STATUS_EP_NAME);
    ump_set_rawmidi_name(ump);
    err = ump_req_msg(ump, msg, UMP_STREAM_MSG_REQUEST_PRODUCT_ID, UMP_STREAM_MSG_STATUS_PRODUCT_ID);
    err = ump_req_msg(ump, msg, UMP_STREAM_MSG_REQUEST_STREAM_CFG, UMP_STREAM_MSG_STATUS_STREAM_CFG);
    choose_default_protocol(ump);
    for blk in 0..(*ump).info.num_blocks as c_int {
        err = create_block_from_fb_info(ump, blk);
        if err < 0 {
            continue;
        }
    }
    snd_ump_update_group_attrs(ump);
    (*ump).parsed = true;
    ump_request_close(ump);
    if err == -ETIMEDOUT {
        err = -ENODEV;
    }
    err
}

unsafe extern "C" fn snd_ump_legacy_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let ump = (*(*substream).rmidi).private_data as *mut snd_ump_endpoint;
    let dir = (*substream).stream;
    let group = (*ump).legacy_mapping[(*substream).number as usize];
    if !(*ump).legacy_substreams[dir as usize][group as usize].is_null() {
        return -EBUSY;
    }
    if (*ump).groups[group as usize].active == 0 {
        return -ENODEV;
    }
    if dir == SNDRV_RAWMIDI_STREAM_OUTPUT {
        if (*ump).legacy_out_opens == 0 {
            let err = snd_rawmidi_kernel_open_nested(
                &mut (*ump).core,
                0,
                SNDRV_RAWMIDI_LFLG_OUTPUT | SNDRV_RAWMIDI_LFLG_APPEND,
                &mut (*ump).legacy_out_rfile,
                SINGLE_DEPTH_NESTING,
            );
            if err < 0 {
                return err;
            }
        }
        (*ump).legacy_out_opens += 1;
        snd_ump_convert_reset((*ump).out_cvts.add(group as usize));
    }
    (*ump).legacy_substreams[dir as usize][group as usize] = substream;
    0
}

unsafe extern "C" fn snd_ump_legacy_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let ump = (*(*substream).rmidi).private_data as *mut snd_ump_endpoint;
    let dir = (*substream).stream;
    let group = (*ump).legacy_mapping[(*substream).number as usize];
    (*ump).legacy_substreams[dir as usize][group as usize] = ptr::null_mut();
    if dir == SNDRV_RAWMIDI_STREAM_OUTPUT {
        (*ump).legacy_out_opens -= 1;
        if (*ump).legacy_out_opens == 0 {
            snd_rawmidi_kernel_release_nested(&mut (*ump).legacy_out_rfile, SINGLE_DEPTH_NESTING);
        }
    }
    0
}

unsafe extern "C" fn snd_ump_legacy_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let ump = (*(*substream).rmidi).private_data as *mut snd_ump_endpoint;
    let dir = (*substream).stream;
    ((*(*ump).ops).trigger.unwrap())(ump, dir, up);
}

unsafe extern "C" fn snd_ump_legacy_drain(substream: *mut snd_rawmidi_substream) {
    let ump = (*(*substream).rmidi).private_data as *mut snd_ump_endpoint;
    if let Some(drain) = (*(*ump).ops).drain {
        drain(ump, SNDRV_RAWMIDI_STREAM_OUTPUT);
    }
}

unsafe extern "C" fn snd_ump_legacy_dev_register(_rmidi: *mut snd_rawmidi) -> c_int {
    0
}

static snd_ump_legacy_input_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_ump_legacy_open),
    close: Some(snd_ump_legacy_close),
    trigger: Some(snd_ump_legacy_trigger),
    drain: None,
};

static snd_ump_legacy_output_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_ump_legacy_open),
    close: Some(snd_ump_legacy_close),
    trigger: Some(snd_ump_legacy_trigger),
    drain: Some(snd_ump_legacy_drain),
};

static snd_ump_legacy_ops: snd_rawmidi_global_ops = snd_rawmidi_global_ops {
    dev_register: Some(snd_ump_legacy_dev_register),
    dev_unregister: None,
    ioctl: None,
    proc_read: None,
};

unsafe fn process_legacy_output(ump: *mut snd_ump_endpoint, buffer: *mut u32, count: c_int) -> c_int {
    let dir = SNDRV_RAWMIDI_STREAM_OUTPUT;
    let mut size = 0;
    if (*ump).out_cvts.is_null() || (*ump).legacy_out_opens == 0 {
        return 0;
    }
    for group in 0..SNDRV_UMP_MAX_GROUPS {
        let substream = (*ump).legacy_substreams[dir as usize][group as usize];
        if substream.is_null() {
            continue;
        }
        let ctx = (*ump).out_cvts.add(group as usize);
        let mut protocol = (*ump).info.protocol;
        if (protocol & SNDRV_UMP_EP_INFO_PROTO_MIDI2) != 0 && (*ump).groups[group as usize].is_midi1 {
            protocol = SNDRV_UMP_EP_INFO_PROTO_MIDI1;
        }
        let mut c: c_uchar = 0;
        while (*ctx).ump_bytes == 0 && snd_rawmidi_transmit(substream, &mut c, 1) > 0 {
            snd_ump_convert_to_ump(ctx, group, protocol, c);
        }
        if (*ctx).ump_bytes != 0 && (*ctx).ump_bytes <= count {
            size = (*ctx).ump_bytes;
            memcpy(buffer as *mut c_void, (*ctx).ump.as_ptr() as *const c_void, size as size_t);
            (*ctx).ump_bytes = 0;
            break;
        }
    }
    size
}

unsafe fn process_legacy_input(ump: *mut snd_ump_endpoint, src: *const u32, _words: c_int) {
    let mut buf = [0u8; 16];
    let mut group: c_uchar = 0;
    let size = snd_ump_convert_from_ump(src, buf.as_mut_ptr(), &mut group);
    if size <= 0 {
        return;
    }
    let substream = (*ump).legacy_substreams[SNDRV_RAWMIDI_STREAM_INPUT as usize][group as usize];
    if !substream.is_null() {
        snd_rawmidi_receive(substream, buf.as_ptr() as *const c_char, size);
    }
}

unsafe fn fill_legacy_mapping(ump: *mut snd_ump_endpoint) -> c_int {
    let mut group_maps: c_uint = 0;
    if ((*ump).info.flags & SNDRV_UMP_EP_INFO_STATIC_BLOCKS) != 0 {
        let mut pos = (*ump).block_list.next;
        while pos != &mut (*ump).block_list {
            let fb = container_of_block_list(pos);
            for i in 0..(*fb).info.num_groups {
                group_maps |= 1u32 << ((*fb).info.first_group + i);
            }
            pos = (*pos).next;
        }
    }
    if group_maps == 0 {
        group_maps = (1u32 << SNDRV_UMP_MAX_GROUPS) - 1;
    }
    let mut num = 0;
    for i in 0..SNDRV_UMP_MAX_GROUPS {
        if (group_maps & (1u32 << i)) != 0 {
            (*ump).legacy_mapping[num as usize] = i;
            num += 1;
        }
    }
    num
}

unsafe fn update_legacy_substreams(ump: *mut snd_ump_endpoint, rmidi: *mut snd_rawmidi, dir: c_int) {
    let mut pos = (*rmidi).streams[dir as usize].substreams.next;
    while pos != &mut (*rmidi).streams[dir as usize].substreams {
        let s = container_of_substream_list(pos);
        let idx = (*ump).legacy_mapping[(*s).number as usize];
        let mut name = (*ump).groups[idx as usize].name.as_ptr();
        if *name == 0 {
            name = (*ump).core.name.as_ptr();
        }
        scnprintf(
            (*s).name.as_mut_ptr(),
            (*s).name.len(),
            c"Group %d (%.16s)%s".as_ptr(),
            idx + 1,
            name,
            if (*ump).groups[idx as usize].active != 0 { c"".as_ptr() } else { c" [Inactive]".as_ptr() },
        );
        (*s).inactive = (*ump).groups[idx as usize].active == 0;
        pos = (*pos).next;
    }
}

unsafe fn update_legacy_names(ump: *mut snd_ump_endpoint) {
    let rmidi = (*ump).legacy_rmidi;
    update_legacy_substreams(ump, rmidi, SNDRV_RAWMIDI_STREAM_INPUT);
    update_legacy_substreams(ump, rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT);
}

unsafe fn ump_legacy_set_rawmidi_name(ump: *mut snd_ump_endpoint) {
    let rmidi = (*ump).legacy_rmidi;
    snprintf(rmidi.as_mut().unwrap().name.as_mut_ptr(), (*rmidi).name.len(), c"%.68s (MIDI 1.0)".as_ptr(), (*ump).core.name.as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ump_attach_legacy_rawmidi(ump: *mut snd_ump_endpoint, id: *mut c_char, device: c_int) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    (*ump).out_cvts = kzalloc(size_of::<ump_cvt_to_ump>() * SNDRV_UMP_MAX_GROUPS as usize, GFP_KERNEL) as *mut ump_cvt_to_ump;
    if (*ump).out_cvts.is_null() {
        return -ENOMEM;
    }
    let num = fill_legacy_mapping(ump);
    let input = ((*ump).core.info_flags & SNDRV_RAWMIDI_INFO_INPUT) != 0;
    let output = ((*ump).core.info_flags & SNDRV_RAWMIDI_INFO_OUTPUT) != 0;
    let err = snd_rawmidi_new((*ump).core.card, id, device, if output { num } else { 0 }, if input { num } else { 0 }, &mut rmidi);
    if err < 0 {
        kfree((*ump).out_cvts as *mut c_void);
        (*ump).out_cvts = ptr::null_mut();
        return err;
    }
    if input {
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_ump_legacy_input_ops);
    }
    if output {
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_ump_legacy_output_ops);
    }
    (*rmidi).info_flags = (*ump).core.info_flags & !SNDRV_RAWMIDI_INFO_UMP;
    (*rmidi).ops = &snd_ump_legacy_ops;
    (*rmidi).private_data = ump as *mut c_void;
    (*ump).legacy_rmidi = rmidi;
    ump_legacy_set_rawmidi_name(ump);
    update_legacy_names(ump);
    snd_rawmidi_tie_devices(rmidi, &mut (*ump).core);
    0
}

/* MODULE_DESCRIPTION("Universal MIDI Packet (UMP) Core Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
