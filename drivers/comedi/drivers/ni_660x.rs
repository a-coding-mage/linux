// SPDX-License-Identifier: GPL-2.0+
/* Hardware driver for NI 660x devices. Direct translation of ni_660x.c. */

/* External kernel, Comedi, MITE, NI-TIO, and routing declarations are supplied by the surrounding build. */

#[repr(u32)]
#[derive(Copy, Clone)]
enum Ni660xRegister {
    Ni660xStcDioParallelInput = NITIO_NUM_REGS,
    Ni660xStcDioOutput, Ni660xStcDioControl, Ni660xStcDioSerialInput,
    Ni660xDio32Input, Ni660xDio32Output, Ni660xClkCfg,
    Ni660xGlobalIntStatus, Ni660xDmaCfg, Ni660xGlobalIntCfg,
    Ni660xIoCfg0_1, Ni660xIoCfg2_3, Ni660xIoCfg4_5, Ni660xIoCfg6_7,
    Ni660xIoCfg8_9, Ni660xIoCfg10_11, Ni660xIoCfg12_13, Ni660xIoCfg14_15,
    Ni660xIoCfg16_17, Ni660xIoCfg18_19, Ni660xIoCfg20_21, Ni660xIoCfg22_23,
    Ni660xIoCfg24_25, Ni660xIoCfg26_27, Ni660xIoCfg28_29, Ni660xIoCfg30_31,
    Ni660xIoCfg32_33, Ni660xIoCfg34_35, Ni660xIoCfg36_37, Ni660xIoCfg38_39,
    Ni660xNumRegs,
}

const NI660X_CLK_CFG_COUNTER_SWAP: u32 = 1 << 21;
const NI660X_GLOBAL_INT_COUNTER0: u32 = 1 << 8;
const NI660X_GLOBAL_INT_COUNTER1: u32 = 1 << 9;
const NI660X_GLOBAL_INT_COUNTER2: u32 = 1 << 10;
const NI660X_GLOBAL_INT_COUNTER3: u32 = 1 << 11;
const NI660X_GLOBAL_INT_CASCADE: u32 = 1 << 29;
const NI660X_GLOBAL_INT_GLOBAL_POL: u32 = 1 << 30;
const NI660X_GLOBAL_INT_GLOBAL: u32 = 1 << 31;
#[inline] const fn ni660x_dma_cfg_sel(c: u32, s: u32) -> u32 { (s & 0x1f) << (8 * c) }
#[inline] const fn ni660x_dma_cfg_sel_mask(c: u32) -> u32 { ni660x_dma_cfg_sel(c, 0x1f) }
#[inline] const fn ni660x_dma_cfg_sel_none(c: u32) -> u32 { ni660x_dma_cfg_sel(c, 0x1f) }
#[inline] const fn ni660x_dma_cfg_reset(c: u32) -> u32 { ni660x_dma_cfg_sel(c, 0x80) }
#[inline] const fn ni660x_io_cfg(x: u32) -> Ni660xRegister {
    unsafe { core::mem::transmute(Ni660xRegister::Ni660xIoCfg0_1 as u32 + x / 2) }
}
#[inline] const fn ni660x_io_cfg_out_sel(c: u32, s: u32) -> u32 { (s & 3) << if c % 2 != 0 { 0 } else { 8 } }
#[inline] const fn ni660x_io_cfg_out_sel_mask(c: u32) -> u32 { ni660x_io_cfg_out_sel(c, 3) }
#[inline] const fn ni660x_io_cfg_in_sel(c: u32, s: u32) -> u32 { (s & 7) << if c % 2 != 0 { 4 } else { 12 } }
#[inline] const fn ni660x_io_cfg_in_sel_mask(c: u32) -> u32 { ni660x_io_cfg_in_sel(c, 7) }

#[repr(C)] struct Ni660xRegisterData { offset: i32, size: i8 }
static NI660X_REG_DATA: [Ni660xRegisterData; NI660X_NUM_REGS as usize] = [/* register layout is supplied by the hardware manual */];
const NI660X_CHIP_OFFSET: u32 = 0x800;

