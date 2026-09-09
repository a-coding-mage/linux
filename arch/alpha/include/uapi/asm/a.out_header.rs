/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent: linux/types.h supplies the __u* integer types.

/*
 * OSF/1 ECOFF header structs.  ECOFF files consist of:
 * 	- a file header (struct filehdr),
 * 	- an a.out header (struct aouthdr),
 * 	- one or more section headers (struct scnhdr).
 *	  The filhdr's "f_nscns" field contains the
 *	  number of section headers.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filehdr {
    /* OSF/1 "file" header */
    pub f_magic: u16,
    pub f_nscns: u16,
    pub f_timdat: u32,
    pub f_symptr: u64,
    pub f_nsyms: u32,
    pub f_opthdr: u16,
    pub f_flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aouthdr {
    pub info: u64, /* after that it looks quite normal.. */
    pub tsize: u64,
    pub dsize: u64,
    pub bsize: u64,
    pub entry: u64,
    pub text_start: u64, /* with a few additions that actually make sense */
    pub data_start: u64,
    pub bss_start: u64,
    pub gprmask: u32,
    pub fprmask: u32, /* bitmask of general & floating point regs used in binary */
    pub gpvalue: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scnhdr {
    pub s_name: [core::ffi::c_char; 8],
    pub s_paddr: u64,
    pub s_vaddr: u64,
    pub s_size: u64,
    pub s_scnptr: u64,
    pub s_relptr: u64,
    pub s_lnnoptr: u64,
    pub s_nreloc: u16,
    pub s_nlnno: u16,
    pub s_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exec {
    /* OSF/1 "file" header */
    pub fh: filehdr,
    pub ah: aouthdr,
}

/* Define's so that the kernel exec code can access the a.out header fields. */
#[macro_export]
macro_rules! a_info { ($x:expr) => { ($x).ah.info }; }
#[macro_export]
macro_rules! a_text { ($x:expr) => { ($x).ah.tsize }; }
#[macro_export]
macro_rules! a_data { ($x:expr) => { ($x).ah.dsize }; }
#[macro_export]
macro_rules! a_bss { ($x:expr) => { ($x).ah.bsize }; }
#[macro_export]
macro_rules! a_entry { ($x:expr) => { ($x).ah.entry }; }
#[macro_export]
macro_rules! a_textstart { ($x:expr) => { ($x).ah.text_start }; }
#[macro_export]
macro_rules! a_datastart { ($x:expr) => { ($x).ah.data_start }; }
#[macro_export]
macro_rules! a_bssstart { ($x:expr) => { ($x).ah.bss_start }; }
#[macro_export]
macro_rules! a_gprmask { ($x:expr) => { ($x).ah.gprmask }; }
#[macro_export]
macro_rules! a_fprmask { ($x:expr) => { ($x).ah.fprmask }; }
#[macro_export]
macro_rules! a_gpvalue { ($x:expr) => { ($x).ah.gpvalue }; }

#[macro_export]
macro_rules! N_TXTADDR { ($x:expr) => { ($x).ah.text_start }; }
#[macro_export]
macro_rules! N_DATADDR { ($x:expr) => { ($x).ah.data_start }; }
#[macro_export]
macro_rules! N_BSSADDR { ($x:expr) => { ($x).ah.bss_start }; }
#[macro_export]
macro_rules! N_DRSIZE { ($x:expr) => { 0 }; }
#[macro_export]
macro_rules! N_TRSIZE { ($x:expr) => { 0 }; }
#[macro_export]
macro_rules! N_SYMSIZE { ($x:expr) => { 0 }; }

pub const AOUTHSZ: usize = core::mem::size_of::<aouthdr>();
pub const SCNHSZ: usize = core::mem::size_of::<scnhdr>();
pub const SCNROUND: usize = 16;

// N_TXTOFF depends on the externally supplied N_MAGIC and ZMAGIC definitions.
#[macro_export]
macro_rules! N_TXTOFF {
    ($x:expr) => {
        if (N_MAGIC!($x) as isize) == ZMAGIC {
            0
        } else {
            (core::mem::size_of::<exec>() + ($x).fh.f_nscns as usize * SCNHSZ
                + SCNROUND - 1) & !(SCNROUND - 1)
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
