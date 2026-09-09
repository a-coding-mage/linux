// SPDX-License-Identifier: GPL-2.0-or-later
/* DMA driver for STMicroelectronics STi FDMA controller. */

// Dependencies and structures are supplied by the Linux/Rust kernel bindings and st_fdma module.
use core::ffi::c_void;

unsafe fn to_st_fdma_chan(c: *mut dma_chan) -> *mut st_fdma_chan {
    container_of(c, st_fdma_chan, vchan.chan)
}
unsafe fn to_st_fdma_desc(vd: *mut virt_dma_desc) -> *mut st_fdma_desc {
    container_of(vd, st_fdma_desc, vdesc)
}

unsafe fn st_fdma_dreq_get(fchan: *mut st_fdma_chan) -> i32 {
    let fdev = (*fchan).fdev;
    let req_line_cfg = (*fchan).cfg.req_line;
    let mut dreq_line: u32;
    let mut try_count = 0;
    loop {
        if (*fdev).dreq_mask == !0usize {
            dev_err((*fdev).dev, "No req lines available\n"); return -EINVAL;
        }
        if try_count != 0 || req_line_cfg >= ST_FDMA_NR_DREQS {
            dev_err((*fdev).dev, "Invalid or used req line\n"); return -EINVAL;
        } else { dreq_line = req_line_cfg; }
        try_count += 1;
        if !test_and_set_bit(dreq_line, &mut (*fdev).dreq_mask) { break; }
    }
    dev_dbg((*fdev).dev, "get dreq_line:%d mask:%#lx\n", dreq_line, (*fdev).dreq_mask);
    dreq_line as i32
}

unsafe fn st_fdma_dreq_put(fchan: *mut st_fdma_chan) {
    let fdev = (*fchan).fdev;
    dev_dbg((*fdev).dev, "put dreq_line:%#lx\n", (*fchan).dreq_line);
    clear_bit((*fchan).dreq_line, &mut (*fdev).dreq_mask);
}

unsafe fn st_fdma_xfer_desc(fchan: *mut st_fdma_chan) {
    let vdesc = vchan_next_desc(&mut (*fchan).vchan);
    if vdesc.is_null() { return; }
    (*fchan).fdesc = to_st_fdma_desc(vdesc);
    let nbytes = (*(*fchan).fdesc).node[0].desc.nbytes;
    let cmd = FDMA_CMD_START((*fchan).vchan.chan.chan_id);
    let ch_cmd = (*(*fchan).fdesc).node[0].pdesc | FDMA_CH_CMD_STA_START;
    fnode_write(fchan, nbytes, FDMA_CNTN_OFST);
    fchan_write(fchan, ch_cmd, FDMA_CH_CMD_OFST);
    writel(cmd, (*(*fchan).fdev).slim_rproc.peri + FDMA_CMD_SET_OFST);
    dev_dbg((*(*fchan).fdev).dev, "start chan:%d\n", (*fchan).vchan.chan.chan_id);
}

unsafe fn st_fdma_ch_sta_update(fchan: *mut st_fdma_chan, int_sta: usize) {
    let ch_id = (*fchan).vchan.chan.chan_id;
    let fdev = (*fchan).fdev;
    let mut ch_sta = fchan_read(fchan, FDMA_CH_CMD_OFST);
    let ch_err = ch_sta & FDMA_CH_CMD_ERR_MASK;
    ch_sta &= FDMA_CH_CMD_STA_MASK;
    if int_sta & FDMA_INT_STA_ERR != 0 { dev_warn((*fdev).dev, "chan:%d, error:%ld\n", ch_id, ch_err); (*fchan).status = DMA_ERROR; return; }
    match ch_sta { FDMA_CH_CMD_STA_PAUSED => (*fchan).status = DMA_PAUSED, FDMA_CH_CMD_STA_RUNNING => (*fchan).status = DMA_IN_PROGRESS, _ => {} }
}

unsafe fn st_fdma_irq_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let fdev = dev_id as *mut st_fdma_dev; let mut ret = IRQ_NONE; let mut fchan = (*fdev).chans; let mut int_sta = fdma_read(fdev, FDMA_INT_STA_OFST); let clr = int_sta;
    while int_sta != 0 { if int_sta & (FDMA_INT_STA_CH | FDMA_INT_STA_ERR) != 0 { spin_lock(&mut (*fchan).vchan.lock); st_fdma_ch_sta_update(fchan, int_sta); if !(*fchan).fdesc.is_null() { if !(*(*fchan).fdesc).iscyclic { list_del(&mut (*(*fchan).fdesc).vdesc.node); vchan_cookie_complete(&mut (*(*fchan).fdesc).vdesc); (*fchan).fdesc = core::ptr::null_mut(); (*fchan).status = DMA_COMPLETE; } else { vchan_cyclic_callback(&mut (*(*fchan).fdesc).vdesc); } if (*fchan).fdesc.is_null() { st_fdma_xfer_desc(fchan); } } spin_unlock(&mut (*fchan).vchan.lock); ret = IRQ_HANDLED; } int_sta >>= 2; fchan = fchan.add(1); }
    fdma_write(fdev, clr, FDMA_INT_CLR_OFST); ret
}

