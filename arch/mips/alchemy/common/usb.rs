// SPDX-License-Identifier: GPL-2.0-only
/* USB block power/access management abstraction. */

/* External kernel/platform symbols supplied by other translation units. */
extern "C" {
    fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn wmb();
    fn udelay(usecs: u32);
    fn KSEG1ADDR(addr: usize) -> usize;
    fn alchemy_get_cputype() -> i32;
    fn register_syscore(ops: *mut syscore);
}

#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct syscore_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}
#[repr(C)]
pub struct syscore { pub ops: *const syscore_ops }

extern "C" {
    fn clk_get(dev: *mut core::ffi::c_void, name: *const i8) -> *mut clk;
    fn clk_round_rate(c: *mut clk, rate: i64) -> i64;
    fn clk_set_rate(c: *mut clk, rate: i64) -> i32;
    fn clk_prepare_enable(c: *mut clk) -> i32;
    fn clk_disable_unprepare(c: *mut clk);
    fn clk_put(c: *mut clk);
    fn IS_ERR(ptr: *mut clk) -> bool;
    static mut alchemy_usb_lock: core::ffi::c_void;
}

const AU1000_OHCICFG: usize = 0x7fffc;
const AU1550_OHCICFG: usize = 0x07ffc;
const AU1200_USBCFG: usize = 0x04;
const USBHEN_RD: u32 = 1 << 4;
const USBHEN_CE: u32 = 1 << 3;
const USBHEN_E: u32 = 1 << 2;
const USBHEN_C: u32 = 1 << 1;
const USBHEN_BE: u32 = 1 << 0;
const USBCFG_PFEN: u32 = 1 << 31;
const USBCFG_RDCOMB: u32 = 1 << 30;
const USBCFG_UNKNOWN: u32 = 5 << 20;
const USBCFG_SSD: u32 = 1 << 23;
const USBCFG_PPE: u32 = 1 << 19;
const USBCFG_UCE: u32 = 1 << 18;
const USBCFG_ECE: u32 = 1 << 17;
const USBCFG_OCE: u32 = 1 << 16;
const USBCFG_UCAM: u32 = 1 << 7;
const USBCFG_GME: u32 = 1 << 6;
const USBCFG_DBE: u32 = 1 << 5;
const USBCFG_DME: u32 = 1 << 4;
const USBCFG_EBE: u32 = 1 << 3;
const USBCFG_EME: u32 = 1 << 2;
const USBCFG_OBE: u32 = 1 << 1;
const USBCFG_OME: u32 = 1;
const USBCFG_FLA: u32 = 0x20 << 8;
const USBCFG_INIT_AU1200: u32 = USBCFG_PFEN|USBCFG_RDCOMB|USBCFG_UNKNOWN|USBCFG_SSD|USBCFG_FLA|USBCFG_UCAM|USBCFG_GME|USBCFG_DBE|USBCFG_DME|USBCFG_EBE|USBCFG_EME|USBCFG_OBE|USBCFG_OME;
const USB_DWC_CTRL1: usize=0x00; const USB_DWC_CTRL2: usize=0x04; const USB_MSR_ERR: usize=0x18; const USB_DWC_CTRL3: usize=0x1c; const USB_DWC_CTRL4: usize=0x20; const USB_DWC_CTRL7: usize=0x34; const USB_INT_STATUS: usize=0xc4; const USB_INT_ENABLE: usize=0xc8;
const USB_DWC_CTRL1_OTGD:u32=4; const USB_DWC_CTRL1_HSTRS:u32=2; const USB_DWC_CTRL1_DCRS:u32=1;
const USB_DWC_CTRL2_PHY1RS:u32=4; const USB_DWC_CTRL2_PHY0RS:u32=2; const USB_DWC_CTRL2_PHYRS:u32=1;
const USB_DWC_CTRL3_OHCI1_CKEN:u32=1<<19; const USB_DWC_CTRL3_OHCI0_CKEN:u32=1<<18; const USB_DWC_CTRL3_EHCI0_CKEN:u32=1<<17; const USB_DWC_CTRL3_OTG0_CKEN:u32=1<<16;
const USB_SBUS_CTRL_SBCA:u32=4; const USB_INTEN_UDC:u32=8; const USB_INTEN_EHCI:u32=4; const USB_INTEN_OHCI1:u32=2; const USB_INTEN_OHCI0:u32=1;

