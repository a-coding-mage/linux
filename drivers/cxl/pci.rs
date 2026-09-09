// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2020 Intel Corporation. All rights reserved. */
// Kernel headers and symbols are supplied by the surrounding Rust kernel bindings.

const CXL_MAILBOX_TIMEOUT_MS: u64 = 2 * HZ;
static mut MBOX_READY_TIMEOUT: u16 = 60;

#[repr(C)]
struct CxlDevId { cxlds: *mut cxl_dev_state }

unsafe fn cxl_doorbell_busy(cxlds: *mut cxl_dev_state) -> bool {
    (readl((*cxlds).regs.mbox.add(CXLDEV_MBOX_CTRL_OFFSET)) & CXLDEV_MBOX_CTRL_DOORBELL) != 0
}

unsafe fn cxl_pci_mbox_wait_for_doorbell(cxlds: *mut cxl_dev_state) -> i32 {
    let start = jiffies();
    let mut end = start;
    while cxl_doorbell_busy(cxlds) {
        end = jiffies();
        if time_after(end, start + CXL_MAILBOX_TIMEOUT_MS) {
            if !cxl_doorbell_busy(cxlds) { break; }
            return -ETIMEDOUT;
        }
        cpu_relax();
    }
    dev_dbg((*cxlds).dev, "Doorbell wait took %dms", jiffies_to_msecs(end) - jiffies_to_msecs(start));
    0
}

unsafe fn cxl_request_irq(cxlds: *mut cxl_dev_state, irq: i32, thread_fn: irq_handler_t) -> i32 {
    let dev = (*cxlds).dev;
    let dev_id = devm_kzalloc(dev, core::mem::size_of::<CxlDevId>(), GFP_KERNEL) as *mut CxlDevId;
    if dev_id.is_null() { return -ENOMEM; }
    (*dev_id).cxlds = cxlds;
    devm_request_threaded_irq(dev, irq, None, thread_fn, IRQF_SHARED | IRQF_ONESHOT, core::ptr::null(), dev_id as *mut _)
}

unsafe fn cxl_mbox_background_complete(cxlds: *mut cxl_dev_state) -> bool {
    let reg = readq((*cxlds).regs.mbox.add(CXLDEV_MBOX_BG_CMD_STATUS_OFFSET));
    FIELD_GET(CXLDEV_MBOX_BG_CMD_COMMAND_PCT_MASK, reg) == 100
}

unsafe extern "C" fn cxl_pci_mbox_irq(_irq: i32, id: *mut core::ffi::c_void) -> irqreturn_t {
    let dev_id = id as *mut CxlDevId;
    let cxlds = (*dev_id).cxlds;
    let cxl_mbox = &mut (*cxlds).cxl_mbox;
    let mds = to_cxl_memdev_state(cxlds);
    if !cxl_mbox_background_complete(cxlds) { return IRQ_NONE; }
    let reg = readq((*cxlds).regs.mbox.add(CXLDEV_MBOX_BG_CMD_STATUS_OFFSET));
    let opcode = FIELD_GET(CXLDEV_MBOX_BG_CMD_COMMAND_OPCODE_MASK, reg);
    if opcode == CXL_MBOX_OP_SANITIZE {
        mutex_lock(&mut cxl_mbox.mbox_mutex);
        if !(*mds).security.sanitize_node.is_null() { mod_delayed_work(system_percpu_wq, &mut (*mds).security.poll_dwork, 0); }
        mutex_unlock(&mut cxl_mbox.mbox_mutex);
    } else { rcuwait_wake_up(&mut cxl_mbox.mbox_wait); }
    IRQ_HANDLED
}

unsafe extern "C" fn cxl_mbox_sanitize_work(work: *mut work_struct) {
    let mds = container_of!(work, cxl_memdev_state, security.poll_dwork.work);
    let cxlds = &mut (*mds).cxlds;
    let cxl_mbox = &mut cxlds.cxl_mbox;
    mutex_lock(&mut cxl_mbox.mbox_mutex);
    if cxl_mbox_background_complete(cxlds) {
        (*mds).security.poll_tmo_secs = 0;
        if !(*mds).security.sanitize_node.is_null() { sysfs_notify_dirent((*mds).security.sanitize_node); }
        (*mds).security.sanitize_active = false;
        dev_dbg(cxlds.dev, "Sanitization operation ended\n");
    } else {
        let timeout = (*mds).security.poll_tmo_secs + 10;
        (*mds).security.poll_tmo_secs = core::cmp::min(15 * 60, timeout);
        schedule_delayed_work(&mut (*mds).security.poll_dwork, timeout * HZ);
    }
    mutex_unlock(&mut cxl_mbox.mbox_mutex);
}