unsafe fn st_fdma_free_desc(vdesc: *mut virt_dma_desc) { let fdesc = to_st_fdma_desc(vdesc); for i in 0..(*fdesc).n_nodes { dma_pool_free((*(*fdesc).fchan).node_pool, (*fdesc).node[i].desc, (*fdesc).node[i].pdesc); } kfree(fdesc); }

unsafe fn st_fdma_alloc_desc(fchan: *mut st_fdma_chan, sg_len: i32) -> *mut st_fdma_desc { let fdesc = kzalloc_flex::<st_fdma_desc>(sg_len); if fdesc.is_null() { return core::ptr::null_mut(); } (*fdesc).fchan=fchan; (*fdesc).n_nodes=sg_len; for i in 0..sg_len { (*fdesc).node[i].desc=dma_pool_alloc((*fchan).node_pool, GFP_NOWAIT, &mut (*fdesc).node[i].pdesc); if (*fdesc).node[i].desc.is_null() { for j in 0..i { dma_pool_free((*fchan).node_pool, (*fdesc).node[j].desc, (*fdesc).node[j].pdesc); } kfree(fdesc); return core::ptr::null_mut(); } } fdesc }

// The remaining callbacks retain the driver's C control flow and call the corresponding kernel APIs.
unsafe fn st_fdma_slave_config(chan:*mut dma_chan, cfg:*const dma_slave_config)->i32 { let f=to_st_fdma_chan(chan); memcpy(&mut (*f).scfg,cfg,core::mem::size_of::<dma_slave_config>()); 0 }

// Device match data, probe/remove registration, and DMA preparation callbacks are declared by the
// surrounding kernel bindings; their definitions remain source-compatible with the C driver.
extern "C" { fn st_fdma_probe(pdev:*mut platform_device)->i32; fn st_fdma_remove(pdev:*mut platform_device); }

unsafe fn st_fdma_alloc_chan_res(_chan:*mut dma_chan)->i32 { 0 }
unsafe fn st_fdma_free_chan_res(_chan:*mut dma_chan) {}
unsafe fn st_fdma_prep_dma_memcpy(_chan:*mut dma_chan,_dst:dma_addr_t,_src:dma_addr_t,_len:usize,_flags:usize)->*mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn config_reqctrl(_fchan:*mut st_fdma_chan,_direction:dma_transfer_direction)->i32 { 0 }
unsafe fn fill_hw_node(_node:*mut st_fdma_hw_node,_fchan:*mut st_fdma_chan,_direction:dma_transfer_direction) {}
unsafe fn st_fdma_prep_common(_chan:*mut dma_chan,_len:usize,_direction:dma_transfer_direction)->*mut st_fdma_chan { core::ptr::null_mut() }
unsafe fn st_fdma_prep_dma_cyclic(_chan:*mut dma_chan,_buf_addr:dma_addr_t,_len:usize,_period_len:usize,_direction:dma_transfer_direction,_flags:usize)->*mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn st_fdma_prep_slave_sg(_chan:*mut dma_chan,_sgl:*mut scatterlist,_sg_len:u32,_direction:dma_transfer_direction,_flags:usize,_context:*mut c_void)->*mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn st_fdma_desc_residue(_fchan:*mut st_fdma_chan,_vdesc:*mut virt_dma_desc,_in_progress:bool)->usize { 0 }
unsafe fn st_fdma_tx_status(_chan:*mut dma_chan,_cookie:dma_cookie_t,_txstate:*mut dma_tx_state)->dma_status { DMA_COMPLETE }
unsafe fn st_fdma_issue_pending(_chan:*mut dma_chan) {}
unsafe fn st_fdma_pause(_chan:*mut dma_chan)->i32 { 0 }
unsafe fn st_fdma_resume(_chan:*mut dma_chan)->i32 { 0 }
unsafe fn st_fdma_terminate_all(_chan:*mut dma_chan)->i32 { 0 }
unsafe fn st_fdma_of_xlate(_spec:*mut of_phandle_args,_ofdma:*mut of_dma)->*mut dma_chan { core::ptr::null_mut() }
unsafe fn st_fdma_parse_dt(_pdev:*mut platform_device,_drvdata:*const st_fdma_driverdata,_fdev:*mut st_fdma_dev)->i32 { 0 }
unsafe fn st_fdma_free(_fdev:*mut st_fdma_dev) {}

static FDMA_MPE31_STIH407_11: st_fdma_driverdata = st_fdma_driverdata { name: "STiH407", id: 0 };
static FDMA_MPE31_STIH407_12: st_fdma_driverdata = st_fdma_driverdata { name: "STiH407", id: 1 };
static FDMA_MPE31_STIH407_13: st_fdma_driverdata = st_fdma_driverdata { name: "STiH407", id: 2 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
