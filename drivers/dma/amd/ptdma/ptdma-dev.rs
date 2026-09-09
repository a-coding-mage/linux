// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Passthru DMA device driver
 * -- Based on the CCP driver
 *
 * Copyright (C) 2016,2021 Advanced Micro Devices, Inc.
 *
 * Author: Sanjay R Mehta <sanju.mehta@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Kernel and PTDMA declarations are supplied by external dependencies.

/* Human-readable error strings */
static mut PT_ERROR_CODES: [&'static [u8]; 27] = [
    b"\0",
    b"ERR 01: ILLEGAL_ENGINE\0",
    b"ERR 03: ILLEGAL_FUNCTION_TYPE\0",
    b"ERR 04: ILLEGAL_FUNCTION_MODE\0",
    b"ERR 06: ILLEGAL_FUNCTION_SIZE\0",
    b"ERR 08: ILLEGAL_FUNCTION_RSVD\0",
    b"ERR 09: ILLEGAL_BUFFER_LENGTH\0",
    b"ERR 10: VLSB_FAULT\0",
    b"ERR 11: ILLEGAL_MEM_ADDR\0",
    b"ERR 12: ILLEGAL_MEM_SEL\0",
    b"ERR 13: ILLEGAL_CONTEXT_ID\0",
    b"ERR 15: 0xF Reserved\0",
    b"ERR 18: CMD_TIMEOUT\0",
    b"ERR 19: IDMA0_AXI_SLVERR\0",
    b"ERR 20: IDMA0_AXI_DECERR\0",
    b"ERR 21: 0x15 Reserved\0",
    b"ERR 22: IDMA1_AXI_SLAVE_FAULT\0",
    b"ERR 23: IDMA1_AIXI_DECERR\0",
    b"ERR 24: 0x18 Reserved\0",
    b"ERR 27: 0x1B Reserved\0",
    b"ERR 38: ODMA0_AXI_SLVERR\0",
    b"ERR 39: ODMA0_AXI_DECERR\0",
    b"ERR 40: 0x28 Reserved\0",
    b"ERR 41: ODMA1_AXI_SLVERR\0",
    b"ERR 42: ODMA1_AXI_DECERR\0",
    b"ERR 43: LSB_PARITY_ERR\0",
];

unsafe fn pt_log_error(d: *mut pt_device, e: i32) {
    dev_err((*d).dev, "PTDMA error: %s (0x%x)\n", PT_ERROR_CODES[e as usize].as_ptr(), e);
}

pub unsafe fn pt_start_queue(cmd_q: *mut pt_cmd_queue) {
    /* Turn on the run bit */
    iowrite32((*cmd_q).qcontrol | CMD_Q_RUN, (*cmd_q).reg_control);
}

pub unsafe fn pt_stop_queue(cmd_q: *mut pt_cmd_queue) {
    /* Turn off the run bit */
    iowrite32((*cmd_q).qcontrol & !CMD_Q_RUN, (*cmd_q).reg_control);
}

unsafe fn pt_core_execute_cmd(desc: *mut ptdma_desc, cmd_q: *mut pt_cmd_queue) -> i32 {
    let soc: bool = FIELD_GET(DWORD0_SOC, (*desc).dw0) != 0;
    let q_desc: *mut u8 = (*cmd_q).qbase.add((*cmd_q).qidx) as *mut u8;
    let mut tail: u32;
    let mut flags: c_ulong = 0;

    if soc {
        (*desc).dw0 |= FIELD_PREP(DWORD0_IOC, (*desc).dw0);
        (*desc).dw0 &= !DWORD0_SOC;
    }
    spin_lock_irqsave(&mut (*cmd_q).q_lock, &mut flags);

    /* Copy 32-byte command descriptor to hw queue. */
    memcpy(q_desc, desc as *const c_void, 32);
    (*cmd_q).qidx = ((*cmd_q).qidx + 1) % CMD_Q_LEN;

    /* The data used by this command must be flushed to memory */
    wmb();

    /* Write the new tail address back to the queue register */
    tail = lower_32_bits((*cmd_q).qdma_tail + (*cmd_q).qidx * Q_DESC_SIZE);
    iowrite32(tail, (*cmd_q).reg_control + 0x0004);

    /* Turn the queue back on using our cached control register */
    pt_start_queue(cmd_q);
    spin_unlock_irqrestore(&mut (*cmd_q).q_lock, flags);

    0
}

