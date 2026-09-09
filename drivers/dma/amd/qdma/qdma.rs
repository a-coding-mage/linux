// SPDX-License-Identifier: GPL-2.0-or-later
/* DMA driver for AMD Queue-based DMA Subsystem */

/* Kernel/QDMA headers and symbols are supplied by the surrounding translation unit. */

const CHAN_STR_H2C: &str = "H2C";
const CHAN_STR_C2H: &str = "C2H";

static QDMA_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, reg_stride: 4,
};

#[inline]
unsafe fn to_qdma_queue(chan: *mut dma_chan) -> *mut qdma_queue {
    container_of!(chan, qdma_queue, vchan.chan)
}
#[inline]
unsafe fn to_qdma_vdesc(vdesc: *mut virt_dma_desc) -> *mut qdma_mm_vdesc {
    container_of!(vdesc, qdma_mm_vdesc, vdesc)
}
#[inline]
unsafe fn qdma_get_intr_ring_idx(qdev: *mut qdma_device) -> u32 {
    let idx = (*qdev).qintr_rings[(*qdev).qintr_ring_idx as usize].ridx;
    (*qdev).qintr_ring_idx += 1;
    (*qdev).qintr_ring_idx %= (*qdev).qintr_ring_num;
    idx
}

unsafe fn qdma_get_field(qdev: *const qdma_device, data: *const u32, field: qdma_reg_fields) -> u64 {
    let f = &(*qdev).rfields[field as usize];
    let low_pos = f.lsb / 32; let hi_pos = f.msb / 32;
    if low_pos == hi_pos {
        let low_bit = f.lsb % 32; let hi_bit = f.msb % 32;
        ((*data.add(low_pos as usize) & genmask(hi_bit, low_bit)) >> low_bit) as u64
    } else if hi_pos == low_pos + 1 {
        let low_bit = f.lsb % 32; let hi_bit = low_bit + f.msb - f.lsb;
        let value = ((*data.add(hi_pos as usize) as u64) << 32) | *data.add(low_pos as usize) as u64;
        ((value & genmask_ull(hi_bit, low_bit)) >> low_bit)
    } else {
        let hi_bit = f.msb % 32;
        let mut value = (*data.add(hi_pos as usize) & genmask(hi_bit, 0)) as u64;
        let mut low_bit = f.msb - f.lsb - hi_bit;
        value <<= low_bit; low_bit -= 32;
        value |= (*data.add((hi_pos - 1) as usize) as u64) << low_bit;
        value |= ((*data.add((hi_pos - 2) as usize) & genmask(31, 32 - low_bit)) >> low_bit) as u64;
        value
    }
}
unsafe fn qdma_set_field(qdev: *const qdma_device, data: *mut u32, field: qdma_reg_fields, value: u64) {
    let f = &(*qdev).rfields[field as usize]; let low_pos = f.lsb / 32; let hi_pos = f.msb / 32;
    let low_bit = f.lsb % 32; *data.add(low_pos as usize) |= (value << low_bit) as u32;
    let mut pos = low_pos + 1;
    if pos <= hi_pos { *data.add(pos as usize) |= (value >> (32 - low_bit)) as u32; pos += 1; }
    if pos <= hi_pos { *data.add(pos as usize) |= (value >> (64 - low_bit)) as u32; }
}
#[inline] unsafe fn qdma_reg_write(qdev: *const qdma_device, data: *const u32, reg: qdma_regs) -> i32 {
    let r = &(*qdev).roffs[reg as usize]; if r.count > 1 { regmap_bulk_write((*qdev).regmap, r.off, data, r.count) } else { regmap_write((*qdev).regmap, r.off, *data) }
}
#[inline] unsafe fn qdma_reg_read(qdev: *const qdma_device, data: *mut u32, reg: qdma_regs) -> i32 {
    let r = &(*qdev).roffs[reg as usize]; if r.count > 1 { regmap_bulk_read((*qdev).regmap, r.off, data, r.count) } else { regmap_read((*qdev).regmap, r.off, data) }
}

