// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* FSE : Finite State Entropy decoder.  Translated from fse_decompress.c. */

// Dependencies supplied by the surrounding translation unit:
// debug.h, bitstream.h, compiler.h, fse.h, error_private.h, zstd_deps.h, bits.h

static unsafe fn fse_build_dtable_internal(
    dt: *mut FSE_DTable, normalized_counter: *const i16, max_symbol_value: u32,
    table_log: u32, workspace: *mut c_void, wksp_size: usize,
) -> usize {
    let td_ptr = dt.add(1) as *mut c_void;
    let table_decode = td_ptr as *mut FSE_DECODE_TYPE;
    let symbol_next = workspace as *mut u16;
    let spread = symbol_next.add(max_symbol_value as usize + 1) as *mut u8;
    let max_sv1 = max_symbol_value + 1;
    let table_size = 1u32 << table_log;
    let mut high_threshold = table_size - 1;

    if FSE_BUILD_DTABLE_WKSP_SIZE(table_log, max_symbol_value) as usize > wksp_size { return ERROR(maxSymbolValue_tooLarge); }
    if max_symbol_value > FSE_MAX_SYMBOL_VALUE { return ERROR(maxSymbolValue_tooLarge); }
    if table_log > FSE_MAX_TABLELOG { return ERROR(tableLog_tooLarge); }

    let mut dtable_h = FSE_DTableHeader { tableLog: table_log as u16, fastMode: 1 };
    let large_limit = (1i32 << (table_log - 1)) as i16;
    for s in 0..max_sv1 {
        let n = *normalized_counter.add(s as usize);
        if n == -1 {
            (*table_decode.add(high_threshold as usize)).symbol = s as FSE_FUNCTION_TYPE;
            high_threshold -= 1;
            *symbol_next.add(s as usize) = 1;
        } else {
            if n >= large_limit { dtable_h.fastMode = 0; }
            *symbol_next.add(s as usize) = n as u16;
        }
    }
    ZSTD_memcpy(dt as *mut c_void, &dtable_h as *const _ as *const c_void, core::mem::size_of::<FSE_DTableHeader>());

    if high_threshold == table_size - 1 {
        let table_mask = (table_size - 1) as usize;
        let step = FSE_TABLESTEP(table_size) as usize;
        let add = 0x0101010101010101u64;
        let mut pos = 0usize;
        let mut sv = 0u64;
        for s in 0..max_sv1 {
            let n = *normalized_counter.add(s as usize) as usize;
            MEM_write64(spread.add(pos), sv);
            let mut i = 8usize;
            while i < n { MEM_write64(spread.add(pos + i), sv); i += 8; }
            pos += n; sv = sv.wrapping_add(add);
        }
        let mut position = 0usize;
        for s in (0..table_size as usize).step_by(2) {
            let p0 = (position + step * 0) & table_mask;
            let p1 = (position + step) & table_mask;
            (*table_decode.add(p0)).symbol = *spread.add(s);
            (*table_decode.add(p1)).symbol = *spread.add(s + 1);
            position = (position + 2 * step) & table_mask;
        }
    } else {
        let table_mask = table_size - 1;
        let step = FSE_TABLESTEP(table_size);
        let mut position = 0u32;
        for s in 0..max_sv1 {
            let n = *normalized_counter.add(s as usize);
            for _ in 0..n {
                (*table_decode.add(position as usize)).symbol = s as FSE_FUNCTION_TYPE;
                position = (position + step) & table_mask;
                while position > high_threshold { position = (position + step) & table_mask; }
            }
        }
        if position != 0 { return ERROR(GENERIC); }
    }

    for u in 0..table_size {
        let symbol = (*table_decode.add(u as usize)).symbol as usize;
        let next_state = *symbol_next.add(symbol);
        *symbol_next.add(symbol) = next_state + 1;
        (*table_decode.add(u as usize)).nbBits = (table_log - ZSTD_highbit32(next_state as u32)) as u8;
        (*table_decode.add(u as usize)).newState = ((next_state as u32) << (*table_decode.add(u as usize)).nbBits) as u16 - table_size as u16;
    }
    0
}

pub unsafe fn FSE_buildDTable_wksp(dt: *mut FSE_DTable, normalizedCounter: *const i16, maxSymbolValue: u32, tableLog: u32, workSpace: *mut c_void, wkspSize: usize) -> usize {
    fse_build_dtable_internal(dt, normalizedCounter, maxSymbolValue, tableLog, workSpace, wkspSize)
}