pub unsafe fn pt_core_perform_passthru(cmd_q: *mut pt_cmd_queue, pt_engine: *mut pt_passthru_engine) -> i32 {
    let mut desc: ptdma_desc = core::mem::zeroed();
    let pt: *mut pt_device = container_of(cmd_q, pt_device, cmd_q);

    (*cmd_q).cmd_error = 0;
    (*cmd_q).total_pt_ops += 1;
    desc.dw0 = CMD_DESC_DW0_VAL;
    desc.length = (*pt_engine).src_len;
    desc.src_lo = lower_32_bits((*pt_engine).src_dma);
    desc.dw3.src_hi = upper_32_bits((*pt_engine).src_dma);
    desc.dst_lo = lower_32_bits((*pt_engine).dst_dma);
    desc.dw5.dst_hi = upper_32_bits((*pt_engine).dst_dma);

    if (*cmd_q).int_en {
        pt_core_enable_queue_interrupts(pt);
    } else {
        pt_core_disable_queue_interrupts(pt);
    }

    pt_core_execute_cmd(&mut desc, cmd_q)
}

unsafe fn pt_do_cmd_complete(data: c_ulong) {
    let tdata: *mut pt_tasklet_data = data as *mut pt_tasklet_data;
    let cmd: *mut pt_cmd = (*tdata).cmd;
    let cmd_q: *mut pt_cmd_queue = &mut (*(*cmd).pt).cmd_q;
    let mut tail: u32;

    if (*cmd_q).cmd_error != 0 {
        /* Log the error and flush the queue by moving the head pointer */
        tail = lower_32_bits((*cmd_q).qdma_tail + (*cmd_q).qidx * Q_DESC_SIZE);
        pt_log_error((*cmd_q).pt, (*cmd_q).cmd_error);
        iowrite32(tail, (*cmd_q).reg_control + 0x0008);
    }

    ((*cmd).pt_cmd_callback)((*cmd).data, (*cmd).ret);
}

pub unsafe fn pt_check_status_trans(pt: *mut pt_device, cmd_q: *mut pt_cmd_queue) {
    let status: u32 = ioread32((*cmd_q).reg_control + 0x0010);
    if status != 0 {
        (*cmd_q).int_status = status;
        (*cmd_q).q_status = ioread32((*cmd_q).reg_control + 0x0100);
        (*cmd_q).q_int_status = ioread32((*cmd_q).reg_control + 0x0104);

        /* On error, only save the first error value */
        if (status & INT_ERROR) != 0 && (*cmd_q).cmd_error == 0 {
            (*cmd_q).cmd_error = CMD_Q_ERROR((*cmd_q).q_status);
        }

        /* Acknowledge the completion */
        iowrite32(status, (*cmd_q).reg_control + 0x0010);
        pt_do_cmd_complete(&mut (*pt).tdata as *mut _ as c_ulong);
    }
}

unsafe extern "C" fn pt_core_irq_handler(_irq: i32, data: *mut c_void) -> irqreturn_t {
    let pt: *mut pt_device = data as *mut pt_device;
    let cmd_q: *mut pt_cmd_queue = &mut (*pt).cmd_q;

    pt_core_disable_queue_interrupts(pt);
    (*pt).total_interrupts += 1;
    pt_check_status_trans(pt, cmd_q);
    pt_core_enable_queue_interrupts(pt);
    IRQ_HANDLED
}

