/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe fn xgpu_ai_mailbox_send_ack(adev: *mut amdgpu_device) {
    WREG8(adev, AI_MAIBOX_CONTROL_RCV_OFFSET_BYTE, 2);
}

unsafe fn xgpu_ai_mailbox_set_valid(adev: *mut amdgpu_device, val: bool) {
    WREG8(adev, AI_MAIBOX_CONTROL_TRN_OFFSET_BYTE, if val { 1 } else { 0 });
}

/* See the C implementation: this may only be called from the IRQ routine. */
unsafe fn xgpu_ai_mailbox_peek_msg(adev: *mut amdgpu_device) -> idh_event {
    RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_RCV_DW0))
}

unsafe fn xgpu_ai_mailbox_rcv_msg(adev: *mut amdgpu_device, event: idh_event) -> i32 {
    let reg = RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_RCV_DW0));
    if reg != event { return -ENOENT; }
    xgpu_ai_mailbox_send_ack(adev);
    0
}

unsafe fn xgpu_ai_peek_ack(adev: *mut amdgpu_device) -> u8 {
    RREG8(adev, AI_MAIBOX_CONTROL_TRN_OFFSET_BYTE) & 2
}

unsafe fn xgpu_ai_poll_ack(adev: *mut amdgpu_device) -> i32 {
    let mut timeout = AI_MAILBOX_POLL_ACK_TIMEDOUT;
    loop {
        let reg = RREG8(adev, AI_MAIBOX_CONTROL_TRN_OFFSET_BYTE);
        if reg & 2 != 0 { return 0; }
        mdelay(5);
        timeout -= 5;
        if timeout <= 1 { break; }
    }
    dev_err((*adev).dev, "Doesn't get TRN_MSG_ACK from pf in %d msec\n", AI_MAILBOX_POLL_ACK_TIMEDOUT);
    -ETIME
}

unsafe fn xgpu_ai_poll_msg(adev: *mut amdgpu_device, event: idh_event) -> i32 {
    let mut timeout = AI_MAILBOX_POLL_MSG_TIMEDOUT;
    let mut r;
    loop {
        r = xgpu_ai_mailbox_rcv_msg(adev, event);
        if r == 0 { return 0; }
        msleep(10);
        timeout -= 10;
        if timeout <= 1 { break; }
    }
    dev_err((*adev).dev, "Doesn't get msg:%d from pf, error=%d\n", event, r);
    -ETIME
}

unsafe fn xgpu_ai_mailbox_trans_msg(adev: *mut amdgpu_device, req: idh_request, data1: u32, data2: u32, data3: u32) {
    let mut reg;
    let mut r;
    let mut trn;
    loop {
        xgpu_ai_mailbox_set_valid(adev, false);
        trn = xgpu_ai_peek_ack(adev);
        if trn != 0 { dev_err_ratelimited((*adev).dev, "trn=%x ACK should not assert! wait again !\n", trn); msleep(1); }
        if trn == 0 { break; }
    }
    reg = RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW0));
    reg = REG_SET_FIELD(reg, BIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW0, MSGBUF_DATA, req);
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW0), reg);
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW1), data1);
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW2), data2);
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_TRN_DW3), data3);
    xgpu_ai_mailbox_set_valid(adev, true);
    r = xgpu_ai_poll_ack(adev);
    if r != 0 { dev_err((*adev).dev, "Doesn't get ack from pf, continue\n"); }
    xgpu_ai_mailbox_set_valid(adev, false);
}

unsafe fn xgpu_ai_send_access_requests(adev: *mut amdgpu_device, req: idh_request) -> i32 {
    xgpu_ai_mailbox_trans_msg(adev, req, 0, 0, 0);
    if req == IDH_REQ_GPU_INIT_ACCESS || req == IDH_REQ_GPU_FINI_ACCESS || req == IDH_REQ_GPU_RESET_ACCESS {
        let r = xgpu_ai_poll_msg(adev, IDH_READY_TO_ACCESS_GPU);
        if r != 0 { dev_err((*adev).dev, "Doesn't get READY_TO_ACCESS_GPU from pf, give up\n"); return r; }
        if req == IDH_REQ_GPU_INIT_ACCESS || req == IDH_REQ_GPU_RESET_ACCESS {
            (*adev).virt.fw_reserve.checksum_key = RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_MSGBUF_RCV_DW2));
        }
    } else if req == IDH_REQ_GPU_INIT_DATA {
        xgpu_ai_poll_msg(adev, IDH_REQ_GPU_INIT_DATA_READY);
        (*adev).virt.req_init_data_ver = GPU_CRIT_REGION_V1;
    }
    0
}

