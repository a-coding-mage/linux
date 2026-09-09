/* Translated from asm/uv/uv_hub.h. Kernel dependencies are supplied externally. */

// CONFIG_X86_64 conditional content from the source header.

pub const UV_MAX_NUMALINK_BLADES: usize = 16384;
pub const UV_MAX_SSI_BLADES: usize = 256;
pub const UV_MAX_NASID_VALUE: usize = UV_MAX_NUMALINK_BLADES * 2;

#[repr(C)]
pub struct uv_gam_range_s {
    pub limit: u32,
    pub nasid: u16,
    pub base: i8,
    pub reserved: u8,
}

#[repr(C)]
pub struct uv_hub_info_s {
    pub hub_type: u32,
    pub hub_revision: u8,
    pub global_mmr_base: usize,
    pub global_mmr_shift: usize,
    pub gpa_mask: usize,
    pub socket_to_node: *mut u16,
    pub socket_to_pnode: *mut u16,
    pub pnode_to_socket: *mut u16,
    pub gr_table: *mut uv_gam_range_s,
    pub min_socket: u16,
    pub min_pnode: u16,
    pub m_val: u8,
    pub n_val: u8,
    pub gr_table_len: u8,
    pub apic_pnode_shift: u8,
    pub gpa_shift: u8,
    pub nasid_shift: u8,
    pub m_shift: u8,
    pub n_lshift: u8,
    pub gnode_extra: u32,
    pub gnode_upper: usize,
    pub lowmem_remap_top: usize,
    pub lowmem_remap_base: usize,
    pub global_gru_base: usize,
    pub global_gru_shift: usize,
    pub pnode: u16,
    pub pnode_mask: u16,
    pub coherency_domain_number: u16,
    pub numa_blade_id: u16,
    pub nr_possible_cpus: u16,
    pub nr_online_cpus: u16,
    pub memory_nid: i16,
    pub node_to_socket: *mut u16,
}

#[repr(C)]
pub struct uv_cpu_info_s {
    pub p_uv_hub_info: *mut core::ffi::c_void,
    pub blade_cpu_id: u8,
    pub reserved: *mut core::ffi::c_void,
}

extern "C" {
    pub static mut __uv_hub_info_list: *mut *mut core::ffi::c_void;
    pub static mut uv_possible_blades: i16;
    pub fn uv_nmi_setup();
    pub fn uv_nmi_setup_hubless();
    pub fn __pa(v: *mut core::ffi::c_void) -> usize;
    pub fn __va(v: usize) -> *mut core::ffi::c_void;
    pub fn pr_crit(fmt: *const i8, ...);
    pub fn BUG() -> !;
}

extern "C" {
    pub static mut __uv_cpu_info: uv_cpu_info_s;
}

#[inline]
pub unsafe fn uv_cpu_info() -> *mut uv_cpu_info_s { &mut __uv_cpu_info }
#[inline]
pub unsafe fn uv_cpu_info_per(_cpu: i32) -> *mut uv_cpu_info_s { &mut __uv_cpu_info }

#[inline]
pub unsafe fn uv_hub_info_list(node: i32) -> *mut uv_hub_info_s {
    __uv_hub_info_list.add(node as usize) as *mut *mut uv_hub_info_s as *mut uv_hub_info_s
}
#[inline]
pub unsafe fn _uv_hub_info() -> *mut uv_hub_info_s { (*uv_cpu_info()).p_uv_hub_info as *mut uv_hub_info_s }
#[inline]
pub unsafe fn uv_cpu_hub_info(cpu: i32) -> *mut uv_hub_info_s { (*uv_cpu_info_per(cpu)).p_uv_hub_info as *mut uv_hub_info_s }
#[inline] pub unsafe fn uv_hub_type() -> i32 { (*_uv_hub_info()).hub_type as i32 }
#[inline] pub unsafe fn uv_hub_type_set(uvmask: i32) { (*_uv_hub_info()).hub_type = uvmask as u32; }

pub const UV2_HUB_REVISION_BASE: i32 = 3;
pub const UV3_HUB_REVISION_BASE: i32 = 5;
pub const UV4_HUB_REVISION_BASE: i32 = 7;
pub const UV4A_HUB_REVISION_BASE: i32 = 8;
pub const UV5_HUB_REVISION_BASE: i32 = 9;

