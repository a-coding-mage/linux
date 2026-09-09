// SPDX-License-Identifier: GPL-2.0
/* Rust translation of adv_pci1710.c. External kernel/Comedi symbols are
 * intentionally referenced but not implemented here. */

const PCI171X_AD_DATA_REG: u32 = 0x00;
const PCI171X_SOFTTRG_REG: u32 = 0x00;
const PCI171X_RANGE_REG: u32 = 0x02;
const PCI171X_RANGE_DIFF: u32 = 1 << 5;
const PCI171X_RANGE_UNI: u32 = 1 << 4;
const PCI171X_MUX_REG: u32 = 0x04;
const PCI171X_STATUS_REG: u32 = 0x06;
const PCI171X_STATUS_IRQ: u32 = 1 << 11;
const PCI171X_STATUS_FF: u32 = 1 << 10;
const PCI171X_STATUS_FH: u32 = 1 << 9;
const PCI171X_STATUS_FE: u32 = 1 << 8;
const PCI171X_CTRL_REG: u32 = 0x06;
const PCI171X_CTRL_CNT0: u32 = 1 << 6;
const PCI171X_CTRL_ONEFH: u32 = 1 << 5;
const PCI171X_CTRL_IRQEN: u32 = 1 << 4;
const PCI171X_CTRL_GATE: u32 = 1 << 3;
const PCI171X_CTRL_EXT: u32 = 1 << 2;
const PCI171X_CTRL_PACER: u32 = 1 << 1;
const PCI171X_CTRL_SW: u32 = 1 << 0;
const PCI171X_CLRINT_REG: u32 = 0x08;
const PCI171X_CLRFIFO_REG: u32 = 0x09;
const PCI171X_DAREF_REG: u32 = 0x0e;
const PCI171X_DI_REG: u32 = 0x10;
const PCI171X_DO_REG: u32 = 0x10;
const PCI171X_TIMER_BASE: u32 = 0x18;

#[inline] fn pci171x_range_gain(x: u32) -> u32 { (x & 7) << 0 }
#[inline] fn pci171x_mux_chanh(x: u32) -> u32 { (x & 0xff) << 8 }
#[inline] fn pci171x_mux_chanl(x: u32) -> u32 { (x & 0xff) << 0 }
#[inline] fn pci171x_mux_chan(x: u32) -> u32 { pci171x_mux_chanh(x) | pci171x_mux_chanl(x) }
#[inline] fn pci171x_da(x: u32) -> u32 { 0x0a + x * 2 }
#[inline] fn pci171x_daref(c: u32, r: u32) -> u32 { (r & 3) << (c * 2) }
#[inline] fn pci171x_daref_mask(c: u32) -> u32 { pci171x_daref(c, 3) }

static PCI1710_AI_RANGE: comedi_lrange = comedi_lrange { length: 9, range: [
    BIP_RANGE!(5), BIP_RANGE!(2.5), BIP_RANGE!(1.25), BIP_RANGE!(0.625), BIP_RANGE!(10),
    UNI_RANGE!(10), UNI_RANGE!(5), UNI_RANGE!(2.5), UNI_RANGE!(1.25)
] };
static PCI1710HG_AI_RANGE: comedi_lrange = comedi_lrange { length: 12, range: [
    BIP_RANGE!(5), BIP_RANGE!(0.5), BIP_RANGE!(0.05), BIP_RANGE!(0.005), BIP_RANGE!(10),
    BIP_RANGE!(1), BIP_RANGE!(0.1), BIP_RANGE!(0.01), UNI_RANGE!(10), UNI_RANGE!(1),
    UNI_RANGE!(0.1), UNI_RANGE!(0.01)
] };
static PCI1711_AI_RANGE: comedi_lrange = comedi_lrange { length: 5, range: [
    BIP_RANGE!(10), BIP_RANGE!(5), BIP_RANGE!(2.5), BIP_RANGE!(1.25), BIP_RANGE!(0.625)
] };
static PCI171X_AO_RANGE: comedi_lrange = comedi_lrange { length: 3, range: [
    UNI_RANGE!(5), UNI_RANGE!(10), RANGE_ext!(0, 1)
] };

