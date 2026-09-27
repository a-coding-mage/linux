/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/info.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const SNDRV_INFO_CONTENT_TEXT: ::kernel::ffi::c_int = 0;
pub const SNDRV_INFO_CONTENT_DATA: ::kernel::ffi::c_int = 1;

#[repr(C)]
pub struct snd_info_buffer {
    pub buffer: *mut ::kernel::ffi::c_char,
    pub curr: ::kernel::ffi::c_uint,
    pub size: ::kernel::ffi::c_uint,
    pub len: ::kernel::ffi::c_uint,
    pub stop: ::kernel::ffi::c_int,
    pub error: ::kernel::ffi::c_int,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_entry_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_info_entry, u16, *mut *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut snd_info_entry, u16, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>,
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut file, *mut ::kernel::ffi::c_char, usize, loff_t) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut file, *const ::kernel::ffi::c_char, usize, loff_t) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut file, loff_t, ::kernel::ffi::c_int) -> loff_t>,
    pub poll: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut file, *mut poll_table) -> __poll_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut file, ::kernel::ffi::c_uint, ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut ::kernel::ffi::c_void, *mut inode, *mut file, *mut vm_area_struct) -> ::kernel::ffi::c_int>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: ::core::mem::ManuallyDrop<snd_info_entry_text>,
    pub ops: *const snd_info_entry_ops,
}

#[repr(C)]
pub struct snd_info_entry {
    pub name: *const ::kernel::ffi::c_char,
    pub mode: umode_t,
    pub size: ::kernel::ffi::c_long,
    pub content: u16,
    pub c: snd_info_entry_c,
    pub parent: *mut snd_info_entry,
    pub module: *mut module,
    pub private_data: *mut ::kernel::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_info_entry)>,
    pub p: *mut proc_dir_entry,
    pub access: mutex,
    pub children: list_head,
    pub list: list_head,
}

pub const SNDRV_OSS_INFO_DEV_AUDIO: ::kernel::ffi::c_int = 0;
pub const SNDRV_OSS_INFO_DEV_SYNTH: ::kernel::ffi::c_int = 1;
pub const SNDRV_OSS_INFO_DEV_MIDI: ::kernel::ffi::c_int = 2;
pub const SNDRV_OSS_INFO_DEV_TIMERS: ::kernel::ffi::c_int = 4;
pub const SNDRV_OSS_INFO_DEV_MIXERS: ::kernel::ffi::c_int = 5;
pub const SNDRV_OSS_INFO_DEV_COUNT: ::kernel::ffi::c_int = 6;

extern "C" {
    pub static mut snd_seq_root: *mut snd_info_entry;
    pub static mut snd_oss_root: *mut snd_info_entry;
    pub fn snd_card_info_read_oss(buffer: *mut snd_info_buffer);
    pub fn snd_info_init() -> ::kernel::ffi::c_int;
    pub fn snd_info_done() -> ::kernel::ffi::c_int;
    pub fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut ::kernel::ffi::c_char, len: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn snd_info_get_str(dest: *mut ::kernel::ffi::c_char, src: *const ::kernel::ffi::c_char, len: ::kernel::ffi::c_int) -> *const ::kernel::ffi::c_char;
    pub fn snd_info_create_module_entry(module: *mut module, name: *const ::kernel::ffi::c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    pub fn snd_info_create_card_entry(card: *mut snd_card, name: *const ::kernel::ffi::c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    pub fn snd_info_free_entry(entry: *mut snd_info_entry);
    pub fn snd_info_card_create(card: *mut snd_card) -> ::kernel::ffi::c_int;
    pub fn snd_info_card_register(card: *mut snd_card) -> ::kernel::ffi::c_int;
    pub fn snd_info_card_free(card: *mut snd_card) -> ::kernel::ffi::c_int;
    pub fn snd_info_card_disconnect(card: *mut snd_card);
    pub fn snd_info_card_id_change(card: *mut snd_card);
    pub fn snd_info_register(entry: *mut snd_info_entry) -> ::kernel::ffi::c_int;
    pub fn snd_card_rw_proc_new(card: *mut snd_card, name: *const ::kernel::ffi::c_char, private_data: *mut ::kernel::ffi::c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>, write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>) -> ::kernel::ffi::c_int;
    pub fn snd_info_check_reserved_words(string: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn snd_oss_info_register(dev: ::kernel::ffi::c_int, num: ::kernel::ffi::c_int, string: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
}

#[inline]
pub unsafe fn snd_card_proc_new(card: *mut snd_card, name: *const ::kernel::ffi::c_char, entryp: *mut *mut snd_info_entry) -> ::kernel::ffi::c_int {
    *entryp = snd_info_create_card_entry(card, name, (*card).proc_root);
    if !(*entryp).is_null() { 0 } else { -12 }
}

#[inline]
pub unsafe fn snd_info_set_text_ops(entry: *mut snd_info_entry, private_data: *mut ::kernel::ffi::c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>) {
    (*entry).private_data = private_data;
    (*entry).c.text.read = read;
}

#[inline]
pub unsafe fn snd_oss_info_unregister(dev: ::kernel::ffi::c_int, num: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int {
    snd_oss_info_register(dev, num, ::core::ptr::null_mut())
}

#[inline]
pub unsafe fn snd_card_ro_proc_new(card: *mut snd_card, name: *const ::kernel::ffi::c_char, private_data: *mut ::kernel::ffi::c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>) -> ::kernel::ffi::c_int {
    snd_card_rw_proc_new(card, name, private_data, read, None)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