pub unsafe fn pt_core_init(pt: *mut pt_device) -> i32 {
    let mut dma_pool_name: [u8; MAX_DMAPOOL_NAME_LEN] = [0; MAX_DMAPOOL_NAME_LEN];
    let cmd_q: *mut pt_cmd_queue = &mut (*pt).cmd_q;
    let mut dma_addr_lo: u32;
    let mut dma_addr_hi: u32;
    let dev: *mut device = (*pt).dev;
    let dma_pool: *mut dma_pool;
    let mut ret: i32;

    /* Allocate a dma pool for the queue */
    snprintf(dma_pool_name.as_mut_ptr(), dma_pool_name.len(), "%s_q", dev_name((*pt).dev));
    dma_pool = dma_pool_create(dma_pool_name.as_ptr(), dev, PT_DMAPOOL_MAX_SIZE, PT_DMAPOOL_ALIGN, 0);
    if dma_pool.is_null() { return -ENOMEM; }

    /* ptdma core initialisation */
    iowrite32(CMD_CONFIG_VHB_EN, (*pt).io_regs + CMD_CONFIG_OFFSET);
    iowrite32(CMD_QUEUE_PRIO, (*pt).io_regs + CMD_QUEUE_PRIO_OFFSET);
    iowrite32(CMD_TIMEOUT_DISABLE, (*pt).io_regs + CMD_TIMEOUT_OFFSET);
    iowrite32(CMD_CLK_GATE_CONFIG, (*pt).io_regs + CMD_CLK_GATE_CTL_OFFSET);
    iowrite32(CMD_CONFIG_REQID, (*pt).io_regs + CMD_REQID_CONFIG_OFFSET);

    (*cmd_q).pt = pt;
    (*cmd_q).dma_pool = dma_pool;
    spin_lock_init(&mut (*cmd_q).q_lock);
    (*cmd_q).qsize = Q_SIZE(Q_DESC_SIZE);
    (*cmd_q).qbase = dma_alloc_coherent(dev, (*cmd_q).qsize, &mut (*cmd_q).qbase_dma, GFP_KERNEL);
    if (*cmd_q).qbase.is_null() {
        dev_err(dev, "unable to allocate command queue\n");
        ret = -ENOMEM;
        goto e_destroy_pool;
    }
    (*cmd_q).qidx = 0;
    (*cmd_q).reg_control = (*pt).io_regs + CMD_Q_STATUS_INCR;
    pt_core_disable_queue_interrupts(pt);
    (*cmd_q).qcontrol = 0;
    iowrite32((*cmd_q).qcontrol, (*cmd_q).reg_control);
    ioread32((*cmd_q).reg_control + 0x0104);
    ioread32((*cmd_q).reg_control + 0x0100);
    iowrite32(SUPPORTED_INTERRUPTS, (*cmd_q).reg_control + 0x0010);
    ret = request_irq((*pt).pt_irq, pt_core_irq_handler, 0, dev_name((*pt).dev), pt);
    if ret != 0 {
        dev_err(dev, "unable to allocate an IRQ\n");
        goto e_free_dma;
    }

    (*cmd_q).qcontrol &= !CMD_Q_SIZE;
    (*cmd_q).qcontrol |= FIELD_PREP(CMD_Q_SIZE, QUEUE_SIZE_VAL);
    (*cmd_q).qdma_tail = (*cmd_q).qbase_dma;
    dma_addr_lo = lower_32_bits((*cmd_q).qdma_tail);
    iowrite32(dma_addr_lo, (*cmd_q).reg_control + 0x0004);
    iowrite32(dma_addr_lo, (*cmd_q).reg_control + 0x0008);
    dma_addr_hi = upper_32_bits((*cmd_q).qdma_tail);
    (*cmd_q).qcontrol |= dma_addr_hi << 16;
    iowrite32((*cmd_q).qcontrol, (*cmd_q).reg_control);
    pt_core_enable_queue_interrupts(pt);
    ret = pt_dmaengine_register(pt);
    if ret != 0 { goto e_free_irq; }
    ptdma_debugfs_setup(pt);
    return 0;

e_free_irq:
    free_irq((*pt).pt_irq, pt);
e_free_dma:
    dma_free_coherent(dev, (*cmd_q).qsize, (*cmd_q).qbase, (*cmd_q).qbase_dma);
e_destroy_pool:
    dma_pool_destroy((*pt).cmd_q.dma_pool);
    ret
}

pub unsafe fn pt_core_destroy(pt: *mut pt_device) {
    let dev: *mut device = (*pt).dev;
    let cmd_q: *mut pt_cmd_queue = &mut (*pt).cmd_q;
    let mut cmd: *mut pt_cmd;

    /* Unregister the DMA engine */
    pt_dmaengine_unregister(pt);
    /* Disable and clear interrupts */
    pt_core_disable_queue_interrupts(pt);
    /* Turn off the run bit */
    pt_stop_queue(cmd_q);
    /* Clear the interrupt status */
    iowrite32(SUPPORTED_INTERRUPTS, (*cmd_q).reg_control + 0x0010);
    ioread32((*cmd_q).reg_control + 0x0104);
    ioread32((*cmd_q).reg_control + 0x0100);
    free_irq((*pt).pt_irq, pt);
    dma_free_coherent(dev, (*cmd_q).qsize, (*cmd_q).qbase, (*cmd_q).qbase_dma);

    /* Flush the cmd queue */
    while !list_empty(&mut (*pt).cmd) {
        /* Invoke the callback directly with an error code */
        cmd = list_first_entry(&mut (*pt).cmd, pt_cmd, entry);
        list_del(&mut (*cmd).entry);
        ((*cmd).pt_cmd_callback)((*cmd).data, -ENODEV);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
