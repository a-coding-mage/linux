/* SPDX-License-Identifier: GPL-2.0 */
/* Linux Socket Filter Data Structures.  Dependencies are supplied by other kernel bindings. */

/* Register mappings. */
pub const BPF_REG_ARG1: u32 = BPF_REG_1; pub const BPF_REG_ARG2: u32 = BPF_REG_2;
pub const BPF_REG_ARG3: u32 = BPF_REG_3; pub const BPF_REG_ARG4: u32 = BPF_REG_4;
pub const BPF_REG_ARG5: u32 = BPF_REG_5; pub const BPF_REG_CTX: u32 = BPF_REG_6;
pub const BPF_REG_FP: u32 = BPF_REG_10; pub const BPF_REG_A: u32 = BPF_REG_0;
pub const BPF_REG_X: u32 = BPF_REG_7; pub const BPF_REG_TMP: u32 = BPF_REG_2;
pub const BPF_REG_D: u32 = BPF_REG_8; pub const BPF_REG_H: u32 = BPF_REG_9;
pub const BPF_REG_PARAMS: u32 = MAX_BPF_REG; pub const BPF_REG_AX: u32 = MAX_BPF_REG + 1;
pub const MAX_BPF_EXT_REG: u32 = MAX_BPF_REG + 2; pub const MAX_BPF_JIT_REG: u32 = MAX_BPF_EXT_REG;
pub const BPF_TAIL_CALL: u32 = 0xf0; pub const BPF_PROBE_MEM: u32 = 0x20;
pub const BPF_PROBE_MEMSX: u32 = 0x40; pub const BPF_PROBE_MEM32: u32 = 0xa0;
pub const BPF_PROBE_ATOMIC: u32 = 0xe0; pub const BPF_PROBE_MEM32SX: u32 = 0xc0;
pub const BPF_CALL_ARGS: u32 = 0xe0; pub const BPF_NOSPEC: u32 = 0xc0;
pub const BPF_SYM_ELF_TYPE: u8 = b't'; pub const MAX_BPF_STACK: usize = 512;

