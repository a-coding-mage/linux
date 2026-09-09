// SPDX-License-Identifier: GPL-2.0

/* Direct Rust translation of iblib.c.  Declarations supplied by ibsys.h and
 * the kernel are intentionally left as external dependencies. */

pub unsafe fn ibcac(board: *mut gpib_board, sync: i32, fallback_to_async: i32) -> i32 {
    let status = ibstatus(board);
    let mut retval;
    if (status & CIC) == 0 { return -EINVAL; }
    if (status & ATN) != 0 { return 0; }
    if sync != 0 && (status & LACS) == 0 {
        retval = -ETIMEDOUT;
    } else {
        retval = (*(*board).interface).take_control(board, sync);
    }
    if retval < 0 && fallback_to_async != 0 && sync != 0 && retval == -ETIMEDOUT {
        retval = (*(*board).interface).take_control(board, 0);
    }
    (*(*board).interface).update_status(board, 0);
    retval
}

unsafe fn check_for_command_acceptors(board: *mut gpib_board) -> i32 {
    if (*(*board).interface).skip_check_for_command_acceptors != 0 || (*(*board).interface).line_status.is_none() { return 0; }
    udelay(2);
    let lines = (*(*board).interface).line_status.unwrap()(board);
    if lines < 0 { return lines; }
    if (lines & VALID_NRFD) != 0 && (lines & VALID_NDAC) != 0 && (lines & BUS_NRFD) == 0 && (lines & BUS_NDAC) == 0 { return -ENOTCONN; }
    0
}

pub unsafe fn ibcmd(board: *mut gpib_board, mut buf: *mut u8, length: usize, bytes_written: *mut usize) -> isize {
    *bytes_written = 0;
    let status = ibstatus(board);
    if (status & CIC) == 0 { return -EINVAL as isize; }
    os_start_timer(board, (*board).usec_timeout);
    let mut ret = ibcac(board, 1, 1);
    if ret == 0 { ret = check_for_command_acceptors(board); if ret == 0 { ret = (*(*board).interface).command(board, buf, length, bytes_written) as i32; } }
    os_remove_timer(board);
    if io_timed_out(board) != 0 { ret = -ETIMEDOUT; }
    ret as isize
}

pub unsafe fn ibgts(board: *mut gpib_board) -> i32 {
    let status = ibstatus(board); if (status & CIC) == 0 { return -EINVAL; }
    let retval = (*(*board).interface).go_to_standby(board);
    (*(*board).interface).update_status(board, 0); retval
}

unsafe fn autospoll_wait_should_wake_up(board: *mut gpib_board) -> i32 {
    mutex_lock(&mut (*board).big_gpib_mutex);
    let retval = ((*board).master != 0 && (*board).autospollers > 0 && atomic_read(&(*board).stuck_srq) == 0 && test_and_clear_bit(SRQI_NUM, &mut (*board).status)) as i32;
    mutex_unlock(&mut (*board).big_gpib_mutex); retval
}

unsafe fn autospoll_thread(board: *mut core::ffi::c_void) -> i32 {
    let board = board as *mut gpib_board; let mut retval = 0;
    dev_dbg((*board).gpib_dev, "entering autospoll thread\n");
    loop {
        wait_event_interruptible(&mut (*board).wait, kthread_should_stop() != 0 || autospoll_wait_should_wake_up(board) != 0);
        dev_dbg((*board).gpib_dev, "autospoll wait satisfied\n"); if kthread_should_stop() != 0 { break; }
        mutex_lock(&mut (*board).big_gpib_mutex);
        if (*board).autospollers <= 0 || (*board).master == 0 { mutex_unlock(&mut (*board).big_gpib_mutex); continue; }
        mutex_unlock(&mut (*board).big_gpib_mutex);
        if try_module_get((*board).provider_module) != 0 { retval = autopoll_all_devices(board); module_put((*board).provider_module); } else { dev_err((*board).gpib_dev, "try_module_get() failed!\n"); }
        if retval <= 0 { dev_err((*board).gpib_dev, "stuck SRQ\n"); }
    } retval
}

