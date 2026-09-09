// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding x86/Linux translation.

use core::ffi::c_void;

extern "C" {
    static mut i8259A_lock: RawSpinlock;
    static mut cached_slave_mask: u8;
    static mut cached_master_mask: u8;
    static mut legacy_pic: *mut LegacyPic;
    static mut dummy_irq_chip: IrqChip;
    fn raw_spin_lock_irqsave(lock: *mut RawSpinlock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinlock, flags: usize);
    fn outb(value: u8, port: u16);
    fn outb_pic(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn disable_irq_nosync(irq: u32);
    fn enable_irq(irq: u32);
    fn irq_set_chip_and_handler(irq: u32, chip: *mut IrqChip, handler: unsafe extern "C" fn());
    fn irq_set_status_flags(irq: u32, flags: u32);
    fn lapic_assign_legacy_vector(irq: u32, arg: bool);
    fn irq_stat_inc_and_enable(irq: u32);
    fn printk_deferred(fmt: *const u8, ...);
    fn printk(fmt: *const u8, ...);
    fn udelay(value: u32);
    fn nr_legacy_irqs() -> i32;
    fn register_syscore(syscore: *mut Syscore);
}

#[repr(C)] pub struct RawSpinlock { _private: [u8; 0] }
#[repr(C)] pub struct IrqData { pub irq: u32 }
#[repr(C)] pub struct IrqChip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut IrqData)>,
}
#[repr(C)] pub struct LegacyPic {
    pub nr_legacy_irqs: i32, pub chip: *mut IrqChip,
    pub mask: Option<unsafe extern "C" fn(u32)>, pub unmask: Option<unsafe extern "C" fn(u32)>,
    pub mask_all: Option<unsafe extern "C" fn()>, pub restore_mask: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn(i32)>, pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub irq_pending: Option<unsafe extern "C" fn(u32) -> i32>, pub make_irq: Option<unsafe extern "C" fn(u32)>,
}
#[repr(C)] pub struct SyscoreOps { pub suspend: Option<unsafe extern "C" fn(*mut c_void)->i32>, pub resume: Option<unsafe extern "C" fn(*mut c_void)>, pub shutdown: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct Syscore { pub ops: *const SyscoreOps }

const PIC_MASTER_IMR: u16 = 0x21; const PIC_SLAVE_IMR: u16 = 0xa1;
const PIC_MASTER_CMD: u16 = 0x20; const PIC_SLAVE_CMD: u16 = 0xa0;
const PIC_ELCR1: u16 = 0x4d0; const PIC_ELCR2: u16 = 0x4d1;
const PIC_CASCADE_IR: u32 = 2; const IRQ_COUNT_PIC_APIC_ERROR: u32 = 0;
const IRQ_LEVEL: u32 = 1; const NR_IRQS_LEGACY: i32 = 16;
const MASTER_ICW4_DEFAULT: u8 = 1; const SLAVE_ICW4_DEFAULT: u8 = 1;
const PIC_ICW4_AEOI: u8 = 2;
extern "C" { fn ISA_IRQ_VECTOR(irq: u32) -> u8; fn handle_level_irq(); }

static mut pcat_compat: bool = false;
static mut i8259A_auto_eoi: i32 = 0;
pub static mut cached_irq_mask: u32 = 0xffff;
pub static mut io_apic_irqs: usize = 0;

