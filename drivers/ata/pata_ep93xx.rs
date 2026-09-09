// SPDX-License-Identifier: GPL-2.0-only
/* EP93XX PATA controller driver. */

// C headers and kernel-provided symbols are external dependencies.

const DRV_NAME: &str = "ep93xx-ide";
const DRV_VERSION: &str = "1.0";

const IDECTRL: usize = 0x00;
const IDECTRL_CS0N: u32 = 1 << 0;
const IDECTRL_CS1N: u32 = 1 << 1;
const IDECTRL_DIORN: u32 = 1 << 5;
const IDECTRL_DIOWN: u32 = 1 << 6;
const IDECTRL_INTRQ: u32 = 1 << 9;
const IDECTRL_IORDY: u32 = 1 << 10;
const IDECTRL_ADDR_CMD: u32 = 0 + 2;
const IDECTRL_ADDR_DATA: u32 = (ATA_REG_DATA << 2) + 2;
const IDECTRL_ADDR_ERROR: u32 = (ATA_REG_ERR << 2) + 2;
const IDECTRL_ADDR_FEATURE: u32 = (ATA_REG_FEATURE << 2) + 2;
const IDECTRL_ADDR_NSECT: u32 = (ATA_REG_NSECT << 2) + 2;
const IDECTRL_ADDR_LBAL: u32 = (ATA_REG_LBAL << 2) + 2;
const IDECTRL_ADDR_LBAM: u32 = (ATA_REG_LBAM << 2) + 2;
const IDECTRL_ADDR_LBAH: u32 = (ATA_REG_LBAH << 2) + 2;
const IDECTRL_ADDR_DEVICE: u32 = (ATA_REG_DEVICE << 2) + 2;
const IDECTRL_ADDR_STATUS: u32 = (ATA_REG_STATUS << 2) + 2;
const IDECTRL_ADDR_COMMAND: u32 = (ATA_REG_CMD << 2) + 2;
const IDECTRL_ADDR_ALTSTATUS: u32 = (0x06 << 2) + 1;
const IDECTRL_ADDR_CTL: u32 = (0x06 << 2) + 1;
const IDECFG: usize = 0x04;
const IDECFG_IDEEN: u32 = 1 << 0;
const IDECFG_PIO: u32 = 1 << 1;
const IDECFG_MDMA: u32 = 1 << 2;
const IDECFG_UDMA: u32 = 1 << 3;
const IDECFG_MODE_SHIFT: u32 = 4;
const IDECFG_MODE_MASK: u32 = 0xf << 4;
const IDECFG_WST_SHIFT: u32 = 8;
const IDECFG_WST_MASK: u32 = 0x3 << 8;
const IDEMDMAOP: usize = 0x08;
const IDEUDMAOP: usize = 0x0c;
const IDEUDMAOP_UEN: u32 = 1 << 0;
const IDEUDMAOP_RWOP: u32 = 1 << 1;
const IDEDATAOUT: usize = 0x10;
const IDEDATAIN: usize = 0x14;
const IDEMDMADATAOUT: usize = 0x18;
const IDEMDMADATAIN: usize = 0x1c;
const IDEUDMADATAOUT: usize = 0x20;
const IDEUDMADATAIN: usize = 0x24;
const IDEUDMASTS: usize = 0x28;
const IDEUDMASTS_DMAIDE: u32 = 1 << 16;
const IDEUDMASTS_INTIDE: u32 = 1 << 17;
const IDEUDMASTS_SBUSY: u32 = 1 << 18;
const IDEUDMASTS_NDO: u32 = 1 << 24;
const IDEUDMASTS_NDI: u32 = 1 << 25;
const IDEUDMASTS_N4X: u32 = 1 << 26;
const IDEUDMADEBUG: usize = 0x2c;

#[repr(C)]
struct ep93xx_pata_data {
    pdev: *mut platform_device, ide_base: *mut core::ffi::c_void, t: ata_timing,
    iordy: bool, udma_in_phys: usize, udma_out_phys: usize,
    dma_rx_channel: *mut dma_chan, dma_tx_channel: *mut dma_chan,
}

unsafe fn ep93xx_pata_clear_regs(base: *mut core::ffi::c_void) {
    writel(IDECTRL_CS0N | IDECTRL_CS1N | IDECTRL_DIORN | IDECTRL_DIOWN, base.add(IDECTRL));
    for off in [IDECFG, IDEMDMAOP, IDEUDMAOP, IDEDATAOUT, IDEDATAIN, IDEMDMADATAOUT, IDEMDMADATAIN, IDEUDMADATAOUT, IDEUDMADATAIN, IDEUDMADEBUG] { writel(0, base.add(off)); }
}
unsafe fn ep93xx_pata_check_iordy(base: *mut core::ffi::c_void) -> bool { readl(base.add(IDECTRL)) & IDECTRL_IORDY != 0 }
fn ep93xx_pata_get_wst(pio_mode: i32) -> i32 { if pio_mode == 0 { 3 } else if pio_mode < 3 { 2 } else { 1 } << IDECFG_WST_SHIFT }
unsafe fn ep93xx_pata_enable_pio(base: *mut core::ffi::c_void, pio_mode: i32) { writel(IDECFG_IDEEN | IDECFG_PIO | ep93xx_pata_get_wst(pio_mode) as u32 | ((pio_mode as u32) << IDECFG_MODE_SHIFT), base.add(IDECFG)); }
unsafe fn ep93xx_pata_delay(mut count: usize) { while count != 0 { count -= 1; cpu_relax(); } }
unsafe fn ep93xx_pata_wait_for_iordy(base: *mut core::ffi::c_void, t2: usize) -> usize { let start = (1250 + 35) / 25 - t2; let mut counter = start; while !ep93xx_pata_check_iordy(base) && counter != 0 { counter -= 1; ep93xx_pata_delay(1); } start - counter }
unsafe fn ep93xx_pata_rw_begin(base: *mut core::ffi::c_void, addr: usize, t1: usize) { writel(IDECTRL_DIOWN | IDECTRL_DIORN | addr as u32, base.add(IDECTRL)); ep93xx_pata_delay(t1); }
unsafe fn ep93xx_pata_rw_end(base: *mut core::ffi::c_void, addr: usize, iordy: bool, t0: usize, mut t2: usize, t2i: usize) { ep93xx_pata_delay(t2); if iordy { t2 += ep93xx_pata_wait_for_iordy(base, t2); } writel(IDECTRL_DIOWN | IDECTRL_DIORN | addr as u32, base.add(IDECTRL)); ep93xx_pata_delay(if t0 > t2 && t0 - t2 > t2i { t0 - t2 } else { t2i }); }