extern "C" { pub const UV2: i32; pub const UV3: i32; pub const UV4: i32; pub const UV4A: i32; pub const UV5: i32; pub const UVX: i32; pub const UVY: i32; pub const UV_ANY: i32; pub const UV_GAM_RANGE_SHFT: u32; pub const HZ: usize; }
#[inline] pub unsafe fn is_uv(uvmask: i32) -> i32 { uv_hub_type() & uvmask }
#[inline] pub unsafe fn is_uv1_hub() -> i32 { 0 }
#[inline] pub unsafe fn is_uv2_hub() -> i32 { is_uv(UV2) }
#[inline] pub unsafe fn is_uv3_hub() -> i32 { is_uv(UV3) }
#[inline] pub unsafe fn is_uv4a_hub() -> i32 { is_uv(UV4A) }
#[inline] pub unsafe fn is_uv4_hub() -> i32 { is_uv(UV4) }
#[inline] pub unsafe fn is_uv5_hub() -> i32 { is_uv(UV5) }
#[inline] pub unsafe fn is_uvx_hub() -> i32 { is_uv(UVX) }
#[inline] pub unsafe fn is_uvy_hub() -> i32 { is_uv(UVY) }
#[inline] pub unsafe fn is_uv_hub() -> i32 { is_uv(UV_ANY) }

#[repr(C)]
pub struct uvh_apicid_s { pub local_apic_mask: u64, pub local_apic_shift: u64, pub unused1: u64, pub pnode_mask: u64, pub pnode_shift: u64, pub unused2: u64 }
#[repr(C)] pub union uvh_apicid { pub v: usize, pub s: uvh_apicid_s }

pub const UV2_LOCAL_MMR_BASE: usize = 0xfa000000;
pub const UV2_GLOBAL_MMR32_BASE: usize = 0xfc000000;
pub const UV2_LOCAL_MMR_SIZE: usize = 32 * 1024 * 1024;
pub const UV2_GLOBAL_MMR32_SIZE: usize = 32 * 1024 * 1024;
pub const UV3_LOCAL_MMR_BASE: usize = 0xfa000000;
pub const UV3_GLOBAL_MMR32_BASE: usize = 0xfc000000;
pub const UV3_LOCAL_MMR_SIZE: usize = 32 * 1024 * 1024;
pub const UV3_GLOBAL_MMR32_SIZE: usize = 32 * 1024 * 1024;
pub const UV4_LOCAL_MMR_BASE: usize = 0xfa000000;
pub const UV4_GLOBAL_MMR32_BASE: usize = 0;
pub const UV4_LOCAL_MMR_SIZE: usize = 32 * 1024 * 1024;
pub const UV4_GLOBAL_MMR32_SIZE: usize = 0;
pub const UV5_LOCAL_MMR_BASE: usize = 0xfa000000;
pub const UV5_GLOBAL_MMR32_BASE: usize = 0;
pub const UV5_LOCAL_MMR_SIZE: usize = 32 * 1024 * 1024;
pub const UV5_GLOBAL_MMR32_SIZE: usize = 0;
pub const UV_GLOBAL_GRU_MMR_BASE: usize = 0x4000000;
pub const UV_GLOBAL_MMR32_PNODE_SHIFT: usize = 15;
pub const _UV_GLOBAL_MMR64_PNODE_SHIFT: usize = 26;
pub const UVH_APICID: usize = 0x002d0e00;
pub const UV_APIC_PNODE_SHIFT: usize = 6;
pub const LOCAL_BUS_BASE: usize = 0x1c00000;
pub const LOCAL_BUS_SIZE: usize = 4 * 1024 * 1024;
pub const SCIR_WINDOW_COUNT: usize = 64;
pub const SCIR_LOCAL_MMR_BASE: usize = LOCAL_BUS_BASE + LOCAL_BUS_SIZE - SCIR_WINDOW_COUNT;
pub const SCIR_CPU_HEARTBEAT: usize = 0x01;
pub const SCIR_CPU_ACTIVITY: usize = 0x02;
pub const SCIR_CPU_HB_INTERVAL: usize = HZ;

