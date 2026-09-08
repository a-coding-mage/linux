/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding translation unit:
// elf.h, tools/be_byteshift.h, and tools/le_byteshift.h.

#[repr(C)]
pub union Elf_Ehdr {
    pub e32: Elf32_Ehdr,
    pub e64: Elf64_Ehdr,
}

#[repr(C)]
pub union Elf_Shdr {
    pub e32: Elf32_Shdr,
    pub e64: Elf64_Shdr,
}

#[repr(C)]
pub union Elf_Sym {
    pub e32: Elf32_Sym,
    pub e64: Elf64_Sym,
}

#[repr(C)]
pub union Elf_Rela {
    pub e32: Elf32_Rela,
    pub e64: Elf64_Rela,
}

#[repr(C)]
pub struct elf_funcs {
    pub compare_extable: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32>,
    pub ehdr_shoff: Option<unsafe extern "C" fn(*mut Elf_Ehdr) -> u64>,
    pub ehdr_shstrndx: Option<unsafe extern "C" fn(*mut Elf_Ehdr) -> u16>,
    pub ehdr_shentsize: Option<unsafe extern "C" fn(*mut Elf_Ehdr) -> u16>,
    pub ehdr_shnum: Option<unsafe extern "C" fn(*mut Elf_Ehdr) -> u16>,
    pub shdr_addr: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u64>,
    pub shdr_offset: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u64>,
    pub shdr_size: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u64>,
    pub shdr_entsize: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u64>,
    pub shdr_link: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u32>,
    pub shdr_name: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u32>,
    pub shdr_type: Option<unsafe extern "C" fn(*mut Elf_Shdr) -> u32>,
    pub sym_type: Option<unsafe extern "C" fn(*mut Elf_Sym) -> u8>,
    pub sym_name: Option<unsafe extern "C" fn(*mut Elf_Sym) -> u32>,
    pub sym_value: Option<unsafe extern "C" fn(*mut Elf_Sym) -> u64>,
    pub sym_shndx: Option<unsafe extern "C" fn(*mut Elf_Sym) -> u16>,
    pub rela_offset: Option<unsafe extern "C" fn(*mut Elf_Rela) -> u64>,
    pub rela_info: Option<unsafe extern "C" fn(*mut Elf_Rela) -> u64>,
    pub rela_addend: Option<unsafe extern "C" fn(*mut Elf_Rela) -> u64>,
    pub rela_write_addend: Option<unsafe extern "C" fn(*mut Elf_Rela, u64)>,
    pub r: Option<unsafe extern "C" fn(*const u32) -> u32>,
    pub r2: Option<unsafe extern "C" fn(*const u16) -> u16>,
    pub r8: Option<unsafe extern "C" fn(*const u64) -> u64>,
    pub w: Option<unsafe extern "C" fn(u32, *mut u32)>,
    pub w8: Option<unsafe extern "C" fn(u64, *mut u64)>,
}

extern "C" {
    pub static mut elf_parser: elf_funcs;
}

#[inline]
pub unsafe fn ehdr64_shoff(ehdr: *mut Elf_Ehdr) -> u64 { (elf_parser.r8.unwrap())(&(*ehdr).e64.e_shoff) }
#[inline]
pub unsafe fn ehdr32_shoff(ehdr: *mut Elf_Ehdr) -> u64 { (elf_parser.r.unwrap())(&(*ehdr).e32.e_shoff) as u64 }
#[inline]
pub unsafe fn ehdr_shoff(ehdr: *mut Elf_Ehdr) -> u64 { (elf_parser.ehdr_shoff.unwrap())(ehdr) }

#[inline] pub unsafe fn ehdr64_shentsize(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e64.e_shentsize) }
#[inline] pub unsafe fn ehdr32_shentsize(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e32.e_shentsize) }
#[inline] pub unsafe fn ehdr_shentsize(e: *mut Elf_Ehdr) -> u16 { (elf_parser.ehdr_shentsize.unwrap())(e) }
#[inline] pub unsafe fn ehdr64_shstrndx(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e64.e_shstrndx) }
#[inline] pub unsafe fn ehdr32_shstrndx(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e32.e_shstrndx) }
#[inline] pub unsafe fn ehdr_shstrndx(e: *mut Elf_Ehdr) -> u16 { (elf_parser.ehdr_shstrndx.unwrap())(e) }
#[inline] pub unsafe fn ehdr64_shnum(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e64.e_shnum) }
#[inline] pub unsafe fn ehdr32_shnum(e: *mut Elf_Ehdr) -> u16 { (elf_parser.r2.unwrap())(&(*e).e32.e_shnum) }
#[inline] pub unsafe fn ehdr_shnum(e: *mut Elf_Ehdr) -> u16 { (elf_parser.ehdr_shnum.unwrap())(e) }

