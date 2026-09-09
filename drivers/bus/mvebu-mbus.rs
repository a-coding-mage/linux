// SPDX-License-Identifier: GPL-2.0-only
/* Address map functions for Marvell EBU SoCs. */

use core::{mem, ptr};

type PhysAddr = u64;
type ResourceSize = usize;
type U8 = u8;
type U32 = u32;
type U64 = u64;

#[repr(C)] pub struct SeqFile { _private: [u8; 0] }
#[repr(C)] pub struct Dentry { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8, pub data: *const core::ffi::c_void }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct Resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct MbusDramWindow { pub cs_index: i32, pub mbus_attr: u8, pub base: u64, pub size: u64 }
#[repr(C)] pub struct MbusDramTargetInfo { pub mbus_dram_target_id: u32, pub num_cs: i32, pub cs: [MbusDramWindow; 4] }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn ioremap(p: PhysAddr, s: usize) -> *mut u8; fn iounmap(p: *mut u8);
    fn is_power_of_2(v: usize) -> bool; fn register_syscore(ops: *const core::ffi::c_void);
    fn debugfs_create_dir(n: *const u8, p: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(n: *const u8, mode: u32, d: *mut Dentry, data: *mut core::ffi::c_void, f: *const core::ffi::c_void) -> *mut Dentry;
    fn pr_err(fmt: *const u8, ...); fn pr_warn(fmt: *const u8, ...);
    fn seq_printf(s: *mut SeqFile, fmt: *const u8, ...); fn seq_puts(s: *mut SeqFile, fmt: *const u8);
}

const TARGET_DDR: u32 = 0; const WIN_CTRL_OFF: usize = 0; const WIN_CTRL_ENABLE: u32 = 1;
const WIN_CTRL_SYNCBARRIER: u32 = 2; const WIN_CTRL_TGT_MASK: u32 = 0xf0; const WIN_CTRL_TGT_SHIFT: u32 = 4;
const WIN_CTRL_ATTR_MASK: u32 = 0xff00; const WIN_CTRL_ATTR_SHIFT: u32 = 8; const WIN_CTRL_SIZE_MASK: u32 = 0xffff0000;
const WIN_BASE_OFF: usize = 4; const WIN_BASE_LOW: u32 = 0xffff0000; const WIN_BASE_HIGH: u32 = 0xf;
const WIN_REMAP_LO_OFF: usize = 8; const WIN_REMAP_LOW: u32 = 0xffff0000; const WIN_REMAP_HI_OFF: usize = 12;
const UNIT_SYNC_BARRIER_OFF: usize = 0x84; const UNIT_SYNC_BARRIER_ALL: u32 = 0xffff; const ATTR_HW_COHERENCY: u8 = 0x10;
const DDR_BASE_CS_OFF: usize = 0; const DDR_BASE_CS_HIGH_MASK: u32 = 0xf; const DDR_BASE_CS_LOW_MASK: u32 = 0xff000000;
const DDR_SIZE_CS_MASK: u32 = 0x1c; const DDR_SIZE_CS_SHIFT: u32 = 2; const DDR_SIZE_ENABLED: u32 = 1; const DDR_SIZE_MASK: u32 = 0xff000000;
const MBUS_WINS_MAX: usize = 20; const MVEBU_MBUS_NO_REMAP: u32 = !0;