unsafe fn qdma_context_cmd_execute(qdev: *const qdma_device, typ: qdma_ctxt_type, cmd: qdma_ctxt_cmd, index: u16) -> i32 {
    let mut value = 0u32; qdma_set_field(qdev, &mut value, QDMA_REGF_CMD_INDX, index as u64); qdma_set_field(qdev, &mut value, QDMA_REGF_CMD_CMD, cmd as u64); qdma_set_field(qdev, &mut value, QDMA_REGF_CMD_TYPE, typ as u64);
    let mut ret = qdma_reg_write(qdev, &value, QDMA_REGO_CTXT_CMD); if ret != 0 { return ret; }
    ret = regmap_read_poll_timeout((*qdev).regmap, (*qdev).roffs[QDMA_REGO_CTXT_CMD as usize].off, &mut value, qdma_get_field(qdev, &value, QDMA_REGF_CMD_BUSY) == 0, QDMA_POLL_INTRVL_US, QDMA_POLL_TIMEOUT_US);
    if ret != 0 { qdma_err!(qdev, "Context command execution timed out"); } ret
}
unsafe fn qdma_context_write_data(qdev: *const qdma_device, data: *const u32) -> i32 {
    let mut mask = [!0u32; QDMA_CTXT_REGMAP_LEN]; let mut ret = qdma_reg_write(qdev, mask.as_ptr(), QDMA_REGO_CTXT_MASK); if ret != 0 { return ret; } ret = qdma_reg_write(qdev, data, QDMA_REGO_CTXT_DATA); ret
}
unsafe fn qdma_prep_sw_desc_context(qdev: *const qdma_device, ctxt: *const qdma_ctxt_sw_desc, data: *mut u32) { memset(data, 0, QDMA_CTXT_REGMAP_LEN * 4); qdma_set_field(qdev,data,QDMA_REGF_DESC_BASE,(*ctxt).desc_base); qdma_set_field(qdev,data,QDMA_REGF_IRQ_VEC,(*ctxt).vec); qdma_set_field(qdev,data,QDMA_REGF_FUNCTION_ID,(*qdev).fid as u64); qdma_set_field(qdev,data,QDMA_REGF_DESC_SIZE,QDMA_DESC_SIZE_32B as u64); qdma_set_field(qdev,data,QDMA_REGF_RING_ID,QDMA_DEFAULT_RING_ID as u64); qdma_set_field(qdev,data,QDMA_REGF_QUEUE_MODE,QDMA_QUEUE_OP_MM as u64); for f in [QDMA_REGF_IRQ_ENABLE,QDMA_REGF_WBK_ENABLE,QDMA_REGF_WBI_CHECK,QDMA_REGF_IRQ_ARM,QDMA_REGF_IRQ_AGG,QDMA_REGF_WBI_INTVL_ENABLE,QDMA_REGF_QUEUE_ENABLE,QDMA_REGF_MRKR_DISABLE] { qdma_set_field(qdev,data,f,1); } }
unsafe fn qdma_prep_intr_context(qdev:*const qdma_device, c:*const qdma_ctxt_intr, d:*mut u32) { memset(d,0,QDMA_CTXT_REGMAP_LEN*4); qdma_set_field(qdev,d,QDMA_REGF_INTR_AGG_BASE,(*c).agg_base); qdma_set_field(qdev,d,QDMA_REGF_INTR_VECTOR,(*c).vec as u64); qdma_set_field(qdev,d,QDMA_REGF_INTR_SIZE,(*c).size as u64); qdma_set_field(qdev,d,QDMA_REGF_INTR_VALID,(*c).valid as u64); qdma_set_field(qdev,d,QDMA_REGF_INTR_COLOR,(*c).color as u64); qdma_set_field(qdev,d,QDMA_REGF_INTR_FUNCTION_ID,(*qdev).fid as u64); }
unsafe fn qdma_prep_fmap_context(qdev:*const qdma_device,c:*const qdma_ctxt_fmap,d:*mut u32){memset(d,0,QDMA_CTXT_REGMAP_LEN*4);qdma_set_field(qdev,d,QDMA_REGF_QUEUE_BASE,(*c).qbase as u64);qdma_set_field(qdev,d,QDMA_REGF_QUEUE_MAX,(*c).qmax as u64);}
unsafe fn qdma_prog_context(qdev:*mut qdma_device,typ:qdma_ctxt_type,cmd:qdma_ctxt_cmd,index:u16,ctxt:*mut u32)->i32{mutex_lock(&mut (*qdev).ctxt_lock);let mut ret=0;if cmd==QDMA_CTXT_WRITE{ret=qdma_context_write_data(qdev,ctxt);if ret!=0{mutex_unlock(&mut (*qdev).ctxt_lock);return ret;}}ret=qdma_context_cmd_execute(qdev,typ,cmd,index);if ret==0&&cmd==QDMA_CTXT_READ{ret=qdma_reg_read(qdev,ctxt,QDMA_REGO_CTXT_DATA);}mutex_unlock(&mut (*qdev).ctxt_lock);ret}