// Direct expansions of the C accessor macros.
#[inline] pub unsafe fn shdr64_addr(s:*mut Elf_Shdr)->u64{(elf_parser.r8.unwrap())(&(*s).e64.sh_addr)}
#[inline] pub unsafe fn shdr32_addr(s:*mut Elf_Shdr)->u64{(elf_parser.r.unwrap())(&(*s).e32.sh_addr) as u64}
#[inline] pub unsafe fn shdr_addr(s:*mut Elf_Shdr)->u64{(elf_parser.shdr_addr.unwrap())(s)}
#[inline] pub unsafe fn shdr64_offset(s:*mut Elf_Shdr)->u64{(elf_parser.r8.unwrap())(&(*s).e64.sh_offset)}
#[inline] pub unsafe fn shdr32_offset(s:*mut Elf_Shdr)->u64{(elf_parser.r.unwrap())(&(*s).e32.sh_offset) as u64}
#[inline] pub unsafe fn shdr_offset(s:*mut Elf_Shdr)->u64{(elf_parser.shdr_offset.unwrap())(s)}
#[inline] pub unsafe fn shdr64_size(s:*mut Elf_Shdr)->u64{(elf_parser.r8.unwrap())(&(*s).e64.sh_size)}
#[inline] pub unsafe fn shdr32_size(s:*mut Elf_Shdr)->u64{(elf_parser.r.unwrap())(&(*s).e32.sh_size) as u64}
#[inline] pub unsafe fn shdr_size(s:*mut Elf_Shdr)->u64{(elf_parser.shdr_size.unwrap())(s)}
#[inline] pub unsafe fn shdr64_entsize(s:*mut Elf_Shdr)->u64{(elf_parser.r8.unwrap())(&(*s).e64.sh_entsize)}
#[inline] pub unsafe fn shdr32_entsize(s:*mut Elf_Shdr)->u64{(elf_parser.r.unwrap())(&(*s).e32.sh_entsize) as u64}
#[inline] pub unsafe fn shdr_entsize(s:*mut Elf_Shdr)->u64{(elf_parser.shdr_entsize.unwrap())(s)}

#[inline] pub unsafe fn shdr64_link(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e64.sh_link)}
#[inline] pub unsafe fn shdr32_link(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e32.sh_link)}
#[inline] pub unsafe fn shdr_link(s:*mut Elf_Shdr)->u32{(elf_parser.shdr_link.unwrap())(s)}
#[inline] pub unsafe fn shdr64_name(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e64.sh_name)}
#[inline] pub unsafe fn shdr32_name(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e32.sh_name)}
#[inline] pub unsafe fn shdr_name(s:*mut Elf_Shdr)->u32{(elf_parser.shdr_name.unwrap())(s)}
#[inline] pub unsafe fn shdr64_type(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e64.sh_type)}
#[inline] pub unsafe fn shdr32_type(s:*mut Elf_Shdr)->u32{(elf_parser.r.unwrap())(&(*s).e32.sh_type)}
#[inline] pub unsafe fn shdr_type(s:*mut Elf_Shdr)->u32{(elf_parser.shdr_type.unwrap())(s)}

