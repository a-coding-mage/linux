// SPDX-License-Identifier: GPL-2.0
/* Save/restore floating point context for signal handlers. */

// Kernel-provided types, globals, macros, and functions are intentionally left
// as external dependencies of this translation.
const FPSCR_RCHG: usize = 0x00000000;

pub unsafe fn save_fpu(tsk: *mut task_struct) {
    let mut dummy: usize;
    enable_fpu();
    core::arch::asm!(
        "sts.l fpul, @-{0}\n\tsts.l fpscr, @-{0}\n\tfmov.s fr15, @-{0}\n\tfmov.s fr14, @-{0}\n\tfmov.s fr13, @-{0}\n\tfmov.s fr12, @-{0}\n\tfmov.s fr11, @-{0}\n\tfmov.s fr10, @-{0}\n\tfmov.s fr9, @-{0}\n\tfmov.s fr8, @-{0}\n\tfmov.s fr7, @-{0}\n\tfmov.s fr6, @-{0}\n\tfmov.s fr5, @-{0}\n\tfmov.s fr4, @-{0}\n\tfmov.s fr3, @-{0}\n\tfmov.s fr2, @-{0}\n\tfmov.s fr1, @-{0}\n\tfmov.s fr0, @-{0}\n\tlds {2}, fpscr",
        inout(reg) dummy,
        in(reg) &mut (*(*tsk).thread.xstate).hardfpu.status,
        in(reg) FPSCR_RCHG,
        in(reg) FPSCR_INIT,
        options(nostack)
    );
    disable_fpu();
}

pub unsafe fn restore_fpu(tsk: *mut task_struct) {
    let mut dummy: usize;
    enable_fpu();
    core::arch::asm!(
        "fmov.s @{0}+, fr0\n\tfmov.s @{0}+, fr1\n\tfmov.s @{0}+, fr2\n\tfmov.s @{0}+, fr3\n\tfmov.s @{0}+, fr4\n\tfmov.s @{0}+, fr5\n\tfmov.s @{0}+, fr6\n\tfmov.s @{0}+, fr7\n\tfmov.s @{0}+, fr8\n\tfmov.s @{0}+, fr9\n\tfmov.s @{0}+, fr10\n\tfmov.s @{0}+, fr11\n\tfmov.s @{0}+, fr12\n\tfmov.s @{0}+, fr13\n\tfmov.s @{0}+, fr14\n\tfmov.s @{0}+, fr15\n\tlds.l @{0}+, fpscr\n\tlds.l @{0}+, fpul",
        inout(reg) dummy,
        in(reg) (*tsk).thread.xstate,
        in(reg) FPSCR_RCHG,
        options(nostack)
    );
    disable_fpu();
}

unsafe fn denormal_mulf(hx: i32, hy: i32) -> i32 {
    let mut ix = (hx as u32) & 0x7fffffff;
    let mut iy = (hy as u32) & 0x7fffffff;
    if iy < 0x00800000 || ix == 0 { return ((hx ^ hy) as u32 & 0x80000000) as i32; }
    let mut exp = ((iy & 0x7f800000) >> 23) as i32;
    ix &= 0x007fffff; iy = (iy & 0x007fffff) | 0x00800000;
    let m = ix as u64 * iy as u64; let mut n = m; let mut w: i32 = -1;
    while n != 0 { n >>= 1; w += 1; }
    exp += w - 126 - 46;
    if exp > 0 { ix = (((m >> (w - 23)) as u32) & 0x007fffff) | ((exp as u32) << 23); }
    else if exp + 22 >= 0 { ix = (m >> (w - 22 - exp)) as u32 & 0x007fffff; }
    else { ix = 0; }
    (ix | ((hx ^ hy) as u32 & 0x80000000)) as i32
}

unsafe fn mult64(x: u64, y: u64, highp: *mut u64, lowp: *mut u64) {
    let sub0 = (x >> 32) * (y >> 32); let sub1 = (x & 0xffffffff) * (y >> 32);
    let sub2 = (x >> 32) * (y & 0xffffffff); let mut sub3 = (x & 0xffffffff) * (y & 0xffffffff);
    let mut low = sub3; let mut high = 0;
    sub3 = sub3.wrapping_add(sub1 << 32); if low > sub3 { high += 1; } low = sub3;
    sub3 = sub3.wrapping_add(sub2 << 32); if low > sub3 { high += 1; } low = sub3;
    high = high.wrapping_add((sub1 >> 32) + (sub2 >> 32) + sub0);
    *lowp = low; *highp = high;
}

unsafe fn rshift64(mh: u64, ml: u64, n: i32) -> u64 { if n >= 64 { mh >> (n - 64) } else { (mh << (64 - n)) | (ml >> n) } }

