# SPDX-License-Identifier: GPL-2.0-only
"""Private independent callers; only unchanged original C is an algorithm oracle."""
import ast
from pathlib import Path
import re

SYMBOLS = ('get_option', 'get_options', 'memparse', 'next_arg')
SIGNATURES = (
    ('int', 'char **s, int *p', 's, p', 'ffi::c_int', 's: *mut *mut ffi::c_char, p: *mut ffi::c_int', '*mut *mut ffi::c_char, *mut ffi::c_int'),
    ('char *', 'const char *s, int n, int *p', 's, n, p', '*mut ffi::c_char', 's: *const ffi::c_char, n: ffi::c_int, p: *mut ffi::c_int', '*const ffi::c_char, ffi::c_int, *mut ffi::c_int'),
    ('unsigned long long', 'const char *s, char **p', 's, p', 'ffi::c_ulonglong', 's: *const ffi::c_char, p: *mut *mut ffi::c_char', '*const ffi::c_char, *mut *mut ffi::c_char'),
    ('char *', 'char *s, char **p, char **v', 's, p, v', '*mut ffi::c_char', 's: *mut ffi::c_char, p: *mut *mut ffi::c_char, v: *mut *mut ffi::c_char', '*mut ffi::c_char, *mut *mut ffi::c_char, *mut *mut ffi::c_char'),
)
SPECIAL = (b'', b'-', b'+1', b'0x', b'0xg', b'0779', b'08', b'1,2,3', b'-2147483648',
    b'2147483648', b'4294967295', b'18446744073709551615', b'18446744073709551616',
    b'99999999999999999999999999E', b'16E', b'15E',
    *(str(1).encode()+bytes([c]) for c in b'KMGTPEkmgtpe'), b'K', b'-K', b'0x1E',
    b'a=b', b'=a=b', b'a="b c" z', b'"a=b c" z', b'""', b'a="', b' a=b', b'a\xa0b',
    b'a,b,c', b',,', b'x=1,other', b'foo="bar baz" qux=1', b'foo=bar "', b'bbb= jjj kkk="a=b"',
    b'a\t\n\r\v\fb', b'"', b'a=\xa0b')
RANGES = (b'', b'-', b'1,2,3', b'1-4,9', b'4-1', b'-3--1,4', b'0-0', b'0-20', b'1-',
    b'1-x', b'1-2-4', b'-2147483648', b'2147483647', b'0x1-0x5', b'3,',
    b'-2147483648-0', b'-1-2147483647', b'0-4294967295')
HUGE = (b'0-2147483647', b'0-2147483647,1', b'-2147483648-2147483647',
    b'2147483647--2147483648', b'0-2147483646,1,2', b'-1-2147483647', b'2147483647-2147483647')
RANDOM = 20000

# Private link fixtures only: parse_option_str is deliberately not a module
# import/export. Keep the four exported-API module workloads unchanged.
FIFTH_CASES = (b'', b'a', b'a,b', b',', b',,', b',a,', b'a,,b', b'aa,a,ab',
               b'foo=bar,foo', b'foo,foobar', b' a,a ', b'a\xff,b', b'\xff', b'a,b,c,')
FIFTH_COUNT = len(FIFTH_CASES)**2 + sum(len(s)+1 for s in FIFTH_CASES) + 40001


