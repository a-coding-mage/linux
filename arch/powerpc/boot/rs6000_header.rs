/* SPDX-License-Identifier: GPL-2.0 */
/* IBM RS/6000 "XCOFF" file definitions for BFD.
   Copyright (C) 1990, 1991 Free Software Foundation, Inc.
   Written by Mimi Phuong-Thao Vo of IBM
   and John Gilmore of Cygnus Support.  */

/********************** FILE HEADER **********************/

#[repr(C)]
pub struct external_filehdr {
    pub f_magic: [i8; 2],
    pub f_nscns: [i8; 2],
    pub f_timdat: [i8; 4],
    pub f_symptr: [i8; 4],
    pub f_nsyms: [i8; 4],
    pub f_opthdr: [i8; 2],
    pub f_flags: [i8; 2],
}

/* IBM RS/6000 */
pub const U802WRMAGIC: i32 = 0o730;
pub const U802ROMAGIC: i32 = 0o735;
pub const U802TOCMAGIC: i32 = 0o737;

/* BADMAG(x): ((x).f_magic != U802ROMAGIC && (x).f_magic != U802WRMAGIC &&
   (x).f_magic != U802TOCMAGIC) */
pub const FILHSZ: usize = 20;

/********************** AOUT "OPTIONAL HEADER" **********************/

#[repr(C)]
pub struct aouthdr {
    pub magic: [u8; 2],
    pub vstamp: [u8; 2],
    pub tsize: [u8; 4],
    pub dsize: [u8; 4],
    pub bsize: [u8; 4],
    pub entry: [u8; 4],
    pub text_start: [u8; 4],
    pub data_start: [u8; 4],
    pub o_toc: [u8; 4],
    pub o_snentry: [u8; 2],
    pub o_sntext: [u8; 2],
    pub o_sndata: [u8; 2],
    pub o_sntoc: [u8; 2],
    pub o_snloader: [u8; 2],
    pub o_snbss: [u8; 2],
    pub o_algntext: [u8; 2],
    pub o_algndata: [u8; 2],
    pub o_modtype: [u8; 2],
    pub o_cputype: [u8; 2],
    pub o_maxstack: [u8; 4],
    pub o_maxdata: [u8; 4],
    pub o_resv2: [u8; 12],
}

pub const AOUTSZ: usize = 72;
pub const SMALL_AOUTSZ: usize = 28;
pub const AOUTHDRSZ: usize = 72;
pub const RS6K_AOUTHDR_OMAGIC: u16 = 0x0107;
pub const RS6K_AOUTHDR_NMAGIC: u16 = 0x0108;
pub const RS6K_AOUTHDR_ZMAGIC: u16 = 0x010B;

/********************** SECTION HEADER **********************/

#[repr(C)]
pub struct external_scnhdr {
    pub s_name: [i8; 8],
    pub s_paddr: [i8; 4],
    pub s_vaddr: [i8; 4],
    pub s_size: [i8; 4],
    pub s_scnptr: [i8; 4],
    pub s_relptr: [i8; 4],
    pub s_lnnoptr: [i8; 4],
    pub s_nreloc: [i8; 2],
    pub s_nlnno: [i8; 2],
    pub s_flags: [i8; 4],
}

pub const _TEXT: &str = ".text";
pub const _DATA: &str = ".data";
pub const _BSS: &str = ".bss";
pub const _PAD: &str = ".pad";
pub const _LOADER: &str = ".loader";
pub const SCNHSZ: usize = 40;
pub const STYP_LOADER: u16 = 0x1000;
pub const STYP_DEBUG: u16 = 0x2000;
pub const STYP_OVRFLO: u16 = 0x8000;

/********************** LINE NUMBERS **********************/

#[repr(C)]
pub union external_lineno_addr {
    pub l_symndx: [i8; 4],
    pub l_paddr: [i8; 4],
}

#[repr(C)]
pub struct external_lineno {
    pub l_addr: external_lineno_addr,
    pub l_lnno: [i8; 2],
}

pub const LINESZ: usize = 6;

/********************** SYMBOLS **********************/

pub const E_SYMNMLEN: usize = 8;
pub const E_FILNMLEN: usize = 14;
pub const E_DIMNUM: usize = 4;

#[repr(C)]
pub struct external_syment_name_e {
    pub e_zeroes: [i8; 4],
    pub e_offset: [i8; 4],
}

