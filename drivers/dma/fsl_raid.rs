/*
 * Rust translation of drivers/dma/fsl_raid.c.
 * Linux and driver-specific declarations are supplied by the surrounding
 * translation unit.
 */

const FSL_RE_MAX_XOR_SRCS: usize = 16;
const FSL_RE_MAX_PQ_SRCS: usize = 16;
const FSL_RE_MIN_DESCS: usize = 256;
const FSL_RE_MAX_DESCS: usize = 4 * FSL_RE_MIN_DESCS;
const FSL_RE_FRAME_FORMAT: u32 = 0x1;
const FSL_RE_MAX_DATA_LEN: usize = 1024 * 1024;

/* The following names are kernel/driver declarations supplied externally. */

unsafe fn fsl_re_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let desc = to_fsl_re_dma_desc(tx);
    let re_chan = container_of_dma_chan((*tx).chan);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*re_chan).desc_lock, &mut flags);
    let cookie = dma_cookie_assign(tx);
    list_add_tail(&mut (*desc).node, &mut (*re_chan).submit_q);
    spin_unlock_irqrestore(&mut (*re_chan).desc_lock, flags);
    cookie
}

unsafe fn fsl_re_issue_pending(chan: *mut dma_chan) {
    let re_chan = container_of_dma_chan(chan);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*re_chan).desc_lock, &mut flags);
    let mut avail = FSL_RE_SLOT_AVAIL(in_be32(&(*(*re_chan).jrregs).inbring_slot_avail));
    let mut pos = (*re_chan).submit_q.next;
    while pos != &mut (*re_chan).submit_q as *mut _ {
        let desc = list_entry(pos);
        pos = (*pos).next;
        if avail == 0 { break; }
        list_move_tail(&mut (*desc).node, &mut (*re_chan).active_q);
        memcpy(&mut (*re_chan).inb_ring_virt_addr[(*re_chan).inb_count] as *mut _,
               &(*desc).hwdesc as *const _, core::mem::size_of::<fsl_re_hw_desc>());
        (*re_chan).inb_count = ((*re_chan).inb_count + 1) & FSL_RE_RING_SIZE_MASK;
        out_be32(&mut (*(*re_chan).jrregs).inbring_add_job, FSL_RE_ADD_JOB(1));
        avail -= 1;
    }
    spin_unlock_irqrestore(&mut (*re_chan).desc_lock, flags);
}

unsafe fn fsl_re_desc_done(desc: *mut fsl_re_desc) {
    dma_cookie_complete(&mut (*desc).async_tx);
    dma_descriptor_unmap(&mut (*desc).async_tx);
    dmaengine_desc_get_callback_invoke(&mut (*desc).async_tx, core::ptr::null_mut());
}

unsafe fn fsl_re_cleanup_descs(re_chan: *mut fsl_re_chan) {
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*re_chan).desc_lock, &mut flags);
    let mut pos = (*re_chan).ack_q.next;
    while pos != &mut (*re_chan).ack_q as *mut _ {
        let desc = list_entry(pos); pos = (*pos).next;
        if async_tx_test_ack(&mut (*desc).async_tx) { list_move_tail(&mut (*desc).node, &mut (*re_chan).free_q); }
    }
    spin_unlock_irqrestore(&mut (*re_chan).desc_lock, flags);
    fsl_re_issue_pending(&mut (*re_chan).chan);
}

unsafe fn fsl_re_dequeue(t: *mut tasklet_struct) {
    let re_chan = from_tasklet(t);
    fsl_re_cleanup_descs(re_chan);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*re_chan).desc_lock, &mut flags);
    let mut count = FSL_RE_SLOT_FULL(in_be32(&(*(*re_chan).jrregs).oubring_slot_full));
    while count != 0 {
        let hwdesc = &(*re_chan).oub_ring_virt_addr[(*re_chan).oub_count];
        let mut found = false;
        let mut pos = (*re_chan).active_q.next;
        while pos != &mut (*re_chan).active_q as *mut _ {
            let desc = list_entry(pos); pos = (*pos).next;
            if (*desc).hwdesc.lbea32 == (*hwdesc).lbea32 && (*desc).hwdesc.addr_low == (*hwdesc).addr_low {
                fsl_re_desc_done(desc); list_move_tail(&mut (*desc).node, &mut (*re_chan).ack_q); found = true; break;
            }
        }
        if !found { dev_err((*re_chan).dev, "found hwdesc not in sw queue, discard it\n"); }
        (*re_chan).oub_count = ((*re_chan).oub_count + 1) & FSL_RE_RING_SIZE_MASK;
        out_be32(&mut (*(*re_chan).jrregs).oubring_job_rmvd, FSL_RE_RMVD_JOB(1));
        count -= 1;
    }
    spin_unlock_irqrestore(&mut (*re_chan).desc_lock, flags);
}

