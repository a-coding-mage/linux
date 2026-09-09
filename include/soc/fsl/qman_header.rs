/* Faithful Rust translation of qman.h. Linux-provided types and helpers are
 * referenced as external dependencies, as in the original header. */

pub type u8_t = u8;
pub type u16_t = u16;
pub type u32_t = u32;
pub type u64_t = u64;
pub type s8_t = i8;
pub type dma_addr_t = u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

extern "C" {
    pub static mut qm_channel_pool1: u16;
    pub static mut qm_channel_caam: u16;
}

pub const QM_CHANNEL_SWPORTAL0: u32 = 0;
pub const QMAN_CHANNEL_POOL1: u32 = 0x21;
pub const QMAN_CHANNEL_CAAM: u32 = 0x80;
pub const QMAN_CHANNEL_POOL1_REV3: u32 = 0x401;
pub const QMAN_CHANNEL_CAAM_REV3: u32 = 0x840;
pub const QM_PIRQ_CSCI: u32 = 0x00100000;
pub const QM_PIRQ_EQCI: u32 = 0x00080000;
pub const QM_PIRQ_EQRI: u32 = 0x00040000;
pub const QM_PIRQ_DQRI: u32 = 0x00020000;
pub const QM_PIRQ_MRI: u32 = 0x00010000;
pub const QM_PIRQ_SLOW: u32 = QM_PIRQ_CSCI | QM_PIRQ_EQCI | QM_PIRQ_EQRI | QM_PIRQ_MRI;
pub const QM_SDQCR_CHANNELS_POOL_MASK: u32 = 0x00007fff;
#[inline] pub fn QM_SDQCR_CHANNELS_POOL(n: u16) -> u32 { 0x00008000u32 >> n }
#[inline] pub unsafe fn QM_SDQCR_CHANNELS_POOL_CONV(channel: u16) -> u32 { QM_SDQCR_CHANNELS_POOL(channel + 1 - qm_channel_pool1) }

#[repr(C)]
pub union qm_fd_data { pub fields: qm_fd_fields, pub data: __be64 }
#[repr(C, packed)] pub struct qm_fd_fields { pub cfg8b_w1:u8, pub bpid:u8, pub cfg8b_w3:u8, pub addr_hi:u8, pub addr_lo:__be32 }
#[repr(C)] pub union qm_fd_cmd { pub cmd:__be32, pub status:__be32 }
#[repr(C, align(8))] pub struct qm_fd { pub data:qm_fd_data, pub cfg:__be32, pub cmd:qm_fd_cmd }