unsafe fn xgpu_ai_request_reset(adev: *mut amdgpu_device) -> i32 {
    let mut ret = 0;
    let mut i = 0;
    while i < AI_MAILBOX_POLL_MSG_REP_MAX { ret = xgpu_ai_send_access_requests(adev, IDH_REQ_GPU_RESET_ACCESS); if ret == 0 { break; } i += 1; }
    ret
}

unsafe fn xgpu_ai_request_full_gpu_access(adev: *mut amdgpu_device, init: bool) -> i32 {
    xgpu_ai_send_access_requests(adev, if init { IDH_REQ_GPU_INIT_ACCESS } else { IDH_REQ_GPU_FINI_ACCESS })
}

unsafe fn xgpu_ai_release_full_gpu_access(adev: *mut amdgpu_device, init: bool) -> i32 {
    xgpu_ai_send_access_requests(adev, if init { IDH_REL_GPU_INIT_ACCESS } else { IDH_REL_GPU_FINI_ACCESS })
}

unsafe fn xgpu_ai_mailbox_ack_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 {
    dev_dbg((*adev).dev, "get ack intr and do nothing.\n"); 0
}

unsafe fn xgpu_ai_set_mailbox_ack_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _ty: u32, state: amdgpu_interrupt_state) -> i32 {
    let mut tmp = RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_INT_CNTL));
    tmp = REG_SET_FIELD(tmp, BIF_BX_PF0_MAILBOX_INT_CNTL, ACK_INT_EN, if state == AMDGPU_IRQ_STATE_ENABLE { 1 } else { 0 });
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_INT_CNTL), tmp); 0
}

unsafe fn xgpu_ai_ready_to_reset(adev: *mut amdgpu_device) { xgpu_ai_mailbox_trans_msg(adev, IDH_READY_TO_RESET, 0, 0, 0); }

unsafe fn xgpu_ai_wait_reset(adev: *mut amdgpu_device) -> i32 {
    let mut timeout = AI_MAILBOX_POLL_FLR_TIMEDOUT;
    loop {
        if xgpu_ai_mailbox_peek_msg(adev) == IDH_FLR_NOTIFICATION_CMPL { return 0; }
        msleep(10); timeout -= 10; if timeout <= 1 { break; }
    }
    dev_dbg((*adev).dev, "waiting AI IDH_FLR_NOTIFICATION_CMPL timeout\n"); -ETIME
}

unsafe fn xgpu_ai_mailbox_flr_work(work: *mut work_struct) {
    let virt = container_of!(work, amdgpu_virt, flr_work);
    let adev = container_of!(virt, amdgpu_device, virt);
    let mut reset_context = amdgpu_reset_context::default();
    amdgpu_virt_fini_data_exchange(adev);
    if amdgpu_device_should_recover_gpu(adev) && (!amdgpu_device_has_job_running(adev) || (*adev).sdma_timeout == MAX_SCHEDULE_TIMEOUT) {
        reset_context.method = AMD_RESET_METHOD_NONE; reset_context.reset_req_dev = adev;
        clear_bit(AMDGPU_NEED_FULL_RESET, &mut reset_context.flags); set_bit(AMDGPU_HOST_FLR, &mut reset_context.flags);
        amdgpu_device_gpu_recover(adev, core::ptr::null_mut(), &mut reset_context);
    }
}

unsafe fn xgpu_ai_mailbox_req_bad_pages_work(work: *mut work_struct) {
    let virt = container_of!(work, amdgpu_virt, req_bad_pages_work); let adev = container_of!(virt, amdgpu_device, virt);
    if down_read_trylock((*adev).reset_domain.sem) { amdgpu_virt_fini_data_exchange(adev); amdgpu_virt_request_bad_pages(adev); up_read((*adev).reset_domain.sem); }
}

