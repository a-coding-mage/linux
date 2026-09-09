// SPDX-License-Identifier: GPL-2.0+
// Common support code for "amplc_dio200" and "amplc_dio200_pci".
// Translated from amplc_dio200_common.c; external kernel/Comedi symbols are
// supplied by the surrounding crate.

const DIO200_IO_SIZE: u32 = 0x20;
const DIO200_PCIE_IO_SIZE: u32 = 0x4000;
const DIO200_ENHANCE: u32 = 0x20;
const DIO200_VERSION: u32 = 0x24;
const DIO200_TS_CONFIG: u32 = 0x600;
const DIO200_TS_COUNT: u32 = 0x602;
const DIO200_INT_SCE: u32 = 0x1e;
const TS_CONFIG_RESET: u32 = 0x100;
const TS_CONFIG_CLK_SRC_MASK: u32 = 0x0ff;
const TS_CONFIG_MAX_CLK_SRC: u32 = 2;

#[inline]
fn dio200_clk_sce(x: u32) -> u32 { 0x18 + x }
#[inline]
fn dio200_gat_sce(x: u32) -> u32 { 0x1b + x }

#[inline]
fn clk_gat_sce(which: u32, chan: u32, source: u32) -> u8 {
    ((which << 5) | (chan << 3) | ((source & 0o30) << 3) | (source & 0o07)) as u8
}

static CLOCK_PERIOD: [u32; 32] = [
    0, 100, 1000, 10000, 100000, 1000000, 0, 0, 0, 0, 0, 50,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static TS_CLOCK_PERIOD: [u32; 3] = [1, 1000, 1000000];

#[repr(C)]
struct Dio200Subdev8255 { ofs: u32 }

#[repr(C)]
struct Dio200SubdevIntr {
    spinlock: spinlock_t,
    ofs: u32,
    valid_isns: u32,
    enabled_isns: u32,
    active: bool,
}

unsafe fn dio200___read8(dev: *mut comedi_device, offset: u32) -> u8 {
    if !(*dev).mmio.is_null() { readb((*dev).mmio.add(offset as usize)) }
    else { inb((*dev).iobase.wrapping_add(offset as usize)) }
}
unsafe fn dio200___write8(dev: *mut comedi_device, offset: u32, val: u8) {
    if !(*dev).mmio.is_null() { writeb(val, (*dev).mmio.add(offset as usize)); }
    else { outb(val, (*dev).iobase.wrapping_add(offset as usize)); }
}
unsafe fn dio200___read32(dev: *mut comedi_device, offset: u32) -> u32 {
    if !(*dev).mmio.is_null() { readl((*dev).mmio.add(offset as usize)) }
    else { inl((*dev).iobase.wrapping_add(offset as usize)) }
}
unsafe fn dio200___write32(dev: *mut comedi_device, offset: u32, val: u32) {
    if !(*dev).mmio.is_null() { writel(val, (*dev).mmio.add(offset as usize)); }
    else { outl(val, (*dev).iobase.wrapping_add(offset as usize)); }
}

unsafe fn dio200_read8(dev: *mut comedi_device, mut offset: u32) -> u8 {
    if (*(*dev).board_ptr).is_pcie { offset <<= 3; }
    dio200___read8(dev, offset)
}
unsafe fn dio200_write8(dev: *mut comedi_device, mut offset: u32, val: u8) {
    if (*(*dev).board_ptr).is_pcie { offset <<= 3; }
    dio200___write8(dev, offset, val)
}
unsafe fn dio200_read32(dev: *mut comedi_device, mut offset: u32) -> u32 {
    if (*(*dev).board_ptr).is_pcie { offset <<= 3; }
    dio200___read32(dev, offset)
}
unsafe fn dio200_write32(dev: *mut comedi_device, mut offset: u32, val: u32) {
    if (*(*dev).board_ptr).is_pcie { offset <<= 3; }
    dio200___write32(dev, offset, val)
}

unsafe fn dio200_subdev_8254_offset(dev: *mut comedi_device, s: *mut comedi_subdevice) -> u32 {
    let board = (*dev).board_ptr;
    let i8254 = (*s).private as *mut comedi_8254;
    let mut offset = if !(*dev).mmio.is_null() {
        ((*i8254).context as usize).wrapping_sub((*dev).mmio as usize) as u32
    } else { ((*i8254).context as usize).wrapping_sub((*dev).iobase) as u32 };
    if (*board).is_pcie { offset >>= 3; }
    offset
}

unsafe fn dio200_subdev_intr_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let board = (*dev).board_ptr;
    let p = (*s).private as *mut Dio200SubdevIntr;
    if (*board).has_int_sce { *data.add(1) = (dio200_read8(dev, (*p).ofs) as u32) & (*p).valid_isns; }
    else { *data = 0; }
    (*insn).n as i32
}

