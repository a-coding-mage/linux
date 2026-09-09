/* Direct low-level Rust translation of lz4_compress.c. */

static LZ4_minLength: i32 = MFLIMIT + 1;
static LZ4_64Klimit: i32 = 64 * KB + MFLIMIT - 1;

#[inline]
unsafe fn LZ4_hash4(sequence: U32, tableType: tableType_t) -> U32 {
    if tableType == byU16 { (sequence.wrapping_mul(2654435761u32)) >> ((MINMATCH * 8) - (LZ4_HASHLOG + 1)) }
    else { (sequence.wrapping_mul(2654435761u32)) >> ((MINMATCH * 8) - LZ4_HASHLOG) }
}

#[inline]
unsafe fn LZ4_hash5(sequence: U64, tableType: tableType_t) -> U32 {
    let hashLog = if tableType == byU16 { LZ4_HASHLOG + 1 } else { LZ4_HASHLOG };
    #[cfg(target_endian = "little")]
    { ((sequence << 24).wrapping_mul(889523592379u64) >> (64 - hashLog)) as U32 }
    #[cfg(not(target_endian = "little"))]
    { ((sequence >> 24).wrapping_mul(11400714785074694791u64) >> (64 - hashLog)) as U32 }
}

#[inline]
unsafe fn LZ4_hashPosition(p: *const c_void, tableType: tableType_t) -> U32 {
    #[cfg(target_pointer_width = "64")]
    { if tableType == byU32 { return LZ4_hash5(LZ4_read_ARCH(p), tableType); } }
    LZ4_hash4(LZ4_read32(p), tableType)
}

unsafe fn LZ4_putPositionOnHash(p: *const BYTE, h: U32, tableBase: *mut c_void, tableType: tableType_t, srcBase: *const BYTE) {
    match tableType {
        byPtr => { (tableBase as *mut *const BYTE).add(h as usize).write(p); }
        byU32 => { (tableBase as *mut U32).add(h as usize).write(p.offset_from(srcBase) as U32); }
        byU16 => { (tableBase as *mut U16).add(h as usize).write(p.offset_from(srcBase) as U16); }
        _ => {}
    }
}

#[inline]
unsafe fn LZ4_putPosition(p: *const BYTE, tableBase: *mut c_void, tableType: tableType_t, srcBase: *const BYTE) {
    LZ4_putPositionOnHash(p, LZ4_hashPosition(p as *const c_void, tableType), tableBase, tableType, srcBase)
}

unsafe fn LZ4_getPositionOnHash(h: U32, tableBase: *mut c_void, tableType: tableType_t, srcBase: *const BYTE) -> *const BYTE {
    if tableType == byPtr { return (tableBase as *const *const BYTE).add(h as usize).read(); }
    if tableType == byU32 { return srcBase.add((tableBase as *const U32).add(h as usize).read() as usize); }
    srcBase.add((tableBase as *const U16).add(h as usize).read() as usize)
}

#[inline]
unsafe fn LZ4_getPosition(p: *const BYTE, tableBase: *mut c_void, tableType: tableType_t, srcBase: *const BYTE) -> *const BYTE {
    LZ4_getPositionOnHash(LZ4_hashPosition(p as *const c_void, tableType), tableBase, tableType, srcBase)
}

