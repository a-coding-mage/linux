// SPDX-License-Identifier: Zlib

// Dependencies supplied by the corresponding zlib and dfltcc modules.

unsafe fn get_dfltcc_deflate_state(
    state: *mut deflate_state,
) -> *mut dfltcc_deflate_state {
    GET_DFLTCC_STATE(state) as *mut dfltcc_deflate_state
}

/*
 * Compress.
 */
#[no_mangle]
pub unsafe extern "C" fn dfltcc_can_deflate(strm: z_streamp) -> c_int {
    let state = (*strm).state as *mut deflate_state;
    let dfltcc_state = get_dfltcc_deflate_state(state);

    /* Check for kernel dfltcc command line parameter */
    if zlib_dfltcc_support == ZLIB_DFLTCC_DISABLED
        || zlib_dfltcc_support == ZLIB_DFLTCC_INFLATE_ONLY
    {
        return 0;
    }

    /* Unsupported compression settings */
    if !dfltcc_are_params_ok(
        (*state).level,
        (*state).w_bits,
        (*state).strategy,
        (*dfltcc_state).level_mask,
    ) {
        return 0;
    }

    /* Unsupported hardware */
    if !is_bit_set((*dfltcc_state).common.af.fns, DFLTCC_GDHT)
        || !is_bit_set((*dfltcc_state).common.af.fns, DFLTCC_CMPR)
        || !is_bit_set((*dfltcc_state).common.af.fmts, DFLTCC_FMT0)
    {
        return 0;
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn dfltcc_reset_deflate_state(strm: z_streamp) {
    let state = (*strm).state as *mut deflate_state;
    let dfltcc_state = get_dfltcc_deflate_state(state);

    dfltcc_reset_state(&mut (*dfltcc_state).common);

    /* Initialize tuning parameters */
    if zlib_dfltcc_support == ZLIB_DFLTCC_FULL_DEBUG {
        (*dfltcc_state).level_mask = DFLTCC_LEVEL_MASK_DEBUG;
    } else {
        (*dfltcc_state).level_mask = DFLTCC_LEVEL_MASK;
    }
    (*dfltcc_state).block_size = DFLTCC_BLOCK_SIZE;
    (*dfltcc_state).block_threshold = DFLTCC_FIRST_FHT_BLOCK_SIZE;
    (*dfltcc_state).dht_threshold = DFLTCC_DHT_MIN_SAMPLE_SIZE;
}

unsafe fn dfltcc_gdht(strm: z_streamp) {
    let state = (*strm).state as *mut deflate_state;
    let param = &mut GET_DFLTCC_STATE(state).param;
    let avail_in = (*strm).avail_in as usize;

    dfltcc(
        DFLTCC_GDHT,
        param,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut (*strm).next_in,
        &avail_in as *const usize as *mut usize,
        core::ptr::null_mut(),
    );
}

unsafe fn dfltcc_cmpr(strm: z_streamp) -> dfltcc_cc {
    let state = (*strm).state as *mut deflate_state;
    let param = &mut GET_DFLTCC_STATE(state).param;
    let mut avail_in = (*strm).avail_in as usize;
    let mut avail_out = (*strm).avail_out as usize;

    let cc = dfltcc(
        DFLTCC_CMPR | HBT_CIRCULAR,
        param,
        &mut (*strm).next_out,
        &mut avail_out,
        &mut (*strm).next_in,
        &mut avail_in,
        (*state).window,
    );
    (*strm).total_in += (*strm).avail_in - avail_in as _;
    (*strm).total_out += (*strm).avail_out - avail_out as _;
    (*strm).avail_in = avail_in as _;
    (*strm).avail_out = avail_out as _;
    cc
}

unsafe fn send_eobs(strm: z_streamp, param: *const dfltcc_param_v0) {
    let state = (*strm).state as *mut deflate_state;

    zlib_tr_send_bits(
        state,
        bi_reverse((*param).eobs >> (15 - (*param).eobl), (*param).eobl),
        (*param).eobl,
    );
    flush_pending(strm);
    if (*state).pending != 0 {
        /* The remaining data is located in pending_out[0:pending]. If someone
         * calls put_byte() - this might happen in deflate() - the byte will be
         * placed into pending_buf[pending], which is incorrect. Move the
         * remaining data to the beginning of pending_buf so that put_byte() is
         * usable again.
         */
        memmove(
            (*state).pending_buf,
            (*state).pending_out,
            (*state).pending,
        );
        (*state).pending_out = (*state).pending_buf;
    }
    #[cfg(feature = "ZLIB_DEBUG")]
    (*state).compressed_len += (*param).eobl;
}

#[no_mangle]
pub unsafe extern "C" fn dfltcc_deflate(
    strm: z_streamp,
    flush: c_int,
    result: *mut block_state,
) -> c_int {
    let state = (*strm).state as *mut deflate_state;
    let dfltcc_state = get_dfltcc_deflate_state(state);
    let param = &mut (*dfltcc_state).common.param;
    let mut masked_avail_in: uInt;
    let mut cc: dfltcc_cc;
    let mut need_empty_block: c_int;
    let mut soft_bcc: c_int;
    let mut no_flush: c_int;

    if dfltcc_can_deflate(strm) == 0 {
        /* Clear history. */
        if flush == Z_FULL_FLUSH {
            param.hl = 0;
        }
        return 0;
    }

    'again: loop {
        masked_avail_in = 0;
        soft_bcc = 0;
        no_flush = (flush == Z_NO_FLUSH) as c_int;

        /* No input data. Return, except when Continuation Flag is set, which means
         * that DFLTCC has buffered some output in the parameter block and needs
         * to be called again in order to flush it.
         */
        if (*strm).avail_in == 0 && param.cf == 0 {
            if no_flush == 0 && param.bcf != 0 {
                send_eobs(strm, param);
                param.bcf = 0;
            }
            if flush == Z_FINISH {
                return 0;
            }
            if flush == Z_FULL_FLUSH {
                param.hl = 0;
            }
            *result = if no_flush != 0 { need_more } else { block_done };
            return 1;
        }

        if param.bcf != 0
            && no_flush != 0
            && (*strm).total_in > (*dfltcc_state).block_threshold
            && (*strm).avail_in >= (*dfltcc_state).dht_threshold
        {
            if param.cf != 0 {
                masked_avail_in += (*strm).avail_in;
                (*strm).avail_in = 0;
                no_flush = 0;
            } else {
                send_eobs(strm, param);
                param.bcf = 0;
                (*dfltcc_state).block_threshold =
                    (*strm).total_in + (*dfltcc_state).block_size;
            }
        }

        if (*strm).avail_out == 0 {
            *result = need_more;
            return 1;
        }

        if no_flush != 0 && (*strm).avail_in > (*dfltcc_state).block_size {
            masked_avail_in += (*strm).avail_in - (*dfltcc_state).block_size;
            (*strm).avail_in = (*dfltcc_state).block_size;
        }

        need_empty_block = (flush == Z_FINISH && param.bcf != 0 && param.bhf == 0) as c_int;
        param.cvt = CVT_ADLER32;
        if no_flush == 0 {
            soft_bcc = 1;
        }
        if flush == Z_FINISH && param.bcf == 0 {
            param.bhf = 1;
        }
        Assert((*state).pending == 0, "There must be no pending bytes");
        Assert((*state).bi_valid < 8, "There must be less than 8 pending bits");
        param.sbb = (*state).bi_valid as _;
        if param.sbb > 0 {
            *(*strm).next_out = (*state).bi_buf as Byte;
        }
        param.nt = 0;
        param.cv = (*strm).adler;

        if param.bcf == 0 {
            if (*strm).total_in == 0 && (*dfltcc_state).block_threshold > 0 {
                param.htt = HTT_FIXED;
            } else {
                param.htt = HTT_DYNAMIC;
                dfltcc_gdht(strm);
            }
        }

        loop {
            cc = dfltcc_cmpr(strm);
            if (*strm).avail_in < 4096 && masked_avail_in > 0 {
                break;
            }
            if cc != DFLTCC_CC_AGAIN {
                break;
            }
        }

        (*strm).msg = oesc_msg((*dfltcc_state).common.msg, param.oesc);
        (*state).bi_valid = param.sbb;
        if (*state).bi_valid == 0 {
            (*state).bi_buf = 0;
        } else {
            (*state).bi_buf = *(*strm).next_out & ((1 << (*state).bi_valid) - 1);
        }
        (*strm).adler = param.cv;
        (*strm).avail_in += masked_avail_in;
        masked_avail_in = 0;
        Assert(cc != DFLTCC_CC_OP2_CORRUPT || param.oesc == 0, "BUG");

        if cc == DFLTCC_CC_OK {
            if soft_bcc != 0 {
                send_eobs(strm, param);
                param.bcf = 0;
                (*dfltcc_state).block_threshold =
                    (*strm).total_in + (*dfltcc_state).block_size;
            } else {
                param.bcf = 1;
            }
            if flush == Z_FINISH {
                if need_empty_block != 0 {
                    return 0;
                }
                bi_windup(state);
                *result = finish_done;
            } else {
                if flush == Z_FULL_FLUSH {
                    param.hl = 0;
                }
                *result = if flush == Z_NO_FLUSH { need_more } else { block_done };
            }
        } else {
            param.bcf = 1;
            *result = need_more;
        }
        if (*strm).avail_in != 0 && (*strm).avail_out != 0 {
            continue 'again;
        }
        return 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