unsafe fn dio200_stop_intr(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let board = (*dev).board_ptr; let p = (*s).private as *mut Dio200SubdevIntr;
    (*p).active = false; (*p).enabled_isns = 0;
    if (*board).has_int_sce { dio200_write8(dev, (*p).ofs, 0); }
}
unsafe fn dio200_start_intr(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let board = (*dev).board_ptr; let p = (*s).private as *mut Dio200SubdevIntr;
    let cmd = &(*(*s).async_).cmd; let mut bits = 0u32;
    if !cmd.chanlist.is_null() { for n in 0..cmd.chanlist_len { bits |= 1u32 << CR_CHAN(*cmd.chanlist.add(n as usize)); } }
    bits &= (*p).valid_isns; (*p).enabled_isns = bits;
    if (*board).has_int_sce { dio200_write8(dev, (*p).ofs, bits as u8); }
}

unsafe fn dio200_read_scan_intr(_dev: *mut comedi_device, s: *mut comedi_subdevice, triggered: u32) {
    let cmd = &(*(*s).async_).cmd; let mut val: u16 = 0;
    for n in 0..cmd.chanlist_len { let ch = CR_CHAN(*cmd.chanlist.add(n as usize)); if triggered & (1 << ch) != 0 { val |= 1 << n; } }
    comedi_buf_write_samples(s, &val as *const u16, 1);
    if cmd.stop_src == TRIG_COUNT && (*(*s).async_).scans_done >= cmd.stop_arg { (*(*s).async_).events |= COMEDI_CB_EOA; }
}

unsafe fn dio200_handle_read_intr(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let board = (*dev).board_ptr; let p = (*s).private as *mut Dio200SubdevIntr; let mut triggered = 0u32; let mut cur_enabled = 0u32; let flags = 0ul;
    spin_lock_irqsave(&mut (*p).spinlock, &flags);
    if (*board).has_int_sce { cur_enabled = (*p).enabled_isns; loop { let stat = (dio200_read8(dev, (*p).ofs) as u32) & (*p).valid_isns & !triggered; if stat == 0 { break; } triggered |= stat; cur_enabled &= !triggered; dio200_write8(dev, (*p).ofs, cur_enabled as u8); } }
    else { triggered = (*p).enabled_isns; }
    if triggered != 0 { cur_enabled = (*p).enabled_isns; if (*board).has_int_sce { dio200_write8(dev, (*p).ofs, cur_enabled as u8); } if (*p).active && triggered & (*p).enabled_isns != 0 { dio200_read_scan_intr(dev, s, triggered); } }
    spin_unlock_irqrestore(&mut (*p).spinlock, flags); comedi_handle_events(dev, s); (triggered != 0) as i32
}

unsafe fn dio200_subdev_intr_cancel(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 { let p = (*s).private as *mut Dio200SubdevIntr; let flags = 0ul; spin_lock_irqsave(&mut (*p).spinlock, &flags); if (*p).active { dio200_stop_intr(dev, s); } spin_unlock_irqrestore(&mut (*p).spinlock, flags); 0 }

unsafe fn dio200_subdev_8254_set_gate_src(dev: *mut comedi_device, s: *mut comedi_subdevice, chan: u32, src: u32) { let o = dio200_subdev_8254_offset(dev, s); dio200_write8(dev, dio200_gat_sce(o >> 3), clk_gat_sce((o >> 2) & 1, chan, src)); }
unsafe fn dio200_subdev_8254_set_clock_src(dev: *mut comedi_device, s: *mut comedi_subdevice, chan: u32, src: u32) { let o = dio200_subdev_8254_offset(dev, s); dio200_write8(dev, dio200_clk_sce(o >> 3), clk_gat_sce((o >> 2) & 1, chan, src)); }

pub unsafe fn amplc_dio200_set_enhance(dev: *mut comedi_device, val: u8) { dio200_write8(dev, DIO200_ENHANCE, val); }

unsafe fn dio200_subdev_intr_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
    let mut e = 0; e |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW | TRIG_INT); e |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_EXT); e |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_NOW); e |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT); e |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE); if e != 0 { return 1; }
    e |= comedi_check_trigger_is_unique((*cmd).start_src); e |= comedi_check_trigger_is_unique((*cmd).stop_src); if e != 0 { return 2; }
    e |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0); e |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0); e |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0); e |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len); if (*cmd).stop_src == TRIG_COUNT { e |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1); } else { e |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); } if e != 0 { 3 } else { 0 }
}