def fifth_source(caller):
    """Independent C/Rust loops compare fifth API with the unchanged C body."""
    rows=['{'+','.join(map(str,s+b'\0'*(64-len(s))))+'}' for s in FIFTH_CASES]
    if caller == 'c':
        return r'''
#include <linux/kernel.h>
#include <linux/string.h>
bool cmdline_reference_parse_option_str(const char *, const char *);
bool cmdline_fifth_call(const char *, const char *);
int cmdline_fifth_exercise(void);
noinline bool cmdline_fifth_call(const char *s, const char *o)
{
    bool (* volatile pointer)(const char *, const char *) = parse_option_str;
    return pointer(s, o);
}
static const unsigned char cases[][64] = { @ROWS@ };
static int one(const unsigned char *s, const unsigned char *o, int alias)
{
    char a[64], b[64], x[64], y[64];
    memcpy(a,s,64); memcpy(b,s,64); memcpy(x,o,64); memcpy(y,o,64);
    if (cmdline_fifth_call(a,alias<0?x:a+alias) !=
        cmdline_reference_parse_option_str(b,alias<0?y:b+alias)) return -22;
    if (memcmp(a,s,64)||memcmp(b,s,64)||memcmp(x,o,64)||memcmp(y,o,64)) return -22;
    return 0;
}
int cmdline_fifth_exercise(void)
{
    unsigned int i,j,k,n,m,state=0x13579bdf,count=0;
    unsigned char s[64],o[64];
    for(i=0;i<ARRAY_SIZE(cases);i++) {
        for(j=0;j<ARRAY_SIZE(cases);j++) { if(one(cases[i],cases[j],-1)) return -22; count++; }
        for(j=0;j<=strlen((const char *)cases[i]);j++) { if(one(cases[i],cases[0],j)) return -22; count++; }
    }
    for(i=0;i<20000;i++) {
        memset(s,0,64);memset(o,0,64);
        state=state*1664525u+1013904223u;n=state%63;
        state=state*1664525u+1013904223u;m=state%63;
        for(k=0;k<n;k++) {state=state*1664525u+1013904223u;s[k]=(state%5)?(unsigned char)(state%255+1):',';}
        for(k=0;k<m;k++) {state=state*1664525u+1013904223u;o[k]=(state%5)?(unsigned char)(state%255+1):',';}
        if(one(s,o,-1)||one(s,o,state%(n+1))) return -22;
        count+=2;
    }
    if(cmdline_fifth_call("",NULL)!=cmdline_reference_parse_option_str("",NULL)) return -22;
    return count+1==@COUNT@ ? 0 : -22;
}
'''.replace('@ROWS@',','.join(rows)).replace('@COUNT@',str(FIFTH_COUNT))
    if caller != 'rust': raise ValueError('unknown fifth caller')
    return r'''
//! Private differential workload for the non-exported fifth cmdline API.
use kernel::{bindings, ffi};
unsafe extern "C" { fn cmdline_reference_parse_option_str(s:*const ffi::c_char,o:*const ffi::c_char)->bool; }
#[no_mangle]
#[inline(never)]
/// Call the genuine native binding through an observable function pointer.
/// # Safety
/// Strings must be terminated; option may be null only when input is empty.
pub unsafe extern "C" fn cmdline_fifth_call(s:*const ffi::c_char,o:*const ffi::c_char)->bool {
    let pointer: unsafe extern "C" fn(*const ffi::c_char,*const ffi::c_char)->bool=bindings::parse_option_str;
    // SAFETY: Each caller supplies terminated initialized strings, or empty s/null o.
    unsafe { core::ptr::read_volatile(&pointer)(s,o) }
}
static CASES:[[u8;64];@N@]=[@ROWS@];
fn one(s:&[u8;64],o:&[u8;64],alias:Option<usize>)->bool {
    let a=*s;let b=*s;let x=*o;let y=*o;
    // SAFETY: All arrays have a NUL; alias is within the s string including its NUL.
    let same=unsafe {
        let ap=a.as_ptr().cast();let bp=b.as_ptr().cast();
        let xp=match alias {Some(i)=>a.as_ptr().add(i).cast(),None=>x.as_ptr().cast()};
        let yp=match alias {Some(i)=>b.as_ptr().add(i).cast(),None=>y.as_ptr().cast()};
        cmdline_fifth_call(ap,xp)==cmdline_reference_parse_option_str(bp,yp)
    };
    same && a==*s && b==*s && x==*o && y==*o
}
fn next(state:&mut u32)->u32 {*state=state.wrapping_mul(1664525).wrapping_add(1013904223);*state}
#[no_mangle]
/// Run independent static, overlapping, random and empty/null comparisons.
pub extern "C" fn cmdline_fifth_exercise()->i32 {
    let mut count=0u32;
    for s in &CASES {
        for o in &CASES {if !one(s,o,None){return -22;} count=count.wrapping_add(1);}
        for i in 0..64 {if !one(s,&CASES[0],Some(i)){return -22;}count=count.wrapping_add(1);if s[i]==0 {break;}}
    }
    let mut state=0x13579bdfu32;
    for _ in 0..20000 {
        let mut s=[0u8;64];let mut o=[0u8;64];
        let n=(next(&mut state)%63) as usize;let m=(next(&mut state)%63) as usize;
        for byte in s.iter_mut().take(n) {let v=next(&mut state);*byte=if v%5==0 {b','} else {(v%255+1) as u8};}
        for byte in o.iter_mut().take(m) {let v=next(&mut state);*byte=if v%5==0 {b','} else {(v%255+1) as u8};}
        if !one(&s,&o,None)||!one(&s,&o,Some(state as usize%(n+1))){return -22;}count=count.wrapping_add(2);
    }
    // SAFETY: An empty input never reads option, which may therefore be null.
    if unsafe {cmdline_fifth_call(b"\0".as_ptr().cast(),core::ptr::null())!=cmdline_reference_parse_option_str(b"\0".as_ptr().cast(),core::ptr::null())}{return -22;}
    if count.wrapping_add(1)==@COUNT@ {0} else {-22}
}
'''.replace('@ROWS@',','.join(row.replace('{','[').replace('}',']') for row in rows)).replace('@N@',str(len(rows))).replace('@COUNT@',str(FIFTH_COUNT))


