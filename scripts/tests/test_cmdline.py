#!/usr/bin/env python3
"""Original-C differential and saved native-command cmdline provider checks.

Run from any directory. --source is the original Linux tree; --root is the
candidate overlay (defaults to this script's tree). unittest discovery uses
CMDLINE_SOURCE, CMDLINE_RUSTC, CMDLINE_ROOT, CMDLINE_NATIVE (os.pathsep-separated),
CMDLINE_SYSROOT32, CMDLINE_KBUILD (0/1), and CMDLINE_LOGS. Relative paths resolve
against ROOT, never the invocation directory. Source defaults to ROOT; rustc is
selected from HOSTRUSTC or PATH. Only an absent optional compiler may skip host
build groups; explicitly empty or invalid inputs always fail validation.
Default workspaces and logs are temporary and cleaned, outside source/native
inputs. Explicit logs retain evidence in unique per-operation run directories.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[2]


def function(text, name):
    """Copy an actual function body, including its original declaration."""
    match = re.search(r"^([A-Za-z_][^\n;{}]+\b" + name + r"\([^;{}]*\)\n\{)", text, re.M)
    if not match:
        raise AssertionError(f"missing original function {name}")
    end = match.end()
    depth = 1
    while depth:
        depth += (text[end] == "{") - (text[end] == "}")
        end += 1
    return text[match.start():end] + "\n"


def run(argv, cwd, log, env=None):
    result = subprocess.run(argv, cwd=cwd, env=env, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    with log.open("a") as out:
        out.write(shlex.join(map(str, argv)) + "\n" + result.stdout)
        out.write(f"exit={result.returncode}\n")
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}); see {log}")
    return result.stdout


def fixture(source):
    read = lambda name: (source / name).read_text()
    # Only includes are removed. Bodies, numeric helpers, table and byte
    # classification macros come directly from the authoritative sources.
    strip = lambda s: re.sub(r"^#include.*\n", "", s, flags=re.M)
    prelude = '''typedef __SIZE_TYPE__ size_t;
typedef _Bool bool;
#define true 1
#define false 0
#define NULL ((void *)0)
#define INT_MAX __INT_MAX__
#define ULLONG_MAX (~0ULL)
#define likely(x) (x)
#define unlikely(x) (x)
#define __must_check_overflow(x) (x)
#define EXPORT_SYMBOL(x)
#define noinline __attribute__((noinline))
#define fallthrough __attribute__((fallthrough))
#define check_mul_overflow(a,b,p) __builtin_mul_overflow(a,b,p)
#define check_add_overflow(a,b,p) __builtin_add_overflow(a,b,p)
#define check_shl_overflow(a,s,p) ((*(p)=(a)<<(s)), (*(p)>>(s))!=(a))
'''
    # Use the real overflow macro: its temporary captures matter for aliasing.
    overflow = read('include/linux/overflow.h')
    start = overflow.index('#define check_shl_overflow(')
    end = overflow.index('\n\n', start)
    prelude = prelude[:prelude.index('#define check_shl_overflow')] + overflow[start:end] + '\n'
    prelude += '#define is_non_negative(a) ((a) > 0 || (a) == 0)\n'
    prelude += strip(read('include/linux/ctype.h'))
    prelude += strip(read('lib/ctype.c'))
    prelude += '#define KSTRTOX_OVERFLOW (1U << 31)\n'
    prelude += '#define _parse_integer(s,b,p,n) _parse_integer_limit(s,b,p,n,0)\n'
    for name in ('_parse_integer_fixup_radix', '_parse_integer_limit'):
        prelude += function(read('lib/kstrtox.c'), name)
    for name in ('simple_strntoull', 'simple_strtoull', 'simple_strtoul', 'simple_strtol'):
        prelude += function(read('lib/vsprintf.c'), name)
    for name in ('strlen', 'strncmp', 'memcpy', 'memset'):
        prelude += function(read('lib/string.c'), name)
    prelude += function(read('lib/string_helpers.c'), 'skip_spaces')
    return prelude, strip(read('lib/cmdline.c'))


HARNESS = r'''
#define DECL(prefix) \
int prefix##get_option(char **s,int *p); \
char *prefix##get_options(const char *s,int n,int *p); \
unsigned long long prefix##memparse(const char *s,char **p); \
bool prefix##parse_option_str(const char *s,const char *p); \
char *prefix##next_arg(char *s,char **p,char **v);
DECL()
DECL(oracle_)
static unsigned random_state = 941;
static unsigned random_byte(void) {
 random_state = random_state * 1664525 + 1013904223; return random_state >> 24;
}
#define CHECK(x) do { if (!(x)) return (__LINE__ % 250)+1; } while (0)
static int same(const void *a,const void *b,size_t n) {
 const unsigned char *p=a,*q=b; for(size_t i=0;i<n;i++) if(p[i]!=q[i]) return 0; return 1;
}
static long offset(char *p,char *base) { return p ? p-base : -1; }
static int one(const char *input, const char *option) {
 char a[256]={0},b[256]={0}; size_t len=strlen(input);
 for(size_t i=0;i<=len;i++) a[i]=b[i]=input[i];
 for(int nullable=0;nullable<2;nullable++) {
  char *p=a,*q=b; int x=0x12345678,y=x;
  CHECK(get_option(&p,nullable?0:&x)==oracle_get_option(&q,nullable?0:&y));
  CHECK(x==y && offset(p,a)==offset(q,b));
  p=q=0;
  CHECK(memparse(a,nullable?0:&p)==oracle_memparse(b,nullable?0:&q));
  CHECK(offset(p,a)==offset(q,b));
 }
 CHECK(parse_option_str(a,option)==oracle_parse_option_str(b,option));
 for(int alias=0;alias<2;alias++) {
  for(size_t i=0;i<sizeof a;i++) a[i]=b[i]= i<=len ? input[i] : 0;
  char *p=0,*q=0,*v=0,*w=0;
  char *r=next_arg(a,&p,alias?&p:&v),*s=oracle_next_arg(b,&q,alias?&q:&w);
  CHECK(offset(r,a)==offset(s,b) && offset(p,a)==offset(q,b) && offset(v,a)==offset(w,b));
  CHECK(same(a,b,sizeof a));
 }
 return 0;
}
static int numbers(const char *s) {
 for(int n=-2;n<=17;n++) {
  int a[32],b[32]; for(int i=0;i<32;i++) a[i]=b[i]=0x5a5a5a5a;
  char *p=get_options(s,n,a),*q=oracle_get_options(s,n,b);
  CHECK(p==q && same(a,b,sizeof a));
 }
 return 0;
}
static int original_corpus(void);
/* Kernel -fno-strict-overflow defines these signed wraps. Huge ranges are
 * count-only: never pass them through numbers()'s small fill arrays. */
static int wrapping_and_nulls(void) {
 const char *ranges[]={"0-2147483647", "0-2147483647,1", "-2147483648-2147483647",
  "2147483647--2147483648", "0-2147483646,1,2", "-1-2147483647", "2147483647-2147483647"};
 for(unsigned j=0;j<sizeof(ranges)/sizeof(*ranges);j++) {
  int a[4]={19,23,29,31},b[4]={19,23,29,31};
  char *p=get_options(ranges[j],0,a),*q=oracle_get_options(ranges[j],0,b);
  CHECK(p==q && same(a,b,sizeof a));
  CHECK(a[1]==23 && a[2]==29 && a[3]==31);
  if(j==0) CHECK(a[0]==(-2147483647-1) && p==ranges[j]+12);
 }
 /* Negative wrapped differences stop before filling: only ints[1] is visited. */
 const char *bounded[]={"-2147483648-0", "-1-2147483647", "0-4294967295"};
 for(unsigned j=0;j<sizeof(bounded)/sizeof(*bounded);j++) {
  int a[3]={11,13,17},b[3]={11,13,17};
  CHECK(get_options(bounded[j],2,a)==oracle_get_options(bounded[j],2,b));
  CHECK(same(a,b,sizeof a) && a[2]==17);
 }
 int a[2]={19,23},b[2]={19,23};
 CHECK(get_options(0,0,a)==oracle_get_options(0,0,b));
 CHECK(a[0]==0 && a[1]==23 && same(a,b,sizeof a));
 a[0]=b[0]=19;
 CHECK(get_options(0,2,a)==oracle_get_options(0,2,b));
 CHECK(a[0]==0 && a[1]==23 && same(a,b,sizeof a));
 CHECK(!parse_option_str("",0) && !oracle_parse_option_str("",0));
 return 0;
}
int test_main(void) {
 int edge_result=wrapping_and_nulls(); if(edge_result)return edge_result;
 int original_result=original_corpus(); if(original_result)return original_result;
 const char *cases[]={"", "-", "+1", "0x", "0xg", "0779", "08", "1,2,3",
 "-2147483648", "2147483648", "4294967295", "18446744073709551615",
 "18446744073709551616", "99999999999999999999999999E", "16E", "15E",
 "1K", "1M", "1G", "1T", "1P", "1E", "1k", "1m", "1g", "1t", "1p", "1e",
 "K", "-K", "0x1E", "a=b", "=a=b", "a=\"b c\" z", "\"a=b c\" z",
 "\"\"", "a=\"", " a=b", "a\240b", "a,b,c", ",,", "x=1,other"};
 for(unsigned i=0;i<sizeof(cases)/sizeof(*cases);i++) {
  int e=one(cases[i],"a"); if(e)return e; e=one(cases[i],""); if(e)return e;
 }
 const char *ranges[]={"", "-", "1,2,3", "1-4,9", "4-1", "-3--1,4", "0-0", "0-20",
 "1-", "1-x", "1-2-4", "-2147483648", "2147483647", "0x1-0x5", "3,"};
 for(unsigned i=0;i<sizeof(ranges)/sizeof(*ranges);i++) {int e=numbers(ranges[i]);if(e)return e;}
 for(unsigned byte=1;byte<256;byte++) {
  char s[]={'a',(char)byte,'b','=',(char)byte,'x',0}; int e=one(s,"a");if(e)return e;
 }
 for(int i=0;i<20000;i++) {
  char s[96]={0},o[8]={0}; unsigned n=random_byte()%90;
  for(unsigned j=0;j<n;j++) s[j]=(char)random_byte();
  for(unsigned j=0;j<7;j++) o[j]=(char)random_byte();
  int e=one(s,o);if(e)return e;
 }
 char *p=0; int x=3; CHECK(get_option(&p,&x)==0 && !p && x==3);
 CHECK(get_options(0,1,&x)==0 && x==0);
 CHECK(get_options(0,-1,&x)==0 && x==0);
 for(int at=0;at<=8;at+=4) {
  union {int align[32];char bytes[128];} a={0},b={0};
  const char *s="123456,7";for(unsigned j=0;j<=strlen(s);j++)a.bytes[j]=b.bytes[j]=s[j];
  char *q=a.bytes,*r=b.bytes;
  CHECK(get_option(&q,(int *)(a.bytes+at))==oracle_get_option(&r,(int *)(b.bytes+at)));
  CHECK(offset(q,a.bytes)==offset(r,b.bytes) && same(&a,&b,sizeof a));
 }
 return 0;
}
#ifdef TEST32
/* Establish the i386 C call alignment before entering the actual harness. */
__asm__(".text\n.globl _start\n_start:\n"
        "andl $-16,%esp\ncall test_main\nmovl %eax,%ebx\n"
        "movl $1,%eax\nint $0x80\nud2\n");
#else
int main(void) { return test_main(); }
#endif
'''


def differential(args, tmp, log, bits):
    prelude, original = fixture(args.source)
    (tmp/'helpers.c').write_text(prelude)
    names = ('get_option', 'get_options', 'memparse', 'parse_option_str', 'next_arg')
    renames = ''.join(f'#define {n} oracle_{n}\n' for n in names)
    (tmp/'oracle.c').write_text(prelude + renames + original)
    # Helpers live once in oracle.o, not a second fabricated provider.
    corpus=(args.source/'lib/tests/cmdline_kunit.c').read_text()
    arrays=corpus[corpus.index('static const char *cmdline_test_strings'):corpus.index('static void cmdline_do_one_test')]
    arrays=re.sub(r'^static_assert.*\n','',arrays,flags=re.M)
    mem=corpus[corpus.index('struct cmdline_test_memparse_entry'):corpus.index('static void cmdline_test_memparse(')]
    sizes=(args.source/'include/linux/sizes.h').read_text()
    sizes=re.sub(r'^#include.*\n','',sizes,flags=re.M)
    corpus_checks=r'''
static int original_corpus(void) {
 for(unsigned i=0;i<sizeof(cmdline_test_strings)/sizeof(*cmdline_test_strings);i++) {
  const char *s=cmdline_test_strings[i]; char *p=(char *)s; int value=9;
  CHECK(get_option(&p,&value)==0 && p==s+(*s=='-'));
  for(unsigned n=0;n<256;n++) {
   char digits[4],input[64]={0}; unsigned length=0;
   if(n>=100)digits[length++]=(char)('0'+n/100);
   if(n>=10)digits[length++]=(char)('0'+(n/10)%10);
   digits[length++]=(char)('0'+n%10);
   for(unsigned j=0;j<length;j++)input[j]=digits[j];
   for(unsigned j=0;j<=strlen(s);j++)input[length+j]=s[j];
   p=input; CHECK(get_option(&p,&value)==cmdline_test_values[i]);
   CHECK(p==input+length+(cmdline_test_values[i]==2));
   int e=one(input,""); if(e)return e;
   for(unsigned j=0;j<strlen(s);j++)input[j]=s[j];
   for(unsigned j=0;j<length;j++)input[strlen(s)+j]=digits[j];
   input[strlen(s)+length]=0;
   p=input; int rc=!*s || (s[0]=='-' && !s[1]);
   CHECK(get_option(&p,&value)==rc);
   CHECK(p==input+(rc?strlen(input):(*s=='-')));
   e=one(input,"");if(e)return e;
  }
 }
 for(unsigned i=0;i<sizeof(cmdline_test_range_strings)/sizeof(*cmdline_test_range_strings);i++) {
  const char *s=cmdline_test_range_strings[i]; int values[16]={0};
  get_options(s,16,values); CHECK(same(values,cmdline_test_range_values[i],sizeof values));
  for(unsigned j=0;j<16;j++)values[j]=0;
  get_options(s,0,values); CHECK(values[0]==cmdline_test_range_values[i][0]);
  for(unsigned j=1;j<16;j++)CHECK(values[j]==0);
  int e=numbers(s);if(e)return e;
 }
 for(unsigned i=0;i<sizeof(testdata)/sizeof(*testdata);i++) {
  char *p=0; CHECK(memparse(testdata[i].input,&p)==testdata[i].result);
  CHECK(*p==*testdata[i].unrecognized);
  int e=one(testdata[i].input,"");if(e)return e;
 }
 const char *tokens[]={"foo=\"bar baz\" qux=1", "foo=bar \"", "bbb= jjj kkk=\"a=b\""};
 for(unsigned i=0;i<3;i++) {
  char a[64]={0},b[64]={0}; for(unsigned j=0;j<=strlen(tokens[i]);j++)a[j]=b[j]=tokens[i][j];
  char *x=a,*y=b;
  for(unsigned step=0;step<4;step++) {
   char *p,*q,*v,*w; x=next_arg(x,&p,&v);y=oracle_next_arg(y,&q,&w);
   CHECK(offset(x,a)==offset(y,b) && offset(p,a)==offset(q,b) && offset(v,a)==offset(w,b));
   CHECK(same(a,b,sizeof a)); if(!*x)break;
  }
 }
 return 0;
}
'''
    (tmp/'harness.c').write_text('typedef __SIZE_TYPE__ size_t; typedef _Bool bool;\n'
                                'size_t strlen(const char *);\n#define ULLONG_MAX (~0ULL)\n'
                                '#define _AC(x,suffix) x##suffix\n'+sizes+arrays+mem+HARNESS+corpus_checks)
    (tmp/'provider.rs').write_text('#![no_std]\n#[path = '+repr(str(args.root/'lib/cmdline.rs')).replace("'",'"')+' ]\nmod implementation;\n')
    rust = [str(args.rustc), '--edition=2021', '--crate-type=lib', '--emit=obj',
            '-Cpanic=abort', '-Cdebug-assertions=n', '-Coverflow-checks=y', '-Dwarnings', '-Dunsafe_op_in_unsafe_fn']
    cc = ['clang', f'-m{bits}', '-std=gnu11', '-funsigned-char', '-fno-strict-overflow', '-fno-builtin', '-fno-stack-protector']
    if bits == 32:
        rust += ['--target=i686-unknown-linux-gnu', '--sysroot='+str(args.sysroot32)]
        cc += ['-ffreestanding', '-DTEST32']
    failures=[]
    for opt in ('0','2','s'):
        run(rust + ['-Copt-level='+opt, str(tmp/'provider.rs'), '-o', str(tmp/'rust.o')], tmp, log)
        for unit in ('oracle', 'harness'):
            run(cc+['-O'+opt,'-c',str(tmp/(unit+'.c')),'-o',str(tmp/(unit+'.o'))], tmp, log)
        link = ['clang', f'-m{bits}']
        if bits == 32:
            link += ['-nostdlib', '-static', '-fuse-ld=lld', '-Wl,-e,_start']
        run(link + [str(tmp/'rust.o'), str(tmp/'oracle.o'), str(tmp/'harness.o'), '-o', str(tmp/'test')], tmp, log)
        try:
            run([str(tmp/'test')], tmp, log)
            print(f'PASS original C {bits}-bit O{opt}', flush=True)
        except RuntimeError as error:
            (args.logs/f'failed-{bits}-O{opt}.elf').write_bytes((tmp/'test').read_bytes())
            (args.logs/f'failed-{bits}-O{opt}.elf').chmod(0o755)
            failures.append(str(error))
            print(f'FAIL original C {bits}-bit O{opt}: {error}',flush=True)
    if failures:
        raise RuntimeError('; '.join(failures))
    # A wrong suffix shift must be detected by the actual differential harness.
    mutant=(args.root/'lib/cmdline.rs').read_text().replace("b'M' | b'm' => 20", "b'M' | b'm' => 10")
    assert mutant != (args.root/'lib/cmdline.rs').read_text()
    (tmp/'mutant.rs').write_text('#![no_std]\n'+mutant)
    run(rust+['-Copt-level=s',str(tmp/'mutant.rs'),'-o',str(tmp/'mutant.o')],tmp,log)
    run(link+[str(tmp/'mutant.o'),str(tmp/'oracle.o'),str(tmp/'harness.o'),'-o',str(tmp/'negative')],tmp,log)
    result=subprocess.run([str(tmp/'negative')])
    with log.open('a') as out: out.write(f'negative suffix control: exit={result.returncode}\n')
    assert result.returncode > 0, 'semantic negative control was not detected'
    print(f'PASS {bits}-bit semantic negative control',flush=True)
    # This mutation used to escape the entire shipped differential corpus.
    source=(args.root/'lib/cmdline.rs').read_text()
    before='i = i.wrapping_add(1);\n            if res'
    assert source.count(before) == 1
    mutant=source.replace(before, 'i = i.saturating_add(1);\n            if res')
    (tmp/'wrapping_mutant.rs').write_text('#![no_std]\n'+mutant)
    run(rust+['-Copt-level=s',str(tmp/'wrapping_mutant.rs'),'-o',str(tmp/'wrapping_mutant.o')],tmp,log)
    run(link+[str(tmp/'wrapping_mutant.o'),str(tmp/'oracle.o'),str(tmp/'harness.o'),'-o',str(tmp/'wrapping_negative')],tmp,log)
    result=subprocess.run([str(tmp/'wrapping_negative')])
    with log.open('a') as out: out.write(f'negative saturating-count control: exit={result.returncode}\n')
    assert result.returncode > 0, 'wrapping negative control was not detected'
    print(f'PASS {bits}-bit wrapping-specific saturating mutant rejected',flush=True)


def native(args, native, tmp, log):
    donor = native/'lib/.hexdump_rust.o.cmd'
    line = donor.read_text().splitlines()[0].split(' := ',1)[1]
    argv = shlex.split(line)
    env = os.environ.copy(); env['RUSTC_BOOTSTRAP']='1'
    while '=' in argv[0] and not argv[0].startswith('-'):
        k,v=argv.pop(0).split('=',1); env[k]=v
    env['RUST_MODFILE']='lib/cmdline'
    argv[0]=str(args.rustc)
    converted=[]
    for arg in argv:
        if arg.startswith('--emit=') or arg == str(args.source/'lib/hexdump_rust.rs'):
            continue
        if arg == 'lib/': arg=str(tmp)
        converted.append(arg)
    converted += ['--emit=obj='+str(tmp/'cmdline_rust.o'), '--emit=llvm-ir='+str(tmp/'cmdline_rust.ll'),
                  '--emit=dep-info='+str(tmp/'cmdline_rust.d'),str(args.root/'lib/cmdline_rust.rs'), '-Dwarnings']
    run(converted,native,log,env)
    nm=run(['llvm-nm',str(tmp/'cmdline_rust.o')],tmp,log)
    for name in ('get_option','get_options','memparse','parse_option_str','next_arg'):
        assert re.search(r' T '+name+r'$',nm,re.M), name
    for name in ('get_option','get_options','memparse','next_arg'):
        assert '__export_symbol_'+name in nm, name
    assert '__export_symbol_parse_option_str' not in nm
    assert not re.search(r' U .*?(panic|alloc|unwind|stack_chk)',nm)
    ir=(tmp/'cmdline_rust.ll').read_text()
    assert '!kcfi_type' in ir
    # Genuine generated binding types must agree at compile time.
    bindings='''//! Genuine generated parser binding signature checks.
use kernel::{bindings, ffi::{c_char,c_int,c_ulonglong}};
const _: unsafe extern "C" fn(*mut *mut c_char,*mut c_int)->c_int = bindings::get_option;
const _: unsafe extern "C" fn(*const c_char,c_int,*mut c_int)->*mut c_char = bindings::get_options;
const _: unsafe extern "C" fn(*const c_char,*mut *mut c_char)->c_ulonglong = bindings::memparse;
const _: unsafe extern "C" fn(*const c_char,*const c_char)->bool = bindings::parse_option_str;
const _: unsafe extern "C" fn(*mut c_char,*mut *mut c_char,*mut *mut c_char)->*mut c_char = bindings::next_arg;
'''
    (tmp/'binding_check.rs').write_text(bindings)
    check=[a for a in converted if not a.startswith('--emit=') and a!=str(args.root/'lib/cmdline_rust.rs')]
    run(check+['--emit=obj='+str(tmp/'binding_check.o'),str(tmp/'binding_check.rs')],native,log,env)
    # A wrong C pointee type must fail against the genuine generated binding.
    (tmp/'binding_bad.rs').write_text(bindings.replace('fn(*mut *mut c_char,*mut c_int)', 'fn(*mut *mut u16,*mut c_int)'))
    bad=check+['--emit=obj='+str(tmp/'binding_bad.o'),str(tmp/'binding_bad.rs')]
    result=subprocess.run(bad,cwd=native,env=env,text=True,capture_output=True)
    with log.open('a') as out: out.write(shlex.join(bad)+'\n'+result.stdout+result.stderr)
    assert result.returncode != 0 and 'mismatched types' in result.stderr
    # Replay ALWAYS-C donor with all flags, original source and original owner.
    c=shlex.split((native/'lib/.scatterlist.o.cmd').read_text().splitlines()[0].split(' := ',1)[1])
    c=[a.replace('scatterlist','cmdline') for a in c]
    c=[a for a in c if not a.startswith('-Wp,-MMD,')]
    c[c.index('-o')+1]=str(tmp/'cmdline_c.o')
    run(c,native,log)
    c[c.index('-o')+1]=str(tmp/'cmdline_c.ll')
    run(c+['-S','-emit-llvm'],native,log)
    cir=(tmp/'cmdline_c.ll').read_text()
    def kcfi(text,name):
        line=next(l for l in text.splitlines() if l.startswith('define ') and '@'+name+'(' in l)
        key=re.search(r'!kcfi_type !(\d+)',line).group(1)
        return re.search(r'^!'+key+r' = !\{i32 (-?\d+)\}',text,re.M).group(1)
    for name in ('get_option','get_options','memparse','parse_option_str','next_arg'):
        assert kcfi(ir,name)==kcfi(cir,name), name
    run(['llvm-dwarfdump','--debug-info',str(tmp/'cmdline_rust.o')],tmp,log)
    verify=subprocess.run(['llvm-dwarfdump','--verify',str(tmp/'cmdline_rust.o')],
                          text=True,capture_output=True)
    with log.open('a') as out:
        out.write('llvm-dwarfdump --verify\n'+verify.stdout+verify.stderr+f'exit={verify.returncode}\n')
    print(f'Generic DWARF verifier: {"FAIL" if verify.returncode else "PASS"} (exit={verify.returncode}); separate from gendwarfksyms',flush=True)
    # The kernel consumer is authoritative for symbol-version extraction.
    dwarf = native/'scripts/gendwarfksyms/gendwarfksyms'
    result = subprocess.run([str(dwarf), str(tmp/'cmdline_rust.o')],
                            input='get_option\nget_options\nmemparse\nnext_arg\n',
                            text=True, capture_output=True)
    with log.open('a') as out:
        out.write('gendwarfksyms '+str(tmp/'cmdline_rust.o')+'\n'+result.stdout+result.stderr)
    assert result.returncode == 0 and result.stdout.count('#SYMVER ') == 4
    for artifact in tmp.glob('cmdline*'):
        if artifact.is_file():
            (args.logs/(native.name+'-'+artifact.name)).write_bytes(artifact.read_bytes())
    print('PASS saved native flags, bindings, exports, KCFI and real gendwarfksyms: '+str(native),flush=True)


def kbuild(args, native, tmp, log):
    """Run real Makefile.build in a private, component-scoped output tree."""
    src=tmp/'kbuild-src'; out=tmp/'kbuild-out'
    for tree in (src,out): (tree/'lib').mkdir(parents=True)
    for rel in ('lib/cmdline.rs','lib/cmdline_rust.rs','rust/ffi_export.rs','include/linux/export_header.rs'):
        dest=src/rel; dest.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(args.root/rel,dest)
    shutil.copyfile(args.source/'lib/cmdline.c',src/'lib/cmdline.c')
    # Config and compiler libraries are read only. All target files live in out/lib.
    for name in ('include','arch','scripts','rust'):
        (out/name).symlink_to(native/name,target_is_directory=True)
    makefile=(args.root/'lib/Makefile').read_text()
    makefile+='''
$(info ORIGINAL_SLOT=$(lib-y))
# Scope this fixture to the selected component after observing the full list.
lib-y := $(filter cmdline.o cmdline_rust.o,$(lib-y))
obj-y :=
obj-m :=
always-y :=
'''
    (src/'lib/Makefile').write_text(makefile)
    c=shlex.split((native/'lib/.scatterlist.o.cmd').read_text().splitlines()[0].split(' := ',1)[1])
    cflags=[]; skip=False
    for a in c[1:]:
        if skip: skip=False; continue
        if a in ('-o','-c'):
            if a=='-o': skip=True
            continue
        if a.startswith(('-Wp,-MMD,','-DKBUILD_','-D__KBUILD_')) or a.endswith('scatterlist.c'): continue
        cflags.append(a)
    r=shlex.split((native/'lib/.hexdump_rust.o.cmd').read_text().splitlines()[0].split(' := ',1)[1])
    rflags=r[r.index('--edition=2021'):next(i for i,a in enumerate(r) if a.startswith('@'))]+['-Dwarnings']
    wrapper=tmp/'kbuild.mk'
    wrapper.write_text('''srctree := '''+str(args.source)+'''
srcroot := '''+str(src)+'''
VPATH := $(srcroot)
objtree := .
obj := lib
CC := clang
LD := ld.lld
AR := llvm-ar
NM := llvm-nm
OBJCOPY := llvm-objcopy
AWK := awk
CONFIG_SHELL := /bin/sh
RUSTC := '''+str(args.rustc)+'''
RUSTC_OR_CLIPPY := $(RUSTC)
KBUILD_CFLAGS := '''+shlex.join(cflags)+'''
KBUILD_RUSTFLAGS := '''+shlex.join(rflags)+'''
rust_allowed_features := arbitrary_self_types,asm_goto,generic_arg_infer,used_with_arg
include $(srctree)/scripts/Makefile.build
''')
    env=os.environ.copy();env['RUSTC_BOOTSTRAP']='1'
    def build(selection):
        result=run(['make','-rR','-f',str(wrapper),'CONFIG_RUST_CMDLINE='+selection,'lib/lib.a','V=1'],out,log,env)
        line=next(l for l in result.splitlines() if l.startswith('ORIGINAL_SLOT='))
        members=line.split('=',1)[1].split()
        selected='cmdline_rust.o' if selection=='y' else 'cmdline.o'
        assert members[members.index(selected)-1]=='vsprintf.o'
        assert members[members.index(selected)+1]=='rbtree.o'
        archive=run(['llvm-ar','t','lib/lib.a'],out,log)
        assert archive.strip().endswith(selected) and len(archive.splitlines())==1
        return out/'lib'/selected
    cobj=build('n'); initial=cobj.stat().st_mtime_ns
    c_initial=initial
    build('n'); assert initial==cobj.stat().st_mtime_ns
    robj=build('y'); initial=robj.stat().st_mtime_ns
    build('y'); assert initial==robj.stat().st_mtime_ns
    command=(out/'lib/.cmdline_rust.o.cmd').read_text()
    assert 'RUST_MODFILE=lib/cmdline ' in command
    assert str(src/'lib/cmdline.rs') in command
    # Touch only the private implementation; real dep-info must rebuild it.
    import time
    time.sleep(0.02)
    (src/'lib/cmdline.rs').touch()
    build('y'); assert initial!=robj.stat().st_mtime_ns
    initial=robj.stat().st_mtime_ns
    build('y'); assert initial==robj.stat().st_mtime_ns
    build('n'); assert cobj.stat().st_mtime_ns==c_initial
    for artifact in (out/'lib').glob('.*.cmd'):
        shutil.copyfile(artifact,args.logs/(native.name+'-kbuild-'+artifact.name))
    print('PASS real private Kbuild C/Rust switching, dependencies, no-ops, owner and original slot',flush=True)


CLI_ARGS = None
ENV_KEYS = ('source', 'root', 'rustc', 'native', 'sysroot32', 'kbuild', 'logs')


def configuration(environ=None):
    """Validate explicit inputs before deciding which optional gates can skip."""
    use_cli = environ is None
    environ = os.environ if environ is None else environ
    unknown = sorted(k for k in environ if k.startswith('CMDLINE_')
                     and k not in {'CMDLINE_'+key.upper() for key in ENV_KEYS})
    if unknown:
        raise ValueError('unknown CMDLINE input: '+', '.join(unknown))
    values = {key: environ['CMDLINE_'+key.upper()] for key in ENV_KEYS
              if 'CMDLINE_'+key.upper() in environ}
    if use_cli and CLI_ARGS is not None:
        values.update(vars(CLI_ARGS))
    for key, value in values.items():
        if value == '' or (isinstance(value, list) and any(v == '' for v in value)):
            raise ValueError('explicit '+key+' input must not be empty')
    def path(value):
        value = Path(value)
        return (ROOT/value).resolve() if not value.is_absolute() else value.resolve()
    native_value = values.get('native', [])
    if isinstance(native_value, str):
        native_value = native_value.split(os.pathsep)
    if any(not n for n in native_value):
        raise ValueError('empty native input')
    kbuild_value = values.get('kbuild', '0')
    if kbuild_value not in ('0', '1', False, True):
        raise ValueError('CMDLINE_KBUILD must be 0 or 1')
    args = SimpleNamespace(
        source=path(values.get('source', ROOT)),
        root=path(values.get('root', ROOT)),
        rustc=None,
        native=[path(n) for n in native_value],
        sysroot32=path(values['sysroot32']) if 'sysroot32' in values else None,
        kbuild=kbuild_value in ('1', True),
        logs=path(values['logs']) if 'logs' in values else None)
    def require(condition, message):
        if not condition:
            raise ValueError(message)
    for rel in ('lib/cmdline.rs', 'lib/cmdline_rust.rs', 'lib/Makefile'):
        require((args.root/rel).is_file(), 'invalid root: missing '+rel)
    if args.source is not None:
        for rel in ('lib/cmdline.c', 'lib/tests/cmdline_kunit.c', 'lib/vsprintf.c',
                    'lib/kstrtox.c', 'lib/string.c', 'lib/ctype.c',
                    'lib/string_helpers.c', 'include/linux/ctype.h',
                    'include/linux/overflow.h', 'include/linux/sizes.h',
                    'scripts/Makefile.build'):
            require((args.source/rel).is_file(), 'invalid source: missing '+rel)
    compiler = values.get('rustc', environ.get('HOSTRUSTC'))
    explicit_compiler = 'rustc' in values or 'HOSTRUSTC' in environ
    if explicit_compiler:
        require(bool(compiler), 'explicit rustc input must not be empty')
    if compiler is None:
        compiler = shutil.which('rustc', path=environ.get('PATH', os.defpath))
    elif '/' not in compiler:
        compiler = shutil.which(compiler, path=environ.get('PATH', os.defpath))
        require(compiler is not None, 'invalid rustc tool')
    if compiler is not None:
        # Preserve the executable name: rustup dispatches on argv[0].
        args.rustc = Path(os.path.abspath(ROOT/compiler))
        require(args.rustc.is_file() and os.access(args.rustc, os.X_OK), 'invalid rustc tool')
        version = subprocess.run([str(args.rustc), '--version'], text=True, capture_output=True)
        match = re.match(r'rustc (\d+)\.(\d+)\.(\d+)', version.stdout)
        require(version.returncode == 0 and match is not None, 'invalid rustc --version')
        require(tuple(map(int, match.groups())) >= (1, 85, 0), 'rustc must be at least 1.85.0')
    if args.sysroot32 is not None:
        require(bool(list((args.sysroot32/'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib'))),
                'invalid 32-bit sysroot')
    for n in args.native:
        for rel in ('lib/.scatterlist.o.cmd', 'lib/.hexdump_rust.o.cmd',
                    'scripts/gendwarfksyms/gendwarfksyms', 'include/generated/rustc_cfg'):
            require((n/rel).is_file(), 'invalid native input: missing '+rel)
    require(not args.kbuild or bool(args.native), 'kbuild requires explicit native input')
    protected = [p for p in [args.root, args.source, *args.native, args.sysroot32]
                 if p is not None]
    def outside_inputs(directory):
        return not any(directory.is_relative_to(p) for p in protected)
    if args.logs is not None:
        for parent in [args.logs, *args.logs.parents]:
            require(not parent.exists() or parent.is_dir(), 'invalid logs directory')
        require(outside_inputs(args.logs), 'logs must not write into input trees')
        ancestor = next(p for p in [args.logs, *args.logs.parents] if p.exists())
        require(os.access(ancestor, os.W_OK | os.X_OK), 'logs directory is not writable')
    # Avoid gettempdir(): even its writability probe can modify a source tree
    # when TMPDIR points there. Select a safe parent without creating anything.
    candidates = [environ.get(key) for key in ('TMPDIR', 'TEMP', 'TMP')]+['/tmp']
    args.temp_parent = next((p for value in candidates if value
                             for p in [Path(value).resolve()]
                             if outside_inputs(p) and p.is_dir()
                             and os.access(p, os.W_OK | os.X_OK)), None)
    require(args.temp_parent is not None, 'no temporary directory outside input trees')
    return args


class InputValidationTests(unittest.TestCase):
    def test_selected_inputs(self):
        selected = configuration()
        runner = BuildTestCase()
        runner.args = SimpleNamespace(**vars(selected))
        seen = []
        def probe(args, tmp, log):
            seen.append((tmp, log))
            log.write_text('retained failure evidence')
            raise RuntimeError('workspace cleanup probe')
        runner.args.logs = None
        with self.assertRaisesRegex(RuntimeError, 'workspace cleanup probe'):
            runner.execute(probe)
        self.assertFalse(seen[-1][0].exists())
        self.assertFalse(seen[-1][1].exists())
        with tempfile.TemporaryDirectory(prefix='cmdline-validation-',
                                         dir=selected.temp_parent) as directory:
            runner.args.logs = Path(directory)/'evidence'
            for _ in range(2):
                with self.assertRaisesRegex(RuntimeError, 'workspace cleanup probe'):
                    runner.execute(probe)
                self.assertFalse(seen[-1][0].exists())
            self.assertNotEqual(seen[-1][1], seen[-2][1])
            for _, log in seen[-2:]:
                self.assertEqual(log.read_text(), 'retained failure evidence')

    def test_empty_inputs_rejected(self):
        for key in ENV_KEYS:
            with self.subTest(key=key), self.assertRaises(ValueError):
                configuration({'CMDLINE_'+key.upper(): ''})

    def test_invalid_inputs_rejected(self):
        for key in ('SOURCE', 'ROOT', 'RUSTC', 'NATIVE', 'SYSROOT32'):
            with self.subTest(key=key), self.assertRaises(ValueError):
                configuration({'CMDLINE_'+key: str(ROOT/'missing-cmdline-input')})
        for env in ({'CMDLINE_KBUILD': 'yes'}, {'CMDLINE_KBUILD': '1'},
                    {'CMDLINE_NATIVE': os.pathsep}, {'CMDLINE_TOOL': 'missing'},
                    {'CMDLINE_LOGS': str(ROOT/'logs')},
                    {'CMDLINE_LOGS': str(Path(__file__)/'child')},
                    {'HOSTRUSTC': ''}, {'HOSTRUSTC': '/missing/cmdline-rustc'}):
            with self.subTest(env=env), self.assertRaises(ValueError):
                configuration(env)


class BuildTestCase(unittest.TestCase):
    def setUp(self):
        self.args = configuration()
        if self.args.rustc is None:
            self.skipTest('optional rustc absent from PATH; supply HOSTRUSTC or CMDLINE_RUSTC')

    def execute(self, operation, *inputs):
        requested_logs = self.args.logs
        with tempfile.TemporaryDirectory(prefix='cmdline-', dir=self.args.temp_parent) as directory:
            tmp = Path(directory)
            if requested_logs is None:
                logs = tmp/'logs'
                logs.mkdir()
            else:
                requested_logs.mkdir(parents=True, exist_ok=True)
                logs = Path(tempfile.mkdtemp(prefix=self._testMethodName+'-', dir=requested_logs))
                print('cmdline retained evidence: '+str(logs), flush=True)
            self.args.logs = logs
            try:
                operation(self.args, *inputs, tmp, logs/'test_cmdline.log')
            finally:
                self.args.logs = requested_logs


class DifferentialTests(BuildTestCase):
    def test_original_c_64(self):
        self.execute(lambda a, t, l: differential(a, t, l, 64))

    def test_original_c_32(self):
        if self.args.sysroot32 is None:
            self.skipTest('no CMDLINE_SYSROOT32 / --sysroot32 supplied')
        self.execute(lambda a, t, l: differential(a, t, l, 32))


class NativeTests(BuildTestCase):
    def test_bindings_exports_kcfi_dwarf(self):
        if not self.args.native:
            self.skipTest('no CMDLINE_NATIVE / --native supplied')
        for n in self.args.native:
            with self.subTest(native=str(n)):
                self.execute(native, n)


class KbuildTests(BuildTestCase):
    def test_selection_dependencies_owner(self):
        if not self.args.kbuild:
            self.skipTest('CMDLINE_KBUILD=1 / --kbuild not supplied')
        for n in self.args.native:
            with self.subTest(native=str(n)):
                self.execute(kbuild, n)


def main():
    global CLI_ARGS
    p=argparse.ArgumentParser(description=__doc__)
    for key in ('source', 'root', 'rustc', 'sysroot32', 'logs'):
        p.add_argument('--'+key, default=argparse.SUPPRESS)
    p.add_argument('--native', action='append', default=argparse.SUPPRESS)
    p.add_argument('--kbuild', action='store_true', default=argparse.SUPPRESS,
                   help='run scoped real Kbuild for every native input')
    CLI_ARGS=p.parse_args()
    unittest.main(argv=[__file__], verbosity=2)


if __name__=='__main__':
    main()
