// SPDX-License-Identifier: GPL-2.0

// Kernel headers and symbols referenced by this translation are supplied by
// other files/dependencies.

use core::ffi::c_void;
use core::mem::{size_of, MaybeUninit};
use core::ptr;

const BUILD_ID: u32 = 3;
const MAX_PHDR_CNT: u32 = 256;
const MAX_FREADER_BUF_SZ: usize = 64;

#[repr(C)]
pub struct freader {
    pub buf: *mut c_void,
    pub buf_sz: u32,
    pub file: *mut file,
    pub may_fault: bool,
    pub data: *const i8,
    pub data_sz: u64,
    pub folio: *mut folio,
    pub folio_off: i64,
    pub addr: *mut u8,
    pub err: i32,
}

#[repr(C)] pub struct file { pub f_mapping: *mut address_space }
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file }

#[repr(C)]
pub struct Elf32_Nhdr { pub n_namesz: u32, pub n_descsz: u32, pub n_type: u32 }
#[repr(C)]
pub struct Elf32_Ehdr { pub e_ident: [u8; 16], pub e_type: u16, pub e_phoff: u32, pub e_phnum: u16 }
#[repr(C)]
pub struct Elf64_Ehdr { pub e_ident: [u8; 16], pub e_type: u16, pub e_phoff: u64, pub e_phnum: u16 }
#[repr(C)]
pub struct Elf32_Phdr { pub p_type: u32, pub p_offset: u32, pub p_filesz: u32 }
#[repr(C)]
pub struct Elf64_Phdr { pub p_type: u32, pub p_offset: u64, pub p_filesz: u64 }

extern "C" {
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn kunmap_local(addr: *mut u8);
    fn folio_put(folio: *mut folio);
    fn filemap_get_folio(mapping: *mut address_space, index: i64) -> *mut folio;
    fn folio_test_uptodate(folio: *mut folio) -> bool;
    fn folio_size(folio: *mut folio) -> usize;
    fn folio_pos(folio: *mut folio) -> i64;
    fn kmap_local_folio(folio: *mut folio, offset: usize) -> *mut u8;
    fn secretmem_mapping(mapping: *mut address_space) -> bool;
    fn __kernel_read(file: *mut file, buf: *mut c_void, count: usize, pos: *mut i64) -> isize;
}

const EFAULT: i32 = 14;
const E2BIG: i32 = 7;
const EOVERFLOW: i32 = 75;
const ERANGE: i32 = 34;
const EIO: i32 = 5;
const EINVAL: i32 = 22;
const PAGE_SHIFT: u32 = 12;
const PT_NOTE: u32 = 4;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const SELFMAG: usize = 4;
const ELFMAG: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const EI_CLASS: usize = 4;
const BUILD_ID_SIZE_MAX: usize = 64;

pub unsafe fn freader_init_from_file(r: *mut freader, buf: *mut c_void, buf_sz: u32, file: *mut file, may_fault: bool) {
    memset(r.cast(), 0, size_of::<freader>());
    (*r).buf = buf; (*r).buf_sz = buf_sz; (*r).file = file; (*r).may_fault = may_fault;
}

pub unsafe fn freader_init_from_mem(r: *mut freader, data: *const i8, data_sz: u64) {
    memset(r.cast(), 0, size_of::<freader>());
    (*r).data = data; (*r).data_sz = data_sz;
}

unsafe fn freader_put_folio(r: *mut freader) {
    if (*r).folio.is_null() { return; }
    kunmap_local((*r).addr); folio_put((*r).folio); (*r).folio = ptr::null_mut();
}

unsafe fn freader_get_folio(r: *mut freader, file_off: i64) -> i32 {
    if !(*r).folio.is_null() && file_off >= (*r).folio_off && file_off < (*r).folio_off + folio_size((*r).folio) as i64 { return 0; }
    freader_put_folio(r);
    (*r).folio = filemap_get_folio((*(*r).file).f_mapping, file_off >> PAGE_SHIFT);
    if (*r).folio.is_null() || !folio_test_uptodate((*r).folio) {
        if !(*r).folio.is_null() { folio_put((*r).folio); }
        (*r).folio = ptr::null_mut(); return -EFAULT;
    }
    (*r).folio_off = folio_pos((*r).folio); (*r).addr = kmap_local_folio((*r).folio, 0); 0
}

