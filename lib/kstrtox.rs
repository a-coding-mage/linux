// SPDX-License-Identifier: GPL-2.0
/* Convert integer string representation to an integer. */

// External kernel definitions and helpers are supplied by other translation units.
extern "C" {
    fn _parse_integer(s: *const core::ffi::c_char, base: u32, p: *mut u64) -> u32;
    fn _tolower(c: u32) -> u32;
    fn isxdigit(c: u32) -> bool;
    fn isdigit(c: u32) -> bool;
    fn check_mul_overflow_u64(a: u64, b: u64, out: *mut u64) -> bool;
    fn check_add_overflow_u64(a: u64, b: u64, out: *mut u64) -> bool;
    fn int_pow(base: u32, exp: u32) -> u64;
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}

pub const KSTRTOX_OVERFLOW: u32 = 1 << 31;
pub const ERANGE: i32 = 34;
pub const EINVAL: i32 = 22;
pub const EFAULT: i32 = 14;

#[no_mangle]
pub unsafe extern "C" fn _parse_integer_fixup_radix(mut s: *const core::ffi::c_char, base: *mut u32) -> *const core::ffi::c_char {
    if *base == 0 {
        if *s as u8 == b'0' {
            if _tolower(*s.add(1) as u32) == b'x' as u32 && isxdigit(*s.add(2) as u32) { *base = 16; } else { *base = 8; }
        } else { *base = 10; }
    }
    if *base == 16 && *s as u8 == b'0' && _tolower(*s.add(1) as u32) == b'x' as u32 { s = s.add(2); }
    s
}

#[no_mangle]
pub unsafe extern "C" fn _parse_integer_limit(s: *const core::ffi::c_char, base: u32, p: *mut u64, max_chars: usize, init: u64) -> u32 {
    let mut rv = 0u32; let mut overflow = 0u32; let mut res = init;
    while (rv as usize) < max_chars {
        let c = *s.add(rv as usize) as u32; let lc = _tolower(c);
        let val = if c >= b'0' as u32 && c <= b'9' as u32 { c - b'0' as u32 } else if lc >= b'a' as u32 && lc <= b'f' as u32 { lc - b'a' as u32 + 10 } else { break };
        if val >= base { break; }
        if (res & (!0u64 << 60)) != 0 {
            if check_mul_overflow_u64(res, base as u64, &mut res) || check_add_overflow_u64(res, val as u64, &mut res) { res = u64::MAX; overflow = KSTRTOX_OVERFLOW; }
        } else { res = res.wrapping_mul(base as u64).wrapping_add(val as u64); }
        rv += 1;
    }
    *p = res; rv | overflow
}

unsafe fn _kstrtoull(mut s: *const core::ffi::c_char, mut base: u32, res: *mut u64) -> i32 {
    s = _parse_integer_fixup_radix(s, &mut base); let mut tmp = 0u64; let rv = _parse_integer(s, base, &mut tmp);
    if rv & KSTRTOX_OVERFLOW != 0 { return -ERANGE; } if rv == 0 { return -EINVAL; }
    s = s.add(rv as usize); if *s as u8 == b'\n' { s = s.add(1); } if *s != 0 { return -EINVAL; } *res = tmp; 0
}

#[no_mangle] pub unsafe extern "C" fn kstrtoull(mut s: *const i8, base: u32, res: *mut u64) -> i32 { if *s as u8 == b'+' { s = s.add(1); } _kstrtoull(s, base, res) }
#[no_mangle] pub unsafe extern "C" fn kstrtoll(s: *const i8, base: u32, res: *mut i64) -> i32 { let mut t=0u64; let rv=if *s as u8==b'-' {_kstrtoull(s.add(1),base,&mut t)} else {kstrtoull(s,base,&mut t)}; if rv<0{return rv}; if *s as u8==b'-' {if (-(t as i64))>0{return -ERANGE;} *res=-(t as i64)} else {if t as i64<0{return -ERANGE;} *res=t as i64}; 0 }
#[no_mangle] pub unsafe extern "C" fn _kstrtoul(s:*const i8,b:u32,r:*mut usize)->i32 {let mut t=0u64;let v=kstrtoull(s,b,&mut t);if v<0{return v} if t as usize as u64!=t{return -ERANGE}*r=t as usize;0}
#[no_mangle] pub unsafe extern "C" fn _kstrtol(s:*const i8,b:u32,r:*mut isize)->i32 {let mut t=0i64;let v=kstrtoll(s,b,&mut t);if v<0{return v} if t as isize as i64!=t{return -ERANGE}*r=t as isize;0}

