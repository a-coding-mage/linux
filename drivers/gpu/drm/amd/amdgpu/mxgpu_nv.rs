/* Translated from mxgpu_nv.c. Includes and external kernel/project symbols are supplied elsewhere. */

unsafe fn xgpu_nv_mailbox_send_ack(adev: *mut amdgpu_device) {
    WREG8(adev, NV_MAIBOX_CONTROL_RCV_OFFSET_BYTE, 2);
}

unsafe fn xgpu_nv_mailbox_set_valid(adev: *mut amdgpu_device, val: bool) {
    WREG8(adev, NV_MAIBOX_CONTROL_TRN_OFFSET_BYTE, if val { 1 } else { 0 });
}

unsafe fn xgpu_nv_mailbox_peek_msg(adev: *mut amdgpu_device) -> idh_event {
    RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW0)
}

unsafe fn xgpu_nv_mailbox_rcv_msg(adev: *mut amdgpu_device, event: idh_event) -> i32 {
    let mut r: i32 = 0;
    let reg = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW0);
    if reg == IDH_FAIL { r = -EINVAL; }
    if reg == IDH_UNRECOV_ERR_NOTIFICATION { r = -ENODEV; }
    else if reg != event { return -ENOENT; }
    xgpu_nv_mailbox_send_ack(adev);
    r
}

unsafe fn xgpu_nv_peek_ack(adev: *mut amdgpu_device) -> u8 {
    RREG8(adev, NV_MAIBOX_CONTROL_TRN_OFFSET_BYTE) & 2
}

unsafe fn xgpu_nv_poll_ack(adev: *mut amdgpu_device) -> i32 {
    let mut timeout = NV_MAILBOX_POLL_ACK_TIMEDOUT;
    loop {
        let reg = RREG8(adev, NV_MAIBOX_CONTROL_TRN_OFFSET_BYTE);
        if reg & 2 != 0 { return 0; }
        mdelay(5);
        timeout -= 5;
        if timeout <= 1 { break; }
    }
    dev_err((*adev).dev, "Doesn't get TRN_MSG_ACK from pf in %d msec\n", NV_MAILBOX_POLL_ACK_TIMEDOUT);
    -ETIME
}

unsafe fn xgpu_nv_poll_msg(adev: *mut amdgpu_device, event: idh_event) -> i32 {
    let mut r;
    let mut now = ktime_to_ms(ktime_get()) as u64;
    let timeout = now + NV_MAILBOX_POLL_MSG_TIMEDOUT;
    let ras = amdgpu_ras_get_context(adev);
    loop {
        r = xgpu_nv_mailbox_rcv_msg(adev, event);
        if r == 0 {
            dev_dbg((*adev).dev, "rcv_msg 0x%x after %llu ms\n", event, NV_MAILBOX_POLL_MSG_TIMEDOUT - timeout + now);
            return 0;
        } else if r == -ENODEV {
            if !amdgpu_ras_is_rma(adev) {
                (*ras).is_rma = true;
                dev_err((*adev).dev, "VF is in an unrecoverable state. Runtime Services are halted.\n");
            }
            return r;
        }
        msleep(10);
        now = ktime_to_ms(ktime_get()) as u64;
        if timeout <= now { break; }
    }
    dev_dbg((*adev).dev, "nv_poll_msg timed out\n");
    -ETIME
}

unsafe fn xgpu_nv_mailbox_trans_msg(adev: *mut amdgpu_device, req: idh_request, data1: u32, data2: u32, data3: u32) {
    let mut r;
    let mut trn;
    loop {
        xgpu_nv_mailbox_set_valid(adev, false);
        trn = xgpu_nv_peek_ack(adev);
        if trn != 0 {
            dev_err_ratelimited((*adev).dev, "trn=%x ACK should not assert! wait again !\n", trn);
            msleep(1);
        }
        if trn == 0 { break; }
    }
    dev_dbg((*adev).dev, "trans_msg req = 0x%x, data1 = 0x%x\n", req, data1);
    WREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_TRN_DW0, req);
    WREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_TRN_DW1, data1);
    WREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_TRN_DW2, data2);
    WREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_TRN_DW3, data3);
    xgpu_nv_mailbox_set_valid(adev, true);
    r = xgpu_nv_poll_ack(adev);
    if r != 0 { dev_err((*adev).dev, "Doesn't get ack from pf, continue\n"); }
    xgpu_nv_mailbox_set_valid(adev, false);
}

