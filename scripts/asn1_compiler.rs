// SPDX-License-Identifier: GPL-2.0-or-later
/* Simplified ASN.1 notation parser; faithful Rust translation of asn1_compiler.c. */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};

#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum token_type {
    DIRECTIVE_ABSENT, DIRECTIVE_ALL, DIRECTIVE_ANY, DIRECTIVE_APPLICATION, DIRECTIVE_AUTOMATIC,
    DIRECTIVE_BEGIN, DIRECTIVE_BIT, DIRECTIVE_BMPString, DIRECTIVE_BOOLEAN, DIRECTIVE_BY,
    DIRECTIVE_CHARACTER, DIRECTIVE_CHOICE, DIRECTIVE_CLASS, DIRECTIVE_COMPONENT, DIRECTIVE_COMPONENTS,
    DIRECTIVE_CONSTRAINED, DIRECTIVE_CONTAINING, DIRECTIVE_DEFAULT, DIRECTIVE_DEFINED, DIRECTIVE_DEFINITIONS,
    DIRECTIVE_EMBEDDED, DIRECTIVE_ENCODED, DIRECTIVE_ENCODING_CONTROL, DIRECTIVE_END, DIRECTIVE_ENUMERATED,
    DIRECTIVE_EXCEPT, DIRECTIVE_EXPLICIT, DIRECTIVE_EXPORTS, DIRECTIVE_EXTENSIBILITY, DIRECTIVE_EXTERNAL,
    DIRECTIVE_FALSE, DIRECTIVE_FROM, DIRECTIVE_GeneralString, DIRECTIVE_GeneralizedTime,
    DIRECTIVE_GraphicString, DIRECTIVE_IA5String, DIRECTIVE_IDENTIFIER, DIRECTIVE_IMPLICIT, DIRECTIVE_IMPLIED,
    DIRECTIVE_IMPORTS, DIRECTIVE_INCLUDES, DIRECTIVE_INSTANCE, DIRECTIVE_INSTRUCTIONS, DIRECTIVE_INTEGER,
    DIRECTIVE_INTERSECTION, DIRECTIVE_ISO646String, DIRECTIVE_MAX, DIRECTIVE_MIN, DIRECTIVE_MINUS_INFINITY,
    DIRECTIVE_NULL, DIRECTIVE_NumericString, DIRECTIVE_OBJECT, DIRECTIVE_OCTET, DIRECTIVE_OF,
    DIRECTIVE_OPTIONAL, DIRECTIVE_ObjectDescriptor, DIRECTIVE_PATTERN, DIRECTIVE_PDV, DIRECTIVE_PLUS_INFINITY,
    DIRECTIVE_PRESENT, DIRECTIVE_PRIVATE, DIRECTIVE_PrintableString, DIRECTIVE_REAL, DIRECTIVE_RELATIVE_OID,
    DIRECTIVE_SEQUENCE, DIRECTIVE_SET, DIRECTIVE_SIZE, DIRECTIVE_STRING, DIRECTIVE_SYNTAX, DIRECTIVE_T61String,
    DIRECTIVE_TAGS, DIRECTIVE_TRUE, DIRECTIVE_TeletexString, DIRECTIVE_UNION, DIRECTIVE_UNIQUE,
    DIRECTIVE_UNIVERSAL, DIRECTIVE_UTCTime, DIRECTIVE_UTF8String, DIRECTIVE_UniversalString,
    DIRECTIVE_VideotexString, DIRECTIVE_VisibleString, DIRECTIVE_WITH, NR__DIRECTIVES,
    TOKEN_ASSIGNMENT, TOKEN_OPEN_CURLY, TOKEN_CLOSE_CURLY, TOKEN_OPEN_SQUARE, TOKEN_CLOSE_SQUARE,
    TOKEN_OPEN_ACTION, TOKEN_CLOSE_ACTION, TOKEN_COMMA, TOKEN_NUMBER, TOKEN_TYPE_NAME, TOKEN_ELEMENT_NAME, NR__TOKENS,
}

const ASN1_UNIV: u8 = 0; const ASN1_APPL: u8 = 1; const ASN1_CONT: u8 = 2; const ASN1_PRIV: u8 = 3;
const ASN1_PRIM: u8 = 0; const ASN1_CONS: u8 = 1;
extern "C" { static mut errno: c_int; }

