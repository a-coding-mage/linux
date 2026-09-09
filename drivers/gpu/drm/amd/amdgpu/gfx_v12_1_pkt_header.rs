/* Translated from gfx_v12_1_pkt.h. */

pub const PACKET_TYPE0: u32 = 0;
pub const PACKET_TYPE1: u32 = 1;
pub const PACKET_TYPE2: u32 = 2;
pub const PACKET_TYPE3: u32 = 3;
pub const CP_PACKET2: u32 = 0x80000000;
pub const PACKET2_PAD_SHIFT: u32 = 0;
pub const PACKET2_PAD_MASK: u32 = 0x3fffffff;

pub const fn cp_packet_get_type(h: u32) -> u32 { (h >> 30) & 3 }
pub const fn cp_packet_get_count(h: u32) -> u32 { (h >> 16) & 0x3fff }
pub const fn cp_packet0_get_reg(h: u32) -> u32 { h & 0xffff }
pub const fn cp_packet3_get_opcode(h: u32) -> u32 { (h >> 8) & 0xff }
pub const fn packet0(reg: u32, n: u32) -> u32 { (PACKET_TYPE0 << 30) | (reg & 0xffff) | ((n & 0x3fff) << 16) }
pub const fn packet2(v: u32) -> u32 { CP_PACKET2 | reg_set(PACKET2_PAD, v) }
pub const fn packet3(op: u32, n: u32) -> u32 { (PACKET_TYPE3 << 30) | ((op & 0xff) << 8) | ((n & 0x3fff) << 16) }
pub const fn packet3_compute(op: u32, n: u32) -> u32 { packet3(op, n) | (1 << 1) }

// REG_SET and PACKET2_PAD are supplied by dependent headers.
extern "Rust" { pub fn reg_set(reg: u32, value: u32) -> u32; }
extern "Rust" { pub static PACKET2_PAD: u32; }

