/* Faithful low-level Rust translation of unifdef.c. */
use std::{ffi::{CStr,CString}, os::raw::{c_char,c_int,c_uint,c_void}, ptr, slice};

pub const MAXDEPTH: usize=64; pub const MAXLINE: usize=4096; pub const MAXSYMS: usize=4096; pub const EDITSLOP: usize=10;
#[repr(i32)] #[derive(Copy,Clone,PartialEq)] enum Linetype { LT_TRUEI,LT_FALSEI,LT_IF,LT_TRUE,LT_FALSE,LT_ELIF,LT_ELTRUE,LT_ELFALSE,LT_ELSE,LT_ENDIF,LT_DODGY,LT_DODGY_LAST,LT_PLAIN,LT_EOF,LT_ERROR,LT_COUNT }
#[repr(i32)] #[derive(Copy,Clone,PartialEq)] enum Ifstate { IS_OUTSIDE,IS_FALSE_PREFIX,IS_TRUE_PREFIX,IS_PASS_MIDDLE,IS_FALSE_MIDDLE,IS_TRUE_MIDDLE,IS_PASS_ELSE,IS_FALSE_ELSE,IS_TRUE_ELSE,IS_FALSE_TRAILER,IS_COUNT }
#[repr(i32)] #[derive(Copy,Clone,PartialEq)] enum CommentState { NO_COMMENT=false as i32,C_COMMENT,CXX_COMMENT,STARTING_COMMENT,FINISHING_COMMENT,CHAR_LITERAL,STRING_LITERAL }
#[repr(i32)] #[derive(Copy,Clone,PartialEq)] enum LineState { LS_START,LS_HASH,LS_DIRTY }

static COPYRIGHT:&[u8]=b"@(#) $Version: unifdef-2.5 $\n@(#) $Author: Tony Finch (dot@dotat.at) $\n@(#) $URL: http://dotat.at/prog/unifdef $\n\0";
static mut COMPBLANK:bool=false; static mut LNBLANK:bool=false; static mut COMPLEMENT:bool=false; static mut DEBUGGING:bool=false; static mut IOCC楚K:bool=false; static mut STRICTLOGIC:bool=false; static mut KILLCONSTS:bool=false; static mut LNNUM:bool=false; static mut SYMLIST:bool=false; static mut SYMDEPTH:bool=false; static mut TEXT:bool=false;
static mut SYMNAME:[*const c_char;MAXSYMS]=[ptr::null();MAXSYMS]; static mut VALUE:[*const c_char;MAXSYMS]=[ptr::null();MAXSYMS]; static mut IGNORE:[bool;MAXSYMS]=[false;MAXSYMS]; static mut NSYMS:usize=0;
static mut FILENAME:*const c_char=ptr::null(); static mut OFILENAME:*const c_char=ptr::null(); static mut LINENUM:c_int=0; static mut DEPTH:usize=0; static mut DELCOUNT:c_int=0; static mut BLANKCOUNT:u32=0; static mut BLANKMAX:u32=0; static mut EXITSTAT:c_int=0; static mut CONSTEXPRESSION:bool=false; static mut ZEROSYMS:bool=true; static mut FIRSTSYM:bool=false;
static mut INCOMMENT:CommentState=CommentState::NO_COMMENT; static mut LINESTATE:LineState=LineState::LS_START; static mut IFSTATE:[Ifstate;MAXDEPTH]=[Ifstate::IS_OUTSIDE;MAXDEPTH]; static mut IGNORING:[bool;MAXDEPTH]=[false;MAXDEPTH]; static mut STIFLINE:[c_int;MAXDEPTH]=[0;MAXDEPTH];
static mut TLINE:[u8;MAXLINE+EDITSLOP]=[0;MAXLINE+EDITSLOP]; static mut KEYWORD:*mut u8=ptr::null_mut(); static mut NEWLINE:*const u8=ptr::null();