unsafe fn xgpu_nv_send_access_requests_with_param(adev: *mut amdgpu_device, req: idh_request, data1: u32, data2: u32, data3: u32) -> i32 {
    let virt = &mut (*adev).virt;
    let mut r = 0;
    let mut retry = 1;
    let mut event: idh_event = -1;
    mutex_lock(&mut virt.access_req_mutex);
    'send_request: loop {
        if amdgpu_ras_is_rma(adev) { r = -ENODEV; break; }
        xgpu_nv_mailbox_trans_msg(adev, req, data1, data2, data3);
        event = match req {
            IDH_REQ_GPU_INIT_ACCESS | IDH_REQ_GPU_FINI_ACCESS | IDH_REQ_GPU_RESET_ACCESS => IDH_READY_TO_ACCESS_GPU,
            IDH_REQ_GPU_INIT_DATA => IDH_REQ_GPU_INIT_DATA_READY,
            IDH_RAS_POISON if data1 != 0 => IDH_RAS_POISON_READY,
            IDH_REQ_RAS_ERROR_COUNT => IDH_RAS_ERROR_COUNT_READY,
            IDH_REQ_RAS_CPER_DUMP => IDH_RAS_CPER_DUMP_READY,
            IDH_REQ_RAS_CHK_CRITI => IDH_REQ_RAS_CHK_CRITI_READY,
            IDH_REQ_RAS_REMOTE_CMD => IDH_REQ_RAS_REMOTE_CMD_READY,
            IDH_REQ_PTL_UPDATE => IDH_PTL_UPDATE_READY,
            _ => -1,
        };
        if event != -1 {
            r = xgpu_nv_poll_msg(adev, event);
            if r != 0 {
                retry += 1;
                if retry < 5 { continue 'send_request; }
                if req != IDH_REQ_GPU_INIT_DATA {
                    dev_err((*adev).dev, "Doesn't get msg:%d from pf, error=%d\n", event, r);
                    break;
                }
                virt.req_init_data_ver = 0;
            } else {
                if req == IDH_REQ_GPU_INIT_DATA {
                    match RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW1) {
                        GPU_CRIT_REGION_V2 => { virt.req_init_data_ver = GPU_CRIT_REGION_V2; virt.init_data_header.offset = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW2); virt.init_data_header.size_kb = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW3); }
                        _ => { virt.req_init_data_ver = GPU_CRIT_REGION_V1; virt.init_data_header.offset = -1; virt.init_data_header.size_kb = 0; }
                    }
                }
            }
            if req == IDH_REQ_GPU_INIT_ACCESS || req == IDH_REQ_GPU_RESET_ACCESS { virt.fw_reserve.checksum_key = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW2); }
            if req == IDH_REQ_PTL_UPDATE {
                let dw1 = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW1);
                let dw2 = RREG32_NO_KIQ(adev, mmMAILBOX_MSGBUF_RCV_DW2);
                let status = AMD_SRIOV_PTL_UNPACK_STATUS(dw1);
                if status == AMD_SRIOV_RESP_UNSUPPORTED { r = -EOPNOTSUPP; }
                else if status == AMD_SRIOV_RESP_FAIL { r = -EIO; }
                else { virt.ptl_state = AMD_SRIOV_PTL_UNPACK_STATE(dw1); virt.ptl_pref_format1 = AMD_SRIOV_PTL_UNPACK_FMT1(dw2); virt.ptl_pref_format2 = AMD_SRIOV_PTL_UNPACK_FMT2(dw2); }
            }
        }
        break;
    }
    mutex_unlock(&mut virt.access_req_mutex);
    r
}

