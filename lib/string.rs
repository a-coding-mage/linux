// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/lib/string.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

/* This file should be used only for library routines. */

// C headers and build-time configuration supplied by the surrounding kernel
// are intentionally represented by the corresponding external Rust symbols.

#[cfg(not(__HAVE_ARCH_STRNCASECMP))]
pub unsafe fn strncasecmp(mut s1: *const u8, mut s2: *const u8, mut len: usize) -> i32 {
    let (mut c1, mut c2): (u8, u8);
    if len == 0 { return 0; }
    loop {
        c1 = *s1; s1 = s1.add(1);
        c2 = *s2; s2 = s2.add(1);
        if c1 == 0 || c2 == 0 { break; }
        if c1 == c2 { } else {
            c1 = c1.to_ascii_lowercase(); c2 = c2.to_ascii_lowercase();
            if c1 != c2 { break; }
        }
        len -= 1;
        if len == 0 { break; }
    }
    c1 as i32 - c2 as i32
}

#[cfg(not(__HAVE_ARCH_STRCASECMP))]
pub unsafe fn strcasecmp(mut s1: *const u8, mut s2: *const u8) -> i32 {
    let (mut c1, mut c2);
    loop {
        c1 = (*s1).to_ascii_lowercase() as i32; s1 = s1.add(1);
        c2 = (*s2).to_ascii_lowercase() as i32; s2 = s2.add(1);
        if c1 != c2 || c1 == 0 { break; }
    }
    c1 - c2
}

#[cfg(not(__HAVE_ARCH_STRCPY))]
pub unsafe fn strcpy(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    let tmp = dest;
    loop { let c = *src; src = src.add(1); *dest = c; dest = dest.add(1); if c == 0 { break; } }
    tmp
}

#[cfg(__BIG_ENDIAN)]
const ALLBUTLAST_BYTE_MASK: usize = !255usize;
#[cfg(not(__BIG_ENDIAN))]
const ALLBUTLAST_BYTE_MASK: usize = !0usize >> 8;

pub unsafe fn sized_strscpy(dest: *mut u8, src: *const u8, mut count: usize) -> isize {
    if count == 0 { return -7; }
    let mut res = 0usize;
    while count >= core::mem::size_of::<usize>() {
        let mut c = (src.add(res) as *const usize).read_unaligned();
        let mut i = 0usize;
        while i < core::mem::size_of::<usize>() {
            let b = (c >> (i * 8)) as u8;
            if b == 0 {
                (dest.add(res) as *mut usize).write_unaligned(c & (!0usize >> ((core::mem::size_of::<usize>() - i - 1) * 8)));
                return (res + i) as isize;
            }
            i += 1;
        }
        count -= core::mem::size_of::<usize>();
        if count == 0 {
            c &= ALLBUTLAST_BYTE_MASK;
            (dest.add(res) as *mut usize).write_unaligned(c);
            return -7;
        }
        (dest.add(res) as *mut usize).write_unaligned(c);
        res += core::mem::size_of::<usize>();
    }
    while count > 1 {
        let c = *src.add(res); *dest.add(res) = c;
        if c == 0 { return res as isize; }
        res += 1; count -= 1;
    }
    *dest.add(res) = 0;
    if *src.add(res) != 0 { -7 } else { res as isize }
}

pub unsafe fn stpcpy(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    loop { let c = *src; src = src.add(1); *dest = c; dest = dest.add(1); if c == 0 { return dest.sub(1); } }
}

#[cfg(not(__HAVE_ARCH_STRCAT))]
pub unsafe fn strcat(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    let tmp = dest; while *dest != 0 { dest = dest.add(1); }
    loop { let c = *src; src = src.add(1); *dest = c; dest = dest.add(1); if c == 0 { break; } } tmp
}

#[cfg(not(__HAVE_ARCH_STRNCAT))]
pub unsafe fn strncat(mut dest: *mut u8, mut src: *const u8, mut count: usize) -> *mut u8 {
    let tmp = dest; if count != 0 { while *dest != 0 { dest = dest.add(1); }
        loop { let c = *src; src = src.add(1); *dest = c; dest = dest.add(1); if c == 0 { break; } count -= 1; if count == 0 { *dest = 0; break; } }
    } tmp
}

#[cfg(not(__HAVE_ARCH_STRLCAT))]
pub unsafe fn strlcat(dest: *mut u8, src: *const u8, count: usize) -> usize {
    let dsize = strlen(dest); let len = strlen(src); let res = dsize + len;
    if dsize >= count { core::hint::unreachable_unchecked(); }
    let mut n = count - dsize; if len >= n { n -= 1; } memcpy(dest.add(dsize), src, n); *dest.add(dsize+n) = 0; res
}

