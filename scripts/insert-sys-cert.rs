/* Write the contents of the <certfile> into kernel symbol system_extra_cert */

use std::ffi::{c_char, c_int, c_ulong, c_void, CStr};
use std::ptr;

const CERT_SYM: &[u8] = b"system_extra_cert\0";
const USED_SYM: &[u8] = b"system_extra_cert_used\0";
const LSIZE_SYM: &[u8] = b"system_certificate_list_size\0";
const LINE_SIZE: usize = 100;
const ELFDATA2LSB: u8 = 1;
const ELFDATA2MSB: u8 = 2;
const ELFCLASS64: u8 = 2;
const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = b'E';
const ELFMAG2: u8 = b'L';
const ELFMAG3: u8 = b'F';

#[repr(C)]
struct ElfEhdr {
    e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32,
    e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32, e_ehsize: u16,
    e_phentsize: u16, e_phnum: u16, e_shentsize: u16, e_shnum: u16, e_shstrndx: u16,
}
#[repr(C)]
struct ElfShdr { sh_name: u32, sh_type: u32, sh_flags: u64, sh_addr: u64, sh_offset: u64, sh_size: u64, sh_link: u32, sh_info: u32, sh_addralign: u64, sh_entsize: u64 }
#[repr(C)]
struct ElfSym { st_name: u32, st_info: u8, st_other: u8, st_shndx: u16, st_value: u64, st_size: u64 }
#[repr(C)]
struct Stat { _data: [u8; 144] }

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ... ) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fseek(stream: *mut c_void, offset: i64, whence: c_int) -> c_int;
    fn fgets(s: *mut c_char, n: c_int, stream: *mut c_void) -> *mut c_char;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, stat: *mut Stat) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: i64) -> *mut c_void;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_ulong;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
}

#[repr(C)] struct Sym { name: *mut c_char, address: c_ulong, offset: c_ulong, content: *mut c_void, size: c_int }

unsafe fn endianness() -> u8 { let x: u16 = 0x00ff; if *(ptr::addr_of!(x) as *const u8) == 0 { ELFDATA2MSB } else { ELFDATA2LSB } }
unsafe fn shdr(h: *mut ElfEhdr) -> *mut ElfShdr { (h as *mut u8).add((*h).e_shoff as usize) as *mut ElfShdr }
unsafe fn get_offset_from_address(h: *mut ElfEhdr, addr: c_ulong) -> c_ulong {
    let x = shdr(h); let n = if (*h).e_shnum == 0 { (*x).sh_size as u32 } else { (*h).e_shnum as u32 };
    for i in 1..n { let s = &*x.add(i as usize); let end = s.sh_addr + s.sh_size; if addr >= s.sh_addr && addr <= end { return addr - s.sh_addr + s.sh_offset; } } 0
}
unsafe fn get_symbol_from_table(h: *mut ElfEhdr, tab: *mut ElfShdr, name: *mut c_char, s: *mut Sym) {
    (*s).size=0; (*s).address=0; (*s).offset=0; let x=shdr(h); let start=(h as *mut u8).add((*tab).sh_offset as usize) as *mut ElfSym; let st=(h as *mut u8).add((*x.add((*tab).sh_link as usize)).sh_offset as usize) as *mut c_char; let n=(*tab).sh_size/(*tab).sh_entsize;
    for i in 0..n { let e=&*start.add(i as usize); let p=st.add(e.st_name as usize); if CStr::from_ptr(p).to_bytes()==CStr::from_ptr(name).to_bytes() { let sec=&*x.add(e.st_shndx as usize); (*s).size=e.st_size as c_int; (*s).address=e.st_value as c_ulong; (*s).offset=(*s).address- sec.sh_addr as c_ulong + sec.sh_offset as c_ulong; (*s).name=name; (*s).content=(h as *mut u8).add((*s).offset as usize) as *mut c_void; return; } }
}
unsafe fn get_symbol_table(h:*mut ElfEhdr)->*mut ElfShdr { let x=shdr(h); let n=if (*h).e_shnum==0 {(*x).sh_size as u32}else{(*h).e_shnum as u32}; for i in 1..n {if (*x.add(i as usize)).sh_type==2{return x.add(i as usize)}} ptr::null_mut() }