unsafe fn xgpu_nv_send_access_requests(adev: *mut amdgpu_device, req: idh_request) -> i32 { xgpu_nv_send_access_requests_with_param(adev, req, 0, 0, 0) }

unsafe fn xgpu_nv_request_reset(adev: *mut amdgpu_device) -> i32 {
    let mut ret = 0; let mut i = 0;
    while i < NV_MAILBOX_POLL_MSG_REP_MAX { ret = xgpu_nv_send_access_requests(adev, IDH_REQ_GPU_RESET_ACCESS); if ret == 0 { break; } i += 1; }
    ret
}

unsafe fn xgpu_nv_request_full_gpu_access(adev: *mut amdgpu_device, init: bool) -> i32 { xgpu_nv_send_access_requests(adev, if init { IDH_REQ_GPU_INIT_ACCESS } else { IDH_REQ_GPU_FINI_ACCESS }) }
unsafe fn xgpu_nv_release_full_gpu_access(adev: *mut amdgpu_device, init: bool) -> i32 { xgpu_nv_send_access_requests(adev, if init { IDH_REL_GPU_INIT_ACCESS } else { IDH_REL_GPU_FINI_ACCESS }) }
unsafe fn xgpu_nv_request_init_data(adev: *mut amdgpu_device) -> i32 { xgpu_nv_send_access_requests_with_param(adev, IDH_REQ_GPU_INIT_DATA, 0, GPU_CRIT_REGION_V2, 0) }

unsafe fn xgpu_nv_mailbox_ack_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 { dev_dbg((*adev).dev, "get ack intr and do nothing.\n"); 0 }
unsafe fn xgpu_nv_set_mailbox_ack_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _type: u32, state: amdgpu_interrupt_state) -> i32 { let mut tmp = RREG32_NO_KIQ(adev, mmMAILBOX_INT_CNTL); if state == AMDGPU_IRQ_STATE_ENABLE { tmp |= 2; } else { tmp &= !2; } WREG32_NO_KIQ(adev, mmMAILBOX_INT_CNTL, tmp); 0 }
unsafe fn xgpu_nv_ready_to_reset(adev: *mut amdgpu_device) { xgpu_nv_mailbox_trans_msg(adev, IDH_READY_TO_RESET, 0, 0, 0); }
unsafe fn xgpu_nv_wait_reset(adev: *mut amdgpu_device) -> i32 { let mut timeout = NV_MAILBOX_POLL_FLR_TIMEDOUT; loop { if xgpu_nv_mailbox_peek_msg(adev) == IDH_FLR_NOTIFICATION_CMPL { dev_dbg((*adev).dev, "Got NV IDH_FLR_NOTIFICATION_CMPL after %d ms\n", NV_MAILBOX_POLL_FLR_TIMEDOUT - timeout); return 0; } msleep(10); timeout -= 10; if timeout <= 1 { break; } } dev_dbg((*adev).dev, "waiting NV IDH_FLR_NOTIFICATION_CMPL timeout\n"); -ETIME }

unsafe fn xgpu_nv_mailbox_flr_work(work: *mut work_struct) {
    let virt = container_of(work, amdgpu_virt, flr_work); let adev = container_of(virt, amdgpu_device, virt); let mut reset_context = amdgpu_reset_context::default();
    amdgpu_virt_fini_data_exchange(adev);
    if amdgpu_device_should_recover_gpu(adev) && (!amdgpu_device_has_job_running(adev) || (*adev).sdma_timeout == MAX_SCHEDULE_TIMEOUT || (*adev).gfx_timeout == MAX_SCHEDULE_TIMEOUT || (*adev).compute_timeout == MAX_SCHEDULE_TIMEOUT || (*adev).video_timeout == MAX_SCHEDULE_TIMEOUT) {
        reset_context.method = AMD_RESET_METHOD_NONE; reset_context.reset_req_dev = adev; clear_bit(AMDGPU_NEED_FULL_RESET, &mut reset_context.flags); set_bit(AMDGPU_HOST_FLR, &mut reset_context.flags); amdgpu_device_gpu_recover(adev, core::ptr::null_mut(), &mut reset_context);
    }
}