pub const QM_FD_FORMAT_SG:u32=1<<31; pub const QM_FD_FORMAT_LONG:u32=1<<30; pub const QM_FD_FORMAT_COMPOUND:u32=1<<29;
pub const QM_FD_FORMAT_MASK:u32=0xe0000000; pub const QM_FD_OFF_SHIFT:u32=20; pub const QM_FD_OFF_MASK:u32=0x1ff00000; pub const QM_FD_LEN_MASK:u32=0xfffff; pub const QM_FD_LEN_BIG_MASK:u32=0x1fffffff;
#[repr(u32)] pub enum qm_fd_format { qm_fd_contig=0, qm_fd_contig_big=QM_FD_FORMAT_LONG, qm_fd_sg=QM_FD_FORMAT_SG, qm_fd_sg_big=QM_FD_FORMAT_SG|QM_FD_FORMAT_LONG, qm_fd_compound=QM_FD_FORMAT_COMPOUND }
#[inline] unsafe fn be16(x:u16)->u16{x.to_be()} #[inline] unsafe fn be32(x:u32)->u32{x.to_be()} #[inline] unsafe fn from_be32(x:u32)->u32{u32::from_be(x)} #[inline] unsafe fn from_be64(x:u64)->u64{u64::from_be(x)}
#[inline] pub unsafe fn qm_fd_addr(fd:*const qm_fd)->dma_addr_t { from_be64((*fd).data.data)&0xffffffffff }
#[inline] pub unsafe fn qm_fd_addr_get64(fd:*const qm_fd)->u64 { qm_fd_addr(fd) }
#[inline] pub unsafe fn qm_fd_addr_set64(fd:*mut qm_fd, addr:u64){(*fd).data.fields.addr_hi=(addr>>32) as u8;(*fd).data.fields.addr_lo=be32(addr as u32);}
#[inline] pub unsafe fn qm_fd_get_format(fd:*const qm_fd)->qm_fd_format { core::mem::transmute(from_be32((*fd).cfg)&QM_FD_FORMAT_MASK) }
#[inline] pub unsafe fn qm_fd_get_offset(fd:*const qm_fd)->i32 {(from_be32((*fd).cfg)&QM_FD_OFF_MASK>>QM_FD_OFF_SHIFT) as i32}
#[inline] pub unsafe fn qm_fd_get_length(fd:*const qm_fd)->i32 {(from_be32((*fd).cfg)&QM_FD_LEN_MASK) as i32}
#[inline] pub unsafe fn qm_fd_get_len_big(fd:*const qm_fd)->i32 {(from_be32((*fd).cfg)&QM_FD_LEN_BIG_MASK) as i32}
#[inline] pub unsafe fn qm_fd_set_param(fd:*mut qm_fd, fmt:qm_fd_format, off:i32, len:i32){(*fd).cfg=be32(fmt as u32 | (len as u32&QM_FD_LEN_BIG_MASK)|(((off as u32)<<QM_FD_OFF_SHIFT)&QM_FD_OFF_MASK));}
#[inline] pub unsafe fn qm_fd_set_contig(fd:*mut qm_fd,off:i32,len:i32){qm_fd_set_param(fd,qm_fd_format::qm_fd_contig,off,len)}
#[inline] pub unsafe fn qm_fd_set_sg(fd:*mut qm_fd,off:i32,len:i32){qm_fd_set_param(fd,qm_fd_format::qm_fd_sg,off,len)}
#[inline] pub unsafe fn qm_fd_set_contig_big(fd:*mut qm_fd,len:i32){qm_fd_set_param(fd,qm_fd_format::qm_fd_contig_big,0,len)}
#[inline] pub unsafe fn qm_fd_set_sg_big(fd:*mut qm_fd,len:i32){qm_fd_set_param(fd,qm_fd_format::qm_fd_sg_big,0,len)}
#[inline] pub unsafe fn qm_fd_set_compound(fd:*mut qm_fd,len:i32){qm_fd_set_param(fd,qm_fd_format::qm_fd_compound,0,len)}
#[inline] pub unsafe fn qm_fd_clear_fd(fd:*mut qm_fd){(*fd).data.data=0;(*fd).cfg=0;(*fd).cmd.cmd=0;}

#[repr(C, packed)] pub struct qm_sg_entry_fields { pub __reserved1:[u8;3],pub addr_hi:u8,pub addr_lo:__be32 }
#[repr(C)] pub union qm_sg_entry_data { pub fields:qm_sg_entry_fields,pub data:__be64 }
#[repr(C, packed)] pub struct qm_sg_entry { pub data:qm_sg_entry_data,pub cfg:__be32,pub __reserved2:u8,pub bpid:u8,pub offset:__be16 }
pub const QM_SG_LEN_MASK:u32=0x3fffffff; pub const QM_SG_OFF_MASK:u16=0x1fff; pub const QM_SG_FIN:u32=1<<30; pub const QM_SG_EXT:u32=1<<31;
#[inline] pub unsafe fn qm_sg_addr(sg:*const qm_sg_entry)->dma_addr_t{from_be64((*sg).data.data)&0xffffffffff}
#[inline] pub unsafe fn qm_sg_entry_get64(sg:*const qm_sg_entry)->u64{qm_sg_addr(sg)}
#[inline] pub unsafe fn qm_sg_entry_set64(sg:*mut qm_sg_entry,addr:u64){(*sg).data.fields.addr_hi=(addr>>32) as u8;(*sg).data.fields.addr_lo=be32(addr as u32)}
#[inline] pub unsafe fn qm_sg_entry_is_final(sg:*const qm_sg_entry)->bool{from_be32((*sg).cfg)&QM_SG_FIN!=0}
#[inline] pub unsafe fn qm_sg_entry_is_ext(sg:*const qm_sg_entry)->bool{from_be32((*sg).cfg)&QM_SG_EXT!=0}
#[inline] pub unsafe fn qm_sg_entry_get_len(sg:*const qm_sg_entry)->i32{(from_be32((*sg).cfg)&QM_SG_LEN_MASK) as i32}
#[inline] pub unsafe fn qm_sg_entry_set_len(sg:*mut qm_sg_entry,len:i32){(*sg).cfg=be32(len as u32&QM_SG_LEN_MASK)}
#[inline] pub unsafe fn qm_sg_entry_set_f(sg:*mut qm_sg_entry,len:i32){(*sg).cfg=be32(QM_SG_FIN|(len as u32&QM_SG_LEN_MASK))}
#[inline] pub unsafe fn qm_sg_entry_get_off(sg:*const qm_sg_entry)->i32{(be16((*sg).offset)&QM_SG_OFF_MASK) as i32}

