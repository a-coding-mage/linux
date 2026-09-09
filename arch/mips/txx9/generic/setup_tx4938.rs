/*
 * TX4938/4937 setup routines
 * Based on linux/arch/mips/txx9/rbtx4938/setup.c,
 *             and RBTX49xx patch from CELF patch archive.
 *
 * 2003-2005 (c) MontaVista Software, Inc.
 * (C) Copyright TOSHIBA CORPORATION 2000-2001, 2004-2007
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Kernel headers and externally supplied symbols are dependencies of this translation.

#[repr(C)]
pub struct pt_regs { pub cp0_cause: u64, pub cp0_epc: u64 }
#[repr(C)]
pub struct resource { pub name: *const u8, pub start: usize, pub end: usize, pub flags: u64 }
#[repr(C)]
pub struct physmap_flash_data { pub width: u32 }
#[repr(C)]
pub struct platform_device;
#[repr(C)]
pub struct tx4938ide_platform_info { pub ioport_shift: u32, pub gbus_clock: u64, pub ebus_ch: i32 }
#[repr(C)]
pub struct txx9ndfmc_platform_data { pub shift: u32, pub gbus_clock: u64, pub hold: u32, pub spw: u32, pub ch_mask: u32 }
#[repr(C)]
pub struct txx9dmac_platform_data { pub have_64bit_regs: bool, pub memcpy_chan: i32 }

extern "C" {
    static mut tx4938_ccfgptr: *mut u8; static mut tx4938_sdramcptr: *mut u8;
    static mut tx4938_sramcptr: *mut u8; static mut tx4938_pioptr: *mut u8;
    static mut tx4938_ebuscptr: *mut u8; static mut txx9_ce_res: [resource; 8];
    static mut iomem_resource: resource; static mut txx9_master_clock: u64;
    static mut txx9_cpu_clock: u64; static mut txx9_gbus_clock: u64;
    static mut loops_per_jiffy: u64; static mut txx9_ccfg_toeon: i32;
    static mut txx9_pcode: u32; static mut txx9_pcode_str: *const u8;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut u8)>;
    static mut board_be_init: Option<unsafe extern "C" fn()>;
    fn ____raw_readq(p: *const u8) -> u64; fn __raw_readq(p: *const u8) -> u64;
    fn __raw_writel(v: u32, p: *mut u8); fn tx4938_ccfg_set(v: u64); fn tx4938_ccfg_clear(v: u64);
    fn txx9_wdt_init(a: u64); fn txx9_wdt_now(a: u64); fn local_irq_disable(); fn local_irq_enable();
    fn mdelay(v: u32); fn read_c0_errorepc() -> u64; fn console_verbose(); fn show_registers(r: *mut pt_regs);
    fn panic(s: *const u8) -> !; fn mips_set_be_handler(f: unsafe extern "C" fn(*mut pt_regs, i32) -> i32);
    fn txx9_reg_res_init(a: u32, b: u64, c: u64); fn set_c0_config(v: u64); fn request_resource(a: *mut resource, b: *mut resource) -> i32;
    fn txx9_clear64(a: *mut u8, v: u64); fn txx9_set64(a: *mut u8, v: u64); fn txx9_tmr_init(a: u64);
    fn txx9_clockevent_init(a: u64, b: u32, c: u32); fn txx9_sio_init(a: u64,b: u32,c:i32,d:u32,e:u32);
    fn txx9_ethaddr_init(a:u32,b:*mut u8); fn txx9_physmap_flash_init(a:i32,b:usize,c:usize,d:*mut physmap_flash_data);
    fn platform_device_alloc(a:*const u8,b:i32)->*mut platform_device; fn platform_device_add_resources(a:*mut platform_device,b:*mut resource,c:usize)->i32;
    fn platform_device_add_data(a:*mut platform_device,b:*mut u8,c:usize)->i32; fn platform_device_add(a:*mut platform_device)->i32; fn platform_device_put(a:*mut platform_device);
    fn txx9_ndfmc_init(a:u64,b:*mut txx9ndfmc_platform_data); fn txx9_dmac_init(a:i32,b:u64,c:u32,d:*mut txx9dmac_platform_data);
    fn txx9_aclc_init(a:u64,b:u32,c:i32,d:i32,e:i32); fn txx9_sramc_init(a:*mut resource);
}

static mut TX4938_SDRAM_RESOURCE: [resource; 4] = [resource { name: core::ptr::null(), start:0, end:0, flags:0 }; 4];
static mut TX4938_SRAM_RESOURCE: resource = resource { name: core::ptr::null(), start:0, end:0, flags:0 };
const TX4938_SRAM_SIZE: usize = 0x800;

unsafe extern "C" fn tx4938_wdr_init() { if ____raw_readq(tx4938_ccfgptr) & TX4938_CCFG_WDRST != 0 { pr_warn(b"Watchdog reset detected at 0x%lx\n\0".as_ptr(), read_c0_errorepc()); } tx4938_ccfg_set(TX4938_CCFG_WDRST); tx4938_ccfg_set(TX4938_CCFG_WR); }
pub unsafe extern "C" fn tx4938_wdt_init() { txx9_wdt_init(TX4938_TMR_REG(2) & 0xfffffffff); }
unsafe extern "C" fn tx4938_machine_restart(_command: *mut u8) { local_irq_disable(); tx4938_ccfg_set(TX4938_CCFG_WDRST); txx9_wdt_now(TX4938_TMR_REG(2)&0xfffffffff); while ____raw_readq(tx4938_ccfgptr)&TX4938_CCFG_WDRST==0 {} mdelay(10); if ____raw_readq(tx4938_ccfgptr)&TX4938_CCFG_WDREXEN!=0 { tx4938_ccfg_clear(TX4938_CCFG_WDREXEN); } if let Some(f)=_machine_halt { f(); } }
unsafe extern "C" fn tx4938_be_handler(regs:*mut pt_regs,_is_fixup:i32)->i32 { let data=(*regs).cp0_cause & 4; console_verbose(); show_registers(regs); panic(if data!=0 { b"BusError!\0".as_ptr() } else { b"BusError!\0".as_ptr() }); }
unsafe extern "C" fn tx4938_be_init() { mips_set_be_handler(tx4938_be_handler); }
pub unsafe extern "C" fn tx4938_setup() {
    let mut i=0; let ccfg=____raw_readq(tx4938_ccfgptr); txx9_reg_res_init(TX4938_REV_PCODE(),TX4938_REG_BASE,TX4938_REG_SIZE); set_c0_config(TX49_CONF_CWFON);
    while i<8 { if TX4938_EBUSC_CR(i)&8 != 0 { txx9_ce_res[i].start=TX4938_EBUSC_BA(i) as usize; txx9_ce_res[i].end=txx9_ce_res[i].start+TX4938_EBUSC_SIZE(i) as usize-1; request_resource(&mut iomem_resource,&mut txx9_ce_res[i]); } i+=1; }
    let divmode=ccfg as u32 & TX4938_CCFG_DIVMODE_MASK; if txx9_master_clock!=0 { txx9_gbus_clock=match divmode { TX4938_CCFG_DIVMODE_8|TX4938_CCFG_DIVMODE_10|TX4938_CCFG_DIVMODE_12|TX4938_CCFG_DIVMODE_16|TX4938_CCFG_DIVMODE_18=>txx9_master_clock*4,_=>txx9_master_clock }; txx9_cpu_clock=match divmode { TX4938_CCFG_DIVMODE_2|TX4938_CCFG_DIVMODE_8=>txx9_gbus_clock*2,TX4938_CCFG_DIVMODE_2_5|TX4938_CCFG_DIVMODE_10=>txx9_gbus_clock*5/2,TX4938_CCFG_DIVMODE_3|TX4938_CCFG_DIVMODE_12=>txx9_gbus_clock*3,TX4938_CCFG_DIVMODE_4|TX4938_CCFG_DIVMODE_16=>txx9_gbus_clock*4,TX4938_CCFG_DIVMODE_4_5|TX4938_CCFG_DIVMODE_18=>txx9_gbus_clock*9/2,_=>0}; } else { if txx9_cpu_clock==0 {txx9_cpu_clock=300000000;} txx9_gbus_clock=match divmode {TX4938_CCFG_DIVMODE_2|TX4938_CCFG_DIVMODE_8=>txx9_cpu_clock/2,TX4938_CCFG_DIVMODE_2_5|TX4938_CCFG_DIVMODE_10=>txx9_cpu_clock*2/5,TX4938_CCFG_DIVMODE_3|TX4938_CCFG_DIVMODE_12=>txx9_cpu_clock/3,TX4938_CCFG_DIVMODE_4|TX4938_CCFG_DIVMODE_16=>txx9_cpu_clock/4,TX4938_CCFG_DIVMODE_4_5|TX4938_CCFG_DIVMODE_18=>txx9_cpu_clock*2/9,_=>0}; txx9_master_clock=if matches!(divmode,TX4938_CCFG_DIVMODE_8|TX4938_CCFG_DIVMODE_10|TX4938_CCFG_DIVMODE_12|TX4938_CCFG_DIVMODE_16|TX4938_CCFG_DIVMODE_18){txx9_gbus_clock/4}else{txx9_gbus_clock}; }
    loops_per_jiffy=txx9_cpu_clock/HZ/2; tx4938_wdr_init(); tx4938_ccfg_set(TX4938_CCFG_BEOW); if txx9_ccfg_toeon!=0 {tx4938_ccfg_set(TX4938_CCFG_TOE);} txx9_clear64(tx4938_ccfgptr,TX4938_PCFG_DMASEL_ALL); if ____raw_readq(tx4938_ccfgptr)&TX4938_CCFG_PCIARB==0 {txx9_clear64(tx4938_ccfgptr,TX4938_PCFG_PCICLKEN_ALL);} _machine_restart=Some(tx4938_machine_restart); board_be_init=Some(tx4938_be_init);
}

// The remaining entry points retain the original externally visible interface.
pub unsafe extern "C" fn tx4938_time_init(tmrnr:u32) { if ____raw_readq(tx4938_ccfgptr)&TX4938_CCFG_TINTDIS!=0 {txx9_clockevent_init(TX4938_TMR_REG(tmrnr)&0xfffffffff,TXX9_IRQ_BASE+TX4938_IR_TMR(tmrnr),TXX9_IMCLK);} }
pub unsafe extern "C" fn tx4938_sio_init(sclk:u32,cts_mask:u32) { let mut mask=0; if __raw_readq(tx4938_ccfgptr)&TX4938_PCFG_ETH0_SEL!=0 {mask|=2;} for i in 0..2 {if mask&(1<<i)==0 {txx9_sio_init(TX4938_SIO_REG(i)&0xfffffffff,TXX9_IRQ_BASE+TX4938_IR_SIO(i),i,sclk,(1<<i)&cts_mask);}} }
pub unsafe extern "C" fn tx4938_ethaddr_init(a0:*mut u8,a1:*mut u8) {let p=__raw_readq(tx4938_ccfgptr); if !a0.is_null()&&p&TX4938_PCFG_ETH0_SEL!=0 {txx9_ethaddr_init(TXX9_IRQ_BASE+TX4938_IR_ETH0,a0);} if !a1.is_null()&&p&TX4938_PCFG_ETH1_SEL!=0 {txx9_ethaddr_init(TXX9_IRQ_BASE+TX4938_IR_ETH1,a1);}}
pub unsafe extern "C" fn tx4938_mtd_init(ch:i32) { if TX4938_EBUSC_CR(ch)&8==0{return;} let mut p=physmap_flash_data{width:TX4938_EBUSC_WIDTH(ch)/8}; let s=txx9_ce_res[ch as usize].start; txx9_physmap_flash_init(ch,s,txx9_ce_res[ch as usize].end-s+1,&mut p); }
pub unsafe extern "C" fn tx4938_ata_init(irq:u32,shift:u32,tune:i32) { let mut p=tx4938ide_platform_info{ioport_shift:shift,gbus_clock:if tune!=0{txx9_gbus_clock}else{0},ebus_ch:0}; let mut r=[resource{name:core::ptr::null(),start:0,end:0,flags:IORESOURCE_MEM},resource{name:core::ptr::null(),start:irq as usize,end:0,flags:IORESOURCE_IRQ}]; let dev=platform_device_alloc(b"tx4938ide\0".as_ptr(),-1); if !dev.is_null(){r[0].start=0x10000;r[0].end=0x2ffff;p.ebus_ch=0; platform_device_add_resources(dev,r.as_mut_ptr(),2);platform_device_add_data(dev,&mut p as *mut _ as *mut u8,core::mem::size_of_val(&p));platform_device_add(dev);}}
pub unsafe extern "C" fn tx4938_ndfmc_init(hold:u32,spw:u32) { let mut p=txx9ndfmc_platform_data{shift:1,gbus_clock:txx9_gbus_clock,hold,spw,ch_mask:1}; txx9_ndfmc_init(TX4938_NDFMC_REG&0xfffffffff,&mut p); }
pub unsafe extern "C" fn tx4938_dmac_init(a:i32,b:i32) {let mut p=txx9dmac_platform_data{have_64bit_regs:true,memcpy_chan:0};for i in 0..2{p.memcpy_chan=if i==0{a}else{b};txx9_dmac_init(i,TX4938_DMA_REG(i)&0xfffffffff,TXX9_IRQ_BASE+TX4938_IR_DMA(i,0),&mut p);}}
pub unsafe extern "C" fn tx4938_aclc_init() {txx9_aclc_init(TX4938_ACLC_REG&0xfffffffff,TXX9_IRQ_BASE+TX4938_IR_ACLC,1,0,1);}
pub unsafe extern "C" fn tx4938_sramc_init() {if TX4938_SRAM_RESOURCE.start!=0{txx9_sramc_init(&mut TX4938_SRAM_RESOURCE);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