unsafe fn xgpu_nv_mailbox_req_bad_pages_work(work: *mut work_struct) { let virt = container_of(work, amdgpu_virt, req_bad_pages_work); let adev = container_of(virt, amdgpu_device, virt); if down_read_trylock((*(*adev).reset_domain).sem) { amdgpu_virt_fini_data_exchange(adev); amdgpu_virt_request_bad_pages(adev); up_read((*(*adev).reset_domain).sem); } }
unsafe fn xgpu_nv_mailbox_handle_bad_pages_work(work: *mut work_struct) { let virt = container_of(work, amdgpu_virt, handle_bad_pages_work); let adev = container_of(virt, amdgpu_device, virt); if down_read_trylock((*(*adev).reset_domain).sem) { amdgpu_virt_fini_data_exchange(adev); amdgpu_virt_init_data_exchange(adev); up_read((*(*adev).reset_domain).sem); } }

unsafe fn xgpu_nv_set_mailbox_rcv_irq(adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _type: u32, state: amdgpu_interrupt_state) -> i32 { let mut tmp = RREG32_NO_KIQ(adev, mmMAILBOX_INT_CNTL); if state == AMDGPU_IRQ_STATE_ENABLE { tmp |= 1; } else { tmp &= !1; } WREG32_NO_KIQ(adev, mmMAILBOX_INT_CNTL, tmp); 0 }

unsafe fn xgpu_nv_mailbox_rcv_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 {
    let event = xgpu_nv_mailbox_peek_msg(adev); let ras = amdgpu_ras_get_context(adev);
    match event {
        IDH_RAS_BAD_PAGES_READY => { xgpu_nv_mailbox_send_ack(adev); if amdgpu_sriov_runtime(adev) { schedule_work(&mut (*adev).virt.handle_bad_pages_work); } }
        IDH_RAS_BAD_PAGES_NOTIFICATION => { xgpu_nv_mailbox_send_ack(adev); if amdgpu_sriov_runtime(adev) { schedule_work(&mut (*adev).virt.req_bad_pages_work); } }
        IDH_UNRECOV_ERR_NOTIFICATION => { xgpu_nv_mailbox_send_ack(adev); if !amdgpu_ras_is_rma(adev) { (*ras).is_rma = true; dev_err((*adev).dev, "VF is in an unrecoverable state. Runtime Services are halted.\n"); } if amdgpu_sriov_runtime(adev) { WARN_ONCE(!amdgpu_reset_domain_schedule((*adev).reset_domain, &mut (*adev).virt.flr_work), "Failed to queue work! at %s", __func__); } }
        IDH_FLR_NOTIFICATION => { if amdgpu_sriov_runtime(adev) { WARN_ONCE(!amdgpu_reset_domain_schedule((*adev).reset_domain, &mut (*adev).virt.flr_work), "Failed to queue work! at %s", __func__); } }
        IDH_CLR_MSG_BUF | IDH_FLR_NOTIFICATION_CMPL | IDH_READY_TO_ACCESS_GPU | _ => {}
    }
    0
}

static XgpuNvMailboxAckIrqFuncs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: xgpu_nv_set_mailbox_ack_irq, process: xgpu_nv_mailbox_ack_irq };
static XgpuNvMailboxRcvIrqFuncs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: xgpu_nv_set_mailbox_rcv_irq, process: xgpu_nv_mailbox_rcv_irq };

pub unsafe fn xgpu_nv_mailbox_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).virt.ack_irq.num_types = 1; (*adev).virt.ack_irq.funcs = &XgpuNvMailboxAckIrqFuncs; (*adev).virt.rcv_irq.num_types = 1; (*adev).virt.rcv_irq.funcs = &XgpuNvMailboxRcvIrqFuncs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