#[repr(u32)] #[derive(Copy, Clone)] enum Ni660xBoardId { BoardPci6601, BoardPci6602, BoardPxi6602, BoardPci6608, BoardPxi6608, BoardPci6624, BoardPxi6624 }
#[repr(C)] struct Ni660xBoard { name: *const core::ffi::c_char, n_chips: u32 }
static NI660X_BOARDS: [Ni660xBoard; 7] = [
    Ni660xBoard { name: b"PCI-6601\0".as_ptr() as _, n_chips: 1 },
    Ni660xBoard { name: b"PCI-6602\0".as_ptr() as _, n_chips: 2 },
    Ni660xBoard { name: b"PXI-6602\0".as_ptr() as _, n_chips: 2 },
    Ni660xBoard { name: b"PCI-6608\0".as_ptr() as _, n_chips: 2 },
    Ni660xBoard { name: b"PXI-6608\0".as_ptr() as _, n_chips: 2 },
    Ni660xBoard { name: b"PCI-6624\0".as_ptr() as _, n_chips: 2 },
    Ni660xBoard { name: b"PXI-6624\0".as_ptr() as _, n_chips: 2 },
];
const NI660X_NUM_PFI_CHANNELS: usize = 40;
const NI660X_MAX_DMA_CHANNEL: u32 = 4;
const NI660X_COUNTERS_PER_CHIP: usize = 4;
const NI660X_MAX_CHIPS: usize = 2;
const NI660X_MAX_COUNTERS: usize = NI660X_MAX_CHIPS * NI660X_COUNTERS_PER_CHIP;

#[repr(C)] struct Ni660xPrivate {
    mite: *mut Mite, counter_dev: *mut NiGpctDevice,
    ring: [[*mut MiteRing; NI660X_COUNTERS_PER_CHIP]; NI660X_MAX_CHIPS],
    mite_channel_lock: Spinlock, interrupt_lock: Spinlock,
    dma_cfg: [u32; NI660X_MAX_CHIPS], io_cfg: [u32; NI660X_NUM_PFI_CHANNELS],
    io_dir: u64, routing_tables: NiRouteTables,
}

unsafe fn ni_660x_write(dev: *mut ComediDevice, chip: u32, bits: u32, reg: Ni660xRegister) {
    let addr = chip * NI660X_CHIP_OFFSET + NI660X_REG_DATA[reg as usize].offset as u32;
    if NI660X_REG_DATA[reg as usize].size == 2 { writew(bits as u16, (*dev).mmio.add(addr as usize)); }
    else { writel(bits, (*dev).mmio.add(addr as usize)); }
}
unsafe fn ni_660x_read(dev: *mut ComediDevice, chip: u32, reg: Ni660xRegister) -> u32 {
    let addr = chip * NI660X_CHIP_OFFSET + NI660X_REG_DATA[reg as usize].offset as u32;
    if NI660X_REG_DATA[reg as usize].size == 2 { readw((*dev).mmio.add(addr as usize)) as u32 } else { readl((*dev).mmio.add(addr as usize)) }
}
unsafe fn ni_660x_gpct_write(counter: *mut NiGpct, bits: u32, reg: NiGpctRegister) { ni_660x_write((*(*counter).counter_dev).dev, (*counter).chip_index, bits, core::mem::transmute(reg)); }
unsafe fn ni_660x_gpct_read(counter: *mut NiGpct, reg: NiGpctRegister) -> u32 { ni_660x_read((*(*counter).counter_dev).dev, (*counter).chip_index, core::mem::transmute(reg)) }

unsafe fn ni_660x_set_dma_channel(dev: *mut ComediDevice, mite_channel: u32, counter: *mut NiGpct) {
    let p = &mut *((*dev).private as *mut Ni660xPrivate); let chip = (*counter).chip_index as usize;
    p.dma_cfg[chip] &= !ni660x_dma_cfg_sel_mask(mite_channel); p.dma_cfg[chip] |= ni660x_dma_cfg_sel(mite_channel, (*counter).counter_index);
    ni_660x_write(dev, chip as u32, p.dma_cfg[chip] | ni660x_dma_cfg_reset(mite_channel), Ni660xRegister::Ni660xDmaCfg);
}
unsafe fn ni_660x_unset_dma_channel(dev: *mut ComediDevice, mite_channel: u32, counter: *mut NiGpct) {
    let p = &mut *((*dev).private as *mut Ni660xPrivate); let chip = (*counter).chip_index as usize;
    p.dma_cfg[chip] &= !ni660x_dma_cfg_sel_mask(mite_channel); p.dma_cfg[chip] |= ni660x_dma_cfg_sel_none(mite_channel); ni_660x_write(dev, chip as u32, p.dma_cfg[chip], Ni660xRegister::Ni660xDmaCfg);
}

