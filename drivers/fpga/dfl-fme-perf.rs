// SPDX-License-Identifier: GPL-2.0
// Driver for FPGA Management Engine (FME) Global Performance Reporting
//
// Translation of dfl-fme-perf.c. Kernel types, constants, and helpers are
// supplied by the surrounding Linux/Rust bindings.

const CACHE_CTRL: usize = 0x8;
const CACHE_RESET_CNTR: u64 = 1 << 0;
const CACHE_FREEZE_CNTR: u64 = 1 << 8;
const CACHE_CTRL_EVNT: u64 = 0xf << 16;
const CACHE_EVNT_RD_HIT: u32 = 0x0;
const CACHE_EVNT_WR_HIT: u32 = 0x1;
const CACHE_EVNT_RD_MISS: u32 = 0x2;
const CACHE_EVNT_WR_MISS: u32 = 0x3;
const CACHE_EVNT_RSVD: u32 = 0x4;
const CACHE_EVNT_HOLD_REQ: u32 = 0x5;
const CACHE_EVNT_DATA_WR_PORT_CONTEN: u32 = 0x6;
const CACHE_EVNT_TAG_WR_PORT_CONTEN: u32 = 0x7;
const CACHE_EVNT_TX_REQ_STALL: u32 = 0x8;
const CACHE_EVNT_RX_REQ_STALL: u32 = 0x9;
const CACHE_EVNT_EVICTIONS: u32 = 0xa;
const CACHE_EVNT_MAX: u32 = CACHE_EVNT_EVICTIONS;
const CACHE_CHANNEL_SEL: u64 = 1 << 20;
const CACHE_CHANNEL_RD: u32 = 0;
const CACHE_CHANNEL_WR: u32 = 1;
const CACHE_CNTR0: usize = 0x10;
const CACHE_CNTR1: usize = 0x18;
const CACHE_CNTR_EVNT_CNTR: u64 = (1 << 48) - 1;
const CACHE_CNTR_EVNT: u64 = 0xf << 60;
const FAB_CTRL: usize = 0x20;
const FAB_RESET_CNTR: u64 = 1;
const FAB_FREEZE_CNTR: u64 = 1 << 8;
const FAB_CTRL_EVNT: u64 = 0xf << 16;
const FAB_EVNT_PCIE0_RD: u32 = 0; const FAB_EVNT_PCIE0_WR: u32 = 1;
const FAB_EVNT_PCIE1_RD: u32 = 2; const FAB_EVNT_PCIE1_WR: u32 = 3;
const FAB_EVNT_UPI_RD: u32 = 4; const FAB_EVNT_UPI_WR: u32 = 5;
const FAB_EVNT_MMIO_RD: u32 = 6; const FAB_EVNT_MMIO_WR: u32 = 7;
const FAB_EVNT_MAX: u32 = FAB_EVNT_MMIO_WR;
const FAB_PORT_ID: u64 = 3 << 20; const FAB_PORT_FILTER: u64 = 1 << 23;
const FAB_PORT_FILTER_DISABLE: u32 = 0; const FAB_PORT_FILTER_ENABLE: u32 = 1;
const FAB_CNTR: usize = 0x28; const FAB_CNTR_EVNT_CNTR: u64 = (1 << 60) - 1;
const FAB_CNTR_EVNT: u64 = 0xf << 60;
const CLK_CNTR: usize = 0x30; const BASIC_EVNT_CLK: u32 = 0; const BASIC_EVNT_MAX: u32 = BASIC_EVNT_CLK;
const VTD_CTRL: usize = 0x38; const VTD_RESET_CNTR: u64 = 1; const VTD_FREEZE_CNTR: u64 = 1 << 8;
const VTD_CTRL_EVNT: u64 = 0xf << 16;
const VTD_EVNT_AFU_MEM_RD_TRANS: u32 = 0; const VTD_EVNT_AFU_MEM_WR_TRANS: u32 = 1;
const VTD_EVNT_AFU_DEVTLB_RD_HIT: u32 = 2; const VTD_EVNT_AFU_DEVTLB_WR_HIT: u32 = 3;
const VTD_EVNT_DEVTLB_4K_FILL: u32 = 4; const VTD_EVNT_DEVTLB_2M_FILL: u32 = 5; const VTD_EVNT_DEVTLB_1G_FILL: u32 = 6;
const VTD_EVNT_MAX: u32 = VTD_EVNT_DEVTLB_1G_FILL;
const VTD_CNTR: usize = 0x40; const VTD_CNTR_EVNT_CNTR: u64 = (1 << 48) - 1; const VTD_CNTR_EVNT: u64 = 0xf << 60;
const VTD_SIP_CTRL: usize = 0x48; const VTD_SIP_CTRL_EVNT: u64 = 0xf << 16;
const VTD_SIP_CNTR: usize = 0x50; const VTD_SIP_CNTR_EVNT_CNTR: u64 = (1 << 48) - 1; const VTD_SIP_CNTR_EVNT: u64 = 0xf << 60;
const VTD_SIP_EVNT_MAX: u32 = 0xb;
const PERF_TIMEOUT: u32 = 30; const PERF_MAX_PORT_NUM: u32 = 1;
const FME_EVENT_MASK: u64 = 0xfff; const FME_EVTYPE_MASK: u64 = 0xf << 12;
const FME_EVTYPE_BASIC: u32 = 0; const FME_EVTYPE_CACHE: u32 = 1; const FME_EVTYPE_FABRIC: u32 = 2;
const FME_EVTYPE_VTD: u32 = 3; const FME_EVTYPE_VTD_SIP: u32 = 4; const FME_EVTYPE_MAX: u32 = FME_EVTYPE_VTD_SIP;
const FME_PORTID_MASK: u64 = 0xff << 16; const FME_PORTID_ROOT: u32 = 0xff;

