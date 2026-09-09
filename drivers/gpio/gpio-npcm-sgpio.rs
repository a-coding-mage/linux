// SPDX-License-Identifier: GPL-2.0
/* Nuvoton NPCM Serial GPIO Driver */

const MAX_NR_HW_SGPIO: usize = 64;
const NPCM_IOXCFG1: usize = 0x2A;
const NPCM_IOXCFG1_SFT_CLK: u8 = 0x0f;
const NPCM_IOXCFG1_SCLK_POL: u8 = 1 << 4;
const NPCM_IOXCFG1_LDSH_POL: u8 = 1 << 5;
const NPCM_IOXCTS: usize = 0x28;
const NPCM_IOXCTS_IOXIF_EN: u8 = 1 << 7;
const NPCM_IOXCTS_RD_MODE: u8 = 0x06;
const NPCM_IOXCTS_RD_MODE_PERIODIC: u8 = 1 << 2;
const NPCM_IOXCFG2: usize = 0x2B;
const NPCM_IOXCFG2_PORT: u8 = 0x0f;
const NPCM_IXOEVCFG_MASK: u16 = 0x03;
const NPCM_IXOEVCFG_FALLING: u16 = 1 << 1;
const NPCM_IXOEVCFG_RISING: u16 = 1;
const NPCM_IXOEVCFG_BOTH: u16 = NPCM_IXOEVCFG_FALLING | NPCM_IXOEVCFG_RISING;
const NPCM_CLK_MHZ: u64 = 8 * HZ_PER_MHZ;
const NPCM_750_OPT: usize = 6;
const NPCM_845_OPT: usize = 5;

const fn gpio_bank(x: u32) -> usize { (x / 8) as usize }
const fn gpio_bit(x: u32) -> u8 { (x % 8) as u8 }

#[repr(C)]
pub struct npcm_clk_cfg { pub sft_clk: *mut u32, pub clk_sel: *mut u32, pub cfg_opt: u32 }
#[repr(C)]
pub struct npcm_sgpio {
    pub chip: gpio_chip, pub pclk: *mut clk, pub intc: irq_chip, pub lock: raw_spinlock_t,
    pub base: *mut u8, pub irq: i32, pub nin_sgpio: u8, pub nout_sgpio: u8,
    pub in_port: u8, pub out_port: u8, pub int_type: [u8; MAX_NR_HW_SGPIO],
}
#[repr(C)]
pub struct npcm_sgpio_bank { pub rdata_reg: u8, pub wdata_reg: u8, pub event_config: u8, pub event_status: u8 }
#[repr(C)]
pub struct gpio_chip { pub parent: *mut device, pub ngpio: u32, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip,u32,i32)->i32>, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub get: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip,u32,i32)->i32>, pub label: *const u8, pub base: i32, pub irq: gpio_irq_chip }
#[repr(C)] pub struct gpio_irq_chip { pub chip: *const irq_chip, pub init_valid_mask: Option<unsafe extern "C" fn(*mut gpio_chip,*mut usize,u32)>, pub handler: usize, pub default_type: u32, pub parent_handler: usize, pub parent_handler_data: *mut npcm_sgpio, pub parents: *mut i32, pub num_parents: u32 }
#[repr(C)] pub struct irq_chip { pub name: *const u8 }
#[repr(C)] pub struct irq_data { pub dummy: usize }
#[repr(C)] pub struct irq_desc { pub dummy: usize }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub dummy: usize }
#[repr(C)] pub struct clk { pub dummy: usize }
#[repr(C)] pub struct raw_spinlock_t { pub dummy: usize }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8, pub data: *const npcm_clk_cfg }
#[repr(C)] pub struct platform_driver { pub dummy: usize }

#[repr(u32)] pub enum npcm_sgpio_reg { READ_DATA, WRITE_DATA, EVENT_CFG, EVENT_STS }
static NPCM_SGPIO_BANKS: [npcm_sgpio_bank; 8] = [
    npcm_sgpio_bank{wdata_reg:0x00,rdata_reg:0x08,event_config:0x10,event_status:0x20},
    npcm_sgpio_bank{wdata_reg:0x01,rdata_reg:0x09,event_config:0x12,event_status:0x21},
    npcm_sgpio_bank{wdata_reg:0x02,rdata_reg:0x0a,event_config:0x14,event_status:0x22},
    npcm_sgpio_bank{wdata_reg:0x03,rdata_reg:0x0b,event_config:0x16,event_status:0x23},
    npcm_sgpio_bank{wdata_reg:0x04,rdata_reg:0x0c,event_config:0x18,event_status:0x24},
    npcm_sgpio_bank{wdata_reg:0x05,rdata_reg:0x0d,event_config:0x1a,event_status:0x25},
    npcm_sgpio_bank{wdata_reg:0x06,rdata_reg:0x0e,event_config:0x1c,event_status:0x26},
    npcm_sgpio_bank{wdata_reg:0x07,rdata_reg:0x0f,event_config:0x1e,event_status:0x27},
];

