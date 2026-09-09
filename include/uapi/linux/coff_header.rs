/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* This file is derived from the GAS 2.1.4 assembler control file. */

pub const E_SYMNMLEN: usize = 8;
pub const E_FILNMLEN: usize = 14;
pub const E_DIMNUM: usize = 4;

/* Byte-order independent loads from character tables. */
#[inline]
pub unsafe fn coff_short_l(ps: *const u8) -> i16 {
    (((*ps.add(1) as u16) << 8) | (*ps as u16)) as i16
}
#[inline]
pub unsafe fn coff_long_l(ps: *const u8) -> i32 {
    (((*ps.add(3) as u32) << 24) | ((*ps.add(2) as u32) << 16)
        | ((*ps.add(1) as u32) << 8) | (*ps as u32)) as i32
}
#[inline]
pub unsafe fn coff_short_h(ps: *const u8) -> i16 {
    (((*ps as u16) << 8) | (*ps.add(1) as u16)) as i16
}
#[inline]
pub unsafe fn coff_long_h(ps: *const u8) -> i32 {
    (((*ps as u32) << 24) | ((*ps.add(1) as u32) << 16)
        | ((*ps.add(2) as u32) << 8) | (*ps.add(3) as u32)) as i32
}
#[inline]
pub unsafe fn coff_long(v: *const u8) -> i32 { coff_long_l(v) }
#[inline]
pub unsafe fn coff_short(v: *const u8) -> i16 { coff_short_l(v) }

#[repr(C)]
pub struct COFF_filehdr {
    pub f_magic: [u8; 2], pub f_nscns: [u8; 2], pub f_timdat: [u8; 4],
    pub f_symptr: [u8; 4], pub f_nsyms: [u8; 4], pub f_opthdr: [u8; 2],
    pub f_flags: [u8; 2],
}

pub const COFF_F_RELFLG: u32 = 0o0000001;
pub const COFF_F_EXEC: u32 = 0o0000002;
pub const COFF_F_LNNO: u32 = 0o0000004;
pub const COFF_F_LSYMS: u32 = 0o0000010;
pub const COFF_F_MINMAL: u32 = 0o0000020;
pub const COFF_F_UPDATE: u32 = 0o0000040;
pub const COFF_F_SWABD: u32 = 0o0000100;
pub const COFF_F_AR16WR: u32 = 0o0000200;
pub const COFF_F_AR32WR: u32 = 0o0000400;
pub const COFF_F_AR32W: u32 = 0o0001000;
pub const COFF_F_PATCH: u32 = 0o0002000;
pub const COFF_F_NODF: u32 = 0o0002000;
pub const COFF_I386MAGIC: u16 = 0x14c;
pub const COFF_FILHSZ: usize = core::mem::size_of::<COFF_filehdr>();

#[repr(C)]
pub struct COFF_AOUTHDR {
    pub magic: [u8; 2], pub vstamp: [u8; 2], pub tsize: [u8; 4],
    pub dsize: [u8; 4], pub bsize: [u8; 4], pub entry: [u8; 4],
    pub text_start: [u8; 4], pub data_start: [u8; 4],
}
pub const COFF_AOUTSZ: usize = core::mem::size_of::<COFF_AOUTHDR>();
pub const COFF_STMAGIC: u16 = 0o401;
pub const COFF_OMAGIC: u16 = 0o404;
pub const COFF_JMAGIC: u16 = 0o407;
pub const COFF_DMAGIC: u16 = 0o410;
pub const COFF_ZMAGIC: u16 = 0o413;
pub const COFF_SHMAGIC: u16 = 0o443;

#[repr(C)]
pub struct COFF_scnhdr {
    pub s_name: [u8; 8], pub s_paddr: [u8; 4], pub s_vaddr: [u8; 4],
    pub s_size: [u8; 4], pub s_scnptr: [u8; 4], pub s_relptr: [u8; 4],
    pub s_lnnoptr: [u8; 4], pub s_nreloc: [u8; 2], pub s_nlnno: [u8; 2],
    pub s_flags: [u8; 4],
}
pub const COFF_SCNHSZ: usize = core::mem::size_of::<COFF_scnhdr>();
pub const COFF_TEXT: &str = ".text";
pub const COFF_DATA: &str = ".data";
pub const COFF_BSS: &str = ".bss";
pub const COFF_COMMENT: &str = ".comment";
pub const COFF_LIB: &str = ".lib";
pub const COFF_SECT_TEXT: u32 = 0; pub const COFF_SECT_DATA: u32 = 1;
pub const COFF_SECT_BSS: u32 = 2; pub const COFF_SECT_REQD: u32 = 3;
pub const COFF_STYP_REG: u32 = 0x00; pub const COFF_STYP_DSECT: u32 = 0x01;
pub const COFF_STYP_NOLOAD: u32 = 0x02; pub const COFF_STYP_GROUP: u32 = 0x04;
pub const COFF_STYP_PAD: u32 = 0x08; pub const COFF_STYP_COPY: u32 = 0x10;
pub const COFF_STYP_TEXT: u32 = 0x20; pub const COFF_STYP_DATA: u32 = 0x40;
pub const COFF_STYP_BSS: u32 = 0x80; pub const COFF_STYP_INFO: u32 = 0x200;
pub const COFF_STYP_OVER: u32 = 0x400; pub const COFF_STYP_LIB: u32 = 0x800;

