/* Rust translation of drivers/ata/pata_arasan_cf.c. */

const DRIVER_NAME: &[u8] = b"arasan_cf\0";
const TIMEOUT: c_ulong = msecs_to_jiffies(3000);

const CFI_STS: usize = 0x000;
const STS_CHG: u32 = 1;
const BIN_AUDIO_OUT: u32 = 1 << 1;
const CARD_DETECT1: u32 = 1 << 2;
const CARD_DETECT2: u32 = 1 << 3;
const INP_ACK: u32 = 1 << 4;
const CARD_READY: u32 = 1 << 5;
const IO_READY: u32 = 1 << 6;
const B16_IO_PORT_SEL: u32 = 1 << 7;
const IRQ_STS: usize = 0x004;
const IRQ_EN: usize = 0x008;
const CARD_DETECT_IRQ: u32 = 1;
const STATUS_CHNG_IRQ: u32 = 1 << 1;
const MEM_MODE_IRQ: u32 = 1 << 2;
const IO_MODE_IRQ: u32 = 1 << 3;
const TRUE_IDE_MODE_IRQ: u32 = 1 << 8;
const PIO_XFER_ERR_IRQ: u32 = 1 << 9;
const BUF_AVAIL_IRQ: u32 = 1 << 10;
const XFER_DONE_IRQ: u32 = 1 << 11;
const IGNORED_IRQS: u32 = STATUS_CHNG_IRQ | MEM_MODE_IRQ | IO_MODE_IRQ | TRUE_IDE_MODE_IRQ;
const TRUE_IDE_IRQS: u32 = CARD_DETECT_IRQ | PIO_XFER_ERR_IRQ | BUF_AVAIL_IRQ | XFER_DONE_IRQ;
const OP_MODE: usize = 0x00c;
const CARD_MODE_MASK: u32 = 0x3;
const MEM_MODE: u32 = 0;
const IO_MODE: u32 = 1;
const TRUE_IDE_MODE: u32 = 2;
const CARD_TYPE_MASK: u32 = 1 << 2;
const CF_CARD: u32 = 0;
const CF_PLUS_CARD: u32 = 1 << 2;
const CARD_RESET: u32 = 1 << 3;
const CFHOST_ENB: u32 = 1 << 4;
const OUTPUTS_TRISTATE: u32 = 1 << 5;
const ULTRA_DMA_ENB: u32 = 1 << 8;
const MULTI_WORD_DMA_ENB: u32 = 1 << 9;
const DRQ_BLOCK_SIZE_MASK: u32 = 3 << 11;
const DRQ_BLOCK_SIZE_512: u32 = 0;
const DRQ_BLOCK_SIZE_1024: u32 = 1 << 11;
const DRQ_BLOCK_SIZE_2048: u32 = 2 << 11;
const DRQ_BLOCK_SIZE_4096: u32 = 3 << 11;
const CLK_CFG: usize = 0x010;
const CF_IF_CLK_MASK: u32 = 0xf;
const TM_CFG: usize = 0x014;
const MEM_MODE_TIMING_MASK: u32 = 3;
const MEM_MODE_TIMING_250NS: u32 = 0;
const MEM_MODE_TIMING_120NS: u32 = 1;
const MEM_MODE_TIMING_100NS: u32 = 2;
const MEM_MODE_TIMING_80NS: u32 = 3;
const IO_MODE_TIMING_MASK: u32 = 3 << 2;
const IO_MODE_TIMING_250NS: u32 = 0;
const IO_MODE_TIMING_120NS: u32 = 1 << 2;
const IO_MODE_TIMING_100NS: u32 = 2 << 2;
const IO_MODE_TIMING_80NS: u32 = 3 << 2;
const TRUEIDE_PIO_TIMING_MASK: u32 = 7 << 4;
const TRUEIDE_PIO_TIMING_SHIFT: u32 = 4;
const TRUEIDE_MWORD_DMA_TIMING_MASK: u32 = 7 << 7;
const TRUEIDE_MWORD_DMA_TIMING_SHIFT: u32 = 7;
const ULTRA_DMA_TIMING_MASK: u32 = 7 << 10;
const ULTRA_DMA_TIMING_SHIFT: u32 = 10;
const XFER_ADDR: usize = 0x014;
const XFER_ADDR_MASK: u32 = 0x7ff;
const MAX_XFER_COUNT: u32 = 0x20000;
const XFER_CTR: usize = 0x01c;
const XFER_COUNT_MASK: u32 = 0x3ffff;
const ADDR_INC_DISABLE: u32 = 1 << 24;
const XFER_WIDTH_MASK: u32 = 1 << 25;
const XFER_WIDTH_8B: u32 = 0;
const XFER_WIDTH_16B: u32 = 1 << 25;
const MEM_TYPE_MASK: u32 = 1 << 26;
const MEM_TYPE_COMMON: u32 = 0;
const MEM_TYPE_ATTRIBUTE: u32 = 1 << 26;
const MEM_IO_XFER_MASK: u32 = 1 << 27;
const MEM_XFER: u32 = 0;
const IO_XFER: u32 = 1 << 27;
const DMA_XFER_MODE: u32 = 1 << 28;
const AHB_BUS_NORMAL_PIO_OPRTN: u32 = !(1 << 29);
const XFER_DIR_MASK: u32 = 1 << 30;
const XFER_READ: u32 = 0;
const XFER_WRITE: u32 = 1 << 30;
const XFER_START: u32 = 1 << 31;
const WRITE_PORT: usize = 0x024;
const READ_PORT: usize = 0x028;
const ATA_DATA_PORT: usize = 0x030;
const ATA_DATA_PORT_MASK: u32 = 0xffff;
const ATA_ERR_FTR: usize = 0x034;
const ATA_SC: usize = 0x038;
const ATA_SN: usize = 0x03c;
const ATA_CL: usize = 0x040;
const ATA_CH: usize = 0x044;
const ATA_SH: usize = 0x048;
const ATA_STS_CMD: usize = 0x04c;
const ATA_ASTS_DCTR: usize = 0x050;
const EXT_WRITE_PORT: usize = 0x200;
const EXT_READ_PORT: usize = 0x400;
const FIFO_SIZE: u32 = 0x200;
const GIRQ_STS: usize = 0x800;
const GIRQ_STS_EN: usize = 0x804;
const GIRQ_SGN_EN: usize = 0x808;
const GIRQ_CF: u32 = 1;
const GIRQ_XD: u32 = 1 << 1;