#[repr(C, packed)] pub struct qm_dqrr_entry {pub verb:u8,pub stat:u8,pub seqnum:__be16,pub tok:u8,pub __reserved2:[u8;3],pub fqid:__be32,pub context_b:__be32,pub fd:qm_fd,pub __reserved4:[u8;32]}
pub const QM_DQRR_VERB_VBIT:u8=0x80; pub const QM_DQRR_VERB_MASK:u8=0x7f; pub const QM_DQRR_VERB_FRAME_DEQUEUE:u8=0x60; pub const QM_DQRR_STAT_FQ_EMPTY:u8=0x80; pub const QM_DQRR_STAT_FQ_HELDACTIVE:u8=0x40; pub const QM_DQRR_STAT_FQ_FORCEELIGIBLE:u8=0x20; pub const QM_DQRR_STAT_FD_VALID:u8=0x10; pub const QM_DQRR_STAT_UNSCHEDULED:u8=2; pub const QM_DQRR_STAT_DQCR_EXPIRED:u8=1;
pub const QM_FQID_MASK:u32=0xffffff;
#[inline] pub unsafe fn qm_fqid_set(p:*mut qm_dqrr_entry,v:u32){(*p).fqid=be32(v&QM_FQID_MASK)} #[inline] pub unsafe fn qm_fqid_get(p:*const qm_dqrr_entry)->u32{from_be32((*p).fqid)&QM_FQID_MASK}

#[repr(C)] pub union qm_mr_entry { pub verb:[u8;64], pub ern:qm_mr_ern, pub fq:qm_mr_fq }
#[repr(C, packed)] pub struct qm_mr_ern {pub verb:u8,pub dca:u8,pub seqnum:__be16,pub rc:u8,pub __reserved:[u8;3],pub fqid:__be32,pub tag:__be32,pub fd:qm_fd,pub __reserved1:[u8;32]}
#[repr(C, packed)] pub struct qm_mr_fq {pub verb:u8,pub fqs:u8,pub __reserved1:[u8;6],pub fqid:__be32,pub context_b:__be32,pub __reserved2:[u8;48]}
pub const QM_MR_VERB_VBIT:u8=0x80; pub const QM_MR_VERB_TYPE_MASK:u8=0x27; pub const QM_MR_VERB_DC_ERN:u8=0x20; pub const QM_MR_VERB_FQRN:u8=0x21; pub const QM_MR_VERB_FQRNI:u8=0x22; pub const QM_MR_VERB_FQRL:u8=0x23; pub const QM_MR_VERB_FQPN:u8=0x24; pub const QM_MR_RC_MASK:u8=0xf0; pub const QM_MR_RC_CGR_TAILDROP:u8=0; pub const QM_MR_RC_WRED:u8=0x10; pub const QM_MR_RC_ERROR:u8=0x20; pub const QM_MR_RC_ORPWINDOW_EARLY:u8=0x30; pub const QM_MR_RC_ORPWINDOW_LATE:u8=0x40; pub const QM_MR_RC_FQ_TAILDROP:u8=0x50; pub const QM_MR_RC_ORPWINDOW_RETIRED:u8=0x60; pub const QM_MR_RC_ORP_ZERO:u8=0x70; pub const QM_MR_FQS_ORLPRESENT:u8=2; pub const QM_MR_FQS_NOTEMPTY:u8=1;