/* The following macros retain the source-level instruction initializers. */
macro_rules! bpf_insn { ($code:expr,$dst:expr,$src:expr,$off:expr,$imm:expr) => { bpf_insn { code:$code, dst_reg:$dst, src_reg:$src, off:$off, imm:$imm } }; }
macro_rules! BPF_ALU64_REG_OFF { ($op:expr,$dst:expr,$src:expr,$off:expr) => { bpf_insn!(BPF_ALU64|BPF_OP!($op)|BPF_X,$dst,$src,$off,0) }; }
macro_rules! BPF_ALU64_REG { ($op:expr,$dst:expr,$src:expr) => { BPF_ALU64_REG_OFF!($op,$dst,$src,0) }; }
macro_rules! BPF_ALU32_REG_OFF { ($op:expr,$dst:expr,$src:expr,$off:expr) => { bpf_insn!(BPF_ALU|BPF_OP!($op),$dst,$src,$off,0) }; }
macro_rules! BPF_ALU32_REG { ($op:expr,$dst:expr,$src:expr) => { BPF_ALU32_REG_OFF!($op,$dst,$src,0) }; }
macro_rules! BPF_ALU64_IMM_OFF { ($op:expr,$dst:expr,$imm:expr,$off:expr) => { bpf_insn!(BPF_ALU64|BPF_OP!($op)|BPF_K,$dst,0,$off,$imm) }; }
macro_rules! BPF_ALU64_IMM { ($op:expr,$dst:expr,$imm:expr) => { BPF_ALU64_IMM_OFF!($op,$dst,$imm,0) }; }
macro_rules! BPF_ALU32_IMM_OFF { ($op:expr,$dst:expr,$imm:expr,$off:expr) => { bpf_insn!(BPF_ALU|BPF_OP!($op)|BPF_K,$dst,0,$off,$imm) }; }
macro_rules! BPF_ALU32_IMM { ($op:expr,$dst:expr,$imm:expr) => { BPF_ALU32_IMM_OFF!($op,$dst,$imm,0) }; }
macro_rules! BPF_ENDIAN { ($t:expr,$d:expr,$l:expr) => { bpf_insn!(BPF_ALU|BPF_END|BPF_SRC!($t),$d,0,0,$l) }; }
macro_rules! BPF_BSWAP { ($d:expr,$l:expr) => { bpf_insn!(BPF_ALU64|BPF_END|BPF_SRC!(BPF_TO_LE),$d,0,0,$l) }; }
macro_rules! BPF_MOV64_REG { ($d:expr,$s:expr) => { bpf_insn!(BPF_ALU64|BPF_MOV|BPF_X,$d,$s,0,0) }; }
macro_rules! BPF_MOV32_REG { ($d:expr,$s:expr) => { bpf_insn!(BPF_ALU|BPF_MOV|BPF_X,$d,$s,0,0) }; }
pub const BPF_ADDR_PERCPU: i16 = -1;
macro_rules! BPF_MOV64_PERCPU_REG { ($d:expr,$s:expr) => { bpf_insn!(BPF_ALU64|BPF_MOV|BPF_X,$d,$s,BPF_ADDR_PERCPU,0) }; }
pub unsafe fn insn_is_mov_percpu_addr(i:*const bpf_insn)->bool { (*i).code==(BPF_ALU64|BPF_MOV|BPF_X) && (*i).off==BPF_ADDR_PERCPU }
macro_rules! BPF_MOV64_IMM { ($d:expr,$i:expr) => { bpf_insn!(BPF_ALU64|BPF_MOV|BPF_K,$d,0,0,$i) }; }
macro_rules! BPF_MOV32_IMM { ($d:expr,$i:expr) => { bpf_insn!(BPF_ALU|BPF_MOV|BPF_K,$d,0,0,$i) }; }
macro_rules! BPF_MOVSX64_REG { ($d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_ALU64|BPF_MOV|BPF_X,$d,$s,$o,0) }; }
macro_rules! BPF_MOVSX32_REG { ($d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_ALU|BPF_MOV|BPF_X,$d,$s,$o,0) }; }
macro_rules! BPF_ZEXT_REG { ($d:expr) => { bpf_insn!(BPF_ALU|BPF_MOV|BPF_X,$d,$d,0,1) }; }
pub unsafe fn insn_is_zext(i:*const bpf_insn)->bool { (*i).code==(BPF_ALU|BPF_MOV|BPF_X) && (*i).imm==1 }
pub unsafe fn insn_is_cast_user(i:*const bpf_insn)->bool { (*i).code==(BPF_ALU64|BPF_MOV|BPF_X) && (*i).off==BPF_ADDR_SPACE_CAST && (*i).imm==(1u32<<16) }
macro_rules! BPF_LD_IMM64_RAW { ($d:expr,$s:expr,$i:expr) => { [bpf_insn!(BPF_LD|BPF_DW|BPF_IMM,$d,$s,0,($i as u32)),bpf_insn!(0,0,0,0,(($i as u64)>>32))] }; }
macro_rules! BPF_LD_IMM64 { ($d:expr,$i:expr) => { BPF_LD_IMM64_RAW!($d,0,$i) }; }
macro_rules! BPF_LD_MAP_FD { ($d:expr,$fd:expr) => { BPF_LD_IMM64_RAW!($d,BPF_PSEUDO_MAP_FD,$fd) }; }
macro_rules! BPF_MOV64_RAW { ($t:expr,$d:expr,$s:expr,$i:expr) => { bpf_insn!(BPF_ALU64|BPF_MOV|BPF_SRC!($t),$d,$s,0,$i) }; }
macro_rules! BPF_MOV32_RAW { ($t:expr,$d:expr,$s:expr,$i:expr) => { bpf_insn!(BPF_ALU|BPF_MOV|BPF_SRC!($t),$d,$s,0,$i) }; }
macro_rules! BPF_LD_ABS { ($z:expr,$i:expr) => { bpf_insn!(BPF_LD|BPF_SIZE!($z)|BPF_ABS,0,0,0,$i) }; }
macro_rules! BPF_LD_IND { ($z:expr,$s:expr,$i:expr) => { bpf_insn!(BPF_LD|BPF_SIZE!($z)|BPF_IND,0,$s,0,$i) }; }
macro_rules! BPF_LDX_MEM { ($z:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_LDX|BPF_SIZE!($z)|BPF_MEM,$d,$s,$o,0) }; }
macro_rules! BPF_LDX_MEMSX { ($z:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_LDX|BPF_SIZE!($z)|BPF_MEMSX,$d,$s,$o,0) }; }
macro_rules! BPF_STX_MEM { ($z:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_STX|BPF_SIZE!($z)|BPF_MEM,$d,$s,$o,0) }; }
macro_rules! BPF_ATOMIC_OP { ($z:expr,$op:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_STX|BPF_SIZE!($z)|BPF_ATOMIC,$d,$s,$o,$op) }; }
macro_rules! BPF_STX_XADD { ($z:expr,$d:expr,$s:expr,$o:expr) => { BPF_ATOMIC_OP!($z,BPF_ADD,$d,$s,$o) }; }
macro_rules! BPF_ST_MEM { ($z:expr,$d:expr,$o:expr,$i:expr) => { bpf_insn!(BPF_ST|BPF_SIZE!($z)|BPF_MEM,$d,0,$o,$i) }; }
macro_rules! BPF_JMP_REG { ($op:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_JMP|BPF_OP!($op)|BPF_X,$d,$s,$o,0) }; }
macro_rules! BPF_JMP_IMM { ($op:expr,$d:expr,$i:expr,$o:expr) => { bpf_insn!(BPF_JMP|BPF_OP!($op)|BPF_K,$d,0,$o,$i) }; }
macro_rules! BPF_JMP32_REG { ($op:expr,$d:expr,$s:expr,$o:expr) => { bpf_insn!(BPF_JMP32|BPF_OP!($op)|BPF_X,$d,$s,$o,0) }; }
macro_rules! BPF_JMP32_IMM { ($op:expr,$d:expr,$i:expr,$o:expr) => { bpf_insn!(BPF_JMP32|BPF_OP!($op)|BPF_K,$d,0,$o,$i) }; }
macro_rules! BPF_JMP_A { ($o:expr) => { bpf_insn!(BPF_JMP|BPF_JA,0,0,$o,0) }; }
macro_rules! BPF_JMP32_A { ($i:expr) => { bpf_insn!(BPF_JMP32|BPF_JA,0,0,0,$i) }; }
macro_rules! BPF_CALL_REL { ($t:expr) => { bpf_insn!(BPF_JMP|BPF_CALL,0,BPF_PSEUDO_CALL,0,$t) }; }
macro_rules! BPF_CALL_KFUNC { ($o:expr,$i:expr) => { bpf_insn!(BPF_JMP|BPF_CALL,0,BPF_PSEUDO_KFUNC_CALL,$o,$i) }; }
macro_rules! BPF_RAW_INSN { ($c:expr,$d:expr,$s:expr,$o:expr,$i:expr) => { bpf_insn!($c,$d,$s,$o,$i) }; }
macro_rules! BPF_EXIT_INSN { () => { bpf_insn!(BPF_JMP|BPF_EXIT,0,0,0,0) }; }
macro_rules! BPF_ST_NOSPEC { () => { bpf_insn!(BPF_ST|BPF_NOSPEC,0,0,0,0) }; }