unsafe fn bank_reg(gpio: *mut npcm_sgpio, bank: *const npcm_sgpio_bank, reg: npcm_sgpio_reg) -> *mut u8 {
    let off = match reg { npcm_sgpio_reg::READ_DATA => (*bank).rdata_reg, npcm_sgpio_reg::WRITE_DATA => (*bank).wdata_reg, npcm_sgpio_reg::EVENT_CFG => (*bank).event_config, npcm_sgpio_reg::EVENT_STS => (*bank).event_status };
    (*gpio).base.add(off as usize)
}
unsafe fn offset_to_bank(offset: u32) -> *const npcm_sgpio_bank { &NPCM_SGPIO_BANKS[gpio_bank(offset)] }
unsafe fn npcm_sgpio_irqd_to_data(d:*mut irq_data, gpio:*mut *mut npcm_sgpio, bank:*mut *const npcm_sgpio_bank, bit:*mut u8, offset:*mut u32) { *offset=irqd_to_hwirq(d); *gpio=irq_data_get_irq_chip_data(d); *offset-=(*(*gpio)).nout_sgpio as u32; *bank=offset_to_bank(*offset); *bit=gpio_bit(*offset); }

unsafe fn npcm_sgpio_init_port(gpio:*mut npcm_sgpio)->i32 { let mut inp=gpio_bank((*gpio).nin_sgpio as u32) as u8; if gpio_bit((*gpio).nin_sgpio as u32)>0 {inp+=1;} let mut outp=gpio_bank((*gpio).nout_sgpio as u32) as u8; if gpio_bit((*gpio).nout_sgpio as u32)>0 {outp+=1;} (*gpio).in_port=inp; (*gpio).out_port=outp; let setp=((outp&NPCM_IOXCFG2_PORT)<<4)|(inp&NPCM_IOXCFG2_PORT); iowrite8(setp,(*gpio).base.add(NPCM_IOXCFG2)); if ioread8((*gpio).base.add(NPCM_IOXCFG2))==setp {0} else {-EINVAL} }
unsafe fn npcm_sgpio_dir_in(gc:*mut gpio_chip, offset:u32)->i32 { let g=gpiochip_get_data(gc); if offset<(*g).nout_sgpio as u32 {-EINVAL} else {0} }
unsafe fn npcm_sgpio_dir_out(gc:*mut gpio_chip, offset:u32, val:i32)->i32 { npcm_sgpio_set(gc,offset,val) }
unsafe fn npcm_sgpio_get_direction(gc:*mut gpio_chip, offset:u32)->i32 { let g=gpiochip_get_data(gc); if offset<(*g).nout_sgpio as u32 {GPIO_LINE_DIRECTION_OUT} else {GPIO_LINE_DIRECTION_IN} }
unsafe fn npcm_sgpio_set(gc:*mut gpio_chip, offset:u32, val:i32)->i32 { let g=gpiochip_get_data(gc); let b=offset_to_bank(offset); let a=bank_reg(g,b,npcm_sgpio_reg::WRITE_DATA); let mut r=ioread8(a); if val!=0 {r|=1<<gpio_bit(offset);} else {r&=!(1<<gpio_bit(offset));} iowrite8(r,a); 0 }
unsafe fn npcm_sgpio_get(gc:*mut gpio_chip, mut offset:u32)->i32 { let g=gpiochip_get_data(gc); let b; if offset<(*g).nout_sgpio as u32 {b=offset_to_bank(offset);} else {offset-=(*g).nout_sgpio as u32; b=offset_to_bank(offset);} let a=bank_reg(g,b,if offset<(*g).nout_sgpio as u32 {npcm_sgpio_reg::WRITE_DATA} else {npcm_sgpio_reg::READ_DATA}); ((ioread8(a)&(1<<gpio_bit(offset)))!=0) as i32 }
unsafe fn npcm_sgpio_setup_enable(g:*mut npcm_sgpio, enable:bool) { let mut r=ioread8((*g).base.add(NPCM_IOXCTS)); r=(r&!NPCM_IOXCTS_RD_MODE)|NPCM_IOXCTS_RD_MODE_PERIODIC; if enable {r|=NPCM_IOXCTS_IOXIF_EN;} else {r&=!NPCM_IOXCTS_IOXIF_EN;} iowrite8(r,(*g).base.add(NPCM_IOXCTS)); }
unsafe fn npcm_sgpio_setup_clk(g:*mut npcm_sgpio,c:*const npcm_clk_cfg)->i32 { let apb=clk_get_rate((*g).pclk); let tmp=ioread8((*g).base.add(NPCM_IOXCFG1))&!NPCM_IOXCFG1_SFT_CLK; let mut i=(*c).cfg_opt as isize-1; while i>0 {let v=apb/(*(*c).sft_clk.offset(i)); if NPCM_CLK_MHZ>v {iowrite8((*(*c).clk_sel.offset(i)) as u8|tmp,(*g).base.add(NPCM_IOXCFG1)); return 0;} i-=1;} -EINVAL }