unsafe fn __cxl_pci_mbox_send_cmd(cxl_mbox: *mut cxl_mailbox, cmd: *mut cxl_mbox_cmd) -> i32 {
    let cxlds = mbox_to_cxlds(cxl_mbox);
    let mds = to_cxl_memdev_state(cxlds);
    let payload = (*cxlds).regs.mbox.add(CXLDEV_MBOX_PAYLOAD_OFFSET);
    if cxl_doorbell_busy(cxlds) { return -EBUSY; }
    if (*mds).security.poll_tmo_secs > 0 && (*cmd).opcode != CXL_MBOX_OP_GET_HEALTH_INFO { return -EBUSY; }
    let mut cmd_reg = FIELD_PREP(CXLDEV_MBOX_CMD_COMMAND_OPCODE_MASK, (*cmd).opcode as u64);
    if (*cmd).size_in != 0 {
        if (*cmd).payload_in.is_null() { return -EINVAL; }
        cmd_reg |= FIELD_PREP(CXLDEV_MBOX_CMD_PAYLOAD_LENGTH_MASK, (*cmd).size_in as u64);
        memcpy_toio(payload, (*cmd).payload_in, (*cmd).size_in);
    }
    writeq(cmd_reg, (*cxlds).regs.mbox.add(CXLDEV_MBOX_CMD_OFFSET));
    writel(CXLDEV_MBOX_CTRL_DOORBELL, (*cxlds).regs.mbox.add(CXLDEV_MBOX_CTRL_OFFSET));
    let rc = cxl_pci_mbox_wait_for_doorbell(cxlds);
    if rc == -ETIMEDOUT { return rc; }
    let status = readq((*cxlds).regs.mbox.add(CXLDEV_MBOX_STATUS_OFFSET));
    (*cmd).return_code = FIELD_GET(CXLDEV_MBOX_STATUS_RET_CODE_MASK, status);
    if (*cmd).return_code == CXL_MBOX_CMD_RC_BACKGROUND {
        if (*cmd).opcode == CXL_MBOX_OP_SANITIZE {
            if (*mds).security.sanitize_active { return -EBUSY; }
            (*mds).security.poll_tmo_secs = 1; (*mds).security.sanitize_active = true;
            schedule_delayed_work(&mut (*mds).security.poll_dwork, HZ); return 0;
        }
        let timeout = (*cmd).poll_interval_ms;
        for _ in 0..(*cmd).poll_count { if rcuwait_wait_event_timeout(&mut (*cxl_mbox).mbox_wait, cxl_mbox_background_complete(cxlds), TASK_UNINTERRUPTIBLE, msecs_to_jiffies(timeout)) > 0 { break; } }
        if !cxl_mbox_background_complete(cxlds) { return -ETIMEDOUT; }
        let bg = readq((*cxlds).regs.mbox.add(CXLDEV_MBOX_BG_CMD_STATUS_OFFSET));
        (*cmd).return_code = FIELD_GET(CXLDEV_MBOX_BG_CMD_COMMAND_RC_MASK, bg);
    }
    if (*cmd).return_code != CXL_MBOX_CMD_RC_SUCCESS { return 0; }
    let out_len = FIELD_GET(CXLDEV_MBOX_CMD_PAYLOAD_LENGTH_MASK, readq((*cxlds).regs.mbox.add(CXLDEV_MBOX_CMD_OFFSET))) as usize;
    if out_len != 0 && !(*cmd).payload_out.is_null() {
        let n = core::cmp::min((*cmd).size_out, core::cmp::min((*cxl_mbox).payload_size, out_len));
        memcpy_fromio((*cmd).payload_out, payload, n); (*cmd).size_out = n;
    } else { (*cmd).size_out = 0; }
    0
}

unsafe fn cxl_pci_mbox_send(mbox: *mut cxl_mailbox, cmd: *mut cxl_mbox_cmd) -> i32 {
    mutex_lock(&mut (*mbox).mbox_mutex); let rc = __cxl_pci_mbox_send_cmd(mbox, cmd); mutex_unlock(&mut (*mbox).mbox_mutex); rc
}

unsafe fn free_event_buf(buf: *mut core::ffi::c_void) { kvfree(buf); }

unsafe fn cxl_mem_alloc_event_buf(mds: *mut cxl_memdev_state) -> i32 {
    let size = (*mds).cxlds.cxl_mbox.payload_size;
    let buf = kvmalloc(size, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    (*mds).event.buf = buf as *mut _;
    devm_add_action_or_reset((*mds).cxlds.dev, free_event_buf, buf)
}

// The remaining PCI registration, event policy, probe, reset/error handlers, CPER work,
// and module init/exit retain their C interfaces and are declared through the kernel ABI.
extern "C" {
    fn cxl_pci_setup_mailbox(mds: *mut cxl_memdev_state, irq_avail: bool) -> i32;
    fn cxl_event_config(host_bridge: *mut pci_host_bridge, mds: *mut cxl_memdev_state, irq_avail: bool) -> i32;
    fn cxl_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> i32;
    fn cxl_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t;
    fn cxl_error_resume(pdev: *mut pci_dev);
    fn cxl_reset_done(pdev: *mut pci_dev);
}

const CXL_EVENT_HDR_FLAGS_REC_SEVERITY: u32 = GENMASK(1, 0);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