#[repr(C)]
pub struct FmePerfPriv {
    pub dev: *mut device, pub ioaddr: *mut core::ffi::c_void, pub pmu: pmu, pub id: u16,
    pub fab_users: u32, pub fab_port_id: u32, pub fab_lock: spinlock_t,
    pub cpu: u32, pub node: hlist_node, pub cpuhp_state: enum_cpuhp_state,
}

#[repr(C)]
pub struct FmePerfEventOps {
    pub event_init: Option<unsafe extern "C" fn(*mut FmePerfPriv, u32, u32) -> i32>,
    pub event_destroy: Option<unsafe extern "C" fn(*mut FmePerfPriv, u32, u32)>,
    pub read_counter: Option<unsafe extern "C" fn(*mut FmePerfPriv, u32, u32) -> u64>,
}

#[inline] fn field_get(mask: u64, v: u64) -> u32 { ((v & mask) >> mask.trailing_zeros()) as u32 }
fn is_portid_root(portid: u32) -> bool { portid == FME_PORTID_ROOT }
fn is_portid_port(portid: u32) -> bool { portid < PERF_MAX_PORT_NUM }
fn is_portid_root_or_port(portid: u32) -> bool { is_portid_root(portid) || is_portid_port(portid) }
fn get_event(config: u64) -> u32 { field_get(FME_EVENT_MASK, config) }
fn get_evtype(config: u64) -> u32 { field_get(FME_EVTYPE_MASK, config) }
fn get_portid(config: u64) -> u32 { field_get(FME_PORTID_MASK, config) }