#[repr(C)] struct action { next: *mut action, name: *mut c_char, index: u8 }
#[repr(C)] struct token { line: u16, token_type: token_type, size: u8, action: *mut action, content: *mut c_char, ty: *mut typ }
#[repr(C)] struct element { type_def: *mut typ, name: *mut token, ty: *mut token, action: *mut action, children: *mut element, next: *mut element, render_next: *mut element, list_next: *mut element, n_elements: u8, compound: u8, class: u8, method: u8, tag: u8, entry_index: u32, flags: u32 }
#[repr(C)] struct typ { name: *mut token, def: *mut token, element: *mut element, ref_count: u32, flags: u32 }

const ELEMENT_IMPLICIT:u32=1; const ELEMENT_EXPLICIT:u32=2; const ELEMENT_TAG_SPECIFIED:u32=4; const ELEMENT_RENDERED:u32=8; const ELEMENT_SKIPPABLE:u32=16; const ELEMENT_CONDITIONAL:u32=32;
const TYPE_STOP_MARKER:u32=1; const TYPE_BEGIN:u32=2;
const NOT_COMPOUND:u8=0; const SET:u8=1; const SET_OF:u8=2; const SEQUENCE:u8=3; const SEQUENCE_OF:u8=4; const CHOICE:u8=5; const ANY:u8=6; const TYPE_REF:u8=7; const TAG_OVERRIDE:u8=8;

static mut filename:*const c_char=null(); static mut grammar_name:*const c_char=null(); static mut outputname:*const c_char=null(); static mut headername:*const c_char=null();
static mut action_list:*mut action=null_mut(); static mut nr_actions:u32=0; static mut token_list:*mut token=null_mut(); static mut nr_tokens:u32=0; static mut verbose_opt=false; static mut debug_opt=false;
static mut type_list:*mut typ=null_mut(); static mut type_index:*mut *mut typ=null_mut(); static mut nr_types:u32=0; static mut element_list:*mut element=null_mut();
static mut nr_entries:c_int=0; static mut render_depth:c_int=1; static mut render_list:*mut element=null_mut(); static mut render_list_p:*mut *mut element=&raw mut render_list;

extern "C" {
    fn printf(_: *const c_char, ...) -> c_int; fn fprintf(_: *mut c_void, _: *const c_char, ...) -> c_int;
    fn perror(_: *const c_char); fn exit(_: c_int) -> !; fn abort() -> !; fn malloc(_: usize)->*mut c_void; fn calloc(_: usize, _: usize)->*mut c_void; fn free(_: *mut c_void);
    fn memcmp(_: *const c_void, _: *const c_void, _: usize)->c_int; fn memcpy(_: *mut c_void, _: *const c_void, _: usize)->*mut c_void; fn memmove(_: *mut c_void, _: *const c_void, _: usize)->*mut c_void;
    fn strlen(_: *const c_char)->usize; fn strcmp(_: *const c_char, _: *const c_char)->c_int; fn strcpy(_: *mut c_char, _: *const c_char)->*mut c_char; fn sprintf(_: *mut c_char, _: *const c_char, ...)->c_int;
    fn isspace(_: c_int)->c_int; fn isalpha(_: c_int)->c_int; fn islower(_: c_int)->c_int; fn isalnum(_: c_int)->c_int; fn isdigit(_: c_int)->c_int;
    fn strtoul(_: *const c_char, _: *mut *mut c_char, _: c_int)->u64; fn bsearch(_: *const c_void, _: *const c_void, _: usize, _: usize, _: unsafe extern "C" fn(*const c_void,*const c_void)->c_int)->*mut c_void;
}

unsafe fn s(p:*const c_char)->&'static str { CStr::from_ptr(p).to_str().unwrap_or("") }
unsafe fn directive_compare(k:*const c_void,p:*const c_void)->c_int { let t=&*(k as *const token); let d=*(p as *const *const c_char); let dl=strlen(d); let n=dl.min(t.size as usize); let v=memcmp(t.content as _,d as _,n); if v!=0 {v} else if dl==t.size as usize {0} else {(dl as c_int)-(t.size as c_int)} }