#[inline] pub unsafe fn uv_global_mmr32_address(pnode:i32, offset:usize)->*mut usize { __va(UV_GLOBAL_MMR32_BASE | ((pnode as usize)<<UV_GLOBAL_MMR32_PNODE_SHIFT) | offset) as *mut usize }
extern "C" { pub fn writeq(v:usize,p:*mut usize); pub fn readq(p:*mut usize)->usize; pub fn writeb(v:u8,p:*mut usize); pub fn readb(p:*mut usize)->u8; }
#[inline] pub unsafe fn uv_write_global_mmr32(pnode:i32,offset:usize,val:usize){writeq(val,uv_global_mmr32_address(pnode,offset));}
#[inline] pub unsafe fn uv_read_global_mmr32(pnode:i32,offset:usize)->usize{readq(uv_global_mmr32_address(pnode,offset))}
#[inline] pub unsafe fn uv_global_mmr64_address(pnode:i32,offset:usize)->*mut usize { __va(((*_uv_hub_info()).global_mmr_base | ((pnode as usize)<<(*_uv_hub_info()).global_mmr_shift) | offset)) as *mut usize }
#[inline] pub unsafe fn uv_write_global_mmr64(pnode:i32,offset:usize,val:usize){writeq(val,uv_global_mmr64_address(pnode,offset));}
#[inline] pub unsafe fn uv_read_global_mmr64(pnode:i32,offset:usize)->usize{readq(uv_global_mmr64_address(pnode,offset))}
#[inline] pub unsafe fn uv_write_global_mmr8(pnode:i32,offset:usize,val:u8){writeb(val,uv_global_mmr64_address(pnode,offset));}
#[inline] pub unsafe fn uv_read_global_mmr8(pnode:i32,offset:usize)->u8{readb(uv_global_mmr64_address(pnode,offset))}
#[inline] pub unsafe fn uv_local_mmr_address(offset:usize)->*mut usize {__va(UV2_LOCAL_MMR_BASE|offset) as *mut usize}
#[inline] pub unsafe fn uv_read_local_mmr(offset:usize)->usize{readq(uv_local_mmr_address(offset))}
#[inline] pub unsafe fn uv_write_local_mmr(offset:usize,val:usize){writeq(val,uv_local_mmr_address(offset));}
#[inline] pub unsafe fn uv_read_local_mmr8(offset:usize)->u8{readb(uv_local_mmr_address(offset))}
#[inline] pub unsafe fn uv_write_local_mmr8(offset:usize,val:u8){writeb(val,uv_local_mmr_address(offset));}
#[inline] pub unsafe fn uv_blade_processor_id()->i32{(*uv_cpu_info()).blade_cpu_id as i32}
#[inline] pub unsafe fn uv_cpu_blade_processor_id(cpu:i32)->i32{(*uv_cpu_info_per(cpu)).blade_cpu_id as i32}
#[inline] pub unsafe fn uv_pnode_offset_to_vaddr(pnode:i32,offset:usize)->*mut core::ffi::c_void {let h=_uv_hub_info();if (*h).m_val!=0{return __va(((pnode as usize)<<(*h).m_val)|offset)}let sock=uv_pnode_to_socket(pnode);if sock==0{return __va(offset)}__va(((*h).gr_table.add((sock-1) as usize).read().limit as usize)<<UV_GAM_RANGE_SHFT|offset)}

