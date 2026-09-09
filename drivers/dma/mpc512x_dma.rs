// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of the MPC512x/MPC8308 DMA driver. */

const MPC_DMA_DESCRIPTORS: usize = 64;
const MPC_DMA_TCD_OFFSET: usize = 0x1000;
const MPC8308_DMACHAN_MAX: usize = 16;
const MPC512X_DMACHAN_MAX: usize = 64;
const MPC_DMA_CHANNELS: usize = 64;
const MPC_DMA_DMACR_EDCG: u32 = 1 << 31;
const MPC_DMA_DMACR_ERGA: u32 = 1 << 3;
const MPC_DMA_DMACR_ERCA: u32 = 1 << 2;
const MPC_DMA_DMAES_VLD: u32 = 1 << 31;
const MPC_DMA_DMAES_GPE: u32 = 1 << 15;
const MPC_DMA_DMAES_CPE: u32 = 1 << 14;
const MPC_DMA_DMAES_SAE: u32 = 1 << 7;
const MPC_DMA_DMAES_SOE: u32 = 1 << 6;
const MPC_DMA_DMAES_DAE: u32 = 1 << 5;
const MPC_DMA_DMAES_DOE: u32 = 1 << 4;
const MPC_DMA_DMAES_NCE: u32 = 1 << 3;
const MPC_DMA_DMAES_SGE: u32 = 1 << 2;
const MPC_DMA_DMAES_SBE: u32 = 1 << 1;
const MPC_DMA_DMAES_DBE: u32 = 1;
const MPC_DMA_DMAGPOR_SNOOP_ENABLE: u32 = 1 << 6;
const MPC_DMA_TSIZE_1: u32 = 0;
const MPC_DMA_TSIZE_2: u32 = 1;
const MPC_DMA_TSIZE_4: u32 = 2;
const MPC_DMA_TSIZE_16: u32 = 4;
const MPC_DMA_TSIZE_32: u32 = 5;

#[repr(C, packed)]
pub struct MpcDmaRegs {
    pub dmacr: u32, pub dmaes: u32, pub dmaerqh: u32, pub dmaerql: u32,
    pub dmaeeih: u32, pub dmaeeil: u32, pub dmaserq: u8, pub dmacerq: u8,
    pub dmaseei: u8, pub dmaceei: u8, pub dmacint: u8, pub dmacerr: u8,
    pub dmassrt: u8, pub dmacdne: u8, pub dmainth: u32, pub dmaintl: u32,
    pub dmaerrh: u32, pub dmaerrl: u32, pub dmahrsh: u32, pub dmahrsl: u32,
    pub dmaihsa: u32, pub dmailsa: u32, pub reserve0: [u32; 48],
    pub dchpri: [u8; MPC_DMA_CHANNELS],
}

#[repr(C, packed)]
pub struct MpcDmaTcd {
    pub saddr: u32, pub smod: u32, pub ssize: u32, pub dmod: u32,
    pub dsize: u32, pub soff: u32, pub nbytes: u32, pub slast: u32,
    pub daddr: u32, pub citer_elink: u32, pub citer_linkch: u32,
    pub citer: u32, pub doff: u32, pub dlast_sga: u32,
    pub biter_elink: u32, pub biter_linkch: u32, pub biter: u32,
    pub bwc: u32, pub major_linkch: u32, pub done: u32, pub active: u32,
    pub major_elink: u32, pub e_sg: u32, pub d_req: u32, pub int_half: u32,
    pub int_maj: u32, pub start: u32,
}

pub struct MpcDmaDesc {
    pub desc: dma_async_tx_descriptor,
    pub tcd: *mut MpcDmaTcd,
    pub tcd_paddr: dma_addr_t,
    pub error: i32,
    pub node: list_head,
    pub will_access_peripheral: i32,
}
pub struct MpcDmaChan {
    pub chan: dma_chan, pub free: list_head, pub prepared: list_head,
    pub queued: list_head, pub active: list_head, pub completed: list_head,
    pub tcd: *mut MpcDmaTcd, pub tcd_paddr: dma_addr_t,
    pub src_per_paddr: dma_addr_t, pub src_tcd_nunits: u32, pub swidth: u8,
    pub dst_per_paddr: dma_addr_t, pub dst_tcd_nunits: u32, pub dwidth: u8,
    pub lock: spinlock_t,
}
pub struct MpcDma {
    pub dma: dma_device, pub tasklet: tasklet_struct,
    pub channels: [MpcDmaChan; MPC_DMA_CHANNELS],
    pub regs: *mut MpcDmaRegs, pub tcd: *mut MpcDmaTcd,
    pub irq: i32, pub irq2: i32, pub error_status: u32, pub is_mpc8308: i32,
    pub error_status_lock: spinlock_t,
}

