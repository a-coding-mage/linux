// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Faithful low-level Rust translation of fse_compress.c.  Types and helper
 * symbols supplied by the surrounding zstd sources remain external. */

// The original file is a type-specialized template.  This translation keeps
// the template's externally supplied names available through these aliases.
type FseFunctionType = u8;

pub unsafe fn FSE_buildCTable_wksp(ct: *mut FSE_CTable, normalizedCounter: *const i16,
    maxSymbolValue: u32, tableLog: u32, workSpace: *mut core::ffi::c_void,
    wkspSize: usize) -> usize {
    let tableSize: u32 = 1u32 << tableLog;
    let tableMask = tableSize - 1;
    let ptr = ct as *mut u8;
    let tableU16 = ptr.add(4) as *mut u16;
    let fsct = ptr.add(4 + if tableLog != 0 { (tableSize >> 1) as usize * 4 } else { 4 }) as *mut FSE_symbolCompressionTransform;
    let step = FSE_TABLESTEP(tableSize);
    let maxSV1 = maxSymbolValue + 1;
    let cumul = workSpace as *mut u16;
    let tableSymbol = cumul.add((maxSV1 + 1) as usize) as *mut FseFunctionType;
    let mut highThreshold = tableSize - 1;
    assert!((workSpace as usize & 1) == 0);
    if FSE_BUILD_CTABLE_WORKSPACE_SIZE(maxSymbolValue, tableLog) > wkspSize { return ERROR(tableLog_tooLarge); }
    *tableU16.offset(-2) = tableLog as u16;
    *tableU16.offset(-1) = maxSymbolValue as u16;
    assert!(tableLog < 16);
    *cumul = 0;
    for u in 1..=maxSV1 {
        let n = *normalizedCounter.add((u - 1) as usize);
        if n == -1 { *cumul.add(u as usize) = *cumul.add((u-1) as usize) + 1; *tableSymbol.add(highThreshold as usize) = (u-1) as FseFunctionType; highThreshold -= 1; }
        else { assert!(n >= 0); *cumul.add(u as usize) = *cumul.add((u-1) as usize) + n as u16; }
    }
    *cumul.add(maxSV1 as usize) = (tableSize + 1) as u16;
    if highThreshold == tableSize - 1 {
        let spread = tableSymbol.add(tableSize as usize) as *mut u8;
        let add: u64 = 0x0101010101010101;
        let mut pos = 0usize; let mut sv = 0u64;
        for s in 0..maxSV1 { let n = *normalizedCounter.add(s as usize); let v = sv as u64; core::ptr::write_unaligned(spread.add(pos) as *mut u64, v); let mut i=8; while i < n as usize { core::ptr::write_unaligned(spread.add(pos+i) as *mut u64, v); i+=8; } pos += n as usize; sv = sv.wrapping_add(add); }
        let mut position=0usize; let unroll=2usize;
        for s in (0..tableSize as usize).step_by(unroll) { for u in 0..unroll { let p=(position + u * step as usize) & tableMask as usize; *tableSymbol.add(p)=*spread.add(s+u) as FseFunctionType; } position=(position+unroll*step as usize)&tableMask as usize; }
        assert_eq!(position,0);
    } else { let mut position=0u32; for symbol in 0..maxSV1 { let freq=*normalizedCounter.add(symbol as usize); for _ in 0..freq.max(0) { *tableSymbol.add(position as usize)=symbol as FseFunctionType; position=(position+step)&tableMask; while position>highThreshold { position=(position+step)&tableMask; } } } assert_eq!(position,0); }
    for u in 0..tableSize { let s=*tableSymbol.add(u as usize) as usize; let old=*cumul.add(s); *cumul.add(s)=old+1; *tableU16.add(old as usize)=(tableSize+u) as u16; }
    let mut total=0u32;
    for s in 0..=maxSymbolValue { let n=*normalizedCounter.add(s as usize); let tt=&mut *fsct.add(s as usize); match n { 0 => tt.deltaNbBits=((tableLog+1)<<16)-(1<<tableLog), -1|1 => { tt.deltaNbBits=(tableLog<<16)-(1<<tableLog); tt.deltaFindState=(total as i32)-1; total+=1; }, _ => { let maxBitsOut=tableLog-ZSTD_highbit32((n as u32)-1); let minStatePlus=(n as u32)<<maxBitsOut; tt.deltaNbBits=(maxBitsOut<<16)-minStatePlus; tt.deltaFindState=total as i32-n as i32; total+=n as u32; } } }
    0
}

pub fn FSE_NCountWriteBound(maxSymbolValue:u32, tableLog:u32)->usize { let n=(((maxSymbolValue+1)*tableLog+6)/8)+1+2; if maxSymbolValue!=0 {n as usize} else {FSE_NCOUNTBOUND} }