const ALCHEMY_CPU_AU1000:i32=0; const ALCHEMY_CPU_AU1500:i32=1; const ALCHEMY_CPU_AU1100:i32=2; const ALCHEMY_CPU_AU1550:i32=3; const ALCHEMY_CPU_AU1200:i32=4; const ALCHEMY_CPU_AU1300:i32=5;
const ALCHEMY_USB_OHCI0:i32=0; const ALCHEMY_USB_OHCI1:i32=1; const ALCHEMY_USB_EHCI0:i32=2; const ALCHEMY_USB_UDC0:i32=3; const ALCHEMY_USB_OTG0:i32=4;
const AU1000_USB_OHCI_PHYS_ADDR:usize=0; const AU1550_USB_OHCI_PHYS_ADDR:usize=0; const AU1200_USB_CTL_PHYS_ADDR:usize=0; const AU1200_USB_OTG_PHYS_ADDR:usize=0; const AU1300_USB_CTL_PHYS_ADDR:usize=0;

#[inline] unsafe fn rd(p:*mut u8, o:usize)->u32 { __raw_readl(p.add(o) as *mut _) }
#[inline] unsafe fn wr(p:*mut u8, o:usize, v:u32) { __raw_writel(v,p.add(o) as *mut _); }
#[inline] unsafe fn phy(p:*mut u8, en:i32) { let mut r=rd(p,USB_DWC_CTRL2); let s=rd(p,USB_DWC_CTRL3)& (USB_DWC_CTRL3_OHCI1_CKEN|USB_DWC_CTRL3_OHCI0_CKEN|USB_DWC_CTRL3_EHCI0_CKEN|USB_DWC_CTRL3_OTG0_CKEN); if en!=0 { r|=USB_DWC_CTRL2_PHY1RS|USB_DWC_CTRL2_PHY0RS|USB_DWC_CTRL2_PHYRS; wr(p,USB_DWC_CTRL2,r); wmb(); } else if s==0 { r&=!(USB_DWC_CTRL2_PHY1RS|USB_DWC_CTRL2_PHY0RS|USB_DWC_CTRL2_PHYRS); wr(p,USB_DWC_CTRL2,r); wmb(); } }