pub unsafe fn ibonline(board: *mut gpib_board) -> i32 {
    if (*board).online != 0 { return -EBUSY; } if (*board).interface.is_null() { return -ENODEV; }
    let mut retval = gpib_allocate_board(board); if retval < 0 { return retval; }
    (*board).dev = core::ptr::null_mut(); (*board).local_ppoll_mode = 0;
    retval = (*(*board).interface).attach(board, &mut (*board).config); if retval < 0 { (*(*board).interface).detach(board); return retval; }
    // CONFIG_NIOS2 condition from the C translation: omit the autospoll task on NIOS2.
    #[cfg(not(CONFIG_NIOS2))]
    { (*board).autospoll_task = kthread_run(autospoll_thread, board as *mut _, "gpib%d_autospoll_kthread", (*board).minor); if IS_ERR((*board).autospoll_task) { dev_err((*board).gpib_dev, "failed to create autospoll thread\n"); (*(*board).interface).detach(board); return PTR_ERR((*board).autospoll_task); } }
    (*board).online = 1; dev_dbg((*board).gpib_dev, "board online\n"); 0
}

pub unsafe fn iboffline(board: *mut gpib_board) -> i32 {
    if (*board).online == 0 { return 0; } if (*board).interface.is_null() { return -ENODEV; }
    if !(*board).autospoll_task.is_null() && !IS_ERR((*board).autospoll_task) { let retval = kthread_stop((*board).autospoll_task); if retval != 0 { dev_err((*board).gpib_dev, "kthread_stop returned %i\n", retval); } (*board).autospoll_task = core::ptr::null_mut(); }
    (*(*board).interface).detach(board); gpib_deallocate_board(board); (*board).online = 0; dev_dbg((*board).gpib_dev, "board offline\n"); 0
}

pub unsafe fn iblines(board: *const gpib_board, lines: *mut i16) -> i32 {
    *lines = 0; if (*(*board).interface).line_status.is_none() { return 0; }
    let retval = (*(*board).interface).line_status.unwrap()(board as *mut gpib_board); if retval < 0 { return retval; } *lines = retval as i16; 0
}

pub unsafe fn ibrd(board: *mut gpib_board, mut buf: *mut u8, length: usize, end_flag: *mut i32, nbytes: *mut usize) -> isize {
    *nbytes = 0; *end_flag = 0; if length == 0 { return 0; }
    if (*board).master != 0 { let retval = ibgts(board); if retval < 0 { return retval as isize; } }
    os_start_timer(board, (*board).usec_timeout); let mut ret: isize; let mut bytes_read = 0usize;
    loop { ret = (*(*board).interface).read(board, buf, length - *nbytes, end_flag, &mut bytes_read); if ret < 0 { break; } buf = buf.add(bytes_read); *nbytes += bytes_read; if need_resched() != 0 { schedule(); } if !(ret == 0 && *nbytes > 0 && *nbytes < length && *end_flag == 0) { break; } }
    os_remove_timer(board); ret
}

pub unsafe fn ibrpp(board: *mut gpib_board, result: *mut u8) -> i32 {
    os_start_timer(board, (*board).usec_timeout); let retval = ibcac(board, 1, 1); if retval != 0 { return -1; }
    let retval = (*(*board).interface).parallel_poll(board, result); os_remove_timer(board); retval
}

pub unsafe fn ibppc(board: *mut gpib_board, mut configuration: u8) -> i32 { configuration &= 0x1f; (*(*board).interface).parallel_poll_configure(board, configuration); (*board).parallel_poll_configuration = configuration; 0 }