#[cfg(not(__HAVE_ARCH_STRCMP))]
pub unsafe fn strcmp(mut cs: *const u8, mut ct: *const u8) -> i32 { loop { let c1=*cs; let c2=*ct; cs=cs.add(1); ct=ct.add(1); if c1!=c2 { return if c1<c2 {-1} else {1}; } if c1==0 { return 0; } } }
#[cfg(not(__HAVE_ARCH_STRNCMP))]
pub unsafe fn strncmp(mut cs: *const u8, mut ct: *const u8, mut count: usize) -> i32 { while count != 0 { let c1=*cs; let c2=*ct; cs=cs.add(1); ct=ct.add(1); if c1!=c2 { return if c1<c2 {-1} else {1}; } if c1==0 { break; } count-=1; } 0 }

#[cfg(not(__HAVE_ARCH_STRCHR))]
pub unsafe fn strchr(mut s: *const u8, c: i32) -> *mut u8 { loop { if *s == c as u8 { return s as *mut u8; } if *s == 0 { return core::ptr::null_mut(); } s=s.add(1); } }
#[cfg(not(__HAVE_ARCH_STRCHRNUL))]
pub unsafe fn strchrnul(mut s: *const u8, c: i32) -> *mut u8 { while *s != 0 && *s != c as u8 { s=s.add(1); } s as *mut u8 }
pub unsafe fn strnchrnul(mut s: *const u8, mut count: usize, c: i32) -> *mut u8 { while count != 0 && *s != 0 && *s != c as u8 { s=s.add(1); count-=1; } s as *mut u8 }
#[cfg(not(__HAVE_ARCH_STRRCHR))]
pub unsafe fn strrchr(mut s: *const u8, c: i32) -> *mut u8 { let mut last=core::ptr::null(); loop { if *s==c as u8 {last=s;} let z=*s; s=s.add(1); if z==0 {return last as *mut u8;} } }
#[cfg(not(__HAVE_ARCH_STRNCHR))]
pub unsafe fn strnchr(mut s: *const u8, mut count: usize, c: i32) -> *mut u8 { while count!=0 { if *s==c as u8{return s as *mut u8;} let z=*s; s=s.add(1); count-=1; if z==0{break;} } core::ptr::null_mut() }
#[cfg(not(__HAVE_ARCH_STRLEN))]
pub unsafe fn strlen(mut s: *const u8) -> usize { let p=s; while *s!=0{s=s.add(1);} s.offset_from(p) as usize }
#[cfg(not(__HAVE_ARCH_STRNLEN))]
pub unsafe fn strnlen(mut s: *const u8, mut count: usize) -> usize { let p=s; while count!=0&&*s!=0{s=s.add(1);count-=1;} s.offset_from(p) as usize }

#[cfg(not(__HAVE_ARCH_STRSPN))]
pub unsafe fn strspn(mut s:*const u8, accept:*const u8)->usize{let p=s;while *s!=0{if strchr(accept,*s as i32).is_null(){break;}s=s.add(1);}s.offset_from(p)as usize}
#[cfg(not(__HAVE_ARCH_STRCSPN))]
pub unsafe fn strcspn(mut s:*const u8,reject:*const u8)->usize{let p=s;while *s!=0{if !strchr(reject,*s as i32).is_null(){break;}s=s.add(1);}s.offset_from(p)as usize}
#[cfg(not(__HAVE_ARCH_STRPBRK))]
pub unsafe fn strpbrk(mut cs:*const u8,ct:*const u8)->*mut u8{while *cs!=0{if !strchr(ct,*cs as i32).is_null(){return cs as *mut u8;}cs=cs.add(1);}core::ptr::null_mut()}
#[cfg(not(__HAVE_ARCH_STRSEP))]
pub unsafe fn strsep(s:*mut *mut u8,ct:*const u8)->*mut u8{let begin=*s;if begin.is_null(){return core::ptr::null_mut();}let end=strpbrk(begin,ct);if !end.is_null(){*end=0;*s=end.add(1);}else{*s=core::ptr::null_mut();}begin}

