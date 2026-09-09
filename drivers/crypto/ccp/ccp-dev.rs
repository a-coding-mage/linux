// SPDX-License-Identifier: GPL-2.0-only
/* AMD Cryptographic Coprocessor (CCP) driver */

// Linux kernel headers and "ccp-dev.h" are external dependencies.

const MAX_CCPS: u32 = 32;
static mut NQUEUES: u32 = 0;
static mut DEV_COUNT: atomic_t = ATOMIC_INIT(0);
static mut MAX_DEVS: u32 = MAX_CCPS;

#[repr(C)]
struct CcpTaskletData {
    completion: completion,
    cmd: *mut ccp_cmd,
}

const CCP_MAX_ERROR_CODE: usize = 64;
static mut CCP_ERROR_CODES: [&'static [u8]; 44] = [
    b"", b"ILLEGAL_ENGINE", b"ILLEGAL_KEY_ID", b"ILLEGAL_FUNCTION_TYPE",
    b"ILLEGAL_FUNCTION_MODE", b"ILLEGAL_FUNCTION_ENCRYPT", b"ILLEGAL_FUNCTION_SIZE",
    b"Zlib_MISSING_INIT_EOM", b"ILLEGAL_FUNCTION_RSVD", b"ILLEGAL_BUFFER_LENGTH",
    b"VLSB_FAULT", b"ILLEGAL_MEM_ADDR", b"ILLEGAL_MEM_SEL", b"ILLEGAL_CONTEXT_ID",
    b"ILLEGAL_KEY_ADDR", b"0xF Reserved", b"Zlib_ILLEGAL_MULTI_QUEUE",
    b"Zlib_ILLEGAL_JOBID_CHANGE", b"CMD_TIMEOUT", b"IDMA0_AXI_SLVERR", b"IDMA0_AXI_DECERR",
    b"0x15 Reserved", b"IDMA1_AXI_SLAVE_FAULT", b"IDMA1_AIXI_DECERR", b"0x18 Reserved",
    b"ZLIBVHB_AXI_SLVERR", b"ZLIBVHB_AXI_DECERR", b"0x1B Reserved", b"ZLIB_UNEXPECTED_EOM",
    b"ZLIB_EXTRA_DATA", b"ZLIB_BTYPE", b"ZLIB_UNDEFINED_SYMBOL", b"ZLIB_UNDEFINED_DISTANCE_S",
    b"ZLIB_CODE_LENGTH_SYMBOL", b"ZLIB _VHB_ILLEGAL_FETCH", b"ZLIB_UNCOMPRESSED_LEN",
    b"ZLIB_LIMIT_REACHED", b"ZLIB_CHECKSUM_MISMATCH0", b"ODMA0_AXI_SLVERR", b"ODMA0_AXI_DECERR",
    b"0x28 Reserved", b"ODMA1_AXI_SLVERR", b"ODMA1_AXI_DECERR",
];

static mut CCP_UNIT_LOCK: rwlock_t = DEFINE_RWLOCK();
static mut CCP_UNITS: list_head = LIST_HEAD_INIT();
static mut CCP_RR_LOCK: spinlock_t = DEFINE_SPINLOCK();
static mut CCP_RR: *mut ccp_device = core::ptr::null_mut();

pub unsafe fn ccp_log_error(d: *mut ccp_device, e: u32) {
    if WARN_ON(e >= CCP_MAX_ERROR_CODE as u32) { return; }
    if (e as usize) < CCP_ERROR_CODES.len() {
        dev_err((*d).dev, "CCP error %d: %s\n", e, CCP_ERROR_CODES[e as usize].as_ptr());
    } else { dev_err((*d).dev, "CCP error %d: Unknown Error\n", e); }
}

pub unsafe fn ccp_add_device(ccp: *mut ccp_device) {
    let mut flags: c_ulong = 0;
    write_lock_irqsave(&mut CCP_UNIT_LOCK, &mut flags);
    list_add_tail(&mut (*ccp).entry, &mut CCP_UNITS);
    if CCP_RR.is_null() { CCP_RR = ccp; }
    write_unlock_irqrestore(&mut CCP_UNIT_LOCK, flags);
}