#[repr(C)] pub struct compat_sock_fprog { pub len:u16, pub filter:compat_uptr_t }
#[repr(C)] pub struct sock_fprog_kern { pub len:u16, pub filter:*mut sock_filter }
pub const BPF_IMAGE_ALIGNMENT: usize = 8;
#[repr(C)] pub struct bpf_binary_header { pub size:u32, pub image:[u8;0] }
#[repr(C,align(16))] pub struct bpf_prog_stats { pub cnt:u64_stats_t, pub nsecs:u64_stats_t, pub misses:u64_stats_t, pub syncp:u64_stats_sync }
#[repr(C)] pub struct bpf_timed_may_goto { pub count:u64, pub timestamp:u64 }
#[repr(C)] pub struct sk_filter { pub refcnt:refcount_t, pub rcu:rcu_head, pub prog:*mut bpf_prog }

pub const BPF_SKB_CB_LEN: usize = QDISC_CB_PRIV_LEN;
#[repr(C)] pub struct bpf_skb_data_end { pub qdisc_cb:qdisc_skb_cb, pub data_meta:*mut core::ffi::c_void, pub data_end:*mut core::ffi::c_void }
#[repr(C)] pub union bpf_nh_union { pub ipv4_nh:u32, pub ipv6_nh:in6_addr }
#[repr(C)] pub struct bpf_nh_params { pub nh_family:u32, pub nh:bpf_nh_union }
pub const BPF_RI_F_RF_NO_DIRECT:u32=BIT(0); pub const BPF_RI_F_RI_INIT:u32=BIT(1); pub const BPF_RI_F_CPU_MAP_INIT:u32=BIT(2); pub const BPF_RI_F_DEV_MAP_INIT:u32=BIT(3); pub const BPF_RI_F_XSK_MAP_INIT:u32=BIT(4);
#[repr(C)] pub struct bpf_redirect_info { pub tgt_index:u64, pub tgt_value:*mut core::ffi::c_void, pub map:*mut bpf_map, pub flags:u32, pub map_id:u32, pub map_type:bpf_map_type, pub nh:bpf_nh_params, pub kern_flags:u32 }
#[repr(C)] pub struct bpf_net_context { pub ri:bpf_redirect_info, pub cpu_map_flush_list:list_head, pub dev_map_flush_list:list_head, pub xskmap_map_flush_list:list_head }