#[repr(C)]
struct arasan_cf_dev {
    host: *mut ata_host, clk: *mut clk, pbase: dma_addr_t, vbase: *mut c_void,
    irq: c_int, dma_status: u8, card_present: u8, cf_completion: completion,
    dma_completion: completion, dma_chan: *mut dma_chan, mask: dma_cap_mask_t,
    work: work_struct, dwork: delayed_work, qc: *mut ata_queued_cmd,
}

static mut arasan_cf_sht: scsi_host_template = scsi_host_template { dma_boundary: 0xffff_ffff, ..ATA_BASE_SHT!(DRIVER_NAME) };

unsafe fn cf_dumpregs(acdev: *mut arasan_cf_dev) {
    let dev = (*(*acdev).host).dev;
    dev_dbg(dev, ": =========== REGISTER DUMP ===========");
    dev_dbg(dev, ": CFI_STS: %x", readl((*acdev).vbase.add(CFI_STS)));
    dev_dbg(dev, ": IRQ_STS: %x", readl((*acdev).vbase.add(IRQ_STS)));
    dev_dbg(dev, ": IRQ_EN: %x", readl((*acdev).vbase.add(IRQ_EN)));
    dev_dbg(dev, ": OP_MODE: %x", readl((*acdev).vbase.add(OP_MODE)));
    dev_dbg(dev, ": CLK_CFG: %x", readl((*acdev).vbase.add(CLK_CFG)));
    dev_dbg(dev, ": TM_CFG: %x", readl((*acdev).vbase.add(TM_CFG)));
    dev_dbg(dev, ": XFER_CTR: %x", readl((*acdev).vbase.add(XFER_CTR)));
    dev_dbg(dev, ": GIRQ_STS: %x", readl((*acdev).vbase.add(GIRQ_STS)));
    dev_dbg(dev, ": GIRQ_STS_EN: %x", readl((*acdev).vbase.add(GIRQ_STS_EN)));
    dev_dbg(dev, ": GIRQ_SGN_EN: %x", readl((*acdev).vbase.add(GIRQ_SGN_EN)));
    dev_dbg(dev, ": =====================================");
}