unsafe fn FSE_writeNCount_generic(header:*mut core::ffi::c_void, headerBufferSize:usize, normalizedCounter:*const i16, maxSymbolValue:u32, tableLog:u32, writeIsSafe:u32)->usize {
    let ostart=header as *mut u8; let mut out=ostart; let oend=ostart.add(headerBufferSize); let tableSize=1i32<<tableLog; let alphabetSize=maxSymbolValue+1; let mut bitStream=0u32; let mut bitCount=0i32; let mut symbol=0u32; let mut remaining=tableSize+1; let mut threshold=tableSize; let mut nbBits=tableLog as i32+1; let mut previousIs0=0;
    bitStream += (tableLog-FSE_MIN_TABLELOG)<<bitCount; bitCount+=4;
    while symbol<alphabetSize && remaining>1 { if previousIs0!=0 { let start=symbol; while symbol<alphabetSize && *normalizedCounter.add(symbol as usize)==0 {symbol+=1;} if symbol==alphabetSize {break;} while symbol>=start+24 { bitStream+=0xffff<<bitCount; if writeIsSafe==0 && out>oend.sub(2){return ERROR(dstSize_tooSmall);} *out=bitStream as u8; *out.add(1)=(bitStream>>8) as u8; out=out.add(2); bitStream>>=16; symbol-=24; } let mut st=start; while symbol>=st+3 {st+=3; bitStream+=3<<bitCount; bitCount+=2;} bitStream+=(symbol-st)<<bitCount; bitCount+=2; if bitCount>16 {if writeIsSafe==0&&out>oend.sub(2){return ERROR(dstSize_tooSmall);} *out=bitStream as u8;*out.add(1)=(bitStream>>8)as u8;out=out.add(2);bitStream>>=16;bitCount-=16;} }
        let mut count=*normalizedCounter.add(symbol as usize) as i32; symbol+=1; let max=(2*threshold-1)-remaining; remaining-=if count<0{-count}else{count}; count+=1; if count>=threshold {count+=max;} bitStream+=(count as u32)<<bitCount; bitCount+=nbBits; if count<max {bitCount-=1;} previousIs0=(count==1) as i32; if remaining<1{return ERROR(GENERIC);} while remaining<threshold {nbBits-=1;threshold>>=1;} if bitCount>16 {if writeIsSafe==0&&out>oend.sub(2){return ERROR(dstSize_tooSmall);}*out=bitStream as u8;*out.add(1)=(bitStream>>8)as u8;out=out.add(2);bitStream>>=16;bitCount-=16;}
    } if remaining!=1{return ERROR(GENERIC);} if writeIsSafe==0&&out>oend.sub(2){return ERROR(dstSize_tooSmall);} *out=bitStream as u8;*out.add(1)=(bitStream>>8)as u8;out=out.add(((bitCount+7)/8) as usize);out.offset_from(ostart) as usize
}

pub unsafe fn FSE_writeNCount(buffer:*mut core::ffi::c_void, bufferSize:usize, normalizedCounter:*const i16, maxSymbolValue:u32, tableLog:u32)->usize { if tableLog>FSE_MAX_TABLELOG{return ERROR(tableLog_tooLarge);} if tableLog<FSE_MIN_TABLELOG{return ERROR(GENERIC);} FSE_writeNCount_generic(buffer,bufferSize,normalizedCounter,maxSymbolValue,tableLog,(bufferSize>=FSE_NCountWriteBound(maxSymbolValue,tableLog)) as u32) }

unsafe fn FSE_minTableLog(srcSize:usize,maxSymbolValue:u32)->u32 { assert!(srcSize>1); let a=ZSTD_highbit32(srcSize as u32)+1; let b=ZSTD_highbit32(maxSymbolValue)+2; if a<b {a}else{b} }
pub unsafe fn FSE_optimalTableLog_internal(maxTableLog:u32,srcSize:usize,maxSymbolValue:u32,minus:u32)->u32 { let maxBits=ZSTD_highbit32((srcSize-1) as u32)-minus; let mut t=if maxTableLog==0{FSE_DEFAULT_TABLELOG}else{maxTableLog}; let min=FSE_minTableLog(srcSize,maxSymbolValue); if maxBits<t{t=maxBits;}if min>t{t=min;}if t<FSE_MIN_TABLELOG{t=FSE_MIN_TABLELOG;}if t>FSE_MAX_TABLELOG{t=FSE_MAX_TABLELOG;}t }
pub unsafe fn FSE_optimalTableLog(m:u32,s:usize,v:u32)->u32{FSE_optimalTableLog_internal(m,s,v,2)}