#[repr(C)] pub struct qm_fqd_stashing{pub exclusive:u8,pub cl:u8} #[repr(C)] pub struct qm_fqd_oac{pub oac:u8,pub oal:i8}
#[repr(C, packed)] pub struct qm_fqd {pub orpc:u8,pub cgid:u8,pub fq_ctrl:__be16,pub dest_wq:__be16,pub ics_cred:__be16,pub td:__be16,pub context_b:__be32,pub context_a:qm_fqd_context_a,pub oac_query:qm_fqd_oac}
#[repr(C)] pub union qm_fqd_context_a {pub opaque:__be64,pub pair:qm_fqd_pair,pub stashing:qm_fqd_stashing_addr}
#[repr(C)] pub struct qm_fqd_pair{pub hi:__be32,pub lo:__be32} #[repr(C,packed)] pub struct qm_fqd_stashing_addr{pub stashing:qm_fqd_stashing,pub context_hi:__be16,pub context_lo:__be32}
pub const QM_FQD_CHAN_OFF:u16=3; pub const QM_FQD_WQ_MASK:u16=7; pub const QM_FQD_TD_EXP_MASK:u16=0x1f; pub const QM_FQD_TD_MANT_OFF:u16=5; pub const QM_FQD_TD_MANT_MASK:u16=0x1fe0; pub const QM_FQD_TD_MAX:u32=0xe0000000; pub const QM_FQD_TD_MANT_MAX:u32=0xff; pub const QM_FQD_OAC_OFF:u8=6; pub const QM_FQD_AS_OFF:u8=4; pub const QM_FQD_DS_OFF:u8=2; pub const QM_FQD_XS_MASK:u8=3;
#[inline] pub unsafe fn qm_fqd_stashing_get64(f:*const qm_fqd)->u64{from_be64((*f).context_a.opaque)&0xffffffffffff} #[inline] pub unsafe fn qm_fqd_stashing_addr(f:*const qm_fqd)->dma_addr_t{qm_fqd_stashing_get64(f)} #[inline] pub unsafe fn qm_fqd_context_a_get64(f:*const qm_fqd)->u64{qm_fqd_stashing_get64(f)}
#[inline] pub unsafe fn qm_fqd_stashing_set64(f:*mut qm_fqd,a:u64){(*f).context_a.stashing.context_hi=be16((a>>32) as u16);(*f).context_a.stashing.context_lo=be32(a as u32)} #[inline] pub unsafe fn qm_fqd_context_a_set64(f:*mut qm_fqd,a:u64){(*f).context_a.pair.hi=be32((a>>32) as u32);(*f).context_a.pair.lo=be32(a as u32)}
#[inline] pub unsafe fn qm_fqd_set_taildrop(f:*mut qm_fqd,mut val:u32,roundup:i32)->i32{if val>QM_FQD_TD_MAX{return -34}let mut e=0;while val>QM_FQD_TD_MANT_MAX{let odd=val&1;val>>=1;e+=1;if roundup!=0&&odd!=0{val+=1}}(*f).td=be16((((val<<QM_FQD_TD_MANT_OFF)&QM_FQD_TD_MANT_MASK as u32)|(e&QM_FQD_TD_EXP_MASK as u32)) as u16);0}
#[inline] pub unsafe fn qm_fqd_get_taildrop(f:*const qm_fqd)->i32{let td=be16((*f).td) as u32;(((td&QM_FQD_TD_MANT_MASK as u32)>>QM_FQD_TD_MANT_OFF)<<(td&QM_FQD_TD_EXP_MASK as u32)) as i32}
#[inline] pub unsafe fn qm_fqd_set_stashing(f:*mut qm_fqd,as_:u8,ds:u8,cs:u8){(*f).context_a.stashing.stashing.cl=((as_&3)<<4)|((ds&3)<<2)|(cs&3)} #[inline] pub unsafe fn qm_fqd_get_stashing(f:*const qm_fqd)->u8{(*f).context_a.stashing.stashing.cl} #[inline] pub unsafe fn qm_fqd_set_oac(f:*mut qm_fqd,v:u8){(*f).oac_query.oac=v<<6} #[inline] pub unsafe fn qm_fqd_set_oal(f:*mut qm_fqd,v:i8){(*f).oac_query.oal=v}
#[inline] pub unsafe fn qm_fqd_set_destwq(f:*mut qm_fqd,ch:i32,wq:i32){(*f).dest_wq=be16(((ch<<3)|(wq&7)) as u16)} #[inline] pub unsafe fn qm_fqd_get_chan(f:*const qm_fqd)->i32{(be16((*f).dest_wq)>>3) as i32} #[inline] pub unsafe fn qm_fqd_get_wq(f:*const qm_fqd)->i32{(be16((*f).dest_wq)&7) as i32}

