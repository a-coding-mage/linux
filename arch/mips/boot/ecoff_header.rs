/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Some ECOFF definitions.
 */

#[repr(C)]
pub struct filehdr {
    pub f_magic: u16,   /* magic number */
    pub f_nscns: u16,   /* number of sections */
    pub f_timdat: i32,  /* time & date stamp */
    pub f_symptr: i32,  /* file pointer to symbolic header */
    pub f_nsyms: i32,   /* sizeof(symbolic hdr) */
    pub f_opthdr: u16,  /* sizeof(optional hdr) */
    pub f_flags: u16,   /* flags */
}
pub type FILHDR = filehdr;
pub const FILHSZ: usize = core::mem::size_of::<FILHDR>();

pub const MIPSEBMAGIC: u16 = 0x160;
pub const MIPSELMAGIC: u16 = 0x162;

#[repr(C)]
pub struct scnhdr {
    pub s_name: [core::ffi::c_char; 8], /* section name */
    pub s_paddr: i32,      /* physical address, aliased s_nlib */
    pub s_vaddr: i32,      /* virtual address */
    pub s_size: i32,       /* section size */
    pub s_scnptr: i32,     /* file ptr to raw data for section */
    pub s_relptr: i32,     /* file ptr to relocation */
    pub s_lnnoptr: i32,    /* file ptr to gp histogram */
    pub s_nreloc: u16,     /* number of relocation entries */
    pub s_nlnno: u16,      /* number of gp histogram entries */
    pub s_flags: i32,      /* flags */
}
pub type SCNHDR = scnhdr;
pub const SCNHSZ: usize = core::mem::size_of::<SCNHDR>();
pub const SCNROUND: i32 = 16;

#[repr(C)]
pub struct aouthdr {
    pub magic: i16,       /* see above */
    pub vstamp: i16,      /* version stamp */
    pub tsize: i32,       /* text size in bytes, padded to DW bdry*/
    pub dsize: i32,       /* initialized data "  " */
    pub bsize: i32,       /* uninitialized data "\t  " */
    pub entry: i32,       /* entry pt. */
    pub text_start: i32,  /* base of text used for this file */
    pub data_start: i32,  /* base of data used for this file */
    pub bss_start: i32,   /* base of bss used for this file */
    pub gprmask: i32,     /* general purpose register mask */
    pub cprmask: [i32; 4], /* co-processor register masks */
    pub gp_value: i32,    /* the gp value used for this object */
}
pub type AOUTHDR = aouthdr;
pub const AOUTHSZ: usize = core::mem::size_of::<AOUTHDR>();

pub const OMAGIC: i32 = 0o407;
pub const NMAGIC: i32 = 0o410;
pub const ZMAGIC: i32 = 0o413;
pub const SMAGIC: i32 = 0o411;
pub const LIBMAGIC: i32 = 0o443;

#[inline]
pub const fn N_TXTOFF(f: &FILHDR, a: &AOUTHDR) -> usize {
    if a.magic == ZMAGIC as i16 || a.magic == LIBMAGIC as i16 {
        0
    } else if a.vstamp < 23 {
        (FILHSZ + AOUTHSZ + (f.f_nscns as usize) * SCNHSZ + 7) & 0xfffffff8
    } else {
        (FILHSZ + AOUTHSZ + (f.f_nscns as usize) * SCNHSZ + (SCNROUND as usize) - 1)
            & !(SCNROUND as usize - 1)
    }
}

#[inline]
pub const fn N_DATOFF(f: &FILHDR, a: &AOUTHDR) -> usize {
    N_TXTOFF(f, a) + a.tsize as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
