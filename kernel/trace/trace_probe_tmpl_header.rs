/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Traceprobe fetch helper inlines
 */

unsafe fn fetch_store_raw(val: ::std::os::raw::c_ulong, code: *mut fetch_insn, buf: *mut ::std::ffi::c_void) {
    match (*code).size {
        1 => *(buf as *mut u8) = val as u8,
        2 => *(buf as *mut u16) = val as u16,
        4 => *(buf as *mut u32) = val as u32,
        8 => {
            //TBD: 32bit signed
            *(buf as *mut u64) = val as u64;
        }
        _ => *(buf as *mut ::std::os::raw::c_ulong) = val,
    }
}

unsafe fn fetch_apply_bitfield(code: *mut fetch_insn, buf: *mut ::std::ffi::c_void) {
    match (*code).basesize {
        1 => { *(buf as *mut u8) <<= (*code).lshift; *(buf as *mut u8) >>= (*code).rshift; }
        2 => { *(buf as *mut u16) <<= (*code).lshift; *(buf as *mut u16) >>= (*code).rshift; }
        4 => { *(buf as *mut u32) <<= (*code).lshift; *(buf as *mut u32) >>= (*code).rshift; }
        8 => { *(buf as *mut u64) <<= (*code).lshift; *(buf as *mut u64) >>= (*code).rshift; }
        _ => {}
    }
}

extern "C" {
    fn process_fetch_insn(code: *mut fetch_insn, rec: *mut ::std::ffi::c_void, edata: *mut ::std::ffi::c_void, dest: *mut ::std::ffi::c_void, base: *mut ::std::ffi::c_void) -> i32;
    fn fetch_store_strlen(addr: ::std::os::raw::c_ulong) -> i32;
    fn fetch_store_string(addr: ::std::os::raw::c_ulong, dest: *mut ::std::ffi::c_void, base: *mut ::std::ffi::c_void) -> i32;
    fn fetch_store_strlen_user(addr: ::std::os::raw::c_ulong) -> i32;
    fn fetch_store_string_user(addr: ::std::os::raw::c_ulong, dest: *mut ::std::ffi::c_void, base: *mut ::std::ffi::c_void) -> i32;
    fn probe_mem_read(dest: *mut ::std::ffi::c_void, src: *mut ::std::ffi::c_void, size: usize) -> i32;
    fn probe_mem_read_user(dest: *mut ::std::ffi::c_void, src: *mut ::std::ffi::c_void, size: usize) -> i32;
}

unsafe fn fetch_store_symstrlen(addr: ::std::os::raw::c_ulong) -> i32 {
    let mut namebuf = [0i8; KSYM_SYMBOL_LEN];
    let ret = sprint_symbol(namebuf.as_mut_ptr(), addr);
    if ret < 0 { return 0; }
    ret + 1
}

unsafe fn fetch_store_symstring(addr: ::std::os::raw::c_ulong, dest: *mut ::std::ffi::c_void, base: *mut ::std::ffi::c_void) -> i32 {
    let maxlen = get_loc_len(*(dest as *mut u32));
    if !maxlen { return -ENOMEM; }
    let __dest = get_loc_data(dest, base);
    sprint_symbol(__dest, addr)
}

unsafe fn process_common_fetch_insn(code: *mut fetch_insn, val: *mut ::std::os::raw::c_ulong) -> i32 {
    match (*code).op {
        FETCH_OP_IMM => *val = (*code).immediate,
        FETCH_OP_COMM => *val = current.comm as ::std::os::raw::c_ulong,
        FETCH_OP_IMMSTR => *val = (*code).data as ::std::os::raw::c_ulong,
        FETCH_OP_CURRENT => *val = current as ::std::os::raw::c_ulong,
        _ => return -EILSEQ,
    }
    0
}

