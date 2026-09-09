// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Translated from zstd_compress_sequences.c. External symbols are supplied by
 * the surrounding zstd translation. */

static K_INVERSE_PROBABILITY_LOG256: [u32; 256] = [
    0,2048,1792,1642,1536,1453,1386,1329,1280,1236,1197,1162,1130,1100,1073,1047,
    1024,1001,980,960,941,923,906,889,874,859,844,830,817,804,791,779,768,756,745,734,
    724,714,704,694,685,676,667,658,650,642,633,626,618,610,603,595,588,581,574,567,561,554,548,542,
    535,529,523,517,512,506,500,495,489,484,478,473,468,463,458,453,448,443,438,434,429,424,420,415,
    411,407,402,398,394,390,386,382,377,373,370,366,362,358,354,350,347,343,339,336,332,329,325,322,
    318,315,311,308,305,302,298,295,292,289,286,282,279,276,273,270,267,264,261,258,256,253,250,247,
    244,241,239,236,233,230,228,225,222,220,217,215,212,209,207,204,202,199,197,194,192,190,187,185,
    182,180,178,175,173,171,168,166,164,162,159,157,155,153,151,149,146,144,142,140,138,136,134,132,
    130,128,126,123,121,119,117,115,114,112,110,108,106,104,102,100,98,96,94,93,91,89,87,85,83,82,80,78,76,74,73,71,69,67,66,64,62,61,59,57,55,54,52,50,49,47,46,44,42,41,39,37,36,34,33,31,30,28,26,25,23,22,20,19,17,16,14,13,11,10,8,7,5,4,2,1,
];

unsafe fn ZSTD_getFSEMaxSymbolValue(ctable: *const FSE_CTable) -> u32 {
    let p = ctable as *const U16;
    MEM_read16(p.add(1)) as u32
}
unsafe fn ZSTD_useLowProbCount(nbSeq: size_t) -> unsigned { (nbSeq >= 2048) as unsigned }

unsafe fn ZSTD_NCountCost(count: *const unsigned, max: unsigned, nbSeq: size_t, FSELog: unsigned) -> size_t {
    let mut wksp = [0u8; FSE_NCOUNTBOUND];
    let mut norm = [0i16; MaxSeq + 1];
    let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
    FORWARD_IF_ERROR!(FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count, nbSeq, max, ZSTD_useLowProbCount(nbSeq)), "");
    FSE_writeNCount(wksp.as_mut_ptr(), wksp.len(), norm.as_ptr(), max, tableLog)
}

unsafe fn ZSTD_entropyCost(count: *const unsigned, max: unsigned, total: size_t) -> size_t {
    let mut cost: unsigned = 0;
    assert!(total > 0);
    for s in 0..=max {
        let mut norm = ((256 * *count.add(s as usize)) / total as unsigned) as unsigned;
        if *count.add(s as usize) != 0 && norm == 0 { norm = 1; }
        assert!((*count.add(s as usize) as size_t) < total);
        cost += *count.add(s as usize) * K_INVERSE_PROBABILITY_LOG256[norm as usize];
    }
    (cost >> 8) as size_t
}

pub unsafe fn ZSTD_fseBitCost(ctable: *const FSE_CTable, count: *const unsigned, max: unsigned) -> size_t {
    let mut cost: size_t = 0;
    let mut cstate = core::mem::zeroed::<FSE_CState_t>();
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max { return ERROR!(GENERIC); }
    for s in 0..=max {
        let tableLog = cstate.stateLog;
        let badCost = (tableLog + 1) << 8;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, 8);
        if *count.add(s as usize) == 0 { continue; }
        if bitCost >= badCost { return ERROR!(GENERIC); }
        cost += *count.add(s as usize) as size_t * bitCost as size_t;
    }
    cost >> 8
}

pub unsafe fn ZSTD_crossEntropyCost(norm: *const S16, accuracyLog: unsigned, count: *const unsigned, max: unsigned) -> size_t {
    let shift = 8 - accuracyLog;
    let mut cost: size_t = 0;
    assert!(accuracyLog <= 8);
    for s in 0..=max {
        let n = if *norm.add(s as usize) != -1 { *norm.add(s as usize) as unsigned } else { 1 };
        let n256 = n << shift;
        assert!(n256 > 0 && n256 < 256);
        cost += *count.add(s as usize) as size_t * K_INVERSE_PROBABILITY_LOG256[n256 as usize] as size_t;
    }
    cost >> 8
}

#[repr(C)]
pub struct ZSTD_BuildCTableWksp { pub norm: [S16; MaxSeq + 1], pub wksp: [U32; FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32!(MaxSeq, MaxFSELog)] }

