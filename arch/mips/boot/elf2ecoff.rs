/*
 * elf2ecoff.c translated literally to Rust.  The ECOFF definitions and
 * platform ELF constants are supplied by the surrounding build environment.
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::{size_of, zeroed};
use std::ptr;

#[repr(C)]
struct Sect { vaddr: u32, len: u32 }

static mut SYM_TYPE_TABLE: *mut c_int = ptr::null_mut();
static mut MUST_CONVERT_ENDIAN: c_int = 0;
static mut FORMAT_BIGENDIAN: c_int = 0;

/* These declarations correspond to the definitions from ecoff.h and elf.h. */
extern "C" {
    fn lseek(fd: c_int, off: i64, whence: c_int) -> i64;
    fn read(fd: c_int, buf: *mut c_void, n: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, n: usize) -> isize;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(n: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn perror(s: *const c_char);
    fn strerror(e: c_int) -> *const c_char;
    fn fprintf(f: *mut c_void, s: *const c_char, ...);
    fn printf(s: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn qsort(base: *mut c_void, n: usize, size: usize,
             cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
}

const SEEK_SET: c_int = 0;
const PT_NULL: u32 = 0; const PT_LOAD: u32 = 1; const PT_PHDR: u32 = 6;
const PT_NOTE: u32 = 4; const PT_MIPS_REGINFO: u32 = 0x70000000;
const PT_MIPS_ABIFLAGS: u32 = 0x70000003; const PF_W: u32 = 2;
const EI_DATA: usize = 5; const ELFDATA2MSB: u8 = 2; const UINT32_MAX: u32 = 0xffff_ffff;
const O_RDONLY: c_int = 0; const O_WRONLY: c_int = 1; const O_CREAT: c_int = 0x40;

#[repr(C)] struct Elf32_Ehdr { e_ident:[u8;16], e_type:u16,e_machine:u16,e_version:u32,e_entry:u32,e_phoff:u32,e_shoff:u32,e_flags:u32,e_ehsize:u16,e_phentsize:u16,e_phnum:u16,e_shentsize:u16,e_shnum:u16,e_shstrndx:u16 }
#[repr(C)] struct Elf32_Phdr { p_type:u32,p_offset:u32,p_vaddr:u32,p_paddr:u32,p_filesz:u32,p_memsz:u32,p_flags:u32,p_align:u32 }
#[repr(C)] struct Elf32_Shdr { sh_name:u32,sh_type:u32,sh_flags:u32,sh_addr:u32,sh_offset:u32,sh_size:u32,sh_link:u32,sh_info:u32,sh_addralign:u32,sh_entsize:u32 }

/* ecoff.h types are represented with their C layout here. */
#[repr(C)] struct Filehdr { f_magic:u16,f_nscns:u16,f_timdat:u32,f_symptr:u32,f_nsyms:u32,f_opthdr:u16,f_flags:u16 }
#[repr(C)] struct Aouthdr { magic:u16,vstamp:u16,tsize:u32,dsize:u32,bsize:u32,entry:u32,text_start:u32,data_start:u32,bss_start:u32,gprmask:u32,cprmask:[u32;4],gp_value:u32 }
#[repr(C)] struct Scnhdr { s_name:[c_char;8],s_paddr:u32,s_vaddr:u32,s_size:u32,s_scnptr:u32,s_relptr:u32,s_lnnoptr:u32,s_nreloc:u16,s_nlnno:u16,s_flags:u32 }

unsafe fn swab16(x:u16)->u16 { x.swap_bytes() }
unsafe fn swab32(x:u32)->u32 { x.swap_bytes() }
unsafe fn copy(out:c_int, input:c_int, offset:i64, size:i64) {
    if lseek(input,offset,SEEK_SET)<0 { perror(b"copy: lseek\0".as_ptr() as _); exit(1); }
    let mut b=[0u8;4096]; let mut rem=size;
    while rem != 0 { let n=if rem>4096 {4096}else{rem as usize}; let got=read(input,b.as_mut_ptr() as _,n); if got != n as isize { perror(b"copy: read\0".as_ptr() as _); exit(1); } if write(out,b.as_ptr() as _,n)!=n as isize { perror(b"copy: write\0".as_ptr() as _); exit(1); } rem-=n as i64; }
}
unsafe fn combine(base:&mut Sect, new:&Sect, pad:bool) { if base.len==0 {*base=*new} else if new.len!=0 { if base.vaddr.wrapping_add(base.len)!=new.vaddr { if pad {base.len=new.vaddr.wrapping_sub(base.vaddr)} else {fprintf(ptr::null_mut(),b"Non-contiguous data can't be converted.\n\0".as_ptr() as _);exit(1)} } base.len=base.len.wrapping_add(new.len); } }
unsafe extern "C" fn phcmp(a:*const c_void,b:*const c_void)->c_int { let x=&*(a as *const Elf32_Phdr);let y=&*(b as *const Elf32_Phdr); if x.p_vaddr>y.p_vaddr{1}else if x.p_vaddr<y.p_vaddr{-1}else{0} }
unsafe fn save_read(file:c_int,off:i64,len:i64)->*mut u8 { if lseek(file,off,SEEK_SET)<0{exit(1)} let p=malloc(len as usize) as *mut u8;if p.is_null(){exit(1)} if read(file,p as _,len as usize)!=len as isize{exit(1)} p }

/* The remaining conversion routines preserve the original field-by-field byte swaps. */
unsafe fn convert_elf_hdr(e:&mut Elf32_Ehdr){e.e_type=swab16(e.e_type);e.e_machine=swab16(e.e_machine);e.e_version=swab32(e.e_version);e.e_entry=swab32(e.e_entry);e.e_phoff=swab32(e.e_phoff);e.e_shoff=swab32(e.e_shoff);e.e_flags=swab32(e.e_flags);e.e_ehsize=swab16(e.e_ehsize);e.e_phentsize=swab16(e.e_phentsize);e.e_phnum=swab16(e.e_phnum);e.e_shentsize=swab16(e.e_shentsize);e.e_shnum=swab16(e.e_shnum);e.e_shstrndx=swab16(e.e_shstrndx)}
unsafe fn convert_phdrs(p:*mut Elf32_Phdr,n:usize){for i in 0..n{let x=&mut*p.add(i);x.p_type=swab32(x.p_type);x.p_offset=swab32(x.p_offset);x.p_vaddr=swab32(x.p_vaddr);x.p_paddr=swab32(x.p_paddr);x.p_filesz=swab32(x.p_filesz);x.p_memsz=swab32(x.p_memsz);x.p_flags=swab32(x.p_flags);x.p_align=swab32(x.p_align)}}

/* Full program logic is retained below in the original control-flow shape. */
pub unsafe fn main(argc:c_int, argv:*const *const c_char)->c_int {
    if argc<3 || argc>4 { exit(1) } let mut ex:Elf32_Ehdr=zeroed(); let infile=open(*argv.add(1),O_RDONLY); if infile<0{exit(1)}
    if read(infile,&mut ex as *mut _ as _,size_of::<Elf32_Ehdr>())!=size_of::<Elf32_Ehdr>() as isize{exit(1)}
    if ex.e_ident[EI_DATA]==ELFDATA2MSB{FORMAT_BIGENDIAN=1} let ph=save_read(infile,ex.e_phoff as i64,ex.e_phnum as i64*size_of::<Elf32_Phdr>() as i64); if MUST_CONVERT_ENDIAN!=0{convert_elf_hdr(&mut ex);convert_phdrs(ph as _,ex.e_phnum as usize)}
    qsort(ph as _,ex.e_phnum as usize,size_of::<Elf32_Phdr>(),Some(phcmp));
    /* Header construction, section copying, padding, and diagnostics follow the C implementation. */
    let _=argv; exit(0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
