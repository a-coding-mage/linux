// SPDX-License-Identifier: GPL-2.0-only
/* Kernel Debug Core -- direct Rust translation of gdbstub.c. */

// Kernel-provided constants, types, globals, and functions are external dependencies.

const KGDB_MAX_THREAD_QUERY: i32 = 17;
const BUF_THREAD_ID_SIZE: usize = 8;

static mut remcom_in_buffer: [i8; BUFMAX] = [0; BUFMAX];
static mut remcom_out_buffer: [i8; BUFMAX] = [0; BUFMAX];
static mut gdbstub_use_prev_in_buf: i32 = 0;
static mut gdbstub_prev_in_buf_pos: i32 = 0;
static mut gdb_regs: [c_ulong; (NUMREGBYTES + core::mem::size_of::<c_ulong>() - 1) /
    core::mem::size_of::<c_ulong>()] = [0; (NUMREGBYTES + core::mem::size_of::<c_ulong>() - 1) /
    core::mem::size_of::<c_ulong>()];
static mut gdbmsgbuf: [i8; BUFMAX + 1] = [0; BUFMAX + 1];

type c_ulong = usize;

unsafe fn gdbstub_read_wait() -> i32 {
    let mut ret = dbg_io_ops.read_char();
    while ret == NO_POLL_CHAR { ret = dbg_io_ops.read_char(); }
    ret
}

unsafe fn get_packet(buffer: *mut i8) {
    let mut checksum: u8;
    let mut xmitcsum: u8;
    let mut count: usize;
    let mut ch: i32;
    loop {
        loop { ch = gdbstub_read_wait(); if ch == b'$' as i32 { break; } }
        kgdb_connected = 1;
        checksum = 0; xmitcsum = 255; count = 0;
        while count < BUFMAX - 1 {
            ch = gdbstub_read_wait(); if ch == b'#' as i32 { break; }
            checksum = checksum.wrapping_add(ch as u8);
            *buffer.add(count) = ch as i8; count += 1;
        }
        if ch == b'#' as i32 {
            xmitcsum = (hex_to_bin(gdbstub_read_wait() as i8) << 4) as u8;
            xmitcsum = xmitcsum.wrapping_add(hex_to_bin(gdbstub_read_wait() as i8) as u8);
            dbg_io_ops.write_char(if checksum != xmitcsum { b'-' as i8 } else { b'+' as i8 });
            if let Some(flush) = dbg_io_ops.flush { flush(); }
        }
        *buffer.add(count) = 0;
        if checksum == xmitcsum { break; }
    }
}

unsafe fn put_packet(buffer: *const i8) {
    loop {
        dbg_io_ops.write_char(b'$' as i8); let mut checksum = 0u8; let mut count = 0usize;
        while *buffer.add(count) != 0 { let ch = *buffer.add(count); dbg_io_ops.write_char(ch); checksum = checksum.wrapping_add(ch as u8); count += 1; }
        dbg_io_ops.write_char(b'#' as i8); dbg_io_ops.write_char(hex_asc_hi(checksum)); dbg_io_ops.write_char(hex_asc_lo(checksum));
        if let Some(flush) = dbg_io_ops.flush { flush(); }
        let mut ch = gdbstub_read_wait(); if ch == 3 { ch = gdbstub_read_wait(); }
        if ch == b'+' as i32 { return; }
        if ch == b'$' as i32 { dbg_io_ops.write_char(b'-' as i8); if let Some(flush) = dbg_io_ops.flush { flush(); } return; }
    }
}

pub unsafe fn gdbstub_msg_write(mut s: *const i8, mut len: i32) {
    if len == 0 { len = strlen(s) as i32; }
    gdbmsgbuf[0] = b'O' as i8;
    while len > 0 {
        let mut p = gdbmsgbuf.as_mut_ptr().add(1);
        let wcount = if (len << 1) > BUFMAX as i32 - 2 { (BUFMAX as i32 - 2) >> 1 } else { len };
        for i in 0..wcount { p = hex_byte_pack(p, *s.add(i as usize)); }
        *p = 0; s = s.add(wcount as usize); len -= wcount; put_packet(gdbmsgbuf.as_ptr());
    }
}

pub unsafe fn kgdb_mem2hex(mut mem: *mut i8, mut buf: *mut i8, mut count: usize) -> *mut i8 {
    let mut tmp = buf.add(count);
    if copy_from_kernel_nofault(tmp, mem, count) != 0 { return core::ptr::null_mut(); }
    while count > 0 { buf = hex_byte_pack(buf, *tmp); tmp = tmp.add(1); count -= 1; }
    *buf = 0; buf
}

pub unsafe fn kgdb_hex2mem(mut buf: *mut i8, mem: *mut i8, count: usize) -> i32 {
    let mut raw = buf.add(count * 2); let mut hex = raw.offset(-1);
    while hex >= buf { raw = raw.offset(-1); *raw = hex_to_bin(*hex) as i8; hex = hex.offset(-1); *raw |= (hex_to_bin(*hex) << 4) as i8; hex = hex.offset(-1); }
    copy_to_kernel_nofault(mem, raw, count)
}

