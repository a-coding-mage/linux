// SPDX-License-Identifier: GPL-2.0
/* Kernel Debugger Architecture Independent Support Functions */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Types, constants, macros, and external functions are supplied by the kernel
// and kdb headers corresponding to the original C translation unit.
extern "C" {
    fn kdb_dbg_printf(level: c_int, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kallsyms_lookup_name(name: *const c_char) -> usize;
    fn kallsyms_lookup(addr: usize, symbolsize: *mut usize, offset: *mut usize,
                       modname: *mut *mut c_char, namebuf: *mut c_char) -> *mut c_char;
    fn kdb_walk_kallsyms(pos: *mut i64) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kdb_printf(fmt: *const c_char, ...);
    fn kmalloc(size: usize, flags: usize) -> *mut c_char;
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn copy_to_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn kdb_func_printf(fmt: *const c_char, ...);
    fn pfn_valid(pfn: usize) -> bool;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn kmap_local_page(page: *mut page) -> *mut c_void;
    fn kunmap_local(addr: *mut c_void);
    fn kdb_getarea<T>(value: *mut T, addr: usize) -> c_int;
    fn kdb_putarea<T>(addr: usize, value: T) -> c_int;
    fn task_state_to_char(p: *mut task_struct) -> c_char;
    fn is_idle_task(p: *const task_struct) -> bool;
    fn kdb_process_cpu(p: *const task_struct) -> c_int;
    fn kdb_task_has_cpu(p: *const task_struct) -> bool;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn tolower(c: c_int) -> c_int;
}

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct kdb_symtab_t {
    pub sym_start: usize, pub sym_end: usize,
    pub sym_name: *mut c_char, pub mod_name: *mut c_char,
}

static mut KS_NAMEBUF: [c_char; KSYM_NAME_LEN + 1] = [0; KSYM_NAME_LEN + 1];
static mut KS_NAMEBUF_PREV: [c_char; KSYM_NAME_LEN + 1] = [0; KSYM_NAME_LEN + 1];
static mut NAMEBUF: [c_char; KSYM_NAME_LEN] = [0; KSYM_NAME_LEN];
static mut POS: i64 = 0;

pub unsafe fn kdbgetsymval(symname: *const c_char, symtab: *mut kdb_symtab_t) -> c_int {
    kdb_dbg_printf(AR, b"symname=%s, symtab=%px\0".as_ptr() as _, symname, symtab);
    memset(symtab as _, 0, core::mem::size_of::<kdb_symtab_t>());
    (*symtab).sym_start = kallsyms_lookup_name(symname);
    if (*symtab).sym_start != 0 { kdb_dbg_printf(AR, b"returns 1, symtab->sym_start=0x%lx\n\0".as_ptr() as _, (*symtab).sym_start); return 1; }
    kdb_dbg_printf(AR, b"returns 0\n\0".as_ptr() as _); 0
}

pub unsafe fn kdbnearsym(mut addr: usize, symtab: *mut kdb_symtab_t) -> c_int {
    let mut ret = 0; let mut symbolsize = 0usize; let mut offset = 0usize;
    kdb_dbg_printf(AR, b"addr=0x%lx, symtab=%px\n\0".as_ptr() as _, addr, symtab);
    memset(symtab as _, 0, core::mem::size_of::<kdb_symtab_t>());
    if addr < 4096 { return ret; }
    (*symtab).sym_name = kallsyms_lookup(addr, &mut symbolsize, &mut offset, &mut (*symtab).mod_name, NAMEBUF.as_mut_ptr());
    if offset > 8 * 1024 * 1024 { (*symtab).sym_name = core::ptr::null_mut(); addr = 0; offset = 0; symbolsize = 0; }
    (*symtab).sym_start = addr.wrapping_sub(offset); (*symtab).sym_end = (*symtab).sym_start.wrapping_add(symbolsize);
    ret = (!(*symtab).sym_name.is_null() && *(*symtab).sym_name != 0) as c_int;
    if (*symtab).mod_name.is_null() { (*symtab).mod_name = b"kernel\0".as_ptr() as *mut c_char; }
    kdb_dbg_printf(AR, b"returns %d symtab->sym_start=0x%lx, symtab->mod_name=%px, symtab->sym_name=%px (%s)\n\0".as_ptr() as _, ret, (*symtab).sym_start, (*symtab).mod_name, (*symtab).sym_name, (*symtab).sym_name); ret
}

pub unsafe fn kallsyms_symbol_complete(prefix_name: *mut c_char, max_len: c_int) -> c_int {
    let prefix_len = strlen(prefix_name); let mut prev_len = 0usize; let mut number = 0;
    loop { let name = kdb_walk_kallsyms(&mut 0); if name.is_null() { break; } if strncmp(name, prefix_name, prefix_len) == 0 { strscpy(KS_NAMEBUF.as_mut_ptr(), name, KS_NAMEBUF.len()); number += 1; if number == 1 { prev_len = core::cmp::min((max_len - 1) as usize, strlen(KS_NAMEBUF.as_ptr())); memcpy(KS_NAMEBUF_PREV.as_mut_ptr() as _, KS_NAMEBUF.as_ptr() as _, prev_len); KS_NAMEBUF_PREV[prev_len] = 0; continue; } for i in 0..prev_len { if KS_NAMEBUF[i] != KS_NAMEBUF_PREV[i] { prev_len = i; KS_NAMEBUF_PREV[i] = 0; break; } } } }
    if prev_len > prefix_len { memcpy(prefix_name as _, KS_NAMEBUF_PREV.as_ptr() as _, prev_len + 1); } number
}

pub unsafe fn kallsyms_symbol_next(prefix_name: *mut c_char, flag: c_int, buf_size: c_int) -> c_int {
    let prefix_len = strlen(prefix_name); if flag == 0 { POS = 0; }
    loop { let name = kdb_walk_kallsyms(&mut POS); if name.is_null() { return 0; } if strncmp(name, prefix_name, prefix_len) == 0 { return strscpy(prefix_name, name, buf_size as usize) as c_int; } }
}

pub unsafe fn kdb_strdup(str_: *const c_char, ty: usize) -> *mut c_char { let n = strlen(str_) + 1; let s = kmalloc(n, ty); if s.is_null() { return core::ptr::null_mut(); } memcpy(s as _, str_ as _, n); s }
pub unsafe fn kdb_strdup_dequote(mut str_: *const c_char, ty: usize) -> *mut c_char { let mut len = strlen(str_); if *str_ == b'"' as c_char && len > 1 && *str_.add(len-1) == b'"' as c_char { str_ = str_.add(1); len -= 2; } len += 1; let s = kmalloc(len, ty); if s.is_null() { return core::ptr::null_mut(); } memcpy(s as _, str_ as _, len-1); *s.add(len-1)=0; s }

pub unsafe fn kdb_getarea_size(res: *mut c_void, addr: usize, size: usize) -> c_int { let mut ret=copy_from_kernel_nofault(res, addr as _, size); if ret!=0 { if !KDB_STATE(SUPPRESS) { kdb_func_printf(b"Bad address 0x%lx\n\0".as_ptr() as _,addr); KDB_STATE_SET(SUPPRESS); } ret=KDB_BADADDR; } else { KDB_STATE_CLEAR(SUPPRESS); } ret }
pub unsafe fn kdb_putarea_size(addr: usize, res: *mut c_void, size: usize) -> c_int { let mut ret=copy_to_kernel_nofault(addr as _,res,size); if ret!=0 { if !KDB_STATE(SUPPRESS) { kdb_func_printf(b"Bad address 0x%lx\n\0".as_ptr() as _,addr); KDB_STATE_SET(SUPPRESS); } ret=KDB_BADADDR; } else { KDB_STATE_CLEAR(SUPPRESS); } ret }

pub unsafe fn kdb_symbol_print(addr: usize, supplied: *const kdb_symtab_t, punc: c_uint) {
    let mut local = kdb_symtab_t { sym_start:0, sym_end:0, sym_name:core::ptr::null_mut(), mod_name:core::ptr::null_mut() };
    let s = if supplied.is_null() { kdbnearsym(addr, &mut local); &local } else { &*supplied };
    if s.sym_name.is_null() && (punc & KDB_SP_VALUE)==0 { return; }
    if (punc & KDB_SP_SPACEB)!=0 { kdb_printf(b" \0".as_ptr() as _); }
    if (punc & KDB_SP_VALUE)!=0 { kdb_printf(kdb_machreg_fmt0, addr); }
    if !s.sym_name.is_null() { if (punc & KDB_SP_VALUE)!=0 { kdb_printf(b" \0".as_ptr() as _); } if (punc & KDB_SP_PAREN)!=0 { kdb_printf(b"(\0".as_ptr() as _); } if strcmp(s.mod_name,b"kernel\0".as_ptr() as _)!=0 { kdb_printf(b"[%s]\0".as_ptr() as _,s.mod_name); } kdb_printf(b"%s\0".as_ptr() as _,s.sym_name); if addr!=s.sym_start { kdb_printf(b"+0x%lx\0".as_ptr() as _,addr-s.sym_start); } if (punc & KDB_SP_SYMSIZE)!=0 { kdb_printf(b"/0x%lx\0".as_ptr() as _,s.sym_end-s.sym_start); } if (punc & KDB_SP_PAREN)!=0 { kdb_printf(b")\0".as_ptr() as _); } }
    if (punc & KDB_SP_SPACEA)!=0 { kdb_printf(b" \0".as_ptr() as _); } if (punc & KDB_SP_NEWLINE)!=0 { kdb_printf(b"\n\0".as_ptr() as _); }
}

pub unsafe fn kdb_getword(word:*mut usize,addr:usize,size:usize)->c_int { *word=0; let mut b=[0u8;8]; if size!=1&&size!=2&&size!=4&&size!=8 { kdb_func_printf(b"bad width %zu\n\0".as_ptr() as _,size); return KDB_BADWIDTH; } let d=kdb_getarea(b.as_mut_ptr(),addr); if d==0 { core::ptr::copy_nonoverlapping(b.as_ptr(),word as _,size); } d }
pub unsafe fn kdb_putword(addr:usize,word:usize,size:usize)->c_int { if size!=1&&size!=2&&size!=4&&size!=8 { kdb_func_printf(b"bad width %zu\n\0".as_ptr() as _,size); return KDB_BADWIDTH; } kdb_putarea(addr,word) }

// Remaining low-level task/word helpers retain the kernel-provided types and macros.
pub unsafe fn kdb_getphysword(word: *mut usize, addr: usize, size: usize) -> c_int { *word=0; let mut b=[0u8;8]; if size!=1&&size!=2&&size!=4&&size!=8 { kdb_func_printf(b"bad width %zu\n\0".as_ptr() as _,size); return KDB_BADWIDTH; } let d=kdb_getphys(b.as_mut_ptr() as _,addr,size); if d==0 { core::ptr::copy_nonoverlapping(b.as_ptr(),word as _,size); } d }
unsafe fn kdb_getphys(res:*mut c_void,addr:usize,size:usize)->c_int { let pfn=addr>>PAGE_SHIFT; if !pfn_valid(pfn){return 1;} let v=kmap_local_page(pfn_to_page(pfn)); memcpy(res,(v as *mut u8).add(addr&(PAGE_SIZE-1)) as _,size); kunmap_local(v); 0 }
pub unsafe fn kdb_task_state_char(p:*const task_struct)->c_char { if p.is_null(){return b'E' as c_char;} task_state_to_char(p as *mut _) }
pub unsafe fn kdb_task_state(p:*const task_struct,mask:*const c_char)->bool { let s=kdb_task_state_char(p); if mask.is_null()||*mask==0{return strchr(b"-ims\0".as_ptr() as _,s as _) .is_null();} if !strchr(mask,b'A' as _).is_null(){return true;} !strchr(mask,s as _).is_null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