pub const QM_FQCTRL_MASK:u16=0x07ff; pub const QM_FQCTRL_CGE:u16=0x0400; pub const QM_FQCTRL_TDE:u16=0x0200; pub const QM_FQCTRL_CTXASTASHING:u16=0x80; pub const QM_FQCTRL_CPCSTASH:u16=0x40; pub const QM_FQCTRL_FORCESFDR:u16=8; pub const QM_FQCTRL_AVOIDBLOCK:u16=4; pub const QM_FQCTRL_HOLDACTIVE:u16=2; pub const QM_FQCTRL_PREFERINCACHE:u16=1; pub const QM_FQCTRL_LOCKINCACHE:u16=1;
pub const QM_STASHING_EXCL_ANNOTATION:u8=4; pub const QM_STASHING_EXCL_DATA:u8=2; pub const QM_STASHING_EXCL_CTX:u8=1; pub const QM_OAC_ICS:u8=2; pub const QM_OAC_CG:u8=1;
#[repr(C)] pub struct qm_cgr_wr_parm{pub word:__be32} #[repr(C)] pub struct qm_cgr_cs_thres{pub word:__be16}
#[repr(C,packed)] pub struct qm_mc_cgr{pub wr_parm_g:qm_cgr_wr_parm,pub wr_parm_y:qm_cgr_wr_parm,pub wr_parm_r:qm_cgr_wr_parm,pub wr_en_g:u8,pub wr_en_y:u8,pub wr_en_r:u8,pub cscn_en:u8,pub cscn_targ:__be32,pub cstd_en:u8,pub cs:u8,pub cs_thres:qm_cgr_cs_thres,pub mode:u8}
pub const QM_CGR_EN:u8=1; pub const QM_CGR_TARG_UDP_CTRL_WRITE_BIT:u16=0x8000; pub const QM_CGR_TARG_UDP_CTRL_DCP:u16=0x4000; pub const QM_CGR_TARG_FMAN0:u32=0x00200000; pub const QM_CGR_TARG_FMAN1:u32=0x00100000;
#[inline] pub unsafe fn qm_cgr_cs_thres_get64(t:*const qm_cgr_cs_thres)->u64{let v=be16((*t).word) as u64;((v>>5)&0xff)<<(v&0x1f)} #[inline] pub unsafe fn qm_cgr_cs_thres_set64(t:*mut qm_cgr_cs_thres,mut v:u64,roundup:i32)->i32{let mut e=0;while v>0xff{let odd=v&1;v>>=1;e+=1;if roundup!=0&&odd!=0{v+=1}}(*t).word=be16((((v&0xff)<<5)|(e&0x1f)) as u16);0}

#[repr(C,packed)] pub struct qm_mcc_initfq{pub __reserved1:[u8;2],pub we_mask:__be16,pub fqid:__be32,pub count:__be16,pub fqd:qm_fqd,pub __reserved2:[u8;30]} #[repr(C,packed)] pub struct qm_mcc_initcgr{pub __reserve1:[u8;2],pub we_mask:__be16,pub cgr:qm_mc_cgr,pub __reserved2:[u8;2],pub cgid:u8,pub __reserved3:[u8;32]}
pub const QM_INITFQ_WE_MASK:u16=0x1ff; pub const QM_INITFQ_WE_OAC:u16=0x100; pub const QM_INITFQ_WE_ORPC:u16=0x80; pub const QM_INITFQ_WE_CGID:u16=0x40; pub const QM_INITFQ_WE_FQCTRL:u16=0x20; pub const QM_INITFQ_WE_DESTWQ:u16=0x10; pub const QM_INITFQ_WE_ICSCRED:u16=8; pub const QM_INITFQ_WE_TDTHRESH:u16=4; pub const QM_INITFQ_WE_CONTEXTB:u16=2; pub const QM_INITFQ_WE_CONTEXTA:u16=1;
pub const QM_CGR_WE_MASK:u16=0x7ff; pub const QM_CGR_WE_WR_PARM_G:u16=0x400; pub const QM_CGR_WE_WR_PARM_Y:u16=0x200; pub const QM_CGR_WE_WR_PARM_R:u16=0x100; pub const QM_CGR_WE_WR_EN_G:u16=0x80; pub const QM_CGR_WE_WR_EN_Y:u16=0x40; pub const QM_CGR_WE_WR_EN_R:u16=0x20; pub const QM_CGR_WE_CSCN_EN:u16=0x10; pub const QM_CGR_WE_CSCN_TARG:u16=8; pub const QM_CGR_WE_CSTD_EN:u16=4; pub const QM_CGR_WE_CS_THRES:u16=2; pub const QM_CGR_WE_MODE:u16=1;
pub const QMAN_CGR_FLAG_USE_INIT:u32=1; pub const QMAN_CGR_MODE_FRAME:u32=1;