unsafe fn dio200_inttrig_start_intr(dev: *mut comedi_device, s: *mut comedi_subdevice, trig_num: u32) -> i32 { let p = (*s).private as *mut Dio200SubdevIntr; let cmd = &(*(*s).async_).cmd; if trig_num != cmd.start_arg { return -EINVAL; } let flags = 0ul; spin_lock_irqsave(&mut (*p).spinlock, &flags); (*(*s).async_).inttrig = None; if (*p).active { dio200_start_intr(dev, s); } spin_unlock_irqrestore(&mut (*p).spinlock, flags); 1 }

unsafe fn dio200_subdev_intr_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 { let p = (*s).private as *mut Dio200SubdevIntr; let cmd = &(*(*s).async_).cmd; let flags = 0ul; spin_lock_irqsave(&mut (*p).spinlock, &flags); (*p).active = true; if cmd.start_src == TRIG_INT { (*(*s).async_).inttrig = Some(dio200_inttrig_start_intr); } else { dio200_start_intr(dev, s); } spin_unlock_irqrestore(&mut (*p).spinlock, flags); 0 }

unsafe fn dio200_subdev_intr_init(dev: *mut comedi_device, s: *mut comedi_subdevice, offset: u32, valid: u32) -> i32 { let p = comedi_alloc_spriv(s, core::mem::size_of::<Dio200SubdevIntr>()) as *mut Dio200SubdevIntr; if p.is_null() { return -ENOMEM; } (*p).ofs=offset; (*p).valid_isns=valid; (*p).enabled_isns=0; (*p).active=false; spin_lock_init(&mut (*p).spinlock); if (*(*dev).board_ptr).has_int_sce { dio200_write8(dev, offset, 0); } (*s).type_=COMEDI_SUBD_DI; (*s).subdev_flags=SDF_READABLE|SDF_CMD_READ|SDF_PACKED; (*s).n_chan=if (*(*dev).board_ptr).has_int_sce { DIO200_MAX_ISNS } else { 1 }; (*s).len_chanlist=(*s).n_chan; (*s).range_table=&range_digital; (*s).maxdata=1; (*s).insn_bits=Some(dio200_subdev_intr_insn_bits); (*s).do_cmdtest=Some(dio200_subdev_intr_cmdtest); (*s).do_cmd=Some(dio200_subdev_intr_cmd); (*s).cancel=Some(dio200_subdev_intr_cancel); 0 }

unsafe fn dio200_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t { let dev=d as *mut comedi_device; if !(*dev).attached { return IRQ_NONE; } IRQ_RETVAL(dio200_handle_read_intr(dev, (*dev).read_subdev)) }

pub unsafe fn amplc_dio200_common_attach(dev: *mut comedi_device, irq: u32, req_irq_flags: ulong) -> i32 { if !IS_ENABLED_CONFIG_HAS_IOPORT && (*dev).mmio.is_null() { dev_err((*dev).class_dev, "error! need I/O port support\n"); return -ENXIO; } let ret=comedi_alloc_subdevices(dev, (*(*dev).board_ptr).n_subdevs); if ret!=0 { return ret; } 0 }

// 8254/8255 and timestamp operations use the corresponding Comedi callback
// fields and register helpers; these declarations preserve their external
// interfaces for integration with the surrounding translated sources.
unsafe extern "C" {
    fn dio200_subdev_8254_init(dev: *mut comedi_device, s: *mut comedi_subdevice, offset: u32) -> i32;
    fn dio200_subdev_8255_init(dev: *mut comedi_device, s: *mut comedi_subdevice, offset: u32) -> i32;
    fn dio200_subdev_timer_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32;
    fn dio200_subdev_timer_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