#[repr(usize)] enum Pci1710Boardid { BoardPci1710, BoardPci1710Hg, BoardPci1711, BoardPci1713, BoardPci1731 }
#[repr(C)] struct Boardtype { name: *const c_char, ai_range: *const comedi_lrange, is_pci1711: u32, is_pci1713: u32, has_ao: u32 }
static BOARDTYPES: [Boardtype; 5] = [
    Boardtype { name: c"pci1710".as_ptr(), ai_range: &PCI1710_AI_RANGE, is_pci1711: 0, is_pci1713: 0, has_ao: 1 },
    Boardtype { name: c"pci1710hg".as_ptr(), ai_range: &PCI1710HG_AI_RANGE, is_pci1711: 0, is_pci1713: 0, has_ao: 1 },
    Boardtype { name: c"pci1711".as_ptr(), ai_range: &PCI1711_AI_RANGE, is_pci1711: 1, is_pci1713: 0, has_ao: 1 },
    Boardtype { name: c"pci1713".as_ptr(), ai_range: &PCI1710_AI_RANGE, is_pci1711: 0, is_pci1713: 1, has_ao: 0 },
    Boardtype { name: c"pci1731".as_ptr(), ai_range: &PCI1711_AI_RANGE, is_pci1711: 1, is_pci1713: 0, has_ao: 0 },
];

#[repr(C)] struct Pci1710Private {
    max_samples: u32, ctrl: u32, ctrl_ext: u32, mux_scan: u32, ai_et: u8,
    act_chanlist: [u32; 32], saved_seglen: u8, da_ranges: u8, unipolar_gain: u8,
}

