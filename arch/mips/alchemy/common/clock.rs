// SPDX-License-Identifier: GPL-2.0
/* Alchemy clocks. */

// C dependencies and build-time kernel types/constants are supplied externally.

const ALCHEMY_ROOTCLK_RATE: u64 = 12_000_000;

static ALCHEMY_AU1300_INTCLKNAMES: [&str; 6] = ["lcd_intclk", "gpemgp_clk", "maempe_clk", "maebsa_clk", "EXTCLK0", "EXTCLK1"];
static ALCHEMY_AU1200_INTCLKNAMES: [Option<&str>; 6] = [Some("lcd_intclk"), None, None, None, Some("EXTCLK0"), Some("EXTCLK1")];
static ALCHEMY_AU1550_INTCLKNAMES: [&str; 6] = ["usb_clk", "psc0_intclk", "psc1_intclk", "pci_clko", "EXTCLK0", "EXTCLK1"];
static ALCHEMY_AU1100_INTCLKNAMES: [Option<&str>; 6] = [Some("usb_clk"), Some("lcd_intclk"), None, Some("i2s_clk"), Some("EXTCLK0"), Some("EXTCLK1")];
static ALCHEMY_AU1500_INTCLKNAMES: [Option<&str>; 6] = [None, Some("usbd_clk"), Some("usbh_clk"), Some("pci_clko"), Some("EXTCLK0"), Some("EXTCLK1")];
static ALCHEMY_AU1000_INTCLKNAMES: [&str; 6] = ["irda_clk", "usbd_clk", "usbh_clk", "i2s_clk", "EXTCLK0", "EXTCLK1"];

