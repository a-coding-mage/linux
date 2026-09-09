/* SPDX-License-Identifier: GPL-2.0 */
/* CAAM hardware register-level view, translated from regs.h. */

extern "C" {
    pub static mut caam_little_end: bool;
    pub static mut caam_imx: bool;
    pub static mut caam_ptr_sz: usize;
}

pub type dma_addr_t = u64;
pub type __iomem = core::ffi::c_void;

#[inline] pub unsafe fn caam16_to_cpu(v: u16) -> u16 { if caam_little_end { u16::from_le(v) } else { u16::from_be(v) } }
#[inline] pub unsafe fn caam32_to_cpu(v: u32) -> u32 { if caam_little_end { u32::from_le(v) } else { u32::from_be(v) } }
#[inline] pub unsafe fn caam64_to_cpu(v: u64) -> u64 { if caam_little_end { u64::from_le(v) } else { u64::from_be(v) } }
#[inline] pub unsafe fn cpu_to_caam16(v: u16) -> u16 { if caam_little_end { v.to_le() } else { v.to_be() } }
#[inline] pub unsafe fn cpu_to_caam32(v: u32) -> u32 { if caam_little_end { v.to_le() } else { v.to_be() } }
#[inline] pub unsafe fn cpu_to_caam64(v: u64) -> u64 { if caam_little_end { v.to_le() } else { v.to_be() } }

extern "C" {
    fn iowrite32(v: u32, p: *mut __iomem); fn iowrite32be(v: u32, p: *mut __iomem);
    fn ioread32(p: *mut __iomem) -> u32; fn ioread32be(p: *mut __iomem) -> u32;
    fn iowrite64(v: u64, p: *mut __iomem); fn iowrite64be(v: u64, p: *mut __iomem);
    fn ioread64(p: *mut __iomem) -> u64; fn ioread64be(p: *mut __iomem) -> u64;
}
#[inline] pub unsafe fn wr_reg32(r: *mut __iomem, d: u32) { if caam_little_end { iowrite32(d,r) } else { iowrite32be(d,r) } }
#[inline] pub unsafe fn rd_reg32(r: *mut __iomem) -> u32 { if caam_little_end { ioread32(r) } else { ioread32be(r) } }
#[inline] pub unsafe fn clrsetbits_32(r: *mut __iomem, c: u32, s: u32) { wr_reg32(r, rd_reg32(r) & !c | s); }
#[inline] pub unsafe fn wr_reg64(r: *mut __iomem, d: u64) { if caam_little_end && caam_imx { iowrite32((d>>32) as u32,r); iowrite32(d as u32,r.add(1) as *mut _); } else if caam_little_end { iowrite64(d,r) } else { iowrite64be(d,r) } }
#[inline] pub unsafe fn rd_reg64(r: *mut __iomem) -> u64 { if caam_little_end && caam_imx { let h=ioread32(r) as u64; let l=ioread32(r.add(1) as *mut _) as u64; l|(h<<32) } else if caam_little_end { ioread64(r) } else { ioread64be(r) } }
#[inline] pub unsafe fn cpu_to_caam_dma64(v: dma_addr_t) -> u64 { if caam_imx { ((cpu_to_caam32(v as u32) as u64)<<32) | cpu_to_caam32((v>>32) as u32) as u64 } else { cpu_to_caam64(v) } }
#[inline] pub unsafe fn caam_dma64_to_cpu(v:u64)->u64 { if caam_imx { ((caam32_to_cpu(v as u32) as u64)<<32)|caam32_to_cpu((v>>32) as u32) as u64 } else { caam64_to_cpu(v) } }
#[inline] pub unsafe fn cpu_to_caam_dma(v:u64)->u64 { if caam_ptr_sz==8 {cpu_to_caam_dma64(v)} else {cpu_to_caam32(v as u32) as u64} }
#[inline] pub unsafe fn caam_dma_to_cpu(v:u64)->u64 { if caam_ptr_sz==8 {caam_dma64_to_cpu(v)} else {caam32_to_cpu(v as u32) as u64} }

