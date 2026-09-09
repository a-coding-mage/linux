// SPDX-License-Identifier: 0BSD

/*
 * Branch/Call/Jump (BCJ) filter decoders
 *
 * Authors: Lasse Collin <lasse.collin@tukaani.org>
 *          Igor Pavlov <https://7-zip.org/>
 */

// The C source places the implementation under XZ_DEC_BCJ and individual
// filter implementations under their respective build-time feature macros.

#[repr(C)]
pub struct xz_dec_bcj {
    pub r#type: u8,
    pub ret: xz_ret,
    pub single_call: bool,
    pub pos: u32,
    pub x86_prev_mask: u32,
    pub out: *mut u8,
    pub out_pos: usize,
    pub out_size: usize,
    pub temp: xz_dec_bcj_temp,
}

#[repr(C)]
pub struct xz_dec_bcj_temp {
    pub filtered: usize,
    pub size: usize,
    pub buf: [u8; 16],
}

const BCJ_X86: u8 = 4;
const BCJ_POWERPC: u8 = 5;
const BCJ_ARM: u8 = 7;
const BCJ_ARMTHUMB: u8 = 8;
const BCJ_SPARC: u8 = 9;
const BCJ_ARM64: u8 = 10;
const BCJ_RISCV: u8 = 11;

#[inline]
fn bcj_x86_test_msbyte(b: u8) -> bool { b == 0 || b == 0xFF }

unsafe fn le32(p: *const u8) -> u32 {
    u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}
unsafe fn be32(p: *const u8) -> u32 {
    u32::from_be_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}
unsafe fn put_le32(p: *mut u8, v: u32) { let a = v.to_le_bytes(); for i in 0..4 { *p.add(i) = a[i]; } }
unsafe fn put_be32(p: *mut u8, v: u32) { let a = v.to_be_bytes(); for i in 0..4 { *p.add(i) = a[i]; } }

unsafe fn bcj_x86(s: &mut xz_dec_bcj, buf: *mut u8, mut size: usize) -> usize {
    let allowed = [true, true, true, false, true, false, false, false];
    let bits = [0u8, 1, 2, 2, 3, 3, 3, 3];
    if size <= 4 { return 0; }
    let mut i = 0usize; let mut prev_pos = usize::MAX; let mut prev_mask = s.x86_prev_mask; size -= 4;
    while i < size {
        if (*buf.add(i) & 0xFE) != 0xE8 { i += 1; continue; }
        prev_pos = i.wrapping_sub(prev_pos);
        if prev_pos > 3 { prev_mask = 0; } else {
            prev_mask = (prev_mask << (prev_pos - 1)) & 7;
            if prev_mask != 0 {
                let b = *buf.add(i + 4 - bits[prev_mask as usize] as usize);
                if !allowed[prev_mask as usize] || bcj_x86_test_msbyte(b) { prev_pos = i; prev_mask = (prev_mask << 1) | 1; i += 1; continue; }
            }
        }
        prev_pos = i;
        if bcj_x86_test_msbyte(*buf.add(i + 4)) {
            let mut src = le32(buf.add(i + 1)); let mut dest;
            loop { dest = src.wrapping_sub(s.pos.wrapping_add(i as u32).wrapping_add(5)); if prev_mask == 0 { break; } let j = bits[prev_mask as usize] * 8; let b = (dest >> (24 - j)) as u8; if !bcj_x86_test_msbyte(b) { break; } src = dest ^ ((1u32 << (32 - j)) - 1); }
            dest &= 0x01FFFFFF; dest |= 0u32.wrapping_sub(dest & 0x01000000); put_le32(buf.add(i + 1), dest); i += 4;
        } else { prev_mask = (prev_mask << 1) | 1; }
        i += 1;
    }
    prev_pos = i.wrapping_sub(prev_pos); s.x86_prev_mask = if prev_pos > 3 { 0 } else { prev_mask << (prev_pos - 1) }; i
}