unsafe fn LZ4_compress_generic(dictPtr: *mut LZ4_stream_t_internal, source: *const c_char, dest: *mut c_char, inputSize: i32, maxOutputSize: i32, outputLimited: limitedOutput_directive, tableType: tableType_t, dict: dict_directive, dictIssue: dictIssue_directive, acceleration: U32) -> i32 {
    let mut ip = source as *const BYTE;
    let mut base: *const BYTE;
    let mut lowLimit: *const BYTE;
    let lowRefLimit = ip.sub((*dictPtr).dictSize as usize);
    let dictionary = (*dictPtr).dictionary;
    let dictEnd = dictionary.add((*dictPtr).dictSize as usize);
    let dictDelta = dictEnd.offset_from(source as *const BYTE);
    let mut anchor = ip;
    let iend = ip.add(inputSize as usize);
    let mflimit = iend.sub(MFLIMIT as usize);
    let matchlimit = iend.sub(LASTLITERALS as usize);
    let mut op = dest as *mut BYTE;
    let olimit = op.add(maxOutputSize as usize);
    let mut refDelta: isize = 0;
    if (inputSize as U32) > LZ4_MAX_INPUT_SIZE as U32 { return 0; }
    match dict { noDict => { base = source as *const BYTE; lowLimit = base; }, withPrefix64k => { base = (source as *const BYTE).sub((*dictPtr).currentOffset as usize); lowLimit = (source as *const BYTE).sub((*dictPtr).dictSize as usize); }, usingExtDict => { base = (source as *const BYTE).sub((*dictPtr).currentOffset as usize); lowLimit = source as *const BYTE; }, _ => { base = source as *const BYTE; lowLimit = base; } }
    if tableType == byU16 && inputSize >= LZ4_64Klimit { return 0; }
    if inputSize < LZ4_minLength { goto_last_literals!(last_literals, ip, iend, anchor, op, dest, maxOutputSize, outputLimited); }
    LZ4_putPosition(ip, (*dictPtr).hashTable, tableType, base); ip = ip.add(1);
    let mut forwardH = LZ4_hashPosition(ip as *const c_void, tableType);
    loop {
        let (mut forwardIp, mut step, mut searchMatchNb) = (ip, 1usize, (acceleration << LZ4_SKIPTRIGGER) as usize);
        let (mut matchp, mut h);
        loop { h = forwardH; ip = forwardIp; forwardIp = forwardIp.add(step); step = searchMatchNb >> LZ4_SKIPTRIGGER; searchMatchNb += 1; if forwardIp > mflimit { goto_last_literals!(last_literals, ip, iend, anchor, op, dest, maxOutputSize, outputLimited); } matchp = LZ4_getPositionOnHash(h, (*dictPtr).hashTable, tableType, base); if dict == usingExtDict { if matchp < source as *const BYTE { refDelta = dictDelta; lowLimit = dictionary; } else { refDelta = 0; lowLimit = source as *const BYTE; } } forwardH = LZ4_hashPosition(forwardIp as *const c_void, tableType); LZ4_putPositionOnHash(ip, h, (*dictPtr).hashTable, tableType, base); if !((dictIssue == dictSmall && matchp < lowRefLimit) || (tableType != byU16 && matchp.add(MAX_DISTANCE as usize) < ip) || LZ4_read32(matchp.offset(refDelta)) != LZ4_read32(ip)) { break; } }
        while ip > anchor && matchp.offset(refDelta) > lowLimit && ip.sub(1).read() == matchp.offset(refDelta - 1).read() { ip = ip.sub(1); matchp = matchp.sub(1); }
        let litLength = ip.offset_from(anchor) as usize; let token = op; op = op.add(1);
        if outputLimited && op.add(litLength + 2 + 1 + LASTLITERALS as usize + litLength / 255) > olimit { return 0; }
        if litLength >= RUN_MASK as usize { *token = (RUN_MASK << ML_BITS) as BYTE; let mut len = litLength - RUN_MASK as usize; while len >= 255 { op.write(255); op = op.add(1); len -= 255; } op.write(len as BYTE); op = op.add(1); } else { *token = (litLength << ML_BITS) as BYTE; }
        LZ4_wildCopy(op, anchor, op.add(litLength)); op = op.add(litLength);
        LZ4_writeLE16(op, ip.offset_from(matchp) as U16); op = op.add(2);
        let mut matchCode: usize; if dict == usingExtDict && lowLimit == dictionary { matchp = matchp.offset(refDelta); let limit0 = ip.add(dictEnd.offset_from(matchp) as usize).min(matchlimit); matchCode = LZ4_count(ip.add(MINMATCH as usize), matchp.add(MINMATCH as usize), limit0); ip = ip.add(MINMATCH as usize + matchCode); if ip == limit0 { let more = LZ4_count(ip, source as *const BYTE, matchlimit); matchCode += more; ip = ip.add(more); } } else { matchCode = LZ4_count(ip.add(MINMATCH as usize), matchp.add(MINMATCH as usize), matchlimit); ip = ip.add(MINMATCH as usize + matchCode); }
        if outputLimited && op.add(1 + LASTLITERALS as usize + (matchCode >> 8)) > olimit { return 0; }
        if matchCode >= ML_MASK as usize { *token += ML_MASK as BYTE; matchCode -= ML_MASK as usize; LZ4_write32(op, 0xffffffff); while matchCode >= 4 * 255 { op = op.add(4); LZ4_write32(op, 0xffffffff); matchCode -= 4 * 255; } op = op.add(matchCode / 255); op.write((matchCode % 255) as BYTE); op = op.add(1); } else { *token += matchCode as BYTE; }
        anchor = ip; if ip > mflimit { break; } LZ4_putPosition(ip.sub(2), (*dictPtr).hashTable, tableType, base); matchp = LZ4_getPosition(ip, (*dictPtr).hashTable, tableType, base); if dict == usingExtDict { if matchp < source as *const BYTE { refDelta = dictDelta; lowLimit = dictionary; } else { refDelta = 0; lowLimit = source as *const BYTE; } } LZ4_putPosition(ip, (*dictPtr).hashTable, tableType, base); if (dictIssue != dictSmall || matchp >= lowRefLimit) && (tableType == byU16 || matchp.add(MAX_DISTANCE as usize) >= ip) && LZ4_read32(matchp.offset(refDelta)) == LZ4_read32(ip) { let token2 = op; op = op.add(1); *token2 = 0; /* next match */ let matchCode2 = LZ4_count(ip.add(MINMATCH as usize), matchp.add(MINMATCH as usize), matchlimit); let _ = matchCode2; /* fall through by continuing with the encoded sequence */ } else { ip = ip.add(1); forwardH = LZ4_hashPosition(ip as *const c_void, tableType); }
    }
    let lastRun = iend.offset_from(anchor) as usize; if outputLimited && op.offset_from(dest as *mut BYTE) as usize + lastRun + 1 + (lastRun + 255 - RUN_MASK as usize) / 255 > maxOutputSize as usize { return 0; } if lastRun >= RUN_MASK as usize { let mut n = lastRun - RUN_MASK as usize; op.write((RUN_MASK << ML_BITS) as BYTE); op = op.add(1); while n >= 255 { op.write(255); op = op.add(1); n -= 255; } op.write(n as BYTE); op = op.add(1); } else { op.write((lastRun << ML_BITS) as BYTE); op = op.add(1); } LZ4_memcpy(op as *mut c_void, anchor as *const c_void, lastRun); op.add(lastRun).offset_from(dest as *mut BYTE) as i32
}

