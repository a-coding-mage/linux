// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 IBM Corp.
 *
 * Joel Stanley <joel@jms.id.au>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const GPIO_G7_IRQ_STS_BASE: usize = 0x100;
const GPIO_G7_CTRL_REG_BASE: usize = 0x180;
const GPIO_G7_CTRL_OUT_DATA: u32 = 1 << 0;
const GPIO_G7_CTRL_DIR: u32 = 1 << 1;
const GPIO_G7_CTRL_IRQ_EN: u32 = 1 << 2;
const GPIO_G7_CTRL_IRQ_TYPE0: u32 = 1 << 3;
const GPIO_G7_CTRL_IRQ_TYPE1: u32 = 1 << 4;
const GPIO_G7_CTRL_IRQ_TYPE2: u32 = 1 << 5;
const GPIO_G7_CTRL_RST_TOLERANCE: u32 = 1 << 6;
const GPIO_G7_CTRL_DEBOUNCE_SEL2: u32 = 1 << 7;
const GPIO_G7_CTRL_DEBOUNCE_SEL1: u32 = 1 << 8;
const GPIO_G7_CTRL_INPUT_MASK: u32 = 1 << 9;
const GPIO_G7_CTRL_IRQ_STS: u32 = 1 << 12;
const GPIO_G7_CTRL_IN_DATA: u32 = 1 << 13;

#[repr(C)]
struct aspeed_bank_props { bank: u32, input: u32, output: u32 }

#[repr(C)]
struct aspeed_gpio_config {
    nr_gpios: u32,
    props: *const aspeed_bank_props,
    llops: *const aspeed_gpio_llops,
    debounce_timers_array: *const i32,
    debounce_timers_num: i32,
    require_dcache: bool,
}

#[repr(C)]
struct aspeed_gpio {
    chip: gpio_chip,
    dev: *mut device,
    lock: raw_spinlock_t,
    base: *mut core::ffi::c_void,
    irq: i32,
    config: *const aspeed_gpio_config,
    offset_timer: *mut u8,
    timer_users: [u32; 4],
    clk: *mut clk,
    dcache: *mut u32,
    cf_copro_bankmap: *mut u8,
}

#[repr(C)]
struct aspeed_gpio_bank { val_regs: u16, rdata_reg: u16, irq_regs: u16, debounce_regs: u16, tolerance_regs: u16, cmdsrc_regs: u16 }

static debounce_timers: [i32; 4] = [0x00, 0x50, 0x54, 0x58];
static g7_debounce_timers: [i32; 4] = [0x00, 0x00, 0x04, 0x08];

/* The debounce timers array maps debounce settings to timer register offsets. */
static mut copro_ops: *const aspeed_gpio_copro_ops = core::ptr::null();
static mut copro_data: *mut core::ffi::c_void = core::ptr::null_mut();

static aspeed_gpio_banks: [aspeed_gpio_bank; 8] = [
    aspeed_gpio_bank { val_regs:0x0000,rdata_reg:0x00c0,irq_regs:0x0008,debounce_regs:0x0040,tolerance_regs:0x001c,cmdsrc_regs:0x0060 },
    aspeed_gpio_bank { val_regs:0x0020,rdata_reg:0x00c4,irq_regs:0x0028,debounce_regs:0x0048,tolerance_regs:0x003c,cmdsrc_regs:0x0068 },
    aspeed_gpio_bank { val_regs:0x0070,rdata_reg:0x00c8,irq_regs:0x0098,debounce_regs:0x00b0,tolerance_regs:0x00ac,cmdsrc_regs:0x0090 },
    aspeed_gpio_bank { val_regs:0x0078,rdata_reg:0x00cc,irq_regs:0x00e8,debounce_regs:0x0100,tolerance_regs:0x00fc,cmdsrc_regs:0x00e0 },
    aspeed_gpio_bank { val_regs:0x0080,rdata_reg:0x00d0,irq_regs:0x0118,debounce_regs:0x0130,tolerance_regs:0x012c,cmdsrc_regs:0x0110 },
    aspeed_gpio_bank { val_regs:0x0088,rdata_reg:0x00d4,irq_regs:0x0148,debounce_regs:0x0160,tolerance_regs:0x015c,cmdsrc_regs:0x0140 },
    aspeed_gpio_bank { val_regs:0x01e0,rdata_reg:0x00d8,irq_regs:0x0178,debounce_regs:0x0190,tolerance_regs:0x018c,cmdsrc_regs:0x0170 },
    aspeed_gpio_bank { val_regs:0x01e8,rdata_reg:0x00dc,irq_regs:0x01a8,debounce_regs:0x01c0,tolerance_regs:0x01bc,cmdsrc_regs:0x01a0 },
];

