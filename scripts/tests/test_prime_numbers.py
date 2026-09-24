# SPDX-License-Identifier: GPL-2.0-only
"""Full original prime cache under deterministic host boundary transport.

The actual provider/sieve/headers and genuine bindgen C layouts execute at both
word widths. Allocator, mutex scheduling and RCU grace periods are deliberately
instrumented host transport, NOT proof of native kernel synchronization. Actual
kernel KUnit, fault injection, competing threads and unload remain separate VM
gates. No test substitutes trial division for the cache implementation.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

from rust_exports_test_support import read_exports, rust_targets
from test_ctype_translation import run
from test_int_math_translation import rust_flags
from test_kunit_parameters import panic_handler
from test_rational_build import environment, headers, module_info


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "lib/math/prime_numbers.rs"
PUBLIC = ROOT / "include/linux/prime_numbers_header.rs"
PRIVATE = ROOT / "lib/math/prime_numbers_private_header.rs"


def primes_callback_definition():
    """Reuse the production blocklisted typedef with the generated C layout."""
    parameters = shlex.split((ROOT / "rust/bindgen_parameters").read_text(), comments=True)
    if parameters.count("primes_fn") != 1 or parameters[parameters.index("primes_fn") - 1] != "--blocklist-type":
        raise AssertionError("production primes_fn blocklist changed")
    source = (ROOT / "rust/bindings/lib.rs").read_text()
    definitions = re.findall(r'#\[cfg\(CONFIG_PRIME_NUMBERS_KUNIT_TEST\)\]\s*'
                             r'pub type primes_fn = unsafe extern "C" fn\(\*mut ffi::c_void, \*const primes\);', source)
    if len(definitions) != 1:
        raise AssertionError("production non-null callback definition changed")
    return "\nuse crate::ffi;\n" + definitions[0] + "\n"


def declarations():
    """Extract actual C storage definitions, not hand-written struct layouts."""
    types = (ROOT / "include/linux/types.h").read_text()
    text = """#ifndef PRIME_TEST_TYPES