def strings(text):
    return [ast.literal_eval('b'+s) for s in re.findall(r'"(?:[^"\\]|\\.)*"', text)]


def corpus(root):
    text = (Path(root)/'lib/tests/cmdline_kunit.c').read_text()
    def array(name):
        match = re.search(r'\b'+name+r'\[\]\s*=\s*\{(.*?)\};', text, re.S)
        if not match: raise ValueError('missing original cmdline array '+name)
        return strings(match[1])
    suffixes = array('cmdline_test_strings')
    ranges = array('cmdline_test_range_strings')
    mem = re.search(r'\btestdata\[\]\s*=\s*\{(.*?)\n\};', text, re.S)
    if not mem: raise ValueError('missing original memparse cases')
    memory = strings(mem[1])[::2]
    if len(suffixes) != 16 or len(ranges) != 20 or len(memory) != 30:
        raise ValueError('original cmdline corpus changed; review coverage')
    return suffixes, list(SPECIAL)+memory, list(RANGES)+ranges


def case_count(root):
    suffix, special, ranges = corpus(root)
    return len(suffix)*513 + len(special) + 255 + RANDOM + 4096 + len(ranges)*20 + len(HUGE) + 4 + 3


def marker(root, caller):
    if caller not in ('c', 'rust'): raise ValueError('unknown cmdline caller')
    return f'LUPOS_CMDLINE_{"RUST_ABI" if caller == "rust" else "ABI"}_OK cases={case_count(root)}'.encode()


def reference_source(root):
    declarations = ''.join(f'{sig[0]} cmdline_reference_{name}({sig[1]});\n' for name,sig in zip(SYMBOLS,SIGNATURES))
    return ('#include <linux/kernel.h>\n#include <linux/export.h>\n'+declarations+
        'bool cmdline_reference_parse_option_str(const char *, const char *);\n'+
        '#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(symbol)\n'+
        ''.join(f'#define {name} cmdline_reference_{name}\n' for name in (*SYMBOLS,'parse_option_str'))+
        '#include "'+str(Path(root)/'lib/cmdline.c')+'"\n')


