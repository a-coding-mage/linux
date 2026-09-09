// SPDX-License-Identifier: GPL-2.0
//
// EDAC driver for DMC-520 memory controller.
// The driver supports 10 interrupt lines, though only dram_ecc_errc and
// dram_ecc_errd are currently handled.

// Dependencies supplied by the surrounding kernel/Rust translation.
use core::ffi::{c_char, c_int, c_void};

const REG_OFFSET_FEATURE_CONFIG: u32 = 0x130;
const REG_OFFSET_ECC_ERRC_COUNT_31_00: u32 = 0x158;
const REG_OFFSET_ECC_ERRC_COUNT_63_32: u32 = 0x15c;
const REG_OFFSET_ECC_ERRD_COUNT_31_00: u32 = 0x160;
const REG_OFFSET_ECC_ERRD_COUNT_63_32: u32 = 0x164;
const REG_OFFSET_INTERRUPT_CONTROL: u32 = 0x500;
const REG_OFFSET_INTERRUPT_CLR: u32 = 0x508;
const REG_OFFSET_INTERRUPT_STATUS: u32 = 0x510;
const REG_OFFSET_DRAM_ECC_ERRC_INT_INFO_31_00: u32 = 0x528;
const REG_OFFSET_DRAM_ECC_ERRC_INT_INFO_63_32: u32 = 0x52c;
const REG_OFFSET_DRAM_ECC_ERRD_INT_INFO_31_00: u32 = 0x530;
const REG_OFFSET_DRAM_ECC_ERRD_INT_INFO_63_32: u32 = 0x534;
const REG_OFFSET_ADDRESS_CONTROL_NOW: u32 = 0x1010;
const REG_OFFSET_MEMORY_TYPE_NOW: u32 = 0x1128;
const REG_OFFSET_SCRUB_CONTROL0_NOW: u32 = 0x1170;
const REG_OFFSET_FORMAT_CONTROL: u32 = 0x18;

const RAM_ECC_INT_CE_BIT: u32 = 1 << 0;
const RAM_ECC_INT_UE_BIT: u32 = 1 << 1;
const DRAM_ECC_INT_CE_BIT: u32 = 1 << 2;
const DRAM_ECC_INT_UE_BIT: u32 = 1 << 3;
const FAILED_ACCESS_INT_BIT: u32 = 1 << 4;
const FAILED_PROG_INT_BIT: u32 = 1 << 5;
const LINK_ERR_INT_BIT: u32 = 1 << 6;
const TEMPERATURE_EVENT_INT_BIT: u32 = 1 << 7;
const ARCH_FSM_INT_BIT: u32 = 1 << 8;
const PHY_REQUEST_INT_BIT: u32 = 1 << 9;
const MEMORY_WIDTH_MASK: u32 = 0x3;
const SCRUB_TRIGGER0_NEXT_MASK: u32 = 0x3;
const REG_FIELD_DRAM_ECC_ENABLED: u32 = 0x3;
const REG_FIELD_MEMORY_TYPE: u32 = 0x7;
const REG_FIELD_DEVICE_WIDTH: u32 = 0x300;
const REG_FIELD_ADDRESS_CONTROL_COL: u32 = 0x7;
const REG_FIELD_ADDRESS_CONTROL_ROW: u32 = 0x700;
const REG_FIELD_ADDRESS_CONTROL_BANK: u32 = 0x70000;
const REG_FIELD_ADDRESS_CONTROL_RANK: u32 = 0x3000000;
const REG_FIELD_ERR_INFO_LOW_VALID: u32 = 1;
const REG_FIELD_ERR_INFO_LOW_COL: u32 = 0x7fe;
const REG_FIELD_ERR_INFO_LOW_ROW: u32 = 0x1ffff800;
const REG_FIELD_ERR_INFO_LOW_RANK: u32 = 0xe0000000;
const REG_FIELD_ERR_INFO_HIGH_BANK: u32 = 0xf;
const REG_FIELD_ERR_INFO_HIGH_VALID: u32 = 1 << 31;
const DRAM_ADDRESS_CONTROL_MIN_COL_BITS: u32 = 8;
const DRAM_ADDRESS_CONTROL_MIN_ROW_BITS: u32 = 11;
const DMC520_SCRUB_TRIGGER_ERR_DETECT: u32 = 2;
const DMC520_SCRUB_TRIGGER_IDLE: u32 = 3;
const DMC520_MSG_BUF_SIZE: usize = 40;
const EDAC_MOD_NAME: &[u8] = b"dmc520-edac\0";
const EDAC_CTL_NAME: &[u8] = b"dmc520\0";