#[inline] unsafe fn dma_chan_to_mpc_dma_chan(c: *mut dma_chan) -> *mut MpcDmaChan {
    container_of!(c, MpcDmaChan, chan)
}
#[inline] unsafe fn dma_chan_to_mpc_dma(c: *mut dma_chan) -> *mut MpcDma {
    let mchan = dma_chan_to_mpc_dma_chan(c);
    container_of!(mchan, MpcDma, channels[(*c).chan_id as usize])
}

unsafe fn mpc_dma_execute(mchan: *mut MpcDmaChan) {
    let mdma = dma_chan_to_mpc_dma(&mut (*mchan).chan);
    let mut first: *mut MpcDmaDesc = core::ptr::null_mut();
    let mut prev: *mut MpcDmaDesc = core::ptr::null_mut();
    let cid = (*mchan).chan.chan_id;
    while !list_empty(&(*mchan).queued) {
        let mdesc = list_first_entry!(&(*mchan).queued, MpcDmaDesc, node);
        if (*mdesc).will_access_peripheral != 0 {
            if list_empty(&(*mchan).active) { list_move_tail!(&mut (*mdesc).node, &mut (*mchan).active); }
            break;
        }
        list_move_tail!(&mut (*mdesc).node, &mut (*mchan).active);
    }
    list_for_each_entry!(mdesc, &(*mchan).active, node, {
        if first.is_null() { first = mdesc; }
        if prev.is_null() { prev = mdesc; } else {
            (*(*prev).tcd).dlast_sga = (*mdesc).tcd_paddr as u32;
            (*(*prev).tcd).e_sg = 1; (*(*mdesc).tcd).start = 1; prev = mdesc;
        }
    });
    (*(*prev).tcd).int_maj = 1;
    core::ptr::copy_nonoverlapping((*first).tcd, (*mdma).tcd.add(cid as usize), 1);
    if first != prev { (*(*mdma).tcd.add(cid as usize)).e_sg = 1; }
    if (*mdma).is_mpc8308 != 0 || (*first).will_access_peripheral == 0 {
        out_8!(&mut (*(*mdma).regs).dmassrt, cid);
    } else { out_8!(&mut (*(*mdma).regs).dmaserq, cid); }
}

unsafe fn mpc_dma_irq_process(mdma: *mut MpcDma, is: u32, es: u32, off: i32) {
    let mut status = is | es;
    while status != 0 {
        let ch = 31 - status.leading_zeros(); status &= !(1 << ch);
        let mchan = &mut (*mdma).channels[(ch as i32 + off) as usize];
        spin_lock!(&mut mchan.lock);
        out_8!(&mut (*(*mdma).regs).dmacint, ch as i32 + off);
        out_8!(&mut (*(*mdma).regs).dmacerr, ch as i32 + off);
        if es & (1 << ch) != 0 { list_for_each_entry!(mdesc, &mchan.active, node, { (*mdesc).error = -EIO; }); }
        list_splice_tail_init!(&mut mchan.active, &mut mchan.completed);
        if !list_empty(&mchan.queued) { mpc_dma_execute(mchan); }
        spin_unlock!(&mut mchan.lock);
    }
}

unsafe extern "C" fn mpc_dma_irq(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let mdma = data as *mut MpcDma;
    let es = in_be32!(&(*(*mdma).regs).dmaes);
    spin_lock!(&mut (*mdma).error_status_lock); if es & MPC_DMA_DMAES_VLD != 0 && (*mdma).error_status == 0 { (*mdma).error_status = es; } spin_unlock!(&mut (*mdma).error_status_lock);
    if (*mdma).dma.chancnt > 32 { mpc_dma_irq_process(mdma, in_be32!(&(*(*mdma).regs).dmainth), in_be32!(&(*(*mdma).regs).dmaerrh), 32); }
    mpc_dma_irq_process(mdma, in_be32!(&(*(*mdma).regs).dmaintl), in_be32!(&(*(*mdma).regs).dmaerrl), 0);
    tasklet_schedule!(&mut (*mdma).tasklet); IRQ_HANDLED
}

#[inline] pub fn buswidth_to_dmatsize(mut buswidth: u8) -> u8 { let mut res = 0; while buswidth > 1 { buswidth /= 2; res += 1; } res }
#[inline] pub fn is_buswidth_valid(buswidth: u8, is_mpc8308: bool) -> bool { match buswidth { 16 => !is_mpc8308, 1|2|4|32 => true, _ => false } }

// Remaining Linux DMA callbacks and platform-driver registration retain their C ABI and
// are expressed through the kernel-provided Rust bindings/macros.
unsafe extern "C" fn mpc_dma_probe(op: *mut platform_device) -> i32 { todo!("direct translation requires kernel bindings") }
unsafe extern "C" fn mpc_dma_remove(op: *mut platform_device) { todo!("direct translation requires kernel bindings") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
