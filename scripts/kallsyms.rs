/* Generate assembler source containing symbol information. */

use std::ffi::c_void;
use std::mem;
use std::ptr;

const KSYM_NAME_LEN: usize = 512;

#[repr(C)]
struct SymEntry {
    addr: u64,
    len: u32,
    seq: u32,
    sym: [u8; 0],
}

#[repr(C)]
struct AddrRange {
    start_sym: *const u8,
    end_sym: *const u8,
    start: u64,
    end: u64,
}

extern "C" {
    fn fprintf(stream: *mut c_void, fmt: *const u8, ...) -> i32;
    fn printf(fmt: *const u8, ... ) -> i32;
    fn exit(status: i32) -> !;
    fn free(p: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
    fn fopen(path: *const u8, mode: *const u8) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> i32;
    fn feof(stream: *mut c_void) -> i32;
    fn getline(line: *mut *mut u8, cap: *mut usize, stream: *mut c_void) -> isize;
    fn perror(s: *const u8);
    fn strtoull(s: *const u8, end: *mut *mut u8, base: i32) -> u64;
    fn strlen(s: *const u8) -> usize;
    fn strcmp(a: *const u8, b: *const u8) -> i32;
    fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn qsort(base: *mut c_void, n: usize, size: usize, cmp: unsafe extern "C" fn(*const c_void, *const c_void) -> i32);
    fn getopt_long(argc: i32, argv: *mut *mut u8, shortopts: *const u8, longopts: *const c_void, index: *mut i32) -> i32;
    static mut optind: i32;
}

static mut TEXT: u64 = 0;
static mut TEXT_RANGES: [AddrRange; 2] = [
    AddrRange { start_sym: b"_stext\0".as_ptr(), end_sym: b"_etext\0".as_ptr(), start: 0, end: 0 },
    AddrRange { start_sym: b"_sinittext\0".as_ptr(), end_sym: b"_einittext\0".as_ptr(), start: 0, end: 0 },
];
static mut TABLE: *mut *mut SymEntry = ptr::null_mut();
static mut TABLE_SIZE: u32 = 0;
static mut TABLE_CNT: u32 = 0;
static mut ALL_SYMBOLS: i32 = 0;
static mut PC_RELATIVE: i32 = 0;
static mut TOKEN_PROFIT: [i32; 0x10000] = [0; 0x10000];
static mut BEST_TABLE: [[u8; 2]; 256] = [[0; 2]; 256];
static mut BEST_TABLE_LEN: [u8; 256] = [0; 256];

unsafe fn xmalloc(n: usize) -> *mut c_void { let p = malloc(n); if p.is_null() { exit(1) }; p }
unsafe fn xrealloc(p: *mut c_void, n: usize) -> *mut c_void { let q = realloc(p, n); if q.is_null() { exit(1) }; q }
unsafe fn sym_name(s: *const SymEntry) -> *mut u8 { (s as *mut u8).add(mem::size_of::<SymEntry>() + 1) }

unsafe fn usage() -> ! { fprintf(ptr::null_mut(), b"Usage: kallsyms [--all-symbols] in.map > out.S\n\0".as_ptr()); exit(1) }
unsafe fn is_ignored_symbol(name: *const u8, typ: u8) -> bool {
    if typ == b'u' || typ == b'n' { return true; }
    if (typ as char).to_ascii_uppercase() as u8 == b'A' {
        return strcmp(name,b"__kernel_syscall_via_break\0".as_ptr()) != 0 && strcmp(name,b"__kernel_syscall_via_epc\0".as_ptr()) != 0 && strcmp(name,b"__kernel_sigtramp\0".as_ptr()) != 0 && strcmp(name,b"__gp\0".as_ptr()) != 0;
    }
    false
}
unsafe fn check_symbol_range(sym:*const u8, addr:u64, ranges:*mut AddrRange, entries:usize) { for i in 0..entries { let r=ranges.add(i); if strcmp(sym,(*r).start_sym)==0 {(*r).start=addr;return} else if strcmp(sym,(*r).end_sym)==0 {(*r).end=addr;return} } }
unsafe fn read_symbol(_in:*mut c_void, buf:*mut *mut u8, blen:*mut usize) -> *mut SymEntry {
    let n=getline(buf,blen,_in); if n<0 { return ptr::null_mut() }; if *buf.add(0).add(0).add(n as usize-1)==b'\n' {*(*buf).add(n as usize-1)=0;}
    let mut p=ptr::null_mut(); let addr=strtoull(*buf,&mut p,16); if p.is_null() || *p!=b' ' {exit(1)}; p=p.add(1); let typ=*p; p=p.add(1); if *p!=b' ' {exit(1)}; p=p.add(1);
    let len=strlen(p); if len>=KSYM_NAME_LEN{return ptr::null_mut()}; if strcmp(p,b"_text\0".as_ptr())==0 {TEXT=addr}; if is_ignored_symbol(p,typ){return ptr::null_mut()}; check_symbol_range(p,addr,TEXT_RANGES.as_mut_ptr(),2);
    let s=xmalloc(mem::size_of::<SymEntry>()+len+2) as *mut SymEntry; (*s).addr=addr;(*s).len=(len+1) as u32;(*s).sym.as_mut_ptr(); *((s as *mut u8).add(mem::size_of::<SymEntry>()))=typ; ptr::copy_nonoverlapping(p,sym_name(s),len+1); s
}

unsafe fn symbol_in_range(s:*const SymEntry)->bool { for r in &TEXT_RANGES {if (*s).addr>=r.start&&(*s).addr<=r.end{return true}} false }
unsafe fn symbol_valid(s:*const SymEntry)->bool { let n=sym_name(s); if ALL_SYMBOLS==0 {if strncmp(n,b"__start_\0".as_ptr(),8)==0||strncmp(n,b"__stop_\0".as_ptr(),7)==0{return true}; if !symbol_in_range(s){return false}; if ((*s).addr==TEXT_RANGES[0].end&&strcmp(n,TEXT_RANGES[0].end_sym)!=0)||((*s).addr==TEXT_RANGES[1].end&&strcmp(n,TEXT_RANGES[1].end_sym)!=0){return false}} true }
unsafe fn shrink_table(){let mut pos=0;for i in 0..TABLE_CNT{let s=*TABLE.add(i as usize);if symbol_valid(s){*TABLE.add(pos)=s;pos+=1}else{free(s as *mut c_void)}}TABLE_CNT=pos;}
unsafe fn read_map(path:*const u8){let f=fopen(path,b"r\0".as_ptr());if f.is_null(){exit(1)};let mut b=ptr::null_mut();let mut n=0;while feof(f)==0{let s=read_symbol(f,&mut b,&mut n);if s.is_null(){continue}(*s).seq=TABLE_CNT;if TABLE_CNT>=TABLE_SIZE{TABLE_SIZE+=10000;TABLE=xrealloc(TABLE as *mut c_void,TABLE_SIZE as usize*mem::size_of::<*mut SymEntry>()) as *mut *mut SymEntry}*TABLE.add(TABLE_CNT as usize)=s;TABLE_CNT+=1}free(b as *mut c_void);fclose(f);}

unsafe fn learn(s:*const u8,n:i32,d:i32){for i in 0..n-1{TOKEN_PROFIT[*s.add(i as usize) as usize+((*s.add(i as usize+1) as usize)<<8)]+=d;}}
unsafe fn find_token(s:*mut u8,n:i32,t:*const u8)->*mut u8{for i in 0..n-1{if *s.add(i as usize)==*t&&*s.add(i as usize+1)==*t.add(1){return s.add(i as usize)}}ptr::null_mut()}
unsafe fn compress(str_:*const u8,idx:u8){for i in 0..TABLE_CNT{let s=*TABLE.add(i as usize);let mut len=(*s).len as i32;let mut p1=(s as *mut u8).add(mem::size_of::<SymEntry>());let mut p2=find_token(p1,len,str_);if p2.is_null(){continue}learn(p1,len,-1);let mut size=len;loop{*p2=idx;p2=p2.add(1);size-=p2.offset_from(p1) as i32;memmove(p2 as *mut c_void,p2.add(1),size as usize);p1=p2;len-=1;if size<2{break}p2=find_token(p1,size,str_);if p2.is_null(){break}}(*s).len=len as u32;learn(p1.offset(-(len as isize) as isize),len,1);}}
unsafe fn optimize(){for i in (0..256).rev(){if BEST_TABLE_LEN[i]==0{let mut best=0;for j in 0..65536{if TOKEN_PROFIT[j]>TOKEN_PROFIT[best]{best=j}}if TOKEN_PROFIT[best]==0{break}BEST_TABLE_LEN[i]=2;BEST_TABLE[i]=[best as u8,(best>>8) as u8];compress(BEST_TABLE[i].as_ptr(),i as u8)}}}
unsafe fn optimize_table(){for i in 0..TABLE_CNT{let s=*TABLE.add(i as usize);let p=(s as *mut u8).add(mem::size_of::<SymEntry>());learn(p,(*s).len as i32,1);for j in 0..(*s).len{let c=*p.add(j as usize);BEST_TABLE[c as usize]=[c,0];BEST_TABLE_LEN[c as usize]=1;}}optimize();}

unsafe fn expand(data:*const u8,len:i32,result:*mut u8)->i32{let mut total=0;for i in 0..len{let c=*data.add(i as usize) as usize;if BEST_TABLE[c][0]==c as u8&&BEST_TABLE_LEN[c]==1{*result.add(total as usize)=c as u8;total+=1}else{total+=expand(BEST_TABLE[c].as_ptr(),BEST_TABLE_LEN[c] as i32,result.add(total as usize));}}*result.add(total as usize)=0;total}
unsafe fn output_label(label:*const u8){printf(b".globl %s\n\t.balign 4\n%s:\n\0".as_ptr(),label,label);}
unsafe fn write_src(){let mut b=[0u8;KSYM_NAME_LEN];printf(b"\t.section .rodata, \"a\"\n\0".as_ptr());output_label(b"kallsyms_num_syms\0".as_ptr());printf(b"\t.long\t%u\n\n\0".as_ptr(),TABLE_CNT);output_label(b"kallsyms_names\0".as_ptr());for i in 0..TABLE_CNT{let s=*TABLE.add(i as usize);printf(b"\t.byte %u\n\0".as_ptr(),(*s).len);for j in 0..(*s).len{printf(b", 0x%02x\0".as_ptr(),*((s as *mut u8).add(mem::size_of::<SymEntry>()+j as usize) as *mut u8));}expand((s as *mut u8).add(mem::size_of::<SymEntry>()),(*s).len as i32,b.as_mut_ptr());printf(b"\t/* %s */\n\0".as_ptr(),b.as_ptr());}printf(b".size kallsyms_names, . - kallsyms_names\n\0".as_ptr());}

#[no_mangle] pub unsafe extern "C" fn main(argc:i32,argv:*mut *mut u8)->i32{if argc<2{usage()}read_map(*argv.add(1));shrink_table();optimize_table();write_src();0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