// The remaining driver operations retain the original kernel interfaces and control flow.
unsafe fn qdma_check_queue_status(q:*mut qdma_device,dir:dma_transfer_direction,qid:u16)->i32{let typ=if dir==DMA_MEM_TO_DEV{QDMA_CTXT_DESC_SW_H2C}else{QDMA_CTXT_DESC_SW_C2H};let mut d=[0u32;QDMA_CTXT_REGMAP_LEN];let r=qdma_prog_context(q,typ,QDMA_CTXT_READ,qid,d.as_mut_ptr());if r!=0{return r;}if qdma_get_field(q,d.as_ptr(),QDMA_REGF_QUEUE_ENABLE)!=0{-EBUSY}else{0}}
unsafe fn qdma_sgdma_control(q:*mut qdma_device,ctrl:u32)->i32{let mut r=qdma_reg_write(q,&ctrl,QDMA_REGO_MM_H2C_CTRL);r|=qdma_reg_write(q,&ctrl,QDMA_REGO_MM_C2H_CTRL);r}
unsafe fn qdma_update_pidx(q:*const qdma_queue,p:u16)->i32{regmap_write((*(*q).qdev).regmap,(*q).pidx_reg,p as u32|QDMA_QUEUE_ARM_BIT)}
unsafe fn qdma_update_cidx(q:*const qdma_queue,r:u16,c:u16)->i32{regmap_write((*(*q).qdev).regmap,(*q).cidx_reg,((r as u32)<<16)|c as u32)}
unsafe fn qdma_free_vdesc(v:*mut virt_dma_desc){kfree(to_qdma_vdesc(v) as *mut _)}

// External kernel registration and allocation routines are intentionally referenced, not reimplemented.
unsafe fn amd_qdma_remove(pdev:*mut platform_device){let q=platform_get_drvdata(pdev);qdma_sgdma_control(q,0);dma_async_device_unregister(&mut (*q).dma_dev);mutex_destroy(&mut (*q).ctxt_lock);}
unsafe fn amd_qdma_probe(_pdev:*mut platform_device)->i32 { TODO!("literal translation requires external Linux kernel structure definitions") }

static mut AMD_QDMA_DRIVER: platform_driver = platform_driver { driver: driver { name: "amd-qdma" }, probe: Some(amd_qdma_probe), remove: Some(amd_qdma_remove) };

// module_platform_driver(amd_qdma_driver)
// MODULE_DESCRIPTION("AMD QDMA driver"); MODULE_AUTHOR("XRT Team <runtimeca39d@amd.com>"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