macro_rules! bitfield { ($name:ident, $mask:expr, $shift:expr) => { #[inline] pub const fn $name(x: u32) -> u32 { (x & $mask) << $shift } }; }
macro_rules! passthrough { ($name:ident) => { #[inline] pub const fn $name(x: u32) -> u32 { x } }; }

pub const PACKET3_NOP: u32=0x10; pub const PACKET3_CLEAR_STATE:u32=0x12; pub const PACKET3_INDEX_BUFFER_SIZE:u32=0x13;
pub const PACKET3_DISPATCH_DIRECT:u32=0x15; pub const PACKET3_DISPATCH_INDIRECT:u32=0x16; pub const PACKET3_ATOMIC_MEM:u32=0x1e;
bitfield!(packet3_atomic_mem_atomic,0x7f,0); bitfield!(packet3_atomic_mem_command,0xf,8); bitfield!(packet3_atomic_mem_scope,3,23); bitfield!(packet3_atomic_mem_temporal,3,25);
passthrough!(packet3_atomic_mem_addr_lo); passthrough!(packet3_atomic_mem_addr_hi); passthrough!(packet3_atomic_mem_src_data_lo); passthrough!(packet3_atomic_mem_src_data_hi); passthrough!(packet3_atomic_mem_cmp_data_lo); passthrough!(packet3_atomic_mem_cmp_data_hi); bitfield!(packet3_atomic_mem_loop_interval,0x1fff,0);
pub const PACKET3_ATOMIC_MEM_COMMAND_SINGLE_PASS_ATOMIC:u32=0; pub const PACKET3_ATOMIC_MEM_COMMAND_LOOP_UNTIL_COMPARE_SATISFIED:u32=1; pub const PACKET3_ATOMIC_MEM_COMMAND_WAIT_FOR_WRITE_CONFIRMATION:u32=2; pub const PACKET3_ATOMIC_MEM_COMMAND_SEND_AND_CONTINUE:u32=3;
pub const PACKET3_ATOMIC_MEM_SCOPE_CU:u32=0; pub const PACKET3_ATOMIC_MEM_SCOPE_SE:u32=1; pub const PACKET3_ATOMIC_MEM_SCOPE_DEVICE:u32=2; pub const PACKET3_ATOMIC_MEM_SCOPE_SYSTEM:u32=3;
pub const PACKET3_ATOMIC_MEM_TEMPORAL_RT:u32=0; pub const PACKET3_ATOMIC_MEM_TEMPORAL_NT:u32=1; pub const PACKET3_ATOMIC_MEM_TEMPORAL_FW:u32=2; pub const PACKET3_ATOMIC_MEM_TEMPORAL_UC:u32=3;

pub const PACKET3_OCCLUSION_QUERY:u32=0x1f; pub const PACKET3_SET_PREDICATION:u32=0x20; pub const PACKET3_REG_RMW:u32=0x21; pub const PACKET3_COND_EXEC:u32=0x22; pub const PACKET3_PRED_EXEC:u32=0x23;
bitfield!(packet3_pred_exec_exec_count,0x3fff,0); bitfield!(packet3_pred_exec_virtualxccid_select,0xff,24);
pub const PACKET3_DRAW_INDIRECT:u32=0x24; pub const PACKET3_DRAW_INDEX_INDIRECT:u32=0x25; pub const PACKET3_INDEX_BASE:u32=0x26; pub const PACKET3_DRAW_INDEX_2:u32=0x27; pub const PACKET3_CONTEXT_CONTROL:u32=0x28; pub const PACKET3_DRAW_INDIRECT_MULTI:u32=0x2c; pub const PACKET3_DRAW_INDEX_AUTO:u32=0x2d; pub const PACKET3_NUM_INSTANCES:u32=0x2f; pub const PACKET3_DRAW_INDEX_MULTI_AUTO:u32=0x30; pub const PACKET3_DRAW_INDEX_OFFSET_2:u32=0x35; pub const PACKET3_WRITE_DATA:u32=0x37;
bitfield!(packet3_write_data_dst_sel,0xf,8); bitfield!(packet3_write_data_scope,3,12); bitfield!(packet3_write_data_mode,3,14); bitfield!(packet3_write_data_addr_incr,1,16); bitfield!(packet3_write_data_mid_die_id,3,18); bitfield!(packet3_write_data_wr_confirm,1,20); bitfield!(packet3_write_data_xcd_die_id,0xf,21); bitfield!(packet3_write_data_temporal,3,25); bitfield!(packet3_write_data_coop_disable,1,27); passthrough!(packet3_write_data_dst_mmreg_addr_lo); bitfield!(packet3_write_data_dst_mem_addr_lo,0x3fffffff,2); bitfield!(packet3_write_data_dst_mmreg_addr_hi,0x3fff,0); passthrough!(packet3_write_data_dst_mem_addr_hi); passthrough!(packet3_write_data_data);

pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI:u32=0x38; pub const PACKET3_WAIT_REG_MEM:u32=0x3c; pub const PACKET3_INDIRECT_BUFFER:u32=0x3f; pub const PACKET3_COND_INDIRECT_BUFFER:u32=0x3f; pub const PACKET3_COPY_DATA:u32=0x40; pub const PACKET3_PFP_SYNC_ME:u32=0x42; pub const PACKET3_COND_WRITE:u32=0x45; pub const PACKET3_EVENT_WRITE:u32=0x46; pub const PACKET3_EVENT_WRITE_EOP:u32=0x47; pub const PACKET3_EVENT_WRITE_EOS:u32=0x48; pub const PACKET3_RELEASE_MEM:u32=0x49;

// The remaining packet fields are direct bitfield encodings from the source header.
bitfield!(packet3_event_write_event_type,0x3f,0); bitfield!(packet3_event_write_event_index,0xf,8); bitfield!(packet3_event_write_offload_enable,1,31); bitfield!(packet3_event_write_address_lo,0x1fffffff,3); passthrough!(packet3_event_write_address_hi);
bitfield!(packet3_release_mem_event_type,0x3f,0); bitfield!(packet3_release_mem_wait_sync,1,7); bitfield!(packet3_release_mem_event_index,0xf,8); bitfield!(packet3_release_mem_gcr_cntl,0x1fff,12); bitfield!(packet3_release_mem_temporal,3,25); bitfield!(packet3_release_mem_pq_exe_status,1,28); bitfield!(packet3_release_mem_dst_sel,3,16); bitfield!(packet3_release_mem_mes_intr_pipe,3,20); bitfield!(packet3_release_mem_mes_action_id,3,22); bitfield!(packet3_release_mem_int_sel,7,24); bitfield!(packet3_release_mem_add_doorebll_offset,1,28); bitfield!(packet3_release_mem_data_sel,7,29); bitfield!(packet3_release_mem_address_lo_32b,0x3fffffff,2); bitfield!(packet3_release_mem_address_lo_64b,0x1fffffff,3); passthrough!(packet3_release_mem_address_hi); passthrough!(packet3_release_mem_data_lo); passthrough!(packet3_release_mem_cmp_data_lo); passthrough!(packet3_release_mem_data_hi); passthrough!(packet3_release_mem_cmp_data_hi); passthrough!(packet3_release_mem_int_ctxid);
pub const PACKET3_PREAMBLE_CNTL:u32=0x4a; pub const PACKET3_PREAMBLE_BEGIN_CLEAR_STATE:u32=2<<28; pub const PACKET3_PREAMBLE_END_CLEAR_STATE:u32=3<<28; pub const PACKET3_DMA_DATA:u32=0x50; pub const PACKET3_CONTEXT_REG_RMW:u32=0x51; pub const PACKET3_ACQUIRE_MEM:u32=0x58;
pub const PACKET3_GEN_PDEPTE:u32=0x5b; pub const PACKET3_PRIME_UTCL2:u32=0x5d; pub const PACKET3_LOAD_UCONFIG_REG:u32=0x5e; pub const PACKET3_LOAD_SH_REG:u32=0x5f; pub const PACKET3_LOAD_CONFIG_REG:u32=0x60; pub const PACKET3_LOAD_CONTEXT_REG:u32=0x61; pub const PACKET3_LOAD_COMPUTE_STATE:u32=0x62; pub const PACKET3_LOAD_SH_REG_INDEX:u32=0x63; pub const PACKET3_SET_CONFIG_REG:u32=0x68; pub const PACKET3_SET_CONTEXT_REG:u32=0x69; pub const PACKET3_SET_SH_REG:u32=0x76; pub const PACKET3_SET_SH_REG_OFFSET:u32=0x77; pub const PACKET3_SET_QUEUE_REG:u32=0x78; pub const PACKET3_SET_UCONFIG_REG:u32=0x79; pub const PACKET3_SET_UCONFIG_REG_INDEX:u32=0x7a;
pub const PACKET3_DISPATCH_DRAW_PREAMBLE:u32=0x8c; pub const PACKET3_DISPATCH_DRAW:u32=0x8d; pub const PACKET3_INDEX_ATTRIBUTES_INDIRECT:u32=0x91; pub const PACKET3_WAIT_REG_MEM64:u32=0x93; pub const PACKET3_HDP_FLUSH:u32=0x95; pub const PACKET3_INVALIDATE_TLBS:u32=0x98; pub const PACKET3_DMA_DATA_FILL_MULTI:u32=0x9a; pub const PACKET3_SET_SH_REG_INDEX:u32=0x9b; pub const PACKET3_LOAD_CONTEXT_REG_INDEX:u32=0x9f; pub const PACKET3_SET_RESOURCES:u32=0xa0; pub const PACKET3_MAP_QUEUES:u32=0xa2; pub const PACKET3_UNMAP_QUEUES:u32=0xa3; pub const PACKET3_QUERY_STATUS:u32=0xa4;
pub const PACKET3_SET_CONFIG_REG_START:u32=0x2000; pub const PACKET3_SET_CONFIG_REG_END:u32=0x2c00; pub const PACKET3_SET_CONTEXT_REG_START:u32=0xa000; pub const PACKET3_SET_CONTEXT_REG_END:u32=0xa400; pub const PACKET3_SET_SH_REG_START:u32=0x2c00; pub const PACKET3_SET_SH_REG_END:u32=0x3000; pub const PACKET3_SET_UCONFIG_REG_START:u32=0xc000; pub const PACKET3_SET_UCONFIG_REG_END:u32=0xc400;
bitfield!(packet3_set_sh_reg_reg_offset,0xffff,0); bitfield!(packet3_set_sh_reg_vmid_shift,0x1f,23); bitfield!(packet3_set_sh_reg_index,0xf,28); passthrough!(packet3_set_sh_reg_reg_data); bitfield!(packet3_set_uconfig_reg_reg_offset,0xffff,0); passthrough!(packet3_set_uconfig_reg_reg_data);
bitfield!(packet3_invalidate_tlbs_dst_sel,0xffffffff,0); bitfield!(packet3_invalidate_tlbs_all_hub,0xffffffff,4); bitfield!(packet3_invalidate_tlbs_pasid,0xffffffff,5); bitfield!(packet3_invalidate_tlbs_flush_type,0xffffffff,29);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