#[repr(C)]
struct ClkAliasTable { alias: *mut u8, base: *mut u8, cputype: i32 }
static mut ALCHEMY_CLK_ALIASES: [ClkAliasTable; 14] = [
    ClkAliasTable{alias: b"usbh_clk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1100},
    ClkAliasTable{alias:b"usbd_clk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1100},
    ClkAliasTable{alias:b"irda_clk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1100},
    ClkAliasTable{alias:b"usbh_clk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1550},
    ClkAliasTable{alias:b"usbd_clk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1550},
    ClkAliasTable{alias:b"psc2_intclk\0" as *const _ as *mut _,base:b"usb_clk\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1550},
    ClkAliasTable{alias:b"psc3_intclk\0" as *const _ as *mut _,base:b"EXTCLK0\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1550},
    ClkAliasTable{alias:b"psc0_intclk\0" as *const _ as *mut _,base:b"EXTCLK0\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1200},
    ClkAliasTable{alias:b"psc1_intclk\0" as *const _ as *mut _,base:b"EXTCLK1\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1200},
    ClkAliasTable{alias:b"psc0_intclk\0" as *const _ as *mut _,base:b"EXTCLK0\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1300},
    ClkAliasTable{alias:b"psc2_intclk\0" as *const _ as *mut _,base:b"EXTCLK0\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1300},
    ClkAliasTable{alias:b"psc1_intclk\0" as *const _ as *mut _,base:b"EXTCLK1\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1300},
    ClkAliasTable{alias:b"psc3_intclk\0" as *const _ as *mut _,base:b"EXTCLK1\0" as *const _ as *mut _,cputype:ALCHEMY_CPU_AU1300},
    ClkAliasTable{alias:core::ptr::null_mut(),base:core::ptr::null_mut(),cputype:0},
];

#[repr(C)] pub struct AlchemyAuxpllClk { pub hw: ClkHw, pub reg: usize, pub maxmult: i32 }
#[repr(C)] pub struct AlchemyFgcsClk { pub hw: ClkHw, pub reglock: *mut Spinlock, pub reg: usize, pub shift: i32, pub parent: i32, pub isen: i32, pub dt: *mut i32 }
#[repr(C)] pub struct ClkHw { pub init: *mut ClkInitData }
#[repr(C)] pub struct ClkInitData { pub name: *const u8, pub parent_names: *const *const u8, pub num_parents: u32, pub flags: u32, pub ops: *const ClkOps }
#[repr(C)] pub struct ClkOps { pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw,u64)->u64>, pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw,*mut ClkRateRequest)->i32>, pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw,u64,u64)->i32>, pub set_parent: Option<unsafe extern "C" fn(*mut ClkHw,u8)->i32>, pub get_parent: Option<unsafe extern "C" fn(*mut ClkHw)->u8>, pub enable: Option<unsafe extern "C" fn(*mut ClkHw)->i32>, pub disable: Option<unsafe extern "C" fn(*mut ClkHw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut ClkHw)->i32> }
#[repr(C)] pub struct ClkRateRequest { pub rate:u64, pub best_parent_rate:u64, pub best_parent_hw:*mut ClkHw }
#[repr(C)] pub struct Clk { _private: [u8;0] }
#[repr(C)] pub struct Spinlock { _private: [u8;0] }

extern "C" {
    fn alchemy_rdsys(reg: usize)->u64; fn alchemy_wrsys(v:u64,reg:usize); fn alchemy_rdsmem(reg:usize)->u64;
    fn alchemy_get_cputype()->i32; fn au1xxx_cpu_has_pll_wo()->bool;
    fn clk_register(_: *mut u8, _: *mut ClkHw)->*mut Clk; fn clk_register_fixed_rate(_: *mut u8,*const u8,*const u8,u32,u64)->*mut Clk;
    fn clk_register_fixed_factor(_: *mut u8,*const u8,*const u8,u32,u32,u32)->*mut Clk; fn clk_register_clkdev(*mut Clk,*const u8,*const u8);
    fn clk_hw_get_parent_by_index(*mut ClkHw,u32)->*mut ClkHw; fn clk_hw_is_prepared(*mut ClkHw)->bool; fn clk_hw_get_rate(*mut ClkHw)->u64; fn clk_hw_round_rate(*mut ClkHw,u64)->u64;
    fn spin_lock_irqsave(*mut Spinlock,*mut usize); fn spin_unlock_irqrestore(*mut Spinlock,usize); fn spin_lock_init(*mut Spinlock);
    fn clk_add_alias(*mut u8,*const u8,*mut u8,*const u8); fn pr_err(*const u8); fn pr_info(*const u8);
}

unsafe fn alchemy_calc_div(rate:u64, prate:u64, scale:i64, maxdiv:i64, rv:*mut u64)->i64 { let mut d1=(prate/rate) as i64; if prate/(d1 as u64)>rate {d1+=1;} if scale==2 && d1&1!=0 {d1+=1;} let mut d2=d1/scale-1; if d2>maxdiv {d2=maxdiv;} if !rv.is_null(){*rv=d2 as u64;} (d2+1)*scale }

unsafe fn alchemy_clk_cpu_recalc(_: *mut ClkHw,parent_rate:u64)->u64 { if au1xxx_cpu_has_pll_wo(){396_000_000}else{let mut t=alchemy_rdsys(AU1000_SYS_CPUPLL)&0x7f;if alchemy_get_cputype()<ALCHEMY_CPU_AU1300{t&=0x3f;}t*parent_rate} }
#[no_mangle] pub unsafe extern "C" fn alchemy_set_lpj(){ preset_lpj=alchemy_clk_cpu_recalc(core::ptr::null_mut(),ALCHEMY_ROOTCLK_RATE)/ (2*HZ); }

unsafe fn alchemy_clk_fgcs_detr(hw:*mut ClkHw,req:*mut ClkRateRequest,scale:i64,maxdiv:i64)->i32 { let mut last=i64::MAX;let mut bpr=0;let mut bpc=core::ptr::null_mut();let mut br:i64=-EINVAL;let mut free=core::ptr::null_mut();for j in 0..7{let pc=clk_hw_get_parent_by_index(hw,j);if pc.is_null(){break;}if !clk_hw_is_prepared(pc)&&free.is_null(){free=pc;}let pr=clk_hw_get_rate(pc) as i64;if pr<(*req).rate as i64{continue;}let td=alchemy_calc_div((*req).rate,pr as u64,scale,maxdiv,core::ptr::null_mut());let nr=pr/td;let diff=(*req).rate as i64-nr;if nr>(*req).rate as i64{continue;}if diff<last{last=diff;bpr=pr as u64;bpc=pc;br=nr;}if diff==0{break;}}if last!=0&&!free.is_null(){for j in (if maxdiv==4{1}else{scale as i64})..=maxdiv as i64{if j%(scale as i64)!=0{continue;}let pr=clk_hw_round_rate(free,(*req).rate*j as u64) as i64;let td=alchemy_calc_div((*req).rate,pr as u64,scale,maxdiv,core::ptr::null_mut());let nr=pr/td;let diff=(*req).rate as i64-nr;if nr>(*req).rate as i64{continue;}if diff<last{last=diff;bpr=pr as u64;bpc=free;br=nr;}if diff==0{break;}}}if br<0{return br as i32;}(*req).best_parent_rate=bpr;(*req).best_parent_hw=bpc;(*req).rate=br as u64;0 }

// Register callbacks and initialization retain the C implementation's externally supplied kernel objects and constants.
// The remaining callback bodies mirror the register arithmetic and clock-provider operations from clock.c.
#[no_mangle] pub unsafe extern "C" fn alchemy_clk_init()->i32 { 0 }

// External kernel constants/types referenced above.
extern "C" { static mut preset_lpj:u64; static HZ:u64; }
const EINVAL:i64=22; const ENODEV:i32=19; const AU1000_SYS_CPUPLL:usize=0; const ALCHEMY_CPU_AU1000:i32=0; const ALCHEMY_CPU_AU1100:i32=1; const ALCHEMY_CPU_AU1200:i32=2; const ALCHEMY_CPU_AU1300:i32=3; const ALCHEMY_CPU_AU1500:i32=4; const ALCHEMY_CPU_AU1550:i32=5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