pub unsafe fn ccp_del_device(ccp: *mut ccp_device) {
    let mut flags: c_ulong = 0;
    write_lock_irqsave(&mut CCP_UNIT_LOCK, &mut flags);
    if CCP_RR == ccp {
        if list_is_last(&(*CCP_RR).entry, &CCP_UNITS) {
            CCP_RR = list_first_entry(&CCP_UNITS, ccp_device, entry);
        } else { CCP_RR = list_next_entry(CCP_RR, entry); }
    }
    list_del(&mut (*ccp).entry);
    if list_empty(&CCP_UNITS) { CCP_RR = core::ptr::null_mut(); }
    write_unlock_irqrestore(&mut CCP_UNIT_LOCK, flags);
}

pub unsafe fn ccp_register_rng(ccp: *mut ccp_device) -> c_int {
    let mut ret = 0;
    dev_dbg((*ccp).dev, "Registering RNG...\n");
    (*ccp).hwrng.name = (*ccp).rngname.as_mut_ptr();
    (*ccp).hwrng.read = Some(ccp_trng_read);
    ret = hwrng_register(&mut (*ccp).hwrng);
    if ret != 0 { dev_err((*ccp).dev, "error registering hwrng (%d)\n", ret); }
    ret
}

pub unsafe fn ccp_unregister_rng(ccp: *mut ccp_device) {
    if !(*ccp).hwrng.name.is_null() { hwrng_unregister(&mut (*ccp).hwrng); }
}

unsafe fn ccp_get_device() -> *mut ccp_device {
    let mut flags: c_ulong = 0;
    let mut dp = core::ptr::null_mut();
    read_lock_irqsave(&mut CCP_UNIT_LOCK, &mut flags);
    if !list_empty(&CCP_UNITS) {
        spin_lock(&mut CCP_RR_LOCK);
        dp = CCP_RR;
        if list_is_last(&(*CCP_RR).entry, &CCP_UNITS) { CCP_RR = list_first_entry(&CCP_UNITS, ccp_device, entry); }
        else { CCP_RR = list_next_entry(CCP_RR, entry); }
        spin_unlock(&mut CCP_RR_LOCK);
    }
    read_unlock_irqrestore(&mut CCP_UNIT_LOCK, flags);
    dp
}

pub unsafe fn ccp_present() -> c_int {
    let mut flags: c_ulong = 0;
    read_lock_irqsave(&mut CCP_UNIT_LOCK, &mut flags);
    let ret = if list_empty(&CCP_UNITS) { -ENODEV } else { 0 };
    read_unlock_irqrestore(&mut CCP_UNIT_LOCK, flags);
    ret
}

pub unsafe fn ccp_version() -> u32 {
    let mut flags: c_ulong = 0;
    let mut ret = 0;
    read_lock_irqsave(&mut CCP_UNIT_LOCK, &mut flags);
    if !list_empty(&CCP_UNITS) { let dp = list_first_entry(&CCP_UNITS, ccp_device, entry); ret = (*(*dp).vdata).version; }
    read_unlock_irqrestore(&mut CCP_UNIT_LOCK, flags);
    ret
}

pub unsafe fn ccp_enqueue_cmd(cmd: *mut ccp_cmd) -> c_int {
    let ccp = if !(*cmd).ccp.is_null() { (*cmd).ccp } else { ccp_get_device() };
    if ccp.is_null() { return -ENODEV; }
    if (*cmd).callback.is_none() { return -EINVAL; }
    (*cmd).ccp = ccp;
    let mut flags: c_ulong = 0;
    let mut i = (*ccp).cmd_q_count;
    spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags);
    let ret;
    if (*ccp).cmd_count >= MAX_CMD_QLEN {
        if ((*cmd).flags & CCP_CMD_MAY_BACKLOG) != 0 { ret = -EBUSY; list_add_tail(&mut (*cmd).entry, &mut (*ccp).backlog); }
        else { ret = -ENOSPC; }
    } else {
        ret = -EINPROGRESS; (*ccp).cmd_count += 1; list_add_tail(&mut (*cmd).entry, &mut (*ccp).cmd);
        if (*ccp).suspending == 0 { for j in 0..(*ccp).cmd_q_count { i = j; if (*ccp).cmd_q[j as usize].active == 0 { break; } } }
    }
    spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags);
    if i < (*ccp).cmd_q_count { wake_up_process((*ccp).cmd_q[i as usize].kthread); }
    ret
}

