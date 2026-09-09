/* Rust translation of lz4_decompress.c. External LZ4/kernel symbols are supplied elsewhere. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn LZ4_decompress_generic(
    src: *const i8, dst: *mut i8, srcSize: i32, outputSize: i32,
    endOnInput: endCondition_directive, partialDecoding: earlyEnd_directive,
    dict: dict_directive, lowPrefix: *const BYTE, dictStart: *const BYTE,
    dictSize: usize,
) -> i32 {
    let mut ip = src as *const BYTE;
    let iend = ip.add(srcSize as usize);
    let mut op = dst as *mut BYTE;
    let oend = op.add(outputSize as usize);
    let mut cpy: *mut BYTE;
    let dictEnd = dictStart.add(dictSize);
    let inc32table: [u32; 8] = [0, 1, 2, 1, 0, 4, 4, 4];
    let dec64table: [i32; 8] = [0, 0, 0, -1, -4, 1, 2, 3];
    let safeDecode = endOnInput == endOnInputSize;
    let checkOffset = safeDecode && dictSize < (64 * KB) as usize;
    let shortiend = iend.sub((if endOnInput { 14 } else { 8 }) + 2);
    let shortoend = oend.sub((if endOnInput { 14 } else { 8 }) + 18);

    if endOnInput && outputSize == 0 { return if srcSize == 1 && *ip == 0 { 0 } else { -1 }; }
    if !endOnInput && outputSize == 0 { return if *ip == 0 { 1 } else { -1 }; }
    if endOnInput && srcSize == 0 { return -1; }

    loop {
        let mut length: usize;
        let mut match_: *const BYTE;
        let mut offset: usize;
        let token = *ip; ip = ip.add(1);
        length = (token >> ML_BITS) as usize;
        if (if endOnInput { length != RUN_MASK as usize } else { length <= 8 })
            && (if endOnInput { ip < shortiend } else { true }) && op <= shortoend {
            LZ4_memcpy(op as *mut _, ip as *const _, if endOnInput { 16 } else { 8 });
            op = op.add(length); ip = ip.add(length);
            length = (token & ML_MASK) as usize;
            offset = LZ4_readLE16(ip) as usize; ip = ip.add(2);
            match_ = op.sub(offset);
            if length != ML_MASK as usize && offset >= 8 &&
                (dict == withPrefix64k || match_ >= lowPrefix) {
                LZ4_memcpy(op, match_, 8); LZ4_memcpy(op.add(8), match_.add(8), 8);
                LZ4_memcpy(op.add(16), match_.add(16), 2);
                op = op.add(length + MINMATCH as usize); continue;
            }
        } else {
            if length == RUN_MASK as usize {
                let mut s: u32;
                if endOnInput && ip >= iend.sub(RUN_MASK as usize) { return -(ip as isize - src as isize) as i32 - 1; }
                loop { s = *ip as u32; ip = ip.add(1); length += s as usize; if !(s == 255 && (!endOnInput || ip < iend.sub(RUN_MASK as usize))) { break; } }
                if safeDecode && (op as usize).wrapping_add(length) < op as usize { return -(ip as isize - src as isize) as i32 - 1; }
                if safeDecode && (ip as usize).wrapping_add(length) < ip as usize { return -(ip as isize - src as isize) as i32 - 1; }
            }
            cpy = op.add(length);
            if (endOnInput && (cpy > oend.sub(MFLIMIT as usize) || ip.add(length) > iend.sub((2 + 1 + LASTLITERALS) as usize))) ||
                (!endOnInput && cpy > oend.sub(WILDCOPYLENGTH as usize)) {
                if partialDecoding {
                    if cpy > oend { cpy = oend; length = oend.offset_from(op) as usize; }
                    if endOnInput && ip.add(length) > iend { return -(ip as isize - src as isize) as i32 - 1; }
                } else if (!endOnInput && cpy != oend) || (endOnInput && (ip.add(length) != iend || cpy > oend)) { return -(ip as isize - src as isize) as i32 - 1; }
                LZ4_memmove(op as *mut _, ip as *const _, length); ip = ip.add(length); op = op.add(length);
                if !partialDecoding || cpy == oend || ip >= iend.sub(2) { break; }
            } else { LZ4_wildCopy(op, ip, cpy); ip = ip.add(length); op = cpy; }
            offset = LZ4_readLE16(ip) as usize; ip = ip.add(2); match_ = op.sub(offset);
            length = (token & ML_MASK) as usize;
        }
        if checkOffset && (match_ as usize).wrapping_add(dictSize) < lowPrefix as usize { return -(ip as isize - src as isize) as i32 - 1; }
        if !partialDecoding { LZ4_write32(op, offset as U32); }
        if length == ML_MASK as usize {
            loop { let s = *ip; ip = ip.add(1); if endOnInput && ip > iend.sub(LASTLITERALS as usize) { return -(ip as isize - src as isize) as i32 - 1; } length += s as usize; if s != 255 { break; } }
            if safeDecode && (op as usize).wrapping_add(length) < op as usize { return -(ip as isize - src as isize) as i32 - 1; }
        }
        length += MINMATCH as usize;
        if dict == usingExtDict && match_ < lowPrefix {
            if op.add(length) > oend.sub(LASTLITERALS as usize) { if !partialDecoding { return -(ip as isize - src as isize) as i32 - 1; } length = core::cmp::min(length, oend.offset_from(op) as usize); }
            let n = lowPrefix.offset_from(match_) as usize;
            if length <= n { core::ptr::copy(dictEnd.sub(n), op, length); op = op.add(length); }
            else { core::ptr::copy(dictEnd.sub(n), op, n); op = op.add(n); let rest = length - n; let end = op.add(rest); let mut from = lowPrefix; while op < end { *op = *from; op = op.add(1); from = from.add(1); } }
            continue;
        }
        cpy = op.add(length);
        if partialDecoding && cpy > oend.sub(MATCH_SAFEGUARD_DISTANCE as usize) {
            let mlen = core::cmp::min(length, oend.offset_from(op) as usize); let matchEnd = match_.add(mlen); let copyEnd = op.add(mlen);
            if matchEnd > op { while op < copyEnd { *op = *match_; op = op.add(1); match_ = match_.add(1); } } else { LZ4_memcpy(op, match_, mlen); }
            op = copyEnd; if op == oend { break; } continue;
        }
        if offset < 8 { *op = *match_; *op.add(1)=*match_.add(1); *op.add(2)=*match_.add(2); *op.add(3)=*match_.add(3); match_ = match_.add(inc32table[offset] as usize); LZ4_memcpy(op.add(4), match_, 4); match_ = match_.offset(-(dec64table[offset] as isize)); } else { LZ4_copy8(op, match_); match_ = match_.add(8); }
        op = op.add(8);
        if cpy > oend.sub(MATCH_SAFEGUARD_DISTANCE as usize) { let limit = oend.sub((WILDCOPYLENGTH - 1) as usize); if cpy > oend.sub(LASTLITERALS as usize) { return -(ip as isize - src as isize) as i32 - 1; } if op < limit { LZ4_wildCopy(op, match_, limit); match_ = match_.add(limit.offset_from(op) as usize); op = limit; } while op < cpy { *op = *match_; op=op.add(1); match_=match_.add(1); } } else { LZ4_copy8(op, match_); if length > 16 { LZ4_wildCopy(op.add(8), match_.add(8), cpy); } }
        op = cpy;
    }
    if endOnInput { op.offset_from(dst as *mut BYTE) as i32 } else { ip.offset_from(src as *const BYTE) as i32 }
}

pub unsafe fn LZ4_decompress_safe(source:*const i8,dest:*mut i8,compressedSize:i32,maxDecompressedSize:i32)->i32 { LZ4_decompress_generic(source,dest,compressedSize,maxDecompressedSize,endOnInputSize,decode_full_block,noDict,dest as *const BYTE,core::ptr::null(),0) }
pub unsafe fn LZ4_decompress_safe_partial(src:*const i8,dst:*mut i8,compressedSize:i32,targetOutputSize:i32,mut dstCapacity:i32)->i32 { dstCapacity=core::cmp::min(targetOutputSize,dstCapacity); LZ4_decompress_generic(src,dst,compressedSize,dstCapacity,endOnInputSize,partial_decode,noDict,dst as *const BYTE,core::ptr::null(),0) }
pub unsafe fn LZ4_decompress_fast(source:*const i8,dest:*mut i8,originalSize:i32)->i32 { LZ4_decompress_generic(source,dest,0,originalSize,endOnOutputSize,decode_full_block,withPrefix64k,(dest as *mut BYTE).sub(64*KB as usize),core::ptr::null(),0) }

unsafe fn LZ4_decompress_safe_withPrefix64k(s:*const i8,d:*mut i8,cs:i32,mo:i32)->i32 { LZ4_decompress_generic(s,d,cs,mo,endOnInputSize,decode_full_block,withPrefix64k,(d as *mut BYTE).sub(64*KB as usize),core::ptr::null(),0) }
unsafe fn LZ4_decompress_safe_withSmallPrefix(s:*const i8,d:*mut i8,cs:i32,mo:i32,ps:usize)->i32 { LZ4_decompress_generic(s,d,cs,mo,endOnInputSize,decode_full_block,noDict,(d as *mut BYTE).sub(ps),core::ptr::null(),0) }
unsafe fn LZ4_decompress_safe_forceExtDict(s:*const i8,d:*mut i8,cs:i32,mo:i32,ds:*const core::ffi::c_void,sz:usize)->i32 { LZ4_decompress_generic(s,d,cs,mo,endOnInputSize,decode_full_block,usingExtDict,d as *const BYTE,ds as *const BYTE,sz) }
unsafe fn LZ4_decompress_fast_extDict(s:*const i8,d:*mut i8,os:i32,ds:*const core::ffi::c_void,sz:usize)->i32 { LZ4_decompress_generic(s,d,0,os,endOnOutputSize,decode_full_block,usingExtDict,d as *const BYTE,ds as *const BYTE,sz) }
unsafe fn LZ4_decompress_safe_doubleDict(s:*const i8,d:*mut i8,cs:i32,mo:i32,ps:usize,ds:*const core::ffi::c_void,sz:usize)->i32 { LZ4_decompress_generic(s,d,cs,mo,endOnInputSize,decode_full_block,usingExtDict,(d as *mut BYTE).sub(ps),ds as *const BYTE,sz) }
unsafe fn LZ4_decompress_fast_doubleDict(s:*const i8,d:*mut i8,os:i32,ps:usize,ds:*const core::ffi::c_void,sz:usize)->i32 { LZ4_decompress_generic(s,d,0,os,endOnOutputSize,decode_full_block,usingExtDict,(d as *mut BYTE).sub(ps),ds as *const BYTE,sz) }

pub unsafe fn LZ4_setStreamDecode(st:*mut LZ4_streamDecode_t,dictionary:*const i8,dictSize:i32)->i32 { let s=&mut (*st).internal_donotuse; s.prefixSize=dictSize as usize; s.prefixEnd=(dictionary as *const BYTE).add(dictSize as usize); s.externalDict=core::ptr::null(); s.extDictSize=0; 1 }

pub unsafe fn LZ4_decompress_safe_continue(st:*mut LZ4_streamDecode_t,source:*const i8,dest:*mut i8,cs:i32,mo:i32)->i32 { let s=&mut (*st).internal_donotuse; let r=if s.prefixSize==0 { let r=LZ4_decompress_safe(source,dest,cs,mo); if r<=0{return r}; s.prefixSize=r as usize;s.prefixEnd=(dest as *mut BYTE).add(r as usize);r } else if s.prefixEnd==dest as *const BYTE { let r=if s.prefixSize>=64*KB as usize-1 {LZ4_decompress_safe_withPrefix64k(source,dest,cs,mo)} else if s.extDictSize==0 {LZ4_decompress_safe_withSmallPrefix(source,dest,cs,mo,s.prefixSize)} else {LZ4_decompress_safe_doubleDict(source,dest,cs,mo,s.prefixSize,s.externalDict as *const _,s.extDictSize)}; if r<=0{return r};s.prefixSize+=r as usize;s.prefixEnd=s.prefixEnd.add(r as usize);r } else {s.extDictSize=s.prefixSize;s.externalDict=s.prefixEnd.sub(s.extDictSize);let r=LZ4_decompress_safe_forceExtDict(source,dest,cs,mo,s.externalDict as *const _,s.extDictSize);if r<=0{return r};s.prefixSize=r as usize;s.prefixEnd=(dest as *mut BYTE).add(r as usize);r}; r }

pub unsafe fn LZ4_decompress_fast_continue(st:*mut LZ4_streamDecode_t,source:*const i8,dest:*mut i8,os:i32)->i32 { let s=&mut (*st).internal_donotuse; let r=if s.prefixSize==0 {let r=LZ4_decompress_fast(source,dest,os);if r<=0{return r};s.prefixSize=os as usize;s.prefixEnd=(dest as *mut BYTE).add(os as usize);r}else if s.prefixEnd==dest as *const BYTE {let r=if s.prefixSize>=64*KB as usize-1||s.extDictSize==0{LZ4_decompress_fast(source,dest,os)}else{LZ4_decompress_fast_doubleDict(source,dest,os,s.prefixSize,s.externalDict as *const _,s.extDictSize)};if r<=0{return r};s.prefixSize+=os as usize;s.prefixEnd=s.prefixEnd.add(os as usize);r}else{s.extDictSize=s.prefixSize;s.externalDict=s.prefixEnd.sub(s.extDictSize);let r=LZ4_decompress_fast_extDict(source,dest,os,s.externalDict as *const _,s.extDictSize);if r<=0{return r};s.prefixSize=os as usize;s.prefixEnd=(dest as *mut BYTE).add(os as usize);r};r }

pub unsafe fn LZ4_decompress_safe_usingDict(s:*const i8,d:*mut i8,cs:i32,mo:i32,dict:*const i8,ds:i32)->i32 {if ds==0{return LZ4_decompress_safe(s,d,cs,mo)} if dict.add(ds as usize)==d as *const i8 {if ds>=64*KB as i32-1{return LZ4_decompress_safe_withPrefix64k(s,d,cs,mo)} return LZ4_decompress_safe_withSmallPrefix(s,d,cs,mo,ds as usize)} LZ4_decompress_safe_forceExtDict(s,d,cs,mo,dict as *const _,ds as usize)}
pub unsafe fn LZ4_decompress_fast_usingDict(s:*const i8,d:*mut i8,os:i32,dict:*const i8,ds:i32)->i32 {if ds==0||dict.add(ds as usize)==d as *const i8{LZ4_decompress_fast(s,d,os)}else{LZ4_decompress_fast_extDict(s,d,os,dict as *const _,ds as usize)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