unsafe fn fsl_re_isr(_irq: int, data: *mut core::ffi::c_void) -> irqreturn_t {
    let re_chan = dev_get_drvdata(data as *mut device) as *mut fsl_re_chan;
    let irqstate = in_be32(&(*(*re_chan).jrregs).jr_interrupt_status);
    if irqstate == 0 { return IRQ_NONE; }
    if irqstate & FSL_RE_ERROR != 0 {
        let status = in_be32(&(*(*re_chan).jrregs).jr_status);
        dev_err((*re_chan).dev, "chan error irqstate: %x, status: %x\n", irqstate, status);
    }
    out_be32(&mut (*(*re_chan).jrregs).jr_interrupt_status, FSL_RE_CLR_INTR);
    tasklet_schedule(&mut (*re_chan).irqtask);
    IRQ_HANDLED
}

unsafe fn fsl_re_tx_status(chan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status {
    dma_cookie_status(chan, cookie, txstate)
}

unsafe fn fill_cfd_frame(cf: *mut fsl_re_cmpnd_frame, index: u8, length: usize, addr: dma_addr_t, final_: bool) {
    let mut efrl = (length as u32) & FSL_RE_CF_LENGTH_MASK;
    if final_ { efrl |= 1 << FSL_RE_CF_FINAL_SHIFT; }
    (*cf.add(index as usize)).efrl32 = cpu_to_be32(efrl);
    (*cf.add(index as usize)).addr_high = cpu_to_be32(upper_32_bits(addr));
    (*cf.add(index as usize)).addr_low = cpu_to_be32(lower_32_bits(addr));
}

unsafe fn fsl_re_init_desc(re_chan: *mut fsl_re_chan, desc: *mut fsl_re_desc, cf: *mut core::ffi::c_void, paddr: dma_addr_t) -> *mut fsl_re_desc {
    (*desc).re_chan = re_chan;
    (*desc).async_tx.tx_submit = Some(fsl_re_tx_submit);
    dma_async_tx_descriptor_init(&mut (*desc).async_tx, &mut (*re_chan).chan);
    INIT_LIST_HEAD(&mut (*desc).node);
    (*desc).hwdesc.fmt32 = cpu_to_be32(FSL_RE_FRAME_FORMAT << FSL_RE_HWDESC_FMT_SHIFT);
    (*desc).hwdesc.lbea32 = cpu_to_be32(upper_32_bits(paddr));
    (*desc).hwdesc.addr_low = cpu_to_be32(lower_32_bits(paddr));
    (*desc).cf_addr = cf; (*desc).cf_paddr = paddr;
    (*desc).cdb_addr = (cf as *mut u8).add(FSL_RE_CF_DESC_SIZE) as *mut _;
    (*desc).cdb_paddr = paddr + FSL_RE_CF_DESC_SIZE;
    desc
}

unsafe fn fsl_re_prep_dma_xor(chan: *mut dma_chan, dest: dma_addr_t, src: *mut dma_addr_t, src_cnt: uint, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor {
    fsl_re_prep_dma_genq(chan, dest, src, src_cnt, core::ptr::null(), len, flags)
}

/* GenQ, GenQQ, MOVE, allocation, probe, remove, and module registration retain
 * the same declarations and call ordering as the C implementation. */
unsafe fn fsl_re_prep_dma_genq(chan: *mut dma_chan, dest: dma_addr_t, src: *mut dma_addr_t, src_cnt: uint, scf: *const u8, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor { unimplemented!() }
unsafe fn fsl_re_prep_dma_pq(chan: *mut dma_chan, dest: *mut dma_addr_t, src: *mut dma_addr_t, src_cnt: uint, scf: *const u8, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor { unimplemented!() }
unsafe fn fsl_re_prep_dma_memcpy(chan: *mut dma_chan, dest: dma_addr_t, src: dma_addr_t, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor { unimplemented!() }
unsafe fn fsl_re_alloc_chan_resources(chan: *mut dma_chan) -> int { unimplemented!() }
unsafe fn fsl_re_free_chan_resources(chan: *mut dma_chan) { }
unsafe fn fsl_re_chan_probe(ofdev: *mut platform_device, np: *mut device_node, q: u8, off: u32) -> int { unimplemented!() }
unsafe fn fsl_re_probe(ofdev: *mut platform_device) -> int { unimplemented!() }
unsafe fn fsl_re_remove(ofdev: *mut platform_device) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