unsafe extern "C" fn mask_8259A_irq(irq: u32) { let mask=1u32<<irq; let mut flags=0; raw_spin_lock_irqsave(&mut i8259A_lock,&mut flags); cached_irq_mask|=mask; if irq&8!=0 { outb(cached_slave_mask,PIC_SLAVE_IMR); } else { outb(cached_master_mask,PIC_MASTER_IMR); } raw_spin_unlock_irqrestore(&mut i8259A_lock,flags); }
unsafe extern "C" fn disable_8259A_irq(data:*mut IrqData) { mask_8259A_irq((*data).irq); }
unsafe extern "C" fn unmask_8259A_irq(irq:u32) { let mask=!(1u32<<irq); let mut flags=0; raw_spin_lock_irqsave(&mut i8259A_lock,&mut flags); cached_irq_mask&=mask; if irq&8!=0 {outb(cached_slave_mask,PIC_SLAVE_IMR);} else {outb(cached_master_mask,PIC_MASTER_IMR);} raw_spin_unlock_irqrestore(&mut i8259A_lock,flags); }
unsafe extern "C" fn enable_8259A_irq(data:*mut IrqData) {unmask_8259A_irq((*data).irq);}
unsafe extern "C" fn i8259A_irq_pending(irq:u32)->i32 {let mask=1<<irq;let mut f=0;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);let r=if irq<8 {(inb(PIC_MASTER_CMD) as u32&mask) as i32}else{(inb(PIC_SLAVE_CMD) as u32&(mask>>8)) as i32};raw_spin_unlock_irqrestore(&mut i8259A_lock,f);r}
unsafe extern "C" fn i8259A_irq_real(irq:u32)->i32 {let m=1<<irq;if irq<8{outb(0xb,PIC_MASTER_CMD);let v=(inb(PIC_MASTER_CMD) as u32&m)as i32;outb(0xa,PIC_MASTER_CMD);v}else{outb(0xb,PIC_SLAVE_CMD);let v=(inb(PIC_SLAVE_CMD)as u32&(m>>8))as i32;outb(0xa,PIC_SLAVE_CMD);v}}
unsafe extern "C" fn mask_and_ack_8259A(data:*mut IrqData){let irq=(*data).irq;let m=1<<irq;let mut f=0;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);if cached_irq_mask&m!=0{if i8259A_irq_real(irq)==0{static mut SPURIOUS:u32=0;if SPURIOUS&m==0{printk_deferred(b"spurious 8259A interrupt: IRQ%d.\0".as_ptr(),irq);SPURIOUS|=m;}irq_stat_inc_and_enable(IRQ_COUNT_PIC_APIC_ERROR);}}cached_irq_mask|=m;if irq&8!=0{inb(PIC_SLAVE_IMR);outb(cached_slave_mask,PIC_SLAVE_IMR);outb(0x60+(irq&7)as u8,PIC_SLAVE_CMD);outb(0x60+PIC_CASCADE_IR as u8,PIC_MASTER_CMD);}else{inb(PIC_MASTER_IMR);outb(cached_master_mask,PIC_MASTER_IMR);outb(0x60+irq as u8,PIC_MASTER_CMD);}raw_spin_unlock_irqrestore(&mut i8259A_lock,f);}