extern "C" {
    pub fn sk_filter_trim_cap(sk:*mut sock, skb:*mut sk_buff, cap:u32)->skb_drop_reason;
    pub fn bpf_prog_run(prog:*const bpf_prog, ctx:*const core::ffi::c_void)->u32;
    pub fn bpf_prog_unlock_free(fp:*mut bpf_prog);
    pub fn bpf_prog_kallsyms_del_all(fp:*mut bpf_prog);
    pub fn bpf_internal_load_pointer_neg_helper(skb:*const sk_buff,k:i32,size:u32)->*mut core::ffi::c_void;
}
pub unsafe fn sk_filter(sk:*mut sock,skb:*mut sk_buff)->i32 { if sk_filter_trim_cap(sk,skb,1) as i32 != 0 {-EPERM} else {0} }
pub unsafe fn sk_filter_reason(sk:*mut sock,skb:*mut sk_buff)->skb_drop_reason { sk_filter_trim_cap(sk,skb,1) }
pub unsafe fn bpf_prog_insn_size(p:*const bpf_prog)->u32 { (*p).len * core::mem::size_of::<bpf_insn>() as u32 }
pub unsafe fn bpf_prog_was_classic(p:*const bpf_prog)->bool { (*p).type_ == BPF_PROG_TYPE_UNSPEC }
pub unsafe fn bpf_tell_extensions()->i32 { SKF_AD_MAX }
pub const BPF_ANC:u32=BIT(15);

