// SPDX-License-Identifier: GPL-2.0-only
/* PXA2xx pin mux configuration support. */

use core::ptr::{read_volatile, write_volatile};

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    static mut pxa_last_gpio: i32;
    fn gpio_to_bank(gpio: i32) -> i32;
    fn mfp_to_gpio(mfp: i32) -> i32;
    fn cpu_is_pxa2xx() -> bool;
    fn cpu_is_pxa25x() -> bool;
    fn cpu_is_pxa27x() -> bool;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn pr_warn(fmt: *const u8, ...);
}

const MFP_PIN_GPIO127: i32 = 127;
const MFP_DIR_OUT: u64 = 1 << 8;
const MFP_LPM_STATE_MASK: u64 = 0x700;
const MFP_LPM_DRIVE_HIGH: u64 = 0x100;
const MFP_LPM_DRIVE_LOW: u64 = 0x200;
const MFP_LPM_INPUT: u64 = 0x300;
const MFP_LPM_DEFAULT: u64 = 0;
const MFP_LPM_CAN_WAKEUP: u64 = 1 << 12;
const MFP_LPM_EDGE_RISE: u64 = 1 << 13;
const MFP_LPM_EDGE_FALL: u64 = 1 << 14;
const MFP_LPM_KEEP_OUTPUT: u64 = 1 << 15;

extern "C" { fn MFP_AF(c: u64) -> i32; fn MFP_PIN(c: u64) -> i32; }

#[repr(C)]
struct gpio_desc {
    valid: bool, can_wakeup: bool, keypad_gpio: bool, dir_inverted: bool,
    mask: u32, mux_mask: u32, config: u64,
}

static mut GPIO_DESC: [gpio_desc; 128] = [gpio_desc { valid:false, can_wakeup:false,
    keypad_gpio:false, dir_inverted:false, mask:0, mux_mask:0, config:0 }; 128];
static mut GPDR_LPM: [u32; 4] = [0; 4];

#[inline] unsafe fn reg_read(addr: usize) -> u32 { read_volatile(addr as *const u32) }
#[inline] unsafe fn reg_write(addr: usize, v: u32) { write_volatile(addr as *mut u32, v); }
#[inline] fn pgsr(x: i32) -> usize { 0x40f00020 + ((x as usize) << 2) }
#[inline] fn gafr(u: i32, x: i32) -> usize { (if u != 0 { 0x40e00058 } else { 0x40e00054 }) + ((x as usize) << 3) }
#[inline] fn bank_off(n: i32) -> usize { if n < 3 { (n as usize) << 2 } else { 0x100 + (((n - 3) as usize) << 2) } }
#[inline] fn gplr(x: i32) -> usize { 0x40e00000 + bank_off(x >> 5) }
#[inline] fn gpdr(x: i32) -> usize { 0x40e00000 + bank_off(x >> 5) + 0x0c }
#[inline] fn gpsr(x: i32) -> usize { 0x40e00000 + bank_off(x >> 5) + 0x18 }
#[inline] fn gpcr(x: i32) -> usize { 0x40e00000 + bank_off(x >> 5) + 0x24 }
#[inline] fn gpio_bit(gpio: i32) -> u32 { 1u32 << (gpio & 31) }

extern "C" {
    static mut PWER: u32; static mut PKWR: u32; static mut PRER: u32; static mut PFER: u32;
    static mut PSSR: u32;
}
const PSSR_RDH: u32 = 1 << 3; const PSSR_PH: u32 = 1 << 4; const PWER_WE35: u32 = 1 << 24;

unsafe fn mfp_config_gpio(gpio: i32, c: u64) -> i32 {
    let mask = gpio_bit(gpio); let bank = gpio_to_bank(gpio); let uorl = ((gpio & 0x10) != 0) as i32;
    let shft = ((gpio & 0xf) << 1) as u32; let fn_ = MFP_AF(c); let mut is_out = (c & MFP_DIR_OUT != 0) as i32;
    if fn_ > 3 { return -22; }
    let a = gafr(uorl, bank); let mut v = reg_read(a); v = (v & !(3 << shft)) | ((fn_ as u32) << shft); reg_write(a, v);
    let d = &mut GPIO_DESC[gpio as usize]; let a = gpdr(gpio); let mut v = reg_read(a);
    if (is_out != 0) ^ d.dir_inverted { v |= mask; } else { v &= !mask; } reg_write(a, v);
    match c & MFP_LPM_STATE_MASK {
        MFP_LPM_DRIVE_HIGH => { let a=pgsr(bank); reg_write(a, reg_read(a)|mask); is_out=1; }
        MFP_LPM_DRIVE_LOW => { let a=pgsr(bank); reg_write(a, reg_read(a)&!mask); is_out=1; }
        MFP_LPM_INPUT | MFP_LPM_DEFAULT => {}
        _ => { /* warning and fall through, treating as default */ }
    }
    if (is_out != 0) ^ d.dir_inverted { GPDR_LPM[bank as usize] |= mask; } else { GPDR_LPM[bank as usize] &= !mask; }
    if c & MFP_LPM_CAN_WAKEUP != 0 && !d.can_wakeup { return -22; }
    if c & MFP_LPM_CAN_WAKEUP != 0 && is_out != 0 { return -22; }
    0
}