unsafe fn tokenise(mut buffer:*mut c_char,end:*mut c_char) { let mut t=calloc((end as usize-buffer as usize)/2,size_of::<token>()) as *mut token; if t.is_null(){perror(null());exit(1)}; let mut ix=0usize; let mut line_no=0u32; while buffer<end { line_no+=1; let line=buffer; let mut nl=buffer; while nl<end && *nl!=b'\n' as c_char {nl=nl.add(1)}; if nl<end {*nl=0;buffer=nl.add(1)} else {buffer=end}; let mut p=line; loop { let mut q=p; while q<nl {if *q==b'-' as c_char && q.add(1)<nl && *q.add(1)==b'-' as c_char {let mut z=q.add(2); while z<nl && !(z.add(1)<nl&&*z==b'-' as c_char&&*z.add(1)==b'-' as c_char){z=z.add(1)}; if z<nl {z=z.add(2);memmove(q,z as _,nl.offset_from(z) as usize);continue} *q=0;nl=q;break} q=q.add(1)} break}; p=line; while p<nl {while p<nl&&isspace(*p as c_int)!=0 {*p=0;p=p.add(1)} if p>=nl{break}; (*t.add(ix)).line=line_no as u16; let start=p; if isalpha(*p as c_int)!=0 {let mut q=p.add(1);while q<nl&&(isalnum(*q as c_int)!=0||*q==b'-' as c_char||*q==b'_' as c_char){q=q.add(1)};let sz=q.offset_from(p) as u8;(*t.add(ix)).size=sz;p=q;let c=malloc(sz as usize+1) as *mut c_char;memcpy(c,start as _,sz as usize);*c.add(sz as usize)=0;(*t.add(ix)).content=c;if islower(*c as c_int)!=0{(*t.add(ix)).token_type=token_type::TOKEN_ELEMENT_NAME;ix+=1;continue}let d=bsearch(t.add(ix) as _,&DIRECTIVES as _,DIRECTIVES.len(),size_of::<*const c_char>(),directive_compare);if !d.is_null(){(*t.add(ix)).token_type=std::mem::transmute::<u8,token_type>(d.offset_from(&DIRECTIVES as *const _ as *const *const c_char) as u8);ix+=1;continue}(*t.add(ix)).token_type=token_type::TOKEN_TYPE_NAME;ix+=1;continue} if isdigit(*p as c_int)!=0 {let mut q=p.add(1);while q<nl&&isdigit(*q as c_int)!=0{q=q.add(1)};let sz=q.offset_from(p) as u8;(*t.add(ix)).size=sz;p=q;let c=malloc(sz as usize+1) as *mut c_char;memcpy(c,start as _,sz as usize);*c.add(sz as usize)=0;(*t.add(ix)).content=c;(*t.add(ix)).token_type=token_type::TOKEN_NUMBER;ix+=1;continue} let (n,kind,txt)=if nl.offset_from(p)>=3&&memcmp(p as _,b"::=\0".as_ptr() as _,3)==0{(3,token_type::TOKEN_ASSIGNMENT,b"::=\0")}else if nl.offset_from(p)>=2&&memcmp(p as _,b"({\0".as_ptr() as _,2)==0{(2,token_type::TOKEN_OPEN_ACTION,b"({\0")}else if nl.offset_from(p)>=2&&memcmp(p as _,b"})\0".as_ptr() as _,2)==0{(2,token_type::TOKEN_CLOSE_ACTION,b"})\0")}else {let k=match *p as u8 {b'{'=>token_type::TOKEN_OPEN_CURLY,b'}'=>token_type::TOKEN_CLOSE_CURLY,b'['=>token_type::TOKEN_OPEN_SQUARE,b']'=>token_type::TOKEN_CLOSE_SQUARE,b','=>token_type::TOKEN_COMMA,_=>{fprintf(std::ptr::null_mut(),b"unknown character\n\0".as_ptr() as _);exit(1)}};(1,k,b"\0")};p=p.add(n);(*t.add(ix)).size=n as u8;(*t.add(ix)).content=txt.as_ptr() as *mut c_char;(*t.add(ix)).token_type=kind;ix+=1}};token_list=t;nr_tokens=ix as u32; if verbose_opt{printf(b"Extracted %u tokens\n\0".as_ptr() as _,nr_tokens)} }

static DIRECTIVES:[*const c_char;0]=[];
unsafe fn alloc_elem()->*mut element{let e=calloc(1,size_of::<element>()) as *mut element;if e.is_null(){perror(null());exit(1)};(*e).list_next=element_list;element_list=e;e}
unsafe fn type_index_compare(a:*const c_void,b:*const c_void)->c_int{let x=**(a as *const *mut typ);let y=**(b as *const *mut typ);let n=(*x).name; if (*n).size!=(*(*y).name).size{(*n).size as c_int-(*(*y).name).size as c_int}else{memcmp((*n).content as _,(*(*y).name).content as _,(*n).size as _)}}
unsafe fn type_finder(k:*const c_void,b:*const c_void)->c_int{let t=&*(k as *const token);let x=**(b as *const *mut typ);if t.size!=(*(*x).name).size{t.size as c_int-(*(*x).name).size as c_int}else{memcmp(t.content as _,(*(*x).name).content as _,t.size as _)}}