#[repr(C, packed)] pub struct JrOut32 { pub desc:u32, pub jrstatus:u32 }
#[repr(C, packed)] pub struct JrOut64 { pub desc:dma_addr_t, pub jrstatus:u32 }
#[inline] pub unsafe fn jr_outentry_get(o:*mut core::ffi::c_void,i:isize,d:*mut dma_addr_t,s:*mut u32) { if caam_ptr_sz==4 { let x=(o as *mut JrOut32).add(i as usize); *d=(*x).desc as u64; *s=(*x).jrstatus } else { let x=(o as *mut JrOut64).add(i as usize); *d=(*x).desc; *s=(*x).jrstatus } }
pub const SIZEOF_JR_OUTENTRY: usize = 0; // caam_ptr_sz + sizeof(u32)
#[inline] pub unsafe fn jr_outentry_desc(o:*mut core::ffi::c_void,i:isize)->dma_addr_t { let mut d=0; let mut s=0; jr_outentry_get(o,i,&mut d,&mut s); d }
#[inline] pub unsafe fn jr_outentry_jrstatus(o:*mut core::ffi::c_void,i:isize)->u32 { let mut d=0; let mut s=0; jr_outentry_get(o,i,&mut d,&mut s); s }
#[inline] pub unsafe fn jr_inpentry_set(o:*mut core::ffi::c_void,i:isize,v:dma_addr_t) { if caam_ptr_sz==4 { *(o as *mut u32).offset(i)=v as u32 } else { *(o as *mut dma_addr_t).offset(i)=v } }
pub const SIZEOF_JR_INPENTRY: usize = 0; // caam_ptr_sz

#[repr(C)] pub struct version_regs { pub crca:u32,pub afha:u32,pub kfha:u32,pub pkha:u32,pub aesa:u32,pub mdha:u32,pub desa:u32,pub snw8a:u32,pub snw9a:u32,pub zuce:u32,pub zuca:u32,pub ccha:u32,pub ptha:u32,pub rng:u32,pub trng:u32,pub aaha:u32,pub rsvd:[u32;10],pub sr:u32,pub dma:u32,pub ai:u32,pub qi:u32,pub jr:u32,pub deco:u32 }
pub const CHA_VER_NUM_MASK:u64=0xff; pub const CHA_VER_MISC_SHIFT:u32=8; pub const CHA_VER_MISC_MASK:u64=0xff<<8; pub const CHA_VER_REV_SHIFT:u32=16; pub const CHA_VER_REV_MASK:u64=0xff<<16; pub const CHA_VER_VID_SHIFT:u32=24; pub const CHA_VER_VID_MASK:u64=0xff<<24;
pub const CHA_VER_MISC_AES_NUM_MASK:u32=0xff; pub const CHA_VER_MISC_AES_GCM:u32=1<<(1+8); pub const CHA_VER_MISC_PKHA_NO_CRYPT:u32=1<<(7+8);

#[repr(C)] pub struct sec_vid { pub ip_id:u16,pub maj_rev:u8,pub min_rev:u8 }
#[repr(C)] pub struct masterid { pub liodn_ms:u32,pub liodn_ls:u32 }
#[repr(C)] pub struct rtic_element { pub address:u64,pub rsvd:u32,pub length:u32 }
#[repr(C)] pub struct rtic_block { pub element:[rtic_element;2] }
#[repr(C)] pub struct rtic_memhash { pub memhash_be:[u32;32],pub memhash_le:[u32;32] }
#[repr(C)] pub struct deco_sg_table { pub addr:u64,pub elen:u32,pub bpid_offset:u32 }