#[repr(C)] pub struct COFF_slib { pub sl_entsz: [u8; 4], pub sl_pathndx: [u8; 4] }
pub const COFF_SLIBSZ: usize = core::mem::size_of::<COFF_slib>();

#[repr(C)]
pub union COFF_lineno_addr { pub l_symndx: [u8; 4], pub l_paddr: [u8; 4] }
#[repr(C)] pub struct COFF_lineno { pub l_addr: COFF_lineno_addr, pub l_lnno: [u8; 2] }
pub const COFF_LINESZ: usize = 6;

#[repr(C)] pub struct COFF_syment_name_e { pub e_zeroes: [u8; 4], pub e_offset: [u8; 4] }
#[repr(C)] pub union COFF_syment_name { pub e_name: [u8; 8], pub e: COFF_syment_name_e }
#[repr(C)] pub struct COFF_syment {
    pub e: COFF_syment_name, pub e_value: [u8; 4], pub e_scnum: [u8; 2],
    pub e_type: [u8; 2], pub e_sclass: [u8; 1], pub e_numaux: [u8; 1],
}
pub const COFF_N_BTMASK: u32 = 0xf; pub const COFF_N_TMASK: u32 = 0x30;
pub const COFF_N_BTSHFT: u32 = 4; pub const COFF_N_TSHIFT: u32 = 2;

#[repr(C)] pub struct COFF_aux_x_lnsz { pub x_lnno: [u8; 2], pub x_size: [u8; 2] }
#[repr(C)] pub union COFF_aux_x_misc { pub x_lnsz: COFF_aux_x_lnsz, pub x_fsize: [u8; 4] }
#[repr(C)] pub struct COFF_aux_x_fcn { pub x_lnnoptr: [u8; 4], pub x_endndx: [u8; 4] }
#[repr(C)] pub struct COFF_aux_x_ary { pub x_dimen: [[u8; 2]; 4] }
#[repr(C)] pub union COFF_aux_x_fcnary { pub x_fcn: COFF_aux_x_fcn, pub x_ary: COFF_aux_x_ary }
#[repr(C)] pub struct COFF_aux_x_sym { pub x_tagndx: [u8; 4], pub x_misc: COFF_aux_x_misc, pub x_fcnary: COFF_aux_x_fcnary, pub x_tvndx: [u8; 2] }
#[repr(C)] pub struct COFF_aux_x_n { pub x_zeroes: [u8; 4], pub x_offset: [u8; 4] }
#[repr(C)] pub union COFF_aux_x_file { pub x_fname: [u8; 14], pub x_n: COFF_aux_x_n }
#[repr(C)] pub struct COFF_aux_x_scn { pub x_scnlen: [u8; 4], pub x_nreloc: [u8; 2], pub x_nlinno: [u8; 2] }
#[repr(C)] pub struct COFF_aux_x_tv { pub x_tvfill: [u8; 4], pub x_tvlen: [u8; 2], pub x_tvran: [[u8; 2]; 2] }
#[repr(C)] pub union COFF_auxent { pub x_sym: COFF_aux_x_sym, pub x_file: COFF_aux_x_file, pub x_scn: COFF_aux_x_scn, pub x_tv: COFF_aux_x_tv }
pub const COFF_SYMESZ: usize = 18; pub const COFF_AUXESZ: usize = 18;
pub const COFF_ETEXT: &str = "etext";

#[repr(C)] pub struct COFF_reloc { pub r_vaddr: [u8; 4], pub r_symndx: [u8; 4], pub r_type: [u8; 2] }
pub const COFF_RELSZ: usize = 10;
pub const COFF_DEF_DATA_SECTION_ALIGNMENT: u32 = 4;
pub const COFF_DEF_BSS_SECTION_ALIGNMENT: u32 = 4;
pub const COFF_DEF_TEXT_SECTION_ALIGNMENT: u32 = 4;
pub const COFF_DEF_SECTION_ALIGNMENT: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