#[repr(C)]
enum aspeed_gpio_reg { reg_val, reg_rdata, reg_dir, reg_irq_enable, reg_irq_type0, reg_irq_type1, reg_irq_type2, reg_irq_status, reg_debounce_sel1, reg_debounce_sel2, reg_tolerance, reg_cmdsrc0, reg_cmdsrc1 }

#[repr(C)]
struct aspeed_gpio_llops {
    reg_bit_set: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32,aspeed_gpio_reg,bool)>,
    reg_bit_get: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32,aspeed_gpio_reg)->bool>,
    reg_bank_get: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32,aspeed_gpio_reg)->i32>,
    privilege_ctrl: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32,i32)>,
    privilege_init: Option<unsafe extern "C" fn(*mut aspeed_gpio)>,
    copro_request: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32)->bool>,
    copro_release: Option<unsafe extern "C" fn(*mut aspeed_gpio,u32)>,
}

const GPIO_VAL_VALUE: usize=0; const GPIO_VAL_DIR: usize=4;
const GPIO_IRQ_ENABLE: usize=0; const GPIO_IRQ_TYPE0: usize=4; const GPIO_IRQ_TYPE1: usize=8; const GPIO_IRQ_TYPE2: usize=12; const GPIO_IRQ_STATUS: usize=16;
const GPIO_DEBOUNCE_SEL1: usize=0; const GPIO_DEBOUNCE_SEL2: usize=4;
const GPIO_CMDSRC_0: usize=0; const GPIO_CMDSRC_1: usize=4;
const GPIO_CMDSRC_ARM: i32=0; const GPIO_CMDSRC_LPC: i32=1; const GPIO_CMDSRC_COLDFIRE: i32=2; const GPIO_CMDSRC_RESERVED: i32=3;

#[inline] fn gpio_bank(x:u32)->usize {(x>>5) as usize}
#[inline] fn gpio_offset(x:u32)->u32 {x&0x1f}
#[inline] fn gpio_bit(x:u32)->u32 {1<<gpio_offset(x)}

unsafe fn aspeed_gpio_g4_bank_reg(gpio:*mut aspeed_gpio, bank:*const aspeed_gpio_bank, reg:aspeed_gpio_reg)->*mut u8 {
    let b=(*bank); let off=match reg { aspeed_gpio_reg::reg_val=>b.val_regs as usize+GPIO_VAL_VALUE, aspeed_gpio_reg::reg_rdata=>b.rdata_reg as usize, aspeed_gpio_reg::reg_dir=>b.val_regs as usize+GPIO_VAL_DIR, aspeed_gpio_reg::reg_irq_enable=>b.irq_regs as usize+GPIO_IRQ_ENABLE, aspeed_gpio_reg::reg_irq_type0=>b.irq_regs as usize+GPIO_IRQ_TYPE0, aspeed_gpio_reg::reg_irq_type1=>b.irq_regs as usize+GPIO_IRQ_TYPE1, aspeed_gpio_reg::reg_irq_type2=>b.irq_regs as usize+GPIO_IRQ_TYPE2, aspeed_gpio_reg::reg_irq_status=>b.irq_regs as usize+GPIO_IRQ_STATUS, aspeed_gpio_reg::reg_debounce_sel1=>b.debounce_regs as usize+GPIO_DEBOUNCE_SEL1, aspeed_gpio_reg::reg_debounce_sel2=>b.debounce_regs as usize+GPIO_DEBOUNCE_SEL2, aspeed_gpio_reg::reg_tolerance=>b.tolerance_regs as usize, aspeed_gpio_reg::reg_cmdsrc0=>b.cmdsrc_regs as usize+GPIO_CMDSRC_0, aspeed_gpio_reg::reg_cmdsrc1=>b.cmdsrc_regs as usize+GPIO_CMDSRC_1 };
    (*gpio).base.add(off)
}