#[repr(C)] pub struct caam_perfmon { pub req_dequeued:u64,pub ob_enc_req:u64,pub ib_dec_req:u64,pub ob_enc_bytes:u64,pub ob_prot_bytes:u64,pub ib_dec_bytes:u64,pub ib_valid_bytes:u64,pub rsvd:[u64;13],pub cha_rev_ms:u32,pub cha_rev_ls:u32,pub comp_parms_ms:u32,pub comp_parms_ls:u32,pub rsvd1:[u64;2],pub faultaddr:u64,pub faultliodn:u32,pub faultdetail:u32,pub rsvd2:u32,pub status:u32,pub rsvd3:u64,pub rtic_id:u32,pub ccb_id:u32,pub cha_id_ms:u32,pub cha_id_ls:u32,pub cha_num_ms:u32,pub cha_num_ls:u32,pub caam_id_ms:u32,pub caam_id_ls:u32 }
#[repr(C)] pub struct rngtst { pub mode:u32,pub rsvd1:[u32;3],pub reset:u32,pub rsvd2:[u32;3],pub status:u32,pub rsvd3:u32,pub errstat:u32,pub rsvd4:u32,pub errctl:u32,pub rsvd5:u32,pub entropy:u32,pub rsvd6:[u32;15],pub verifctl:u32,pub rsvd7:u32,pub verifstat:u32,pub rsvd8:u32,pub verifdata:u32,pub rsvd9:u32,pub xkey:u32,pub rsvd10:u32,pub oscctctl:u32,pub rsvd11:u32,pub oscct:u32,pub rsvd12:u32,pub oscctstat:u32,pub rsvd13:[u32;2],pub ofifo:[u32;4],pub rsvd14:[u32;15] }
#[repr(C)] pub union rng4test_union { pub rtpkrmax:u32,pub rtpkrsq:u32 }
#[repr(C)] pub union rng4test_union2 { pub rtsblim:u32,pub rttotsam:u32 }
#[repr(C)] pub union rng4test_union3 { pub rtfrqmax:u32,pub rtfrqcnt:u32 }
#[repr(C)] pub union rng4test_union4 { pub rtscmc:u32,pub rtscml:u32 }
#[repr(C)] pub union rng4test_union5 { pub rtscrc:[u32;6],pub rtscrl:[u32;6] }
#[repr(C)] pub struct rng4tst { pub rtmctl:u32,pub rtscmisc:u32,pub rtpkrrng:u32,pub pkr:rng4test_union,pub rtsdctl:u32,pub sb:rng4test_union2,pub rtfrqmin:u32,pub frq:rng4test_union3,pub scm:rng4test_union4,pub scr:rng4test_union5,pub rsvd1:[u32;33],pub rdsta:u32,pub rsvd2:[u32;15] }

pub const KEK_KEY_SIZE:usize=8; pub const TKEK_KEY_SIZE:usize=8; pub const TDSK_KEY_SIZE:usize=8;
#[repr(C)] pub union caam_rng { pub rtst:[rngtst;2], pub r4tst:[rng4tst;2] }
#[repr(C)] pub struct caam_ctrl { pub rsvd1:u32,pub mcr:u32,pub rsvd2:u32,pub scfgr:u32,pub jr_mid:[masterid;4],pub rsvd3:[u32;11],pub jrstart:u32,pub rtic_mid:[masterid;4],pub rsvd4:[u32;5],pub deco_rsr:u32,pub rsvd11:u32,pub deco_rq:u32,pub deco_mid:[masterid;16],pub deco_avail:u32,pub deco_reset:u32,pub rsvd6:[u32;182],pub kek:[u32;8],pub tkek:[u32;8],pub tdsk:[u32;8],pub rsvd7:[u32;32],pub sknonce:u64,pub rsvd8:[u32;70],pub rng:caam_rng,pub rsvd9:[u32;416],pub vreg:version_regs,pub perfmon:caam_perfmon }
#[repr(C)] pub struct caam_job_ring { pub inpring_base:u64,pub rsvd1:u32,pub inpring_size:u32,pub rsvd2:u32,pub inpring_avail:u32,pub rsvd3:u32,pub inpring_jobadd:u32,pub outring_base:u64,pub rsvd4:u32,pub outring_size:u32,pub rsvd5:u32,pub outring_rmvd:u32,pub rsvd6:u32,pub outring_used:u32,pub rsvd7:u32,pub jroutstatus:u32,pub rsvd8:u32,pub jrintstatus:u32,pub rconfig_hi:u32,pub rconfig_lo:u32,pub rsvd9:u32,pub inp_rdidx:u32,pub rsvd10:u32,pub out_wtidx:u32,pub rsvd11:u32,pub jrcommand:u32,pub rsvd12:[u32;900],pub vreg:version_regs,pub perfmon:caam_perfmon }