#[inline] pub unsafe fn uv_gpa_shift() -> u32 { (*_uv_hub_info()).gpa_shift as u32 }
#[inline] pub unsafe fn uv_gam_range(pa: usize) -> *mut uv_gam_range_s {
    let mut gr = (*_uv_hub_info()).gr_table;
    let pal = (pa & (*_uv_hub_info()).gpa_mask) >> UV_GAM_RANGE_SHFT;
    for _ in 0..(*_uv_hub_info()).gr_table_len { if !gr.is_null() && pal < (*gr).limit as usize { return gr; } gr = gr.add(1); }
    BUG()
}
#[inline] pub unsafe fn uv_gam_range_base(pa: usize) -> usize { let gr=uv_gam_range(pa); if (*gr).base < 0 {0} else {(*_uv_hub_info()).gr_table.add((*gr).base as usize).read().limit as usize} }
#[inline] pub unsafe fn uv_soc_phys_ram_to_nasid(paddr: usize) -> usize { (*uv_gam_range(paddr)).nasid as usize }
#[inline] pub unsafe fn uv_gpa_nasid(v: *mut core::ffi::c_void) -> usize { uv_soc_phys_ram_to_nasid(__pa(v)) }
#[inline] pub unsafe fn uv_soc_phys_ram_to_gpa(mut paddr: usize) -> usize { let h=_uv_hub_info(); if paddr < (*h).lowmem_remap_top {paddr|=(*h).lowmem_remap_base;} if (*h).m_val != 0 {paddr|=(*h).gnode_upper; paddr=((paddr<<(*h).m_shift)>>(*h).m_shift)|((paddr>>(*h).m_val)<<(*h).n_lshift);} else {paddr|=uv_soc_phys_ram_to_nasid(paddr)<<(*h).gpa_shift;} paddr }
#[inline] pub unsafe fn uv_gpa(v: *mut core::ffi::c_void) -> usize { uv_soc_phys_ram_to_gpa(__pa(v)) }
#[inline] pub unsafe fn uv_gpa_in_mmr_space(gpa: usize) -> i32 { ((gpa>>62)==3) as i32 }
#[inline] pub unsafe fn uv_gpa_to_soc_phys_ram(mut gpa: usize) -> usize { let h=_uv_hub_info(); if (*h).m_val!=0 {gpa=((gpa<<(*h).m_shift)>>(*h).m_shift)|((gpa>>(*h).n_lshift)<<(*h).m_val);} let mut p=gpa&(*h).gpa_mask; if p>=(*h).lowmem_remap_base && p<(*h).lowmem_remap_base+(*h).lowmem_remap_top {p-=(*h).lowmem_remap_base;} p }
#[inline] pub unsafe fn uv_gpa_to_gnode(gpa: usize) -> usize { let h=_uv_hub_info(); if (*h).n_lshift!=0 {gpa>>(*h).n_lshift} else {uv_gam_range(gpa).as_ref().unwrap().nasid as usize >> 1} }
#[inline] pub unsafe fn uv_gpa_to_pnode(gpa: usize) -> i32 { (uv_gpa_to_gnode(gpa)&(*_uv_hub_info()).pnode_mask as usize) as i32 }
#[inline] pub unsafe fn uv_gpa_to_offset(gpa: usize) -> usize { let h=_uv_hub_info(); if (*h).m_shift!=0 {(gpa<<(*h).m_shift)>>(*h).m_shift} else {(gpa&(*h).gpa_mask)-uv_gam_range_base(gpa)} }

#[inline] pub unsafe fn _uv_socket_to_node(socket:i32,s2nid:*mut u16)->i32 {if !s2nid.is_null(){*s2nid.add((socket-(*_uv_hub_info()).min_socket as i32) as usize) as i32}else{socket}}
#[inline] pub unsafe fn uv_socket_to_node(socket:i32)->i32 {_uv_socket_to_node(socket,(*_uv_hub_info()).socket_to_node)}
#[inline] pub unsafe fn uv_pnode_to_socket(pnode:i32)->i32 {let p=(*_uv_hub_info()).pnode_to_socket;if !p.is_null(){*p.add((pnode-(*_uv_hub_info()).min_pnode as i32) as usize) as i32}else{pnode}}
#[inline] pub unsafe fn uv_apicid_to_pnode(apicid:i32)->i32 {let p=apicid>>(*_uv_hub_info()).apic_pnode_shift;let s=(*_uv_hub_info()).socket_to_pnode;if !s.is_null(){*s.add((p-(*_uv_hub_info()).min_socket as i32) as usize) as i32}else{p}}
#[inline] pub unsafe fn uv_blade_to_node(blade:i32)->i32 {uv_socket_to_node(blade)}
#[inline] pub unsafe fn uv_numa_blade_id()->i32 {(*_uv_hub_info()).numa_blade_id as i32}
#[inline] pub unsafe fn uv_node_to_blade_id(nid:i32)->i32 {let p=(*_uv_hub_info()).node_to_socket;if !p.is_null(){*p.add(nid as usize) as i32}else{nid}}
#[inline] pub unsafe fn uv_cpu_to_blade_id(cpu:i32)->i32 {(*uv_cpu_hub_info(cpu)).numa_blade_id as i32}
#[inline] pub unsafe fn uv_blade_to_pnode(bid:i32)->i32 {let p=(*_uv_hub_info()).socket_to_pnode;if !p.is_null(){*p.add(bid as usize) as i32}else{bid}}
#[inline] pub unsafe fn uv_blade_to_memory_nid(bid:i32)->i32 {(*uv_hub_info_list(uv_blade_to_node(bid))).memory_nid as i32}
#[inline] pub unsafe fn uv_blade_nr_possible_cpus(bid:i32)->i32 {(*uv_hub_info_list(uv_blade_to_node(bid))).nr_possible_cpus as i32}
#[inline] pub unsafe fn uv_blade_nr_online_cpus(bid:i32)->i32 {(*uv_hub_info_list(uv_blade_to_node(bid))).nr_online_cpus as i32}
#[inline] pub unsafe fn uv_cpu_to_pnode(cpu:i32)->i32 {(*uv_cpu_hub_info(cpu)).pnode as i32}
#[inline] pub unsafe fn uv_node_to_pnode(nid:i32)->i32 {(*uv_hub_info_list(nid)).pnode as i32}
#[inline] pub unsafe fn uv_num_possible_blades()->i32 {uv_possible_blades as i32}

