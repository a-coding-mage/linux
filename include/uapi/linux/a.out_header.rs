/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* __GNU_EXEC_MACROS__ */
/* The C header includes <asm/a.out.h>; its externally supplied declarations
 * are intentionally referenced rather than redefined here. */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum machine_type {
    M_OLDSUN2 = 0,
    M_68010 = 1,
    M_68020 = 2,
    M_SPARC = 3,
    M_386 = 100,
    M_MIPS1 = 151,
    M_MIPS2 = 152,
}

#[macro_export]
macro_rules! N_MAGIC { ($exec:expr) => { (($exec).a_info & 0xffff) }; }
#[macro_export]
macro_rules! N_MACHTYPE { ($exec:expr) => {
    (($exec).a_info >> 16) & 0xff
}; }
#[macro_export]
macro_rules! N_FLAGS { ($exec:expr) => { (($exec).a_info >> 24) & 0xff }; }
#[macro_export]
macro_rules! N_SET_INFO { ($exec:expr, $magic:expr, $type:expr, $flags:expr) => {
    (($exec).a_info = (($magic) & 0xffff)
        | (((($type) as i32) & 0xff) << 16)
        | ((($flags) & 0xff) << 24))
}; }
#[macro_export]
macro_rules! N_SET_MAGIC { ($exec:expr, $magic:expr) => {
    (($exec).a_info = (($exec).a_info & 0xffff0000) | (($magic) & 0xffff))
}; }
#[macro_export]
macro_rules! N_SET_MACHTYPE { ($exec:expr, $machtype:expr) => {
    (($exec).a_info = (($exec).a_info & 0xff00ffff)
        | (((($machtype) as i32) & 0xff) << 16))
}; }
#[macro_export]
macro_rules! N_SET_FLAGS { ($exec:expr, $flags:expr) => {
    (($exec).a_info = (($exec).a_info & 0x00ffffff) | ((($flags) & 0xff) << 24))
}; }

pub const OMAGIC: u32 = 0o407;
pub const NMAGIC: u32 = 0o410;
pub const ZMAGIC: u32 = 0o413;
pub const QMAGIC: u32 = 0o314;
pub const CMAGIC: u32 = 0o421;

#[macro_export]
macro_rules! N_BADMAG { ($x:expr) => {
    $crate::N_MAGIC!($x) != $crate::OMAGIC && $crate::N_MAGIC!($x) != $crate::NMAGIC
        && $crate::N_MAGIC!($x) != $crate::ZMAGIC && $crate::N_MAGIC!($x) != $crate::QMAGIC
}; }
#[macro_export]
macro_rules! _N_HDROFF { ($x:expr) => { 1024usize - core::mem::size_of::<exec>() }; }
#[macro_export]
macro_rules! N_TXTOFF { ($x:expr) => {
    if $crate::N_MAGIC!($x) == $crate::ZMAGIC { $crate::_N_HDROFF!($x) + core::mem::size_of::<exec>() }
    else if $crate::N_MAGIC!($x) == $crate::QMAGIC { 0 } else { core::mem::size_of::<exec>() }
}; }
#[macro_export]
macro_rules! N_DATOFF { ($x:expr) => { $crate::N_TXTOFF!($x) + ($x).a_text }; }
#[macro_export]
macro_rules! N_TRELOFF { ($x:expr) => { $crate::N_DATOFF!($x) + ($x).a_data }; }
#[macro_export]
macro_rules! N_DRELOFF { ($x:expr) => { $crate::N_TRELOFF!($x) + N_TRSIZE!($x) }; }
#[macro_export]
macro_rules! N_SYMOFF { ($x:expr) => { $crate::N_DRELOFF!($x) + N_DRSIZE!($x) }; }
#[macro_export]
macro_rules! N_STROFF { ($x:expr) => { $crate::N_SYMOFF!($x) + N_SYMSIZE!($x) }; }
#[macro_export]
macro_rules! N_TXTADDR { ($x:expr) => { if $crate::N_MAGIC!($x) == $crate::QMAGIC { PAGE_SIZE } else { 0 } }; }

/* On i386 and m68k SEGMENT_SIZE is 1024; otherwise the C header uses
 * getpagesize() outside the kernel. */
pub const SEGMENT_SIZE: usize = 1024;
#[macro_export]
macro_rules! _N_SEGMENT_ROUND { ($x:expr) => { ALIGN!($x, SEGMENT_SIZE) }; }
#[macro_export]
macro_rules! _N_TXTENDADDR { ($x:expr) => { $crate::N_TXTADDR!($x) + ($x).a_text }; }
#[macro_export]
macro_rules! N_DATADDR { ($x:expr) => { if $crate::N_MAGIC!($x) == $crate::OMAGIC { $crate::_N_TXTENDADDR!($x) } else { $crate::_N_SEGMENT_ROUND!($crate::_N_TXTENDADDR!($x)) } }; }
#[macro_export]
macro_rules! N_BSSADDR { ($x:expr) => { $crate::N_DATADDR!($x) + ($x).a_data }; }

#[repr(C)]
pub union nlist_n_un {
    pub n_name: *mut core::ffi::c_char,
    pub n_next: *mut nlist,
    pub n_strx: libc::c_long,
}

#[repr(C)]
pub struct nlist {
    pub n_un: nlist_n_un,
    pub n_type: u8,
    pub n_other: core::ffi::c_char,
    pub n_desc: i16,
    pub n_value: libc::c_ulong,
}

pub const N_UNDF: u32 = 0;
pub const N_ABS: u32 = 2;
pub const N_TEXT: u32 = 4;
pub const N_DATA: u32 = 6;
pub const N_BSS: u32 = 8;
pub const N_FN: u32 = 15;
pub const N_EXT: u32 = 1;
pub const N_TYPE: u32 = 0o36;
pub const N_STAB: u32 = 0o340;
pub const N_INDR: u32 = 0xa;
pub const N_SETA: u32 = 0x14;
pub const N_SETT: u32 = 0x16;
pub const N_SETD: u32 = 0x18;
pub const N_SETB: u32 = 0x1a;
pub const N_SETV: u32 = 0x1c;

#[repr(C)]
pub struct relocation_info {
    pub r_address: i32,
    /* C bitfields are represented by their containing 32-bit storage unit. */
    pub r_symbolnum_pcrel_length_extern_pad: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