#[repr(C)] pub struct bpf_sock_addr_kern { pub sk:*mut sock, pub uaddr:*mut sockaddr_unsized, pub tmp_reg:u64, pub t_ctx:*mut core::ffi::c_void, pub uaddrlen:u32 }
#[repr(C)] pub union bpf_sock_ops_args { pub args:[u32;4], pub reply:u32, pub replylong:[u32;4] }
#[repr(C)] pub struct bpf_sock_ops_kern { pub sk:*mut sock, pub args:bpf_sock_ops_args, pub syn_skb:*mut sk_buff, pub skb:*mut sk_buff, pub skb_data_end:*mut core::ffi::c_void, pub op:u8, pub is_fullsock:u8, pub is_locked_tcp_sock:u8, pub remaining_opt_len:u8, pub temp:u64 }
#[repr(C)] pub struct bpf_sysctl_kern { pub head:*mut ctl_table_header, pub table:*const ctl_table, pub cur_val:*mut core::ffi::c_void, pub cur_len:usize, pub new_val:*mut core::ffi::c_void, pub new_len:usize, pub new_updated:i32, pub write:i32, pub ppos:*mut loff_t, pub tmp_reg:u64 }
pub const BPF_SOCKOPT_KERN_BUF_SIZE:usize=32;
#[repr(C)] pub struct bpf_sockopt_buf { pub data:[u8;BPF_SOCKOPT_KERN_BUF_SIZE] }
#[repr(C)] pub struct bpf_sockopt_kern { pub sk:*mut sock, pub optval:*mut u8, pub optval_end:*mut u8, pub level:i32, pub optname:i32, pub optlen:i32, pub current_task:*mut task_struct, pub tmp_reg:u64 }
#[repr(C)] pub struct bpf_sk_lookup_v4 { pub saddr:__be32, pub daddr:__be32 }
#[repr(C)] pub struct bpf_sk_lookup_v6 { pub saddr:*const in6_addr, pub daddr:*const in6_addr }
#[repr(C)] pub struct bpf_sk_lookup_kern { pub family:u16, pub protocol:u16, pub sport:__be16, pub dport:u16, pub v4:bpf_sk_lookup_v4, pub v6:bpf_sk_lookup_v6, pub selected_sk:*mut sock, pub ingress_ifindex:u32, pub no_reuseport:bool }
pub const BPF_RI_F_MASK:u32=BPF_RI_F_RF_NO_DIRECT|BPF_RI_F_RI_INIT|BPF_RI_F_CPU_MAP_INIT|BPF_RI_F_DEV_MAP_INIT|BPF_RI_F_XSK_MAP_INIT;
#[repr(C)] pub struct bpf_dispatcher_fn { _private:[u8;0] }
pub const BPF_CLASSIC_PROG_LEN_SCALE:usize=core::mem::size_of::<sock_filter>();

extern "C" {
 pub fn bpf_prog_select_runtime(fp:*mut bpf_prog,err:*mut i32)->*mut bpf_prog;
 pub fn bpf_prog_free(fp:*mut bpf_prog); pub fn bpf_opcode_in_insntable(code:u8)->bool;
 pub fn bpf_prog_alloc(size:u32,flags:gfp_t)->*mut bpf_prog; pub fn bpf_prog_alloc_no_stats(size:u32,flags:gfp_t)->*mut bpf_prog;
 pub fn bpf_prog_realloc(old:*mut bpf_prog,size:u32,flags:gfp_t)->*mut bpf_prog; pub fn __bpf_prog_free(fp:*mut bpf_prog);
 pub fn bpf_prog_create(pfp:*mut *mut bpf_prog,fprog:*mut sock_fprog_kern)->i32; pub fn bpf_prog_destroy(fp:*mut bpf_prog);
 pub fn sk_attach_filter(fprog:*mut sock_fprog,sk:*mut sock)->i32; pub fn sk_attach_bpf(ufd:u32,sk:*mut sock)->i32;
 pub fn sk_detach_filter(sk:*mut sock)->i32; pub fn sk_get_filter(sk:*mut sock,optval:sockptr_t,len:u32)->i32;
 pub fn bpf_jit_compile(prog:*mut bpf_prog); pub fn bpf_jit_needs_zext()->bool; pub fn bpf_jit_supports_insn(insn:*mut bpf_insn,in_arena:bool)->bool;
 pub fn bpf_helper_changes_pkt_data(func_id:bpf_func_id)->bool; pub fn bpf_patch_insn_single(prog:*mut bpf_prog,off:u32,patch:*const bpf_insn,len:u32)->*mut bpf_prog;
 pub fn xdp_do_flush(); pub fn xdp_do_redirect(dev:*mut net_device,xdp:*mut xdp_buff,prog:*const bpf_prog)->i32;
 pub fn __bpf_skb_load_bytes(skb:*const sk_buff,offset:u32,to:*mut core::ffi::c_void,len:u32)->i32;
 pub fn __bpf_skb_store_bytes(skb:*mut sk_buff,offset:u32,from:*const core::ffi::c_void,len:u32,flags:u64)->i32;
 pub fn __bpf_xdp_load_bytes(xdp:*mut xdp_buff,offset:u32,buf:*mut core::ffi::c_void,len:u32)->i32;
 pub fn __bpf_xdp_store_bytes(xdp:*mut xdp_buff,offset:u32,buf:*mut core::ffi::c_void,len:u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
