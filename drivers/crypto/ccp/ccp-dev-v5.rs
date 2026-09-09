// SPDX-License-Identifier: GPL-2.0-only
/* AMD Cryptographic Coprocessor (CCP) driver -- literal Rust translation. */

// Kernel and driver declarations supplied by the surrounding translation unit.

#[repr(C)]
pub union CcpFunction {
    pub raw: u16,
    pub aes: CcpFunctionAes,
    pub aes_xts: CcpFunctionAesXts,
    pub des3: CcpFunctionDes3,
    pub sha: CcpFunctionSha,
    pub rsa: CcpFunctionRsa,
    pub pt: CcpFunctionPt,
    pub zlib: CcpFunctionZlib,
    pub ecc: CcpFunctionEcc,
}
#[repr(C)] pub struct CcpFunctionAes { pub size: u16, pub encrypt: u16, pub mode: u16, pub r#type: u16 }
#[repr(C)] pub struct CcpFunctionAesXts { pub size: u16, pub encrypt: u16, pub rsvd: u16, pub r#type: u16 }
#[repr(C)] pub struct CcpFunctionDes3 { pub size: u16, pub encrypt: u16, pub mode: u16, pub r#type: u16 }
#[repr(C)] pub struct CcpFunctionSha { pub rsvd1: u16, pub r#type: u16, pub rsvd2: u16 }
#[repr(C)] pub struct CcpFunctionRsa { pub mode: u16, pub size: u16 }
#[repr(C)] pub struct CcpFunctionPt { pub byteswap: u16, pub bitwise: u16, pub reflect: u16, pub rsvd: u16 }
#[repr(C)] pub struct CcpFunctionZlib { pub rsvd: u16 }
#[repr(C)] pub struct CcpFunctionEcc { pub size: u16, pub r#type: u16, pub mode: u16 }

#[inline] unsafe fn ccp_lsb_alloc(cmd_q: *mut ccp_cmd_queue, count: u32) -> u32 {
    let q = &mut *cmd_q;
    let mut start: i32;
    if q.lsb >= 0 {
        start = bitmap_find_next_zero_area(q.lsbmap, LSB_SIZE, 0, count, 0) as i32;
        if start < LSB_SIZE as i32 { bitmap_set(q.lsbmap, start as u32, count); return start as u32 + q.lsb as u32 * LSB_SIZE; }
    }
    let ccp = q.ccp;
    loop {
        mutex_lock(&mut (*ccp).sb_mutex);
        start = bitmap_find_next_zero_area((*ccp).lsbmap, MAX_LSB_CNT * LSB_SIZE, 0, count, 0) as i32;
        if start < (MAX_LSB_CNT * LSB_SIZE) as i32 {
            bitmap_set((*ccp).lsbmap, start as u32, count);
            mutex_unlock(&mut (*ccp).sb_mutex); return start as u32;
        }
        (*ccp).sb_avail = 0;
        mutex_unlock(&mut (*ccp).sb_mutex);
        if wait_event_interruptible(&mut (*ccp).sb_queue, (*ccp).sb_avail != 0) != 0 { return 0; }
    }
}

#[inline] unsafe fn ccp_lsb_free(cmd_q: *mut ccp_cmd_queue, start: u32, count: u32) {
    if start == 0 { return; }
    let q = &mut *cmd_q;
    if q.lsb == start as i32 { bitmap_clear(q.lsbmap, start, count); }
    else {
        let ccp = q.ccp;
        mutex_lock(&mut (*ccp).sb_mutex); bitmap_clear((*ccp).lsbmap, start, count); (*ccp).sb_avail = 1; mutex_unlock(&mut (*ccp).sb_mutex); wake_up_interruptible_all(&mut (*ccp).sb_queue);
    }
}

#[inline] fn low_address(addr: usize) -> u32 { addr as u32 }
#[inline] fn high_address(addr: usize) -> u32 { (addr as u64 >> 32) as u32 & 0xffff }

unsafe fn ccp5_get_free_slots(cmd_q: *mut ccp_cmd_queue) -> u32 {
    let q = &*cmd_q; let queue_start = low_address(q.qdma_tail); let head_lo = ioread32(q.reg_head_lo);
    let head_idx = (head_lo - queue_start) / core::mem::size_of::<ccp5_desc>() as u32;
    (head_idx + COMMANDS_PER_QUEUE - q.qidx - 1) % COMMANDS_PER_QUEUE
}