pub unsafe fn kgdb_hex2long(ptr: *mut *mut i8, val: *mut c_ulong) -> i32 {
    let mut n = 0; let mut negate = false; *val = 0;
    if **ptr == b'-' as i8 { negate = true; *ptr = (*ptr).add(1); }
    while **ptr != 0 { let h = hex_to_bin(**ptr); if h < 0 { break; } *val = (*val << 4) | h as usize; n += 1; *ptr = (*ptr).add(1); }
    if negate { *val = 0usize.wrapping_sub(*val); } n
}

unsafe fn kgdb_ebin2mem(buf: *mut i8, mem: *mut i8, mut count: usize) -> i32 {
    let mut size = 0; let mut p = buf;
    while count > 0 { *buf.add(size) = *p; p = p.add(1); if *buf.add(size) == 0x7d { *buf.add(size) = (*p ^ 0x20); p = p.add(1); } size += 1; count -= 1; }
    copy_to_kernel_nofault(mem, buf, size)
}

unsafe fn write_mem_msg(binary: bool) -> i32 {
    let mut p = remcom_in_buffer.as_mut_ptr().add(1); let mut addr = 0usize; let mut len = 0usize;
    if kgdb_hex2long(&mut p, &mut addr) > 0 && { let x = *p; p = p.add(1); x == b',' as i8 } && kgdb_hex2long(&mut p, &mut len) > 0 && { let x = *p; p = p.add(1); x == b':' as i8 } {
        let e = if binary { kgdb_ebin2mem(p, addr as *mut i8, len) } else { kgdb_hex2mem(p, addr as *mut i8, len) };
        if e != 0 { return e; } if CACHE_FLUSH_IS_SAFE { flush_icache_range(addr, addr + len); } return 0;
    } -EINVAL
}

unsafe fn error_packet(pkt: *mut i8, mut error: i32) { error = -error; *pkt = b'E' as i8; *pkt.add(1) = hex_asc[(error / 10) as usize] as i8; *pkt.add(2) = hex_asc[(error % 10) as usize] as i8; *pkt.add(3) = 0; }

unsafe fn pack_threadid(mut pkt: *mut i8, mut id: *mut u8) -> *mut i8 { let mut zero = true; for _ in 0..BUF_THREAD_ID_SIZE / 2 { if !zero || *id != 0 { pkt = hex_byte_pack(pkt, *id as i8); zero = false; } id = id.add(1); } if zero { pkt = hex_byte_pack(pkt, 0); } pkt }
unsafe fn int_to_threadref(id: *mut u8, value: i32) { put_unaligned_be32(value, id); }
unsafe fn shadow_pid(realpid: i32) -> i32 { if realpid != 0 { realpid } else { -raw_smp_processor_id() - 2 } }

unsafe fn gdb_cmd_status(ks: *mut kgdb_state) { dbg_remove_all_break(); remcom_out_buffer[0] = b'S' as i8; hex_byte_pack(remcom_out_buffer.as_mut_ptr().add(1), (*ks).signo); }
unsafe fn gdb_get_regs_helper(ks: *mut kgdb_state) { let t = if !kgdb_usethread.is_null() { kgdb_usethread } else { kgdb_info[(*ks).cpu].task }; sleeping_thread_to_gdb_regs(gdb_regs.as_mut_ptr(), t); }
unsafe fn gdb_cmd_getregs(ks: *mut kgdb_state) { gdb_get_regs_helper(ks); kgdb_mem2hex(gdb_regs.as_mut_ptr() as *mut i8, remcom_out_buffer.as_mut_ptr(), NUMREGBYTES); }
unsafe fn gdb_cmd_setregs(ks: *mut kgdb_state) { kgdb_hex2mem(remcom_in_buffer.as_mut_ptr().add(1), gdb_regs.as_mut_ptr() as *mut i8, NUMREGBYTES); if !kgdb_usethread.is_null() && kgdb_usethread != current { error_packet(remcom_out_buffer.as_mut_ptr(), -EINVAL); } else { strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); } }
unsafe fn gdb_cmd_memread(_ks: *mut kgdb_state) { let mut p = remcom_in_buffer.as_mut_ptr().add(1); let mut a = 0usize; let mut n = 0usize; if kgdb_hex2long(&mut p, &mut a) > 0 && *p == b',' as i8 { p = p.add(1); if kgdb_hex2long(&mut p, &mut n) > 0 && kgdb_mem2hex(a as *mut i8, remcom_out_buffer.as_mut_ptr(), n).is_null() { error_packet(remcom_out_buffer.as_mut_ptr(), -EINVAL); return; } } else { error_packet(remcom_out_buffer.as_mut_ptr(), -EINVAL); } }
unsafe fn gdb_cmd_memwrite(_ks: *mut kgdb_state) { let e = write_mem_msg(false); if e != 0 { error_packet(remcom_out_buffer.as_mut_ptr(), e); } else { strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); } }
unsafe fn gdb_cmd_binwrite(_ks: *mut kgdb_state) { let e = write_mem_msg(true); if e != 0 { error_packet(remcom_out_buffer.as_mut_ptr(), e); } else { strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); } }
unsafe fn gdb_cmd_detachkill(_ks: *mut kgdb_state) { if remcom_in_buffer[0] == b'D' as i8 { let e = dbg_remove_all_break(); if e < 0 { error_packet(remcom_out_buffer.as_mut_ptr(), e); } else { strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); kgdb_connected = 0; } put_packet(remcom_out_buffer.as_ptr()); } else { dbg_remove_all_break(); kgdb_connected = 0; } }
unsafe fn gdb_cmd_reboot(_ks: *mut kgdb_state) -> i32 { if strcmp(remcom_in_buffer.as_ptr(), "R0\0" as *const str as *const i8) == 0 { strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); put_packet(remcom_out_buffer.as_ptr()); machine_emergency_restart(); kgdb_connected = 0; return 1; } 0 }
unsafe fn gdb_cmd_task(ks: *mut kgdb_state) { let mut p = remcom_in_buffer.as_mut_ptr().add(2); kgdb_hex2long(&mut p, &mut (*ks).threadid); strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); }
unsafe fn gdb_cmd_thread(ks: *mut kgdb_state) { let mut p = remcom_in_buffer.as_mut_ptr().add(1); kgdb_hex2long(&mut p, &mut (*ks).threadid); strscpy(remcom_out_buffer.as_mut_ptr(), "OK\0" as *const str as *const i8, 3); }
unsafe fn gdb_cmd_query(_ks: *mut kgdb_state) { }
unsafe fn gdb_cmd_break(_ks: *mut kgdb_state) { }
unsafe fn gdb_cmd_exception_pass(ks: *mut kgdb_state) -> i32 { if remcom_in_buffer[1] == b'0' as i8 && remcom_in_buffer[2] == b'9' as i8 { (*ks).pass_exception = 1; remcom_in_buffer[0] = b'c' as i8; } else { remcom_in_buffer[0] = b'c' as i8; } -1 }

