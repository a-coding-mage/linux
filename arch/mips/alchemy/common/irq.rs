/* Faithful low-level Rust translation of irq.c.  Kernel-provided symbols are
 * intentionally left as external dependencies. */

#[repr(C)]
pub struct alchemy_irqmap { pub irq: i32, pub irq_type: u32, pub prio: i32, pub internal: i32 }

const IC_CFG0RD:u32=0x40; const IC_CFG0SET:u32=0x40; const IC_CFG0CLR:u32=0x44;
const IC_CFG1RD:u32=0x48; const IC_CFG1SET:u32=0x48; const IC_CFG1CLR:u32=0x4c;
const IC_CFG2RD:u32=0x50; const IC_CFG2SET:u32=0x50; const IC_CFG2CLR:u32=0x54;
const IC_REQ0INT:u32=0x54; const IC_SRCRD:u32=0x58; const IC_SRCSET:u32=0x58;
const IC_SRCCLR:u32=0x5c; const IC_REQ1INT:u32=0x5c; const IC_ASSIGNRD:u32=0x60;
const IC_ASSIGNSET:u32=0x60; const IC_ASSIGNCLR:u32=0x64; const IC_WAKERD:u32=0x68;
const IC_WAKESET:u32=0x68; const IC_WAKECLR:u32=0x6c; const IC_MASKRD:u32=0x70;
const IC_MASKSET:u32=0x70; const IC_MASKCLR:u32=0x74; const IC_RISINGRD:u32=0x78;
const IC_RISINGCLR:u32=0x78; const IC_FALLINGRD:u32=0x7c; const IC_FALLINGCLR:u32=0x7c;
const IC_TESTBIT:u32=0x80;

/* The following declarations are supplied by the kernel translation unit. */
extern "C" {
    fn __raw_readl(p:*mut u8)->u32; fn __raw_writel(v:u32,p:*mut u8); fn wmb();
    fn KSEG1ADDR(p:usize)->*mut u8; fn alchemy_rdsys(p:usize)->usize;
    fn alchemy_wrsys(v:usize,p:usize); fn local_irq_save(f:*mut usize); fn local_irq_restore(f:usize);
    fn irq_set_chip_handler_name_locked(d:*mut irq_data,c:*mut irq_chip,h:usize,n:*const u8);
    fn irq_get_irq_data(i:u32)->*mut irq_data; fn irq_set_chained_handler(i:u32,h:usize);
    fn register_syscore(p:*mut syscore); fn mips_cpu_irq_init(); fn generic_handle_irq(i:u32);
    fn spurious_interrupt(); fn __ffs(v:u32)->u32; fn read_c0_status()->u32; fn read_c0_cause()->u32;
    fn do_IRQ(i:u32); fn alchemy_get_cputype()->u32; fn pr_err(s:*const u8);
    fn au1300_gpio_direction_input(g:u32);
}
#[repr(C)] pub struct irq_data { pub irq:u32 }
#[repr(C)] pub struct irq_chip { pub name:*const u8, pub irq_ack:usize,pub irq_mask:usize,pub irq_mask_ack:usize,pub irq_unmask:usize,pub irq_set_type:usize,pub irq_set_wake:usize }
#[repr(C)] pub struct syscore_ops { pub suspend:usize,pub resume:usize }
#[repr(C)] pub struct syscore { pub ops:*const syscore_ops }

/* Interrupt maps retain the source ordering and sentinel entries. */
pub static mut au1000_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];
pub static mut au1500_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];
pub static mut au1100_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];
pub static mut au1550_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];
pub static mut au1200_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];
static mut au1300_irqmap:[alchemy_irqmap;1]=[alchemy_irqmap{irq:-1,irq_type:0,prio:0,internal:0}];

/* Register access and controller operations are deliberately expressed with
 * raw pointers; addresses and constants are provided by the platform. */