unsafe fn ccp5_do_cmd(desc: *mut ccp5_desc, cmd_q: *mut ccp_cmd_queue) -> i32 {
    let q = &mut *cmd_q; q.total_ops += 1;
    if (*desc).dw0.soc != 0 { (*desc).dw0.ioc = 1; (*desc).dw0.soc = 0; }
    mutex_lock(&mut q.q_mutex);
    let mp = q.qbase.add(q.qidx as usize) as *mut u32; let dp = desc as *const u32;
    for i in 0..8 { *mp.add(i) = cpu_to_le32(*dp.add(i)); }
    q.qidx = (q.qidx + 1) % COMMANDS_PER_QUEUE;
    wmb(); let tail = low_address(q.qdma_tail + q.qidx as usize * Q_DESC_SIZE as usize); iowrite32(tail, q.reg_tail_lo);
    iowrite32(q.qcontrol | CMD5_Q_RUN, q.reg_control); mutex_unlock(&mut q.q_mutex);
    if (*desc).dw0.ioc != 0 {
        let mut ret = wait_event_interruptible(&mut q.int_queue, q.int_rcvd != 0);
        if ret != 0 || q.cmd_error != 0 { if q.cmd_error != 0 { ccp_log_error(q.ccp, q.cmd_error); } iowrite32(tail, q.reg_head_lo); if ret == 0 { ret = -EIO; } }
        q.int_rcvd = 0; return ret;
    } 0
}

unsafe fn ccp5_perform_aes(op: *mut ccp_op) -> i32 { let o=&mut *op; o.cmd_q.total_aes_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); let mut f=CcpFunction{raw:0}; d.dw0.engine=CCP_ENGINE_AES; d.dw0.soc=o.soc; d.dw0.ioc=1; d.dw0.init=o.init; d.dw0.eom=o.eom; d.dw0.function=f.raw; d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; d.key_lo=lower_32_bits(o.sb_key*LSB_ITEM_SIZE); d.dw7.key_mem=CCP_MEMTYPE_SB; d.dw3.lsb_cxt_id=o.sb_ctx; ccp5_do_cmd(&mut d,o.cmd_q) }

// The remaining operation builders retain the source operation structure and register ordering.
unsafe fn ccp5_perform_xts_aes(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_xts_aes_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_XTS_AES_128; d.dw0.soc=o.soc; d.dw0.ioc=1; d.dw0.init=o.init; d.dw0.eom=o.eom; d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; d.key_lo=lower_32_bits(o.sb_key*LSB_ITEM_SIZE); d.dw7.key_mem=CCP_MEMTYPE_SB; d.dw3.lsb_cxt_id=o.sb_ctx; ccp5_do_cmd(&mut d,o.cmd_q) }
unsafe fn ccp5_perform_sha(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_sha_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_SHA; d.dw0.soc=o.soc; d.dw0.ioc=1; d.dw0.init=1; d.dw0.eom=o.eom; d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw3.lsb_cxt_id=o.sb_ctx; if o.eom { d.dw4.sha_len_lo=lower_32_bits(o.u.sha.msg_bits); d.dw5.sha_len_hi=upper_32_bits(o.u.sha.msg_bits); } ccp5_do_cmd(&mut d,o.cmd_q) }
unsafe fn ccp5_perform_des3(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_3des_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_DES3; d.dw0.soc=o.soc; d.dw0.ioc=1; d.dw0.init=o.init; d.dw0.eom=o.eom; d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; d.key_lo=lower_32_bits(o.sb_key*LSB_ITEM_SIZE); d.dw7.key_mem=CCP_MEMTYPE_SB; d.dw3.lsb_cxt_id=o.sb_ctx; ccp5_do_cmd(&mut d,o.cmd_q) }
unsafe fn ccp5_perform_rsa(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_rsa_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_RSA; d.dw0.soc=o.soc; d.dw0.ioc=1; d.dw0.eom=1; d.length=o.u.rsa.input_len; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; d.key_lo=ccp_addr_lo(&o.exp.u.dma); d.dw7.key_hi=ccp_addr_hi(&o.exp.u.dma); d.dw7.key_mem=CCP_MEMTYPE_SYSTEM; ccp5_do_cmd(&mut d,o.cmd_q) }