#[repr(C)]
#[derive(Copy, Clone)]
enum Dmc520MemWidth { MemWidthX32 = 2, MemWidthX64 = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
enum Dmc520MemType { MemTypeDdr3 = 1, MemTypeDdr4 = 2 }
#[repr(C)]
#[derive(Copy, Clone)]
enum Dmc520DevWidth { DevWidthX4 = 0, DevWidthX8 = 1, DevWidthX16 = 2 }

#[repr(C)]
struct EccErrorInfo { col: u32, row: u32, bank: u32, rank: u32 }
#[repr(C)]
struct Dmc520IrqConfig { name: *mut c_char, mask: c_int }

static mut DMC520_IRQ_CONFIGS: [Dmc520IrqConfig; 10] = [
    Dmc520IrqConfig { name: b"ram_ecc_errc\0" as *const u8 as *mut c_char, mask: RAM_ECC_INT_CE_BIT as c_int },
    Dmc520IrqConfig { name: b"ram_ecc_errd\0" as *const u8 as *mut c_char, mask: RAM_ECC_INT_UE_BIT as c_int },
    Dmc520IrqConfig { name: b"dram_ecc_errc\0" as *const u8 as *mut c_char, mask: DRAM_ECC_INT_CE_BIT as c_int },
    Dmc520IrqConfig { name: b"dram_ecc_errd\0" as *const u8 as *mut c_char, mask: DRAM_ECC_INT_UE_BIT as c_int },
    Dmc520IrqConfig { name: b"failed_access\0" as *const u8 as *mut c_char, mask: FAILED_ACCESS_INT_BIT as c_int },
    Dmc520IrqConfig { name: b"failed_prog\0" as *const u8 as *mut c_char, mask: FAILED_PROG_INT_BIT as c_int },
    Dmc520IrqConfig { name: b"link_err\0" as *const u8 as *mut c_char, mask: LINK_ERR_INT_BIT as c_int },
    Dmc520IrqConfig { name: b"temperature_event\0" as *const u8 as *mut c_char, mask: TEMPERATURE_EVENT_INT_BIT as c_int },
    Dmc520IrqConfig { name: b"arch_fsm\0" as *const u8 as *mut c_char, mask: ARCH_FSM_INT_BIT as c_int },
    Dmc520IrqConfig { name: b"phy_request\0" as *const u8 as *mut c_char, mask: PHY_REQUEST_INT_BIT as c_int },
];

#[repr(C)]
struct Dmc520Edac {
    reg_base: *mut u8,
    error_lock: Spinlock,
    mem_width_in_bytes: u32,
    irqs: [c_int; 10],
    masks: [c_int; 10],
}

#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct MemCtlInfo { pvt_info: *mut Dmc520Edac, nr_csrows: c_int, csrows: *mut *mut CsrowInfo, pdev: *mut Device, mtype_cap: u32, edac_ctl_cap: u32, edac_cap: u32, scrub_cap: u32, scrub_mode: ScrubType, ctl_name: *const u8, dev_name: *const u8, mod_name: *const u8 }
#[repr(C)] struct CsrowInfo { nr_channels: c_int, channels: *mut *mut ChannelInfo }
#[repr(C)] struct ChannelInfo { dimm: *mut DimmInfo }
#[repr(C)] struct DimmInfo { grain: u32, dtype: DevType, mtype: MemType, edac_mode: u32, nr_pages: u32 }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct EdacMcLayer { type_: u32, size: u32, is_virt_csrow: bool }
#[repr(C)] #[derive(Copy, Clone)] enum ScrubType { ScrubNone, ScrubHwProg }
#[repr(C)] #[derive(Copy, Clone)] enum MemType { MemUnknown, MemDdr3, MemDdr4 }
#[repr(C)] #[derive(Copy, Clone)] enum DevType { DevUnknown, DevX4, DevX8, DevX16 }
type IrqReturn = c_int;
const IRQ_NONE: IrqReturn = 0;
const IRQ_HANDLED: IrqReturn = 1;

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(val: u32, addr: *mut u8);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn edac_mc_handle_error(type_: u32, mci: *mut MemCtlInfo, count: u32, page: u64, offset: u32, syndrome: u32, row: u32, channel: c_int, label: c_int, msg: *const c_char, other: *const c_char);
    fn platform_get_irq_byname_optional(pdev: *mut PlatformDevice, name: *mut c_char) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: c_int) -> *mut u8;
    fn edac_mc_alloc(idx: c_int, layers: usize, layer: *mut EdacMcLayer, size: usize) -> *mut MemCtlInfo;
    fn edac_mc_free(mci: *mut MemCtlInfo);
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut MemCtlInfo);
    fn devm_request_irq(dev: *mut Device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturn, flags: u32, name: *const c_char, data: *mut MemCtlInfo) -> c_int;
    fn devm_free_irq(dev: *mut Device, irq: c_int, data: *mut MemCtlInfo);
    fn edac_mc_add_mc(mci: *mut MemCtlInfo) -> c_int;
    fn edac_mc_del_mc(dev: *mut Device);
}