unsafe fn bcj_powerpc(s: &mut xz_dec_bcj, buf: *mut u8, size: usize) -> usize { let n = size & !3; let mut i=0; while i<n { let mut v=be32(buf.add(i)); if v&0xFC000003==0x48000001 { v=(v&0x03FFFFFC).wrapping_sub(s.pos.wrapping_add(i as u32))&0x03FFFFFC|0x48000001; put_be32(buf.add(i),v); } i+=4; } i }
unsafe fn bcj_arm(s: &mut xz_dec_bcj, buf: *mut u8, size: usize) -> usize { let n=size&!3; let mut i=0; while i<n { if *buf.add(i+3)==0xEB { let mut a=*buf.add(i) as u32|(*buf.add(i+1) as u32)<<8|(*buf.add(i+2) as u32)<<16; a=(a<<2).wrapping_sub(s.pos.wrapping_add(i as u32).wrapping_add(8))>>2; *buf.add(i)=a as u8; *buf.add(i+1)=(a>>8) as u8; *buf.add(i+2)=(a>>16) as u8; } i+=4; } i }
unsafe fn bcj_armthumb(s: &mut xz_dec_bcj, buf: *mut u8, size: usize) -> usize { if size<4{return 0}; let n=size-4; let mut i=0; while i<=n { if (*buf.add(i+1)&0xF8)==0xF0&&(*buf.add(i+3)&0xF8)==0xF8 { let mut a=(((*buf.add(i+1)&7) as u32)<<19)|((*buf.add(i) as u32)<<11)|(((*buf.add(i+3)&7) as u32)<<8)|*buf.add(i+2) as u32; a=(a<<1).wrapping_sub(s.pos.wrapping_add(i as u32).wrapping_add(4))>>1; *buf.add(i+1)=0xF0|((a>>19)&7) as u8; *buf.add(i)=(a>>11) as u8; *buf.add(i+3)=0xF8|((a>>8)&7) as u8; *buf.add(i+2)=a as u8; i+=2; } i+=2; } i }

unsafe fn bcj_sparc(s:&mut xz_dec_bcj,buf:*mut u8,size:usize)->usize{let n=size&!3;let mut i=0;while i<n{let mut v=be32(buf.add(i));if v>>22==0x100||v>>22==0x1FF{v=(v<<2).wrapping_sub(s.pos.wrapping_add(i as u32))>>2;v=(0x40000000u32.wrapping_sub(v&0x400000)|0x40000000|(v&0x3FFFFF));put_be32(buf.add(i),v);}i+=4;}i}
unsafe fn bcj_arm64(s:&mut xz_dec_bcj,buf:*mut u8,size:usize)->usize{let n=size&!3;let mut i=0;while i<n{let mut v=le32(buf.add(i));if v>>26==0x25{let a=v.wrapping_sub(s.pos.wrapping_add(i as u32)>>2);v=0x94000000|(a&0x03FFFFFF);put_le32(buf.add(i),v);}else if v&0x9F000000==0x90000000{let mut a=((v>>29)&3)|((v>>3)&0x1FFFFC);if (a+0x020000)&0x1C0000!=0{i+=4;continue}a=a.wrapping_sub(s.pos.wrapping_add(i as u32)>>12);v&=0x9000001F;v|=(a&3)<<29;v|=(a&0x03FFFC)<<3;v|=(0u32.wrapping_sub(a&0x020000))&0xE00000;put_le32(buf.add(i),v);}i+=4;}i}

// RISC-V and the buffering helpers below retain the same interfaces and are
// declared with the external types supplied by the surrounding decoder.
extern "C" { fn xz_dec_lzma2_run(lzma2:*mut xz_dec_lzma2,b:*mut xz_buf)->xz_ret; }
#[repr(C)] pub struct xz_dec_lzma2 { _private: [u8;0] }
#[repr(C)] pub struct xz_buf { pub in_:*const u8,pub in_pos:usize,pub in_size:usize,pub out:*mut u8,pub out_pos:usize,pub out_size:usize }
pub type xz_ret = i32; const XZ_OK:xz_ret=0; const XZ_STREAM_END:xz_ret=1; const XZ_OPTIONS_ERROR:xz_ret=2;