#[repr(C)] pub struct caam_assurance { pub rsvd1:u32,pub status:u32,pub rsvd2:u32,pub cmd:u32,pub rsvd3:u32,pub ctrl:u32,pub rsvd4:u32,pub throttle:u32,pub rsvd5:[u32;2],pub watchdog:u64,pub rsvd6:u32,pub rend:u32,pub rsvd7:[u32;50],pub memblk:[rtic_block;4],pub rsvd8:[u32;32],pub hash:[rtic_memhash;4],pub rsvd_3:[u32;640] }
#[repr(C)] pub struct caam_queue_if { pub qi_control_hi:u32,pub qi_control_lo:u32,pub rsvd1:u32,pub qi_status:u32,pub qi_deq_cfg_hi:u32,pub qi_deq_cfg_lo:u32,pub qi_enq_cfg_hi:u32,pub qi_enq_cfg_lo:u32,pub rsvd2:[u32;1016] }

#[repr(C)] pub struct caam_deco { pub rsvd1:u32,pub cls1_mode:u32,pub rsvd2:u32,pub cls1_keysize:u32,pub cls1_datasize_hi:u32,pub cls1_datasize_lo:u32,pub rsvd3:u32,pub cls1_icvsize:u32,pub rsvd4:[u32;5],pub cha_ctrl:u32,pub rsvd5:u32,pub irq_crtl:u32,pub rsvd6:u32,pub clr_written:u32,pub ccb_status_hi:u32,pub ccb_status_lo:u32,pub rsvd7:[u32;3],pub aad_size:u32,pub rsvd8:u32,pub cls1_iv_size:u32,pub rsvd9:[u32;7],pub pkha_a_size:u32,pub rsvd10:u32,pub pkha_b_size:u32,pub rsvd11:u32,pub pkha_n_size:u32,pub rsvd12:u32,pub pkha_e_size:u32,pub rsvd13:[u32;24],pub cls1_ctx:[u32;16],pub rsvd14:[u32;48],pub cls1_key:[u32;8],pub rsvd15:[u32;121],pub cls2_mode:u32,pub rsvd16:u32,pub cls2_keysize:u32,pub cls2_datasize_hi:u32,pub cls2_datasize_lo:u32,pub rsvd17:u32,pub cls2_icvsize:u32,pub rsvd18:[u32;56],pub cls2_ctx:[u32;18],pub rsvd19:[u32;46],pub cls2_key:[u32;32],pub rsvd20:[u32;84],pub inp_infofifo_hi:u32,pub inp_infofifo_lo:u32,pub rsvd21:[u32;2],pub inp_datafifo:u64,pub rsvd22:[u32;2],pub out_datafifo:u64,pub rsvd23:[u32;2],pub jr_ctl_hi:u32,pub jr_ctl_lo:u32,pub jr_descaddr:u64,pub op_status_hi:u32,pub op_status_lo:u32,pub rsvd24:[u32;2],pub liodn:u32,pub td_liodn:u32,pub rsvd26:[u32;6],pub math:[u64;4],pub rsvd27:[u32;8],pub gthr_tbl:[deco_sg_table;4],pub rsvd28:[u32;16],pub sctr_tbl:[deco_sg_table;4],pub rsvd29:[u32;48],pub descbuf:[u32;64],pub rscvd30:[u32;193],pub desc_dbg:u32,pub rsvd31:[u32;13],pub dbg_exec:u32,pub rsvd32:[u32;112] }