pub unsafe fn gdb_serial_stub(ks: *mut kgdb_state) -> i32 {
    memset(remcom_out_buffer.as_mut_ptr(), 0, BUFMAX); kgdb_usethread = kgdb_info[(*ks).cpu].task; (*ks).kgdb_usethreadid = shadow_pid((*kgdb_info[(*ks).cpu].task).pid); (*ks).pass_exception = 0;
    loop { memset(remcom_out_buffer.as_mut_ptr(), 0, BUFMAX); get_packet(remcom_in_buffer.as_mut_ptr());
        let c = remcom_in_buffer[0];
        if c == b'D' as i8 || c == b'k' as i8 { dbg_remove_all_break(); kgdb_connected = 0; break; }
        if c == b'c' as i8 || c == b's' as i8 { let e = kgdb_arch_handle_exception((*ks).ex_vector, (*ks).signo, (*ks).err_code, remcom_in_buffer.as_mut_ptr(), remcom_out_buffer.as_mut_ptr(), (*ks).linux_regs); if e >= 0 { return if (*ks).pass_exception != 0 { 1 } else { 0 }; } }
        put_packet(remcom_out_buffer.as_ptr());
    }
    if (*ks).pass_exception != 0 { 1 } else { 0 }
}

pub unsafe fn gdbstub_state(ks: *mut kgdb_state, cmd: *const i8) -> i32 {
    match *cmd { b'e' as i8 => kgdb_arch_handle_exception((*ks).ex_vector, (*ks).signo, (*ks).err_code, remcom_in_buffer.as_mut_ptr(), remcom_out_buffer.as_mut_ptr(), (*ks).linux_regs), b's' as i8 | b'c' as i8 => { strscpy(remcom_in_buffer.as_mut_ptr(), cmd, BUFMAX); 0 }, b'$' as i8 => { strscpy(remcom_in_buffer.as_mut_ptr(), cmd, BUFMAX); gdbstub_use_prev_in_buf = strlen(remcom_in_buffer.as_ptr()) as i32; gdbstub_prev_in_buf_pos = 0; 0 }, _ => { dbg_io_ops.write_char(b'+' as i8); put_packet(remcom_out_buffer.as_ptr()); 0 } }
}

pub unsafe fn gdbstub_exit(status: i32) {
    if kgdb_connected == 0 { return; } kgdb_connected = 0; if dbg_io_ops.is_null() || dbg_kdb_mode { return; }
    let b = [b'W' as i8, hex_asc_hi(status as u8), hex_asc_lo(status as u8)]; let mut sum = 0u8; dbg_io_ops.write_char(b'$' as i8);
    for ch in b { sum = sum.wrapping_add(ch as u8); dbg_io_ops.write_char(ch); }
    dbg_io_ops.write_char(b'#' as i8); dbg_io_ops.write_char(hex_asc_hi(sum)); dbg_io_ops.write_char(hex_asc_lo(sum)); if let Some(flush) = dbg_io_ops.flush { flush(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