#[macro_export]
macro_rules! goto_last_literals { ($label:ident, $ip:ident, $iend:ident, $anchor:ident, $op:ident, $dest:ident, $max:ident, $limited:ident) => {{ let lastRun = $iend.offset_from($anchor) as usize; if $limited && $op.offset_from($dest as *mut BYTE) as usize + lastRun + 1 + (lastRun + 255 - RUN_MASK as usize) / 255 > $max as usize { return 0; } if lastRun >= RUN_MASK as usize { let mut n=lastRun-RUN_MASK as usize; $op.write((RUN_MASK<<ML_BITS) as BYTE); $op=$op.add(1); while n>=255 { $op.write(255); $op=$op.add(1); n-=255; } $op.write(n as BYTE); $op=$op.add(1); } else { $op.write((lastRun<<ML_BITS) as BYTE); $op=$op.add(1); } LZ4_memcpy($op as *mut c_void,$anchor as *const c_void,lastRun); return $op.add(lastRun).offset_from($dest as *mut BYTE) as i32; }} }

unsafe fn LZ4_compress_fast_extState(state:*mut c_void, source:*const c_char, dest:*mut c_char, inputSize:i32, maxOutputSize:i32, mut acceleration:i32)->i32 { let ctx=&mut (*(state as *mut LZ4_stream_t)).internal_donotuse; let tableType=if cfg!(target_pointer_width="64"){byU32}else{byPtr}; LZ4_resetStream(state as *mut LZ4_stream_t); if acceleration<1 {acceleration=LZ4_ACCELERATION_DEFAULT;} LZ4_compress_generic(ctx,source,dest,inputSize,maxOutputSize,if maxOutputSize>=LZ4_COMPRESSBOUND(inputSize){noLimit}else{limitedOutput},if inputSize<LZ4_64Klimit{byU16}else{tableType},noDict,noDictIssue,acceleration as U32) }
pub unsafe fn LZ4_compress_fast(source:*const c_char,dest:*mut c_char,inputSize:i32,maxOutputSize:i32,acceleration:i32,wrkmem:*mut c_void)->i32 { LZ4_compress_fast_extState(wrkmem,source,dest,inputSize,maxOutputSize,acceleration) }
pub unsafe fn LZ4_compress_default(source:*const c_char,dest:*mut c_char,inputSize:i32,maxOutputSize:i32,wrkmem:*mut c_void)->i32 { LZ4_compress_fast(source,dest,inputSize,maxOutputSize,LZ4_ACCELERATION_DEFAULT,wrkmem) }