unsafe fn aspeed_gpio_g7_reg_mask(reg:aspeed_gpio_reg)->u32 { match reg { aspeed_gpio_reg::reg_val|aspeed_gpio_reg::reg_rdata=>GPIO_G7_CTRL_OUT_DATA, aspeed_gpio_reg::reg_dir=>GPIO_G7_CTRL_DIR, aspeed_gpio_reg::reg_irq_enable=>GPIO_G7_CTRL_IRQ_EN, aspeed_gpio_reg::reg_irq_type0=>GPIO_G7_CTRL_IRQ_TYPE0, aspeed_gpio_reg::reg_irq_type1=>GPIO_G7_CTRL_IRQ_TYPE1, aspeed_gpio_reg::reg_irq_type2=>GPIO_G7_CTRL_IRQ_TYPE2, aspeed_gpio_reg::reg_tolerance=>GPIO_G7_CTRL_RST_TOLERANCE, aspeed_gpio_reg::reg_debounce_sel1=>GPIO_G7_CTRL_DEBOUNCE_SEL1, aspeed_gpio_reg::reg_debounce_sel2=>GPIO_G7_CTRL_DEBOUNCE_SEL2, aspeed_gpio_reg::reg_irq_status=>GPIO_G7_CTRL_IRQ_STS, _=>0 } }

unsafe fn to_bank(offset:u32)->*const aspeed_gpio_bank { &aspeed_gpio_banks[gpio_bank(offset)] }

/* Unspecified banks have full input and output capability. */
unsafe fn find_bank_props(gpio:*mut aspeed_gpio, offset:u32)->*const aspeed_bank_props { let mut p=(*(*gpio).config).props; while (*p).input!=0 || (*p).output!=0 { if (*p).bank==gpio_bank(offset) as u32{return p;} p=p.add(1); } core::ptr::null() }
unsafe fn have_gpio(g:*mut aspeed_gpio,o:u32)->bool { if o>=(*g).chip.ngpio as u32{return false}; let p=find_bank_props(g,o); p.is_null() || (((*p).input|(*p).output)&gpio_bit(o))!=0 }
unsafe fn have_input(g:*mut aspeed_gpio,o:u32)->bool {let p=find_bank_props(g,o);p.is_null()||((*p).input&gpio_bit(o))!=0}
unsafe fn have_output(g:*mut aspeed_gpio,o:u32)->bool {let p=find_bank_props(g,o);p.is_null()||((*p).output&gpio_bit(o))!=0}

/* The remaining driver callbacks retain the kernel ABI and are declared in the same low-level form. */
unsafe fn aspeed_gpio_copro_request(g:*mut aspeed_gpio,o:u32)->bool { (*(*g).config).llops.as_ref().and_then(|l|l.copro_request).map(|f|f(g,o)).unwrap_or(false) }
unsafe fn aspeed_gpio_copro_release(g:*mut aspeed_gpio,o:u32) { if let Some(f)=(*(*g).config).llops.as_ref().and_then(|l|l.copro_release){f(g,o)} }
unsafe fn aspeed_gpio_support_copro(g:*mut aspeed_gpio)->bool { let l=&*(*g).config; let x=&*l.llops; x.copro_request.is_some()&&x.copro_release.is_some()&&x.privilege_ctrl.is_some()&&x.privilege_init.is_some() }

/* Direct translations of the exported coprocessor API. */
#[no_mangle] pub unsafe extern "C" fn aspeed_gpio_copro_set_ops(ops:*const aspeed_gpio_copro_ops,data:*mut core::ffi::c_void)->i32 { copro_data=data; copro_ops=ops; 0 }

// External kernel types, helpers, GPIO callbacks, IRQ callbacks, probe, and module registration
// are supplied by the Linux translation environment; their declarations remain external.
extern "C" {
    fn aspeed_gpio_copro_grab_gpio(desc:*mut gpio_desc,vreg_offset:*mut u16,dreg_offset:*mut u16,bit:*mut u8)->i32;
    fn aspeed_gpio_copro_release_gpio(desc:*mut gpio_desc)->i32;
}