pub enum qman_portal{} pub enum qman_fq{} pub enum qman_cgr{}
#[repr(i32)] pub enum qman_cb_dqrr_result{qman_cb_dqrr_consume,qman_cb_dqrr_park,qman_cb_dqrr_defer,qman_cb_dqrr_stop,qman_cb_dqrr_consume_stop}
pub type qman_cb_dqrr=unsafe extern "C" fn(*mut qman_portal,*mut qman_fq,*const qm_dqrr_entry,bool)->qman_cb_dqrr_result; pub type qman_cb_mr=unsafe extern "C" fn(*mut qman_portal,*mut qman_fq,*const qm_mr_entry); pub type qman_cb_cgr=unsafe extern "C" fn(*mut qman_portal,*mut qman_cgr,i32);
#[repr(i32)] pub enum qman_fq_state{qman_fq_state_oos,qman_fq_state_parked,qman_fq_state_sched,qman_fq_state_retired}
pub const QMAN_FQ_STATE_CHANGING:u32=0x80000000; pub const QMAN_FQ_STATE_NE:u32=0x40000000; pub const QMAN_FQ_STATE_ORL:u32=0x20000000; pub const QMAN_FQ_STATE_BLOCKOOS:u32=0xe0000000; pub const QMAN_FQ_STATE_CGR_EN:u32=0x10000000; pub const QMAN_FQ_STATE_VDQCR:u32=0x08000000;
#[repr(C)] pub struct qman_fq_cb{pub dqrr:Option<qman_cb_dqrr>,pub ern:Option<qman_cb_mr>,pub fqs:Option<qman_cb_mr>} #[repr(C)] pub struct qman_fq_real{pub cb:qman_fq_cb,pub fqid:u32,pub idx:u32,pub flags:usize,pub state:qman_fq_state,pub cgr_groupid:i32} #[repr(C)] pub struct qman_cgr_real{pub cgrid:u32,pub cb:Option<qman_cb_cgr>,pub chan:u16,pub node:*mut core::ffi::c_void}
pub const QMAN_FQ_FLAG_NO_ENQUEUE:u32=1; pub const QMAN_FQ_FLAG_NO_MODIFY:u32=2; pub const QMAN_FQ_FLAG_TO_DCPORTAL:u32=4; pub const QMAN_FQ_FLAG_DYNAMIC_FQID:u32=0x20; pub const QMAN_INITFQ_FLAG_SCHED:u32=1; pub const QMAN_INITFQ_FLAG_LOCAL:u32=4;
pub const QM_VDQCR_PRECEDENCE_VDQCR:u32=0; pub const QM_VDQCR_PRECEDENCE_SDQCR:u32=0x80000000; pub const QM_VDQCR_EXACT:u32=0x40000000; pub const QM_VDQCR_NUMFRAMES_MASK:u32=0x3f000000; #[inline] pub fn QM_VDQCR_NUMFRAMES_SET(n:u32)->u32{(n&0x3f)<<24} #[inline] pub fn QM_VDQCR_NUMFRAMES_GET(n:u32)->u32{(n>>24)&0x3f} pub const QM_VDQCR_NUMFRAMES_TILLEMPTY:u32=0; pub const QMAN_VOLATILE_FLAG_WAIT:u32=1; pub const QMAN_VOLATILE_FLAG_WAIT_INT:u32=2; pub const QMAN_VOLATILE_FLAG_FINISH:u32=4;