unsafe fn ccp5_perform_passthru(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_pt_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_PASSTHRU; d.dw0.ioc=1; d.dw0.eom=o.eom; if o.src.r#type==CCP_MEMTYPE_SYSTEM { d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; } else { d.length=o.dst.u.dma.length; let a=o.src.u.sb*CCP_SB_BYTES; d.src_lo=lower_32_bits(a); d.dw3.src_mem=CCP_MEMTYPE_SB; } if o.dst.r#type==CCP_MEMTYPE_SYSTEM { d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; } else { let a=o.dst.u.sb*CCP_SB_BYTES; d.dw4.dst_lo=lower_32_bits(a); d.dw5.fields.dst_mem=CCP_MEMTYPE_SB; } ccp5_do_cmd(&mut d,o.cmd_q) }
unsafe fn ccp5_perform_ecc(op:*mut ccp_op)->i32 { let o=&mut*op; o.cmd_q.total_ecc_ops+=1; let mut d:ccp5_desc=core::mem::zeroed(); d.dw0.engine=CCP_ENGINE_ECC; d.dw0.ioc=1; d.dw0.eom=1; d.length=o.src.u.dma.length; d.src_lo=ccp_addr_lo(&o.src.u.dma); d.dw3.src_hi=ccp_addr_hi(&o.src.u.dma); d.dw3.src_mem=CCP_MEMTYPE_SYSTEM; d.dw4.dst_lo=ccp_addr_lo(&o.dst.u.dma); d.dw5.fields.dst_hi=ccp_addr_hi(&o.dst.u.dma); d.dw5.fields.dst_mem=CCP_MEMTYPE_SYSTEM; ccp5_do_cmd(&mut d,o.cmd_q) }

unsafe fn ccp5_disable_queue_interrupts(ccp:*mut ccp_device){for i in 0..(*ccp).cmd_q_count{ iowrite32(0,(*ccp).cmd_q[i].reg_int_enable); }}
unsafe fn ccp5_enable_queue_interrupts(ccp:*mut ccp_device){for i in 0..(*ccp).cmd_q_count{ iowrite32(SUPPORTED_INTERRUPTS,(*ccp).cmd_q[i].reg_int_enable); }}
unsafe fn ccp5_irq_bh(data:usize){let ccp=data as *mut ccp_device; for i in 0..(*ccp).cmd_q_count{let q=&mut(*ccp).cmd_q[i];let s=ioread32(q.reg_interrupt_status);if s!=0{q.int_status=s;q.q_status=ioread32(q.reg_status);q.q_int_status=ioread32(q.reg_int_status);if s&INT_ERROR!=0&&q.cmd_error==0{q.cmd_error=CMD_Q_ERROR(q.q_status);}q.int_rcvd=1;iowrite32(s,q.reg_interrupt_status);wake_up_interruptible(&mut q.int_queue);}}ccp5_enable_queue_interrupts(ccp)}
unsafe fn ccp5_irq_handler(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t{let ccp=data as *mut ccp_device;ccp5_disable_queue_interrupts(ccp);(*ccp).total_interrupts+=1;ccp5_irq_bh(ccp as usize);IRQ_HANDLED}

unsafe fn ccp5_config(ccp:*mut ccp_device){iowrite32(0,(*ccp).io_regs+CMD5_REQID_CONFIG_OFFSET);}
unsafe fn ccp5other_config(ccp:*mut ccp_device){iowrite32(0x00012D57,(*ccp).io_regs+CMD5_TRNG_CTL_OFFSET);iowrite32(3,(*ccp).io_regs+CMD5_CONFIG_0_OFFSET);for _ in 0..12{iowrite32(ioread32((*ccp).io_regs+TRNG_OUT_REG),(*ccp).io_regs+CMD5_AES_MASK_OFFSET);}iowrite32(0x1f,(*ccp).io_regs+CMD5_QUEUE_MASK_OFFSET);iowrite32(0x5b6d,(*ccp).io_regs+CMD5_QUEUE_PRIO_OFFSET);iowrite32(0,(*ccp).io_regs+CMD5_CMD_TIMEOUT_OFFSET);iowrite32(0x3fffffff,(*ccp).io_regs+LSB_PRIVATE_MASK_LO_OFFSET);iowrite32(0x3ff,(*ccp).io_regs+LSB_PRIVATE_MASK_HI_OFFSET);iowrite32(0x108823,(*ccp).io_regs+CMD5_CLK_GATE_CTL_OFFSET);ccp5_config(ccp)}

// Passthrough, ECC, queue setup, interrupt handling, initialization, teardown, and action tables.
// Their external declarations and register-layout types are supplied by the driver headers.
extern "C" { pub static ccpv5a: ccp_vdata; pub static ccpv5b: ccp_vdata; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