unsafe fn mfp_validate(mfp: i32) -> i32 { let gpio=mfp_to_gpio(mfp); if mfp > MFP_PIN_GPIO127 || !GPIO_DESC[gpio as usize].valid { -1 } else { gpio } }

#[no_mangle] pub unsafe extern "C" fn pxa2xx_mfp_config(mfp_cfgs: *mut u64, num: i32) {
    for i in 0..num { let c=*mfp_cfgs.add(i as usize); let gpio=mfp_validate(MFP_PIN(c)); if gpio<0 {continue;} let mut f=0; local_irq_save(&mut f); GPIO_DESC[gpio as usize].config=c; mfp_config_gpio(gpio,c); local_irq_restore(f); }
}
#[no_mangle] pub unsafe extern "C" fn pxa2xx_mfp_set_lpm(mfp:i32,lpm:u64) { let gpio=mfp_validate(mfp); if gpio<0{return;} let mut f=0; local_irq_save(&mut f); let c=(GPIO_DESC[gpio as usize].config & !MFP_LPM_STATE_MASK)|(lpm&MFP_LPM_STATE_MASK); mfp_config_gpio(gpio,c); local_irq_restore(f); }

#[no_mangle] pub unsafe extern "C" fn gpio_set_wake(gpio:u32,on:u32)->i32 { if gpio>mfp_to_gpio(MFP_PIN_GPIO127) as u32{return -22;} let d=&mut GPIO_DESC[gpio as usize]; if !d.valid{return -22;} let c=d.config;
    if d.keypad_gpio && MFP_AF(c)==0 && c&MFP_LPM_CAN_WAKEUP!=0 { if on!=0{PKWR|=d.mask}else{PKWR&=!d.mask} return 0; }
    if on!=0 && (PWER&d.mux_mask)&!d.mask!=0{return -16;} if d.can_wakeup&&c&MFP_LPM_CAN_WAKEUP!=0 { if on!=0 {PWER=(PWER&!d.mux_mask)|d.mask; if c&MFP_LPM_EDGE_RISE!=0{PRER|=d.mask}else{PRER&=!d.mask}; if c&MFP_LPM_EDGE_FALL!=0{PFER|=d.mask}else{PFER&=!d.mask};}else{PWER&=!d.mask;PRER&=!d.mask;PFER&=!d.mask;} } 0 }

#[cfg(feature="CONFIG_PXA25x")] unsafe fn pxa25x_mfp_init(){ pxa_last_gpio=84; for i in 0..=84{GPIO_DESC[i as usize].valid=true;} for i in 0..=15{GPIO_DESC[i].can_wakeup=true;GPIO_DESC[i].mask=gpio_bit(i as i32);} for i in 86..=pxa_last_gpio { GPIO_DESC[i as usize].dir_inverted=true; } }
#[cfg(not(feature="CONFIG_PXA25x"))] unsafe fn pxa25x_mfp_init(){}