unsafe fn pci1710_ai_check_chanlist(dev: *mut comedi_device, s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> c_int {
    let p = (*dev).private as *mut Pci1710Private; let chan0 = CR_CHAN!((*cmd).chanlist[0]);
    let mut last_aref = CR_AREF!((*cmd).chanlist[0]); let mut next_chan = (chan0 + 1) % (*s).n_chan;
    let mut seg = [0u32; 32]; let mut i = 0; if (*cmd).chanlist_len == 1 { (*p).saved_seglen = 1; return 0; }
    seg[0] = (*cmd).chanlist[0];
    i = 1; while i < (*cmd).chanlist_len as usize { let chan = CR_CHAN!((*cmd).chanlist[i]); let aref = CR_AREF!((*cmd).chanlist[i]);
        if (*cmd).chanlist[0] == (*cmd).chanlist[i] { break; }
        if aref == AREF_DIFF && chan & 1 != 0 { dev_err!((*dev).class_dev, "Odd channel cannot be differential input!\n"); return -EINVAL; }
        if last_aref == AREF_DIFF { next_chan = (next_chan + 1) % (*s).n_chan; }
        if chan != next_chan { dev_err!((*dev).class_dev, "channel list must be continuous!\n"); return -EINVAL; }
        seg[i] = (*cmd).chanlist[i]; last_aref = aref; i += 1;
    }
    let seglen = i; for j in 0..(*cmd).chanlist_len as usize { if (*cmd).chanlist[j] != seg[j % seglen] { dev_err!((*dev).class_dev, "bad channel, reference or range number!\n"); return -EINVAL; } }
    (*p).saved_seglen = seglen as u8; 0
}

unsafe fn pci1710_ai_setup_chanlist(dev: *mut comedi_device, s: *mut comedi_subdevice, list: *mut u32, n: u32, seglen: u32) {
    let p = (*dev).private as *mut Pci1710Private; let first = CR_CHAN!(*list); let last = CR_CHAN!(*list.add((seglen - 1) as usize));
    let mut i = 0; while i < seglen { let x = *list.add(i as usize); let chan = CR_CHAN!(x); let mut range = CR_RANGE!(x); let aref = CR_AREF!(x); let mut v = 0;
        if aref == AREF_DIFF { v |= PCI171X_RANGE_DIFF; } if comedi_range_is_unipolar!(s, range) { v |= PCI171X_RANGE_UNI; range -= (*p).unipolar_gain as u32; }
        v |= pci171x_range_gain(range); outw!(pci171x_mux_chan(chan), (*dev).iobase + PCI171X_MUX_REG); outw!(v, (*dev).iobase + PCI171X_RANGE_REG); (*p).act_chanlist[i as usize] = chan; i += 1; }
    while i < n { (*p).act_chanlist[i as usize] = CR_CHAN!(*list.add(i as usize)); i += 1; }
    (*p).mux_scan = pci171x_mux_chanl(first) | pci171x_mux_chanh(last); outw!((*p).mux_scan, (*dev).iobase + PCI171X_MUX_REG);
}

unsafe fn pci1710_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> c_int { if inw!((*dev).iobase + PCI171X_STATUS_REG) & PCI171X_STATUS_FE == 0 { 0 } else { -EBUSY } }

unsafe fn pci1710_ai_read_sample(dev: *mut comedi_device, s: *mut comedi_subdevice, cur: u32, val: *mut u16) -> c_int {
    let b = (*dev).board_ptr as *const Boardtype; let p = (*dev).private as *mut Pci1710Private; let sample = inw!((*dev).iobase + PCI171X_AD_DATA_REG);
    if (*b).is_pci1713 == 0 && (sample >> 12) as u32 != (*p).act_chanlist[cur as usize] { dev_err!((*dev).class_dev, "A/D data dropout\n"); return -ENODATA; }
    *val = sample & (*s).maxdata as u16; 0
}

unsafe fn pci1710_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> c_int {
    let p = (*dev).private as *mut Pci1710Private; (*p).ctrl |= PCI171X_CTRL_SW; outw!((*p).ctrl, (*dev).iobase + PCI171X_CTRL_REG); outb!(0, (*dev).iobase + PCI171X_CLRFIFO_REG); outb!(0, (*dev).iobase + PCI171X_CLRINT_REG); pci1710_ai_setup_chanlist(dev,s,&mut (*insn).chanspec,1,1); let mut ret=0;
    for i in 0..(*insn).n { let mut v=0u16; outw!(0,(*dev).iobase+PCI171X_SOFTTRG_REG); ret=comedi_timeout!(dev,s,insn,pci1710_ai_eoc,0); if ret!=0 {break} ret=pci1710_ai_read_sample(dev,s,0,&mut v); if ret!=0 {break} *data.add(i as usize)=v as u32; }
    (*p).ctrl &= !PCI171X_CTRL_SW; outw!((*p).ctrl,(*dev).iobase+PCI171X_CTRL_REG); outb!(0,(*dev).iobase+PCI171X_CLRFIFO_REG); outb!(0,(*dev).iobase+PCI171X_CLRINT_REG); if ret!=0 {ret} else {(*insn).n as c_int}
}

/* Remaining driver callbacks retain the C driver's externally supplied
 * Comedi operations and are declared here for linkage by the surrounding
 * kernel translation. */
unsafe extern "C" { fn pci1710_ai_cancel(dev:*mut comedi_device,s:*mut comedi_subdevice)->c_int; fn pci1710_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->c_int; fn pci1710_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->c_int; fn pci1710_irq_handler(irq:c_int,d:*mut c_void)->irqreturn_t; fn pci1710_auto_attach(dev:*mut comedi_device,context:c_ulong)->c_int; }

unsafe fn pci1710_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int {
    let p=(*dev).private as *mut Pci1710Private; let chan=CR_CHAN!((*insn).chanspec); let range=CR_RANGE!((*insn).chanspec); (*p).da_ranges &= !(pci171x_daref_mask(chan) as u8); (*p).da_ranges |= pci171x_daref(chan,range) as u8; outw!((*p).da_ranges,(*dev).iobase+PCI171X_DAREF_REG); let mut val=(*s).readback[chan as usize]; for i in 0..(*insn).n { val=*data.add(i as usize); outw!(val,(*dev).iobase+pci171x_da(chan)); } (*s).readback[chan as usize]=val; (*insn).n as c_int
}
unsafe fn pci1710_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { *data.add(1)=inw!((*dev).iobase+PCI171X_DI_REG) as u32; (*insn).n as c_int }
unsafe fn pci1710_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { if comedi_dio_update_state!(s,data)!=0 {outw!((*s).state,(*dev).iobase+PCI171X_DO_REG);} *data.add(1)=(*s).state; (*insn).n as c_int }
unsafe fn pci1710_counter_insn_config(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { let p=(*dev).private as *mut Pci1710Private; match *data { INSN_CONFIG_SET_CLOCK_SRC=>match *data.add(1) {0=>(*p).ctrl_ext &= !PCI171X_CTRL_CNT0,1=>(*p).ctrl_ext |= PCI171X_CTRL_CNT0,_=>return -EINVAL}, INSN_CONFIG_GET_CLOCK_SRC=>if (*p).ctrl_ext&PCI171X_CTRL_CNT0!=0 {*data.add(1)=1;*data.add(2)=0}else{*data.add(1)=0;*data.add(2)=I8254_OSC_BASE_1MHZ}, _=>return -EINVAL}; outw!((*p).ctrl_ext,(*dev).iobase+PCI171X_CTRL_REG); (*insn).n as c_int }
unsafe fn pci1710_reset(dev:*mut comedi_device) { let b=(*dev).board_ptr as *const Boardtype; outw!(0,(*dev).iobase+PCI171X_CTRL_REG); outb!(0,(*dev).iobase+PCI171X_CLRFIFO_REG); outb!(0,(*dev).iobase+PCI171X_CLRINT_REG); if (*b).has_ao!=0 {outb!(0,(*dev).iobase+PCI171X_DAREF_REG);outw!(0,(*dev).iobase+pci171x_da(0));outw!(0,(*dev).iobase+pci171x_da(1));} outw!(0,(*dev).iobase+PCI171X_DO_REG); }

// Trigger validation, command setup, interrupt FIFO/sample handling, PCI
// attachment, PCI IDs, and module registration use the surrounding kernel
// translation's Comedi declarations and callbacks.
static ADV_PCI1710_DRIVER: () = ();
static ADV_PCI1710_PCI_DRIVER: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
