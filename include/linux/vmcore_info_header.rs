/* SPDX-License-Identifier: GPL-2.0 */

// Required external kernel definitions: ALIGN, PAGE_SIZE, NN_PRSTATUS,
// elf_note, elf_prstatus, vmlinux_build_id, phys_addr_t, Elf_Word, and
// enum hwerr_error_type are supplied by the surrounding translation.

pub const CRASH_CORE_NOTE_HEAD_BYTES: usize = ALIGN(core::mem::size_of::<elf_note>(), 4);
pub const CRASH_CORE_NOTE_NAME_BYTES: usize = ALIGN(core::mem::size_of::<NN_PRSTATUS>(), 4);
pub const CRASH_CORE_NOTE_DESC_BYTES: usize = ALIGN(core::mem::size_of::<elf_prstatus>(), 4);

/*
 * The per-cpu notes area is a list of notes terminated by a "NULL"
 * note header.  For kdump, the code in vmcore.c runs in the context
 * of the second kernel to combine them into one note.
 */
pub const CRASH_CORE_NOTE_BYTES: usize =
    (CRASH_CORE_NOTE_HEAD_BYTES * 2) +
    CRASH_CORE_NOTE_NAME_BYTES +
    CRASH_CORE_NOTE_DESC_BYTES;

pub const VMCOREINFO_BYTES: usize = PAGE_SIZE;
pub const VMCOREINFO_NOTE_NAME: &str = "VMCOREINFO";
pub const VMCOREINFO_NOTE_NAME_BYTES: usize = ALIGN(core::mem::size_of::<[u8; 11]>(), 4);
pub const VMCOREINFO_NOTE_SIZE: usize =
    (CRASH_CORE_NOTE_HEAD_BYTES * 2) + VMCOREINFO_NOTE_NAME_BYTES + VMCOREINFO_BYTES;

pub type NoteBufT = [u32; CRASH_CORE_NOTE_BYTES / 4];

/* Per cpu memory for storing cpu states in case of system crash. */
extern "C" {
    pub static mut crash_notes: *mut NoteBufT;

    pub fn crash_update_vmcoreinfo_safecopy(ptr: *mut core::ffi::c_void);
    pub fn crash_save_vmcoreinfo();
    pub fn arch_crash_save_vmcoreinfo();
    pub fn vmcoreinfo_append_str(fmt: *const core::ffi::c_char, ...);
    pub fn paddr_vmcoreinfo_note() -> phys_addr_t;

    pub static mut vmcoreinfo_data: *mut u8;
    pub static mut vmcoreinfo_size: usize;
    pub static mut vmcoreinfo_note: *mut u32;

    pub fn append_elf_note(
        buf: *mut Elf_Word,
        name: *mut core::ffi::c_char,
        type_: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
        data_len: usize,
    ) -> *mut Elf_Word;
    pub fn final_note(buf: *mut Elf_Word);
}

#[macro_export]
macro_rules! VMCOREINFO_OSRELEASE {
    ($value:expr) => { unsafe { vmcoreinfo_append_str(c"OSRELEASE=%s\n".as_ptr(), $value) } };
}

#[macro_export]
macro_rules! VMCOREINFO_BUILD_ID {
    () => {{
        static_assert!(core::mem::size_of_val(&vmlinux_build_id) == 20);
        unsafe { vmcoreinfo_append_str(c"BUILD-ID=%20phN\n".as_ptr(), vmlinux_build_id) }
    }};
}

#[macro_export]
macro_rules! VMCOREINFO_PAGESIZE {
    ($value:expr) => { unsafe { vmcoreinfo_append_str(c"PAGESIZE=%ld\n".as_ptr(), $value) } };
}

#[macro_export]
macro_rules! VMCOREINFO_SYMBOL {
    ($name:ident) => { unsafe { vmcoreinfo_append_str(c"SYMBOL(%s)=%lx\n".as_ptr(), stringify!($name), &$name as *const _ as usize) } };
}

#[macro_export]
macro_rules! VMCOREINFO_SYMBOL_ARRAY {
    ($name:ident) => { unsafe { vmcoreinfo_append_str(c"SYMBOL(%s)=%lx\n".as_ptr(), stringify!($name), $name as usize) } };
}

#[macro_export]
macro_rules! VMCOREINFO_SIZE {
    ($name:ty) => { unsafe { vmcoreinfo_append_str(c"SIZE(%s)=%lu\n".as_ptr(), stringify!($name), core::mem::size_of::<$name>()) } };
}

#[macro_export]
macro_rules! VMCOREINFO_STRUCT_SIZE {
    ($name:ty) => { unsafe { vmcoreinfo_append_str(c"SIZE(%s)=%lu\n".as_ptr(), stringify!($name), core::mem::size_of::<$name>()) } };
}

#[macro_export]
macro_rules! VMCOREINFO_OFFSET {
    ($name:ty, $field:tt) => { unsafe { vmcoreinfo_append_str(c"OFFSET(%s.%s)=%lu\n".as_ptr(), stringify!($name), stringify!($field), core::mem::offset_of!($name, $field)) } };
}

#[macro_export]
macro_rules! VMCOREINFO_TYPE_OFFSET {
    ($name:ty, $field:tt) => { unsafe { vmcoreinfo_append_str(c"OFFSET(%s.%s)=%lu\n".as_ptr(), stringify!($name), stringify!($field), core::mem::offset_of!($name, $field)) } };
}

#[macro_export]
macro_rules! VMCOREINFO_LENGTH {
    ($name:ident, $value:expr) => { unsafe { vmcoreinfo_append_str(c"LENGTH(%s)=%lu\n".as_ptr(), stringify!($name), $value as usize) } };
}

#[macro_export]
macro_rules! VMCOREINFO_NUMBER {
    ($name:ident) => { unsafe { vmcoreinfo_append_str(c"NUMBER(%s)=%ld\n".as_ptr(), stringify!($name), $name as isize) } };
}

#[macro_export]
macro_rules! VMCOREINFO_CONFIG {
    ($name:ident) => { unsafe { vmcoreinfo_append_str(c"CONFIG_%s=y\n".as_ptr(), stringify!($name)) } };
}

#[cfg(CONFIG_VMCORE_INFO)]
extern "C" {
    pub fn hwerr_log_error_type(src: hwerr_error_type);
}

#[cfg(not(CONFIG_VMCORE_INFO))]
#[inline]
pub unsafe fn hwerr_log_error_type(_src: hwerr_error_type) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