pub unsafe fn ibrsv2(board: *mut gpib_board, status_byte: u8, new_reason_for_service: i32) -> i32 {
    let board_status = ibstatus(board); let mss = (status_byte as u32) & request_service_bit;
    if (board_status & CIC) != 0 || (mss == 0 && new_reason_for_service != 0) { return -EINVAL; }
    if let Some(f) = (*(*board).interface).serial_poll_response2 { f(board, status_byte, new_reason_for_service); }
    else if let Some(f) = (*(*board).interface).serial_poll_response { if mss == 0 || (mss != 0 && new_reason_for_service != 0) { f(board, status_byte); } else { return -EOPNOTSUPP; } }
    else { return -EOPNOTSUPP; } 0
}

pub unsafe fn ibsic(board: *mut gpib_board, mut usec_duration: u32) -> i32 {
    if (*board).master == 0 { return -EINVAL; } if usec_duration < 100 { usec_duration = 100; } if usec_duration > 1000 { usec_duration = 1000; }
    dev_dbg((*board).gpib_dev, "sending interface clear, delay = %ius\n", usec_duration); (*(*board).interface).interface_clear(board, 1); udelay(usec_duration); (*(*board).interface).interface_clear(board, 0); 0
}

pub unsafe fn ibrsc(board: *mut gpib_board, request_control: i32) -> i32 { if (*(*board).interface).request_system_control.is_none() { return -EPERM; } let retval = (*(*board).interface).request_system_control.unwrap()(board, request_control); if retval != 0 { return retval; } (*board).master = (request_control != 0) as i32; 0 }
pub unsafe fn ibsre(board: *mut gpib_board, enable: i32) -> i32 { if (*board).master == 0 { return -EINVAL; } (*(*board).interface).remote_enable(board, enable); if enable == 0 { usleep_range(100, 150); } 0 }
pub unsafe fn ibpad(board: *mut gpib_board, addr: u32) -> i32 { if addr > MAX_GPIB_PRIMARY_ADDRESS { return -EINVAL; } (*board).pad = addr as i32; if (*board).online != 0 { (*(*board).interface).primary_address(board, (*board).pad); } dev_dbg((*board).gpib_dev, "set primary addr to %i\n", (*board).pad); 0 }
pub unsafe fn ibsad(board: *mut gpib_board, addr: i32) -> i32 { if addr > MAX_GPIB_SECONDARY_ADDRESS { return -EINVAL; } (*board).sad = addr; if (*board).online != 0 { if (*board).sad >= 0 { (*(*board).interface).secondary_address(board, (*board).sad, 1); } else { (*(*board).interface).secondary_address(board, 0, 0); } } dev_dbg((*board).gpib_dev, "set secondary addr to %i\n", (*board).sad); 0 }

pub unsafe fn ibeos(board: *mut gpib_board, eos: i32, eosflags: i32) -> i32 { if (eosflags & !EOS_MASK) != 0 { return -EINVAL; } if (eosflags & REOS) != 0 { (*(*board).interface).enable_eos(board, eos, eosflags & BIN) } else { (*(*board).interface).disable_eos(board); 0 } }
pub unsafe fn ibstatus(board: *mut gpib_board) -> i32 { general_ibstatus(board, core::ptr::null(), 0, 0, core::ptr::null_mut()) }

pub unsafe fn general_ibstatus(board: *mut gpib_board, device: *const gpib_status_queue, clear_mask: i32, set_mask: i32, desc: *mut gpib_descriptor) -> i32 {
    let mut status = 0; let mut line_status = 0i16;
    if !(*board).private_data.is_null() { status = (*(*board).interface).update_status(board, clear_mask); status &= !TIMO; if iblines(board, &mut line_status) == 0 && (line_status as i32 & VALID_SRQ) != 0 { if (line_status as i32 & BUS_SRQ) != 0 { status |= SRQI; } else { status &= !SRQI; } } }
    if !device.is_null() && num_status_bytes(device) != 0 { status |= RQS; }
    if !desc.is_null() { if (set_mask & CMPL) != 0 { atomic_set(&mut (*desc).io_in_progress, 0); } else if (clear_mask & CMPL) != 0 { atomic_set(&mut (*desc).io_in_progress, 1); } if atomic_read(&(*desc).io_in_progress) != 0 { status &= !CMPL; } else { status |= CMPL; } }
    if num_gpib_events(&(*board).event_queue) != 0 { status |= EVENT; } else { status &= !EVENT; } status
}