pub static mut i8259A_chip: IrqChip=IrqChip{name:b"XT-PIC\0".as_ptr(),irq_mask:Some(disable_8259A_irq),irq_disable:Some(disable_8259A_irq),irq_unmask:Some(enable_8259A_irq),irq_mask_ack:Some(mask_and_ack_8259A)};
static mut irq_trigger:[u8;2]=[0;2];
unsafe extern "C" fn restore_ELCR(t:*mut u8){outb(*t,PIC_ELCR1);outb(*t.add(1),PIC_ELCR2)}
unsafe extern "C" fn save_ELCR(t:*mut u8){*t=inb(PIC_ELCR1)&0xf8;*t.add(1)=inb(PIC_ELCR2)&0xde;}
unsafe extern "C" fn i8259A_shutdown(_: *mut c_void){outb(0xff,PIC_MASTER_IMR);outb(0xff,PIC_SLAVE_IMR)}
unsafe extern "C" fn i8259A_resume(_: *mut c_void){init_8259A(i8259A_auto_eoi);restore_ELCR(irq_trigger.as_mut_ptr());}
unsafe extern "C" fn i8259A_suspend(_: *mut c_void)->i32{save_ELCR(irq_trigger.as_mut_ptr());0}
static i8259_syscore_ops:SyscoreOps=SyscoreOps{suspend:Some(i8259A_suspend),resume:Some(i8259A_resume),shutdown:Some(i8259A_shutdown)};
static mut i8259_syscore:Syscore=Syscore{ops:&i8259_syscore_ops};
unsafe extern "C" fn mask_8259A(){let mut f=0;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);outb(0xff,PIC_MASTER_IMR);outb(0xff,PIC_SLAVE_IMR);raw_spin_unlock_irqrestore(&mut i8259A_lock,f)}
unsafe extern "C" fn unmask_8259A(){let mut f=0;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);outb(cached_master_mask,PIC_MASTER_IMR);outb(cached_slave_mask,PIC_SLAVE_IMR);raw_spin_unlock_irqrestore(&mut i8259A_lock,f)}
unsafe extern "C" fn probe_8259A()->i32{let probe=!(1u8<<PIC_CASCADE_IR);if pcat_compat{return nr_legacy_irqs()}let mut f=0;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);outb(0xff,PIC_SLAVE_IMR);outb(probe,PIC_MASTER_IMR);if inb(PIC_MASTER_IMR)!=probe{printk(b"Using NULL legacy PIC\0".as_ptr());legacy_pic=&mut null_legacy_pic;}raw_spin_unlock_irqrestore(&mut i8259A_lock,f);nr_legacy_irqs()}
unsafe extern "C" fn init_8259A(auto_eoi:i32){let mut f=0;i8259A_auto_eoi=auto_eoi;raw_spin_lock_irqsave(&mut i8259A_lock,&mut f);outb(0xff,PIC_MASTER_IMR);outb_pic(0x11,PIC_MASTER_CMD);outb_pic(ISA_IRQ_VECTOR(0),PIC_MASTER_IMR);outb_pic(1<<PIC_CASCADE_IR,PIC_MASTER_IMR);outb_pic(MASTER_ICW4_DEFAULT|if auto_eoi!=0{PIC_ICW4_AEOI}else{0},PIC_MASTER_IMR);outb_pic(0x11,PIC_SLAVE_CMD);outb_pic(ISA_IRQ_VECTOR(8),PIC_SLAVE_IMR);outb_pic(PIC_CASCADE_IR as u8,PIC_SLAVE_IMR);outb_pic(SLAVE_ICW4_DEFAULT,PIC_SLAVE_IMR);i8259A_chip.irq_mask_ack=Some(if auto_eoi!=0{disable_8259A_irq}else{mask_and_ack_8259A});udelay(100);outb(cached_master_mask,PIC_MASTER_IMR);outb(cached_slave_mask,PIC_SLAVE_IMR);raw_spin_unlock_irqrestore(&mut i8259A_lock,f)}
unsafe extern "C" fn legacy_pic_noop(){}
unsafe extern "C" fn legacy_pic_uint_noop(_:u32){}
unsafe extern "C" fn legacy_pic_int_noop(_:i32){}
unsafe extern "C" fn legacy_pic_irq_pending_noop(_:u32)->i32{0}
unsafe extern "C" fn legacy_pic_probe()->i32{0}
unsafe extern "C" fn make_8259A_irq(irq:u32){disable_irq_nosync(irq);io_apic_irqs&=!(1usize<<irq);irq_set_chip_and_handler(irq,&mut i8259A_chip,handle_level_irq);irq_set_status_flags(irq,IRQ_LEVEL);enable_irq(irq);lapic_assign_legacy_vector(irq,true)}
pub static mut null_legacy_pic:LegacyPic=LegacyPic{nr_legacy_irqs:0,chip:&mut dummy_irq_chip,mask:Some(legacy_pic_uint_noop),unmask:Some(legacy_pic_uint_noop),mask_all:Some(legacy_pic_noop),restore_mask:Some(legacy_pic_noop),init:Some(legacy_pic_int_noop),probe:Some(legacy_pic_probe),irq_pending:Some(legacy_pic_irq_pending_noop),make_irq:Some(legacy_pic_uint_noop)};
static mut default_legacy_pic:LegacyPic=LegacyPic{nr_legacy_irqs:NR_IRQS_LEGACY,chip:&mut i8259A_chip,mask:Some(mask_8259A_irq),unmask:Some(unmask_8259A_irq),mask_all:Some(mask_8259A),restore_mask:Some(unmask_8259A),init:Some(init_8259A),probe:Some(probe_8259A),irq_pending:Some(i8259A_irq_pending),make_irq:Some(make_8259A_irq)};
pub static mut legacy_pic_ptr:*mut LegacyPic=unsafe{&mut default_legacy_pic};
unsafe extern "C" fn i8259A_init_ops()->i32{register_syscore(&mut i8259_syscore);0}
#[no_mangle] pub unsafe extern "C" fn legacy_pic_pcat_compat(){pcat_compat=true;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