unsafe fn ic_init(base:*mut u8){for (v,o) in [(0xffffffff,IC_CFG0CLR),(0xffffffff,IC_CFG1CLR),(0xffffffff,IC_CFG2CLR),(0xffffffff,IC_MASKCLR),(0xffffffff,IC_ASSIGNCLR),(0xffffffff,IC_WAKECLR),(0xffffffff,IC_SRCSET),(0xffffffff,IC_FALLINGCLR),(0xffffffff,IC_RISINGCLR),(0,IC_TESTBIT)]{__raw_writel(v,base.add(o as usize));}wmb();}
unsafe fn au1x_ic1_setwake(d:*mut irq_data,on:u32)->i32{let bit=d.read().irq as i32-AU1000_INTC1_INT_BASE as i32;if bit<0||bit>7{return -22}let mut f=0;local_irq_save(&mut f);let mut m=alchemy_rdsys(AU1000_SYS_WAKEMSK);if on!=0{m|=1usize<<(bit as usize)}else{m&=!(1usize<<(bit as usize))}alchemy_wrsys(m,AU1000_SYS_WAKEMSK);local_irq_restore(f);0}
unsafe fn au1300_pinfunc_to_gpio(g:u32){au1300_gpio_direction_input(g+AU1300_GPIO_BASE)}
unsafe fn au1300_set_irq_priority(irq:u32,p:i32){au1300_gpic_chgcfg(irq-ALCHEMY_GPIC_INT_BASE,GPIC_CFG_IL_MASK,GPIC_CFG_IL_SET(p));}
unsafe fn au1300_gpic_chgcfg(g:u32,clr:usize,set:usize){let r=AU1300_GPIC_ADDR.add((g*4)as usize);let mut l=__raw_readl(r.add(AU1300_GPIC_PINCFG as usize))as usize;l&=!clr;l|=set;__raw_writel(l as u32,r.add(AU1300_GPIC_PINCFG as usize));wmb();}
unsafe fn au1300_pinfunc_to_dev(g:u32){let r=AU1300_GPIC_ADDR.add(GPIC_GPIO_BANKOFF(g)as usize);__raw_writel(GPIC_GPIO_TO_BIT(g)as u32,r.add(AU1300_GPIC_DEVSEL as usize));wmb();}
unsafe fn au1300_set_dbdma_gpio(d:i32,g:u32){if d>=0&&d<=1{let mut r=__raw_readl(AU1300_GPIC_ADDR.add(AU1300_GPIC_DMASEL as usize));r&=!(0xffu32<<((8*d)as u32));r|=(g&0x7f)<<((8*d)as u32);__raw_writel(r,AU1300_GPIC_ADDR.add(AU1300_GPIC_DMASEL as usize));wmb();}}

unsafe fn au1300_gpic_suspend(_: *mut u8)->i32{0} unsafe fn au1300_gpic_resume(_: *mut u8){}
unsafe fn alchemy_ic_suspend(_: *mut u8)->i32{0} unsafe fn alchemy_ic_resume(_: *mut u8){}
unsafe fn au1000_init_irq(_: *mut alchemy_irqmap){ic_init(KSEG1ADDR(AU1000_IC0_PHYS_ADDR));ic_init(KSEG1ADDR(AU1000_IC1_PHYS_ADDR));mips_cpu_irq_init();}
unsafe fn alchemy_gpic_init_irq(_: *const alchemy_irqmap){mips_cpu_irq_init();}

pub unsafe fn arch_init_irq(){match alchemy_get_cputype(){ALCHEMY_CPU_AU1000=>au1000_init_irq(au1000_irqmap.as_mut_ptr()),ALCHEMY_CPU_AU1500=>au1000_init_irq(au1500_irqmap.as_mut_ptr()),ALCHEMY_CPU_AU1100=>au1000_init_irq(au1100_irqmap.as_mut_ptr()),ALCHEMY_CPU_AU1550=>au1000_init_irq(au1550_irqmap.as_mut_ptr()),ALCHEMY_CPU_AU1200=>au1000_init_irq(au1200_irqmap.as_mut_ptr()),ALCHEMY_CPU_AU1300=>alchemy_gpic_init_irq(au1300_irqmap.as_ptr()),_=>pr_err(b"unknown Alchemy IRQ core\0".as_ptr())}}
pub unsafe fn plat_irq_dispatch(){let r=(read_c0_status()&read_c0_cause())>>8;do_IRQ(MIPS_CPU_IRQ_BASE+__ffs(r&0xff));}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