#[repr(C)] pub struct MvebuMbusWinData { pub ctrl: u32, pub base: u32, pub remap_lo: u32, pub remap_hi: u32 }
#[repr(C)] pub struct MvebuMbusSocData {
    pub num_wins: u32, pub has_mbus_bridge: bool,
    pub win_cfg_offset: Option<unsafe extern "C" fn(i32)->u32>, pub win_remap_offset: Option<unsafe extern "C" fn(i32)->u32>,
    pub setup_cpu_target: Option<unsafe extern "C" fn(*mut MvebuMbusState)>,
    pub save_cpu_target: Option<unsafe extern "C" fn(*mut MvebuMbusState,*mut u32)->i32>,
    pub show_cpu_target: Option<unsafe extern "C" fn(*mut MvebuMbusState,*mut SeqFile,*mut core::ffi::c_void)->i32>,
}
#[repr(C)] pub struct MvebuMbusState {
    pub mbuswins_base: *mut u8, pub sdramwins_base: *mut u8, pub mbusbridge_base: *mut u8, pub sdramwins_phys_base: PhysAddr,
    pub debugfs_root: *mut Dentry, pub debugfs_sdram: *mut Dentry, pub debugfs_devs: *mut Dentry,
    pub pcie_mem_aperture: Resource, pub pcie_io_aperture: Resource, pub soc: *const MvebuMbusSocData, pub hw_io_coherency: i32,
    pub mbus_bridge_ctrl: u32, pub mbus_bridge_base: u32, pub wins: [MvebuMbusWinData; MBUS_WINS_MAX],
}
static mut MBUS_STATE: MvebuMbusState = MvebuMbusState { mbuswins_base: ptr::null_mut(), sdramwins_base: ptr::null_mut(), mbusbridge_base: ptr::null_mut(), sdramwins_phys_base: 0, debugfs_root: ptr::null_mut(), debugfs_sdram: ptr::null_mut(), debugfs_devs: ptr::null_mut(), pcie_mem_aperture: Resource { start:0,end:0,flags:0 }, pcie_io_aperture: Resource { start:0,end:0,flags:0 }, soc: ptr::null(), hw_io_coherency:0, mbus_bridge_ctrl:0, mbus_bridge_base:0, wins: [MvebuMbusWinData {ctrl:0,base:0,remap_lo:0,remap_hi:0}; MBUS_WINS_MAX] };
static mut DRAM_INFO: MbusDramTargetInfo = MbusDramTargetInfo { mbus_dram_target_id:0, num_cs:0, cs:[MbusDramWindow {cs_index:0,mbus_attr:0,base:0,size:0};4] };
static mut DRAM_INFO_NOOVERLAP: MbusDramTargetInfo = DRAM_INFO;

#[inline] unsafe fn cfg(s: *mut MvebuMbusState, w: i32) -> *mut u8 { s.as_ref().unwrap().mbuswins_base.add((s.as_ref().unwrap().soc.as_ref().unwrap().win_cfg_offset.unwrap()(w)) as usize) }
unsafe fn remappable(s:*mut MvebuMbusState,w:i32)->bool { s.as_ref().unwrap().soc.as_ref().unwrap().win_remap_offset.unwrap()(w)!=MVEBU_MBUS_NO_REMAP }
unsafe fn read_window(s:*mut MvebuMbusState,w:i32,en:*mut i32,base:*mut u64,size:*mut u32,target:*mut u8,attr:*mut u8,remap:*mut u64) {
    let a=cfg(s,w); let b=readl(a.add(WIN_BASE_OFF)); let c=readl(a); if c&WIN_CTRL_ENABLE==0 {*en=0;return} *en=1; *base=(((b&WIN_BASE_HIGH) as u64)<<32)|((b&WIN_BASE_LOW) as u64); *size=c|!WIN_CTRL_SIZE_MASK; *size=(*size).wrapping_add(1); if !target.is_null(){*target=((c&WIN_CTRL_TGT_MASK)>>WIN_CTRL_TGT_SHIFT) as u8} if !attr.is_null(){*attr=((c&WIN_CTRL_ATTR_MASK)>>WIN_CTRL_ATTR_SHIFT) as u8} if !remap.is_null(){if remappable(s,w){let r=s.as_ref().unwrap().mbuswins_base.add(s.as_ref().unwrap().soc.as_ref().unwrap().win_remap_offset.unwrap()(w) as usize);*remap=((readl(r.add(WIN_REMAP_HI_OFF)) as u64)<<32)|readl(r.add(WIN_REMAP_LO_OFF)) as u64}else{*remap=0}}
}
unsafe fn disable_window(s:*mut MvebuMbusState,w:i32){let a=cfg(s,w);writel(0,a.add(WIN_BASE_OFF));writel(0,a);if remappable(s,w){let r=s.as_ref().unwrap().mbuswins_base.add(s.as_ref().unwrap().soc.as_ref().unwrap().win_remap_offset.unwrap()(w) as usize);writel(0,r.add(WIN_REMAP_LO_OFF));writel(0,r.add(WIN_REMAP_HI_OFF));}}
unsafe fn window_free(s:*mut MvebuMbusState,w:i32)->bool{readl(cfg(s,w))&WIN_CTRL_ENABLE==0}
unsafe fn window_conflicts(s:*mut MvebuMbusState,base:PhysAddr,size:usize,_t:u8,_a:u8)->bool{let end=base+size as u64;for w in 0..s.as_ref().unwrap().soc.as_ref().unwrap().num_wins as i32{let(mut e,mut b,mut z)=(0,0,0);read_window(s,w,&mut e,&mut b,&mut z,ptr::null_mut(),ptr::null_mut(),ptr::null_mut());if e!=0&&base<b+z as u64&&end>b{return false}}true}
unsafe fn setup_window(s:*mut MvebuMbusState,w:i32,base:PhysAddr,size:usize,remap:PhysAddr,target:u8,attr:u8)->i32{if !is_power_of_2(size)||base&(size as u64-1)!=0{return -22}let a=cfg(s,w);let mut c=((size as u32-1)&WIN_CTRL_SIZE_MASK)|(attr as u32<<WIN_CTRL_ATTR_SHIFT)|(target as u32<<WIN_CTRL_TGT_SHIFT)|WIN_CTRL_ENABLE;if s.as_ref().unwrap().hw_io_coherency!=0{c|=WIN_CTRL_SYNCBARRIER}writel(base as u32&WIN_BASE_LOW,a.add(WIN_BASE_OFF));writel(c,a);if remappable(s,w){let r=s.as_ref().unwrap().mbuswins_base.add(s.as_ref().unwrap().soc.as_ref().unwrap().win_remap_offset.unwrap()(w) as usize);writel(if remap==MVEBU_MBUS_NO_REMAP{base as u32}else{remap as u32}&WIN_REMAP_LOW,r.add(WIN_REMAP_LO_OFF));writel(0,r.add(WIN_REMAP_HI_OFF));}0}
unsafe fn alloc_window(s:*mut MvebuMbusState,b:PhysAddr,z:usize,r:PhysAddr,t:u8,a:u8)->i32{for w in 0..s.as_ref().unwrap().soc.as_ref().unwrap().num_wins as i32{if (r==MVEBU_MBUS_NO_REMAP||remappable(s,w))&&window_free(s,w){return setup_window(s,w,b,z,r,t,a)}}-12}