unsafe fn process_fetch_insn_bottom(mut code: *mut fetch_insn, mut val: ::std::os::raw::c_ulong, dest: *mut ::std::ffi::c_void, base: *mut ::std::ffi::c_void) -> i32 {
    let mut s3: *mut fetch_insn;
    let mut total = 0;
    let mut ret = 0;
    let mut i = 0;
    let mut loc: u32 = 0;
    let mut lval;
    let mut llval = val;
    'stage2: loop {
        loop {
            lval = val;
            match (*code).op {
                FETCH_OP_DEREF => ret = probe_mem_read(&mut val as *mut _ as *mut _, (val as *mut u8).offset((*code).offset as isize) as *mut _, std::mem::size_of_val(&val)),
                FETCH_OP_UDEREF => ret = probe_mem_read_user(&mut val as *mut _ as *mut _, (val as *mut u8).offset((*code).offset as isize) as *mut _, std::mem::size_of_val(&val)),
                FETCH_OP_CPU_PTR => { val = this_cpu_ptr(val as *mut ::std::ffi::c_void) as ::std::os::raw::c_ulong; ret = 0; }
                _ => { lval = llval; break; }
            }
            if ret != 0 { return ret; }
            llval = lval;
            code = code.add(1);
        }
        s3 = code;
        'stage3: loop {
            if dest.is_null() {
                match (*code).op {
                    FETCH_OP_ST_STRING => { ret = fetch_store_strlen(val.wrapping_add((*code).offset as _)); code = code.add(1); }
                    FETCH_OP_ST_USTRING => { ret = fetch_store_strlen_user(val.wrapping_add((*code).offset as _)); code = code.add(1); }
                    FETCH_OP_ST_SYMSTR => { ret = fetch_store_symstrlen(val.wrapping_add((*code).offset as _)); code = code.add(1); }
                    _ => return -EILSEQ,
                }
            } else {
                match (*code).op {
                    FETCH_OP_ST_RAW => fetch_store_raw(val, code, dest),
                    FETCH_OP_ST_MEM => { probe_mem_read(dest, (val as *mut u8).offset((*code).offset as isize) as *mut _, (*code).size as usize); }
                    FETCH_OP_ST_UMEM => { probe_mem_read_user(dest, (val as *mut u8).offset((*code).offset as isize) as *mut _, (*code).size as usize); }
                    FETCH_OP_ST_STRING => { loc = *(dest as *mut u32); ret = fetch_store_string(val.wrapping_add((*code).offset as _), dest, base); }
                    FETCH_OP_ST_USTRING => { loc = *(dest as *mut u32); ret = fetch_store_string_user(val.wrapping_add((*code).offset as _), dest, base); }
                    FETCH_OP_ST_SYMSTR => { loc = *(dest as *mut u32); ret = fetch_store_symstring(val.wrapping_add((*code).offset as _), dest, base); }
                    _ => return -EILSEQ,
                }
                code = code.add(1);
                if (*code).op == FETCH_OP_MOD_BF { fetch_apply_bitfield(code, dest); code = code.add(1); }
            }
            if (*code).op == FETCH_OP_LP_ARRAY {
                if ret < 0 { ret = 0; }
                total += ret; i += 1;
                if i < (*code).param {
                    code = s3;
                    if (*s3).op != FETCH_OP_ST_STRING && (*s3).op != FETCH_OP_ST_USTRING { dest = dest.add((*s3).size as usize); val = val.wrapping_add((*s3).size as _); continue 'stage3; }
                    code = code.sub(1); val = lval.wrapping_add(std::mem::size_of::<*mut ::std::ffi::c_void>() as _);
                    if !dest.is_null() { dest = dest.add(4); *(dest as *mut u32) = update_data_loc(loc, ret); }
                    continue 'stage2;
                }
                code = code.add(1); ret = total;
            }
            return if (*code).op == FETCH_OP_END { ret } else { -EILSEQ };
        }
    }
}

unsafe fn __get_data_size(tp: *mut trace_probe, regs: *mut ::std::ffi::c_void, edata: *mut ::std::ffi::c_void) -> i32 {
    let mut ret = 0;
    for i in 0..(*tp).nr_args { let arg = (*tp).args.add(i as usize); if (*arg).dynamic { let len = process_fetch_insn((*arg).code, regs, edata, std::ptr::null_mut(), std::ptr::null_mut()); if len > 0 { ret += len; } } }
    ret
}

unsafe fn store_trace_args(data: *mut ::std::ffi::c_void, tp: *mut trace_probe, rec: *mut ::std::ffi::c_void, edata: *mut ::std::ffi::c_void, header_size: i32, mut maxlen: i32) {
    let base = data.offset(-(header_size as isize));
    let mut dyndata = data.add((*tp).size as usize);
    for i in 0..(*tp).nr_args { let arg = (*tp).args.add(i as usize); let dl = data.add((*arg).offset as usize); if (*arg).dynamic { *(dl as *mut u32) = make_data_loc(maxlen, dyndata.offset_from(base) as u32); } let ret = process_fetch_insn((*arg).code, rec, edata, dl, base); if (*arg).dynamic && ret > 0 { dyndata = dyndata.add(ret as usize); maxlen -= ret; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