pub const MCFGR_SWRESET:u32=0x80000000; pub const MCFGR_WDENABLE:u32=0x40000000; pub const MCFGR_WDFAIL:u32=0x20000000; pub const MCFGR_DMA_RESET:u32=0x10000000; pub const MCFGR_LONG_PTR:u32=0x10000; pub const SCFGR_RDBENABLE:u32=0x400; pub const SCFGR_VIRT_EN:u32=0x8000; pub const DECORR_RQD0ENABLE:u32=1; pub const DECORSR_JR0:u32=1; pub const DECORSR_VALID:u32=0x80000000; pub const DECORR_DEN0:u32=0x10000;
pub const MCFGR_ARCACHE_SHIFT:u32=12; pub const MCFGR_ARCACHE_MASK:u32=0xf<<12; pub const MCFGR_ARCACHE_BUFF:u32=1<<12; pub const MCFGR_ARCACHE_CACH:u32=2<<12; pub const MCFGR_ARCACHE_RALL:u32=4<<12; pub const MCFGR_AWCACHE_SHIFT:u32=8; pub const MCFGR_AWCACHE_MASK:u32=0xf<<8; pub const MCFGR_AWCACHE_BUFF:u32=1<<8; pub const MCFGR_AWCACHE_CACH:u32=2<<8; pub const MCFGR_AWCACHE_WALL:u32=8<<8; pub const MCFGR_AXIPIPE_SHIFT:u32=4; pub const MCFGR_AXIPIPE_MASK:u32=0xf<<4; pub const MCFGR_AXIPRI:u32=8; pub const MCFGR_LARGE_BURST:u32=4; pub const MCFGR_BURST_64:u32=1;
pub const JRSTART_JR0_START:u32=1; pub const JRSTART_JR1_START:u32=2; pub const JRSTART_JR2_START:u32=4; pub const JRSTART_JR3_START:u32=8;
pub const JR_RINGSIZE_MASK:u32=0x3ff; pub const JRCR_RESET:u32=1; pub const JR_BLOCK_NUMBER:u32=1; pub const ASSURE_BLOCK_NUMBER:u32=6; pub const QI_BLOCK_NUMBER:u32=7; pub const DECO_BLOCK_NUMBER:u32=8; pub const PG_SIZE_4K:u32=0x1000; pub const PG_SIZE_64K:u32=0x10000;
pub const QICTL_DQEN:u32=1; pub const QICTL_STOP:u32=2; pub const QICTL_SOE:u32=4; pub const QICTL_MBSI:u32=1; pub const QICTL_MHWSI:u32=2; pub const QICTL_MWSI:u32=4; pub const QICTL_MDWSI:u32=8; pub const QICTL_CBSI:u32=0x10; pub const QICTL_CHWSI:u32=0x20; pub const QICTL_CWSI:u32=0x40; pub const QICTL_CDWSI:u32=0x80; pub const QICTL_MBSO:u32=0x100; pub const QICTL_MHWSO:u32=0x200; pub const QICTL_MWSO:u32=0x400; pub const QICTL_MDWSO:u32=0x800; pub const QICTL_CBSO:u32=0x1000; pub const QICTL_CHWSO:u32=0x2000; pub const QICTL_CWSO:u32=0x4000; pub const QICTL_CDWSO:u32=0x8000; pub const QICTL_DMBS:u32=0x10000; pub const QICTL_EPO:u32=0x20000; pub const QISTA_PHRDERR:u32=1; pub const QISTA_CFRDERR:u32=2; pub const QISTA_OFWRERR:u32=4; pub const QISTA_BPDERR:u32=8; pub const QISTA_BTSERR:u32=0x10; pub const QISTA_CFWRERR:u32=0x20; pub const QISTA_STOPD:u32=0x80000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
