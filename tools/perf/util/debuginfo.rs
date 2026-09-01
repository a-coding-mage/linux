// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DWARF debug information handling code.  Copied from probe-finder.c.
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type Elf32_Word = c_uint;
type Dwarf_Addr = u64;
type GElf_Addr = u64;

const O_RDONLY: c_int = 0;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct Dwfl {
	_private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Module {
	_private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct Elf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
	_private: [u8; 0],
}

#[repr(C)]
pub struct debuginfod_client {
	_private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Shdr {
	pub sh_name: u32,
	pub sh_type: u32,
	pub sh_flags: u64,
	pub sh_addr: u64,
	pub sh_offset: u64,
	pub sh_size: u64,
	pub sh_link: u32,
	pub sh_info: u32,
	pub sh_addralign: u64,
	pub sh_entsize: u64,
}

#[repr(C)]
pub struct Dwfl_Callbacks {
	pub find_debuginfo: *const c_void,
	pub debuginfo_path: *mut *mut c_char,
	pub section_address: *const c_void,
	pub find_elf: *const c_void,
}

#[repr(C)]
pub struct build_id {
	pub size: usize,
}

#[repr(C)]
pub struct dso {
	_private: [u8; 0],
}

#[repr(C)]
pub struct debuginfo {
	pub dwfl: *mut Dwfl,
	pub mod_: *mut Dwfl_Module,
	pub dbg: *mut Dwarf,
	pub bias: Dwarf_Addr,
	pub build_id: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_binary_type {
	DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
	DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
	DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
	DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
	DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
	DSO_BINARY_TYPE__NOT_FOUND,
}

unsafe extern "C" {
	static dwfl_standard_find_debuginfo: c_void;
	static dwfl_offline_section_address: c_void;
	static dwfl_build_id_find_elf: c_void;

	fn open(path: *const c_char, flags: c_int, ...) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn free(ptr: *mut c_void);
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

	fn zalloc(size: size_t) -> *mut c_void;

	fn dwfl_begin(callbacks: *const Dwfl_Callbacks) -> *mut Dwfl;
	fn dwfl_end(dwfl: *mut Dwfl);
	fn dwfl_report_begin(dwfl: *mut Dwfl);
	fn dwfl_report_offline(
		dwfl: *mut Dwfl,
		name: *const c_char,
		file_name: *const c_char,
		fd: c_int,
	) -> *mut Dwfl_Module;
	fn dwfl_module_getdwarf(module: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Dwarf;
	fn dwfl_module_build_id(
		module: *mut Dwfl_Module,
		build_id: *mut *const c_void,
		vaddr: *mut GElf_Addr,
	) -> c_int;
	fn dwfl_report_end(
		dwfl: *mut Dwfl,
		removed: *mut *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn dwfl_module_getelf(module: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Elf;
	fn dwfl_module_relocations(module: *mut Dwfl_Module) -> c_int;
	fn dwfl_module_relocation_info(
		module: *mut Dwfl_Module,
		idx: c_int,
		shndx: *mut Elf32_Word,
	) -> *const c_char;

	fn elf_getscn(elf: *mut Elf, index: Elf32_Word) -> *mut Elf_Scn;
	fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;

	fn dso__new(path: *const c_char) -> *mut dso;
	fn dso__has_build_id(dso: *mut dso) -> bool_;
	fn filename__read_build_id(path: *const c_char, bid: *mut build_id) -> c_int;
	fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
	fn dso__read_binary_type_filename(
		dso: *mut dso,
		type_: dso_binary_type,
		root_dir: *mut c_char,
		filename: *mut c_char,
		size: size_t,
	) -> c_int;
	fn dso__put(dso: *mut dso);
	fn symbol__join_symfs(dst: *mut c_char, path: *const c_char);
	fn pr_debug(fmt: *const c_char, ...);

	/* Requires HAVE_DEBUGINFOD_SUPPORT in the original C build. */
	fn debuginfod_begin() -> *mut debuginfod_client;
	fn debuginfod_find_source(
		client: *mut debuginfod_client,
		build_id: *const u8,
		build_id_len: c_int,
		path: *const c_char,
		new_path: *mut *mut c_char,
	) -> c_int;
	fn debuginfod_end(client: *mut debuginfod_client);
}

/* Dwarf FL wrappers */
static mut debuginfo_path: *mut c_char = ptr::null_mut();	/* Currently dummy */

static offline_callbacks: Dwfl_Callbacks = Dwfl_Callbacks {
	find_debuginfo: unsafe { &dwfl_standard_find_debuginfo as *const c_void },
	debuginfo_path: unsafe { &raw mut debuginfo_path },

	section_address: unsafe { &dwfl_offline_section_address as *const c_void },

	/* We use this table for core files too.  */
	find_elf: unsafe { &dwfl_build_id_find_elf as *const c_void },
};

/* Get a Dwarf from offline image */
unsafe fn debuginfo__init_offline_dwarf(dbg: *mut debuginfo, path: *const c_char) -> c_int {
	let mut dummy: GElf_Addr = 0;
	let fd: c_int;
	let mut fd_consumed: bool_ = false;

	fd = open(path, O_RDONLY);
	if fd < 0 {
		return fd;
	}

	(*dbg).dwfl = dwfl_begin(&offline_callbacks);
	if (*dbg).dwfl.is_null() {
		goto_error(dbg, fd, fd_consumed);
		return -ENOENT;
	}

	dwfl_report_begin((*dbg).dwfl);
	(*dbg).mod_ = dwfl_report_offline((*dbg).dwfl, c"".as_ptr(), c"".as_ptr(), fd);
	if (*dbg).mod_.is_null() {
		goto_error(dbg, fd, fd_consumed);
		return -ENOENT;
	}
	fd_consumed = true;

	(*dbg).dbg = dwfl_module_getdwarf((*dbg).mod_, &mut (*dbg).bias);
	if (*dbg).dbg.is_null() {
		goto_error(dbg, fd, fd_consumed);
		return -ENOENT;
	}

	dwfl_module_build_id((*dbg).mod_, &mut (*dbg).build_id, &mut dummy);

	if dwfl_report_end((*dbg).dwfl, ptr::null_mut(), ptr::null_mut()) != 0 {
		goto_error(dbg, fd, fd_consumed);
		return -ENOENT;
	}

	0
}

unsafe fn goto_error(dbg: *mut debuginfo, fd: c_int, fd_consumed: bool_) {
	if !(*dbg).dwfl.is_null() {
		dwfl_end((*dbg).dwfl);
	}
	if !fd_consumed {
		close(fd);
	}
	memset(dbg as *mut c_void, 0, mem::size_of::<debuginfo>());
}

unsafe fn zfree_dbg(dbg: *mut *mut debuginfo) {
	if !(*dbg).is_null() {
		free(*dbg as *mut c_void);
		*dbg = ptr::null_mut();
	}
}

unsafe fn __debuginfo__new(path: *const c_char) -> *mut debuginfo {
	let mut dbg = zalloc(mem::size_of::<debuginfo>()) as *mut debuginfo;
	if dbg.is_null() {
		return ptr::null_mut();
	}

	if debuginfo__init_offline_dwarf(dbg, path) < 0 {
		zfree_dbg(&mut dbg);
	}
	if !dbg.is_null() {
		pr_debug(c"Open Debuginfo file: %s\n".as_ptr(), path);
	}
	dbg
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn debuginfo__new(path: *const c_char) -> *mut debuginfo {
	static distro_dwarf_types: [dso_binary_type; 6] = [
		dso_binary_type::DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
		dso_binary_type::DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
		dso_binary_type::DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
		dso_binary_type::DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
		dso_binary_type::DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
		dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND,
	];
	let mut type_: *const dso_binary_type;
	let mut buf: [c_char; PATH_MAX] = [0; PATH_MAX];
	let mut nil: c_char = b'\0' as c_char;
	let dso: *mut dso;
	let mut dinfo: *mut debuginfo = ptr::null_mut();
	let mut bid: build_id = build_id { size: 0 };

	/* Try to open distro debuginfo files */
	dso = dso__new(path);
	if dso.is_null() {
		goto_out(path, &mut buf, dinfo)
	} else {
		/*
		 * Set the build id for DSO_BINARY_TYPE__BUILDID_DEBUGINFO. Don't block
		 * incase the path isn't for a regular file.
		 */
		assert!(!dso__has_build_id(dso));
		if filename__read_build_id(path, &mut bid) > 0 {
			dso__set_build_id(dso, &bid);
		}

		type_ = distro_dwarf_types.as_ptr();
		while dinfo.is_null() && *type_ != dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND {
			if dso__read_binary_type_filename(
				dso,
				*type_,
				&mut nil,
				buf.as_mut_ptr(),
				PATH_MAX,
			) < 0
			{
				type_ = type_.add(1);
				continue;
			}
			dinfo = __debuginfo__new(buf.as_mut_ptr());
			type_ = type_.add(1);
		}
		dso__put(dso);

		goto_out(path, &mut buf, dinfo)
	}
}

unsafe fn goto_out(
	path: *const c_char,
	buf: &mut [c_char; PATH_MAX],
	dinfo: *mut debuginfo,
) -> *mut debuginfo {
	if !dinfo.is_null() {
		return dinfo;
	}

	/* if failed to open all distro debuginfo, open given binary */
	symbol__join_symfs(buf.as_mut_ptr(), path);
	__debuginfo__new(buf.as_mut_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn debuginfo__delete(dbg: *mut debuginfo) {
	if !dbg.is_null() {
		if !(*dbg).dwfl.is_null() {
			dwfl_end((*dbg).dwfl);
		}
		free(dbg as *mut c_void);
	}
}

/* For the kernel module, we need a special code to get a DIE */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debuginfo__get_text_offset(
	dbg: *mut debuginfo,
	offs: *mut Dwarf_Addr,
	adjust_offset: bool_,
) -> c_int {
	let n: c_int;
	let mut i: c_int;
	let mut shndx: Elf32_Word = 0;
	let mut scn: *mut Elf_Scn;
	let elf: *mut Elf;
	let mut mem: GElf_Shdr = mem::zeroed();
	let mut shdr: *mut GElf_Shdr;
	let mut p: *const c_char;

	elf = dwfl_module_getelf((*dbg).mod_, &mut (*dbg).bias);
	if elf.is_null() {
		return -EINVAL;
	}

	/* Get the number of relocations */
	n = dwfl_module_relocations((*dbg).mod_);
	if n < 0 {
		return -ENOENT;
	}
	/* Search the relocation related .text section */
	i = 0;
	while i < n {
		p = dwfl_module_relocation_info((*dbg).mod_, i, &mut shndx);
		if !p.is_null() && strcmp(p, c".text".as_ptr()) == 0 {
			/* OK, get the section header */
			scn = elf_getscn(elf, shndx);
			if scn.is_null() {
				return -ENOENT;
			}
			shdr = gelf_getshdr(scn, &mut mem);
			if shdr.is_null() {
				return -ENOENT;
			}
			*offs = (*shdr).sh_addr;
			if adjust_offset {
				*offs = (*offs).wrapping_sub((*shdr).sh_offset);
			}
		}
		i += 1;
	}
	0
}

/* Original C condition: #ifdef HAVE_DEBUGINFOD_SUPPORT */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_source_from_debuginfod(
	raw_path: *const c_char,
	sbuild_id: *const c_char,
	new_path: *mut *mut c_char,
) -> c_int {
	let c: *mut debuginfod_client = debuginfod_begin();
	let p: *const c_char = raw_path;
	let fd: c_int;

	if c.is_null() {
		return -ENOMEM;
	}

	fd = debuginfod_find_source(c, sbuild_id as *const u8, 0, p, new_path);
	pr_debug(c"Search %s from debuginfod -> %d\n".as_ptr(), p, fd);
	if fd >= 0 {
		close(fd);
	}
	debuginfod_end(c);
	if fd < 0 {
		pr_debug(
			c"Failed to find %s in debuginfod (%s)\n".as_ptr(),
			raw_path,
			sbuild_id,
		);
		return -ENOENT;
	}
	pr_debug(c"Got a source %s\n".as_ptr(), *new_path);

	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