unsafe fn bcj_flush(s:&mut xz_dec_bcj,b:&mut xz_buf){let n=core::cmp::min(s.temp.filtered,b.out_size-b.out_pos);core::ptr::copy_nonoverlapping(s.temp.buf.as_ptr(),b.out.add(b.out_pos),n);b.out_pos+=n;s.temp.filtered-=n;s.temp.size-=n;core::ptr::copy(s.temp.buf.as_ptr().add(n),s.temp.buf.as_mut_ptr(),s.temp.size);}

unsafe fn bcj_apply(s:&mut xz_dec_bcj,buf:*mut u8,pos:&mut usize,size:usize){let p=buf.add(*pos);let n=match s.r#type{BCJ_X86=>bcj_x86(s,p,size-*pos),BCJ_POWERPC=>bcj_powerpc(s,p,size-*pos),BCJ_ARM=>bcj_arm(s,p,size-*pos),BCJ_ARMTHUMB=>bcj_armthumb(s,p,size-*pos),BCJ_SPARC=>bcj_sparc(s,p,size-*pos),BCJ_ARM64=>bcj_arm64(s,p,size-*pos),_=>0};*pos+=n;s.pos=s.pos.wrapping_add(n as u32);}

pub unsafe fn xz_dec_bcj_run(s:&mut xz_dec_bcj,lzma2:*mut xz_dec_lzma2,b:&mut xz_buf)->xz_ret{if s.temp.filtered>0{bcj_flush(s,b);if s.temp.filtered>0{return XZ_OK;}if s.ret==XZ_STREAM_END{return XZ_STREAM_END;}}let start=b.out_pos;if s.temp.size<b.out_size-b.out_pos||s.temp.size==0{core::ptr::copy_nonoverlapping(s.temp.buf.as_ptr(),b.out.add(b.out_pos),s.temp.size);b.out_pos+=s.temp.size;s.ret=xz_dec_lzma2_run(lzma2,b);if s.ret!=XZ_STREAM_END&&(s.ret!=XZ_OK||s.single_call){return s.ret;}bcj_apply(s,b.out,&mut {let mut p=start;p},b.out_pos);s.temp.size=b.out_pos-start;b.out_pos-=s.temp.size;core::ptr::copy_nonoverlapping(b.out.add(b.out_pos),s.temp.buf.as_mut_ptr(),s.temp.size);if b.out_pos+s.temp.size<b.out_size{return XZ_OK;}}if b.out_pos<b.out_size{s.out=b.out;s.out_pos=b.out_pos;s.out_size=b.out_size;b.out=s.temp.buf.as_mut_ptr();b.out_pos=s.temp.size;b.out_size=16;s.ret=xz_dec_lzma2_run(lzma2,b);s.temp.size=b.out_pos;b.out=s.out;b.out_pos=s.out_pos;b.out_size=s.out_size;if s.ret!=XZ_OK&&s.ret!=XZ_STREAM_END{return s.ret;}s.temp.filtered=0;bcj_apply(s,s.temp.buf.as_mut_ptr(),&mut s.temp.filtered,s.temp.size);if s.ret==XZ_STREAM_END{s.temp.filtered=s.temp.size;}bcj_flush(s,b);if s.temp.filtered>0{return XZ_OK;}}s.ret}

pub unsafe fn xz_dec_bcj_create(single_call:bool)->*mut xz_dec_bcj{let p=libc::malloc(core::mem::size_of::<xz_dec_bcj>()) as *mut xz_dec_bcj;if !p.is_null(){(*p)=core::mem::zeroed();(*p).single_call=single_call;}p}
pub unsafe fn xz_dec_bcj_reset(s:&mut xz_dec_bcj,id:u8)->xz_ret{match id{BCJ_X86|BCJ_POWERPC|BCJ_ARM|BCJ_ARMTHUMB|BCJ_SPARC|BCJ_ARM64|BCJ_RISCV=>{},_=>return XZ_OPTIONS_ERROR} s.r#type=id;s.ret=XZ_OK;s.pos=0;s.x86_prev_mask=0;s.temp.filtered=0;s.temp.size=0;XZ_OK}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