extern "C" { pub const UVH_SCRATCH5: usize; pub const UVH_SCRATCH5_ALIAS: usize; pub const UVH_SCRATCH5_ALIAS_2: usize; }
pub const UVH_BIOS_KERNEL_MMR: usize = UVH_SCRATCH5;
pub const UVH_BIOS_KERNEL_MMR_ALIAS: usize = UVH_SCRATCH5_ALIAS;
pub const UVH_BIOS_KERNEL_MMR_ALIAS_2: usize = UVH_SCRATCH5_ALIAS_2;
pub const UV_TSC_SYNC_MMR: usize = UVH_BIOS_KERNEL_MMR;
pub const UV_TSC_SYNC_SHIFT: usize = 10;
pub const UV_TSC_SYNC_SHIFT_UV2K: usize = 16;
pub const UV_TSC_SYNC_MASK: usize = 3;
pub const UV_TSC_SYNC_VALID: usize = 3;
pub const UV_TSC_SYNC_UNKNOWN: usize = 0;
pub const UVH_NMI_MMR: usize = UVH_BIOS_KERNEL_MMR;
pub const UVH_NMI_MMR_CLEAR: usize = UVH_BIOS_KERNEL_MMR_ALIAS;
pub const UVH_NMI_MMR_SHIFT: usize = 63;
pub const UVH_NMI_MMR_TYPE: &str = "SCRATCH5";

#[repr(C)] pub struct uv_hub_nmi_s { pub nmi_lock: raw_spinlock_t, pub in_nmi: atomic_t, pub cpu_owner: atomic_t, pub read_mmr_count: atomic_t, pub nmi_count: atomic_t, pub nmi_value: usize, pub hub_present: bool, pub pch_owner: bool }
#[repr(C)] pub struct uv_cpu_nmi_s { pub hub: *mut uv_hub_nmi_s, pub state:i32, pub pinging:i32, pub queries:i32, pub pings:i32 }
extern "C" { pub static mut uv_cpu_nmi: uv_cpu_nmi_s; }
#[inline] pub unsafe fn uv_hub_nmi()->*mut uv_hub_nmi_s { uv_cpu_nmi.hub }
#[inline] pub unsafe fn uv_cpu_nmi_per(_cpu:i32)->*mut uv_cpu_nmi_s { &mut uv_cpu_nmi }
#[inline] pub unsafe fn uv_hub_nmi_per(cpu:i32)->*mut uv_hub_nmi_s { (*uv_cpu_nmi_per(cpu)).hub }
pub const UV_NMI_STATE_OUT:i32=0; pub const UV_NMI_STATE_IN:i32=1; pub const UV_NMI_STATE_DUMP:i32=2; pub const UV_NMI_STATE_DUMP_DONE:i32=3;
#[inline] pub unsafe fn uv_get_min_hub_revision_id()->i32 {(*_uv_hub_info()).hub_revision as i32}

// External kernel types referenced by the source header.
extern "C" { pub type raw_spinlock_t; pub type atomic_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
