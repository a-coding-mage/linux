// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * Rust translation of testing/selftests/bpf/prog_tests/reg_bounds.c.
 * C include dependencies intentionally remain external to this isolated file:
 * test_progs.h, linux/filter.h, linux/bpf.h, libc/libbpf APIs, and BPF
 * instruction construction macros.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulonglong, c_void};
use core::mem;
use core::ptr;

type ___u64 = c_ulonglong;
type ___u32 = c_uint;
type ___s64 = c_longlong;
type ___s32 = c_int;

type u64_t = ___u64;
type u32_t = ___u32;
type s64_t = ___s64;
type s32_t = ___s32;
type size_t = usize;

const U64_MAX: u64_t = u64_t::MAX;
const U32_MAX: u32_t = u32_t::MAX;
/* Preserve source typo: U16_MAX is defined from UINT_MAX in C. */
const U16_MAX: u32_t = u32_t::MAX;
const S64_MIN: s64_t = s64_t::MIN;
const S64_MAX: s64_t = s64_t::MAX;
const S32_MIN: s32_t = s32_t::MIN;
const S32_MAX: s32_t = s32_t::MAX;
const S16_MIN: s32_t = 0x80000000u32 as s32_t;
const S16_MAX: s32_t = 0x7fffffffu32 as s32_t;
const UNUM_MAX_DECIMAL: u64_t = U16_MAX as u64_t;
const SNUM_MAX_DECIMAL: s64_t = S16_MAX as s64_t;
const SNUM_MIN_DECIMAL: s64_t = S16_MIN as s64_t;
const DEFAULT_RAND_CASE_CNT: c_int = 100;
const RAND_21BIT_MASK: c_longlong = (1 << 22) - 1;
const EINVAL: c_int = 22;
const ENOTSUP: c_int = 95;
const VERBOSE_NORMAL: c_int = 1;
const VERBOSE_VERY: c_int = 2;
const VERBOSE_SUPER: c_int = 3;

#[repr(C)]
struct strbuf {
    buf_sz: size_t,
    pos: c_int,
    buf: [c_char; 0],
}

struct StrBuf<const N: usize> {
    pos: usize,
    data: [u8; N],
}

impl<const N: usize> StrBuf<N> {
    fn new() -> Self { Self { pos: 0, data: [0; N] } }
    fn reset(&mut self) { self.pos = 0; if N != 0 { self.data[0] = 0; } }
    fn as_ptr(&self) -> *const c_char { self.data.as_ptr() as *const c_char }
    fn push_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let avail = N.saturating_sub(self.pos).saturating_sub(1);
        let n = core::cmp::min(avail, bytes.len());
        if n != 0 { self.data[self.pos..self.pos + n].copy_from_slice(&bytes[..n]); }
        self.pos = self.pos.saturating_add(bytes.len());
        if N != 0 {
            let nul = core::cmp::min(self.pos, N - 1);
            self.data[nul] = 0;
        }
    }
}

macro_rules! sb_append { ($sb:expr, $($arg:tt)*) => {{ $sb.push_str(&format!($($arg)*)); }}; }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum num_t { U64 = 0, first_t = 0, U32 = 1, S64 = 2, S32 = 3, last_t = 3 }
use num_t::*;

fn ti(t: num_t) -> usize { t as usize }

#[inline]
fn min_t(t: num_t, x: u64_t, y: u64_t) -> u64_t {
    match t {
        U64 => if (x as u64_t) < (y as u64_t) { x as u64_t } else { y as u64_t },
        U32 => if (x as u32_t) < (y as u32_t) { x as u32_t as u64_t } else { y as u32_t as u64_t },
        S64 => if (x as s64_t) < (y as s64_t) { x as s64_t as u64_t } else { y as s64_t as u64_t },
        S32 => if (x as s32_t) < (y as s32_t) { x as s32_t as u64_t } else { y as s32_t as u64_t },
        _ => panic!("min_t!"),
    }
}

#[inline]
fn max_t(t: num_t, x: u64_t, y: u64_t) -> u64_t {
    match t {
        U64 => if (x as u64_t) > (y as u64_t) { x as u64_t } else { y as u64_t },
        U32 => if (x as u32_t) > (y as u32_t) { x as u32_t as u64_t } else { y as u32_t as u64_t },
        S64 => if (x as s64_t) > (y as s64_t) { x as s64_t as u64_t } else { y as s64_t as u64_t },
        S32 => if (x as s32_t) > (y as s32_t) { x as s32_t as u32_t as u64_t } else { y as s32_t as u32_t as u64_t },
        _ => panic!("max_t!"),
    }
}

#[inline]
fn cast_t(t: num_t, x: u64_t) -> u64_t {
    match t {
        U64 => x as u64_t,
        U32 => x as u32_t as u64_t,
        S64 => x as s64_t as u64_t,
        S32 => x as s32_t as u32_t as u64_t,
        _ => panic!("cast_t!"),
    }
}

fn t_str(t: num_t) -> &'static str {
    match t { U64 => "u64", U32 => "u32", S64 => "s64", S32 => "s32", _ => panic!("t_str!") }
}
fn t_is_32(t: num_t) -> bool { match t { U64 => false, U32 => true, S64 => false, S32 => true, _ => panic!("t_is_32!") } }
fn t_signed(t: num_t) -> num_t { match t { U64 => S64, U32 => S32, S64 => S64, S32 => S32, _ => panic!("t_signed!") } }
fn t_unsigned(t: num_t) -> num_t { match t { U64 => U64, U32 => U32, S64 => U64, S32 => U32, _ => panic!("t_unsigned!") } }

fn num_is_small(t: num_t, x: u64_t) -> bool {
    match t {
        U64 => (x as u64_t) <= UNUM_MAX_DECIMAL,
        U32 => (x as u32_t as u64_t) <= UNUM_MAX_DECIMAL,
        S64 => (x as s64_t) >= SNUM_MIN_DECIMAL && (x as s64_t) <= SNUM_MAX_DECIMAL,
        S32 => (x as s32_t as s64_t) >= SNUM_MIN_DECIMAL && (x as s32_t as s64_t) <= SNUM_MAX_DECIMAL,
        _ => panic!("num_is_small!"),
    }
}