unsafe fn ohci1300(p:*mut u8,en:i32,id:i32){if en!=0{wr(p,USB_DWC_CTRL7,1);wmb();let mut r=rd(p,USB_DWC_CTRL3);r|=if id==0{USB_DWC_CTRL3_OHCI0_CKEN}else{USB_DWC_CTRL3_OHCI1_CKEN};wr(p,USB_DWC_CTRL3,r);wmb();phy(p,en);r=rd(p,USB_INT_ENABLE);r|=if id==0{USB_INTEN_OHCI0}else{USB_INTEN_OHCI1};wr(p,USB_INT_ENABLE,r);wmb();wr(p,USB_DWC_CTRL7,0);wmb()}else{let mut r=rd(p,USB_INT_ENABLE);r&=!(if id==0{USB_INTEN_OHCI0}else{USB_INTEN_OHCI1});wr(p,USB_INT_ENABLE,r);wmb();r=rd(p,USB_DWC_CTRL3);r&=!(if id==0{USB_DWC_CTRL3_OHCI0_CKEN}else{USB_DWC_CTRL3_OHCI1_CKEN});wr(p,USB_DWC_CTRL3,r);wmb();phy(p,en)}}
unsafe fn ehci1300(p:*mut u8,en:i32){let mut r;if en!=0{r=rd(p,USB_DWC_CTRL3)|USB_DWC_CTRL3_EHCI0_CKEN;wr(p,USB_DWC_CTRL3,r);wmb();r=rd(p,USB_DWC_CTRL1)|USB_DWC_CTRL1_HSTRS;wr(p,USB_DWC_CTRL1,r);wmb();phy(p,en);r=rd(p,USB_INT_ENABLE)|USB_INTEN_EHCI;wr(p,USB_INT_ENABLE,r);wmb()}else{r=rd(p,USB_INT_ENABLE)&!USB_INTEN_EHCI;wr(p,USB_INT_ENABLE,r);wmb();r=rd(p,USB_DWC_CTRL1)&!USB_DWC_CTRL1_HSTRS;wr(p,USB_DWC_CTRL1,r);wmb();r=rd(p,USB_DWC_CTRL3)&!USB_DWC_CTRL3_EHCI0_CKEN;wr(p,USB_DWC_CTRL3,r);wmb();phy(p,en)}}
unsafe fn udc1300(p:*mut u8,en:i32){let mut r;if en!=0{r=rd(p,USB_DWC_CTRL1)|USB_DWC_CTRL1_DCRS;wr(p,USB_DWC_CTRL1,r);wmb();phy(p,en);r=rd(p,USB_INT_ENABLE)|USB_INTEN_UDC;wr(p,USB_INT_ENABLE,r);wmb()}else{r=rd(p,USB_INT_ENABLE)&!USB_INTEN_UDC;wr(p,USB_INT_ENABLE,r);wmb();r=rd(p,USB_DWC_CTRL1)&!USB_DWC_CTRL1_DCRS;wr(p,USB_DWC_CTRL1,r);wmb();phy(p,en)}}
unsafe fn otg1300(p:*mut u8,en:i32){let mut r;if en!=0{r=rd(p,USB_DWC_CTRL3)|USB_DWC_CTRL3_OTG0_CKEN;wr(p,USB_DWC_CTRL3,r);wmb();r=rd(p,USB_DWC_CTRL1)&!USB_DWC_CTRL1_OTGD;wr(p,USB_DWC_CTRL1,r);wmb();phy(p,en)}else{r=rd(p,USB_DWC_CTRL1)|USB_DWC_CTRL1_OTGD;wr(p,USB_DWC_CTRL1,r);wmb();r=rd(p,USB_DWC_CTRL3)&!USB_DWC_CTRL3_OTG0_CKEN;wr(p,USB_DWC_CTRL3,r);wmb();phy(p,en)}}
unsafe fn au1300_usb_control(block:i32,en:i32)->i32{let p=KSEG1ADDR(AU1300_USB_CTL_PHYS_ADDR) as *mut u8;match block{ALCHEMY_USB_OHCI0=>ohci1300(p,en,0),ALCHEMY_USB_OHCI1=>ohci1300(p,en,1),ALCHEMY_USB_EHCI0=>ehci1300(p,en),ALCHEMY_USB_UDC0=>udc1300(p,en),ALCHEMY_USB_OTG0=>otg1300(p,en),_=>return -19}0}
unsafe fn au1300_usb_init(){let p=KSEG1ADDR(AU1300_USB_CTL_PHYS_ADDR) as *mut u8;wr(p,USB_INT_ENABLE,0);wmb();wr(p,USB_DWC_CTRL3,0);wmb();wr(p,USB_MSR_ERR,!0);wmb();wr(p,USB_INT_STATUS,!0);wmb();wr(p,0x14,USB_SBUS_CTRL_SBCA);wmb()}

unsafe fn au1200_usb_control(block:i32,en:i32)->i32{let p=KSEG1ADDR(AU1200_USB_CTL_PHYS_ADDR) as *mut u8;let mut r=rd(p,AU1200_USBCFG);match block{ALCHEMY_USB_OHCI0=>{wr(p,AU1200_USBCFG,if en!=0{r|=USBCFG_OCE;r}else{r&=!USBCFG_OCE;r});wmb();udelay(if en!=0{2000}else{1000})},ALCHEMY_USB_EHCI0=>{if en!=0{r|=USBCFG_ECE|USBCFG_PPE}else{if r&USBCFG_UCE==0{r&=!USBCFG_PPE};r&=!USBCFG_ECE};wr(p,AU1200_USBCFG,r);wmb();udelay(1000)},ALCHEMY_USB_UDC0=>{if en!=0{r|=USBCFG_UCE|USBCFG_PPE}else{if r&USBCFG_ECE==0{r&=!USBCFG_PPE};r&=!USBCFG_UCE};wr(p,AU1200_USBCFG,r);wmb()},_=>return -19}0}
unsafe fn au1200_usb_init(){let p=KSEG1ADDR(AU1200_USB_CTL_PHYS_ADDR) as *mut u8;wr(p,AU1200_USBCFG,USBCFG_INIT_AU1200);wmb();udelay(1000)}

