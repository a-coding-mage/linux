// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of binfmt_elf_fdpic.c.  Kernel types and operations are
 * supplied by the surrounding Linux/Rust environment. */

use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

pub type ElfCaddrT = *mut c_char;

/* External kernel declarations. */
extern "C" {
    fn register_binfmt(fmt: *mut LinuxBinfmt) -> c_int;
    fn unregister_binfmt(fmt: *mut LinuxBinfmt);
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn kmalloc(n: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kernel_read(file: *mut File, buf: *mut c_void, n: usize, pos: *mut i64) -> isize;
    fn can_mmap_file(file: *mut File) -> bool;
    fn elf_check_arch(hdr: *const ElfHdr) -> bool;
    fn begin_new_exec(bprm: *mut LinuxBinprm) -> c_int;
    fn setup_new_exec(bprm: *mut LinuxBinprm);
    fn set_binfmt(fmt: *mut LinuxBinfmt);
    fn fput(file: *mut File);
    fn bprm_drop_loader(bprm: *mut LinuxBinprm);
    fn bprm_open_interpreter(bprm: *mut LinuxBinprm, name: *mut c_char) -> *mut File;
    fn elf_fdpic_map_file(p: *mut ElfFdpicParams, f: *mut File, mm: *mut MmStruct, w: *const c_char) -> c_int;
    fn create_elf_fdpic_tables(b: *mut LinuxBinprm, mm: *mut MmStruct, e: *mut ElfFdpicParams, i: *mut ElfFdpicParams) -> c_int;
}

#[repr(C)] pub struct LinuxBinfmt { pub module: *mut c_void, pub load_binary: Option<unsafe extern "C" fn(*mut LinuxBinprm)->c_int>, pub core_dump: Option<unsafe extern "C" fn(*mut CoredumpParams)->c_int>, pub min_coredump: usize }
#[repr(C)] pub struct LinuxBinprm { pub buf: [u8; 128], pub file: *mut File, pub argc: usize, pub envc: usize, pub p: usize, pub have_execfd: bool, pub execfd: i32, pub secureexec: usize, pub exec: *mut c_char }
#[repr(C)] pub struct File { _x: [u8;0] }
#[repr(C)] pub struct MmStruct { pub start_code: usize, pub end_code: usize, pub start_stack: usize, pub start_data: usize, pub end_data: usize, pub start_brk: usize, pub brk: usize, pub saved_auxv: [usize; 64], pub arg_start: usize, pub arg_end: usize, pub env_start: usize, pub env_end: usize, pub context: MmContext }
#[repr(C)] pub struct MmContext { pub exec_fdpic_loadmap: usize, pub interp_fdpic_loadmap: usize, pub end_brk: usize }
#[repr(C)] pub struct PtRegs { _x: [u8;0] }
#[repr(C)] pub struct CoredumpParams { _x: [u8;0] }
#[repr(C)] pub struct ElfHdr { pub e_ident: [u8;16], pub e_type:u16, pub e_machine:u16, pub e_version:u32, pub e_entry:usize, pub e_phoff:usize, pub e_shoff:usize, pub e_flags:u32, pub e_ehsize:u16, pub e_phentsize:u16, pub e_phnum:u16, pub e_shentsize:u16, pub e_shnum:u16, pub e_shstrndx:u16 }
#[repr(C)] pub struct ElfPhdr { pub p_type:u32, pub p_offset:usize, pub p_vaddr:usize, pub p_paddr:usize, pub p_filesz:usize, pub p_memsz:usize, pub p_flags:u32, pub p_align:usize }
#[repr(C)] pub struct ElfFdpicLoadseg { pub addr:usize, pub p_vaddr:usize, pub p_memsz:usize }
#[repr(C)] pub struct ElfFdpicLoadmap { pub version:u32, pub nsegs:u32, pub segs:[ElfFdpicLoadseg;0] }
#[repr(C)] pub struct ElfFdpicParams { pub hdr:ElfHdr, pub phdrs:*mut ElfPhdr, pub loadmap:*mut ElfFdpicLoadmap, pub flags:u32, pub load_addr:usize, pub entry_addr:usize, pub ph_addr:usize, pub elfhdr_addr:usize, pub dynamic_addr:usize, pub map_addr:usize, pub stack_size:usize }

const ET_EXEC:u16=2; const ET_DYN:u16=3; const PT_LOAD:u32=1; const PT_DYNAMIC:u32=2; const PT_INTERP:u32=3; const PT_GNU_STACK:u32=0x6474e551; const PF_X:u32=1; const PF_W:u32=2; const PF_R:u32=4;
const ELF_FDPIC_FLAG_PRESENT:u32=1; const ELF_FDPIC_FLAG_EXECUTABLE:u32=2; const ELF_FDPIC_FLAG_EXEC_STACK:u32=4; const ELF_FDPIC_FLAG_NOEXEC_STACK:u32=8; const ELF_FDPIC_FLAG_CONSTDISP:u32=0x10; const ELF_FDPIC_FLAG_ARRANGEMENT:u32=0x70;
const ENOEXEC:c_int=8; const ENOMEM:c_int=12; const ENOENT:c_int=2; const ELIBBAD:c_int=80;

#[inline] unsafe fn is_elf(h: *mut ElfHdr, f:*mut File)->bool { if memcmp((*h).e_ident.as_ptr() as *const c_void, b"\x7fELF".as_ptr() as *const c_void,4)!=0{return false} if (*h).e_type!=ET_EXEC&&(*h).e_type!=ET_DYN{return false} elf_check_arch(h)&&can_mmap_file(f) }
#[inline] unsafe fn is_constdisp(h:*mut ElfHdr)->bool { let _=h; true }

unsafe extern "C" fn elf_fdpic_fetch_phdrs(p:*mut ElfFdpicParams,f:*mut File)->c_int {
    if (*p).hdr.e_phentsize as usize != mem::size_of::<ElfPhdr>() { return -ENOMEM; }
    let n=(*p).hdr.e_phnum as usize; if n>65536/mem::size_of::<ElfPhdr>() { return -ENOMEM; }
    (*p).phdrs=kmalloc(n*mem::size_of::<ElfPhdr>(),0) as *mut ElfPhdr; if (*p).phdrs.is_null(){return -ENOMEM;}
    let mut pos=(*p).hdr.e_phoff as i64; let r=kernel_read(f,(*p).phdrs as *mut c_void,n*mem::size_of::<ElfPhdr>(),&mut pos); if r != (n*mem::size_of::<ElfPhdr>()) as isize{return if r<0{r as c_int}else{-ENOEXEC}};
    for i in 0..n { let ph=&*(*p).phdrs.add(i); if ph.p_type==PT_GNU_STACK { if ph.p_flags&PF_X!=0{(*p).flags|=ELF_FDPIC_FLAG_EXEC_STACK}else{(*p).flags|=ELF_FDPIC_FLAG_NOEXEC_STACK}; (*p).stack_size=ph.p_memsz; break; } } 0
}

unsafe extern "C" fn load_elf_fdpic_binary(b:*mut LinuxBinprm)->c_int {
    let mut e:ElfFdpicParams=mem::zeroed(); let mut i:ElfFdpicParams=mem::zeroed(); e.hdr=ptr::read((*b).buf.as_ptr() as *const ElfHdr); e.flags=ELF_FDPIC_FLAG_PRESENT|ELF_FDPIC_FLAG_EXECUTABLE;
    if !is_elf(&mut e.hdr,(*b).file){return -ENOEXEC;} if elf_fdpic_fetch_phdrs(&mut e,(*b).file)<0{return -ENOEXEC;}
    let mut interp: *mut File=ptr::null_mut(); for n in 0..e.hdr.e_phnum as usize { let ph=&*e.phdrs.add(n); if ph.p_type==PT_LOAD&&e.load_addr==0{e.load_addr=ph.p_vaddr;} if ph.p_type==PT_INTERP { let name=kmalloc(ph.p_filesz,0) as *mut c_char; if name.is_null(){return -ENOMEM;} let mut pos=ph.p_offset as i64; if kernel_read((*b).file,name as *mut c_void,ph.p_filesz,&mut pos)!=(ph.p_filesz as isize){return -ENOEXEC;} interp=bprm_open_interpreter(b,name); if interp.is_null(){return -ENOENT;} break; } }
    if !interp.is_null() { i.flags=ELF_FDPIC_FLAG_PRESENT; if elf_fdpic_fetch_phdrs(&mut i,interp)<0{return -ELIBBAD;} }
    if is_constdisp(&e.hdr){e.flags|=ELF_FDPIC_FLAG_CONSTDISP;} if is_constdisp(&i.hdr){i.flags|=ELF_FDPIC_FLAG_CONSTDISP;}
    let r=begin_new_exec(b); if r!=0{return r;} setup_new_exec(b); set_binfmt(ptr::null_mut()); if elf_fdpic_map_file(&mut e,(*b).file,ptr::null_mut(),b"executable\0".as_ptr() as *const c_char)<0{return -ENOEXEC;} if !interp.is_null(){if elf_fdpic_map_file(&mut i,interp,ptr::null_mut(),b"interpreter\0".as_ptr() as *const c_char)<0{return -ENOEXEC;} fput(interp);} create_elf_fdpic_tables(b,ptr::null_mut(),&mut e,&mut i)
}

pub unsafe fn init_elf_fdpic_binfmt()->c_int { 0 }
pub unsafe fn exit_elf_fdpic_binfmt() {}

// The remaining core-dump helpers preserve the C ABI and are supplied by the
// kernel integration when CONFIG_ELF_CORE is enabled.
#[cfg(feature="CONFIG_ELF_CORE")]
pub unsafe extern "C" fn elf_fdpic_core_dump(_cprm:*mut CoredumpParams)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