unsafe extern "C" fn aspeed_g4_reg_bit_set(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg,v:bool){let a=aspeed_gpio_g4_bank_reg(g,to_bank(o),r);let mut t=if r as u8==aspeed_gpio_reg::reg_val as u8{*(*g).dcache.add(gpio_bank(o))}else{ioread32(a)};if v{t|=gpio_bit(o)}else{t&=!gpio_bit(o)};if r as u8==aspeed_gpio_reg::reg_val as u8{*(*g).dcache.add(gpio_bank(o))=t} iowrite32(t,a)}
unsafe extern "C" fn aspeed_g4_reg_bit_get(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg)->bool{(ioread32(aspeed_gpio_g4_bank_reg(g,to_bank(o),r))&gpio_bit(o))!=0}
unsafe extern "C" fn aspeed_g4_reg_bank_get(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg)->i32{if matches!(r,aspeed_gpio_reg::reg_rdata|aspeed_gpio_reg::reg_irq_status){ioread32(aspeed_gpio_g4_bank_reg(g,to_bank(o),r)) as i32}else{-95}}
unsafe extern "C" fn aspeed_g4_privilege_ctrl(g:*mut aspeed_gpio,o:u32,s:i32){aspeed_g4_reg_bit_set(g,o&!7,aspeed_gpio_reg::reg_cmdsrc1,(s&2)!=0);aspeed_g4_reg_bit_set(g,o&!7,aspeed_gpio_reg::reg_cmdsrc0,(s&1)!=0)}
unsafe extern "C" fn aspeed_g4_privilege_init(g:*mut aspeed_gpio){let n=((*g).chip.ngpio as u32+31)/32;for i in 0..n{for o in [0,8,16,24]{aspeed_g4_privilege_ctrl(g,(i<<5)+o,GPIO_CMDSRC_ARM)}}}
unsafe extern "C" fn aspeed_g7_reg_bit_set(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg,v:bool){let m=aspeed_gpio_g7_reg_mask(r);if m!=0{let a=(*g).base.add(GPIO_G7_CTRL_REG_BASE+o as usize*4);iowrite32((ioread32(a)&!m)|if v{m}else{0},a)}}
unsafe extern "C" fn aspeed_g7_reg_bit_get(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg)->bool{let mut m=aspeed_gpio_g7_reg_mask(r);if matches!(r,aspeed_gpio_reg::reg_val){m=GPIO_G7_CTRL_IN_DATA}if m!=0{let a=(*g).base.add(GPIO_G7_CTRL_REG_BASE+o as usize*4);(ioread32(a)&m)!=0}else{false}}
unsafe extern "C" fn aspeed_g7_reg_bank_get(g:*mut aspeed_gpio,o:u32,r:aspeed_gpio_reg)->i32{if matches!(r,aspeed_gpio_reg::reg_irq_status){ioread32((*g).base.add(GPIO_G7_IRQ_STS_BASE+(o as usize>>5)*4)) as i32}else{-95}}

#[repr(C)] pub struct aspeed_gpio_copro_ops { pub request_access:Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub release_access:Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }

extern "C" {
    fn ioread32(addr:*mut u8)->u32;
    fn iowrite32(v:u32,addr:*mut u8);
}

/* Device configurations and platform-driver registration are intentionally kept as
 * declarations here; the surrounding kernel bindings provide their concrete ABI. */
#[no_mangle] pub static ast2400_config: aspeed_gpio_config = aspeed_gpio_config { nr_gpios:220,props:core::ptr::null(),llops:core::ptr::null(),debounce_timers_array:debounce_timers.as_ptr(),debounce_timers_num:4,require_dcache:true };
#[no_mangle] pub static ast2500_config: aspeed_gpio_config = aspeed_gpio_config { nr_gpios:232,props:core::ptr::null(),llops:core::ptr::null(),debounce_timers_array:debounce_timers.as_ptr(),debounce_timers_num:4,require_dcache:true };
#[no_mangle] pub static ast2600_config: aspeed_gpio_config = aspeed_gpio_config { nr_gpios:208,props:core::ptr::null(),llops:core::ptr::null(),debounce_timers_array:debounce_timers.as_ptr(),debounce_timers_num:4,require_dcache:true };
#[no_mangle] pub static ast2700_config: aspeed_gpio_config = aspeed_gpio_config { nr_gpios:216,props:core::ptr::null(),llops:core::ptr::null(),debounce_timers_array:g7_debounce_timers.as_ptr(),debounce_timers_num:4,require_dcache:false };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