#[cfg(not(__HAVE_ARCH_MEMSET))]
pub unsafe fn memset(s:*mut u8,c:i32,mut count:usize)->*mut u8{let p=s;while count!=0{*s=c as u8;s=s.add(1);count-=1;}p}
#[cfg(not(__HAVE_ARCH_MEMSET16))]
pub unsafe fn memset16(mut s:*mut u16,v:u16,mut count:usize)->*mut u16{let p=s;while count!=0{*s=v;s=s.add(1);count-=1;}p}
#[cfg(not(__HAVE_ARCH_MEMSET32))]
pub unsafe fn memset32(mut s:*mut u32,v:u32,mut count:usize)->*mut u32{let p=s;while count!=0{*s=v;s=s.add(1);count-=1;}p}
#[cfg(not(__HAVE_ARCH_MEMSET64))]
pub unsafe fn memset64(mut s:*mut u64,v:u64,mut count:usize)->*mut u64{let p=s;while count!=0{*s=v;s=s.add(1);count-=1;}p}
#[cfg(not(__HAVE_ARCH_MEMCPY))]
pub unsafe fn memcpy(mut dest:*mut u8,mut src:*const u8,mut count:usize)->*mut u8{let p=dest;while count!=0{*dest=*src;dest=dest.add(1);src=src.add(1);count-=1;}p}
#[cfg(not(__HAVE_ARCH_MEMMOVE))]
pub unsafe fn memmove(dest:*mut u8,src:*const u8,mut count:usize)->*mut u8{if (dest as usize)<=(src as usize){let mut d=dest;let mut s=src;while count!=0{*d=*s;d=d.add(1);s=s.add(1);count-=1;}}else{let mut d=dest.add(count);let mut s=src.add(count);while count!=0{d=d.sub(1);s=s.sub(1);*d=*s;count-=1;}}dest}

#[cfg(not(__HAVE_ARCH_MEMCMP))]
pub unsafe fn memcmp(mut cs:*const u8,mut ct:*const u8,mut count:usize)->i32{while count!=0{let d=*cs as i32-*ct as i32;if d!=0{return d;}cs=cs.add(1);ct=ct.add(1);count-=1;}0}
#[cfg(not(__HAVE_ARCH_BCMP))]
pub unsafe fn bcmp(a:*const u8,b:*const u8,len:usize)->i32{memcmp(a,b,len)}
#[cfg(not(__HAVE_ARCH_MEMSCAN))]
pub unsafe fn memscan(mut addr:*mut u8,c:i32,mut size:usize)->*mut u8{while size!=0{if *addr==c as u8{return addr;}addr=addr.add(1);size-=1;}addr}
#[cfg(not(__HAVE_ARCH_STRSTR))]
pub unsafe fn strstr(mut s1:*const u8,s2:*const u8)->*mut u8{let l2=strlen(s2);if l2==0{return s1 as *mut u8;}let mut l1=strlen(s1);while l1>=l2{l1-=1;if memcmp(s1,s2,l2)==0{return s1 as *mut u8;}s1=s1.add(1);}core::ptr::null_mut()}
#[cfg(not(__HAVE_ARCH_STRNSTR))]
pub unsafe fn strnstr(mut s1:*const u8,s2:*const u8,mut len:usize)->*mut u8{let l2=strlen(s2);if l2==0{return s1 as *mut u8;}while len>=l2{len-=1;if memcmp(s1,s2,l2)==0{return s1 as *mut u8;}s1=s1.add(1);}core::ptr::null_mut()}
#[cfg(not(__HAVE_ARCH_MEMCHR))]
pub unsafe fn memchr(mut s:*const u8,c:i32,mut n:usize)->*mut u8{while n!=0{if *s==c as u8{return s as *mut u8;}s=s.add(1);n-=1;}core::ptr::null_mut()}

unsafe fn check_bytes8(mut start:*const u8,value:u8,mut bytes:usize)->*mut u8{while bytes!=0{if *start!=value{return start as *mut u8;}start=start.add(1);bytes-=1;}core::ptr::null_mut()}
pub unsafe fn memchr_inv(start:*const u8,c:i32,bytes:usize)->*mut u8{let value=c as u8;if bytes<=16{return check_bytes8(start,value,bytes);}let mut p=start;let prefix=(p as usize)%8;if prefix!=0{let n=8-prefix;let r=check_bytes8(p,value,n);if !r.is_null(){return r;}p=p.add(n);}let mut words=(bytes-prefix)/8;let v=(value as u64)*0x0101010101010101;while words!=0{if (p as *const u64).read_unaligned()!=v{return check_bytes8(p,value,8);}p=p.add(8);words-=1;}check_bytes8(p,value,(bytes-prefix)%8)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