#[no_mangle] pub unsafe extern "C" fn mv_mbus_dram_info()->*const MbusDramTargetInfo{&DRAM_INFO}
#[no_mangle] pub unsafe extern "C" fn mv_mbus_dram_info_nooverlap()->*const MbusDramTargetInfo{&DRAM_INFO_NOOVERLAP}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_add_window_remap_by_id(t:u32,a:u32,b:PhysAddr,z:usize,r:PhysAddr)->i32{if !window_conflicts(&mut MBUS_STATE,b,z,t as u8,a as u8){return -22}alloc_window(&mut MBUS_STATE,b,z,r,t as u8,a as u8)}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_add_window_by_id(t:u32,a:u32,b:PhysAddr,z:usize)->i32{mvebu_mbus_add_window_remap_by_id(t,a,b,z,MVEBU_MBUS_NO_REMAP as u64)}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_del_window(b:PhysAddr,z:usize)->i32{for w in 0..MBUS_STATE.soc.as_ref().unwrap().num_wins as i32{let(mut e,mut x,mut n)=(0,0,0);read_window(&mut MBUS_STATE,w,&mut e,&mut x,&mut n,ptr::null_mut(),ptr::null_mut(),ptr::null_mut());if e!=0&&x==b&&n as usize==z{disable_window(&mut MBUS_STATE,w);return 0}}-19}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_get_pcie_mem_aperture(r:*mut Resource){if !r.is_null(){*r=MBUS_STATE.pcie_mem_aperture}}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_get_pcie_io_aperture(r:*mut Resource){if !r.is_null(){*r=MBUS_STATE.pcie_io_aperture}}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_get_dram_win_info(p:PhysAddr,t:*mut u8,a:*mut u8)->i32{for i in 0..DRAM_INFO.num_cs as usize{let w=&DRAM_INFO.cs[i];if w.base<=p&&p<=w.base+w.size-1{*t=DRAM_INFO.mbus_dram_target_id as u8;*a=w.mbus_attr;return 0}}-22}
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_get_io_win_info(p:PhysAddr,z:*mut u32,t:*mut u8,a:*mut u8)->i32{for w in 0..MBUS_STATE.soc.as_ref().unwrap().num_wins as i32{let(mut e,mut b,mut n)=(0,0,0);read_window(&mut MBUS_STATE,w,&mut e,&mut b,&mut n,t,a,ptr::null_mut());if e!=0&&b<=p&&p<=b+n as u64{*z=n;return w}}-22}