unsafe fn map_file(name:*mut c_char,size:*mut c_int)->*mut ElfEhdr { let fd=open(name,2); if fd<0 {perror(name);return ptr::null_mut()} let mut st=Stat{_data:[0;144]}; if fstat(fd,&mut st)!=0 {perror(b"Could not determine file size\0".as_ptr() as *const c_char);close(fd);return ptr::null_mut()} let n=*(st._data.as_ptr().add(48) as *const i64); *size=n as c_int; let p=mmap(ptr::null_mut(),n as usize,3,1,fd,0); close(fd); p as *mut ElfEhdr }
unsafe fn read_file(name:*mut c_char,size:*mut c_int)->*mut c_char { let fd=open(name,0); if fd<0 {perror(name);return ptr::null_mut()} let mut st=Stat{_data:[0;144]}; if fstat(fd,&mut st)!=0 {perror(b"Could not determine file size\0".as_ptr() as *const c_char);close(fd);return ptr::null_mut()} let n=*(st._data.as_ptr().add(48) as *const i64); let p=libc_malloc(n as usize) as *mut c_char; if p.is_null()||read(fd,p as *mut c_void,n as usize)!=n {perror(b"File read failed\0".as_ptr() as *const c_char);close(fd);return ptr::null_mut()} close(fd);*size=n as c_int;p }
extern "C" { fn libc_malloc(n: usize)->*mut c_void; fn memcmp(a:*const c_void,b:*const c_void,n:usize)->c_int; fn memcpy(a:*mut c_void,b:*const c_void,n:usize)->*mut c_void; fn memset(a:*mut c_void,v:c_int,n:usize)->*mut c_void; }

unsafe fn main_c(argc:c_int,argv:*mut *mut c_char)->c_int { let mut b=ptr::null_mut();let mut c=ptr::null_mut();let mut s=ptr::null_mut();let mut o;while {o=getopt(argc,argv,b"b:c:s:\0".as_ptr() as _);o!=-1}{match o{98=>b=optarg,99=>c=optarg,115=>s=optarg,_=>{}}}if b.is_null()||c.is_null(){return 1}let mut cs=0;let cert=read_file(c,&mut cs);let mut vs=0;let hdr=map_file(b,&mut vs);if hdr.is_null(){return 1}if (*hdr).e_ident[0]!=ELFMAG0||(*hdr).e_ident[1]!=ELFMAG1||(*hdr).e_ident[2]!=ELFMAG2||(*hdr).e_ident[3]!=ELFMAG3||(*hdr).e_ident[4]!=ELFCLASS64||(*hdr).e_ident[5]!=endianness(){return 1}let tab=get_symbol_table(hdr);let mut a=Sym{name:ptr::null_mut(),address:0,offset:0,content:ptr::null_mut(),size:0};let mut u=Sym{name:ptr::null_mut(),address:0,offset:0,content:ptr::null_mut(),size:0};let mut l=Sym{name:ptr::null_mut(),address:0,offset:0,content:ptr::null_mut(),size:0};if !tab.is_null(){get_symbol_from_table(hdr,tab,CERT_SYM.as_ptr() as _,&mut a);get_symbol_from_table(hdr,tab,USED_SYM.as_ptr() as _,&mut u);get_symbol_from_table(hdr,tab,LSIZE_SYM.as_ptr() as _,&mut l)}else{return 1}if a.offset==0||u.offset==0||l.offset==0{return 1}if a.size<cs{return 1}let used= u.content as *mut c_int;if cs==*used&&memcmp(a.content,cert as _,cs as usize)==0{return 0}memcpy(a.content,cert as _,cs as usize);if cs<a.size{memset((a.content as *mut u8).add(cs as usize) as _,0,(a.size-cs) as usize)}*(l.content as *mut c_ulong)=*(l.content as *mut c_ulong)+cs as c_ulong-*used as c_ulong;*used=cs;let _=s;0 }
fn main(){unsafe{let args:Vec<*mut c_char>=std::env::args().map(|x|std::ffi::CString::new(x).unwrap().into_raw()).collect();std::process::exit(main_c(args.len() as c_int,args.as_ptr() as *mut _));}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