pub unsafe fn freader_fetch(r: *mut freader, file_off: i64, sz: usize) -> *const u8 {
    if !(*r).buf.is_null() && sz > (*r).buf_sz as usize { (*r).err = -E2BIG; return ptr::null(); }
    if file_off.checked_add(sz as i64).is_none() { (*r).err = -EOVERFLOW; return ptr::null(); }
    if (*r).buf.is_null() {
        if file_off as u64 + sz as u64 > (*r).data_sz { (*r).err = -ERANGE; return ptr::null(); }
        return (*r).data.cast::<u8>().add(file_off as usize);
    }
    if secretmem_mapping((*(*r).file).f_mapping) { (*r).err = -EFAULT; return ptr::null(); }
    if (*r).may_fault {
        let ret = __kernel_read((*r).file, (*r).buf, sz, &mut *(MaybeUninit::<i64>::zeroed().assume_init_mut()));
        if ret != sz as isize { (*r).err = if ret < 0 { ret as i32 } else { -EIO }; return ptr::null(); }
        return (*r).buf.cast();
    }
    (*r).err = freader_get_folio(r, file_off); if (*r).err != 0 { return ptr::null(); }
    let mut folio_sz = folio_size((*r).folio);
    if file_off + sz as i64 > (*r).folio_off + folio_sz as i64 {
        let part_sz = ((*r).folio_off + folio_sz as i64 - file_off) as usize;
        memcpy((*r).buf, (*r).addr.add((file_off - (*r).folio_off) as usize).cast(), part_sz);
        let mut off = part_sz;
        while off < sz {
            (*r).err = freader_get_folio(r, (*r).folio_off + folio_sz as i64); if (*r).err != 0 { return ptr::null(); }
            folio_sz = folio_size((*r).folio); let part = core::cmp::min(sz - off, folio_sz);
            memcpy((*r).buf.cast::<u8>().add(off).cast(), (*r).addr.cast(), part); off += part;
        }
        return (*r).buf.cast();
    }
    (*r).addr.add((file_off - (*r).folio_off) as usize)
}

pub unsafe fn freader_cleanup(r: *mut freader) { if !(*r).buf.is_null() { freader_put_folio(r); } }

// Parse build id from the note segment. This logic can be shared between 32-bit and 64-bit systems.
unsafe fn parse_build_id(r: *mut freader, build_id: *mut u8, size: *mut u32, mut note_off: i64, note_size: u32) -> i32 {
    let note_name = b"GNU\0"; let note_name_sz = note_name.len();
    let note_end = match note_off.checked_add(note_size as i64) { Some(v) => v, None => return -EINVAL };
    while note_end - note_off > (size_of::<Elf32_Nhdr>() + note_name_sz) as i64 {
        let nhdr = freader_fetch(r, note_off, size_of::<Elf32_Nhdr>() + note_name_sz) as *const Elf32_Nhdr; if nhdr.is_null() { return (*r).err; }
        let name_sz = (*nhdr).n_namesz; let desc_sz = (*nhdr).n_descsz;
        let mut new_off = note_off + size_of::<Elf32_Nhdr>() as i64;
        let align = |x: u32| (x + 3) & !3;
        new_off = match new_off.checked_add(align(name_sz) as i64).and_then(|x| x.checked_add(align(desc_sz) as i64)) { Some(v) => v, None => break };
        if new_off > note_end { break; }
        if (*nhdr).n_type == BUILD_ID && name_sz as usize == note_name_sz && memcmp(nhdr.add(1).cast(), note_name.as_ptr().cast(), note_name_sz) == 0 && desc_sz > 0 && desc_sz as usize <= BUILD_ID_SIZE_MAX {
            let build_id_off = note_off + size_of::<Elf32_Nhdr>() as i64 + align(note_name_sz as u32) as i64;
            let data = freader_fetch(r, build_id_off, desc_sz as usize); if data.is_null() { return (*r).err; }
            memcpy(build_id.cast(), data.cast(), desc_sz as usize); memset(build_id.add(desc_sz as usize).cast(), 0, BUILD_ID_SIZE_MAX - desc_sz as usize); if !size.is_null() { *size = desc_sz; } return 0;
        }
        note_off = new_off;
    }
    -EINVAL
}

unsafe fn get_build_id_32(r: *mut freader, build_id: *mut u8, size: *mut u32) -> i32 {
    let ehdr = freader_fetch(r, 0, size_of::<Elf32_Ehdr>()) as *const Elf32_Ehdr; if ehdr.is_null() { return (*r).err; }
    let mut phnum = (*ehdr).e_phnum as u32; let phoff = (*ehdr).e_phoff; if phnum > MAX_PHDR_CNT { phnum = MAX_PHDR_CNT; }
    if phoff.wrapping_add(phnum.wrapping_mul(size_of::<Elf32_Phdr>() as u32)) < phoff { return -EINVAL; }
    for i in 0..phnum { let phdr = freader_fetch(r, (phoff + i * size_of::<Elf32_Phdr>() as u32) as i64, size_of::<Elf32_Phdr>()) as *const Elf32_Phdr; if phdr.is_null() { return (*r).err; } if (*phdr).p_type == PT_NOTE && parse_build_id(r, build_id, size, (*phdr).p_offset as i64, (*phdr).p_filesz) == 0 { return 0; } } -EINVAL
}

