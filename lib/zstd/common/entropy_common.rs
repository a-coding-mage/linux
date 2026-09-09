// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Common functions of New Generation Entropy library. */

// Dependencies are supplied by the surrounding translation unit.

pub unsafe fn FSE_versionNumber() -> ::core::ffi::c_uint { FSE_VERSION_NUMBER }
pub unsafe fn FSE_isError(code: usize) -> ::core::ffi::c_uint { ERR_isError(code) }
pub unsafe fn FSE_getErrorName(code: usize) -> *const ::core::ffi::c_char { ERR_getErrorName(code) }
pub unsafe fn HUF_isError(code: usize) -> ::core::ffi::c_uint { ERR_isError(code) }
pub unsafe fn HUF_getErrorName(code: usize) -> *const ::core::ffi::c_char { ERR_getErrorName(code) }

pub unsafe fn FSE_readNCount_body(
    normalizedCounter: *mut i16, maxSVPtr: *mut ::core::ffi::c_uint,
    tableLogPtr: *mut ::core::ffi::c_uint, headerBuffer: *const ::core::ffi::c_void,
    hbSize: usize,
) -> usize {
    let istart = headerBuffer as *const u8;
    let iend = istart.add(hbSize);
    let mut ip = istart;
    let mut nbBits: i32;
    let mut remaining: i32;
    let mut threshold: i32;
    let mut bitStream: u32;
    let mut bitCount: i32;
    let mut charnum: ::core::ffi::c_uint = 0;
    let maxSV1 = *maxSVPtr + 1;
    let mut previous0 = false;

    if hbSize < 8 {
        let mut buffer = [0u8; 8];
        ZSTD_memcpy(buffer.as_mut_ptr() as *mut _, headerBuffer, hbSize);
        let countSize = FSE_readNCount(normalizedCounter, maxSVPtr, tableLogPtr,
                                       buffer.as_ptr() as *const _, buffer.len());
        if FSE_isError(countSize) != 0 { return countSize; }
        if countSize > hbSize { return ERROR(corruption_detected); }
        return countSize;
    }

    ZSTD_memset(normalizedCounter as *mut _, 0, ((*maxSVPtr + 1) as usize) * core::mem::size_of::<i16>());
    bitStream = MEM_readLE32(ip);
    nbBits = ((bitStream & 0xF) as i32) + FSE_MIN_TABLELOG as i32;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX as i32 { return ERROR(tableLog_tooLarge); }
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as _;
    remaining = (1i32 << nbBits) + 1;
    threshold = 1i32 << nbBits;
    nbBits += 1;

    loop {
        if previous0 {
            let mut repeats = (ZSTD_countTrailingZeros32(!bitStream | 0x80000000) >> 1) as i32;
            while repeats >= 12 {
                charnum += 36;
                if ip <= iend.sub(7) { ip = ip.add(3); }
                else { bitCount -= 8 * (iend.sub(7).offset_from(ip) as i32); bitCount &= 31; ip = iend.sub(4); }
                bitStream = MEM_readLE32(ip) >> bitCount;
                repeats = (ZSTD_countTrailingZeros32(!bitStream | 0x80000000) >> 1) as i32;
            }
            charnum += (3 * repeats) as u32;
            bitStream >>= (2 * repeats) as u32;
            bitCount += 2 * repeats;
            charnum += bitStream & 3;
            bitCount += 2;
            if charnum >= maxSV1 { break; }
            if ip <= iend.sub(7) || ip.add((bitCount >> 3) as usize) <= iend.sub(4) {
                ip = ip.add((bitCount >> 3) as usize); bitCount &= 7;
            } else { bitCount -= 8 * (iend.sub(4).offset_from(ip) as i32); bitCount &= 31; ip = iend.sub(4); }
            bitStream = MEM_readLE32(ip) >> bitCount;
        }
        let max = (2 * threshold - 1) - remaining;
        let mut count: i32;
        if (bitStream & (threshold - 1) as u32) < max as u32 { count = (bitStream & (threshold - 1) as u32) as i32; bitCount += nbBits - 1; }
        else { count = (bitStream & (2 * threshold - 1) as u32) as i32; if count >= threshold { count -= max; } bitCount += nbBits; }
        count -= 1;
        remaining -= count;
        *normalizedCounter.add(charnum as usize) = count as i16;
        charnum += 1;
        previous0 = count == 0;
        if remaining < threshold {
            if remaining <= 1 { break; }
            nbBits = ZSTD_highbit32(remaining as u32) as i32 + 1;
            threshold = 1 << (nbBits - 1);
        }
        if charnum >= maxSV1 { break; }
        if ip <= iend.sub(7) || ip.add((bitCount >> 3) as usize) <= iend.sub(4) { ip = ip.add((bitCount >> 3) as usize); bitCount &= 7; }
        else { bitCount -= 8 * (iend.sub(4).offset_from(ip) as i32); bitCount &= 31; ip = iend.sub(4); }
        bitStream = MEM_readLE32(ip) >> bitCount;
    }
    if remaining != 1 || bitCount > 32 { return ERROR(corruption_detected); }
    if charnum > maxSV1 { return ERROR(maxSymbolValue_tooSmall); }
    *maxSVPtr = charnum - 1;
    ip.add(((bitCount + 7) >> 3) as usize).offset_from(istart) as usize
}