C_BODY = r'''
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/string.h>
@WRAPPERS@
@ARRAYS@
#define CHECK(x) do { if (!(x)) { pr_err("LUPOS_CMDLINE_FAILED line=%d\n", __LINE__); return -22; } } while (0)
static long off(char *p, char *base) { return p ? p-base : -1; }
static int one(const unsigned char *s)
{
    char a[128], b[128]; int x, y, nullable, alias, step;
    char *p, *q, *v, *w, *r, *t;
    for (nullable=0; nullable<2; nullable++) {
        memcpy(a,s,128); memcpy(b,s,128); p=a; q=b; x=y=0x12345678;
        CHECK(cmdline_call_get_option(&p,nullable?NULL:&x)==cmdline_reference_get_option(&q,nullable?NULL:&y));
        CHECK(x==y && off(p,a)==off(q,b) && !memcmp(a,b,128));
        p=q=NULL;
        CHECK(cmdline_call_memparse(a,nullable?NULL:&p)==cmdline_reference_memparse(b,nullable?NULL:&q));
        CHECK(off(p,a)==off(q,b));
    }
    for(alias=0;alias<2;alias++) {
        memcpy(a,s,128); memcpy(b,s,128); r=a; t=b;
        for(step=0;step<128;step++) {
            p=q=v=w=NULL;
            r=cmdline_call_next_arg(r,&p,alias?&p:&v);
            t=cmdline_reference_next_arg(t,&q,alias?&q:&w);
            CHECK(off(r,a)==off(t,b) && off(p,a)==off(q,b) && off(v,a)==off(w,b) && !memcmp(a,b,128));
            if(!*r) break;
        }
        CHECK(step<128);
    }
    return 0;
}
static int numbers(const unsigned char *s, int n)
{
    int a[32], b[32]; char *p,*q;
    memset(a,0x5a,sizeof(a)); memset(b,0x5a,sizeof(b));
    p=cmdline_call_get_options((const char *)s,n,a);
    q=cmdline_reference_get_options((const char *)s,n,b);
    CHECK(p==q && !memcmp(a,b,sizeof(a)));
    return 0;
}
static int exercise(void)
{
    unsigned int i,j,k,count=0,state=941; int n; unsigned char input[128],digits[4];
    for(i=0;i<ARRAY_SIZE(special);i++) { CHECK(!one(special[i])); count++; }
    for(i=0;i<ARRAY_SIZE(suffixes);i++) {
        CHECK(!one(suffixes[i])); count++;
        for(j=0;j<256;j++) {
            unsigned int length=0, slen=strlen((const char *)suffixes[i]);
            if(j>=100) digits[length++]='0'+j/100;
            if(j>=10) digits[length++]='0'+(j/10)%10;
            digits[length++]='0'+j%10;
            memset(input,0,128); memcpy(input,digits,length); memcpy(input+length,suffixes[i],slen);
            CHECK(!one(input)); count++;
            memset(input,0,128); memcpy(input,suffixes[i],slen); memcpy(input+slen,digits,length);
            CHECK(!one(input)); count++;
        }
    }
    for(i=1;i<256;i++) {
        memset(input,0,128); input[0]='a';input[1]=i;input[2]='b';input[3]='=';input[4]=i;input[5]='x';
        CHECK(!one(input)); count++;
    }
    for(i=0;i<20000;i++) {
        state=state*1664525U+1013904223U; n=(state>>24)%90;
        memset(input,0,128);
        for(k=0;k<n;k++) { state=state*1664525U+1013904223U; input[k]=state>>24; }
        CHECK(!one(input)); count++;
    }
    for(i=0;i<ARRAY_SIZE(ranges);i++) for(n=-2;n<=17;n++) { CHECK(!numbers(ranges[i],n)); count++; }
    for(i=0;i<ARRAY_SIZE(huge);i++) { CHECK(!numbers(huge[i],0)); count++; }
    for(i=0;i<4096;i++) {
        memset(input,0,128);
        for(k=0;k<5;k+=2) { state=state*1664525U+1013904223U; input[k]='0'+(state>>24)%10; }
        input[1]='-'; input[3]=',';
        state=state*1664525U+1013904223U; n=(int)((state>>24)%20)-2;
        CHECK(!numbers(input,n)); count++;
    }
    for(n=-1;n<=2;n++) { int a=19,b=19;char *p=NULL,*q=NULL;
        CHECK(cmdline_call_get_option(&p,&a)==cmdline_reference_get_option(&q,&b) && a==b && p==q);
        CHECK(!numbers(NULL,n)); count++;
    }
    for(i=0;i<=8;i+=4) {
        union { int alignment[32]; char bytes[128]; } a,b; char *p=a.bytes,*q=b.bytes;
        memset(&a,0,sizeof(a)); memset(&b,0,sizeof(b)); memcpy(a.bytes,"123456,7",9); memcpy(b.bytes,"123456,7",9);
        CHECK(cmdline_call_get_option(&p,(int *)(a.bytes+i))==cmdline_reference_get_option(&q,(int *)(b.bytes+i)));
        CHECK(off(p,a.bytes)==off(q,b.bytes) && !memcmp(&a,&b,sizeof(a))); count++;
    }
    CHECK(count==@COUNT@);
    return 0;
}
static int __init cmdline_abi_init(void) { int e=exercise(); if(e) return e; pr_info("@MARKER@\n"); return 0; }
static void __exit cmdline_abi_exit(void) {}
module_init(cmdline_abi_init);
module_exit(cmdline_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Independent cmdline native ABI and parsing test");
'''