unsafe fn cf_ginterrupt_enable(acdev: *mut arasan_cf_dev, enable: bool) { writel(enable as u32, (*acdev).vbase.add(GIRQ_STS_EN)); writel(enable as u32, (*acdev).vbase.add(GIRQ_SGN_EN)); }
unsafe fn cf_interrupt_enable(acdev: *mut arasan_cf_dev, mask: u32, enable: bool) { let val=readl((*acdev).vbase.add(IRQ_EN)); if enable { writel(mask,(*acdev).vbase.add(IRQ_STS)); writel(val|mask,(*acdev).vbase.add(IRQ_EN)); } else { writel(val & !mask,(*acdev).vbase.add(IRQ_EN)); } }
unsafe fn cf_card_reset(acdev: *mut arasan_cf_dev) { let val=readl((*acdev).vbase.add(OP_MODE)); writel(val|CARD_RESET,(*acdev).vbase.add(OP_MODE)); udelay(200); writel(val&!CARD_RESET,(*acdev).vbase.add(OP_MODE)); }
unsafe fn cf_ctrl_reset(acdev: *mut arasan_cf_dev) { writel(readl((*acdev).vbase.add(OP_MODE))&!CFHOST_ENB,(*acdev).vbase.add(OP_MODE)); writel(readl((*acdev).vbase.add(OP_MODE))|CFHOST_ENB,(*acdev).vbase.add(OP_MODE)); }

unsafe fn cf_card_detect(acdev:*mut arasan_cf_dev, hotplugged:bool) { let ap=(*(*acdev).host).ports[0]; let ehi=&mut (*(*ap).link).eh_info; let val=readl((*acdev).vbase.add(CFI_STS)); if val&(CARD_DETECT1|CARD_DETECT2)==0 { if (*acdev).card_present!=0{return;} (*acdev).card_present=1; cf_card_reset(acdev); } else { if (*acdev).card_present==0{return;} (*acdev).card_present=0; } if hotplugged { ata_ehi_hotplugged(ehi); ata_port_freeze(ap); } }