#[repr(C,packed)] pub struct qm_mcr_queryfq_np{pub verb:u8,pub result:u8,pub __reserved1:u8,pub state:u8,pub fqd_link:u32,pub odp_seq:u16,pub orp_nesn:u16,pub orp_ea_hseq:u16,pub orp_ea_tseq:u16,pub orp_ea_hptr:u32,pub orp_ea_tptr:u32,pub pfdr_hptr:u32,pub pfdr_tptr:u32,pub __reserved2:[u8;5],pub is:u8,pub ics_surp:u16,pub byte_cnt:u32,pub frm_cnt:u32,pub __reserved3:u32,pub ra1_sfdr:u16,pub ra2_sfdr:u16,pub __reserved4:u16,pub od1_sfdr:u16,pub od2_sfdr:u16,pub od3_sfdr:u16}
pub const QM_MCR_NP_STATE_FE:u8=0x10; pub const QM_MCR_NP_STATE_R:u8=8; pub const QM_MCR_NP_STATE_MASK:u8=7; pub const QM_MCR_NP_PTR_MASK:u16=0x7ff;

/* External portal, queue, pool, CGR, and query APIs remain declarations. */
extern "C" { pub fn qman_p_irqsource_add(p:*mut qman_portal,bits:u32); pub fn qman_p_irqsource_remove(p:*mut qman_portal,bits:u32); pub fn qman_affine_channel(cpu:i32)->u16; pub fn qman_get_affine_portal(cpu:i32)->*mut qman_portal; pub fn qman_p_poll_dqrr(p:*mut qman_portal,limit:u32)->i32; pub fn qman_create_fq(fqid:u32,flags:u32,fq:*mut qman_fq)->i32; pub fn qman_destroy_fq(fq:*mut qman_fq); pub fn qman_fq_fqid(fq:*mut qman_fq)->u32; pub fn qman_init_fq(fq:*mut qman_fq,flags:u32,opts:*mut qm_mcc_initfq)->i32; pub fn qman_schedule_fq(fq:*mut qman_fq)->i32; pub fn qman_retire_fq(fq:*mut qman_fq,flags:*mut u32)->i32; pub fn qman_oos_fq(fq:*mut qman_fq)->i32; pub fn qman_volatile_dequeue(fq:*mut qman_fq,flags:u32,vdqcr:u32)->i32; pub fn qman_enqueue(fq:*mut qman_fq,fd:*const qm_fd)->i32; pub fn qman_alloc_fqid_range(result:*mut u32,count:u32)->i32; pub fn qman_release_fqid(fqid:u32)->i32; pub fn qman_query_fq_np(fq:*mut qman_fq,np:*mut qm_mcr_queryfq_np)->i32; pub fn qman_alloc_pool_range(result:*mut u32,count:u32)->i32; pub fn qman_release_pool(id:u32)->i32; pub fn qman_create_cgr(cgr:*mut qman_cgr,flags:u32,opts:*mut qm_mcc_initcgr)->i32; pub fn qman_delete_cgr(cgr:*mut qman_cgr)->i32; pub fn qman_delete_cgr_safe(cgr:*mut qman_cgr); pub fn qman_update_cgr_safe(cgr:*mut qman_cgr,opts:*mut qm_mcc_initcgr)->i32; pub fn qman_query_cgr_congested(cgr:*mut qman_cgr,result:*mut bool)->i32; pub fn qman_alloc_cgrid_range(result:*mut u32,count:u32)->i32; pub fn qman_release_cgrid(id:u32)->i32; pub fn qman_is_probed()->i32; pub fn qman_portals_probed()->i32; pub fn qman_dqrr_get_ithresh(portal:*mut qman_portal,ithresh:*mut u8); pub fn qman_dqrr_set_ithresh(portal:*mut qman_portal,ithresh:u8)->i32; pub fn qman_portal_get_iperiod(portal:*mut qman_portal,iperiod:*mut u32); pub fn qman_portal_set_iperiod(portal:*mut qman_portal,iperiod:u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