unsafe fn au1000_usb_control(block:i32,en:i32,rb:usize,creg:usize)->i32{if block!=ALCHEMY_USB_OHCI0{return -19}let p=KSEG1ADDR(rb) as *mut u8;let mut r=rd(p,creg);let c=clk_get(core::ptr::null_mut(),b"usbh_clk\0".as_ptr() as *const i8);if IS_ERR(c){return 0}if en!=0{if clk_prepare_enable(c)!=0{clk_put(c);return 0}wr(p,creg,r|USBHEN_CE);wmb();udelay(1000);wr(p,creg,r|USBHEN_CE|USBHEN_E);wmb();udelay(1000);while {let _=rd(p,creg);rd(p,creg)&USBHEN_RD==0}{udelay(1000)}}else{wr(p,creg,r&!(USBHEN_CE|USBHEN_E));wmb();clk_disable_unprepare(c)}clk_put(c);0}
unsafe fn au1000_usb_init(rb:usize,reg:usize)->i32{let p=KSEG1ADDR(rb+reg) as *mut u8;let mut r=rd(p,0);r|=USBHEN_C;wr(p,0,r);wmb();udelay(1000);0}

#[no_mangle] pub unsafe extern "C" fn alchemy_usb_control(block:i32,en:i32)->i32{match alchemy_get_cputype(){ALCHEMY_CPU_AU1000|ALCHEMY_CPU_AU1500|ALCHEMY_CPU_AU1100=>au1000_usb_control(block,en,AU1000_USB_OHCI_PHYS_ADDR,AU1000_OHCICFG),ALCHEMY_CPU_AU1550=>au1000_usb_control(block,en,AU1550_USB_OHCI_PHYS_ADDR,AU1550_OHCICFG),ALCHEMY_CPU_AU1200=>au1200_usb_control(block,en),ALCHEMY_CPU_AU1300=>au1300_usb_control(block,en),_=>-19}}

static mut alchemy_usb_pmdata:[usize;2]=[0;2];
unsafe fn alchemy_usb_pm(susp:i32){match alchemy_get_cputype(){ALCHEMY_CPU_AU1000|ALCHEMY_CPU_AU1500|ALCHEMY_CPU_AU1100=>{let p=KSEG1ADDR(AU1000_USB_OHCI_PHYS_ADDR) as *mut u8;if susp!=0{alchemy_usb_pmdata[0]=rd(p,AU1000_OHCICFG) as usize;wr(p,4,0);wmb();wr(p,AU1000_OHCICFG,0);wmb()}else{wr(p,AU1000_OHCICFG,alchemy_usb_pmdata[0] as u32);wmb()}},ALCHEMY_CPU_AU1550=>{},ALCHEMY_CPU_AU1200=>{if susp==0{au1200_usb_init()}},ALCHEMY_CPU_AU1300=>{if susp==0{au1300_usb_init()}},_=>{}}}
unsafe extern "C" fn alchemy_usb_suspend(_: *mut core::ffi::c_void)->i32{alchemy_usb_pm(1);0}
unsafe extern "C" fn alchemy_usb_resume(_: *mut core::ffi::c_void){alchemy_usb_pm(0)}
static ALCHEMY_USB_PM_SYSCORE_OPS:syscore_ops=syscore_ops{suspend:Some(alchemy_usb_suspend),resume:Some(alchemy_usb_resume)};
static mut ALCHEMY_USB_PM_SYSCORE:syscore=syscore{ops:&ALCHEMY_USB_PM_SYSCORE_OPS};
#[allow(dead_code)] unsafe fn alchemy_usb_init()->i32{let r=match alchemy_get_cputype(){ALCHEMY_CPU_AU1000|ALCHEMY_CPU_AU1500|ALCHEMY_CPU_AU1100=>au1000_usb_init(AU1000_USB_OHCI_PHYS_ADDR,AU1000_OHCICFG),ALCHEMY_CPU_AU1550=>au1000_usb_init(AU1550_USB_OHCI_PHYS_ADDR,AU1550_OHCICFG),ALCHEMY_CPU_AU1200=>{au1200_usb_init();0},ALCHEMY_CPU_AU1300=>{au1300_usb_init();0},_=>0};if r==0{register_syscore(&raw mut ALCHEMY_USB_PM_SYSCORE)}r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
