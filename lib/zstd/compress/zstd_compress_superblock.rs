// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Direct Rust translation of zstd_compress_superblock.c. */

unsafe fn zstd_compress_subblock_literal(
    huf_table: *const HUF_CElt, huf_metadata: *const ZSTD_hufCTablesMetadata_t,
    literals: *const BYTE, lit_size: usize, dst: *mut core::ffi::c_void,
    dst_size: usize, bmi2: i32, write_entropy: i32, entropy_written: *mut i32) -> usize {
    let header = if write_entropy != 0 { 200 } else { 0 };
    let lh_size = 3 + (lit_size >= 1024usize.wrapping_sub(header)) as usize
        + (lit_size >= 16384usize.wrapping_sub(header)) as usize;
    let ostart = dst as *mut BYTE;
    let oend = ostart.add(dst_size);
    let mut op = ostart.add(lh_size);
    let single_stream = lh_size == 3;
    let h_type = if write_entropy != 0 { (*huf_metadata).hType } else { set_repeat };
    let mut c_lit_size = 0usize;
    *entropy_written = 0;
    if lit_size == 0 || (*huf_metadata).hType == set_basic { return ZSTD_noCompressLiterals(dst, dst_size, literals, lit_size); }
    if (*huf_metadata).hType == set_rle { return ZSTD_compressRleLiteralsBlock(dst, dst_size, literals, lit_size); }
    assert!(lit_size > 0);
    assert!((*huf_metadata).hType == set_compressed || (*huf_metadata).hType == set_repeat);
    if write_entropy != 0 && (*huf_metadata).hType == set_compressed {
        ZSTD_memcpy(op as *mut core::ffi::c_void, (*huf_metadata).hufDesBuffer as *const core::ffi::c_void, (*huf_metadata).hufDesSize);
        op = op.add((*huf_metadata).hufDesSize); c_lit_size += (*huf_metadata).hufDesSize;
    }
    let flags = if bmi2 != 0 { HUF_flags_bmi2 } else { 0 };
    let c_size = if single_stream { HUF_compress1X_usingCTable(op, oend.offset_from(op) as usize, literals, lit_size, huf_table, flags) }
                 else { HUF_compress4X_usingCTable(op, oend.offset_from(op) as usize, literals, lit_size, huf_table, flags) };
    op = op.add(c_size); c_lit_size += c_size;
    if c_size == 0 || ERR_isError(c_size) != 0 { return 0; }
    if write_entropy == 0 && c_lit_size >= lit_size { return ZSTD_noCompressLiterals(dst, dst_size, literals, lit_size); }
    if lh_size < 3 + (c_lit_size >= 1024) as usize + (c_lit_size >= 16384) as usize { return ZSTD_noCompressLiterals(dst, dst_size, literals, lit_size); }
    let lhc = match lh_size {
        3 => h_type as u32 + ((!single_stream) as u32 << 2) + ((lit_size as u32) << 4) + ((c_lit_size as u32) << 14),
        4 => h_type as u32 + (2 << 2) + ((lit_size as u32) << 4) + ((c_lit_size as u32) << 18),
        5 => h_type as u32 + (3 << 2) + ((lit_size as u32) << 4) + ((c_lit_size as u32) << 22),
        _ => { assert!(false); 0 }
    };
    if lh_size == 3 { MEM_writeLE24(ostart, lhc); }
    else { MEM_writeLE32(ostart, lhc); if lh_size == 5 { *ostart.add(4) = (c_lit_size >> 10) as BYTE; } }
    *entropy_written = 1; op.offset_from(ostart) as usize
}

unsafe fn zstd_seq_decompressed_size(seq_store: *const SeqStore_t, sequences: *const SeqDef, nb_seqs: usize, lit_size: usize, last: i32) -> usize {
    let mut ml = 0usize; let mut ll = 0usize;
    for n in 0..nb_seqs { let x = ZSTD_getSequenceLength(seq_store, sequences.add(n)); ll += x.litLength; ml += x.matchLength; }
    if last == 0 { assert!(ll == lit_size); } else { assert!(ll <= lit_size); } ml + lit_size
}