unsafe fn xgpu_ai_mailbox_handle_bad_pages_work(work: *mut work_struct) {
    let virt = container_of!(work, amdgpu_virt, handle_bad_pages_work); let adev = container_of!(virt, amdgpu_device, virt);
    if down_read_trylock((*adev).reset_domain.sem) { amdgpu_virt_fini_data_exchange(adev); amdgpu_virt_init_data_exchange(adev); up_read((*adev).reset_domain.sem); }
}

unsafe fn xgpu_ai_set_mailbox_rcv_irq(adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _ty: u32, state: amdgpu_interrupt_state) -> i32 {
    let mut tmp = RREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_INT_CNTL));
    tmp = REG_SET_FIELD(tmp, BIF_BX_PF0_MAILBOX_INT_CNTL, VALID_INT_EN, if state == AMDGPU_IRQ_STATE_ENABLE { 1 } else { 0 });
    WREG32_NO_KIQ(adev, SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_PF0_MAILBOX_INT_CNTL), tmp); 0
}

unsafe fn xgpu_ai_mailbox_rcv_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 {
    let event = xgpu_ai_mailbox_peek_msg(adev);
    let ras = amdgpu_ras_get_context(adev);
    match event {
        IDH_RAS_BAD_PAGES_READY => { xgpu_ai_mailbox_send_ack(adev); if amdgpu_sriov_runtime(adev) { schedule_work(&mut (*adev).virt.handle_bad_pages_work); } }
        IDH_RAS_BAD_PAGES_NOTIFICATION => { xgpu_ai_mailbox_send_ack(adev); if amdgpu_sriov_runtime(adev) { schedule_work(&mut (*adev).virt.req_bad_pages_work); } }
        IDH_UNRECOV_ERR_NOTIFICATION => { xgpu_ai_mailbox_send_ack(adev); (*ras).is_rma = true; dev_err((*adev).dev, "VF is in an unrecoverable state. Runtime Services are halted.\n"); }
        IDH_QUERY_ALIVE => xgpu_ai_mailbox_send_ack(adev),
        IDH_FLR_NOTIFICATION | IDH_CLR_MSG_BUF | IDH_FLR_NOTIFICATION_CMPL | IDH_READY_TO_ACCESS_GPU => (),
        _ => (),
    } 0
}

pub unsafe fn xgpu_ai_mailbox_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).virt.ack_irq.num_types = 1; (*adev).virt.ack_irq.funcs = &xgpu_ai_mailbox_ack_irq_funcs; (*adev).virt.rcv_irq.num_types = 1; (*adev).virt.rcv_irq.funcs = &xgpu_ai_mailbox_rcv_irq_funcs; }
pub unsafe fn xgpu_ai_mailbox_add_irq_id(adev: *mut amdgpu_device) -> i32 { let r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_BIF, 135, &mut (*adev).virt.rcv_irq); if r != 0 { return r; } amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_BIF, 138, &mut (*adev).virt.ack_irq) }
pub unsafe fn xgpu_ai_mailbox_get_irq(adev: *mut amdgpu_device) -> i32 { let r = amdgpu_irq_get(adev, &mut (*adev).virt.rcv_irq, 0); if r != 0 { return r; } amdgpu_irq_get(adev, &mut (*adev).virt.ack_irq, 0) }
pub unsafe fn xgpu_ai_mailbox_put_irq(adev: *mut amdgpu_device) { amdgpu_irq_put(adev, &mut (*adev).virt.ack_irq, 0); amdgpu_irq_put(adev, &mut (*adev).virt.rcv_irq, 0); }
unsafe fn xgpu_ai_request_init_data(adev: *mut amdgpu_device) -> i32 { xgpu_ai_send_access_requests(adev, IDH_REQ_GPU_INIT_DATA) }
unsafe fn xgpu_ai_ras_poison_handler(adev: *mut amdgpu_device, _block: amdgpu_ras_block) { xgpu_ai_send_access_requests(adev, IDH_RAS_POISON); }
unsafe fn xgpu_ai_rcvd_ras_intr(adev: *mut amdgpu_device) -> bool { let msg = xgpu_ai_mailbox_peek_msg(adev); msg == IDH_RAS_ERROR_DETECTED || msg == 0xFFFF_FFFF }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