static mut DMC520_MC_IDX: c_int = 0;
#[inline] fn field_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }
unsafe fn dmc520_read_reg(pvt: *mut Dmc520Edac, offset: u32) -> u32 { readl((*pvt).reg_base.add(offset as usize)) }
unsafe fn dmc520_write_reg(pvt: *mut Dmc520Edac, val: u32, offset: u32) { writel(val, (*pvt).reg_base.add(offset as usize)); }
unsafe fn dmc520_calc_dram_ecc_error(mut value: u32) -> u32 { let mut total = 0; while value > 0 { total += value & 0xff; value >>= 8; } total }
unsafe fn dmc520_get_dram_ecc_error_count(pvt: *mut Dmc520Edac, is_ce: bool) -> u32 {
    let (lo, hi) = if is_ce {(REG_OFFSET_ECC_ERRC_COUNT_31_00, REG_OFFSET_ECC_ERRC_COUNT_63_32)} else {(REG_OFFSET_ECC_ERRD_COUNT_31_00, REG_OFFSET_ECC_ERRD_COUNT_63_32)};
    let a = dmc520_read_reg(pvt, lo); let b = dmc520_read_reg(pvt, hi); dmc520_write_reg(pvt, 0, lo); dmc520_write_reg(pvt, 0, hi); dmc520_calc_dram_ecc_error(a) + dmc520_calc_dram_ecc_error(b)
}
unsafe fn dmc520_get_dram_ecc_error_info(pvt: *mut Dmc520Edac, is_ce: bool, info: *mut EccErrorInfo) {
    let (lo, hi) = if is_ce {(REG_OFFSET_DRAM_ECC_ERRC_INT_INFO_31_00, REG_OFFSET_DRAM_ECC_ERRC_INT_INFO_63_32)} else {(REG_OFFSET_DRAM_ECC_ERRD_INT_INFO_31_00, REG_OFFSET_DRAM_ECC_ERRD_INT_INFO_63_32)};
    let a=dmc520_read_reg(pvt,lo); let b=dmc520_read_reg(pvt,hi);
    if field_get(REG_FIELD_ERR_INFO_LOW_VALID,a)!=0 && field_get(REG_FIELD_ERR_INFO_HIGH_VALID,b)!=0 { (*info).col=field_get(REG_FIELD_ERR_INFO_LOW_COL,a); (*info).row=field_get(REG_FIELD_ERR_INFO_LOW_ROW,a); (*info).rank=field_get(REG_FIELD_ERR_INFO_LOW_RANK,a); (*info).bank=field_get(REG_FIELD_ERR_INFO_HIGH_BANK,b); } else { memset(info as *mut c_void,0,core::mem::size_of::<EccErrorInfo>()); }
}
unsafe fn dmc520_is_ecc_enabled(base: *mut u8) -> bool { field_get(REG_FIELD_DRAM_ECC_ENABLED,readl(base.add(REG_OFFSET_FEATURE_CONFIG as usize))) != 0 }
unsafe fn dmc520_get_scrub_type(pvt:*mut Dmc520Edac)->ScrubType { let x=field_get(SCRUB_TRIGGER0_NEXT_MASK,dmc520_read_reg(pvt,REG_OFFSET_SCRUB_CONTROL0_NOW)); if x==DMC520_SCRUB_TRIGGER_ERR_DETECT||x==DMC520_SCRUB_TRIGGER_IDLE {ScrubType::ScrubHwProg} else {ScrubType::ScrubNone} }
unsafe fn dmc520_get_memory_width(pvt:*mut Dmc520Edac)->u32 { match field_get(MEMORY_WIDTH_MASK,dmc520_read_reg(pvt,REG_OFFSET_FORMAT_CONTROL)) { 2=>4, 3=>8, _=>0 } }
unsafe fn dmc520_get_mtype(pvt:*mut Dmc520Edac)->MemType { match field_get(REG_FIELD_MEMORY_TYPE,dmc520_read_reg(pvt,REG_OFFSET_MEMORY_TYPE_NOW)) {1=>MemType::MemDdr3,2=>MemType::MemDdr4,_=>MemType::MemUnknown} }
unsafe fn dmc520_get_dtype(pvt:*mut Dmc520Edac)->DevType { match field_get(REG_FIELD_DEVICE_WIDTH,dmc520_read_reg(pvt,REG_OFFSET_MEMORY_TYPE_NOW)) {0=>DevType::DevX4,1=>DevType::DevX8,2=>DevType::DevX16,_=>DevType::DevUnknown} }
unsafe fn dmc520_get_rank_count(base:*mut u8)->u32 { 1 << field_get(REG_FIELD_ADDRESS_CONTROL_RANK,readl(base.add(REG_OFFSET_ADDRESS_CONTROL_NOW as usize))) }
unsafe fn dmc520_get_rank_size(pvt:*mut Dmc520Edac)->u64 { let v=dmc520_read_reg(pvt,REG_OFFSET_ADDRESS_CONTROL_NOW); let c=field_get(REG_FIELD_ADDRESS_CONTROL_COL,v)+8; let r=field_get(REG_FIELD_ADDRESS_CONTROL_ROW,v)+11; let b=field_get(REG_FIELD_ADDRESS_CONTROL_BANK,v); ((*pvt).mem_width_in_bytes as u64) << (c+r+b) }