unsafe fn FSE_normalizeM2(norm:*mut i16,tableLog:u32,count:*const u32,total:usize,maxSymbolValue:u32,lowProbCount:i16)->usize { let mut distributed=0u32; let lowThreshold=(total as u32)>>tableLog; let lowOne=((total as u64*3)>>(tableLog+1))as u32; for s in 0..=maxSymbolValue {let c=*count.add(s as usize);if c==0{*norm.add(s as usize)=0;}else if c<=lowThreshold{*norm.add(s as usize)=lowProbCount;distributed+=1;}else if c<=lowOne{*norm.add(s as usize)=1;distributed+=1;}else{*norm.add(s as usize)=-2;}} let mut remain=(1<<tableLog)-distributed;if remain==0{return 0;} let mut sum=0u64;for s in 0..=maxSymbolValue{if *norm.add(s as usize)==-2{sum+=*count.add(s as usize)as u64;}} if sum==0{return 0;} let step=((1u128<<(62-tableLog))*remain as u128/sum as u128)as u64;let mut tmp=(1u64<<((62-tableLog)-1))-1;for s in 0..=maxSymbolValue{if *norm.add(s as usize)==-2{let end=tmp+(*count.add(s as usize)as u64)*step;let w=((end>>(62-tableLog))-(tmp>>(62-tableLog)))as u32;if w<1{return ERROR(GENERIC);}*norm.add(s as usize)=w as i16;tmp=end;}}remain=0;let _=remain;0 }

pub unsafe fn FSE_normalizeCount(norm:*mut i16,mut tableLog:u32,count:*const u32,total:usize,maxSymbolValue:u32,useLowProbCount:u32)->usize {if tableLog==0{tableLog=FSE_DEFAULT_TABLELOG;}if tableLog<FSE_MIN_TABLELOG{return ERROR(GENERIC);}if tableLog>FSE_MAX_TABLELOG{return ERROR(tableLog_tooLarge);}if tableLog<FSE_minTableLog(total,maxSymbolValue){return ERROR(GENERIC);}let low=if useLowProbCount!=0{-1}else{1};let scale=62-tableLog;let step=ZSTD_div64(1u64<<62,total as u32);let mut left=1i32<<tableLog;let lowThreshold=(total as u32)>>tableLog;let mut largest=0;let mut largestP=0i16;for s in 0..=maxSymbolValue{let c=*count.add(s as usize);if c==total as u32{return 0;}if c==0{*norm.add(s as usize)=0;}else if c<=lowThreshold{*norm.add(s as usize)=low;left-=1;}else{let p=((c as u64*step)>>scale)as i16;if p>largestP{largestP=p;largest=s;}*norm.add(s as usize)=p;left-=p as i32;}}if left<0{FSE_normalizeM2(norm,tableLog,count,total,maxSymbolValue,low)}else{*norm.add(largest as usize)=(*norm.add(largest as usize)as i32+left)as i16;tableLog as usize}}

pub unsafe fn FSE_buildCTable_rle(ct:*mut FSE_CTable,symbolValue:u8)->usize{let p=ct as *mut u16;*p.add(0)=0;*p.add(1)=symbolValue as u16;*p.add(2)=0;*p.add(3)=0;0}

unsafe fn FSE_compress_usingCTable_generic(dst:*mut core::ffi::c_void,dstSize:usize,src:*const core::ffi::c_void,srcSize:usize,ct:*const FSE_CTable,fast:u32)->usize {
    if srcSize<=2{return 0;} let mut bitC=core::mem::MaybeUninit::<BIT_CStream_t>::uninit(); let mut s1=core::mem::MaybeUninit::<FSE_CState_t>::uninit();let mut s2=core::mem::MaybeUninit::<FSE_CState_t>::uninit(); let e=BIT_initCStream(bitC.as_mut_ptr(),dst,dstSize);if FSE_isError(e)!=0{return 0;}let mut ip=(src as *const u8).add(srcSize);let c1= s1.as_mut_ptr();let c2=s2.as_mut_ptr();if srcSize&1!=0{FSE_initCState2(c1,ct,*ip.sub(1));ip=ip.sub(1);FSE_initCState2(c2,ct,*ip.sub(1));ip=ip.sub(1);FSE_encodeSymbol(bitC.as_mut_ptr(),c1,*ip.sub(1));ip=ip.sub(1);if fast!=0{BIT_flushBitsFast(bitC.as_mut_ptr());}else{BIT_flushBits(bitC.as_mut_ptr());}}else{FSE_initCState2(c2,ct,*ip.sub(1));ip=ip.sub(1);FSE_initCState2(c1,ct,*ip.sub(1));ip=ip.sub(1);}while ip>(src as *const u8){FSE_encodeSymbol(bitC.as_mut_ptr(),c2,*ip.sub(1));ip=ip.sub(1);FSE_encodeSymbol(bitC.as_mut_ptr(),c1,*ip.sub(1));ip=ip.sub(1);if fast!=0{BIT_flushBitsFast(bitC.as_mut_ptr());}else{BIT_flushBits(bitC.as_mut_ptr());}}FSE_flushCState(bitC.as_mut_ptr(),c2);FSE_flushCState(bitC.as_mut_ptr(),c1);BIT_closeCStream(bitC.as_mut_ptr())
}

pub unsafe fn FSE_compress_usingCTable(dst:*mut core::ffi::c_void,dstSize:usize,src:*const core::ffi::c_void,srcSize:usize,ct:*const FSE_CTable)->usize{FSE_compress_usingCTable_generic(dst,dstSize,src,srcSize,ct,(dstSize>=FSE_BLOCKBOUND(srcSize))as u32)}

pub fn FSE_compressBound(size:usize)->usize{FSE_COMPRESSBOUND(size)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