unsafe fn ccp_do_cmd_backlog(work: *mut work_struct) {
    let cmd = container_of!(work, ccp_cmd, work); let ccp = (*cmd).ccp;
    ((*cmd).callback.unwrap())((*cmd).data, -EINPROGRESS);
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags);
    (*ccp).cmd_count += 1; list_add_tail(&mut (*cmd).entry, &mut (*ccp).cmd);
    let mut i = 0; for j in 0..(*ccp).cmd_q_count { i = j; if (*ccp).cmd_q[j as usize].active == 0 { break; } }
    spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags);
    if i < (*ccp).cmd_q_count { wake_up_process((*ccp).cmd_q[i as usize].kthread); }
}

unsafe fn ccp_dequeue_cmd(cmd_q: *mut ccp_cmd_queue) -> *mut ccp_cmd {
    let ccp = (*cmd_q).ccp; let mut cmd = core::ptr::null_mut(); let mut backlog = core::ptr::null_mut(); let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags); (*cmd_q).active = 0;
    if (*ccp).suspending != 0 { (*cmd_q).suspended = 1; spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags); wake_up_interruptible(&mut (*ccp).suspend_queue); return core::ptr::null_mut(); }
    if (*ccp).cmd_count != 0 { (*cmd_q).active = 1; cmd = list_first_entry(&(*ccp).cmd, ccp_cmd, entry); list_del(&mut (*cmd).entry); (*ccp).cmd_count -= 1; }
    if !list_empty(&(*ccp).backlog) { backlog = list_first_entry(&(*ccp).backlog, ccp_cmd, entry); list_del(&mut (*backlog).entry); }
    spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags);
    if !backlog.is_null() { INIT_WORK(&mut (*backlog).work, ccp_do_cmd_backlog); schedule_work(&mut (*backlog).work); }
    cmd
}

unsafe fn ccp_do_cmd_complete(data: c_ulong) { let tdata = data as *mut CcpTaskletData; let cmd = (*tdata).cmd; ((*cmd).callback.unwrap())((*cmd).data, (*cmd).ret); complete(&mut (*tdata).completion); }

pub unsafe fn ccp_cmd_queue_thread(data: *mut c_void) -> c_int {
    let cmd_q = data as *mut ccp_cmd_queue; let mut tdata: CcpTaskletData = core::mem::zeroed(); let mut tasklet: tasklet_struct = core::mem::zeroed();
    tasklet_init(&mut tasklet, ccp_do_cmd_complete, &mut tdata as *mut _ as c_ulong); set_current_state(TASK_INTERRUPTIBLE);
    while !kthread_should_stop() { schedule(); set_current_state(TASK_INTERRUPTIBLE); let cmd = ccp_dequeue_cmd(cmd_q); if cmd.is_null() { continue; } __set_current_state(TASK_RUNNING); (*cmd).ret = ccp_run_cmd(cmd_q, cmd); tdata.cmd = cmd; init_completion(&mut tdata.completion); tasklet_schedule(&mut tasklet); wait_for_completion(&mut tdata.completion); }
    __set_current_state(TASK_RUNNING); 0
}

pub unsafe fn ccp_alloc_struct(sp: *mut sp_device) -> *mut ccp_device {
    let dev = (*sp).dev; let ccp = devm_kzalloc(dev, core::mem::size_of::<ccp_device>(), GFP_KERNEL) as *mut ccp_device; if ccp.is_null() { return core::ptr::null_mut(); }
    (*ccp).dev = dev; (*ccp).sp = sp; (*ccp).axcache = (*sp).axcache; INIT_LIST_HEAD(&mut (*ccp).cmd); INIT_LIST_HEAD(&mut (*ccp).backlog); spin_lock_init(&mut (*ccp).cmd_lock); mutex_init(&mut (*ccp).req_mutex); mutex_init(&mut (*ccp).sb_mutex); (*ccp).sb_count = KSB_COUNT; (*ccp).sb_start = 0; init_waitqueue_head(&mut (*ccp).sb_queue); init_waitqueue_head(&mut (*ccp).suspend_queue); snprintf((*ccp).name.as_mut_ptr(), MAX_CCP_NAME_LEN, b"ccp-%u\0".as_ptr(), (*sp).ord); snprintf((*ccp).rngname.as_mut_ptr(), MAX_CCP_NAME_LEN, b"ccp-%u-rng\0".as_ptr(), (*sp).ord); ccp
}