#define PRIME_TEST_TYPES
typedef _Bool bool;
typedef __SIZE_TYPE__ size_t;
typedef unsigned int gfp_t;
typedef long long s64;
#define true 1
#define false 0
#define NULL ((void *)0)
#define __aligned(n) __attribute__((aligned(n)))
#define __force
#define __bitwise
#define __guarded_by(x)
#define context_lock_struct(name) struct name
#define rcu_head callback_head
"""
    for pattern in (r"typedef struct \{\s*int __aligned\(sizeof\(int\)\) counter;\s*\} atomic_t;",
                    r"typedef struct \{\s*s64 counter;\s*\} atomic64_t;",
                    r"struct callback_head \{.*?\} __attribute__\(\(aligned\(sizeof\(void \*\)\)\)\);",
                    r"#ifdef CONFIG_KVFREE_RCU_BATCHED\nstruct kvfree_rcu_head \{.*?#endif"):
        text += re.search(pattern, types, re.S)[0] + "\n"
    text += "#define rcu_head callback_head\n"
    atomic = (ROOT / "include/linux/atomic/atomic-long.h").read_text()
    text += atomic[atomic.index("#ifdef CONFIG_64BIT"):atomic.index("/**")] + "\n"
    up = (ROOT / "include/linux/spinlock_types_up.h").read_text()
    text += up[up.index("#ifdef CONFIG_DEBUG_SPINLOCK"):up.index("typedef struct {\n\t/* no debug")]
    raw = (ROOT / "include/linux/spinlock_types_raw.h").read_text()
    text += re.search(r"context_lock_struct\(raw_spinlock\) \{.*?^\};", raw, re.S | re.M)[0]
    text += "\ntypedef struct raw_spinlock raw_spinlock_t;\n"
    mutex = (ROOT / "include/linux/mutex_types.h").read_text()
    text += re.search(r"context_lock_struct\(mutex\) \{.*?^\};", mutex, re.S | re.M)[0]
    text += "\n#endif\n"
    return text


FACADE = r'''
//! Explicit deterministic host transport, not a native-lock implementation.
#![no_std]
#![allow(missing_docs,dead_code,unreachable_pub)]
#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unsafe_op_in_unsafe_fn,improper_ctypes)]
#[path="bindings.rs"] pub mod bindings;
#[allow(unused_attributes)]
#[path=@FFI@] pub mod ffi;
pub mod types {
    use core::{cell::UnsafeCell,marker::{PhantomData,PhantomPinned},mem::MaybeUninit};
    @OPAQUE@
    impl<T> Opaque<T> { @NEW@ @GET@ }
    pub type NotThreadSafe=PhantomData<*mut ()>;
    #[allow(non_upper_case_globals)] pub const NotThreadSafe:NotThreadSafe=PhantomData;
}
pub mod alloc {
    use core::{alloc::Layout,ptr::NonNull};
    #[derive(Clone,Copy)] pub struct Flags(pub u32);
    impl core::ops::BitOr for Flags {type Output=Self;fn bitor(self,rhs:Self)->Self{Self(self.0|rhs.0)}}
    impl core::ops::BitAnd for Flags {type Output=Self;fn bitand(self,rhs:Self)->Self{Self(self.0&rhs.0)}}
    pub mod flags {use super::Flags;pub const GFP_KERNEL:Flags=Flags(crate::bindings::RUST_GFP_KERNEL);pub const __GFP_NOWARN:Flags=Flags(crate::bindings::RUST_GFP_NOWARN);}
    pub struct NumaNode;
    impl NumaNode {pub const NO_NODE:Self=Self;}
    pub trait Allocator {
        fn alloc(layout:Layout,flags:Flags,_node:NumaNode)->Result<NonNull<[u8]>,()>;
        unsafe fn free(pointer:NonNull<u8>,layout:Layout);
    }
    pub mod allocator {
        use super::*;
        pub struct Kmalloc;
        impl Allocator for Kmalloc {
            fn alloc(layout:Layout,flags:Flags,_node:NumaNode)->Result<NonNull<[u8]>,()> {
                let pointer=unsafe{crate::bindings::host_alloc(layout.size(),flags.0)};
                NonNull::new(pointer.cast()).map(|p|NonNull::slice_from_raw_parts(p,layout.size())).ok_or(())
            }
            unsafe fn free(pointer:NonNull<u8>,_layout:Layout){unsafe{crate::bindings::host_free(pointer.as_ptr().cast())}}
        }
    }
}
pub mod sync {
    use core::{marker::PhantomData,ops::Deref};
    use crate::{bindings,types::Opaque};
    pub mod atomic {
        pub use core::sync::atomic::Ordering::{Acquire,Release};
        pub struct Atomic<T>{value:core::sync::atomic::AtomicPtr<()>,kind:core::marker::PhantomData<fn()->T>}
        impl<T> Atomic<*const T> {
            pub const fn new(p:*const T)->Self {Self{value:core::sync::atomic::AtomicPtr::new(p.cast_mut().cast()),kind:core::marker::PhantomData}}
            pub fn load(&self,order:core::sync::atomic::Ordering)->*const T{self.value.load(order).cast()}
            pub fn store(&self,p:*const T,order:core::sync::atomic::Ordering){self.value.store(p.cast_mut().cast(),order)}
        }
    }
    // This private wrapper preserves actual mutex storage. Only lock scheduling
    // is transported to the driver; native kernel Mutex correctness is NOT tested.
    #[repr(transparent)] pub struct Mutex<T>{raw:Opaque<bindings::mutex>,kind:PhantomData<T>}
    pub struct MutexGuard<'a,T>{lock:&'a Mutex<T>}
    impl Mutex<()> {
        pub unsafe fn from_raw<'a>(p:*mut bindings::mutex)->&'a Self{unsafe{&*p.cast()}}
        pub fn lock(&self)->MutexGuard<'_,()>{unsafe{bindings::host_lock(self.raw.get())};MutexGuard{lock:self}}
    }
    impl<T> Drop for MutexGuard<'_,T>{fn drop(&mut self){unsafe{bindings::host_unlock(self.lock.raw.get())}}}
    impl Deref for MutexGuard<'_,()>{type Target=();fn deref(&self)->&(){&()}}
    #[path=@RCU@] pub mod rcu;
}
'''


DRIVER = r'''
#include "fixture.h"
typedef unsigned long long u64;
void *memcpy(void *d,const void *s,size_t n){char *a=d;const char *b=s;while(n--)*a++=*b++;return d;}
void *memset(void *d,int c,size_t n){unsigned char *a=d;while(n--)*a++=c;return d;}
void *memmove(void *d,const void *s,size_t n){char *a=d;const char*b=s;if(a<b)return memcpy(d,s,n);while(n)a[n-1]=b[n-1],n--;return d;}
__attribute__((noreturn)) static void finish(unsigned status){
#if __SIZEOF_POINTER__==8
 __asm__ volatile("syscall"::"a"(60),"D"(status):"rcx","r11","memory");
#else
 __asm__ volatile("int $0x80"::"a"(1),"b"(status):"memory");
#endif
 __builtin_unreachable();
}
static void output(const void *p,size_t n){
#if __SIZEOF_POINTER__==8
 long result;__asm__ volatile("syscall":"=a"(result):"a"(1),"D"(1),"S"(p),"d"(n):"rcx","r11","memory");
#else
 long result;__asm__ volatile("int $0x80":"=a"(result):"a"(4),"b"(1),"c"(p),"d"(n):"memory");
#endif
 if(result!=(long)n)finish(80);
}
#define REQUIRE(x) do{if(!(x))finish(__LINE__%160+40);}while(0)
static union {unsigned long align;unsigned char data[8*1024*1024];} arena;
static size_t used;
static unsigned readers,current_readers,locked,allocations,immediate,retired,reclaimed,fail_alloc;
static unsigned long competing;
static void *pending[128];static unsigned pending_count;
static unsigned long sqrt_calls;
static size_t last_size;
void BUG(void){finish(201);}
unsigned long int_sqrt(unsigned long x){
 unsigned long result=0,bit=1UL<<(sizeof(long)*8-2);sqrt_calls++;
 while(bit>x)bit>>=2;
 while(bit){if(x>=result+bit){x-=result+bit;result=(result>>1)+bit;}else result>>=1;bit>>=2;}
 return result;
}
void host_lock(struct mutex *m){REQUIRE(!current_readers&&!locked);REQUIRE(m->owner.counter==0&&!m->first_waiter);locked=1;}
void host_unlock(struct mutex *m){(void)m;REQUIRE(locked==1);locked=0;}
void *host_alloc(size_t size,unsigned flags){
 REQUIRE(!locked&&!current_readers);REQUIRE(flags==(GFP_KERNEL|__GFP_NOWARN));allocations++;last_size=size;
 if(competing){unsigned long goal=competing;competing=0;REQUIRE(next_prime_number(goal)>goal);}
 if(fail_alloc){fail_alloc--;return NULL;}
 size_t at=(used+63)&~(size_t)63;REQUIRE(size<=sizeof(arena.data)-at);used=at+size;
 memset(arena.data+at,0xa5,size);return arena.data+at;
}
void host_free(void *p){REQUIRE(p&&locked);immediate++;}
void rcu_read_lock(void){readers++;current_readers++;}
static void reclaim(void){if(!readers){reclaimed+=pending_count;pending_count=0;}}
void rcu_read_unlock(void){REQUIRE(readers&&current_readers);readers--;current_readers--;reclaim();}
void rcu_barrier(void){REQUIRE(!readers);reclaim();}
void synchronize_rcu(void){REQUIRE(!readers);reclaim();}
void kvfree_call_rcu(struct kvfree_rcu_head *h,void *p){
 REQUIRE(locked&&p&&h==(void *)&((struct primes *)p)->rcu&&pending_count<128);
 pending[pending_count++]=p;retired++;reclaim();
}
static unsigned long cached_last,cached_size,cache_hash;
static const struct primes *snapshot;
static void observe(void *ctx,const struct primes *p){
 (void)ctx;REQUIRE(current_readers&&readers);cached_last=p->last;cached_size=p->sz;snapshot=p;
 cache_hash=0;for(unsigned long i=0;i<p->sz/BITS_PER_LONG;i++)cache_hash=(cache_hash*33)^p->primes[i];
 REQUIRE(p->sz%BITS_PER_LONG==0&&p->last<p->sz);
 REQUIRE(!(p->primes[0]&3));REQUIRE((p->primes[p->last/BITS_PER_LONG]>>(p->last%BITS_PER_LONG))&1);
}
static void record(unsigned tag,unsigned long value){with_primes(NULL,observe);u64 data[]={tag,value,cached_last,cached_size,cache_hash,allocations,immediate,retired,reclaimed,sqrt_calls,last_size};output(data,sizeof(data));}
static void pinned(void *ctx,const struct primes *p){
 (void)ctx;unsigned before=reclaimed;unsigned long last=p->last,word=p->primes[0];
 unsigned saved=current_readers;current_readers=0;
 REQUIRE(next_prime_number(p->sz+10)>p->sz+10);
 current_readers=saved;
 REQUIRE(reclaimed==before&&p->last==last&&p->primes[0]==word&&pending_count);
}
void cleanup_module(void);
bool fixture_expand(unsigned long);
unsigned fixture_iter(unsigned long,unsigned long,unsigned long*,unsigned);
void run_original_kunit(void);
__attribute__((noreturn)) void suite_main(void){
 record(0,0);REQUIRE(!is_prime_number(0)&&!is_prime_number(1));REQUIRE(slow_is_prime_number(1));
 REQUIRE(next_prime_number(0)==2&&next_prime_number(1)==2);record(1,1);
 // Allocation failure: cache must remain original and exact slow results survive.
 fail_alloc=1;REQUIRE(next_prime_number(BITS_PER_LONG)==(BITS_PER_LONG==64?67:37));record(2,2);
 fail_alloc=1;REQUIRE(!is_prime_number(BITS_PER_LONG+4));record(3,3);
 // A competing expansion completes while the outer allocation is in flight.
 competing=500;REQUIRE(next_prime_number(BITS_PER_LONG)>BITS_PER_LONG);REQUIRE(immediate==1);record(4,4);
 // Actual original C KUnit's complete 2..65535 sweep, against either provider.
 run_original_kunit();record(5,5);
 with_primes(NULL,pinned);REQUIRE(!pending_count&&reclaimed==retired);record(6,6);
 unsigned long words[256];
 for(unsigned long start=0;start<18;start++)for(unsigned long end=0;end<40;end++){
  unsigned n=fixture_iter(start,end,words,256);u64 head[]={start,end,n};output(head,sizeof(head));
  for(unsigned i=0;i<n;i++){u64 value=words[i];output(&value,sizeof(value));}
 }
 cleanup_module();record(7,7);REQUIRE(cached_size==BITS_PER_LONG&&reclaimed==retired);
 REQUIRE(next_prime_number(~0UL)==~0UL);record(8,8);
#ifdef RUST_PROVIDER
 unsigned before=allocations;REQUIRE(!fixture_expand(~0UL));REQUIRE(!fixture_expand(~0UL/2));
 REQUIRE(allocations==before);
 REQUIRE(fixture_iter(~0UL,~0UL,words,256)==1&&words[0]==~0UL);
#endif
 REQUIRE(!locked&&!readers&&!current_readers);finish(0);
}
#if __SIZEOF_POINTER__==8
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%rsp\n call suite_main\n ud2\n");
#else
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%esp\n call suite_main\n ud2\n");
#endif
'''


class Fixture:
    def __init__(self, work, bits):
        self.work, self.bits = Path(work), bits
        self.work.mkdir(parents=True)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        targets = rust_targets()
        if bits not in targets:
            raise unittest.SkipTest("set INT_MATH_I686_SYSROOT for matching genuine 32-bit core")
        self.target = targets[bits]
        explicit = os.environ.get("BINDGEN")
        found = shutil.which("bindgen") or shutil.which("bindgen-0.71")
        if not explicit and not found:
            raise unittest.SkipTest("set BINDGEN for actual prime/mutex C layouts")
        bindgen = shlex.split(explicit) if explicit else [found]
        self.flags = [*headers(self.work), "-m"+str(bits), "-funsigned-char", "-D__KERNEL__",
                      "-DCONFIG_PRIME_NUMBERS_KUNIT_TEST=1", "-DMODULE",
                      '-DKBUILD_MODNAME="prime_numbers"', '-DKBUILD_MODFILE="lib/math/prime_numbers"']
        if bits == 64: self.flags += ["-DCONFIG_64BIT"]
        include = self.work / "include/linux"
        (include / "types.h").write_text(declarations())
        (include / "bits.h").write_text("#define BIT(n) (1UL<<(n))\n")
        module = include / "module.h"
        module.write_text(module.read_text()+"\n#define IS_ENABLED(x) 1\n#define __rcu\n#define __exit\n"
            "#define module_exit(fn) void cleanup_module(void){fn();}\n#define MODULE_AUTHOR(x)\n"
            "#include <linux/export.h>\n#include <linux/gfp_types.h>\n")
        (include / "mutex.h").write_text('#include "'+str(self.work / "transport.h")+'"\n')
        (include / "slab.h").write_text('#include "'+str(self.work / "transport.h")+'"\n')
        (self.work / "transport.h").write_text(r'''
#ifndef PRIME_TRANSPORT
#define PRIME_TRANSPORT
#include <linux/types.h>
#include <linux/gfp_types.h>
#include <linux/limits.h>
#define BITS_PER_LONG (__SIZEOF_LONG__*8)
#define BIT(n) (1UL<<(n))
#define roundup(x,y) ((((x)+(y)-1)/(y))*(y))
#define round_up(x,y) (((x)+(y)-1)&~((y)-1))
#define DEFINE_MUTEX(name) struct mutex name={0}
#define RCU_INITIALIZER(p) (p)
#define rcu_dereference(p) (p)
#define rcu_dereference_protected(p,c) (p)
#define rcu_assign_pointer(p,v) ((p)=(v))
#define lockdep_is_held(p) 1
#define kfree_rcu(p,field) kvfree_call_rcu((void *)&(p)->field,(void *)(p))
#define BUG_ON(x) do{if(x)BUG();}while(0)
void BUG(void);
void host_lock(struct mutex *);
void host_unlock(struct mutex *);
#define mutex_lock host_lock
#define mutex_unlock host_unlock
void *host_alloc(size_t,unsigned);
void host_free(void *);
#define kmalloc host_alloc
#define kfree host_free
void rcu_read_lock(void);
void rcu_read_unlock(void);
void rcu_barrier(void);
void synchronize_rcu(void);
void kvfree_call_rcu(struct kvfree_rcu_head *,void *);
unsigned long int_sqrt(unsigned long);
static inline size_t bitmap_size(unsigned long bits){return ((bits+BITS_PER_LONG-1)/BITS_PER_LONG)*sizeof(long);}
static inline void bitmap_fill(unsigned long *p,unsigned long n){for(unsigned long i=0;i<n/BITS_PER_LONG;i++)p[i]=~0UL;}
static inline void bitmap_copy(unsigned long*d,const unsigned long*s,unsigned long n){for(unsigned long i=0;i<n/BITS_PER_LONG;i++)d[i]=s[i];}
static inline void __clear_bit(unsigned long n,unsigned long*p){p[n/BITS_PER_LONG]&=~BIT(n%BITS_PER_LONG);}
static inline bool test_bit(unsigned long n,const unsigned long*p){return !!(p[n/BITS_PER_LONG]&BIT(n%BITS_PER_LONG));}
static inline unsigned long find_next_bit(const unsigned long*p,unsigned long size,unsigned long start){while(start<size&&!test_bit(start,p))start++;return start;}
#endif
''')
        (self.work / "fixture.h").write_text('#include "transport.h"\n#define IS_ENABLED(x) 1\n#include <linux/prime_numbers.h>\n'
            '#include "'+str(ROOT / "lib/math/prime_numbers_private.h")+'"\n'
            'static const unsigned RUST_GFP_KERNEL=GFP_KERNEL;\nstatic const unsigned RUST_GFP_NOWARN=__GFP_NOWARN;\n')
        run([*bindgen, self.work / "fixture.h", "--rust-target=1.85", "--use-core", "--ctypes-prefix=crate::ffi",
             "--no-layout-tests", "--no-doc-comments", "--no-debug=.*", "--allowlist-type=primes|primes_fn|mutex|raw_spinlock_t",
             "--blocklist-type=primes_fn",
             "--allowlist-function=host_.*|rcu_.*|synchronize_rcu|kvfree_call_rcu|int_sqrt|BUG|.*prime.*",
             "--allowlist-var=RUST_GFP_.*", "-o", self.work / "bindings.rs", "--", *self.flags])
        bindings = self.work / "bindings.rs"
        bindings.write_text(bindings.read_text() + primes_callback_definition())
        types = (ROOT / "rust/kernel/types.rs").read_text()
        facade = FACADE.replace("@FFI@", '"'+str(ROOT / "rust/ffi.rs")+'"').replace("@RCU@", '"'+str(ROOT / "rust/kernel/sync/rcu.rs")+'"')
        facade = facade.replace("@OPAQUE@", re.search(r"#\[repr\(transparent\)\]\npub struct Opaque<T> \{.*?\n\}", types, re.S)[0])
        facade = facade.replace("@NEW@", re.search(r"pub const fn new\(value: T\) -> Self \{.*?\n    \}", types, re.S)[0])
        facade = facade.replace("@GET@", re.search(r"pub const fn get\(&self\) -> \*mut T \{.*?\n    \}", types, re.S)[0])
        (self.work / "kernel.rs").write_text(facade)
        self.env = {**environment(), "RUSTC_BOOTSTRAP":"1", "RUST_MODFILE":"lib/math/prime_numbers"}
        self.library = self.work / "libkernel.rlib"
        run([*self.rustc, *self.target, *rust_flags("2"), "--crate-name=kernel", "--crate-type=rlib",
             "--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST", self.work / "kernel.rs", "-o", self.library], env=self.env)

    def owner(self, rust, optimize, mutation=None, object_only=False, test="y", module=True, kcfi=False):
        path = self.work / ("owner.rs" if rust else "owner.c")
        if rust:
            text = SOURCE.read_text()
            # Only relative module paths change; every provider algorithm and
            # lifetime/synchronization statement is compiled verbatim.
            for relative in ("../../rust/ffi_export.rs", "prime_numbers_mutex.rs"):
                self.assert_path(text, relative)
                text = text.replace('"'+relative+'"', '"'+str((SOURCE.parent / relative).resolve())+'"')
            if mutation:
                old, new, count = {
                    "missing-reader-drop": ("drop(snapshot);", "let _ = &snapshot;", 2),
                    "missing-writer-recheck": ("if x < previous_last {", "if false && x < previous_last {", 1),
                    "wrong-gfp-flags": ("GFP_KERNEL | __GFP_NOWARN", "GFP_KERNEL | (GFP_KERNEL & __GFP_NOWARN)", 1),
                    "nullable-callback": (
                        'r#fn: bindings::primes_fn) {\n    let snapshot = Snapshot::new();',
                        'r#fn: Option<bindings::primes_fn>) {\n    let r#fn = r#fn.unwrap();\n    let snapshot = Snapshot::new();', 1),
                }[mutation]
                if text.count(old) != count:
                    raise AssertionError("production control changed: "+mutation)
                text=text.replace(old,new)
            text += r'''
/// Test-only direct growth probe; actual private implementation is unchanged.
#[no_mangle] pub extern "C" fn fixture_expand(x:kernel::ffi::c_ulong)->bool{expand_to_next_prime(x)}
#[path="@PUBLIC@"] pub mod public_api;
/// Enumerate the actual facade without changing its lazy first element.
#[no_mangle] pub unsafe extern "C" fn fixture_iter(from:kernel::ffi::c_ulong,max:kernel::ffi::c_ulong,out:*mut kernel::ffi::c_ulong,cap:u32)->u32{
 let mut n=0;for value in public_api::for_each_prime_number_from(from,max){if n==cap{break;}unsafe{out.add(n as usize).write(value)};n+=1;}n
}
'''.replace("@PUBLIC@", str(PUBLIC))
            # All safe wrappers are reachable for API/strict-lint compilation.
            text += '\nconst _:fn(kernel::ffi::c_ulong)->bool=public_api::is_prime_number;\nconst _:fn(kernel::ffi::c_ulong)->public_api::PrimeNumbers=public_api::for_each_prime_number;\n'
            if not object_only:
                text += panic_handler()
            path.write_text(text)
            output = self.work / ("owner-O"+optimize+(".o" if object_only else ".a"))
            cfg=(["--cfg=MODULE"] if module else [])
            if test!="n": cfg += ["--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST"]
            if test=="m": cfg += ['--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST="m"']
            run([*self.rustc,*self.target,*rust_flags(optimize),"--crate-name=prime_numbers",
                 "--crate-type="+("rlib" if object_only else "staticlib"),
                 *(["--emit=obj"] if object_only else []),
                 *(["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers",
                    "--emit=llvm-ir="+str(output.with_suffix(".ll"))] if kcfi else []),
                 "--extern","kernel="+str(self.library),*cfg,
                 "-Zcrate-attr=no_std",path,"-o",output],env=self.env)
        else:
            path.write_text('#include "'+str(ROOT / "lib/math/prime_numbers.c")+'"\n'
                'unsigned fixture_iter(unsigned long start,unsigned long max,unsigned long*out,unsigned cap){unsigned n=0;unsigned long p;for_each_prime_number_from(p,start,max){if(n==cap)break;out[n++]=p;}return n;}\n')
            output = self.work / "owner-c.o"
            flags = [*self.cc,*self.flags,"-O"+optimize,"-fno-pic","-fno-pie","-ffreestanding","-fno-stack-protector"]
            if kcfi:
                flags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
                run([*flags,"-S","-emit-llvm",path,"-o",output.with_suffix(".ll")])
            run([*flags,"-c",path,"-o",output])
        return output

    @staticmethod
    def assert_path(text, relative):
        if text.count('"'+relative+'"') != 1:
            raise AssertionError("production module path changed: "+relative)

    def execute(self, rust, optimize="2", mutation=None, expected_exit=0):
        owner = self.owner(rust,optimize,mutation)
        source = (ROOT / "lib/math/tests/prime_numbers_kunit.c").read_text()
        body = re.search(r"static void prime_numbers_test\(struct kunit \*test\)\n\{.*?\n\}",source,re.S)[0]
        # Exact original sweep/body, with only failure transport replaced. The
        # complete original KUnit registration still belongs to native VM tests.
        body = body.replace("static void prime_numbers_test(struct kunit *test)","void run_original_kunit(void)")
        program = DRIVER + '\n#define KUNIT_ASSERT_EQ_MSG(test,a,b,...) REQUIRE((a)==(b))\n'+body+'\n'
        driver = self.work / "driver.c"
        driver.write_text(program)
        linker = self.work / "link.lds"
        linker.write_text('SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr .gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n')
        binary = self.work / ("run-rust-"+optimize if rust else "run-c")
        run([*self.cc,*self.flags,"-O2","-ffreestanding","-fno-builtin","-fno-pic","-fno-pie","-fno-stack-protector",
             "-static","-nostdlib","-no-pie","-Wl,--gc-sections","-Wl,-e,_start","-Wl,-T,"+str(linker),
             *(["-DRUST_PROVIDER"] if rust else []),driver,owner,"-o",binary])
        if binary.read_bytes()[:6] != b"\x7fELF"+bytes([1 if self.bits==32 else 2,1]):
            raise AssertionError("genuine target ABI was not emitted")
        runner=shlex.split(os.environ.get("INT_MATH_I686_RUNNER","")) if self.bits==32 else []
        result=subprocess.run([*runner,binary],capture_output=True,timeout=90)
        if result.returncode!=expected_exit:
            raise AssertionError(f"expected process exit {expected_exit}, got {result.returncode}: "+result.stderr.decode(errors="replace"))
        return result.stdout


class PrimeNumbersTests(unittest.TestCase):
    def setUp(self):
        directory=tempfile.TemporaryDirectory(prefix="prime-numbers-")
        self.addCleanup(directory.cleanup)
        self.work=Path(directory.name)

    def behavior(self,bits):
        fixture=Fixture(self.work / str(bits),bits)
        original=fixture.execute(False)
        self.assertGreater(len(original),720*24)
        for optimize in ("0","2","s"):
            with self.subTest(bits=bits,optimize=optimize):
                self.assertEqual(fixture.execute(True,optimize),original)

    def test_lp64_original_sieve_kunit_sweep_fallback_recheck_rcu_and_iterators(self):
        self.behavior(64)

    def test_ilp32_original_sieve_kunit_sweep_fallback_recheck_rcu_and_iterators(self):
        self.behavior(32)

    def test_runtime_controls_detect_reader_scope_writer_recheck_and_allocation_flags(self):
        fixture=Fixture(self.work / "controls",64)
        fixture.execute(True)
        allocation_line=next(i for i,line in enumerate(DRIVER.splitlines(),1) if "REQUIRE(!locked&&!current_readers)" in line)
        for mutation,status in (("missing-reader-drop",allocation_line%160+40),
                                ("missing-writer-recheck",97),
                                ("wrong-gfp-flags",allocation_line%160+40)):
            with self.subTest(mutation=mutation):
                fixture.execute(True,mutation=mutation,expected_exit=status)

    def test_native_width_exports_bindings_and_original_exit_only_metadata(self):
        for bits in rust_targets():
            fixture=Fixture(self.work / str(bits),bits)
            for module in (False,True):
                for test in ("n","y","m"):
                    with self.subTest(bits=bits,module=module,kunit=test):
                        obj=fixture.owner(True,"2",object_only=True,test=test,module=module)
                        records=read_exports(obj)
                        names=(["with_primes","slow_is_prime_number"] if test!="n" else [])+["next_prime_number","is_prime_number"]
                        self.assertEqual([r["name"] for r in records],names)
                        self.assertTrue(all(r["license"]==r["namespace"]=="" and r["relocation_target"]==r["name"] and r["pointer_width"]==bits//8 for r in records))
                        fields=[b"author=Intel Corporation",b"description=Prime number library",b"license=GPL"]
                        if not module: fields=[b"prime_numbers."+value for value in fields]+[b"prime_numbers.file=lib/math/prime_numbers"]
                        self.assertEqual(sorted(module_info(obj)),sorted(fields))
                        symbols=run([*shlex.split(os.environ.get("NM","nm")),obj])
                        self.assertNotIn(b" init_module\n",symbols)
                        self.assertEqual(b" cleanup_module\n" in symbols,module)
                        self.assertEqual(b"__IS_RUST_MODULE" in symbols,module)
                        self.assertEqual(b" T with_primes\n" in symbols,test!="n")

    def test_unused_public_private_facades_defer_native_imports_to_independent_consumers(self):
        for bits in rust_targets():
            fixture=Fixture(self.work / str(bits),bits)
            header=self.work / ("headers"+str(bits)+".rs")
            header.write_text('//! Header-only built-in-style facade.\n#![no_std]\n'
                '#[path="'+str(PUBLIC)+'"] pub mod primes;\n'
                '#[path="'+str(PRIVATE)+'"] pub mod private;\n')
            consumer=self.work / ("consumer"+str(bits)+".rs")
            consumer.write_text('//! Independent facade consumer.\n#![no_std]\n'
                '/// Use both real ABI imports and lazy iteration.\n#[no_mangle]\n'
                'pub extern "C" fn probe(x:kernel::ffi::c_ulong)->kernel::ffi::c_ulong{'
                'let mut out=headers::primes::is_prime_number(x) as kernel::ffi::c_ulong;'
                'for p in headers::primes::for_each_prime_number_from(x,headers::primes::next_prime_number(x)){out=out.wrapping_add(p);}out}\n')
            for optimize in ("0","2","s"):
                with self.subTest(bits=bits,optimize=optimize):
                    obj=fixture.work / "headers.o"
                    lib=fixture.work / "libheaders.rlib"
                    flags=[*fixture.rustc,*fixture.target,*rust_flags(optimize),"--extern","kernel="+str(fixture.library),
                           "--crate-type=rlib","--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST",
                           '--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST="m"']
                    run([*flags,"--crate-name=headers","--emit=obj="+str(obj),"--emit=link="+str(lib),header],env=fixture.env)
                    self.assertEqual(run(["nm","-u",obj]).strip(),b"")
                    run([*flags,"--crate-name=consumer","-L",fixture.work,"--extern","headers="+str(lib),
                         "--emit=obj="+str(obj),consumer],env=fixture.env)
                    undefined=run(["nm","-u",obj])
                    for name in ("is_prime_number","next_prime_number"):
                        self.assertIn(b"U "+name.encode()+b"\n",undefined)
                    self.assertNotIn(b"host_alloc",undefined)

    def test_exact_original_source_markers_and_native_storage_contract(self):
        for path in (SOURCE,PUBLIC,PRIVATE):
            baseline=run(["git","show","68f3e0875:"+str(path.relative_to(ROOT))],cwd=ROOT).decode()
            self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+",path.read_text()),re.findall(r"SOURCE-COMMIT: \w+",baseline))
        source=SOURCE.read_text()
        self.assertIn("bindings::primes",source)
        self.assertIn("bindings::kvfree_call_rcu",source)
        self.assertNotIn("fn init_module",source)
        self.assertNotIn("call_rcu(",source.replace("kvfree_call_rcu(",""))


if __name__=="__main__":
    unittest.main()