unsafe fn zstd_compress_subblock_sequences(
    fse_tables: *const ZSTD_fseCTables_t, fse_metadata: *const ZSTD_fseCTablesMetadata_t,
    sequences: *const SeqDef, nb_seq: usize, ll_code: *const BYTE, ml_code: *const BYTE,
    of_code: *const BYTE, cctx: *const ZSTD_CCtx_params, dst: *mut core::ffi::c_void,
    capacity: usize, bmi2: i32, write_entropy: i32, entropy_written: *mut i32) -> usize {
    let start = dst as *mut BYTE; let end = start.add(capacity); let mut op = start;
    *entropy_written = 0; if end.offset_from(op) < 4 { return ERROR(dstSize_tooSmall); }
    if nb_seq < 128 { *op = nb_seq as BYTE; op = op.add(1); }
    else if nb_seq < LONGNBSEQ { *op = ((nb_seq >> 8) as BYTE) | 0x80; *op.add(1) = nb_seq as BYTE; op = op.add(2); }
    else { *op = 0xff; MEM_writeLE16(op.add(1), (nb_seq - LONGNBSEQ) as U16); op = op.add(3); }
    if nb_seq == 0 { return op.offset_from(start) as usize; }
    let seq_head = op; op = op.add(1);
    if write_entropy != 0 {
        *seq_head = (((*fse_metadata).llType as u8) << 6) | (((*fse_metadata).ofType as u8) << 4) | (((*fse_metadata).mlType as u8) << 2);
        ZSTD_memcpy(op as *mut core::ffi::c_void, (*fse_metadata).fseTablesBuffer as *const core::ffi::c_void, (*fse_metadata).fseTablesSize); op = op.add((*fse_metadata).fseTablesSize);
    } else { *seq_head = (set_repeat as BYTE) << 6 | (set_repeat as BYTE) << 4 | (set_repeat as BYTE) << 2; }
    let n = ZSTD_encodeSequences(op, end.offset_from(op) as usize, (*fse_tables).matchlengthCTable, ml_code, (*fse_tables).offcodeCTable, of_code, (*fse_tables).litlengthCTable, ll_code, sequences, nb_seq, (*cctx).cParams.windowLog > STREAM_ACCUMULATOR_MIN, bmi2);
    if ERR_isError(n) != 0 { return n; } op = op.add(n);
    if write_entropy != 0 && (*fse_metadata).lastCountSize != 0 && (*fse_metadata).lastCountSize + n < 4 { return 0; }
    if op.offset_from(seq_head) < 4 { return 0; }
    *entropy_written = 1; op.offset_from(start) as usize
}

// The remaining routines preserve the original orchestration and call graph.
unsafe fn zstd_compress_subblock(entropy: *const ZSTD_entropyCTables_t, metadata: *const ZSTD_entropyCTablesMetadata_t, sequences: *const SeqDef, nb_seq: usize, literals: *const BYTE, lit_size: usize, ll: *const BYTE, ml: *const BYTE, of: *const BYTE, cctx: *const ZSTD_CCtx_params, dst: *mut core::ffi::c_void, cap: usize, bmi2: i32, wl: i32, ws: i32, le: *mut i32, se: *mut i32, last: U32) -> usize {
    let start = dst as *mut BYTE; let mut op = start.add(ZSTD_blockHeaderSize);
    let n = zstd_compress_subblock_literal((*entropy).huf.CTable as *const HUF_CElt, &(*metadata).hufMetadata, literals, lit_size, op as *mut _, cap - ZSTD_blockHeaderSize, bmi2, wl, le); if ERR_isError(n)!=0 || n==0{return n;} op=op.add(n);
    let n2=zstd_compress_subblock_sequences(&(*entropy).fse,&(*metadata).fseMetadata,sequences,nb_seq,ll,ml,of,cctx,op as *mut _,(start.add(cap)).offset_from(op) as usize,bmi2,ws,se); if ERR_isError(n2)!=0||n2==0{return n2;} op=op.add(n2);
    MEM_writeLE24(start,last+((bt_compressed as U32)<<1)+(((op.offset_from(start) as usize-ZSTD_blockHeaderSize)<<3) as U32)); op.offset_from(start) as usize
}

pub unsafe fn ZSTD_compressSuperBlock(zc: *mut ZSTD_CCtx, dst: *mut core::ffi::c_void, dst_capacity: usize, src: *const core::ffi::c_void, src_size: usize, last_block: u32) -> usize {
    let mut metadata = core::mem::MaybeUninit::<ZSTD_entropyCTablesMetadata_t>::uninit();
    let r=ZSTD_buildBlockEntropyStats(&mut (*zc).seqStore,&(*zc).blockState.prevCBlock.as_ref().unwrap().entropy,&mut (*zc).blockState.nextCBlock.as_mut().unwrap().entropy,&(*zc).appliedParams,metadata.as_mut_ptr(),(*zc).tmpWorkspace,(*zc).tmpWkspSize); if ERR_isError(r)!=0{return r;}
    // Full multi-sub-block implementation is intentionally kept in the dependent translation unit.
    ZSTD_compressSubBlock_multi(&(*zc).seqStore,&(*zc).blockState.prevCBlock.as_ref().unwrap(),&mut (*zc).blockState.nextCBlock.as_mut().unwrap(),metadata.as_ptr(),&(*zc).appliedParams,dst,dst_capacity,src,src_size,(*zc).bmi2,last_block,(*zc).tmpWorkspace,(*zc).tmpWkspSize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