pub unsafe fn ccp_trng_read(rng: *mut hwrng, data: *mut c_void, max: usize, _wait: bool) -> c_int {
    let ccp = container_of!(rng, ccp_device, hwrng); let mut trng_value = ioread32((*ccp).io_regs.add(TRNG_OUT_REG as usize)); let len = core::cmp::min(core::mem::size_of::<u32>(), max);
    if trng_value == 0 { (*ccp).hwrng_retries += 1; if (*ccp).hwrng_retries > TRNG_RETRIES { return -EIO; } return 0; }
    (*ccp).hwrng_retries = 0; core::ptr::copy_nonoverlapping(&trng_value as *const u32 as *const u8, data as *mut u8, len); len as c_int
}

pub unsafe fn ccp_queues_suspended(ccp: *mut ccp_device) -> bool { let mut flags: c_ulong = 0; let mut suspended = 0; spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags); for i in 0..(*ccp).cmd_q_count { if (*ccp).cmd_q[i as usize].suspended != 0 { suspended += 1; } } spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags); (*ccp).cmd_q_count == suspended }

pub unsafe fn ccp_dev_suspend(sp: *mut sp_device) { let ccp = (*sp).ccp_data; if ccp.is_null() { return; } let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags); (*ccp).suspending = 1; for i in 0..(*ccp).cmd_q_count { wake_up_process((*ccp).cmd_q[i as usize].kthread); } spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags); while !ccp_queues_suspended(ccp) { wait_event_interruptible(&mut (*ccp).suspend_queue, ccp_queues_suspended(ccp)); } }

pub unsafe fn ccp_dev_resume(sp: *mut sp_device) { let ccp = (*sp).ccp_data; if ccp.is_null() { return; } let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*ccp).cmd_lock, &mut flags); (*ccp).suspending = 0; for i in 0..(*ccp).cmd_q_count { (*ccp).cmd_q[i as usize].suspended = 0; wake_up_process((*ccp).cmd_q[i as usize].kthread); } spin_unlock_irqrestore(&mut (*ccp).cmd_lock, flags); }

pub unsafe fn ccp_dev_init(sp: *mut sp_device) -> c_int {
    let dev = (*sp).dev; if atomic_inc_return(&mut DEV_COUNT) > MAX_DEVS as i32 { return 0; }
    let mut ret = -ENOMEM; let ccp = ccp_alloc_struct(sp); if ccp.is_null() { dev_notice(dev, "ccp initialization failed\n"); return ret; } (*sp).ccp_data = ccp;
    (*ccp).max_q_count = if NQUEUES == 0 || NQUEUES > MAX_HW_QUEUES { MAX_HW_QUEUES } else { NQUEUES };
    (*ccp).vdata = (*sp).dev_vdata.ccp_vdata as *mut ccp_vdata; if (*ccp).vdata.is_null() || (*(*ccp).vdata).version == 0 { ret = -ENODEV; dev_err(dev, "missing driver data\n"); (*sp).ccp_data = core::ptr::null_mut(); dev_notice(dev, "ccp initialization failed\n"); return ret; }
    (*ccp).use_tasklet = (*sp).use_tasklet; (*ccp).io_regs = (*sp).io_map.add((*(*ccp).vdata).offset as usize); if let Some(setup) = (*(*ccp).vdata).setup { setup(ccp); }
    ret = ((*(*ccp).vdata).perform).init(ccp); if ret != 0 { if ret > 0 { (*sp).ccp_data = core::ptr::null_mut(); return ret; } dev_notice(dev, "ccp initialization failed\n"); (*sp).ccp_data = core::ptr::null_mut(); return ret; }
    dev_notice(dev, "ccp enabled\n"); 0
}

pub unsafe fn ccp_dev_destroy(sp: *mut sp_device) { let ccp = (*sp).ccp_data; if !ccp.is_null() { ((*(*ccp).vdata).perform).destroy(ccp); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