macro_rules! unsigned {($n:ident,$t:ty,$w:ident)=>{#[no_mangle] pub unsafe extern "C" fn $n(s:*const i8,b:u32,r:*mut $t)->i32{let mut t=0u64;let v=kstrtoull(s,b,&mut t);if v<0{return v}if t as $t as u64!=t{return -ERANGE}*r=t as $t;0}}}
macro_rules! signed {($n:ident,$t:ty,$w:ident)=>{#[no_mangle] pub unsafe extern "C" fn $n(s:*const i8,b:u32,r:*mut $t)->i32{let mut t=0i64;let v=kstrtoll(s,b,&mut t);if v<0{return v}if t as $t as i64!=t{return -ERANGE}*r=t as $t;0}}}
unsigned!(kstrtouint,u32,u); signed!(kstrtoint,i32,s); unsigned!(kstrtou16,u16,u); signed!(kstrtos16,i16,s); unsigned!(kstrtou8,u8,u); signed!(kstrtos8,i8,s);

#[no_mangle] pub unsafe extern "C" fn kstrtobool(s:*const i8,r:*mut bool)->i32 {if s.is_null(){return -EINVAL} match *s as u8 {b'e'|b'E'|b'y'|b'Y'|b't'|b'T'|b'1'=>{*r=true;0},b'd'|b'D'|b'n'|b'N'|b'f'|b'F'|b'0'=>{*r=false;0},b'o'|b'O'=>match *s.add(1) as u8 {b'n'|b'N'=>{*r=true;0},b'f'|b'F'=>{*r=false;0},_=>-EINVAL},_=>-EINVAL}}

unsafe fn _kstrtoudec64(mut s:*const i8,scale:u32,res:*mut u64)->i32 {let mut out=0u64;let a=_parse_integer(s,10,&mut out);if a&KSTRTOX_OVERFLOW!=0{return -ERANGE}s=s.add(a as usize);if *s as u8==b'.'{s=s.add(1)}let b=_parse_integer_limit(s,10,&mut out,scale as usize,out);if b&KSTRTOX_OVERFLOW!=0{return -ERANGE}s=s.add((b&!KSTRTOX_OVERFLOW) as usize);if a==0&&b==0&&!isdigit(*s as u32){return -EINVAL}while isdigit(*s as u32){s=s.add(1)}if *s as u8==b'\n'{s=s.add(1)}if *s!=0{return -EINVAL}if out!=0&&((scale-(b&!KSTRTOX_OVERFLOW))>19||check_mul_overflow_u64(out,int_pow(10,scale-(b&!KSTRTOX_OVERFLOW)),&mut out)){return -ERANGE}*res=out;0}
#[no_mangle] pub unsafe extern "C" fn kstrtoudec64(s:*const i8,scale:u32,r:*mut u64)->i32{_kstrtoudec64(if *s as u8==b'+'{s.add(1)}else{s},scale,r)}
#[no_mangle] pub unsafe extern "C" fn kstrtodec64(s:*const i8,scale:u32,r:*mut i64)->i32{let mut t=0;let v=if *s as u8==b'-'{_kstrtoudec64(s.add(1),scale,&mut t)}else{kstrtoudec64(s,scale,&mut t)};if v<0{return v}if *s as u8==b'-'{if -(t as i64)>0{return -ERANGE}*r=-(t as i64)}else{if t as i64<0{return -ERANGE}*r=t as i64}0}

#[no_mangle] pub unsafe extern "C" fn kstrtobool_from_user(s:*const core::ffi::c_void,count:usize,r:*mut bool)->i32{let mut b=[0i8;4];let n=core::cmp::min(count,3);if copy_from_user(b.as_mut_ptr() as *mut _,s,n)!=0{return -EFAULT}b[n]=0;kstrtobool(b.as_ptr(),r)}

macro_rules! from_user {($n:ident,$f:ident,$t:ty,$size:expr)=>{#[no_mangle] pub unsafe extern "C" fn $n(s:*const core::ffi::c_void,count:usize,base:u32,r:*mut $t)->i32{let mut b=[0i8;$size];let n=core::cmp::min(count,$size-1);if copy_from_user(b.as_mut_ptr() as *mut _,s,n)!=0{return -EFAULT}b[n]=0;$f(b.as_ptr(),base,r)}}}
from_user!(kstrtoull_from_user,kstrtoull,u64,18);
from_user!(kstrtoll_from_user,kstrtoll,i64,18);
from_user!(kstrtoul_from_user,_kstrtoul,usize,10);
from_user!(kstrtol_from_user,_kstrtol,isize,10);
from_user!(kstrtouint_from_user,kstrtouint,u32,10);
from_user!(kstrtoint_from_user,kstrtoint,i32,10);
from_user!(kstrtou16_from_user,kstrtou16,u16,6);
from_user!(kstrtos16_from_user,kstrtos16,i16,6);
from_user!(kstrtou8_from_user,kstrtou8,u8,4);
from_user!(kstrtos8_from_user,kstrtos8,i8,4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