#[cfg(not(FSE_COMMONDEFS_ONLY))]
unsafe fn fse_decompress_using_dtable_generic(dst: *mut c_void, max_dst_size: usize, c_src: *const c_void, c_src_size: usize, dt: *const FSE_DTable, fast: u32) -> usize {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(max_dst_size);
    let olimit = omax.sub(3);
    let mut bit_d: BIT_DStream_t = core::mem::zeroed();
    let mut state1: FSE_DState_t = core::mem::zeroed();
    let mut state2: FSE_DState_t = core::mem::zeroed();
    CHECK_F(BIT_initDStream(&mut bit_d, c_src, c_src_size));
    FSE_initDState(&mut state1, &mut bit_d, dt);
    FSE_initDState(&mut state2, &mut bit_d, dt);
    RETURN_ERROR_IF(BIT_reloadDStream(&mut bit_d) == BIT_DStream_overflow, corruption_detected, "");
    while (BIT_reloadDStream(&mut bit_d) == BIT_DStream_unfinished) && op < olimit {
        *op.add(0) = if fast != 0 { FSE_decodeSymbolFast(&mut state1, &mut bit_d) } else { FSE_decodeSymbol(&mut state1, &mut bit_d) }; op = op.add(1);
        *op.add(0) = if fast != 0 { FSE_decodeSymbolFast(&mut state2, &mut bit_d) } else { FSE_decodeSymbol(&mut state2, &mut bit_d) }; op = op.add(1);
        *op.add(0) = if fast != 0 { FSE_decodeSymbolFast(&mut state1, &mut bit_d) } else { FSE_decodeSymbol(&mut state1, &mut bit_d) }; op = op.add(1);
        *op.add(0) = if fast != 0 { FSE_decodeSymbolFast(&mut state2, &mut bit_d) } else { FSE_decodeSymbol(&mut state2, &mut bit_d) }; op = op.add(1);
    }
    loop {
        if op > omax.sub(2) { return ERROR(dstSize_tooSmall); }
        *op = if fast != 0 { FSE_decodeSymbolFast(&mut state1, &mut bit_d) } else { FSE_decodeSymbol(&mut state1, &mut bit_d) }; op = op.add(1);
        if BIT_reloadDStream(&mut bit_d) == BIT_DStream_overflow { *op = if fast != 0 { FSE_decodeSymbolFast(&mut state2, &mut bit_d) } else { FSE_decodeSymbol(&mut state2, &mut bit_d) }; return op.add(1).offset_from(ostart) as usize; }
        if op > omax.sub(2) { return ERROR(dstSize_tooSmall); }
        *op = if fast != 0 { FSE_decodeSymbolFast(&mut state2, &mut bit_d) } else { FSE_decodeSymbol(&mut state2, &mut bit_d) }; op = op.add(1);
        if BIT_reloadDStream(&mut bit_d) == BIT_DStream_overflow { *op = if fast != 0 { FSE_decodeSymbolFast(&mut state1, &mut bit_d) } else { FSE_decodeSymbol(&mut state1, &mut bit_d) }; return op.add(1).offset_from(ostart) as usize; }
    }
}

#[repr(C)]
struct FSE_DecompressWksp { ncount: [i16; FSE_MAX_SYMBOL_VALUE as usize + 1] }

#[cfg(not(FSE_COMMONDEFS_ONLY))]
unsafe fn fse_decompress_wksp_body(dst: *mut c_void, dst_capacity: usize, c_src: *const c_void, c_src_size: usize, max_log: u32, workspace: *mut c_void, wksp_size: usize, bmi2: i32) -> usize {
    let istart = c_src as *const u8;
    let wksp = workspace as *mut FSE_DecompressWksp;
    let dtable_pos = core::mem::size_of::<FSE_DecompressWksp>() / core::mem::size_of::<FSE_DTable>();
    let dtable = (workspace as *mut FSE_DTable).add(dtable_pos);
    if wksp_size < core::mem::size_of::<FSE_DecompressWksp>() { return ERROR(GENERIC); }
    let mut max_symbol_value = FSE_MAX_SYMBOL_VALUE;
    let mut table_log = 0u32;
    let ncount_length = FSE_readNCount_bmi2((*wksp).ncount.as_mut_ptr(), &mut max_symbol_value, &mut table_log, istart, c_src_size, bmi2);
    if FSE_isError(ncount_length) { return ncount_length; }
    if table_log > max_log { return ERROR(tableLog_tooLarge); }
    let ip = istart.add(ncount_length);
    let remaining = c_src_size - ncount_length;
    if FSE_DECOMPRESS_WKSP_SIZE(table_log, max_symbol_value) > wksp_size { return ERROR(tableLog_tooLarge); }
    let used = core::mem::size_of::<FSE_DecompressWksp>() + FSE_DTABLE_SIZE(table_log);
    CHECK_F(fse_build_dtable_internal(dtable, (*wksp).ncount.as_ptr(), max_symbol_value, table_log, (workspace as *mut u8).add(used) as *mut c_void, wksp_size - used));
    let fast_mode = (*(dtable as *const FSE_DTableHeader)).fastMode as u32;
    fse_decompress_using_dtable_generic(dst, dst_capacity, ip as *const c_void, remaining, dtable, fast_mode)
}

#[cfg(not(FSE_COMMONDEFS_ONLY))]
pub unsafe fn FSE_decompress_wksp_bmi2(dst: *mut c_void, dstCapacity: usize, cSrc: *const c_void, cSrcSize: usize, maxLog: u32, workSpace: *mut c_void, wkspSize: usize, bmi2: i32) -> usize {
    fse_decompress_wksp_body(dst, dstCapacity, cSrc, cSrcSize, maxLog, workSpace, wkspSize, bmi2)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