static PXA27X_PKWR_GPIO:[i32;20]=[13,16,17,34,36,37,38,39,90,91,93,94,95,96,97,98,99,100,101,102];
#[no_mangle] pub unsafe extern "C" fn keypad_set_wake(on:u32)->i32{let mut mask=0;for i in 0..20{let d=&GPIO_DESC[PXA27X_PKWR_GPIO[i] as usize];if MFP_AF(d.config)!=0&&d.config&MFP_LPM_CAN_WAKEUP!=0{mask|=d.mask;}}if on!=0{PKWR|=mask}else{PKWR&=!mask}0}
#[cfg(feature="CONFIG_PXA27x")] unsafe fn pxa27x_mfp_init(){pxa_last_gpio=120;for i in 0..=120{if ![2,5,6,7,8].contains(&i){GPIO_DESC[i].valid=true;}}for i in 0..20{let g=PXA27X_PKWR_GPIO[i] as usize;GPIO_DESC[g].can_wakeup=true;GPIO_DESC[g].keypad_gpio=true;GPIO_DESC[g].mask=1<<i;}for i in 0..=15{if gpio_bit(i)&0x1e4!=0{continue;}GPIO_DESC[i].can_wakeup=true;GPIO_DESC[i].mask=gpio_bit(i as i32);}GPIO_DESC[35].can_wakeup=true;GPIO_DESC[35].mask=PWER_WE35;for &(g,m,mm) in &[(31,1<<19,3<<19),(113,2<<19,3<<19),(38,1<<16,7<<16),(53,2<<16,7<<16),(40,3<<16,7<<16),(36,4<<16,7<<16)]{GPIO_DESC[g].can_wakeup=true;GPIO_DESC[g].mask=m;GPIO_DESC[g].mux_mask=mm;}}
#[cfg(not(feature="CONFIG_PXA27x"))] unsafe fn pxa27x_mfp_init(){}

#[no_mangle] pub unsafe extern "C" fn pxa2xx_mfp_init()->i32{if !cpu_is_pxa2xx(){return 0;}if cpu_is_pxa25x(){pxa25x_mfp_init();}if cpu_is_pxa27x(){pxa27x_mfp_init();}PSSR=PSSR_RDH;for i in 0..=gpio_to_bank(pxa_last_gpio){GPDR_LPM[i as usize]=reg_read(gpdr(i*32));}0}

#[cfg(feature="CONFIG_PM")]
static mut SAVED_GAFR:[[u32;4];2]=[[0;4];2];
#[cfg(feature="CONFIG_PM")] static mut SAVED_GPDR:[u32;4]=[0;4];
#[cfg(feature="CONFIG_PM")] static mut SAVED_GPLR:[u32;4]=[0;4];
#[cfg(feature="CONFIG_PM")] static mut SAVED_PGSR:[u32;4]=[0;4];

#[cfg(feature="CONFIG_PM")]
unsafe fn pxa2xx_mfp_suspend()->i32 {
    for i in 0..pxa_last_gpio { let d=&GPIO_DESC[i as usize]; if d.config&MFP_LPM_KEEP_OUTPUT!=0 && reg_read(gpdr(i))&gpio_bit(i)!=0 { let a=pgsr(gpio_to_bank(i)); if reg_read(gplr(i))&gpio_bit(i)!=0{reg_write(a,reg_read(a)|gpio_bit(i));}else{reg_write(a,reg_read(a)&!gpio_bit(i));} } }
    for i in 0..=gpio_to_bank(pxa_last_gpio) { SAVED_GAFR[0][i as usize]=reg_read(gafr(0,i));SAVED_GAFR[1][i as usize]=reg_read(gafr(1,i));SAVED_GPDR[i as usize]=reg_read(gpdr(i*32));SAVED_GPLR[i as usize]=reg_read(gplr(i*32));SAVED_PGSR[i as usize]=reg_read(pgsr(i));reg_write(gpsr(i*32),SAVED_PGSR[i as usize]);reg_write(gpcr(i*32),!SAVED_PGSR[i as usize]); }
    for i in 0..pxa_last_gpio { let b=gpio_to_bank(i) as usize; let a=gpdr(i); if GPDR_LPM[b]&gpio_bit(i)!=0 || (GPIO_DESC[i as usize].config&MFP_LPM_KEEP_OUTPUT!=0 && SAVED_GPDR[b]&gpio_bit(i)!=0){reg_write(a,reg_read(a)|gpio_bit(i));}else{reg_write(a,reg_read(a)&!gpio_bit(i));} } 0
}
#[cfg(feature="CONFIG_PM")]
unsafe fn pxa2xx_mfp_resume(){for i in 0..=gpio_to_bank(pxa_last_gpio){reg_write(gafr(0,i),SAVED_GAFR[0][i as usize]);reg_write(gafr(1,i),SAVED_GAFR[1][i as usize]);reg_write(gpsr(i*32),SAVED_GPLR[i as usize]);reg_write(gpcr(i*32),!SAVED_GPLR[i as usize]);reg_write(gpdr(i*32),SAVED_GPDR[i as usize]);reg_write(pgsr(i),SAVED_PGSR[i as usize]);}PSSR=PSSR_RDH|PSSR_PH;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