#[repr(C)]
pub struct wait_info { pub board: *mut gpib_board, pub timer: timer_list, pub timed_out: i32, pub usec_timeout: u64 }
unsafe fn wait_timeout(t: *mut timer_list) { let winfo = timer_container_of_wait_info(t); (*winfo).timed_out = 1; wake_up_interruptible(&mut (*(*winfo).board).wait); }
unsafe fn init_wait_info(winfo: *mut wait_info) { (*winfo).board = core::ptr::null_mut(); (*winfo).timed_out = 0; timer_setup_on_stack(&mut (*winfo).timer, wait_timeout, 0); }
unsafe fn wait_satisfied(winfo: *mut wait_info, status_queue: *mut gpib_status_queue, wait_mask: i32, status: *mut i32, desc: *mut gpib_descriptor) -> i32 {
    let board = (*winfo).board; if mutex_lock_interruptible(&mut (*board).big_gpib_mutex) != 0 { return -ERESTARTSYS; }
    let mut temp_status = general_ibstatus(board, status_queue, 0, 0, desc); mutex_unlock(&mut (*board).big_gpib_mutex);
    if (*winfo).timed_out != 0 { temp_status |= TIMO; } else { temp_status &= !TIMO; }
    if (wait_mask & temp_status) != 0 { *status = temp_status; return 1; } 0
}
unsafe fn start_wait_timer(winfo: *mut wait_info) { (*winfo).timed_out = 0; if (*winfo).usec_timeout > 0 { mod_timer(&mut (*winfo).timer, jiffies + usec_to_jiffies((*winfo).usec_timeout)); } }
unsafe fn remove_wait_timer(winfo: *mut wait_info) { timer_delete_sync(&mut (*winfo).timer); timer_destroy_on_stack(&mut (*winfo).timer); }

pub unsafe fn ibwait(board: *mut gpib_board, wait_mask: i32, clear_mask: i32, set_mask: i32, status: *mut i32, usec_timeout: u64, desc: *mut gpib_descriptor) -> i32 {
    let status_queue = if (*desc).is_board != 0 { core::ptr::null_mut() } else { get_gpib_status_queue(board, (*desc).pad, (*desc).sad) };
    if wait_mask == 0 { *status = general_ibstatus(board, status_queue, clear_mask, set_mask, desc); return 0; }
    mutex_unlock(&mut (*board).big_gpib_mutex); let mut winfo = core::mem::MaybeUninit::<wait_info>::uninit(); let w = winfo.as_mut_ptr(); init_wait_info(w); (*w).board = board; (*w).usec_timeout = usec_timeout; start_wait_timer(w);
    let mut retval = 0; if wait_event_interruptible(&mut (*board).wait, wait_satisfied(w, status_queue, wait_mask, status, desc)) != 0 { dev_dbg((*board).gpib_dev, "wait interrupted\n"); retval = -ERESTARTSYS; } remove_wait_timer(w); if retval != 0 { return retval; }
    if mutex_lock_interruptible(&mut (*board).big_gpib_mutex) != 0 { return -ERESTARTSYS; }
    if (*status & clear_mask) != 0 || set_mask != 0 { general_ibstatus(board, status_queue, *status & clear_mask, set_mask, core::ptr::null_mut()); } 0
}

pub unsafe fn ibwrt(board: *mut gpib_board, buf: *mut u8, cnt: usize, send_eoi: i32, bytes_written: *mut usize) -> i32 {
    if cnt == 0 { return 0; } if (*board).master != 0 { let retval = ibgts(board); if retval < 0 { return retval; } }
    os_start_timer(board, (*board).usec_timeout); let mut ret = (*(*board).interface).write(board, buf, cnt, send_eoi, bytes_written) as i32; if io_timed_out(board) != 0 { ret = -ETIMEDOUT; } os_remove_timer(board); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