unsafe fn dmc520_handle_dram_ecc_errors(mci:*mut MemCtlInfo,is_ce:bool) { let p=(*mci).pvt_info; let mut info=EccErrorInfo{col:0,row:0,bank:0,rank:0}; dmc520_get_dram_ecc_error_info(p,is_ce,&mut info); let cnt=dmc520_get_dram_ecc_error_count(p,is_ce); if cnt==0{return;} let mut msg=[0u8;DMC520_MSG_BUF_SIZE]; let text=format!("rank:{} bank:{} row:{} col:{}",info.rank,info.bank,info.row,info.col); let n=text.len().min(39); msg[..n].copy_from_slice(&text.as_bytes()[..n]); spin_lock(&mut (*p).error_lock); edac_mc_handle_error(if is_ce {1}else{2},mci,cnt,0,0,0,info.rank,-1,-1,msg.as_ptr() as *const c_char,b"\0".as_ptr() as *const c_char); spin_unlock(&mut (*p).error_lock); }
unsafe fn dmc520_edac_dram_ecc_isr(irq:c_int,mci:*mut MemCtlInfo,is_ce:bool)->IrqReturn { let p=(*mci).pvt_info; dmc520_handle_dram_ecc_errors(mci,is_ce); dmc520_write_reg(p,if is_ce{DRAM_ECC_INT_CE_BIT}else{DRAM_ECC_INT_UE_BIT},REG_OFFSET_INTERRUPT_CLR); IRQ_HANDLED }
unsafe fn dmc520_edac_dram_all_isr(irq:c_int,mci:*mut MemCtlInfo,mask:u32)->IrqReturn { let p=(*mci).pvt_info; let s=dmc520_read_reg(p,REG_OFFSET_INTERRUPT_STATUS); let mut ret=IRQ_NONE; if mask&DRAM_ECC_INT_CE_BIT!=0&&s&DRAM_ECC_INT_CE_BIT!=0{ret=dmc520_edac_dram_ecc_isr(irq,mci,true);} if mask&DRAM_ECC_INT_UE_BIT!=0&&s&DRAM_ECC_INT_UE_BIT!=0{ret=dmc520_edac_dram_ecc_isr(irq,mci,false);} ret }
unsafe extern "C" fn dmc520_isr(irq:c_int,data:*mut c_void)->IrqReturn { let mci=data as *mut MemCtlInfo; let p=(*mci).pvt_info; let mut mask=0; for i in 0..10 {if (*p).irqs[i]==irq {mask=(*p).masks[i] as u32;break;}} dmc520_edac_dram_all_isr(irq,mci,mask) }