unsafe fn denormal_muld(hx: i64, hy: i64) -> i64 {
    let mut ix = hx as u64 & 0x7fffffffffffffff; let iy0 = hy as u64 & 0x7fffffffffffffff;
    if iy0 < 0x0010000000000000 || ix == 0 { return (hx ^ hy) & 0x8000000000000000u64 as i64; }
    let mut exp = ((iy0 & 0x7ff0000000000000) >> 52) as i32; ix &= 0x000fffffffffffff;
    let iy = (iy0 & 0x000fffffffffffff) | 0x0010000000000000; let mut mh=0; let mut ml=0; mult64(ix,iy,&mut mh,&mut ml);
    let mut nh=mh; let mut nl=ml; let mut w=-1i32; if nh != 0 { while nh != 0 { nh >>= 1; w+=1; } w+=64; } else { while nl != 0 { nl >>= 1; w+=1; } }
    exp += w - 1022 - 104; if exp > 0 { ix = (rshift64(mh,ml,w-52)&0x000fffffffffffff) | ((exp as u64)<<52); }
    else if exp+51>=0 { ix=rshift64(mh,ml,w-51-exp)&0x000fffffffffffff; } else { ix=0; }
    (ix | ((hx ^ hy) as u64 & 0x8000000000000000)) as i64
}

unsafe fn denormal_subf1(mut ix:u32, mut iy:u32)->u32 { if ix<0x00800000{return ix.wrapping_sub(iy)}; let mut exp=((ix&0x7f800000)>>23) as i32; if exp-1>31{return ix}; iy>>=(exp-1) as u32; if iy==0{return ix}; let mut frac=(ix&0x007fffff)|0x00800000; frac=frac.wrapping_sub(iy); while frac<0x00800000 { exp-=1; if exp==0{return frac}; frac<<=1; } (exp as u32)<<23 | frac&0x007fffff }
unsafe fn denormal_addf1(ix:u32, mut iy:u32)->u32 { if ix<0x00800000{return ix.wrapping_add(iy)}; let mut exp=((ix&0x7f800000)>>23) as i32; if exp-1>31{return ix}; iy>>=(exp-1) as u32; if iy==0{return ix}; let mut frac=(ix&0x007fffff)|0x00800000; frac+=iy; if frac>=0x01000000 {frac>>=1;exp+=1;} (exp as u32)<<23|frac&0x007fffff }
unsafe fn denormal_addf(hx:i32,hy:i32)->i32 { let (mut sign,mut ix,mut iy); if ((hx^hy)&0x80000000)!=0 {sign=hx&0x80000000;ix=hx as u32&0x7fffffff;iy=hy as u32&0x7fffffff;if iy<0x00800000 {ix=denormal_subf1(ix,iy);if ix as i32<0 {ix=ix.wrapping_neg();sign^=0x80000000}} else {ix=denormal_subf1(iy,ix);sign^=0x80000000}} else {sign=hx&0x80000000;ix=hx as u32&0x7fffffff;iy=hy as u32&0x7fffffff;if iy<0x00800000{ix=denormal_addf1(ix,iy)}else{ix=denormal_addf1(iy,ix)}} (sign|ix as i32) }

// The remaining double-precision helpers and trap handler retain the C source
// algorithm; kernel structure definitions and trap macros are external.
unsafe fn denormal_subd1(mut ix:u64, mut iy:u64)->i64 { if ix<0x0010000000000000{return ix.wrapping_sub(iy) as i64}; let mut exp=((ix&0x7ff0000000000000)>>52) as i32;if exp-1>63{return ix as i64};iy>>=(exp-1) as u32;if iy==0{return ix as i64};let mut frac=(ix&0x000fffffffffffff)|0x0010000000000000;frac=frac.wrapping_sub(iy);while frac<0x0010000000000000{exp-=1;if exp==0{return frac as i64};frac<<=1;}(((exp as u64)<<52)|(frac&0x000fffffffffffff)) as i64 }
unsafe fn denormal_addd1(ix:u64,mut iy:u64)->i64 { if ix<0x0010000000000000{return ix.wrapping_add(iy) as i64};let mut exp=((ix&0x7ff0000000000000)>>52) as i32;if exp-1>63{return ix as i64};iy>>=(exp-1) as u32;if iy==0{return ix as i64};let mut frac=(ix&0x000fffffffffffff)|0x0010000000000000;frac+=iy;if frac>=0x0020000000000000{frac>>=1;exp+=1;}(((exp as u64)<<52)|(frac&0x000fffffffffffff)) as i64 }
unsafe fn denormal_addd(hx:i64,hy:i64)->i64 { let sign= hx&0x8000000000000000u64 as i64; let ix=hx as u64&0x7fffffffffffffff;let iy=hy as u64&0x7fffffffffffffff;if ((hx^hy)&0x8000000000000000u64 as i64)!=0 { if iy<0x0010000000000000 {let x=denormal_subd1(ix,iy);return if x<0 {sign^0x8000000000000000u64 as i64|(-x) as i64}else{sign|x}} let x=denormal_subd1(iy,ix);return sign^0x8000000000000000u64 as i64|x;} if iy<0x0010000000000000 {sign|denormal_addd1(ix,iy)} else {sign|denormal_addd1(iy,ix)} }