#[inline] pub unsafe fn sym64_value(s:*mut Elf_Sym)->u64{(elf_parser.r8.unwrap())(&(*s).e64.st_value)}
#[inline] pub unsafe fn sym32_value(s:*mut Elf_Sym)->u64{(elf_parser.r.unwrap())(&(*s).e32.st_value) as u64}
#[inline] pub unsafe fn sym_value(s:*mut Elf_Sym)->u64{(elf_parser.sym_value.unwrap())(s)}
#[inline] pub unsafe fn sym64_name(s:*mut Elf_Sym)->u32{(elf_parser.r.unwrap())(&(*s).e64.st_name)}
#[inline] pub unsafe fn sym32_name(s:*mut Elf_Sym)->u32{(elf_parser.r.unwrap())(&(*s).e32.st_name)}
#[inline] pub unsafe fn sym_name(s:*mut Elf_Sym)->u32{(elf_parser.sym_name.unwrap())(s)}
#[inline] pub unsafe fn sym64_shndx(s:*mut Elf_Sym)->u16{(elf_parser.r2.unwrap())(&(*s).e64.st_shndx)}
#[inline] pub unsafe fn sym32_shndx(s:*mut Elf_Sym)->u16{(elf_parser.r2.unwrap())(&(*s).e32.st_shndx)}
#[inline] pub unsafe fn sym_shndx(s:*mut Elf_Sym)->u16{(elf_parser.sym_shndx.unwrap())(s)}
#[inline] pub unsafe fn sym64_type(s:*mut Elf_Sym)->u8{((*s).e64.st_info)&0xf}
#[inline] pub unsafe fn sym32_type(s:*mut Elf_Sym)->u8{((*s).e32.st_info)&0xf}
#[inline] pub unsafe fn sym_type(s:*mut Elf_Sym)->u8{(elf_parser.sym_type.unwrap())(s)}

#[inline] pub unsafe fn rela64_offset(r:*mut Elf_Rela)->u64{(elf_parser.r8.unwrap())(&(*r).e64.r_offset)}
#[inline] pub unsafe fn rela32_offset(r:*mut Elf_Rela)->u64{(elf_parser.r.unwrap())(&(*r).e32.r_offset) as u64}
#[inline] pub unsafe fn rela_offset(r:*mut Elf_Rela)->u64{(elf_parser.rela_offset.unwrap())(r)}
#[inline] pub unsafe fn rela64_info(r:*mut Elf_Rela)->u64{(elf_parser.r8.unwrap())(&(*r).e64.r_info)}
#[inline] pub unsafe fn rela32_info(r:*mut Elf_Rela)->u64{(elf_parser.r.unwrap())(&(*r).e32.r_info) as u64}
#[inline] pub unsafe fn rela_info(r:*mut Elf_Rela)->u64{(elf_parser.rela_info.unwrap())(r)}
#[inline] pub unsafe fn rela64_addend(r:*mut Elf_Rela)->u64{(elf_parser.r8.unwrap())(&(*r).e64.r_addend)}
#[inline] pub unsafe fn rela32_addend(r:*mut Elf_Rela)->u64{(elf_parser.r.unwrap())(&(*r).e32.r_addend) as u64}
#[inline] pub unsafe fn rela_addend(r:*mut Elf_Rela)->u64{(elf_parser.rela_addend.unwrap())(r)}
#[inline] pub unsafe fn rela64_write_addend(r:*mut Elf_Rela, val:u64){(elf_parser.w8.unwrap())(val,&mut (*r).e64.r_addend)}
#[inline] pub unsafe fn rela32_write_addend(r:*mut Elf_Rela, val:u64){(elf_parser.w.unwrap())(val as u32,&mut (*r).e32.r_addend)}

extern "C" {
    pub fn get_unaligned_be32(x: *const u32) -> u32;
    pub fn get_unaligned_be16(x: *const u16) -> u16;
    pub fn get_unaligned_be64(x: *const u64) -> u64;
    pub fn get_unaligned_le32(x: *const u32) -> u32;
    pub fn get_unaligned_le16(x: *const u16) -> u16;
    pub fn get_unaligned_le64(x: *const u64) -> u64;
    pub fn put_unaligned_be32(val: u32, x: *mut u32);
    pub fn put_unaligned_le32(val: u32, x: *mut u32);
    pub fn put_unaligned_be64(val: u64, x: *mut u64);
    pub fn put_unaligned_le64(val: u64, x: *mut u64);
    pub fn elf_map(fname: *const core::ffi::c_char, size: *mut usize, types: u32) -> *mut core::ffi::c_void;
    pub fn elf_unmap(addr: *mut core::ffi::c_void, size: usize);
    pub fn elf_map_machine(addr: *mut core::ffi::c_void) -> i32;
    pub fn elf_map_long_size(addr: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