unsafe fn dmc520_init_csrow(mci:*mut MemCtlInfo) { let p=(*mci).pvt_info; let dt=dmc520_get_dtype(p); let mt=dmc520_get_mtype(p); let pages=(dmc520_get_rank_size(p)>>12) as u32; for row in 0..(*mci).nr_csrows { let csi=*(*mci).csrows.add(row as usize); for ch in 0..(*csi).nr_channels { let d=*(*csi).channels.add(ch as usize); (*d).grain=(*p).mem_width_in_bytes; (*d).dtype=dt; (*d).mtype=mt; (*d).edac_mode=3; (*d).nr_pages=pages/(*csi).nr_channels as u32; } } }

#[no_mangle]
pub unsafe extern "C" fn dmc520_edac_probe(pdev:*mut PlatformDevice)->c_int { let mut irqs=[-6i32;10]; let mut masks=[0i32;10]; let mut all=0u32; for i in 0..10 {irqs[i]=platform_get_irq_byname_optional(pdev,DMC520_IRQ_CONFIGS[i].name); masks[i]=DMC520_IRQ_CONFIGS[i].mask; if irqs[i]>=0 {all|=masks[i] as u32;}} if all==0{return -22;} let base=devm_platform_ioremap_resource(pdev,0); if base.is_null(){return -6;} if !dmc520_is_ecc_enabled(base){return -6;} let mut layer=EdacMcLayer{type_:1,size:dmc520_get_rank_count(base),is_virt_csrow:true}; let mci=edac_mc_alloc(DMC520_MC_IDX,1,&mut layer,core::mem::size_of::<Dmc520Edac>()); DMC520_MC_IDX+=1; if mci.is_null(){return -12;} let p=(*mci).pvt_info; (*p).reg_base=base; (*p).irqs=irqs; (*p).masks=masks; platform_set_drvdata(pdev,mci); (*mci).pdev=&mut (*pdev).dev; (*mci).mtype_cap=3; (*mci).edac_ctl_cap=3; (*mci).edac_cap=2; (*mci).scrub_cap=1; (*mci).scrub_mode=dmc520_get_scrub_type(p); (*mci).ctl_name=EDAC_CTL_NAME.as_ptr(); (*mci).mod_name=EDAC_MOD_NAME.as_ptr(); (*p).mem_width_in_bytes=dmc520_get_memory_width(p); dmc520_init_csrow(mci); let ctl=dmc520_read_reg(p,REG_OFFSET_INTERRUPT_CONTROL); dmc520_write_reg(p,ctl&!all,REG_OFFSET_INTERRUPT_CONTROL); dmc520_write_reg(p,all,REG_OFFSET_INTERRUPT_CLR); for i in 0..10 {if irqs[i]>=0 {let ret=devm_request_irq(&mut (*pdev).dev,irqs[i],dmc520_isr,0x80,b"dmc520\0".as_ptr() as *const c_char,mci); if ret<0 {for j in 0..i {if irqs[j]>=0{devm_free_irq(&mut (*pdev).dev,irqs[j],mci);}} edac_mc_free(mci);return ret;}}} if all&DRAM_ECC_INT_CE_BIT!=0{dmc520_get_dram_ecc_error_count(p,true);} if all&DRAM_ECC_INT_UE_BIT!=0{dmc520_get_dram_ecc_error_count(p,false);} let ret=edac_mc_add_mc(mci); if ret!=0{edac_mc_free(mci);return ret;} dmc520_write_reg(p,ctl|all,REG_OFFSET_INTERRUPT_CONTROL); 0 }

#[no_mangle]
pub unsafe extern "C" fn dmc520_edac_remove(pdev:*mut PlatformDevice) { let mci=platform_get_drvdata(pdev); let p=(*mci).pvt_info; let mut all=0u32; let ctl=dmc520_read_reg(p,REG_OFFSET_INTERRUPT_CONTROL); dmc520_write_reg(p,ctl&!all,REG_OFFSET_INTERRUPT_CONTROL); for i in 0..10 {if (*p).irqs[i]>=0 {all|=(*p).masks[i] as u32;devm_free_irq(&mut (*pdev).dev,(*p).irqs[i],mci);}} edac_mc_del_mc(&mut (*pdev).dev);edac_mc_free(mci); }

extern "C" { fn platform_get_drvdata(pdev:*mut PlatformDevice)->*mut MemCtlInfo; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