unsafe fn denormal_to_double(fpu: *mut sh_fpu_hard_struct, n: usize) {
    let mut x = (*fpu).fpul as u32; let mut exp: i32 = 1023 - 126;
    if x != 0 && (x & 0x7f800000) == 0 {
        let du = x & 0x80000000;
        while x & 0x00800000 == 0 { x <<= 1; exp -= 1; }
        x &= 0x007fffff;
        (*fpu).fp_regs[n] = du | ((exp as u32) << 20) | (x >> 3);
        (*fpu).fp_regs[n + 1] = x << 29;
    }
}

unsafe fn ieee_fpe_handler(regs: *mut pt_regs) -> i32 {
    let insn = *( (*regs).pc as *const u16 );
    let nib = [(insn>>12)&15,(insn>>8)&15,(insn>>4)&15,insn&15];
    let mut nextpc: usize; let finsn: u16;
    if nib[0]==0xb || (nib[0]==4 && nib[2]==0 && nib[3]==0xb) { (*regs).pr=(*regs).pc+4; }
    if nib[0]==0xa || nib[0]==0xb { nextpc=(*regs).pc+4+((((insn&0xfff)<<4) as i16 as isize>>3) as usize); finsn=*(((*regs).pc+2) as *const u16); }
    else if nib[0]==8 && nib[1]==0xd { nextpc=if (*regs).sr&1!=0 {(*regs).pc+4+(((insn&255) as i8 as isize*2) as usize)} else {(*regs).pc+4}; finsn=*(((*regs).pc+2) as *const u16); }
    else if nib[0]==8 && nib[1]==0xf { nextpc=if (*regs).sr&1!=0 {(*regs).pc+4} else {(*regs).pc+4+(((insn&255) as i8 as isize*2) as usize)}; finsn=*(((*regs).pc+2) as *const u16); }
    else if nib[0]==4 && nib[3]==0xb && (nib[2]==0 || nib[2]==2) { nextpc=(*regs).regs[nib[1] as usize]; finsn=*(((*regs).pc+2) as *const u16); }
    else if nib[0]==0 && nib[3]==3 && (nib[2]==0 || nib[2]==2) { nextpc=(*regs).pc+4+(*regs).regs[nib[1] as usize]; finsn=*(((*regs).pc+2) as *const u16); }
    else if insn==0x000b { nextpc=(*regs).pr; finsn=*(((*regs).pc+2) as *const u16); } else { nextpc=(*regs).pc+2; finsn=insn; }
    let tsk = current;
    let fpu=&mut (*(*tsk).thread.xstate).hardfpu;
    if finsn&0xf1ff==0xf0ad { if fpu.fpscr & (1<<17)==0{return 0}; denormal_to_double(fpu,((finsn>>8)&15) as usize);(*regs).pc=nextpc;return 1; }
    let n=((finsn>>8)&15) as usize; let m=((finsn>>4)&15) as usize; let hx=fpu.fp_regs[n];let hy=fpu.fp_regs[m];let prec=fpu.fpscr&(1<<19)!=0;
    if finsn&0xf00f==0xf002 && fpu.fpscr&(1<<17)!=0 { if prec { let x=((hx as u64)<<32|fpu.fp_regs[n+1] as u64) as i64;let y=((hy as u64)<<32|fpu.fp_regs[m+1] as u64) as i64;let z=denormal_muld(if hx&0x7fffffff>=0x00100000{y}else{x},if hx&0x7fffffff>=0x00100000{x}else{y});fpu.fp_regs[n]=(z>>32) as u32;fpu.fp_regs[n+1]=z as u32;} else {fpu.fp_regs[n]=denormal_mulf(if hx&0x7fffffff>=0x00800000{hy as i32}else{hx as i32},if hx&0x7fffffff>=0x00800000{hx as i32}else{hy as i32}) as u32;}(*regs).pc=nextpc;return 1; }
    if finsn&0xf00e==0xf000 && fpu.fpscr&(1<<17)!=0 {
        if prec { let x=((hx as u64)<<32|fpu.fp_regs[n+1] as u64) as i64; let y=((hy as u64)<<32|fpu.fp_regs[m+1] as u64) as i64; let z=if finsn&0xf00f==0xf000{denormal_addd(x,y)}else{denormal_addd(x,y^(1i64<<63))}; fpu.fp_regs[n]=(z>>32) as u32; fpu.fp_regs[n+1]=z as u32; }
        else { fpu.fp_regs[n]=if finsn&0xf00f==0xf000{denormal_addf(hx as i32,hy as i32)}else{denormal_addf(hx as i32,(hy^0x80000000) as i32)} as u32; }
        (*regs).pc=nextpc; return 1;
    }
    0
}

extern "C" { fn enable_fpu(); fn disable_fpu(); }

pub unsafe fn fpu_error(regs: *mut pt_regs) {
    let tsk=current; __unlazy_fpu(tsk,regs);
    if ieee_fpe_handler(regs)!=0 { (*(*tsk).thread.xstate).hardfpu.fpscr &= !(FPSCR_CAUSE_MASK|FPSCR_FLAG_MASK); grab_fpu(regs); restore_fpu(tsk); task_thread_info(tsk).status |= TS_USEDFPU; } else { force_sig(SIGFPE); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
