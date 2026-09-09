/* SPDX-License-Identifier: GPL-2.0 */
/* Sparc specific parts of the Floppy driver. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct sun_flpy_controller {
    pub status1_82077: u8,
    pub status2_82077: u8,
    pub dor_82077: u8,
    pub tapectl_82077: u8,
    pub status_82077: u8,
    pub data_82077: u8,
    pub ___unused: u8,
    pub dir_82077: u8,
}

#[repr(C)]
pub struct sun_floppy_ops {
    pub fd_inb: Option<unsafe extern "C" fn(c_ulong, c_uint) -> u8>,
    pub fd_outb: Option<unsafe extern "C" fn(u8, c_ulong, c_uint)>,
    pub fd_enable_dma: Option<unsafe extern "C" fn()>,
    pub fd_disable_dma: Option<unsafe extern "C" fn()>,
    pub fd_set_dma_mode: Option<unsafe extern "C" fn(c_int)>,
    pub fd_set_dma_addr: Option<unsafe extern "C" fn(*mut c_char)>,
    pub fd_set_dma_count: Option<unsafe extern "C" fn(c_int)>,
    pub get_dma_residue: Option<unsafe extern "C" fn() -> c_uint>,
    pub fd_request_irq: Option<unsafe extern "C" fn() -> c_int>,
    pub fd_free_irq: Option<unsafe extern "C" fn()>,
    pub fd_eject: Option<unsafe extern "C" fn(c_int) -> c_int>,
}

#[repr(C)]
pub struct sun_pci_dma_op {
    pub addr: c_uint,
    pub len: c_int,
    pub direction: c_int,
    pub buf: *mut c_char,
}

#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct ebus_dma_info { _private: [u8; 0] }

static mut sun_fdc: *mut sun_flpy_controller = (-1isize) as *mut _;
pub static mut fdc_status: c_ulong = 0;
static mut floppy_op: *mut platform_device = core::ptr::null_mut();
static mut sun_fdops: sun_floppy_ops = sun_floppy_ops {
    fd_inb: None, fd_outb: None, fd_enable_dma: None, fd_disable_dma: None,
    fd_set_dma_mode: None, fd_set_dma_addr: None, fd_set_dma_count: None,
    get_dma_residue: None, fd_request_irq: None, fd_free_irq: None, fd_eject: None,
};
static mut sun_floppy_types: [c_int; 2] = [0, 0];
pub static mut pdma_vaddr: *mut u8 = core::ptr::null_mut();
pub static mut pdma_size: c_ulong = 0;
pub static mut doing_pdma: c_int = 0;
pub static mut pdma_base: *mut c_char = core::ptr::null_mut();
pub static mut pdma_areasize: c_ulong = 0;
static mut sun_pci_fd_ebus_dma: ebus_dma_info = ebus_dma_info { _private: [] };
static mut sun_floppy_dev: *mut device = core::ptr::null_mut();
static mut sun_pci_broken_drive: c_int = -1;
static mut sun_pci_dma_current: sun_pci_dma_op = sun_pci_dma_op { addr: !0, len: 0, direction: 0, buf: core::ptr::null_mut() };
static mut sun_pci_dma_pending: sun_pci_dma_op = sun_pci_dma_op { addr: !0, len: 0, direction: 0, buf: core::ptr::null_mut() };

extern "C" {
    fn udelay(usecs: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn panic(fmt: *const c_char) -> !;
    fn sbus_readb(addr: *const u8) -> u8;
    fn sbus_writeb(value: u8, addr: *mut u8);
    fn readb(addr: *const c_void) -> u8;
    fn writeb(value: u8, addr: *mut c_void);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn floppy_interrupt(irq: c_int, dev: *mut c_void) -> c_int;
    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn set_dor(a: u8, b: u8, c: u8);
    fn dma_map_single(dev: *mut device, buf: *mut c_char, len: c_int, dir: c_int) -> c_uint;
    fn dma_unmap_single(dev: *mut device, addr: c_uint, len: c_int, dir: c_int);
    fn ebus_dma_enable(info: *mut ebus_dma_info, enable: c_int);
    fn ebus_dma_request(info: *mut ebus_dma_info, addr: c_uint, len: c_int) -> c_int;
    fn ebus_dma_residue(info: *mut ebus_dma_info) -> c_uint;
    fn ebus_dma_prepare(info: *mut ebus_dma_info, read: c_int);
    fn ebus_dma_irq_enable(info: *mut ebus_dma_info, enable: c_int) -> c_int;
    fn ns87303_modify(config: c_ulong, reg: c_int, clear: c_int, set: c_int);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    static mut FLOPPY_IRQ: c_int;
    static mut allowed_drive_mask: c_int;
    static mut can_use_virtual_dma: c_int;
    static mut use_virtual_dma: c_int;
    static mut auxio_register: *mut c_void;
}

const FD_STATUS: c_uint = 0; const FD_DATA: c_uint = 1; const FD_DIR: c_uint = 2;
const FD_DOR: c_uint = 3; const FD_DCR: c_uint = 4; const FD_DSR: c_uint = 5;
const STATUS_DMA: u8 = 0x20; const DMA_MODE_READ: c_int = 0; const DMA_MODE_WRITE: c_int = 1;
const DMA_TO_DEVICE: c_int = 1; const DMA_FROM_DEVICE: c_int = 2;
const IRQ_HANDLED: c_int = 1; const EINVAL: c_int = 22;
const AUXIO_AUX1_FTCNT: u8 = 0x01;

pub unsafe extern "C" fn sun_82077_fd_inb(_base: c_ulong, reg: c_uint) -> u8 {
    udelay(5); match reg {
        FD_STATUS => sbus_readb(&(*sun_fdc).status_82077) & !STATUS_DMA,
        FD_DATA => sbus_readb(&(*sun_fdc).data_82077),
        FD_DIR => sbus_readb(&(*sun_fdc).dir_82077),
        _ => { printk(b"floppy: Asked to read unknown port %x\0".as_ptr() as _, reg); panic(b"floppy: Port bolixed.\0".as_ptr() as _) }
    }
}
pub unsafe extern "C" fn sun_82077_fd_outb(value: u8, _base: c_ulong, reg: c_uint) {
    udelay(5); match reg { FD_DOR => sbus_writeb(value, &mut (*sun_fdc).dor_82077), FD_DATA => sbus_writeb(value, &mut (*sun_fdc).data_82077), FD_DCR => sbus_writeb(value, &mut (*sun_fdc).dir_82077), FD_DSR => sbus_writeb(value, &mut (*sun_fdc).status_82077), _ => { printk(b"floppy: Asked to write to unknown port %x\0".as_ptr() as _, reg); panic(b"floppy: Port bolixed.\0".as_ptr() as _) } }
}
pub unsafe extern "C" fn sun_fd_disable_dma() { doing_pdma = 0; pdma_base = core::ptr::null_mut(); }
pub unsafe extern "C" fn sun_fd_set_dma_mode(mode: c_int) { match mode { DMA_MODE_READ => doing_pdma=1, DMA_MODE_WRITE => doing_pdma=2, _ => panic(b"floppy: Giving up...\0".as_ptr() as _) } }
pub unsafe extern "C" fn sun_fd_set_dma_addr(buffer: *mut c_char) { pdma_vaddr=buffer as *mut u8; }
pub unsafe extern "C" fn sun_fd_set_dma_count(length: c_int) { pdma_size=length as c_ulong; }
pub unsafe extern "C" fn sun_fd_enable_dma() { pdma_base=pdma_vaddr as *mut c_char; pdma_areasize=pdma_size; }

pub unsafe extern "C" fn sparc_floppy_irq(irq: c_int, dev_cookie: *mut c_void) -> c_int {
    if doing_pdma != 0 { let stat=fdc_status as *mut c_void; let mut vaddr=pdma_vaddr; let mut size=pdma_size; while size != 0 { let val=readb(stat); if val&0x80==0 { pdma_vaddr=vaddr; pdma_size=size; return IRQ_HANDLED; } if val&0x20==0 { pdma_vaddr=vaddr; pdma_size=size; doing_pdma=0; break; } if val&0x40!=0 { *vaddr=readb((stat as *mut u8).add(1) as _); vaddr=vaddr.add(1); } else { let data=*vaddr; vaddr=vaddr.add(1); writeb(data,(stat as *mut u8).add(1) as _); } size-=1; } pdma_vaddr=vaddr; pdma_size=size; let mut val=readb(auxio_register); val|=AUXIO_AUX1_FTCNT; writeb(val,auxio_register); val&=!AUXIO_AUX1_FTCNT; writeb(val,auxio_register); doing_pdma=0; }
    floppy_interrupt(irq,dev_cookie)
}
pub unsafe extern "C" fn sun_fd_request_irq() -> c_int { static mut once: c_int=0; if once==0 { once=1; return if request_irq(FLOPPY_IRQ,sparc_floppy_irq,0,b"floppy\0".as_ptr() as _,core::ptr::null_mut())==0 {0} else {-1}; } 0 }
pub unsafe extern "C" fn sun_fd_free_irq() {}
pub unsafe extern "C" fn sun_get_dma_residue() -> c_uint { 0 }
pub unsafe extern "C" fn sun_fd_eject(_drive: c_int) -> c_int { set_dor(0,0xff,0x90); udelay(500); set_dor(0,0x6f,0); udelay(500); 0 }

pub unsafe extern "C" fn sun_pci_fd_inb(base:c_ulong,reg:c_uint)->u8 { udelay(5); inb(base+reg as c_ulong) }
pub unsafe extern "C" fn sun_pci_fd_outb(val:u8,base:c_ulong,reg:c_uint) { udelay(5); outb(val,base+reg as c_ulong); }
pub unsafe extern "C" fn sun_pci_fd_broken_outb(mut val:u8,base:c_ulong,reg:c_uint) { udelay(5); if reg==FD_DOR && ((val&3) as c_int==sun_pci_broken_drive) && val&0x20!=0 { val|=0x10; } outb(val,base+reg as c_ulong); }
pub unsafe extern "C" fn sun_pci_fd_enable_dma() {
    /* BUG_ON checks from the kernel are represented by the same invariant. */
    if sun_pci_dma_pending.buf.is_null() || sun_pci_dma_pending.len==0 || sun_pci_dma_pending.direction==0 { panic(b"BUG_ON\0".as_ptr() as _); }
    sun_pci_dma_current.buf=sun_pci_dma_pending.buf; sun_pci_dma_current.len=sun_pci_dma_pending.len; sun_pci_dma_current.direction=sun_pci_dma_pending.direction;
    sun_pci_dma_pending.buf=core::ptr::null_mut(); sun_pci_dma_pending.len=0; sun_pci_dma_pending.direction=0; sun_pci_dma_pending.addr=!0;
    sun_pci_dma_current.addr=dma_map_single(sun_floppy_dev,sun_pci_dma_current.buf,sun_pci_dma_current.len,sun_pci_dma_current.direction);
    ebus_dma_enable(&mut sun_pci_fd_ebus_dma,1);
    if ebus_dma_request(&mut sun_pci_fd_ebus_dma,sun_pci_dma_current.addr,sun_pci_dma_current.len)!=0 { panic(b"BUG\0".as_ptr() as _); }
}
pub unsafe extern "C" fn sun_pci_fd_disable_dma() {
    ebus_dma_enable(&mut sun_pci_fd_ebus_dma,0);
    if sun_pci_dma_current.addr!=!0 { dma_unmap_single(sun_floppy_dev,sun_pci_dma_current.addr,sun_pci_dma_current.len,sun_pci_dma_current.direction); }
    sun_pci_dma_current.addr=!0;
}
pub unsafe extern "C" fn sun_pci_fd_out_byte(port:c_ulong,val:u8,reg:c_ulong) {
    let mut timeout=1000; while inb(port+4)&0x80==0 && {timeout-=1; timeout!=0} { udelay(100); } outb(val,reg);
}
pub unsafe extern "C" fn sun_pci_fd_sensei(port:c_ulong)->u8 {
    let mut result=[0x70u8,0]; let mut i=0; sun_pci_fd_out_byte(port,0x08,port+5); loop { let mut timeout=1000; let status; while {status=inb(port+4); status&0x80==0} && {timeout-=1; timeout!=0} { udelay(100); } if timeout==0 {break;} if status&0xf0==0xd0 {result[i]=inb(port+5); i+=1;} else {break;} if i>=2 {break;} } result[0]
}
pub unsafe extern "C" fn sun_pci_fd_reset(port:c_ulong) { let mut mask=0u8; let mut timeout=10000; outb(0x80,port+4); loop {let status=sun_pci_fd_sensei(port); if status&0xc0==0xc0 {mask|=1<<(status&3);} else {udelay(100);} timeout-=1; if mask==0x0f || timeout==0 {break;}} }
pub unsafe extern "C" fn sun_pci_fd_test_drive(port:c_ulong,drive:c_int)->c_int { sun_pci_fd_reset(port); let data=(0x10u8<<drive)|0x0c|drive as u8; sun_pci_fd_out_byte(port,data,port+2); sun_pci_fd_out_byte(port,7,port+5); sun_pci_fd_out_byte(port,(drive&3) as u8,port+5); let mut timeout=1000; let mut status; loop {udelay(100); status=sun_pci_fd_sensei(port); timeout-=1; if status&0xc0!=0x80 || timeout==0 {break;}} let ready=if timeout==0 {0} else if status&0x10!=0 {0} else {1}; sun_pci_fd_reset(port); ready }
pub unsafe extern "C" fn sun_pci_fd_set_dma_mode(mode:c_int) { sun_pci_dma_pending.direction=if mode==DMA_MODE_WRITE {DMA_TO_DEVICE} else {DMA_FROM_DEVICE}; ebus_dma_prepare(&mut sun_pci_fd_ebus_dma, (mode!=DMA_MODE_WRITE) as c_int); }
pub unsafe extern "C" fn sun_pci_fd_set_dma_count(length:c_int) { sun_pci_dma_pending.len=length; }
pub unsafe extern "C" fn sun_pci_fd_set_dma_addr(buffer:*mut c_char) { sun_pci_dma_pending.buf=buffer; }
pub unsafe extern "C" fn sun_pci_get_dma_residue()->c_uint { ebus_dma_residue(&mut sun_pci_fd_ebus_dma) }
pub unsafe extern "C" fn sun_pci_fd_request_irq()->c_int { ebus_dma_irq_enable(&mut sun_pci_fd_ebus_dma,1) }
pub unsafe extern "C" fn sun_pci_fd_free_irq() { ebus_dma_irq_enable(&mut sun_pci_fd_ebus_dma,0); }
pub unsafe extern "C" fn sun_pci_fd_eject(_drive:c_int)->c_int { -EINVAL }
pub unsafe extern "C" fn sun_pci_fd_dma_callback(_p:*mut ebus_dma_info,_event:c_int,_cookie:*mut c_void) { floppy_interrupt(0,core::ptr::null_mut()); }

pub const N_FDC: c_int=1; pub const N_DRIVE:c_int=8;
pub const EXTRA_FLOPPY_PARAMS: bool=true;
pub unsafe fn FLOPPY0_TYPE()->c_ulong { sun_floppy_init() }
pub unsafe fn FLOPPY1_TYPE()->c_int { sun_floppy_types[1] }
pub unsafe fn FDC1()->c_ulong { sun_fdc as c_ulong }

pub unsafe extern "C" fn sun_floppy_init()->c_ulong {
    /* The complete probe requires Linux OF/platform structures supplied by the including kernel. */
    0
}

static mut dma_spin_lock: c_ulong=0;
pub unsafe fn claim_dma_lock()->c_ulong { dma_spin_lock }
pub unsafe fn release_dma_lock(_flags:c_ulong) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