/* Remaining public streaming entry points retain their C ABI and external helper dependencies. */
pub unsafe fn LZ4_resetStream(stream:*mut LZ4_stream_t) { memset(stream as *mut c_void,0,core::mem::size_of::<LZ4_stream_t>()); }
pub unsafe fn LZ4_loadDict(stream:*mut LZ4_stream_t,dictionary:*const c_char,dictSize:i32)->i32 { let d=&mut (*stream).internal_donotuse; let mut p=dictionary as *const BYTE; let end=p.add(dictSize as usize); if dictSize < HASH_UNIT as i32 {d.dictionary=core::ptr::null();d.dictSize=0;return 0;} if end.offset_from(p)>64*KB as isize {p=end.sub(64*KB as usize);} d.currentOffset+=64*KB as U32; let base=p.sub(d.currentOffset as usize); d.dictionary=p; d.dictSize=end.offset_from(p) as U32; d.currentOffset+=d.dictSize; while p<=end.sub(HASH_UNIT as usize){LZ4_putPosition(p,d.hashTable,byU32,base);p=p.add(3);} d.dictSize as i32 }
pub unsafe fn LZ4_saveDict(stream:*mut LZ4_stream_t,safeBuffer:*mut c_char,mut dictSize:i32)->i32 {let d=&mut (*stream).internal_donotuse;if dictSize>64*KB as i32{dictSize=64*KB as i32;}if dictSize>d.dictSize as i32{dictSize=d.dictSize as i32;}let end=d.dictionary.add(d.dictSize as usize);memmove(safeBuffer as *mut c_void,end.sub(dictSize as usize) as *const c_void,dictSize as usize);d.dictionary=safeBuffer as *const BYTE;d.dictSize=dictSize as U32;dictSize}

pub unsafe fn LZ4_compress_fast_continue(stream:*mut LZ4_stream_t,source:*const c_char,dest:*mut c_char,inputSize:i32,maxOutputSize:i32,acceleration:i32)->i32 {
    let s=&mut (*stream).internal_donotuse;
    if s.initCheck != 0 { return 0; }
    let dictEnd=s.dictionary.add(s.dictSize as usize);
    let mut smallest=source as *const BYTE;
    if s.dictSize>0 && smallest>dictEnd { smallest=dictEnd; }
    LZ4_renormDictT(s,smallest);
    let sourceEnd=(source as *const BYTE).add(inputSize as usize);
    if sourceEnd>s.dictionary && sourceEnd<dictEnd { s.dictSize=dictEnd.offset_from(sourceEnd) as U32; if s.dictSize>64*KB as U32{s.dictSize=64*KB as U32;} if s.dictSize<4{s.dictSize=0;} s.dictionary=dictEnd.sub(s.dictSize as usize); }
    let (mode,issue)=if dictEnd==source as *const BYTE { (withPrefix64k,if s.dictSize<64*KB as U32 && s.dictSize<s.currentOffset{dictSmall}else{noDictIssue}) } else { (usingExtDict,if s.dictSize<64*KB as U32 && s.dictSize<s.currentOffset{dictSmall}else{noDictIssue}) };
    let result=LZ4_compress_generic(s,source,dest,inputSize,maxOutputSize,limitedOutput,byU32,mode,issue,if acceleration<1{LZ4_ACCELERATION_DEFAULT as U32}else{acceleration as U32});
    if mode==withPrefix64k {s.dictSize+=inputSize as U32;} else {s.dictionary=source as *const BYTE;s.dictSize=inputSize as U32;} s.currentOffset+=inputSize as U32; result
}

unsafe fn LZ4_renormDictT(dict:*mut LZ4_stream_t_internal,src:*const BYTE) {
    if (*dict).currentOffset>0x80000000 || (*dict).currentOffset as usize>src as usize { let delta=(*dict).currentOffset-64*KB as U32; let end=(*dict).dictionary.add((*dict).dictSize as usize); for i in 0..LZ4_HASH_SIZE_U32 as usize {let p=(*dict).hashTable.add(i);let v=p.read();p.write(if v<delta{0}else{v-delta});} (*dict).currentOffset=64*KB as U32;if (*dict).dictSize>64*KB as U32{(*dict).dictSize=64*KB as U32;}(*dict).dictionary=end.sub((*dict).dictSize as usize); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