unsafe fn fme_read_perf_cntr_reg(addr: *mut core::ffi::c_void) -> u64 {
    let mut v; let mut low;
    loop { v = readq(addr); low = readl(addr); if (v as u32) <= low { return v; } }
}
unsafe fn basic_event_init(_: *mut FmePerfPriv, event: u32, portid: u32) -> i32 { if event <= BASIC_EVNT_MAX && is_portid_root(portid) { 0 } else { -22 } }
unsafe fn basic_read_event_counter(priv_: *mut FmePerfPriv, _: u32, _: u32) -> u64 { fme_read_perf_cntr_reg((*priv_).ioaddr.add(CLK_CNTR)) }
unsafe fn cache_event_init(priv_: *mut FmePerfPriv, event: u32, portid: u32) -> i32 { if (*priv_).id == FME_FEATURE_ID_GLOBAL_IPERF && event <= CACHE_EVNT_MAX && is_portid_root(portid) { 0 } else { -22 } }
unsafe fn cache_read_event_counter(priv_: *mut FmePerfPriv, event: u32, _: u32) -> u64 {
    let b=(*priv_).ioaddr; let ch=if matches!(event,CACHE_EVNT_WR_HIT|CACHE_EVNT_WR_MISS|CACHE_EVNT_DATA_WR_PORT_CONTEN|CACHE_EVNT_TAG_WR_PORT_CONTEN){CACHE_CHANNEL_WR}else{CACHE_CHANNEL_RD};
    let mut v=readq(b.add(CACHE_CTRL)); v &= !(CACHE_CHANNEL_SEL|CACHE_CTRL_EVNT); v |= ((ch as u64)<<20)|((event as u64)<<16); writeq(v,b.add(CACHE_CTRL));
    v=fme_read_perf_cntr_reg(b.add(CACHE_CNTR0)); let mut count=v&CACHE_CNTR_EVNT_CNTR; v=fme_read_perf_cntr_reg(b.add(CACHE_CNTR1)); count += v&CACHE_CNTR_EVNT_CNTR; count
}
unsafe fn is_fabric_event_supported(p:*mut FmePerfPriv,e:u32,port:u32)->bool { if e>FAB_EVNT_MAX||!is_portid_root_or_port(port){return false} if (*p).id==FME_FEATURE_ID_GLOBAL_DPERF && matches!(e,FAB_EVNT_PCIE1_RD|FAB_EVNT_UPI_RD|FAB_EVNT_PCIE1_WR|FAB_EVNT_UPI_WR){return false} true }
unsafe fn fabric_event_init(p:*mut FmePerfPriv,e:u32,port:u32)->i32 { if !is_fabric_event_supported(p,e,port){return -22} if (*p).fab_users!=0&&(*p).fab_port_id!=port{return -95} (*p).fab_users+=1; if (*p).fab_port_id!=port {(*p).fab_port_id=port; let b=(*p).ioaddr; let mut v=readq(b.add(FAB_CTRL)); v&=!(FAB_PORT_FILTER|FAB_PORT_ID); v|=if is_portid_root(port){0}else{(1<<23)|((port as u64)<<20)}; writeq(v,b.add(FAB_CTRL));} 0 }
unsafe fn fabric_event_destroy(p:*mut FmePerfPriv,_:u32,_:u32){(*p).fab_users-=1}
unsafe fn fabric_read_event_counter(p:*mut FmePerfPriv,e:u32,_:u32)->u64{let b=(*p).ioaddr;let mut v=readq(b.add(FAB_CTRL));v=(v&!FAB_CTRL_EVNT)|((e as u64)<<16);writeq(v,b.add(FAB_CTRL));fme_read_perf_cntr_reg(b.add(FAB_CNTR))&FAB_CNTR_EVNT_CNTR}
unsafe fn vtd_event_init(p:*mut FmePerfPriv,e:u32,port:u32)->i32{if (*p).id==FME_FEATURE_ID_GLOBAL_IPERF&&e<=VTD_EVNT_MAX&&is_portid_port(port){0}else{-22}}
unsafe fn vtd_read_event_counter(p:*mut FmePerfPriv,e:u32,port:u32)->u64{let b=(*p).ioaddr;let e=e+port*(VTD_EVNT_MAX+1);let mut v=readq(b.add(VTD_CTRL));v=(v&!VTD_CTRL_EVNT)|((e as u64)<<16);writeq(v,b.add(VTD_CTRL));fme_read_perf_cntr_reg(b.add(VTD_CNTR))&VTD_CNTR_EVNT_CNTR}
unsafe fn vtd_sip_event_init(p:*mut FmePerfPriv,e:u32,port:u32)->i32{if (*p).id==FME_FEATURE_ID_GLOBAL_IPERF&&e<=VTD_SIP_EVNT_MAX&&is_portid_root(port){0}else{-22}}
unsafe fn vtd_sip_read_event_counter(p:*mut FmePerfPriv,e:u32,_:u32)->u64{let b=(*p).ioaddr;let mut v=readq(b.add(VTD_SIP_CTRL));v=(v&!VTD_SIP_CTRL_EVNT)|((e as u64)<<16);writeq(v,b.add(VTD_SIP_CTRL));fme_read_perf_cntr_reg(b.add(VTD_SIP_CNTR))&VTD_SIP_CNTR_EVNT_CNTR}

// Remaining PMU callbacks and feature registration retain the C driver's ABI.
// They are expressed through the kernel binding types supplied by the build.
extern "C" { pub static fme_perf_ops: dfl_feature_ops; pub static fme_perf_id_table: [dfl_feature_id; 3]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