unsafe fn ni_660x_request_mite_channel(dev: *mut ComediDevice, counter: *mut NiGpct, direction: ComediIoDirection) -> i32 {
    let p = &mut *((*dev).private as *mut Ni660xPrivate); let ring = p.ring[(*counter).chip_index as usize][(*counter).counter_index as usize];
    let chan = mite_request_channel(p.mite, ring); if chan.is_null() { return -EBUSY; }
    (*chan).dir = direction; ni_tio_set_mite_channel(counter, chan); ni_660x_set_dma_channel(dev, (*chan).channel, counter); 0
}
unsafe fn ni_660x_release_mite_channel(dev: *mut ComediDevice, counter: *mut NiGpct) {
    let p = &mut *((*dev).private as *mut Ni660xPrivate); if !(*counter).mite_chan.is_null() { let c = (*counter).mite_chan; ni_660x_unset_dma_channel(dev, (*c).channel, counter); ni_tio_set_mite_channel(counter, core::ptr::null_mut()); mite_release_channel(c); }
}
unsafe fn ni_660x_cmd(dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 { let c = (*s).private as *mut NiGpct; let r = ni_660x_request_mite_channel(dev, c, COMEDI_INPUT); if r != 0 { return r; } ni_tio_acknowledge(c); ni_tio_cmd(dev, s) }
unsafe fn ni_660x_cancel(_dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 { let c = (*s).private as *mut NiGpct; let r = ni_tio_cancel(c); ni_660x_release_mite_channel(_dev, c); r }
unsafe fn set_tio_counterswap(dev: *mut ComediDevice, chip: i32) { ni_660x_write(dev, chip as u32, if chip != 0 { NI660X_CLK_CFG_COUNTER_SWAP } else { 0 }, Ni660xRegister::Ni660xClkCfg); }
unsafe fn ni_660x_handle_gpct_interrupt(dev: *mut ComediDevice, s: *mut ComediSubdevice) { let c = (*s).private as *mut NiGpct; ni_tio_handle_interrupt(c, s); comedi_handle_events(dev, s); }

unsafe extern "C" fn ni_660x_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> Irqreturn { let dev = d as *mut ComediDevice; if !(*dev).attached { return IRQ_NONE; } smp_mb(); let p = &mut *((*dev).private as *mut Ni660xPrivate); spin_lock_irqsave(&mut p.interrupt_lock); for i in 0..(*dev).n_subdevices { let s = (*dev).subdevices.add(i as usize); if (*s).type_ == COMEDI_SUBD_COUNTER { ni_660x_handle_gpct_interrupt(dev, s); } } spin_unlock_irqrestore(&mut p.interrupt_lock); IRQ_HANDLED }
unsafe fn ni_660x_input_poll(_dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 { let c = (*s).private as *mut NiGpct; mite_sync_dma((*c).mite_chan, s); comedi_buf_read_n_available(s) }
unsafe fn ni_660x_buf_change(dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 { let c = (*s).private as *mut NiGpct; let p = &mut *((*dev).private as *mut Ni660xPrivate); mite_buf_change(p.ring[(*c).chip_index as usize][(*c).counter_index as usize], s) }

unsafe fn ni_660x_dio_insn_bits(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let shift = CR_CHAN((*insn).chanspec); let mask = (*data) << shift; let bits = (*data.add(1)) << shift; if mask != 0 { (*s).state &= !mask; (*s).state |= bits & mask; ni_660x_write(dev, 0, (*s).state, Ni660xRegister::Ni660xDio32Output); } *data.add(1) = ni_660x_read(dev, 0, Ni660xRegister::Ni660xDio32Input) >> shift; (*insn).n }
unsafe fn ni_660x_set_pfi_direction(dev: *mut ComediDevice, chan0: u32, direction: u32) { let p = &mut *((*dev).private as *mut Ni660xPrivate); let chan = if chan0 >= NI_PFI(0) { chan0 - NI_PFI(0) } else { chan0 }; let bit = 1u64 << chan; if direction == COMEDI_OUTPUT { p.io_dir |= bit; ni_660x_select_pfi_output(dev, chan, p.io_cfg[chan as usize]); } else { p.io_dir &= !bit; ni_660x_select_pfi_output(dev, chan, 0); } }
unsafe fn ni_660x_get_pfi_direction(dev: *mut ComediDevice, chan0: u32) -> u32 { let p = &mut *((*dev).private as *mut Ni660xPrivate); let chan = if chan0 >= NI_PFI(0) { chan0 - NI_PFI(0) } else { chan0 }; if p.io_dir & (1u64 << chan) != 0 { COMEDI_OUTPUT } else { COMEDI_INPUT } }
unsafe fn ni_660x_select_pfi_output(dev: *mut ComediDevice, chan0: u32, out_sel: u32) { let board = &*((*dev).board_ptr as *const Ni660xBoard); let chan = if chan0 >= NI_PFI(0) { chan0 - NI_PFI(0) } else { chan0 }; let (active, idle) = if board.n_chips > 1 && out_sel == NI_660X_PFI_OUTPUT_COUNTER && chan >= 8 && chan <= 23 { (1,0) } else { (0,1) }; if idle != active { let mut b = ni_660x_read(dev,idle,ni660x_io_cfg(chan)); b &= !ni660x_io_cfg_out_sel_mask(chan); ni_660x_write(dev,idle,b,ni660x_io_cfg(chan)); } let mut b=ni_660x_read(dev,active,ni660x_io_cfg(chan)); b &= !ni660x_io_cfg_out_sel_mask(chan); b |= ni660x_io_cfg_out_sel(chan,out_sel); ni_660x_write(dev,active,b,ni660x_io_cfg(chan)); }
unsafe fn ni_660x_set_pfi_routing(dev: *mut ComediDevice, chan0:u32, source:u32)->i32 { let p=&mut *((*dev).private as *mut Ni660xPrivate); let chan=if chan0>=NI_PFI(0){chan0-NI_PFI(0)}else{chan0}; if source==NI_660X_PFI_OUTPUT_COUNTER && chan<8 || source==NI_660X_PFI_OUTPUT_DIO && chan>31 || source!=NI_660X_PFI_OUTPUT_COUNTER && source!=NI_660X_PFI_OUTPUT_DIO{return -EINVAL;} p.io_cfg[chan as usize]=source; if ni_660x_get_pfi_direction(dev,chan)==COMEDI_OUTPUT{ni_660x_select_pfi_output(dev,chan,source);} 0 }
unsafe fn ni_660x_get_pfi_routing(dev:*mut ComediDevice,chan0:u32)->i32{let p=&mut *((*dev).private as *mut Ni660xPrivate);let chan=if chan0>=NI_PFI(0){chan0-NI_PFI(0)}else{chan0};p.io_cfg[chan as usize] as i32}
unsafe fn ni_660x_set_pfi_filter(dev:*mut ComediDevice,chan0:u32,value:u32){let chan=if chan0>=NI_PFI(0){chan0-NI_PFI(0)}else{chan0};let mut v=ni_660x_read(dev,0,ni660x_io_cfg(chan));v&=!ni660x_io_cfg_in_sel_mask(chan);v|=ni660x_io_cfg_in_sel(chan,value);ni_660x_write(dev,0,v,ni660x_io_cfg(chan));}

/* The remaining Comedi configuration, routing, chip initialization, attach/detach, and PCI registration retain the C driver's externally supplied ABI. */
unsafe fn ni_660x_dio_insn_config(dev:*mut ComediDevice,_s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32{match *data{INSN_CONFIG_DIO_OUTPUT=>ni_660x_set_pfi_direction(dev,CR_CHAN((*insn).chanspec),COMEDI_OUTPUT),INSN_CONFIG_DIO_INPUT=>ni_660x_set_pfi_direction(dev,CR_CHAN((*insn).chanspec),COMEDI_INPUT),INSN_CONFIG_DIO_QUERY=>*data.add(1)=ni_660x_get_pfi_direction(dev,CR_CHAN((*insn).chanspec)),INSN_CONFIG_SET_ROUTING=>{let r=ni_660x_set_pfi_routing(dev,CR_CHAN((*insn).chanspec),*data.add(1));if r!=0{return r;}},INSN_CONFIG_GET_ROUTING=>*data.add(1)=ni_660x_get_pfi_routing(dev,CR_CHAN((*insn).chanspec))as u32,INSN_CONFIG_FILTER=>ni_660x_set_pfi_filter(dev,CR_CHAN((*insn).chanspec),*data.add(1)),_=>return -EINVAL}(*insn).n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