pub unsafe fn FSE_readNCount_bmi2(a:*mut i16,b:*mut u32,c:*mut u32,d:*const ::core::ffi::c_void,e:usize,_bmi2:i32)->usize { FSE_readNCount_body(a,b,c,d,e) }
pub unsafe fn FSE_readNCount(a:*mut i16,b:*mut u32,c:*mut u32,d:*const ::core::ffi::c_void,e:usize)->usize { FSE_readNCount_bmi2(a,b,c,d,e,0) }

pub unsafe fn HUF_readStats(huffWeight:*mut u8,hwSize:usize,rankStats:*mut u32,nbSymbolsPtr:*mut u32,tableLogPtr:*mut u32,src:*const ::core::ffi::c_void,srcSize:usize)->usize {
    let mut wksp = [0u32; HUF_READ_STATS_WORKSPACE_SIZE_U32 as usize];
    HUF_readStats_wksp(huffWeight,hwSize,rankStats,nbSymbolsPtr,tableLogPtr,src,srcSize,wksp.as_mut_ptr() as *mut _,core::mem::size_of_val(&wksp),0)
}

pub unsafe fn HUF_readStats_body(huffWeight:*mut u8,hwSize:usize,rankStats:*mut u32,nbSymbolsPtr:*mut u32,tableLogPtr:*mut u32,src:*const ::core::ffi::c_void,srcSize:usize,_workSpace:*mut ::core::ffi::c_void,_wkspSize:usize,_bmi2:i32)->usize {
    if srcSize == 0 { return ERROR(srcSize_wrong); }
    let ip = src as *const u8; let mut iSize = *ip as usize; let oSize;
    if iSize >= 128 { oSize = iSize - 127; iSize = (oSize + 1) / 2; if iSize + 1 > srcSize || oSize >= hwSize { return ERROR(corruption_detected); } for n in (0..oSize).step_by(2) { *huffWeight.add(n)=*ip.add(1+n/2)>>4; if n+1<oSize {*huffWeight.add(n+1)=*ip.add(1+n/2)&15;} } }
    else { if iSize + 1 > srcSize { return ERROR(srcSize_wrong); } let n=FSE_decompress_wksp_bmi2(huffWeight,hwSize-1,ip.add(1),iSize,6,core::ptr::null_mut(),0,0); if FSE_isError(n)!=0{return n;} oSize=n; }
    ZSTD_memset(rankStats as *mut _,0,(HUF_TABLELOG_MAX as usize+1)*4); let mut total=0u32;
    for n in 0..oSize { let w=*huffWeight.add(n) as u32; if w>HUF_TABLELOG_MAX{return ERROR(corruption_detected);} *rankStats.add(w as usize)+=1; total += (1<<w)>>1; }
    if total==0{return ERROR(corruption_detected);} let log=ZSTD_highbit32(total)+1; if log>HUF_TABLELOG_MAX{return ERROR(corruption_detected);} *tableLogPtr=log; let rest=(1<<log)-total; let last=ZSTD_highbit32(rest)+1; if (1<<ZSTD_highbit32(rest))!=rest{return ERROR(corruption_detected);} *huffWeight.add(oSize)=last as u8; *rankStats.add(last as usize)+=1; if *rankStats.add(1)<2 || (*rankStats.add(1)&1)!=0{return ERROR(corruption_detected);} *nbSymbolsPtr=(oSize+1) as u32; iSize+1
}
pub unsafe fn HUF_readStats_wksp(a:*mut u8,b:usize,c:*mut u32,d:*mut u32,e:*mut u32,f:*const ::core::ffi::c_void,g:usize,h:*mut ::core::ffi::c_void,i:usize,_flags:i32)->usize { HUF_readStats_body(a,b,c,d,e,f,g,h,i,0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