RUST_BODY = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Independent Rust parsing workload with genuine native bindings.
use kernel::{bindings, ffi};
@WRAPPERS@
@ARRAYS@
macro_rules! check { ($e:expr) => { if !$e { return Err(-22); } }; }
fn off(p: *mut ffi::c_char, base: *mut ffi::c_char) -> isize {
    if p.is_null() { -1 } else { (p as usize).wrapping_sub(base as usize) as isize }
}
fn one(s: &[u8;128]) -> Result<(),i32> {
    // SAFETY: Buffers are terminated, full-sized and separate. Raw output
    // pointers preserve legal aliasing and no Rust reference aliases a write.
    unsafe {
        for nullable in 0..2 {
            let (mut a,mut b)=(*s,*s);
            let (ap,bp)=(a.as_mut_ptr().cast(),b.as_mut_ptr().cast());
            let (mut p,mut q)=(ap,bp); let (mut x,mut y)=(0x12345678,0x12345678);
            check!(cmdline_call_get_option(&mut p,if nullable==1 { core::ptr::null_mut() } else { &mut x }) ==
                cmdline_reference_get_option(&mut q,if nullable==1 { core::ptr::null_mut() } else { &mut y }));
            check!(x==y && off(p,ap)==off(q,bp) && a==b);
            p=core::ptr::null_mut(); q=core::ptr::null_mut();
            check!(cmdline_call_memparse(ap,if nullable==1 { core::ptr::null_mut() } else { &mut p }) ==
                cmdline_reference_memparse(bp,if nullable==1 { core::ptr::null_mut() } else { &mut q }));
            check!(off(p,ap)==off(q,bp));
        }
        for alias in 0..2 {
            let (mut a,mut b)=(*s,*s);
            let (ap,bp)=(a.as_mut_ptr().cast(),b.as_mut_ptr().cast());
            let (mut r,mut t)=(ap,bp); let mut done=false;
            for _ in 0..128 {
                let (mut p,mut q,mut v,mut w)=(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut());
                let pp=&raw mut p; let qp=&raw mut q;
                r=cmdline_call_next_arg(r,pp,if alias==1 { pp } else { &raw mut v });
                t=cmdline_reference_next_arg(t,qp,if alias==1 { qp } else { &raw mut w });
                check!(off(r,ap)==off(t,bp) && off(p,ap)==off(q,bp) && off(v,ap)==off(w,bp) && a==b);
                if *r==0 { done=true; break; }
            }
            check!(done);
        }
    }
    Ok(())
}
fn numbers(s: *const ffi::c_char,n:i32) -> Result<(),i32> {
    let (mut a,mut b)=([0x5a5a5a5ai32;32],[0x5a5a5a5ai32;32]);
    // SAFETY: Static bounded ranges never produce a wrapped fill count;
    // huge ranges use count-only n=0. Null is used only in its valid domain.
    unsafe { check!(cmdline_call_get_options(s,n,a.as_mut_ptr())==cmdline_reference_get_options(s,n,b.as_mut_ptr())); }
    check!(a==b); Ok(())
}
fn exercise() -> Result<(),i32> {
    let mut count=0u32; let mut state=941u32;
    // SAFETY: Static arrays have 128-byte rows with a terminator. Every index
    // below is bounded before access; generated suffixes are at most 3 bytes,
    // decimal digits at most 3, and random strings at most 89. Raw copies use
    // separate local buffers. This avoids pulling GPL-only core panic helpers
    // into the unrestricted-export consumer, without substituting any stubs.
    unsafe {
    let mut index=0;
    while index<SPECIAL.len() { one(&*SPECIAL.as_ptr().add(index))?; count=count.wrapping_add(1); index+=1; }
    index=0;
    while index<SUFFIXES.len() {
        let s=&*SUFFIXES.as_ptr().add(index); index+=1;
        one(s)?; count=count.wrapping_add(1);
        let mut slen=0usize;
        while slen<128 && *s.as_ptr().add(slen)!=0 { slen+=1; }
        check!(slen<120);
        for j in 0..256u32 {
            let mut digits=[0u8;4]; let mut length=0;
            if j>=100 { *digits.as_mut_ptr().add(length)=b'0'+(j/100) as u8; length+=1; }
            if j>=10 { *digits.as_mut_ptr().add(length)=b'0'+((j/10)%10) as u8; length+=1; }
            *digits.as_mut_ptr().add(length)=b'0'+(j%10) as u8; length+=1;
            let mut input=[0u8;128];
            core::ptr::copy_nonoverlapping(digits.as_ptr(),input.as_mut_ptr(),length);
            core::ptr::copy_nonoverlapping(s.as_ptr(),input.as_mut_ptr().add(length),slen);
            one(&input)?; count=count.wrapping_add(1);
            input=[0;128];
            core::ptr::copy_nonoverlapping(s.as_ptr(),input.as_mut_ptr(),slen);
            core::ptr::copy_nonoverlapping(digits.as_ptr(),input.as_mut_ptr().add(slen),length);
            one(&input)?; count=count.wrapping_add(1);
        }
    }
    for i in 1..256u32 {
        let mut input=[0u8;128]; input[0]=b'a';input[1]=i as u8;input[2]=b'b';input[3]=b'=';input[4]=i as u8;input[5]=b'x';
        one(&input)?; count=count.wrapping_add(1);
    }
    for _ in 0..20000 {
        state=state.wrapping_mul(1664525).wrapping_add(1013904223); let n=((state>>24)%90) as usize;
        let mut input=[0u8;128];
        let mut at=0;
        while at<n { state=state.wrapping_mul(1664525).wrapping_add(1013904223); *input.as_mut_ptr().add(at)=(state>>24) as u8; at+=1; }
        one(&input)?; count=count.wrapping_add(1);
    }
    index=0;
    while index<RANGES.len() { for n in -2..=17 { numbers(RANGES.as_ptr().add(index).cast(),n)?; count=count.wrapping_add(1); } index+=1; }
    index=0;
    while index<HUGE.len() { numbers(HUGE.as_ptr().add(index).cast(),0)?; count=count.wrapping_add(1); index+=1; }
    for _ in 0..4096 {
        let mut input=[0u8;128];
        for k in 0..3usize { state=state.wrapping_mul(1664525).wrapping_add(1013904223); *input.as_mut_ptr().add(k*2)=b'0'+((state>>24)%10) as u8; }
        input[1]=b'-'; input[3]=b',';
        state=state.wrapping_mul(1664525).wrapping_add(1013904223);
        let n=((state>>24)%20) as i32-2;
        numbers(input.as_ptr().cast(),n)?; count=count.wrapping_add(1);
    }
    for n in -1..=2 {
        let (mut a,mut b)=(19,19); let (mut p,mut q)=(core::ptr::null_mut(),core::ptr::null_mut());
        // SAFETY: get_option accepts a null string; get_options writes at least ints[0].
        check!(cmdline_call_get_option(&mut p,&mut a)==cmdline_reference_get_option(&mut q,&mut b));
        check!(a==b && p==q); numbers(core::ptr::null(),n)?; count=count.wrapping_add(1);
    }
    for j in 0..3usize {
        let i=j*4;
        let (mut a,mut b)=([0i32;32],[0i32;32]);
        // SAFETY: int arrays provide alignment. Only raw pointers access bytes
        // during output/input overlap, including writes before delimiter reads.
        {
            let ap=a.as_mut_ptr().cast::<ffi::c_char>(); let bp=b.as_mut_ptr().cast::<ffi::c_char>();
            core::ptr::copy_nonoverlapping(b"123456,7\0".as_ptr(),ap.cast(),9);
            core::ptr::copy_nonoverlapping(b"123456,7\0".as_ptr(),bp.cast(),9);
            let (mut p,mut q)=(ap,bp);
            check!(cmdline_call_get_option(&mut p,ap.add(i).cast())==cmdline_reference_get_option(&mut q,bp.add(i).cast()));
            check!(off(p,ap)==off(q,bp) && a==b);
        }
        count=count.wrapping_add(1);
    }
    }
    check!(count==@COUNT@); Ok(())
}
/// Run the entire Rust workload on every load.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    if let Err(e)=exercise() { return e; }
    // SAFETY: Static terminated format with no conversion arguments.
    unsafe { bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast()); }
    0
}
/// No persistent resources survive the workload.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn()->ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=Proprietary\0description=Independent cmdline native ABI and parsing test\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8;INFO.len()] = {
    let mut b=[0;INFO.len()]; let mut i=0; while i<b.len() { b[i]=INFO.as_bytes()[i]; i+=1; } b
};
#[used]
static __IS_RUST_MODULE: () = ();
'''


def caller_source(root, caller):
    if caller not in ('c','rust'): raise ValueError('unknown cmdline caller')
    suffixes,special,ranges=corpus(root)
    arrays=[]
    for name,rows in (('suffixes',suffixes),('special',special),('ranges',ranges),('huge',HUGE)):
        if any(len(row)>=128 for row in rows): raise ValueError('cmdline fixture input too long')
        values=[','.join(map(str,row+b'\0'*(128-len(row)))) for row in rows]
        if caller=='c': arrays.append('static const unsigned char '+name+'[][128]={'+','.join('{'+v+'}' for v in values)+'};')
        else: arrays.append('static '+name.upper()+': [[u8;128];'+str(len(rows))+']=['+','.join('['+v+']' for v in values)+'];')
    wrappers=[]
    for name,sig in zip(SYMBOLS,SIGNATURES):
        ret,args,passed,rret,rargs,rtypes=sig
        if caller=='c':
            wrappers.append(f'{ret} cmdline_reference_{name}({args});\nstatic noinline {ret} cmdline_call_{name}({args}) {{\n'
                f'    {ret} (*volatile selected)({args})={name}; return selected({passed});\n}}')
        else:
            wrappers.append(f'unsafe extern "C" {{ fn cmdline_reference_{name}({rargs})->{rret}; }}\n'
                '/// Invoke the original interface through its native typed binding.\n/// # Safety\n/// Caller satisfies the original pointer contract.\n'
                '#[no_mangle]\n#[inline(never)]\n'+f'pub unsafe extern "C" fn cmdline_call_{name}({rargs})->{rret} {{\n'
                f'    let pointer: unsafe extern "C" fn({rtypes})->{rret}=bindings::{name};\n'
                '    // SAFETY: The complete workload supplies valid storage and domains.\n'+
                f'    unsafe {{ core::ptr::read_volatile(&pointer)({passed}) }}\n}}')
    return (C_BODY if caller=='c' else RUST_BODY).replace('@WRAPPERS@','\n'.join(wrappers)).replace(
        '@ARRAYS@','\n'.join(arrays)).replace('@COUNT@',str(case_count(root))).replace('@MARKER@',marker(root,caller).decode())