unsafe fn npcm_sgpio_irq_init_valid_mask(gc:*mut gpio_chip, valid:*mut usize, _ngpios:u32) { let g=gpiochip_get_data(gc); bitmap_set(valid,(*g).nout_sgpio as u32,(*g).nin_sgpio as u32); bitmap_clear(valid,0,(*g).nout_sgpio as u32); }
unsafe fn npcm_sgpio_irq_set_mask(d:*mut irq_data,set:bool) { let mut g=core::ptr::null_mut(); let mut b=core::ptr::null(); let mut bit=0; let mut off=0; npcm_sgpio_irqd_to_data(d,&mut g,&mut b,&mut bit,&mut off); let a=bank_reg(g,b,npcm_sgpio_reg::EVENT_CFG); let mut r=ioread16(a); if set {r&=!(NPCM_IXOEVCFG_MASK<<(bit*2));} else {r|=((*g).int_type[off as usize] as u16)<<(bit*2);} let mut flags=0; raw_spin_lock_irqsave(&mut (*g).lock,&mut flags); npcm_sgpio_setup_enable(g,false); iowrite16(r,a); npcm_sgpio_setup_enable(g,true); let s=bank_reg(g,b,npcm_sgpio_reg::EVENT_STS); let mut q=ioread8(s); q|=1<<bit; iowrite8(q,s); raw_spin_unlock_irqrestore(&mut (*g).lock,flags); }
unsafe fn npcm_sgpio_irq_ack(d:*mut irq_data) { let mut g=core::ptr::null_mut(); let mut b=core::ptr::null(); let mut bit=0; let mut off=0; npcm_sgpio_irqd_to_data(d,&mut g,&mut b,&mut bit,&mut off); let a=bank_reg(g,b,npcm_sgpio_reg::EVENT_STS); let mut f=0; raw_spin_lock_irqsave(&mut (*g).lock,&mut f); iowrite8(1<<bit,a); raw_spin_unlock_irqrestore(&mut (*g).lock,f); }
unsafe fn npcm_sgpio_irq_mask(d:*mut irq_data){npcm_sgpio_irq_set_mask(d,true)}
unsafe fn npcm_sgpio_irq_unmask(d:*mut irq_data){npcm_sgpio_irq_set_mask(d,false)}
unsafe fn npcm_sgpio_set_type(d:*mut irq_data, typ:u32)->i32 { let mut g=core::ptr::null_mut(); let mut b=core::ptr::null(); let mut bit=0; let mut off=0; npcm_sgpio_irqd_to_data(d,&mut g,&mut b,&mut bit,&mut off); let val=match typ&IRQ_TYPE_SENSE_MASK {IRQ_TYPE_EDGE_BOTH=>NPCM_IXOEVCFG_BOTH,IRQ_TYPE_EDGE_RISING|IRQ_TYPE_LEVEL_HIGH=>NPCM_IXOEVCFG_RISING,IRQ_TYPE_EDGE_FALLING|IRQ_TYPE_LEVEL_LOW=>NPCM_IXOEVCFG_FALLING,_=>return -EINVAL}; (*g).int_type[off as usize]=val as u8; let mut f=0; raw_spin_lock_irqsave(&mut (*g).lock,&mut f); npcm_sgpio_setup_enable(g,false); let a=bank_reg(g,b,npcm_sgpio_reg::EVENT_CFG); let r=ioread16(a)|(val<<(bit*2)); iowrite16(r,a); npcm_sgpio_setup_enable(g,true); raw_spin_unlock_irqrestore(&mut (*g).lock,f); irq_set_handler_locked(d,if typ&IRQ_TYPE_LEVEL_MASK!=0 {handle_level_irq} else {handle_edge_irq}); 0 }
unsafe fn npcm_sgpio_irq_handler(desc:*mut irq_desc) { let gc=irq_desc_get_handler_data(desc); let ic=irq_desc_get_chip(desc); let g=gpiochip_get_data(gc); chained_irq_enter(ic,desc); for i in 0..NPCM_SGPIO_BANKS.len() {let b=&NPCM_SGPIO_BANKS[i]; let mut r=ioread8(bank_reg(g,b,npcm_sgpio_reg::EVENT_STS)); for j in 0..8 {if r&(1<<j)!=0 {generic_handle_domain_irq((*gc).irq.domain,i as u32*8+(*g).nout_sgpio as u32+j); r&=!(1<<j);}}} chained_irq_exit(ic,desc); }