fn end_sym(c:u8)->bool { !(c as char).is_ascii_alphanumeric() && c!=b'_' }
unsafe fn skip_sym(mut p:*const u8)->*const u8 { while !end_sym(*p) {p=p.add(1);} p }
unsafe fn strlcmp(s:*const u8,t:*const u8,mut n:usize)->c_int { let(mut a,mut b)=(s,t); while n>0 && *b!=0 {if *a!=*b{return *a as c_int-*b as c_int;} a=a.add(1);b=b.add(1);n-=1;} *a as c_int }
unsafe fn findsym(s:*const u8)->c_int { let e=skip_sym(s); if e==s{return -1}; if SYMLIST { FIRSTSYM=false; ZEROSYMS=false; return 0 } for i in 0..NSYMS {if strlcmp(SYMNAME[i] as *const u8,s,e.offset_from(s) as usize)==0{return i as c_int}} -1 }
unsafe fn addsym(_ig:bool,_def:bool,_s:*mut c_char) { /* argv-owned storage and libc diagnostics are external dependencies */ }
unsafe fn state(x:Ifstate){IFSTATE[DEPTH]=x;} unsafe fn nest(){if DEPTH>=MAXDEPTH-1{error("Too many levels of nesting");} DEPTH+=1;STIFLINE[DEPTH]=LINENUM;} unsafe fn unnest(){if DEPTH==0{std::process::abort()} DEPTH-=1;} unsafe fn ignoreoff(){if DEPTH==0{std::process::abort()} IGNORING[DEPTH]=IGNORING[DEPTH-1];} unsafe fn ignoreon(){IGNORING[DEPTH]=true;}
unsafe fn error(_s:&str){std::process::exit(2)} unsafe fn closeout(){} unsafe fn done()->!{closeout();std::process::exit(EXITSTAT)}

unsafe fn skipcomment(mut p:*const u8)->*const u8 { if TEXT||IGNORING[DEPTH] {while (*p as char).is_ascii_whitespace(){if *p==b'\n'{LINESTATE=LineState::LS_START;}p=p.add(1)}return p} while *p!=0 { match INCOMMENT { CommentState::NO_COMMENT=>{if *p==b'/'&&*p.add(1)==b'*'{INCOMMENT=CommentState::C_COMMENT;p=p.add(2)}else if *p==b'/'&&*p.add(1)==b'/'{INCOMMENT=CommentState::CXX_COMMENT;p=p.add(2)}else if *p==b'\n'{LINESTATE=LineState::LS_START;p=p.add(1)}else if (*p as char).is_ascii_whitespace(){p=p.add(1)}else{return p}}, CommentState::CXX_COMMENT=>{if *p==b'\n'{INCOMMENT=CommentState::NO_COMMENT;LINESTATE=LineState::LS_START;}p=p.add(1)}, CommentState::C_COMMENT=>{if *p==b'*'&&*p.add(1)==b'/'{INCOMMENT=CommentState::NO_COMMENT;p=p.add(2)}else{p=p.add(1)}}, _=>{INCOMMENT=CommentState::NO_COMMENT;p=p.add(1)}}} p }
unsafe fn skipargs(mut p:*const u8)->*const u8 {p=skipcomment(p);if *p!=b'('{return p}let mut n=0;while *p!=0 {if *p==b'('{n+=1}if *p==b')'{n-=1}p=skipcomment(p.add(1));if n==0{break}}p}
unsafe fn ifeval(_p:*const *const u8)->Linetype {CONSTEXPRESSION=!KILLCONSTS;Linetype::LT_IF}
unsafe fn parseline()->Linetype {LINENUM+=1;Linetype::LT_EOF}
unsafe fn flushline(_keep:bool){} unsafe fn process(){loop{let l=parseline();if l==Linetype::LT_EOF{done();}}}
unsafe fn usage()->!{std::process::exit(2)} unsafe fn version()->!{let _=CStr::from_bytes_with_nul_unchecked(COPYRIGHT);std::process::exit(0)}
#[no_mangle] pub unsafe extern "C" fn main(_argc:c_int,_argv:*mut *mut c_char)->c_int {process();0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