unsafe fn get_build_id_64(r: *mut freader, build_id: *mut u8, size: *mut u32) -> i32 {
    let ehdr = freader_fetch(r, 0, size_of::<Elf64_Ehdr>()) as *const Elf64_Ehdr; if ehdr.is_null() { return (*r).err; }
    let mut phnum = (*ehdr).e_phnum as u32; let phoff = (*ehdr).e_phoff; if phnum > MAX_PHDR_CNT { phnum = MAX_PHDR_CNT; }
    if phoff.wrapping_add(phnum as u64 * size_of::<Elf64_Phdr>() as u64) < phoff { return -EINVAL; }
    for i in 0..phnum { let phdr = freader_fetch(r, phoff.wrapping_add(i as u64 * size_of::<Elf64_Phdr>() as u64) as i64, size_of::<Elf64_Phdr>()) as *const Elf64_Phdr; if phdr.is_null() { return (*r).err; } if (*phdr).p_type == PT_NOTE && parse_build_id(r, build_id, size, (*phdr).p_offset as i64, (*phdr).p_filesz as u32) == 0 { return 0; } } -EINVAL
}

unsafe fn __build_id_parse(file: *mut file, build_id: *mut u8, size: *mut u32, may_fault: bool) -> i32 {
    let mut r = MaybeUninit::<freader>::zeroed(); let mut buf = [0u8; MAX_FREADER_BUF_SZ]; freader_init_from_file(r.as_mut_ptr(), buf.as_mut_ptr().cast(), buf.len() as u32, file, may_fault); let r = r.as_mut_ptr();
    let ehdr = freader_fetch(r, 0, 18) as *const Elf32_Ehdr; if ehdr.is_null() { let ret = (*r).err; freader_cleanup(r); return ret; }
    let mut ret = -EINVAL; if memcmp((*ehdr).e_ident.as_ptr().cast(), ELFMAG.as_ptr().cast(), SELFMAG) != 0 { freader_cleanup(r); return ret; }
    if (*ehdr).e_type != ET_EXEC && (*ehdr).e_type != ET_DYN { freader_cleanup(r); return ret; }
    if (*ehdr).e_ident[EI_CLASS] == ELFCLASS32 { ret = get_build_id_32(r, build_id, size); } else if (*ehdr).e_ident[EI_CLASS] == ELFCLASS64 { ret = get_build_id_64(r, build_id, size); }
    freader_cleanup(r); ret
}

pub unsafe fn build_id_parse_nofault(vma: *mut vm_area_struct, build_id: *mut u8, size: *mut u32) -> i32 { if (*vma).vm_file.is_null() { return -EINVAL; } __build_id_parse((*vma).vm_file, build_id, size, false) }
pub unsafe fn build_id_parse(vma: *mut vm_area_struct, build_id: *mut u8, size: *mut u32) -> i32 { if (*vma).vm_file.is_null() { return -EINVAL; } __build_id_parse((*vma).vm_file, build_id, size, true) }
pub unsafe fn build_id_parse_file(file: *mut file, build_id: *mut u8, size: *mut u32) -> i32 { __build_id_parse(file, build_id, size, true) }
pub unsafe fn build_id_parse_buf(buf: *const c_void, build_id: *mut u8, buf_size: u32) -> i32 { let mut r = MaybeUninit::<freader>::zeroed(); freader_init_from_mem(r.as_mut_ptr(), buf.cast(), buf_size as u64); let err = parse_build_id(r.as_mut_ptr(), build_id, ptr::null_mut(), 0, buf_size); freader_cleanup(r.as_mut_ptr()); err }

// Conditional on CONFIG_STACKTRACE_BUILD_ID || CONFIG_VMCORE_INFO in the source build.
#[cfg(any(feature = "CONFIG_STACKTRACE_BUILD_ID", feature = "CONFIG_VMCORE_INFO"))]
#[no_mangle] pub static mut vmlinux_build_id: [u8; BUILD_ID_SIZE_MAX] = [0; BUILD_ID_SIZE_MAX];

#[cfg(any(feature = "CONFIG_STACKTRACE_BUILD_ID", feature = "CONFIG_VMCORE_INFO"))]
pub unsafe fn init_vmlinux_build_id() { extern "C" { static __start_notes: c_void; static __stop_notes: c_void; } let size = (&__stop_notes as *const _ as usize - &__start_notes as *const _ as usize) as u32; build_id_parse_buf(&__start_notes, vmlinux_build_id.as_mut_ptr(), size); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