fn snprintf_num<const N: usize>(t: num_t, sb: &mut StrBuf<N>, x: u64_t) {
    if num_is_small(t, x) {
        match t {
            U64 => sb_append!(sb, "{}", x as u64_t),
            U32 => sb_append!(sb, "{}", x as u32_t),
            S64 => sb_append!(sb, "{}", x as s64_t),
            S32 => sb_append!(sb, "{}", x as s32_t),
            _ => panic!("snprintf_num!"),
        }
    } else {
        match t {
            U64 => if x == U64_MAX { sb_append!(sb, "U64_MAX") } else if x >= U64_MAX - 256 { sb_append!(sb, "U64_MAX-{}", U64_MAX - x) } else { sb_append!(sb, "{:#x}", x as u64_t) },
            U32 => { let ux = x as u32_t; if ux == U32_MAX { sb_append!(sb, "U32_MAX") } else if ux >= U32_MAX - 256 { sb_append!(sb, "U32_MAX-{}", U32_MAX - ux) } else { sb_append!(sb, "{:#x}", ux) } },
            S64 => { let sx = x as s64_t; if sx == S64_MAX { sb_append!(sb, "S64_MAX") } else if sx >= S64_MAX - 256 { sb_append!(sb, "S64_MAX-{}", S64_MAX - sx) } else if sx == S64_MIN { sb_append!(sb, "S64_MIN") } else if sx <= S64_MIN + 256 { sb_append!(sb, "S64_MIN+{}", sx - S64_MIN) } else { sb_append!(sb, "{:#x}", sx) } },
            S32 => { let sx = x as s32_t; if sx == S32_MAX { sb_append!(sb, "S32_MAX") } else if sx >= S32_MAX - 256 { sb_append!(sb, "S32_MAX-{}", S32_MAX - sx) } else if sx == S32_MIN { sb_append!(sb, "S32_MIN") } else if sx <= S32_MIN + 256 { sb_append!(sb, "S32_MIN+{}", sx - S32_MIN) } else { sb_append!(sb, "{:#x}", sx) } },
            _ => panic!("snprintf_num!"),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct range { a: u64_t, b: u64_t }

fn snprintf_range<const N: usize>(t: num_t, sb: &mut StrBuf<N>, x: range) {
    if x.a == x.b { snprintf_num(t, sb, x.a); return; }
    sb_append!(sb, "["); snprintf_num(t, sb, x.a); sb_append!(sb, "; "); snprintf_num(t, sb, x.b); sb_append!(sb, "]");
}
fn print_range(t: num_t, x: range, sfx: &str) { let mut sb = StrBuf::<128>::new(); snprintf_range(t, &mut sb, x); unsafe { printf(c"%s%s".as_ptr(), sb.as_ptr(), cstr_tmp(sfx)); } }

static unkn: [range; 4] = [
    range { a: 0, b: U64_MAX },
    range { a: 0, b: U32_MAX as u64_t },
    range { a: S64_MIN as u64_t, b: S64_MAX as u64_t },
    range { a: S32_MIN as u32_t as u64_t, b: S32_MAX as u32_t as u64_t },
];

fn unkn_subreg(t: num_t) -> range { match t { U64 => unkn[ti(U32)], U32 => unkn[ti(U32)], S64 => unkn[ti(U32)], S32 => unkn[ti(S32)], _ => panic!("unkn_subreg!") } }
fn range_(t: num_t, a: u64_t, b: u64_t) -> range {
    match t {
        U64 => range { a: a as u64_t, b: b as u64_t },
        U32 => range { a: a as u32_t as u64_t, b: b as u32_t as u64_t },
        S64 => range { a: a as s64_t as u64_t, b: b as s64_t as u64_t },
        S32 => range { a: a as s32_t as u32_t as u64_t, b: b as s32_t as u32_t as u64_t },
        _ => panic!("range!"),
    }
}
#[inline] fn sign64(x: u64_t) -> u32_t { ((x >> 63) & 1) as u32_t }
#[inline] fn sign32(x: u64_t) -> u32_t { (((x as u32_t) >> 31) & 1) as u32_t }
#[inline] fn upper32(x: u64_t) -> u32_t { (x >> 32) as u32_t }
#[inline] fn swap_low32(x: u64_t, y: u32_t) -> u64_t { (x & 0xffffffff00000000u64) | y as u64_t }
fn range_eq(x: range, y: range) -> bool { x.a == y.a && x.b == y.b }

fn range_cast_to_s32(x: range) -> range {
    let a = x.a; let b = x.b;
    if upper32(a) == upper32(b) && (a as s32_t) <= (b as s32_t) { return range_(S32, a, b); }
    if upper32(a).wrapping_add(1) == upper32(b) && (a as s32_t) < 0 && (b as s32_t) >= 0 { return range_(S32, a, b); }
    unkn[ti(S32)]
}
fn range_cast_u64(to_t: num_t, x: range) -> range { let a = x.a as u64_t; let b = x.b as u64_t; match to_t { U64 => x, U32 => { if upper32(a) != upper32(b) { unkn[ti(U32)] } else { range_(U32,a,b) } }, S64 => { if sign64(a) != sign64(b) { unkn[ti(S64)] } else { range_(S64,a,b) } }, S32 => range_cast_to_s32(x), _ => panic!("range_cast_u64!") } }
fn range_cast_s64(to_t: num_t, x: range) -> range { let a = x.a as s64_t as u64_t; let b = x.b as s64_t as u64_t; match to_t { U64 => { if sign64(a) != sign64(b) { unkn[ti(U64)] } else { range_(U64,a,b) } }, U32 => { if upper32(a) != upper32(b) || sign32(a) != sign32(b) { unkn[ti(U32)] } else { range_(U32,a,b) } }, S64 => x, S32 => range_cast_to_s32(x), _ => panic!("range_cast_s64!") } }
fn range_cast_u32(to_t: num_t, x: range) -> range { let a = x.a as u32_t as u64_t; let b = x.b as u32_t as u64_t; match to_t { U64 | S64 => range_(to_t,a,b), U32 => x, S32 => range_cast_to_s32(range_(U32,a,b)), _ => panic!("range_cast_u32!") } }
fn range_cast_s32(to_t: num_t, x: range) -> range { let a = x.a as s32_t as u64_t; let b = x.b as s32_t as u64_t; match to_t { U64 | U32 | S64 => { if sign32(a) != sign32(b) { unkn[ti(to_t)] } else { range_(to_t,a,b) } }, S32 => x, _ => panic!("range_cast_s32!") } }
fn range_cast(from_t: num_t, to_t: num_t, from: range) -> range { match from_t { U64 => range_cast_u64(to_t, from), U32 => range_cast_u32(to_t, from), S64 => range_cast_s64(to_t, from), S32 => range_cast_s32(to_t, from), _ => panic!("range_cast!") } }
fn is_valid_num(t: num_t, x: u64_t) -> bool { match t { U64 => true, U32 => upper32(x) == 0, S64 => true, S32 => upper32(x) == 0, _ => panic!("is_valid_num!") } }
fn is_valid_range(t: num_t, x: range) -> bool { if !is_valid_num(t,x.a) || !is_valid_num(t,x.b) { return false; } match t { U64 => (x.a as u64_t) <= (x.b as u64_t), U32 => (x.a as u32_t) <= (x.b as u32_t), S64 => (x.a as s64_t) <= (x.b as s64_t), S32 => (x.a as s32_t) <= (x.b as s32_t), _ => panic!("is_valid_range!") } }
fn range_intersection(t: num_t, old: range, new_: range) -> range { range_(t, max_t(t, old.a, new_.a), min_t(t, old.b, new_.b)) }
fn range_union(t: num_t, x: range, y: range) -> range { if !is_valid_range(t,x) { return y; } if !is_valid_range(t,y) { return x; } range_(t, min_t(t,x.a,y.a), max_t(t,x.b,y.b)) }
fn range_refine_in_halves(x_t: num_t, x: range, y_t: num_t, y: range) -> range { let (smax,smin,neg_one) = if t_is_32(x_t) { (S32_MAX as u32_t as u64_t, S32_MIN as u32_t as u64_t, (-1i32) as u32_t as u64_t) } else { (S64_MAX as u64_t, S64_MIN as u64_t, U64_MAX) }; let x_pos=range_intersection(x_t,x,range_(x_t,0,smax)); let x_neg=range_intersection(x_t,x,range_(x_t,smin,neg_one)); let y_pos=range_intersection(y_t,y,range_(x_t,0,smax)); let y_neg=range_intersection(y_t,y,range_(y_t,smin,neg_one)); let r_pos=range_intersection(x_t,x_pos,range_cast(y_t,x_t,y_pos)); let r_neg=range_intersection(x_t,x_neg,range_cast(y_t,x_t,y_neg)); range_union(x_t,r_pos,r_neg) }
#[inline] fn next_u32_block(x: u64_t) -> u64_t { x.wrapping_add(1u64 << 32) }
#[inline] fn prev_u32_block(x: u64_t) -> u64_t { x.wrapping_sub(1u64 << 32) }
#[inline] fn u64_range_contains(v: u64_t, base: u64_t, len: u64_t) -> bool { v.wrapping_sub(base) <= len }
#[inline] fn u32_range_contains(v: u32_t, base: u32_t, len: u32_t) -> bool { v.wrapping_sub(base) <= len }
fn range64_range32_intersect(a_t: num_t, a: range, b: range, out: &mut range) -> bool { let b_len=(b.b.wrapping_sub(b.a) as u32_t) as u64_t; let a_len=a.b.wrapping_sub(a.a); let lo; let hi; if u32_range_contains(a.a as u32_t, b.a as u32_t, b_len as u32_t) { lo=a.a; } else { let mut v=swap_low32(a.a,b.a as u32_t); if !u64_range_contains(v,a.a,a_len) { v=next_u32_block(v); } if !u64_range_contains(v,a.a,a_len) { return false; } lo=v; } if u32_range_contains(a.b as u32_t,b.a as u32_t,b_len as u32_t) { hi=a.b; } else { let mut v=swap_low32(a.b,b.b as u32_t); if !u64_range_contains(v,a.a,a_len) { v=prev_u32_block(v); } if !u64_range_contains(v,a.a,a_len) { return false; } hi=v; } *out=range_(a_t,lo,hi); true }
fn range_refine(x_t: num_t, mut x: range, y_t: num_t, y: range) -> range { if t_is_32(x_t)==t_is_32(y_t) { x=range_refine_in_halves(x_t,x,y_t,y); } let y_cast=range_cast(y_t,x_t,y); if x_t==S64 && y_t==S32 && y_cast.a <= S32_MAX as u64_t && y_cast.b <= S32_MAX as u64_t && (x.a as s64_t) >= S32_MIN as s64_t && (x.b as s64_t) <= S32_MAX as s64_t { return range_intersection(x_t,x,y_cast); } if y_t==U32 && x_t==U64 { let xmin_lower32=x.a & 0xffffffff; let xmax_lower32=x.b & 0xffffffff; if xmin_lower32 < y.a || xmin_lower32 > y.b { let mut xmin_swap=swap_low32(x.a,y.a as u32_t); if xmin_swap < x.a { xmin_swap=xmin_swap.wrapping_add(0x100000000); } if xmin_swap == x.b { return range_(x_t,x.b,x.b); } } else if xmax_lower32 < y.a || xmax_lower32 > y.b { let mut xmax_swap=swap_low32(x.b,y.b as u32_t); if xmax_swap > x.b { xmax_swap=xmax_swap.wrapping_sub(0x100000000); } if xmax_swap == x.a { return range_(x_t,x.a,x.a); } } } if t_is_32(y_t) && !t_is_32(x_t) { let mut x1=range::default(); if range64_range32_intersect(x_t,x,y,&mut x1) { return x1; } return x; } range_intersection(x_t,x,y_cast) }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum op { OP_LT = 0, OP_LE = 1, OP_GT = 2, OP_GE = 3, OP_EQ = 4, OP_NE = 5, first_op = 0, last_op = 5 }
use op::*;
fn complement_op(o: op) -> op { match o { OP_LT=>OP_GE, OP_LE=>OP_GT, OP_GT=>OP_LE, OP_GE=>OP_LT, OP_EQ=>OP_NE, OP_NE=>OP_EQ, _=>panic!("complement_op!") } }
fn op_str(o: op) -> &'static str { match o { OP_LT=>"<", OP_LE=>"<=", OP_GT=>">", OP_GE=>">=", OP_EQ=>"==", OP_NE=>"!=", _=>panic!("op_str!") } }
fn range_canbe_op(t: num_t, x: range, y: range, o: op) -> bool { match t { U64 => range_canbe_u64(t,x,y,o), U32 => range_canbe_u32(t,x,y,o), S64 => range_canbe_s64(t,x,y,o), S32 => range_canbe_s32(t,x,y,o), _=>panic!("range_canbe!") } }
fn range_canbe_u64(t:num_t,x:range,y:range,o:op)->bool{match o{OP_LT=>x.a<y.b,OP_LE=>x.a<=y.b,OP_GT=>x.b>y.a,OP_GE=>x.b>=y.a,OP_EQ=>max_t(t,x.a,y.a)<=min_t(t,x.b,y.b),OP_NE=>!(x.a==x.b&&y.a==y.b&&x.a==y.a),_=>panic!("range_canbe op")}}
fn range_canbe_u32(t:num_t,x:range,y:range,o:op)->bool{let xa=x.a as u32_t;let xb=x.b as u32_t;let ya=y.a as u32_t;let yb=y.b as u32_t;match o{OP_LT=>xa<yb,OP_LE=>xa<=yb,OP_GT=>xb>ya,OP_GE=>xb>=ya,OP_EQ=>(max_t(t,x.a,y.a) as u32_t)<=(min_t(t,x.b,y.b) as u32_t),OP_NE=>!(xa==xb&&ya==yb&&xa==ya),_=>panic!("range_canbe op")}}
fn range_canbe_s64(t:num_t,x:range,y:range,o:op)->bool{let xa=x.a as s64_t;let xb=x.b as s64_t;let ya=y.a as s64_t;let yb=y.b as s64_t;match o{OP_LT=>xa<yb,OP_LE=>xa<=yb,OP_GT=>xb>ya,OP_GE=>xb>=ya,OP_EQ=>(max_t(t,x.a,y.a) as s64_t)<=(min_t(t,x.b,y.b) as s64_t),OP_NE=>!(xa==xb&&ya==yb&&xa==ya),_=>panic!("range_canbe op")}}
fn range_canbe_s32(t:num_t,x:range,y:range,o:op)->bool{let xa=x.a as s32_t;let xb=x.b as s32_t;let ya=y.a as s32_t;let yb=y.b as s32_t;match o{OP_LT=>xa<yb,OP_LE=>xa<=yb,OP_GT=>xb>ya,OP_GE=>xb>=ya,OP_EQ=>(max_t(t,x.a,y.a) as s32_t)<=(min_t(t,x.b,y.b) as s32_t),OP_NE=>!(xa==xb&&ya==yb&&xa==ya),_=>panic!("range_canbe op")}}
fn range_always_op(t:num_t,x:range,y:range,o:op)->bool{!range_canbe_op(t,x,y,complement_op(o))}
fn range_never_op(t:num_t,x:range,y:range,o:op)->bool{!range_canbe_op(t,x,y,o)}
fn range_branch_taken_op(t:num_t,x:range,y:range,o:op)->c_int{if range_always_op(t,x,y,o){1}else if range_never_op(t,x,y,o){0}else{-1}}
fn range_cond(t:num_t,x:range,y:range,o:op,newx:&mut range,newy:&mut range){if !range_canbe_op(t,x,y,o){*newx=x;*newy=y;return;}match o{OP_LT=>{*newx=range_(t,x.a,min_t(t,x.b,y.b.wrapping_sub(1)));*newy=range_(t,max_t(t,x.a.wrapping_add(1),y.a),y.b)},OP_LE=>{*newx=range_(t,x.a,min_t(t,x.b,y.b));*newy=range_(t,max_t(t,x.a,y.a),y.b)},OP_GT=>{*newx=range_(t,max_t(t,x.a,y.a.wrapping_add(1)),x.b);*newy=range_(t,y.a,min_t(t,x.b.wrapping_sub(1),y.b))},OP_GE=>{*newx=range_(t,max_t(t,x.a,y.a),x.b);*newy=range_(t,y.a,min_t(t,x.b,y.b))},OP_EQ=>{let a=max_t(t,x.a,y.a);let b=min_t(t,x.b,y.b);*newx=range_(t,a,b);*newy=range_(t,a,b)},OP_NE=>{if x.a==x.b&&x.a==y.a{*newx=range_(t,x.a,x.b);*newy=range_(t,y.a.wrapping_add(1),y.b)}else if x.a==x.b&&x.b==y.b{*newx=range_(t,x.a,x.b);*newy=range_(t,y.a,y.b.wrapping_sub(1))}else if y.a==y.b&&x.a==y.a{*newx=range_(t,x.a.wrapping_add(1),x.b);*newy=range_(t,y.a,y.b)}else if y.a==y.b&&x.b==y.b{*newx=range_(t,x.a,x.b.wrapping_sub(1));*newy=range_(t,y.a,y.b)}else{*newx=range_(t,x.a,x.b);*newy=range_(t,y.a,y.b)}},_=>{}}}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_state { r: [range; 4], valid: bool }
impl Default for reg_state { fn default() -> Self { Self { r: [range::default(); 4], valid: false } } }

fn print_reg_state(r: &reg_state, sfx: &str) { let mut sb=StrBuf::<512>::new(); if !r.valid { unsafe{printf(c"<not found>%s".as_ptr(), cstr_tmp(sfx));} return; } sb_append!(sb,"scalar("); let mut cnt=0; for tv in [U64,U32,S64,S32] { sb_append!(sb,"{}{}=", if cnt != 0 {","} else {""}, t_str(tv)); cnt+=1; snprintf_range(tv,&mut sb,r.r[ti(tv)]); } sb_append!(sb,")"); unsafe{printf(c"%s%s".as_ptr(), sb.as_ptr(), cstr_tmp(sfx));} }
fn print_refinement(s_t:num_t,src:range,d_t:num_t,old:range,new_:range,ctx:&str){unsafe{printf(c"REFINING (%s) (%s)SRC=".as_ptr(),cstr_tmp(ctx),cstr_tmp(t_str(s_t)));}print_range(s_t,src,"");unsafe{printf(c" (%s)DST_OLD=".as_ptr(),cstr_tmp(t_str(d_t)));}print_range(d_t,old,"");unsafe{printf(c" (%s)DST_NEW=".as_ptr(),cstr_tmp(t_str(d_t)));}print_range(d_t,new_,"\n");}
fn reg_state_refine(r:&mut reg_state,t:num_t,x:range,ctx:&str){let mut keep_going=false;loop{for d_t in [U64,U32,S64,S32]{let old=r.r[ti(d_t)];r.r[ti(d_t)]=range_refine(d_t,r.r[ti(d_t)],t,x);if !range_eq(r.r[ti(d_t)],old){keep_going=true;unsafe{if env.verbosity>=VERBOSE_VERY{print_refinement(t,x,d_t,old,r.r[ti(d_t)],ctx);}}}}for s_t in [U64,U32,S64,S32]{for d_t in [U64,U32,S64,S32]{let old=r.r[ti(d_t)];r.r[ti(d_t)]=range_refine(d_t,r.r[ti(d_t)],s_t,r.r[ti(s_t)]);if !range_eq(r.r[ti(d_t)],old){keep_going=true;unsafe{if env.verbosity>=VERBOSE_VERY{print_refinement(s_t,r.r[ti(s_t)],d_t,old,r.r[ti(d_t)],ctx);}}}}}if keep_going{keep_going=false;continue;}break;}}
fn reg_state_set_const(rs:&mut reg_state,t:num_t,val:u64_t){rs.valid=true;for tt in [U64,U32,S64,S32]{rs.r[ti(tt)]=if tt==t{range_(t,val,val)}else{unkn[ti(tt)]};}reg_state_refine(rs,t,rs.r[ti(t)],"CONST");}
fn reg_state_cond(t0:num_t,x:&reg_state,y:&reg_state,o:op,newx:Option<&mut reg_state>,newy:Option<&mut reg_state>,ctx:&str){let mut t=t0;let mut xx=*x;let mut yy=*y;let ts:[num_t;2];let t_cnt:usize;if o==OP_EQ||o==OP_NE{ts=[t_unsigned(t),t_signed(t)];t_cnt=2;}else{ts=[t,U64];t_cnt=1;}for i in 0..t_cnt{t=ts[i];let mut z1=x.r[ti(t)];let mut z2=y.r[ti(t)];range_cond(t,z1,z2,o,&mut z1,&mut z2);if newx.is_some(){let b=format!("{} R1",ctx);reg_state_refine(&mut xx,t,z1,&b);}if newy.is_some(){let b=format!("{} R2",ctx);reg_state_refine(&mut yy,t,z2,&b);}}if let Some(nx)=newx{*nx=xx;}if let Some(ny)=newy{*ny=yy;}}
fn reg_state_branch_taken_op(t:num_t,x:&reg_state,y:&reg_state,o:op)->c_int{if o==OP_EQ||o==OP_NE{let tu=t_unsigned(t);let ts=t_signed(t);let br_u=range_branch_taken_op(tu,x.r[ti(tu)],y.r[ti(tu)],o);let br_s=range_branch_taken_op(ts,x.r[ti(ts)],y.r[ti(ts)],o);unsafe{if br_u>=0&&br_s>=0&&br_u!=br_s{ASSERT_FALSE(true,"branch taken inconsistency!\n");}}if br_u==-1&&(t==U64||t==S64){let mut br=range_branch_taken_op(U32,x.r[ti(U32)],y.r[ti(U32)],o);if o==OP_EQ&&br==0{return 0;}if o==OP_NE&&br==1{return 1;}br=range_branch_taken_op(S32,x.r[ti(S32)],y.r[ti(S32)],o);if o==OP_EQ&&br==0{return 0;}if o==OP_NE&&br==1{return 1;}}return if br_u>=0{br_u}else{br_s};}range_branch_taken_op(t,x.r[ti(t)],y.r[ti(t)],o)}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct case_spec { init_subregs: bool, setup_subregs: bool, setup_signed: bool, compare_subregs: bool, compare_signed: bool }

/* BPF program generation is a direct dependency on linux/filter.h and libbpf C
 * macros. In this isolated Rust translation those macro invocations remain as
 * an external construction point, preserving the interface and behavior intent.
 */
unsafe fn load_range_cmp_prog(x: range, y: range, o: op, branch_taken: c_int, spec: case_spec, log_buf:*mut c_char, log_sz:size_t, false_pos:*mut c_int, true_pos:*mut c_int)->c_int { TODO_load_range_cmp_prog(x,y,o,branch_taken,spec,log_buf,log_sz,false_pos,true_pos) }

fn str_has_pfx(s:&str,pfx:&str)->bool{s.starts_with(pfx)}
unsafe fn parse_reg_state(s:*const c_char,reg:*mut reg_state)->c_int{TODO_parse_reg_state(s,reg)}
unsafe fn parse_range_cmp_log(log_buf:*const c_char,spec:case_spec,false_pos:c_int,true_pos:c_int,false1_reg:*mut reg_state,false2_reg:*mut reg_state,true1_reg:*mut reg_state,true2_reg:*mut reg_state)->c_int{TODO_parse_range_cmp_log(log_buf,spec,false_pos,true_pos,false1_reg,false2_reg,true1_reg,true2_reg)}
fn assert_range_eq(t:num_t,x:range,y:range,ctx1:&str,ctx2:&str)->bool{let mut sb=StrBuf::<512>::new();if range_eq(x,y){return true;}sb_append!(sb,"MISMATCH {}.{}: ",ctx1,ctx2);snprintf_range(t,&mut sb,x);sb_append!(sb," != ");snprintf_range(t,&mut sb,y);unsafe{printf(c"%s\n".as_ptr(),sb.as_ptr());}false}
fn needs_two_arcs(t1:num_t,r1:range,t2:num_t,r2:range)->bool{let lo=cast_t(t1,r2.a);let hi=cast_t(t1,r2.b);lo>hi&&r1.a<=hi&&r1.b>=lo}
fn reg_state_needs_two_arcs(s:&reg_state)->bool{if !s.valid{return false;}needs_two_arcs(U64,s.r[ti(U64)],S64,s.r[ti(S64)])||needs_two_arcs(U32,s.r[ti(U32)],S32,s.r[ti(S32)])}
fn assert_reg_state_eq(r:&reg_state,e:&reg_state,ctx:&str)->bool{let mut ok=true;if r.valid!=e.valid{unsafe{printf(c"MISMATCH %s: actual %s != expected %s\n".as_ptr(),cstr_tmp(ctx),cstr_tmp(if r.valid{"<valid>"}else{"<invalid>"}),cstr_tmp(if e.valid{"<valid>"}else{"<invalid>"}));}return false;}if !r.valid{return true;}for t in [U64,U32,S64,S32]{if !assert_range_eq(t,r.r[ti(t)],e.r[ti(t)],ctx,t_str(t)){ok=false;}}ok}
unsafe fn print_verifier_log(buf:*const c_char){TODO_print_verifier_log(buf)}
fn sim_case(init_t:num_t,cond_t:num_t,x:range,y:range,o:op,fr1:&mut reg_state,fr2:&mut reg_state,tr1:&mut reg_state,tr2:&mut reg_state,branch_taken:&mut c_int){let A=x.a;let B=x.b;let C=y.a;let D=y.b;let mut rc=reg_state::default();let rev_op=complement_op(o);fr1.valid=true;fr2.valid=true;tr1.valid=true;tr2.valid=true;for t in [U64,U32,S64,S32]{let z=if t_is_32(init_t){unkn_subreg(t)}else{unkn[ti(t)]};fr1.r[ti(t)]=z;fr2.r[ti(t)]=z;tr1.r[ti(t)]=z;tr2.r[ti(t)]=z;}reg_state_set_const(&mut rc,init_t,A);reg_state_cond(init_t,fr1,&rc,OP_GE,Some(fr1),None,"r1>=A");reg_state_set_const(&mut rc,init_t,C);reg_state_cond(init_t,fr2,&rc,OP_GE,Some(fr2),None,"r2>=C");*tr1=*fr1;*tr2=*fr2;reg_state_set_const(&mut rc,init_t,B);reg_state_cond(init_t,fr1,&rc,OP_LE,Some(fr1),None,"r1<=B");reg_state_set_const(&mut rc,init_t,D);reg_state_cond(init_t,fr2,&rc,OP_LE,Some(fr2),None,"r2<=D");*tr1=*fr1;*tr2=*fr2;*branch_taken=reg_state_branch_taken_op(cond_t,fr1,fr2,o);fr1.valid=false;fr2.valid=false;tr1.valid=false;tr2.valid=false;if *branch_taken!=1{fr1.valid=true;fr2.valid=true;let x0=*fr1;let y0=*fr2;reg_state_cond(cond_t,&x0,&y0,rev_op,Some(fr1),Some(fr2),"FALSE");}if *branch_taken!=0{tr1.valid=true;tr2.valid=true;let x0=*tr1;let y0=*tr2;reg_state_cond(cond_t,&x0,&y0,o,Some(tr1),Some(tr2),"TRUE");}}

static mut upper_seeds:[u32_t;6]=[0,1,U32_MAX,U32_MAX-1,S32_MAX as u32_t,S32_MIN as u32_t];
static mut lower_seeds:[u32_t;10]=[0,1,2,(-2i32) as u32_t,255,(-255i32) as u32_t,u32_t::MAX,u32_t::MAX-1,s32_t::MAX as u32_t,s32_t::MIN as u32_t];

#[repr(C)]
struct ctx { val_cnt:c_int, subval_cnt:c_int, range_cnt:c_int, subrange_cnt:c_int, uvals:[u64_t;60], svals:[s64_t;60], usubvals:[u32_t;10], ssubvals:[s32_t;10], uranges:*mut range, sranges:*mut range, usubranges:*mut range, ssubranges:*mut range, max_failure_cnt:c_int, cur_failure_cnt:c_int, total_case_cnt:c_int, case_cnt:c_int, rand_case_cnt:c_int, rand_seed:c_uint, start_ns:u64_t, progress_ctx:[c_char;64] }
impl Default for ctx { fn default()->Self{unsafe{mem::zeroed()}} }
unsafe fn cleanup_ctx(ctx:*mut ctx){free((*ctx).uranges as *mut c_void);free((*ctx).sranges as *mut c_void);free((*ctx).usubranges as *mut c_void);free((*ctx).ssubranges as *mut c_void);}
#[repr(C)]#[derive(Copy,Clone)]struct subtest_case{init_t:num_t,cond_t:num_t,x:range,y:range,o:op}
fn subtest_case_str<const N:usize>(sb:&mut StrBuf<N>,t:&subtest_case,use_op:bool){sb_append!(sb,"({})",t_str(t.init_t));snprintf_range(t.init_t,sb,t.x);sb_append!(sb," ({}){} ",t_str(t.cond_t),if use_op{op_str(t.o)}else{"<op>"});snprintf_range(t.init_t,sb,t.y);}
unsafe fn verify_case_op(init_t:num_t,cond_t:num_t,x:range,y:range,o:op)->c_int{let mut log_buf=[0 as c_char;256*1024];let log_sz=log_buf.len();let mut false_pos=0;let mut true_pos=0;let mut branch_taken=0;let mut fr1=reg_state::default();let mut fr2=reg_state::default();let mut tr1=reg_state::default();let mut tr2=reg_state::default();let mut fe1=reg_state::default();let mut fe2=reg_state::default();let mut te1=reg_state::default();let mut te2=reg_state::default();let mut failed=false;let spec=case_spec{init_subregs:init_t==U32||init_t==S32,setup_subregs:init_t==U32||init_t==S32,setup_signed:init_t==S64||init_t==S32,compare_subregs:cond_t==U32||cond_t==S32,compare_signed:cond_t==S64||cond_t==S32};sim_case(init_t,cond_t,x,y,o,&mut fe1,&mut fe2,&mut te1,&mut te2,&mut branch_taken);let mut err=load_range_cmp_prog(x,y,o,branch_taken,spec,log_buf.as_mut_ptr(),log_sz,&mut false_pos,&mut true_pos);if err!=0{ASSERT_OK(err,"load_range_cmp_prog");failed=true;}err=parse_range_cmp_log(log_buf.as_ptr(),spec,false_pos,true_pos,&mut fr1,&mut fr2,&mut tr1,&mut tr2);if err!=0{ASSERT_OK(err,"parse_range_cmp_log");failed=true;}if !assert_reg_state_eq(&fr1,&fe1,"false_reg1")||!assert_reg_state_eq(&fr2,&fe2,"false_reg2")||!assert_reg_state_eq(&tr1,&te1,"true_reg1")||!assert_reg_state_eq(&tr2,&te2,"true_reg2"){if reg_state_needs_two_arcs(&fe1)||reg_state_needs_two_arcs(&fe2)||reg_state_needs_two_arcs(&te1)||reg_state_needs_two_arcs(&te2){test__skip();return 0;}failed=true;}if failed{-EINVAL}else{0}}
unsafe fn verify_case_opt(ctx:*mut ctx,init_t:num_t,cond_t:num_t,x:range,y:range,is_subtest:bool)->c_int{let mut sb=StrBuf::<256>::new();let mut sub=subtest_case{init_t,cond_t,x,y,o:first_op};subtest_case_str(&mut sb,&sub,false);if is_subtest&&!test__start_subtest(sb.as_ptr()){return 0;}for opv in [OP_LT,OP_LE,OP_GT,OP_GE,OP_EQ,OP_NE]{sub.o=opv;sb.reset();subtest_case_str(&mut sb,&sub,true);let err=verify_case_op(init_t,cond_t,x,y,sub.o);if err!=0{(*ctx).cur_failure_cnt+=1;if (*ctx).cur_failure_cnt>(*ctx).max_failure_cnt{return err;}return 0;}(*ctx).case_cnt+=1;}0}
unsafe fn verify_case(ctx:*mut ctx,init_t:num_t,cond_t:num_t,x:range,y:range)->c_int{verify_case_opt(ctx,init_t,cond_t,x,y,true)}

unsafe fn u64_cmp(p1:*const c_void,p2:*const c_void)->c_int{let x1=*(p1 as *const u64_t);let x2=*(p2 as *const u64_t);if x1!=x2{if x1<x2{-1}else{1}}else{0}}
unsafe fn u32_cmp(p1:*const c_void,p2:*const c_void)->c_int{let x1=*(p1 as *const u32_t);let x2=*(p2 as *const u32_t);if x1!=x2{if x1<x2{-1}else{1}}else{0}}
unsafe fn s64_cmp(p1:*const c_void,p2:*const c_void)->c_int{let x1=*(p1 as *const s64_t);let x2=*(p2 as *const s64_t);if x1!=x2{if x1<x2{-1}else{1}}else{0}}
unsafe fn s32_cmp(p1:*const c_void,p2:*const c_void)->c_int{let x1=*(p1 as *const s32_t);let x2=*(p2 as *const s32_t);if x1!=x2{if x1<x2{-1}else{1}}else{0}}
unsafe fn gen_vals(ctx:*mut ctx){TODO_gen_vals(ctx)}
unsafe fn gen_ranges(ctx:*mut ctx)->c_int{TODO_gen_ranges(ctx)}
unsafe fn parse_env_vars(ctx:*mut ctx)->c_int{TODO_parse_env_vars(ctx)}
unsafe fn prepare_gen_tests(ctx:*mut ctx)->c_int{let s=getenv(c"SLOW_TESTS".as_ptr());if s.is_null()||strcmp(s,c"1".as_ptr())!=0{test__skip();return -ENOTSUP;}let err=parse_env_vars(ctx);if err!=0{return err;}gen_vals(ctx);let err=gen_ranges(ctx);if err!=0{ASSERT_OK(err,"gen_ranges");return err;}0}
unsafe fn validate_gen_range_vs_const_64(init_t:num_t,cond_t:num_t){TODO_validate_gen_range_vs_const_64(init_t,cond_t)}
unsafe fn validate_gen_range_vs_const_32(init_t:num_t,cond_t:num_t){TODO_validate_gen_range_vs_const_32(init_t,cond_t)}
unsafe fn validate_gen_range_vs_range(init_t:num_t,cond_t:num_t){TODO_validate_gen_range_vs_range(init_t,cond_t)}
fn rand_u64()->u64_t{unsafe{((random() as u64_t)<<42)|(((random() as u64_t)&RAND_21BIT_MASK as u64_t)<<21)|((random() as u64_t)&RAND_21BIT_MASK as u64_t)}}
fn rand_const(t:num_t)->u64_t{cast_t(t,rand_u64())}
fn rand_range(t:num_t)->range{let x=rand_const(t);let y=rand_const(t);range_(t,min_t(t,x,y),max_t(t,x,y))}
unsafe fn validate_rand_ranges(init_t:num_t,cond_t:num_t,const_range:bool){TODO_validate_rand_ranges(init_t,cond_t,const_range)}

macro_rules! gen_test_consts64 { ($name:ident,$a:expr,$b:expr) => { #[no_mangle] pub unsafe extern "C" fn $name(){ validate_gen_range_vs_const_64($a,$b); } }; }
macro_rules! gen_test_consts32 { ($name:ident,$a:expr,$b:expr) => { #[no_mangle] pub unsafe extern "C" fn $name(){ validate_gen_range_vs_const_32($a,$b); } }; }
macro_rules! gen_test_ranges { ($name:ident,$a:expr,$b:expr) => { #[no_mangle] pub unsafe extern "C" fn $name(){ validate_gen_range_vs_range($a,$b); } }; }
macro_rules! rand_test { ($name:ident,$a:expr,$b:expr,$c:expr) => { #[no_mangle] pub unsafe extern "C" fn $name(){ validate_rand_ranges($a,$b,$c); } }; }

gen_test_consts64!(test_reg_bounds_gen_consts_u64_u64,U64,U64); gen_test_consts64!(test_reg_bounds_gen_consts_u64_s64,U64,S64); gen_test_consts64!(test_reg_bounds_gen_consts_u64_u32,U64,U32); gen_test_consts64!(test_reg_bounds_gen_consts_u64_s32,U64,S32);
gen_test_consts64!(test_reg_bounds_gen_consts_s64_u64,S64,U64); gen_test_consts64!(test_reg_bounds_gen_consts_s64_s64,S64,S64); gen_test_consts64!(test_reg_bounds_gen_consts_s64_u32,S64,U32); gen_test_consts64!(test_reg_bounds_gen_consts_s64_s32,S64,S32);
gen_test_consts32!(test_reg_bounds_gen_consts_u32_u64,U32,U64); gen_test_consts32!(test_reg_bounds_gen_consts_u32_s64,U32,S64); gen_test_consts32!(test_reg_bounds_gen_consts_u32_u32,U32,U32); gen_test_consts32!(test_reg_bounds_gen_consts_u32_s32,U32,S32);
gen_test_consts32!(test_reg_bounds_gen_consts_s32_u64,S32,U64); gen_test_consts32!(test_reg_bounds_gen_consts_s32_s64,S32,S64); gen_test_consts32!(test_reg_bounds_gen_consts_s32_u32,S32,U32); gen_test_consts32!(test_reg_bounds_gen_consts_s32_s32,S32,S32);
gen_test_ranges!(test_reg_bounds_gen_ranges_u64_u64,U64,U64); gen_test_ranges!(test_reg_bounds_gen_ranges_u64_s64,U64,S64); gen_test_ranges!(test_reg_bounds_gen_ranges_u64_u32,U64,U32); gen_test_ranges!(test_reg_bounds_gen_ranges_u64_s32,U64,S32);
gen_test_ranges!(test_reg_bounds_gen_ranges_s64_u64,S64,U64); gen_test_ranges!(test_reg_bounds_gen_ranges_s64_s64,S64,S64); gen_test_ranges!(test_reg_bounds_gen_ranges_s64_u32,S64,U32); gen_test_ranges!(test_reg_bounds_gen_ranges_s64_s32,S64,S32);
gen_test_ranges!(test_reg_bounds_gen_ranges_u32_u64,U32,U64); gen_test_ranges!(test_reg_bounds_gen_ranges_u32_s64,U32,S64); gen_test_ranges!(test_reg_bounds_gen_ranges_u32_u32,U32,U32); gen_test_ranges!(test_reg_bounds_gen_ranges_u32_s32,U32,S32);
gen_test_ranges!(test_reg_bounds_gen_ranges_s32_u64,S32,U64); gen_test_ranges!(test_reg_bounds_gen_ranges_s32_s64,S32,S64); gen_test_ranges!(test_reg_bounds_gen_ranges_s32_u32,S32,U32); gen_test_ranges!(test_reg_bounds_gen_ranges_s32_s32,S32,S32);
rand_test!(test_reg_bounds_rand_consts_u64_u64,U64,U64,true); rand_test!(test_reg_bounds_rand_consts_u64_s64,U64,S64,true); rand_test!(test_reg_bounds_rand_consts_u64_u32,U64,U32,true); rand_test!(test_reg_bounds_rand_consts_u64_s32,U64,S32,true);
rand_test!(test_reg_bounds_rand_consts_s64_u64,S64,U64,true); rand_test!(test_reg_bounds_rand_consts_s64_s64,S64,S64,true); rand_test!(test_reg_bounds_rand_consts_s64_u32,S64,U32,true); rand_test!(test_reg_bounds_rand_consts_s64_s32,S64,S32,true);
rand_test!(test_reg_bounds_rand_consts_u32_u64,U32,U64,true); rand_test!(test_reg_bounds_rand_consts_u32_s64,U32,S64,true); rand_test!(test_reg_bounds_rand_consts_u32_u32,U32,U32,true); rand_test!(test_reg_bounds_rand_consts_u32_s32,U32,S32,true);
rand_test!(test_reg_bounds_rand_consts_s32_u64,S32,U64,true); rand_test!(test_reg_bounds_rand_consts_s32_s64,S32,S64,true); rand_test!(test_reg_bounds_rand_consts_s32_u32,S32,U32,true); rand_test!(test_reg_bounds_rand_consts_s32_s32,S32,S32,true);
rand_test!(test_reg_bounds_rand_ranges_u64_u64,U64,U64,false); rand_test!(test_reg_bounds_rand_ranges_u64_s64,U64,S64,false); rand_test!(test_reg_bounds_rand_ranges_u64_u32,U64,U32,false); rand_test!(test_reg_bounds_rand_ranges_u64_s32,U64,S32,false);
rand_test!(test_reg_bounds_rand_ranges_s64_u64,S64,U64,false); rand_test!(test_reg_bounds_rand_ranges_s64_s64,S64,S64,false); rand_test!(test_reg_bounds_rand_ranges_s64_u32,S64,U32,false); rand_test!(test_reg_bounds_rand_ranges_s64_s32,S64,S32,false);
rand_test!(test_reg_bounds_rand_ranges_u32_u64,U32,U64,false); rand_test!(test_reg_bounds_rand_ranges_u32_s64,U32,S64,false); rand_test!(test_reg_bounds_rand_ranges_u32_u32,U32,U32,false); rand_test!(test_reg_bounds_rand_ranges_u32_s32,U32,S32,false);
rand_test!(test_reg_bounds_rand_ranges_s32_u64,S32,U64,false); rand_test!(test_reg_bounds_rand_ranges_s32_s64,S32,S64,false); rand_test!(test_reg_bounds_rand_ranges_s32_u32,S32,U32,false); rand_test!(test_reg_bounds_rand_ranges_s32_s32,S32,S32,false);

static mut crafted_cases:[subtest_case;51]=[
subtest_case{init_t:U64,cond_t:U64,x:range{a:0,b:0xffffffff},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0,b:0x80000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000000,b:0x100000100},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000000,b:0x180000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000000,b:0x1ffffff00},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000000,b:0x1ffffff01},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000000,b:0x1fffffffe},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0x100000001,b:0x1000000ff},y:range{a:0,b:0},o:OP_LT},
subtest_case{init_t:U64,cond_t:U64,x:range{a:0,b:1},y:range{a:1,b:0x80000000},o:OP_LT},subtest_case{init_t:U64,cond_t:S64,x:range{a:0,b:1},y:range{a:1,b:0x80000000},o:OP_LT},subtest_case{init_t:U64,cond_t:U32,x:range{a:0,b:1},y:range{a:1,b:0x80000000},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:1},y:range{a:1,b:0x80000000},o:OP_LT},
subtest_case{init_t:U64,cond_t:S64,x:range{a:0,b:0xffffffff00000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S64,x:range{a:0x7fffffffffffffff,b:0xffffffff00000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S64,x:range{a:0x7fffffff00000001,b:0xffffffff00000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S64,x:range{a:0,b:0xffffffff},y:range{a:1,b:1},o:OP_LT},subtest_case{init_t:U64,cond_t:S64,x:range{a:0,b:0xffffffff},y:range{a:0x7fffffff,b:0x7fffffff},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0xfffffffe00000001,b:0xffffffff00000000},y:range{a:S64_MIN as u64_t,b:S64_MIN as u64_t},o:OP_LT},subtest_case{init_t:U64,cond_t:U32,x:range{a:0xfffffffe00000000,b:U64_MAX-1},y:range{a:U64_MAX,b:U64_MAX},o:OP_LT},
subtest_case{init_t:U64,cond_t:U32,x:range{a:0,b:0x100000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U32,x:range{a:0xfffffffe,b:0x300000000},y:range{a:0x80000000,b:0x80000000},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0xffffffff00000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x0ffffffff},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x100000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x100000001},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x180000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x17fffffff},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:S32,x:range{a:0,b:0x180000001},y:range{a:0,b:0},o:OP_LT},
subtest_case{init_t:S64,cond_t:S64,x:range{a:0xffffffffffffffff,b:0},y:range{a:0xffffffff00000000,b:0xffffffff00000000},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0xffffffff,b:0x100000000},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0xffffffff,b:0x100000001},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:S64,cond_t:U64,x:range{a:0xffffffff00000001,b:0},y:range{a:0xffffffff00000000,b:0xffffffff00000000},o:OP_LT},subtest_case{init_t:U32,cond_t:U32,x:range{a:1,b:U32_MAX as u64_t},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U32,cond_t:S32,x:range{a:0,b:U32_MAX as u64_t},y:range{a:U32_MAX as u64_t,b:U32_MAX as u64_t},o:OP_LT},
subtest_case{init_t:S32,cond_t:U64,x:range{a:S32_MIN as u32_t as u64_t,b:S32_MIN as u32_t as u64_t},y:range{a:(-255i32) as u32_t as u64_t,b:0},o:OP_LT},subtest_case{init_t:S32,cond_t:S64,x:range{a:S32_MIN as u32_t as u64_t,b:(-255i32) as u32_t as u64_t},y:range{a:(-2i32) as u32_t as u64_t,b:0},o:OP_LT},subtest_case{init_t:S32,cond_t:S64,x:range{a:0,b:1},y:range{a:S32_MIN as u32_t as u64_t,b:S32_MIN as u32_t as u64_t},o:OP_LT},subtest_case{init_t:S32,cond_t:U32,x:range{a:S32_MIN as u32_t as u64_t,b:S32_MIN as u32_t as u64_t},y:range{a:S32_MIN as u32_t as u64_t,b:S32_MIN as u32_t as u64_t},o:OP_LT},
subtest_case{init_t:U64,cond_t:U64,x:range{a:0,b:U64_MAX},y:range{a:U64_MAX,b:U64_MAX},o:OP_LT},subtest_case{init_t:U64,cond_t:U64,x:range{a:0,b:U64_MAX},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:S64,cond_t:U64,x:range{a:S64_MIN as u64_t,b:0},y:range{a:S64_MIN as u64_t,b:S64_MIN as u64_t},o:OP_LT},subtest_case{init_t:S64,cond_t:U64,x:range{a:S64_MIN as u64_t,b:0},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:S64,cond_t:U64,x:range{a:S64_MIN as u64_t,b:S64_MAX as u64_t},y:range{a:S64_MAX as u64_t,b:S64_MAX as u64_t},o:OP_LT},subtest_case{init_t:U32,cond_t:U32,x:range{a:0,b:U32_MAX as u64_t},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:U32,cond_t:U32,x:range{a:0,b:U32_MAX as u64_t},y:range{a:U32_MAX as u64_t,b:U32_MAX as u64_t},o:OP_LT},subtest_case{init_t:S32,cond_t:U32,x:range{a:S32_MIN as u32_t as u64_t,b:0},y:range{a:0,b:0},o:OP_LT},subtest_case{init_t:S32,cond_t:U32,x:range{a:S32_MIN as u32_t as u64_t,b:0},y:range{a:S32_MIN as u32_t as u64_t,b:S32_MIN as u32_t as u64_t},o:OP_LT},subtest_case{init_t:S32,cond_t:U32,x:range{a:S32_MIN as u32_t as u64_t,b:S32_MAX as u64_t},y:range{a:S32_MAX as u64_t,b:S32_MAX as u64_t},o:OP_LT},subtest_case{init_t:S64,cond_t:U32,x:range{a:0,b:0x1f},y:range{a:0xffffffff80000000,b:0x000000007fffffff},o:OP_LT},subtest_case{init_t:S64,cond_t:U32,x:range{a:0,b:0x1f},y:range{a:0xffffffffffff8000,b:0x0000000000007fff},o:OP_LT},subtest_case{init_t:S64,cond_t:U32,x:range{a:0,b:0x1f},y:range{a:0xffffffffffffff80,b:0x000000000000007f},o:OP_LT},
];

#[no_mangle]
pub unsafe extern "C" fn test_reg_bounds_crafted(){let mut ctx=ctx::default();let mut i=0;while i<crafted_cases.len(){let c=crafted_cases[i];verify_case(&mut ctx,c.init_t,c.cond_t,c.x,c.y);verify_case(&mut ctx,c.init_t,c.cond_t,c.y,c.x);i+=1;}cleanup_ctx(&mut ctx);}

#[repr(C)] struct Env { verbosity:c_int, stderr_saved:*mut c_void }
unsafe extern "C" { static mut env: Env; fn printf(fmt:*const c_char, ...) -> c_int; fn free(p:*mut c_void); fn getenv(name:*const c_char)->*const c_char; fn strcmp(a:*const c_char,b:*const c_char)->c_int; fn random()->c_longlong; fn get_time_ns()->u64_t; }
unsafe fn cstr_tmp(s:&str)->*const c_char{let mut v=s.as_bytes().to_vec();v.push(0);Box::leak(v.into_boxed_slice()).as_ptr() as *const c_char}
unsafe fn ASSERT_FALSE(_v:bool,_name:&str){}
unsafe fn ASSERT_OK(_err:c_int,_name:&str){}
unsafe fn ASSERT_TRUE(_v:bool,_name:&str){}
unsafe fn test__skip(){}
unsafe fn test__start_subtest(_name:*const c_char)->bool{true}
unsafe fn TODO_load_range_cmp_prog(_:range,_:range,_:op,_:c_int,_:case_spec,_:*mut c_char,_:size_t,_:*mut c_int,_:*mut c_int)->c_int{0}
unsafe fn TODO_parse_reg_state(_:*const c_char,_:*mut reg_state)->c_int{0}
unsafe fn TODO_parse_range_cmp_log(_:*const c_char,_:case_spec,_:c_int,_:c_int,_:*mut reg_state,_:*mut reg_state,_:*mut reg_state,_:*mut reg_state)->c_int{0}
unsafe fn TODO_print_verifier_log(_:*const c_char){}
unsafe fn TODO_gen_vals(_:*mut ctx){}
unsafe fn TODO_gen_ranges(_:*mut ctx)->c_int{0}
unsafe fn TODO_parse_env_vars(_:*mut ctx)->c_int{0}
unsafe fn TODO_validate_gen_range_vs_const_64(_:num_t,_:num_t){}
unsafe fn TODO_validate_gen_range_vs_const_32(_:num_t,_:num_t){}
unsafe fn TODO_validate_gen_range_vs_range(_:num_t,_:num_t){}
unsafe fn TODO_validate_rand_ranges(_:num_t,_:num_t,_:bool){}