static mut NPCM750_SFT_CLK:[u32;NPCM_750_OPT]=[1024,32,8,4,3,2];
static mut NPCM750_CLK_SEL:[u32;NPCM_750_OPT]=[0,5,7,0x0c,0x0d,0x0e];
static mut NPCM845_SFT_CLK:[u32;NPCM_845_OPT]=[1024,32,16,8,4];
static mut NPCM845_CLK_SEL:[u32;NPCM_845_OPT]=[0,5,6,7,0x0c];
static mut NPCM750_SGPIO_PDATA:npcm_clk_cfg=npcm_clk_cfg{sft_clk:core::ptr::null_mut(),clk_sel:core::ptr::null_mut(),cfg_opt:NPCM_750_OPT as u32};
static NPCM845_SGPIO_PDATA:npcm_clk_cfg=npcm_clk_cfg{sft_clk:core::ptr::null_mut(),clk_sel:core::ptr::null_mut(),cfg_opt:NPCM_845_OPT as u32};
// C device-tree match table, platform-driver registration, and MODULE_* metadata are preserved as external kernel integration points.
unsafe fn npcm_sgpio_setup_irqs(g:*mut npcm_sgpio,p:*mut platform_device)->i32 { let rc=platform_get_irq(p,0); if rc<0{return rc;} (*g).irq=rc; npcm_sgpio_setup_enable(g,false); for b in NPCM_SGPIO_BANKS.iter(){iowrite16(0,bank_reg(g,b,npcm_sgpio_reg::EVENT_CFG));iowrite8(0xff,bank_reg(g,b,npcm_sgpio_reg::EVENT_STS));} let q=&mut (*g).chip.irq; gpio_irq_chip_set_chip(q,core::ptr::null()); q.init_valid_mask=Some(npcm_sgpio_irq_init_valid_mask); q.handler=handle_bad_irq; q.default_type=IRQ_TYPE_NONE; q.parent_handler=npcm_sgpio_irq_handler as usize; q.parent_handler_data=g; q.parents=&mut (*g).irq; q.num_parents=1; 0 }
unsafe fn npcm_sgpio_probe(p:*mut platform_device)->i32 { let g=devm_kzalloc(&mut (*p).dev,core::mem::size_of::<npcm_sgpio>(),GFP_KERNEL); if g.is_null(){return -ENOMEM;} (*g).base=devm_platform_ioremap_resource(p,0); if is_err((*g).base){return ptr_err((*g).base);} let nin=device_property_read_u32(&mut (*p).dev,b"nuvoton,input-ngpios\0".as_ptr(),&mut 0); if nin<0{return nin;} (*g).nin_sgpio=0; (*g).nout_sgpio=0; (*g).pclk=devm_clk_get(&mut (*p).dev,core::ptr::null()); if is_err((*g).pclk){return ptr_err((*g).pclk);} raw_spin_lock_init(&mut (*g).lock); if npcm_sgpio_init_port(g)<0{return -EINVAL;} if npcm_sgpio_setup_irqs(g,p)<0{return -EINVAL;} let rc=devm_gpiochip_add_data(&mut (*p).dev,&mut (*g).chip,g); if rc!=0{return rc;} npcm_sgpio_setup_enable(g,true); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