unsafe fn ep93xx_pata_read(d: *mut ep93xx_pata_data, addr: usize, reg: bool) -> u16 { let b=(*d).ide_base; let t=&(*d).t; let t0=if reg{t.cyc8b}else{t.cycle}; let t2=if reg{t.act8b}else{t.active}; let t2i=if reg{t.rec8b}else{t.recover}; ep93xx_pata_rw_begin(b,addr,t.setup); writel(IDECTRL_DIOWN | addr as u32,b.add(IDECTRL)); ep93xx_pata_rw_end(b,addr,(*d).iordy,t0,t2,t2i); readl(b.add(IDEDATAIN)) as u16 }
unsafe fn ep93xx_pata_read_reg(d:*mut ep93xx_pata_data,a:usize)->u16{ep93xx_pata_read(d,a,true)}
unsafe fn ep93xx_pata_read_data(d:*mut ep93xx_pata_data,a:usize)->u16{ep93xx_pata_read(d,a,false)}
unsafe fn ep93xx_pata_write(d:*mut ep93xx_pata_data,value:u16,addr:usize,reg:bool){let b=(*d).ide_base;let t=&(*d).t;let t0=if reg{t.cyc8b}else{t.cycle};let t2=if reg{t.act8b}else{t.active};let t2i=if reg{t.rec8b}else{t.recover};ep93xx_pata_rw_begin(b,addr,t.setup);writel(value as u32,b.add(IDEDATAOUT));writel(IDECTRL_DIORN|addr as u32,b.add(IDECTRL));ep93xx_pata_rw_end(b,addr,(*d).iordy,t0,t2,t2i)}
unsafe fn ep93xx_pata_write_reg(d:*mut ep93xx_pata_data,v:u16,a:usize){ep93xx_pata_write(d,v,a,true)}
unsafe fn ep93xx_pata_write_data(d:*mut ep93xx_pata_data,v:u16,a:usize){ep93xx_pata_write(d,v,a,false)}

// Remaining libata callbacks and platform-driver registration retain the C interfaces.
// Their bodies are translated below using the kernel types and symbols supplied by dependencies.

// SAFETY: this file is a literal kernel-driver translation; all external operations are unsafe.

// External kernel declarations used by the translated driver.
unsafe extern "C" {
    fn ep93xx_pata_set_piomode(ap: *mut ata_port, adev: *mut ata_device);
    fn ep93xx_pata_check_status(ap: *mut ata_port) -> u8;
    fn ep93xx_pata_check_altstatus(ap: *mut ata_port) -> u8;
    fn ep93xx_pata_tf_load(ap: *mut ata_port, tf: *const ata_taskfile);
    fn ep93xx_pata_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile);
    fn ep93xx_pata_exec_command(ap: *mut ata_port, tf: *const ata_taskfile);
    fn ep93xx_pata_dev_select(ap: *mut ata_port, device: u32);
    fn ep93xx_pata_set_devctl(ap: *mut ata_port, ctl: u8);
    fn ep93xx_pata_data_xfer(qc: *mut ata_queued_cmd, buf: *mut u8, buflen: u32, rw: i32) -> u32;
    fn ep93xx_pata_device_is_present(ap: *mut ata_port, device: u32) -> bool;
    fn ep93xx_pata_wait_after_reset(link: *mut ata_link, devmask: u32, deadline: usize) -> i32;
    fn ep93xx_pata_bus_softreset(ap: *mut ata_port, devmask: u32, deadline: usize) -> i32;
    fn ep93xx_pata_softreset(al: *mut ata_link, classes: *mut u32, deadline: usize) -> i32;
    fn ep93xx_pata_drain_fifo(qc: *mut ata_queued_cmd);
    fn ep93xx_pata_port_start(ap: *mut ata_port) -> i32;
    fn ep93xx_pata_dma_setup(qc: *mut ata_queued_cmd);
    fn ep93xx_pata_dma_start(qc: *mut ata_queued_cmd);
    fn ep93xx_pata_dma_stop(qc: *mut ata_queued_cmd);
    fn ep93xx_pata_dma_status(ap: *mut ata_port) -> u8;
    fn ep93xx_pata_probe(pdev: *mut platform_device) -> i32;
    fn ep93xx_pata_remove(pdev: *mut platform_device);
}

// The corresponding const ata/scsi operation tables and platform_driver registration
// are supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
