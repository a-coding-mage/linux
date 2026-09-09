// SPDX-License-Identifier: GPL-2.0-or-later
/*

   fp_arith.c: floating-point math routines for the Linux-m68k
   floating point emulator.

   Copyright (c) 1998-1999 David Huggins-Daines.

   Somewhat based on the AlphaLinux floating point emulator, by David
   Mosberger-Tang.

 */

// Dependencies supplied by fp_emu.h, multi_arith.h, and fp_arith.h.

pub static mut fp_QNaN: fp_ext = fp_ext {
    exp: 0x7fff,
    mant: fp_mant64 { m64: !0 },
    ..unsafe { core::mem::zeroed() }
};

pub static mut fp_Inf: fp_ext = fp_ext {
    exp: 0x7fff,
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn fp_fabs(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(PINSTR, "fabs\n");
    fp_monadic_check(dest, src);
    (*dest).sign = 0;
    dest
}

pub unsafe fn fp_fneg(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(PINSTR, "fneg\n");
    fp_monadic_check(dest, src);
    (*dest).sign = !(*dest).sign;
    dest
}

pub unsafe fn fp_fadd(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    let mut diff: i32;
    dprint(PINSTR, "fadd\n");
    fp_dyadic_check(dest, src);
    if IS_INF(dest) {
        if IS_INF(src) && (*src).sign != (*dest).sign { fp_set_nan(dest); }
        return dest;
    }
    if IS_INF(src) { fp_copy_ext(dest, src); return dest; }
    if IS_ZERO(dest) {
        if IS_ZERO(src) {
            if (*src).sign != (*dest).sign {
                (*dest).sign = if FPDATA.rnd == FPCR_ROUND_RM { 1 } else { 0 };
            }
        } else { fp_copy_ext(dest, src); }
        return dest;
    }
    (*dest).lowmant = 0; (*src).lowmant = 0;
    diff = (*dest).exp - (*src).exp;
    if diff > 0 { fp_denormalize(src, diff); } else { diff = -diff; if diff > 0 { fp_denormalize(dest, diff); } }
    if (*dest).sign == (*src).sign {
        if fp_addmant(dest, src) && !fp_addcarry(dest) { return dest; }
    } else if (*dest).mant.m64 < (*src).mant.m64 {
        fp_submant(dest, src, dest); (*dest).sign = !(*dest).sign;
    } else { fp_submant(dest, dest, src); }
    dest
}

pub unsafe fn fp_fsub(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(PINSTR, "fsub "); (*src).sign = !(*src).sign; fp_fadd(dest, src)
}

pub unsafe fn fp_fcmp(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(PINSTR, "fcmp "); FPDATA.temp[1] = *dest; (*src).sign = !(*src).sign; fp_fadd(&mut FPDATA.temp[1], src)
}

pub unsafe fn fp_ftst(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    dprint(PINSTR, "ftst\n"); let _ = dest; src
}

pub unsafe fn fp_fmul(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    let mut temp: fp_mant128 = core::mem::zeroed();
    let mut exp: i32;
    dprint(PINSTR, "fmul\n"); fp_dyadic_check(dest, src);
    (*dest).sign = (*src).sign ^ (*dest).sign;
    if IS_INF(dest) { if IS_ZERO(src) { fp_set_nan(dest); } return dest; }
    if IS_INF(src) { if IS_ZERO(dest) { fp_set_nan(dest); } else { fp_copy_ext(dest, src); } return dest; }
    if IS_ZERO(dest) || IS_ZERO(src) { (*dest).exp=0; (*dest).mant.m64=0; (*dest).lowmant=0; return dest; }
    exp = (*dest).exp + (*src).exp - 0x3ffe;
    if (*dest).mant.m32[0] as i32 >= 0 { exp -= fp_overnormalize(dest); }
    if (*src).mant.m32[0] as i32 >= 0 { exp -= fp_overnormalize(src); }
    fp_multiplymant(&mut temp, dest, src);
    if temp.m32[0] as i32 > 0 { exp -= 1; fp_putmant128(dest, &mut temp, 1); } else { fp_putmant128(dest, &mut temp, 0); }
    if exp >= 0x7fff { fp_set_ovrflw(dest); return dest; }
    (*dest).exp=exp; if exp < 0 { fp_set_sr(FPSR_EXC_UNFL); fp_denormalize(dest, -exp); } dest
}

pub unsafe fn fp_fdiv(dest: *mut fp_ext, src: *mut fp_ext) -> *mut fp_ext {
    let mut temp: fp_mant128 = core::mem::zeroed(); let mut exp: i32;
    dprint(PINSTR, "fdiv\n"); fp_dyadic_check(dest, src); (*dest).sign=(*src).sign ^ (*dest).sign;
    if IS_INF(dest) { if IS_INF(src) { fp_set_nan(dest); } return dest; }
    if IS_INF(src) { (*dest).exp=0; (*dest).mant.m64=0; (*dest).lowmant=0; return dest; }
    if IS_ZERO(dest) { if IS_ZERO(src) { fp_set_nan(dest); } return dest; }
    if IS_ZERO(src) { fp_set_sr(FPSR_EXC_DZ); (*dest).exp=0x7fff; (*dest).mant.m64=0; return dest; }
    exp=(*dest).exp-(*src).exp+0x3fff;
    if (*dest).mant.m32[0] as i32 >= 0 { exp-=fp_overnormalize(dest); } if (*src).mant.m32[0] as i32 >= 0 { exp-=fp_overnormalize(src); }
    fp_dividemant(&mut temp,dest,src); if temp.m32[0]==0 { exp-=1; fp_putmant128(dest,&mut temp,32); } else { fp_putmant128(dest,&mut temp,31); }
    if exp>=0x7fff { fp_set_ovrflw(dest); return dest; } (*dest).exp=exp; if exp<0 { fp_set_sr(FPSR_EXC_UNFL); fp_denormalize(dest,-exp); } dest
}

pub unsafe fn fp_fsglmul(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext { let mut exp:i32; dprint(PINSTR,"fsglmul\n"); fp_dyadic_check(dest,src); (*dest).sign=(*src).sign^(*dest).sign; if IS_INF(dest){if IS_ZERO(src){fp_set_nan(dest)} return dest} if IS_INF(src){if IS_ZERO(dest){fp_set_nan(dest)}else{fp_copy_ext(dest,src)} return dest} if IS_ZERO(dest)||IS_ZERO(src){(*dest).exp=0;(*dest).mant.m64=0;(*dest).lowmant=0;return dest} exp=(*dest).exp+(*src).exp-0x3ffe; fp_mul64((*dest).mant.m32[0],(*dest).mant.m32[1],(*dest).mant.m32[0]&0xffffff00,(*src).mant.m32[0]&0xffffff00); if exp>=0x7fff{fp_set_ovrflw(dest);return dest} (*dest).exp=exp;if exp<0{fp_set_sr(FPSR_EXC_UNFL);fp_denormalize(dest,-exp)} dest }

pub unsafe fn fp_fsgldiv(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext { let mut exp:i32; let (mut quot,mut rem):(u64,u64); dprint(PINSTR,"fsgldiv\n"); fp_dyadic_check(dest,src); (*dest).sign=(*src).sign^(*dest).sign; if IS_INF(dest){if IS_INF(src){fp_set_nan(dest)}return dest} if IS_INF(src){(*dest).exp=0;(*dest).mant.m64=0;(*dest).lowmant=0;return dest} if IS_ZERO(dest){if IS_ZERO(src){fp_set_nan(dest)}return dest} if IS_ZERO(src){fp_set_sr(FPSR_EXC_DZ);(*dest).exp=0x7fff;(*dest).mant.m64=0;return dest} exp=(*dest).exp-(*src).exp+0x3fff;(*dest).mant.m32[0]&=0xffffff00;(*src).mant.m32[0]&=0xffffff00; if (*dest).mant.m32[0]>=(*src).mant.m32[0]{fp_sub64((*dest).mant,(*src).mant);fp_div64(&mut quot,&mut rem,(*dest).mant.m32[0],0,(*src).mant.m32[0]);(*dest).mant.m32[0]=0x80000000|(quot>>1) as u32;(*dest).mant.m32[1]=(quot&1) as u32|rem as u32}else{fp_div64(&mut quot,&mut rem,(*dest).mant.m32[0],0,(*src).mant.m32[0]);(*dest).mant.m32[0]=quot as u32;(*dest).mant.m32[1]=rem as u32;exp-=1} if exp>=0x7fff{fp_set_ovrflw(dest);return dest}(*dest).exp=exp;if exp<0{fp_set_sr(FPSR_EXC_UNFL);fp_denormalize(dest,-exp)}dest }

// The remaining routines retain the original control flow and call the shared emulator helpers.
unsafe fn fp_roundint(dest:*mut fp_ext,mode:i32){ let oldmant=(*dest).mant; let mut mask:u32; if fp_normalize_ext(dest)==0{return} if IS_INF(dest)||IS_ZERO(dest){return} match (*dest).exp { 0..=0x3ffe=>{(*dest).mant.m64=0},0x3fff..=0x401e=>{(*dest).mant.m32[0]&=0xffffffffu32 << (0x401e-(*dest).exp);(*dest).mant.m32[1]=0;if oldmant.m64==(*dest).mant.m64{return}},0x401f..=0x403e=>{(*dest).mant.m32[1]&=0xffffffffu32 << (0x403e-(*dest).exp);if oldmant.m32[1]==(*dest).mant.m32[1]{return}},_=>(return)} fp_set_sr(FPSR_EXC_INEX2); match mode { FPCR_ROUND_RN=>{match (*dest).exp {0..=0x3ffd=>return,0x3ffe=>if oldmant.m64==(1u64<<63){return},0x3fff..=0x401d=>{mask=1<<(0x401d-(*dest).exp);if oldmant.m32[0]&mask==0{return}},_=>(return)}},FPCR_ROUND_RZ=>return,_=>if (*dest).sign^(mode-FPCR_ROUND_RM)==0{return}} match (*dest).exp {0..=0x3ffe=>{(*dest).exp=0x3fff;(*dest).mant.m64=1u64<<63},0x3fff..=0x401e=>{mask=1<<(0x401e-(*dest).exp);(*dest).mant.m32[0]=(*dest).mant.m32[0].wrapping_add(mask);if (*dest).mant.m32[0]==0{(*dest).mant.m32[0]=0x80000000;(*dest).exp+=1}},0x401f..=0x403e=>{mask=1<<(0x403e-(*dest).exp);(*dest).mant.m32[1]=(*dest).mant.m32[1].wrapping_add(mask);if (*dest).mant.m32[1]==0{(*dest).mant.m32[0]=(*dest).mant.m32[0].wrapping_add(1);if (*dest).mant.m32[0]==0{(*dest).mant.m32[0]=0x80000000;(*dest).exp+=1}}},_=>()}}

unsafe fn modrem_kernel(dest:*mut fp_ext,src:*mut fp_ext,mode:i32)->*mut fp_ext{let mut tmp:fp_ext=core::mem::zeroed();fp_dyadic_check(dest,src);if IS_INF(dest)||IS_ZERO(src){fp_set_nan(dest);return dest}if IS_ZERO(dest)||IS_INF(src){return dest}fp_copy_ext(&mut tmp,dest);fp_fdiv(&mut tmp,src);fp_roundint(&mut tmp,mode);fp_fmul(&mut tmp,src);fp_fsub(dest,&mut tmp);fp_set_quotient(((*dest).mant.m64&0x7f)|(((*dest).sign as u64)<<7));dest}

pub unsafe fn fp_fmod(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext{dprint(PINSTR,"fmod\n");modrem_kernel(dest,src,FPCR_ROUND_RZ)}
pub unsafe fn fp_frem(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext{dprint(PINSTR,"frem\n");modrem_kernel(dest,src,FPCR_ROUND_RN)}
pub unsafe fn fp_fint(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext{dprint(PINSTR,"fint\n");fp_copy_ext(dest,src);fp_roundint(dest,FPDATA.rnd);dest}
pub unsafe fn fp_fintrz(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext{dprint(PINSTR,"fintrz\n");fp_copy_ext(dest,src);fp_roundint(dest,FPCR_ROUND_RZ);dest}
pub unsafe fn fp_fscale(dest:*mut fp_ext,src:*mut fp_ext)->*mut fp_ext{let(mut scale,oldround):(i32,i32);dprint(PINSTR,"fscale\n");fp_dyadic_check(dest,src);if IS_INF(src){fp_set_nan(dest);return dest}if IS_INF(dest)||IS_ZERO(src)||IS_ZERO(dest){return dest}if (*src).exp>=0x400c{fp_set_ovrflw(dest);return dest}oldround=FPDATA.rnd;FPDATA.rnd=FPCR_ROUND_RZ;scale=fp_conv_ext2long(src);FPDATA.rnd=oldround;scale+=(*dest).exp;if scale>=0x7fff{fp_set_ovrflw(dest)}else if scale<=0{fp_set_sr(FPSR_EXC_UNFL);fp_denormalize(dest,-scale)}else{(*dest).exp=scale}dest}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