pub unsafe fn ZSTD_buildCTable(dst: *mut core::ffi::c_void, dstCapacity: size_t, nextCTable: *mut FSE_CTable, FSELog: U32, typ: SymbolEncodingType_e, count: *mut unsigned, max: U32, codeTable: *const BYTE, nbSeq: size_t, defaultNorm: *const S16, defaultNormLog: U32, defaultMax: U32, prevCTable: *const FSE_CTable, prevCTableSize: size_t, entropyWorkspace: *mut core::ffi::c_void, entropyWorkspaceSize: size_t) -> size_t {
    let mut op = dst as *mut BYTE;
    let oend = op.add(dstCapacity);
    match typ {
        set_rle => { FORWARD_IF_ERROR!(FSE_buildCTable_rle(nextCTable, max as BYTE), ""); RETURN_ERROR_IF!(dstCapacity == 0, dstSize_tooSmall, "not enough space"); *op = *codeTable; 1 }
        set_repeat => { ZSTD_memcpy(nextCTable, prevCTable, prevCTableSize); 0 }
        set_basic => { FORWARD_IF_ERROR!(FSE_buildCTable_wksp(nextCTable, defaultNorm, defaultMax, defaultNormLog, entropyWorkspace, entropyWorkspaceSize), ""); 0 }
        set_compressed => {
            let wksp = &mut *(entropyWorkspace as *mut ZSTD_BuildCTableWksp);
            let mut n = nbSeq;
            let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
            let last = *codeTable.add(nbSeq - 1) as usize;
            if *count.add(last) > 1 { *count.add(last) -= 1; n -= 1; }
            assert!(n > 1 && entropyWorkspaceSize >= core::mem::size_of::<ZSTD_BuildCTableWksp>());
            FORWARD_IF_ERROR!(FSE_normalizeCount(wksp.norm.as_mut_ptr(), tableLog, count, n, max, ZSTD_useLowProbCount(n)), "FSE_normalizeCount failed");
            let sz = FSE_writeNCount(op, oend.offset_from(op) as size_t, wksp.norm.as_ptr(), max, tableLog);
            FORWARD_IF_ERROR!(sz, "FSE_writeNCount failed");
            FORWARD_IF_ERROR!(FSE_buildCTable_wksp(nextCTable, wksp.norm.as_ptr(), max, tableLog, wksp.wksp.as_mut_ptr(), core::mem::size_of_val(&wksp.wksp)), "FSE_buildCTable_wksp failed"); sz
        }
        _ => { assert!(false); RETURN_ERROR!(GENERIC, "impossible to reach") }
    }
}

pub unsafe fn ZSTD_encodeSequences(dst: *mut core::ffi::c_void, dstCapacity: size_t, CTable_MatchLength: *const FSE_CTable, mlCodeTable: *const BYTE, CTable_OffsetBits: *const FSE_CTable, ofCodeTable: *const BYTE, CTable_LitLength: *const FSE_CTable, llCodeTable: *const BYTE, sequences: *const SeqDef, nbSeq: size_t, longOffsets: int, _bmi2: int) -> size_t {
    ZSTD_encodeSequences_default(dst, dstCapacity, CTable_MatchLength, mlCodeTable, CTable_OffsetBits, ofCodeTable, CTable_LitLength, llCodeTable, sequences, nbSeq, longOffsets)
}
unsafe fn ZSTD_encodeSequences_default(dst:*mut core::ffi::c_void, cap:size_t, ml:*const FSE_CTable, mt:*const BYTE, of:*const FSE_CTable, ot:*const BYTE, ll:*const FSE_CTable, lt:*const BYTE, seq:*const SeqDef, n:size_t, lo:int)->size_t { ZSTD_encodeSequences_body(dst,cap,ml,mt,of,ot,ll,lt,seq,n,lo) }
unsafe fn ZSTD_encodeSequences_body(dst:*mut core::ffi::c_void, cap:size_t, ml:*const FSE_CTable, mt:*const BYTE, of:*const FSE_CTable, ot:*const BYTE, ll:*const FSE_CTable, lt:*const BYTE, seq:*const SeqDef, n:size_t, _lo:int)->size_t {
 let mut bs=core::mem::zeroed::<BIT_CStream_t>(); RETURN_ERROR_IF!(ERR_isError(BIT_initCStream(&mut bs,dst,cap)),dstSize_tooSmall,"not enough space remaining"); let mut a=core::mem::zeroed::<FSE_CState_t>(); let mut b=core::mem::zeroed::<FSE_CState_t>(); let mut c=core::mem::zeroed::<FSE_CState_t>(); FSE_initCState2(&mut a,ml,*mt.add(n-1)); FSE_initCState2(&mut b,of,*ot.add(n-1)); FSE_initCState2(&mut c,ll,*lt.add(n-1)); for i in (0..n-1).rev(){FSE_encodeSymbol(&mut bs,&mut b,*ot.add(i));FSE_encodeSymbol(&mut bs,&mut a,*mt.add(i));FSE_encodeSymbol(&mut bs,&mut c,*lt.add(i));} FSE_flushCState(&mut bs,&mut a);FSE_flushCState(&mut bs,&mut b);FSE_flushCState(&mut bs,&mut c); let z=BIT_closeCStream(&mut bs); RETURN_ERROR_IF!(z==0,dstSize_tooSmall,"not enough space"); z
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