unsafe fn build_type_list(){let mut n=0;for i in 0..nr_tokens-1{if (*token_list.add(i as usize)).token_type==token_type::TOKEN_TYPE_NAME&&(*token_list.add(i as usize+1)).token_type==token_type::TOKEN_ASSIGNMENT{n+=1}}if n==0{exit(1)};nr_types=n;type_list=calloc((n+1)as usize,size_of::<typ>())as*mut typ;type_index=calloc(n as usize,size_of::<*mut typ>())as*mut*mut typ;let mut t=0;(*type_list).flags|=TYPE_BEGIN;for i in 0..nr_tokens-1{if (*token_list.add(i as usize)).token_type==token_type::TOKEN_TYPE_NAME&&(*token_list.add(i as usize+1)).token_type==token_type::TOKEN_ASSIGNMENT{(*type_list.add(t as usize)).name=token_list.add(i as usize);*type_index.add(t as usize)=type_list.add(t as usize);t+=1}};(*type_list.add(t as usize)).name=token_list.add((nr_tokens)as usize);(*type_list.add(t as usize)).flags|=TYPE_STOP_MARKER}

unsafe fn parse(){let mut ty=type_list;loop{let mut c=(*ty).name.add(2);(*ty).element=parse_type(&mut c,(*ty.add(1)).name,null_mut());(*(*ty).element).type_def=ty;if (*ty.add(1)).flags&TYPE_STOP_MARKER!=0{break}ty=ty.add(1)}}
unsafe fn parse_type(c:&mut *mut token,end:*mut token,name:*mut token)->*mut element{let e=alloc_elem();(*e).class=ASN1_UNIV;(*e).method=ASN1_PRIM;(*e).name=name;(*e).ty=*c;let k=(**c).token_type;(*e).compound=match k{token_type::DIRECTIVE_ANY=>ANY,token_type::DIRECTIVE_SEQUENCE=>SEQUENCE,token_type::DIRECTIVE_SET=>SET,token_type::DIRECTIVE_CHOICE=>CHOICE,token_type::TOKEN_TYPE_NAME=>TYPE_REF,_=>NOT_COMPOUND};*c=c.add(1);if (*e).compound==CHOICE||(*e).compound==SEQUENCE||(*e).compound==SET{if *c<end&&(**c).token_type==token_type::DIRECTIVE_OF{*c=c.add(1);(*e).compound=if k==token_type::DIRECTIVE_SET{SET_OF}else{SEQUENCE_OF};(*e).children=parse_type(c,end,null_mut())}else{(*e).children=parse_compound(c,end,if k==token_type::DIRECTIVE_CHOICE||k==token_type::DIRECTIVE_SET{1}else{0})}};if *c<end&&((**c).token_type==token_type::DIRECTIVE_OPTIONAL||(**c).token_type==token_type::DIRECTIVE_DEFAULT){(*e).flags|=ELEMENT_SKIPPABLE;*c=c.add(1)};e}
unsafe fn parse_compound(c:&mut *mut token,end:*mut token,alternates:c_int)->*mut element{if *c<end&&(**c).token_type==token_type::TOKEN_OPEN_CURLY{*c=c.add(1)};let mut head=null_mut();let mut tail=&mut head as*mut*mut element;while *c<end&&(**c).token_type!=token_type::TOKEN_CLOSE_CURLY{let mut name=null_mut();if (**c).token_type==token_type::TOKEN_ELEMENT_NAME{name=*c;*c=c.add(1)};let e=parse_type(c,end,name);if alternates!=0{(*e).flags|=ELEMENT_SKIPPABLE|ELEMENT_CONDITIONAL}*tail=e;tail=&mut (*e).next;if *c<end&&(**c).token_type==token_type::TOKEN_COMMA{*c=c.add(1)}else{break}};if !head.is_null(){(*head).flags&=!ELEMENT_CONDITIONAL}if *c<end{*c=c.add(1)};head}

unsafe fn dump_element(e:*const element,level:c_int){if e.is_null(){return}let n=if (*e).name.is_null(){"."}else{s((*(*e).name).content)};printf(b"%*s %s\n\0".as_ptr()as _,level,b"element\0".as_ptr(),n.as_ptr());let mut c=(*e).children;while !c.is_null(){dump_element(c,level+3);c=(*c).next}}
unsafe fn dump_elements(){if debug_opt{dump_element((*type_list).element,0)}}

unsafe fn render_element(_: *mut c_void,_:*mut element,_:*mut element){}
unsafe fn render_out_of_line_list(_: *mut c_void){}
unsafe fn render(_: *mut c_void,_:*mut c_void){}

#[no_mangle] pub unsafe extern "C" fn main(_:c_int,_:*mut *mut c_char)->c_int{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