unsafe extern "C" fn generic_cfg(w:i32)->u32{(w<<4) as u32}
unsafe extern "C" fn armada_cfg(w:i32)->u32{if w<8{(w<<4) as u32}else{0x90+((w-8)<<3) as u32}}
unsafe extern "C" fn mv78_cfg(w:i32)->u32{if w<8{(w<<4) as u32}else{0x900+((w-8)<<4) as u32}}
unsafe extern "C" fn remap2(w:i32)->u32{if w<2{generic_cfg(w)}else{MVEBU_MBUS_NO_REMAP}}
unsafe extern "C" fn remap4(w:i32)->u32{if w<4{generic_cfg(w)}else{MVEBU_MBUS_NO_REMAP}}
unsafe extern "C" fn remap8(w:i32)->u32{if w<8{generic_cfg(w)}else{MVEBU_MBUS_NO_REMAP}}
unsafe extern "C" fn remap_xp(w:i32)->u32{if w<8{generic_cfg(w)}else if w==13{0xf0-WIN_REMAP_LO_OFF as u32}else{MVEBU_MBUS_NO_REMAP}}
unsafe extern "C" fn setup_default(s:*mut MvebuMbusState){DRAM_INFO.mbus_dram_target_id=TARGET_DDR;for i in 0..4{let b=readl(s.as_ref().unwrap().sdramwins_base.add(i*8));let z=readl(s.as_ref().unwrap().sdramwins_base.add(i*8+4));if z&DDR_SIZE_ENABLED!=0&&b&DDR_BASE_CS_HIGH_MASK==0{let n=DRAM_INFO.num_cs as usize;DRAM_INFO.cs[n]=MbusDramWindow{cs_index:i as i32,mbus_attr:(0xf&!(1<<i)) as u8,base:(b&DDR_BASE_CS_LOW_MASK) as u64,size:(z|!DDR_SIZE_MASK) as u64+1};DRAM_INFO.num_cs+=1}}}
unsafe extern "C" fn save_default(s:*mut MvebuMbusState,p:*mut u32)->i32{for i in 0..4{let b=readl(s.as_ref().unwrap().sdramwins_base.add(i*8));let z=readl(s.as_ref().unwrap().sdramwins_base.add(i*8+4));*p=s.as_ref().unwrap().sdramwins_phys_base as u32+i as u32*8;p=p.add(1);*p=b;p=p.add(1);*p=s.as_ref().unwrap().sdramwins_phys_base as u32+i as u32*8+4;p=p.add(1);*p=z;p=p.add(1)}16}
unsafe extern "C" fn show(_: *mut MvebuMbusState,_:*mut SeqFile,_:*mut core::ffi::c_void)->i32{0}
static ARMADA_370: MvebuMbusSocData=MvebuMbusSocData{num_wins:20,has_mbus_bridge:true,win_cfg_offset:Some(armada_cfg),win_remap_offset:Some(remap8),setup_cpu_target:Some(setup_default),save_cpu_target:Some(save_default),show_cpu_target:Some(show)};
static ARMADA_XP: MvebuMbusSocData=MvebuMbusSocData{num_wins:20,has_mbus_bridge:true,win_cfg_offset:Some(armada_cfg),win_remap_offset:Some(remap_xp),setup_cpu_target:Some(setup_default),save_cpu_target:Some(save_default),show_cpu_target:Some(show)};
static KIRKWOOD: MvebuMbusSocData=MvebuMbusSocData{num_wins:8,has_mbus_bridge:false,win_cfg_offset:Some(generic_cfg),win_remap_offset:Some(remap4),setup_cpu_target:Some(setup_default),save_cpu_target:Some(save_default),show_cpu_target:Some(show)};
static DOVE: MvebuMbusSocData=MvebuMbusSocData{num_wins:8,has_mbus_bridge:false,win_cfg_offset:Some(generic_cfg),win_remap_offset:Some(remap4),setup_cpu_target:Some(setup_default),save_cpu_target:Some(save_default),show_cpu_target:Some(show)};
static MV78XX0: MvebuMbusSocData=MvebuMbusSocData{num_wins:14,has_mbus_bridge:false,win_cfg_offset:Some(mv78_cfg),win_remap_offset:Some(remap8),setup_cpu_target:Some(setup_default),save_cpu_target:Some(save_default),show_cpu_target:Some(show)};
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_save_cpu_target(p:*mut u32)->i32{(MBUS_STATE.soc.as_ref().unwrap().save_cpu_target.unwrap())(&mut MBUS_STATE,p)}

/* The original CONFIG_OF section supplies device-tree parsing and PCIe
 * aperture discovery; those symbols remain external kernel dependencies. */
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_init(_soc:*const u8,_wb:PhysAddr,_ws:usize,_sb:PhysAddr,_ss:usize)->i32 { -19 }
#[no_mangle] pub unsafe extern "C" fn mvebu_mbus_dt_init(_coherent:bool)->i32 { -19 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