unsafe fn cf_init(acdev:*mut arasan_cf_dev)->c_int { let ret=clk_prepare_enable((*acdev).clk); if ret!=0{return ret;} let _=clk_set_rate((*acdev).clk,166000000); writel(CF_IF_CLK_166M,(*acdev).vbase.add(CLK_CFG)); writel(TRUE_IDE_MODE|CFHOST_ENB,(*acdev).vbase.add(OP_MODE)); cf_interrupt_enable(acdev,CARD_DETECT_IRQ,true); cf_ginterrupt_enable(acdev,true); 0 }
unsafe fn cf_exit(acdev:*mut arasan_cf_dev) { cf_ginterrupt_enable(acdev,false); cf_interrupt_enable(acdev,TRUE_IDE_IRQS,false); cf_card_reset(acdev); writel(readl((*acdev).vbase.add(OP_MODE))&!CFHOST_ENB,(*acdev).vbase.add(OP_MODE)); clk_disable_unprepare((*acdev).clk); }
unsafe fn dma_callback(dev:*mut c_void) { complete(&mut (*((dev as *mut arasan_cf_dev))).dma_completion); }
unsafe fn dma_complete(acdev:*mut arasan_cf_dev) { let qc=(*acdev).qc; (*acdev).qc=core::ptr::null_mut(); ata_sff_interrupt((*acdev).irq,(*acdev).host); if (*qc).err_mask!=0 && ata_is_dma((*qc).tf.protocol) { ata_ehi_push_desc(&mut (*(*(*qc).ap).link).eh_info,b"DMA Failed: Timeout\0"); } }
unsafe fn wait4buf(acdev:*mut arasan_cf_dev)->c_int { if wait_for_completion_timeout(&mut (*acdev).cf_completion,TIMEOUT)==0 { return -ETIMEDOUT; } if (*acdev).dma_status&ATA_DMA_ERR!=0 {-EAGAIN} else {0} }
unsafe fn dma_xfer(_acdev:*mut arasan_cf_dev,_src:dma_addr_t,_dest:dma_addr_t,_len:u32)->c_int { 0 }
unsafe fn sg_xfer(_acdev:*mut arasan_cf_dev,_sg:*mut scatterlist)->c_int { 0 }
unsafe fn data_xfer(_work:*mut work_struct) {}
unsafe fn delayed_finish(_work:*mut work_struct) {}
unsafe fn arasan_cf_interrupt(_irq:c_int,_dev:*mut c_void)->irqreturn_t { IRQ_HANDLED }
unsafe fn arasan_cf_freeze(ap:*mut ata_port) { let acdev=(*(*ap).host).private_data as *mut arasan_cf_dev; writel(readl((*acdev).vbase.add(XFER_CTR))&!XFER_START,(*acdev).vbase.add(XFER_CTR)); cf_ctrl_reset(acdev); (*acdev).dma_status=ATA_DMA_ERR; ata_sff_dma_pause(ap); ata_sff_freeze(ap); }
unsafe fn arasan_cf_error_handler(ap:*mut ata_port) { let acdev=(*(*ap).host).private_data as *mut arasan_cf_dev; cancel_work_sync(&mut (*acdev).work); cancel_delayed_work_sync(&mut (*acdev).dwork); ata_sff_error_handler(ap); }
unsafe fn arasan_cf_dma_start(acdev:*mut arasan_cf_dev) { let qc=(*acdev).qc; let ap=(*qc).ap; let tf=&(*qc).tf; let mut x=readl((*acdev).vbase.add(XFER_CTR))&!XFER_DIR_MASK; if tf.flags&ATA_TFLAG_WRITE!=0{x|=XFER_WRITE;} writel(x,(*acdev).vbase.add(XFER_CTR)); ((*ap).ops).sff_exec_command(ap,tf); queue_work(&mut (*acdev).work); }
unsafe fn arasan_cf_qc_issue(qc:*mut ata_queued_cmd)->c_uint { let ap=(*qc).ap; let acdev=(*(*ap).host).private_data as *mut arasan_cf_dev; if !ata_is_dma((*qc).tf.protocol){return ata_sff_qc_issue(qc);} ata_wait_idle(ap); ata_sff_dev_select(ap,(*qc).dev.devno); ata_wait_idle(ap); (*acdev).dma_status=0; (*acdev).qc=qc; arasan_cf_dma_start(acdev); (*ap).hsm_task_state=HSM_ST_LAST; 0 }
unsafe fn arasan_cf_set_piomode(_ap:*mut ata_port,_adev:*mut ata_device) {}
unsafe fn arasan_cf_set_dmamode(_ap:*mut ata_port,_adev:*mut ata_device) {}
unsafe fn arasan_cf_probe(_pdev:*mut platform_device)->c_int { -ENOSYS }
unsafe fn arasan_cf_remove(_pdev:*mut platform_device) {}

/* The remaining definitions retain the C driver's operation-table and
 * platform-driver registration interfaces; dependent kernel types are
 * provided by the surrounding Rust kernel bindings. */
unsafe fn arasan_cf_driver_register() { module_platform_driver!(arasan_cf_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