#[repr(C)]
pub union external_syment_name {
    pub e_name: [i8; E_SYMNMLEN],
    pub e: external_syment_name_e,
}

#[repr(C)]
pub struct external_syment {
    pub e: external_syment_name,
    pub e_value: [i8; 4],
    pub e_scnum: [i8; 2],
    pub e_type: [i8; 2],
    pub e_sclass: [i8; 1],
    pub e_numaux: [i8; 1],
}

pub const N_BTMASK: i32 = 0o17;
pub const N_TMASK: i32 = 0o60;
pub const N_BTSHFT: i32 = 4;
pub const N_TSHIFT: i32 = 2;

#[repr(C)]
pub struct external_auxent_x_sym_x_misc_x_lnsz {
    pub x_lnno: [i8; 2],
    pub x_size: [i8; 2],
}

#[repr(C)]
pub union external_auxent_x_sym_x_misc {
    pub x_lnsz: external_auxent_x_sym_x_misc_x_lnsz,
    pub x_fsize: [i8; 4],
}

#[repr(C)]
pub struct external_auxent_x_sym_x_fcnary_x_fcn {
    pub x_lnnoptr: [i8; 4],
    pub x_endndx: [i8; 4],
}

#[repr(C)]
pub struct external_auxent_x_sym_x_fcnary_x_ary {
    pub x_dimen: [[i8; 2]; E_DIMNUM],
}

#[repr(C)]
pub union external_auxent_x_sym_x_fcnary {
    pub x_fcn: external_auxent_x_sym_x_fcnary_x_fcn,
    pub x_ary: external_auxent_x_sym_x_fcnary_x_ary,
}

#[repr(C)]
pub struct external_auxent_x_sym {
    pub x_tagndx: [i8; 4],
    pub x_misc: external_auxent_x_sym_x_misc,
    pub x_fcnary: external_auxent_x_sym_x_fcnary,
    pub x_tvndx: [i8; 2],
}

#[repr(C)]
pub struct external_auxent_x_file_x_n {
    pub x_zeroes: [i8; 4],
    pub x_offset: [i8; 4],
}

#[repr(C)]
pub union external_auxent_x_file {
    pub x_fname: [i8; E_FILNMLEN],
    pub x_n: external_auxent_x_file_x_n,
}

#[repr(C)]
pub struct external_auxent_x_scn {
    pub x_scnlen: [i8; 4],
    pub x_nreloc: [i8; 2],
    pub x_nlinno: [i8; 2],
}

#[repr(C)]
pub struct external_auxent_x_tv {
    pub x_tvfill: [i8; 4],
    pub x_tvlen: [i8; 2],
    pub x_tvran: [[i8; 2]; 2],
}

#[repr(C)]
pub struct external_auxent_x_csect {
    pub x_scnlen: [u8; 4],
    pub x_parmhash: [u8; 4],
    pub x_snhash: [u8; 2],
    pub x_smtyp: [u8; 1],
    pub x_smclas: [u8; 1],
    pub x_stab: [u8; 4],
    pub x_snstab: [u8; 2],
}

#[repr(C)]
pub union external_auxent {
    pub x_sym: external_auxent_x_sym,
    pub x_file: external_auxent_x_file,
    pub x_scn: external_auxent_x_scn,
    pub x_tv: external_auxent_x_tv,
    pub x_csect: external_auxent_x_csect,
}

pub const SYMESZ: usize = 18;
pub const AUXESZ: usize = 18;
pub const DBXMASK: u8 = 0x80;
/* SYMNAME_IN_DEBUG(symptr): ((symptr)->n_sclass & DBXMASK) */

/********************** RELOCATION DIRECTIVES **********************/

#[repr(C)]
pub struct external_reloc {
    pub r_vaddr: [i8; 4],
    pub r_symndx: [i8; 4],
    pub r_size: [i8; 1],
    pub r_type: [i8; 1],
}

pub const RELSZ: usize = 10;
pub const DEFAULT_DATA_SECTION_ALIGNMENT: i32 = 4;
pub const DEFAULT_BSS_SECTION_ALIGNMENT: i32 = 4;
pub const DEFAULT_TEXT_SECTION_ALIGNMENT: i32 = 4;
/* For new sections we haven't heard of before */
pub const DEFAULT_SECTION_ALIGNMENT: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
