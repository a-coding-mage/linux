// SPDX-License-Identifier: GPL-2.0-only
/* Intel E3-1200 EDAC driver, translated literally from ie31200_edac.c. */

const EDAC_MOD_STR: *const u8 = b"ie31200_edac\0".as_ptr();
const IE31200_RANKS_PER_CHANNEL: usize = 8;
const IE31200_DIMMS_PER_CHANNEL: usize = 2;
const IE31200_CHANNELS: usize = 2;
const IE31200_IMC_NUM: usize = 2;
const IE31200_MCHBAR_LOW: u16 = 0x48;
const IE31200_MCHBAR_HIGH: u16 = 0x4c;
const IE31200_ERRSTS: u16 = 0xc8;
const IE31200_ERRSTS_UE: u16 = 1 << 1;
const IE31200_ERRSTS_CE: u16 = 1;
const IE31200_ERRSTS_BITS: u16 = IE31200_ERRSTS_UE | IE31200_ERRSTS_CE;
const IE31200_CAPID0: u16 = 0xe4;
const IE31200_CAPID0_PDCD: u8 = 1 << 4;
const IE31200_CAPID0_DDPCD: u8 = 1 << 6;
const IE31200_CAPID0_ECC: u8 = 1 << 1;

static mut nr_channels: i32 = 0;
static mut mci_pdev: *mut pci_dev = core::ptr::null_mut();
static mut ie31200_registered: i32 = 1;

#[repr(C)]
struct res_config {
    mtype: mem_type, cmci: bool, imc_num: i32,
    reg_mchbar_mask: u64, reg_mchbar_window_size: u64,
    reg_eccerrlog_offset: [u64; IE31200_CHANNELS],
    reg_eccerrlog_ce_mask: u64, reg_eccerrlog_ce_ovfl_mask: u64,
    reg_eccerrlog_ue_mask: u64, reg_eccerrlog_ue_ovfl_mask: u64,
    reg_eccerrlog_rank_mask: u64, reg_eccerrlog_syndrome_mask: u64,
    msr_clear_eccerrlog_offset: u32, reg_mad_dimm_size_granularity: u64,
    reg_mad_dimm_offset: [u64; IE31200_CHANNELS],
    reg_mad_dimm_size_mask: [u32; IE31200_DIMMS_PER_CHANNEL],
    reg_mad_dimm_rank_mask: [u32; IE31200_DIMMS_PER_CHANNEL],
    reg_mad_dimm_width_mask: [u32; IE31200_DIMMS_PER_CHANNEL],
}
#[repr(C)] struct ie31200_priv { window: *mut u8, c0errlog: *mut u8, c1errlog: *mut u8, cfg: *mut res_config, mci: *mut mem_ctl_info, pdev: *mut pci_dev, dev: device }
#[repr(C)] struct ie31200_pvt { priv_: [*mut ie31200_priv; IE31200_IMC_NUM] }
static mut ie31200_pvt: ie31200_pvt = ie31200_pvt { priv_: [core::ptr::null_mut(); IE31200_IMC_NUM] };
#[repr(C)] struct dimm_data { size: u64, ranks: u8, dtype: dev_type }
#[repr(C)] struct ie31200_error_info { errsts: u16, errsts2: u16, eccerrlog: [u64; IE31200_CHANNELS], erraddr: u64 }

unsafe fn how_many_channels(pdev: *mut pci_dev) -> i32 {
    let mut v = 0u8; pci_read_config_byte(pdev, IE31200_CAPID0 + 1, &mut v);
    if v & IE31200_CAPID0_PDCD != 0 { edac_dbg(0, "In single channel mode\n"); 1 } else { edac_dbg(0, "In dual channel mode\n"); 2 }
}
unsafe fn ecc_capable(pdev: *mut pci_dev) -> bool { let mut v=0u8; pci_read_config_byte(pdev, IE31200_CAPID0+3, &mut v); v & IE31200_CAPID0_ECC == 0 }
unsafe fn mci_to_pci_dev(mci: *mut mem_ctl_info) -> *mut pci_dev { (*(mci)).pvt_info as *mut ie31200_priv as *mut pci_dev }

unsafe fn ie31200_clear_error_info(mci: *mut mem_ctl_info) {
    let p=(*mci).pvt_info as *mut ie31200_priv; let c=&*(*p).cfg;
    if c.msr_clear_eccerrlog_offset != 0 { if wrmsrq_safe(c.msr_clear_eccerrlog_offset, c.reg_eccerrlog_ce_mask|c.reg_eccerrlog_ce_ovfl_mask|c.reg_eccerrlog_ue_mask|c.reg_eccerrlog_ue_ovfl_mask)<0 { ie31200_printk(KERN_ERR,"Failed to wrmsr.\n"); } } else { pci_write_bits16(mci_to_pci_dev(mci),IE31200_ERRSTS,IE31200_ERRSTS_BITS,IE31200_ERRSTS_BITS); }
}
unsafe fn ie31200_get_and_clear_error_info(mci:*mut mem_ctl_info, i:*mut ie31200_error_info) {
    let p=(*mci).pvt_info as *mut ie31200_priv; let d=mci_to_pci_dev(mci); let c=&*(*p).cfg;
    if c.msr_clear_eccerrlog_offset!=0 { (*i).eccerrlog[0]=lo_hi_readq((*p).c0errlog); if nr_channels==2 {(*i).eccerrlog[1]=lo_hi_readq((*p).c1errlog);} ie31200_clear_error_info(mci); return; }
    pci_read_config_word(d,IE31200_ERRSTS,&mut (*i).errsts); if (*i).errsts&IE31200_ERRSTS_BITS==0{return;}
    (*i).eccerrlog[0]=lo_hi_readq((*p).c0errlog); if nr_channels==2 {(*i).eccerrlog[1]=lo_hi_readq((*p).c1errlog);} pci_read_config_word(d,IE31200_ERRSTS,&mut (*i).errsts2);
    if ((*i).errsts^(*i).errsts2)&IE31200_ERRSTS_BITS!=0 {(*i).eccerrlog[0]=lo_hi_readq((*p).c0errlog); if nr_channels==2 {(*i).eccerrlog[1]=lo_hi_readq((*p).c1errlog);}}
    ie31200_clear_error_info(mci);
}
unsafe fn ie31200_process_error_info(mci:*mut mem_ctl_info,i:*mut ie31200_error_info) { let p=(*mci).pvt_info as *mut ie31200_priv; let c=&*(*p).cfg; if c.msr_clear_eccerrlog_offset==0 {if (*i).errsts&IE31200_ERRSTS_BITS==0{return;} if ((*i).errsts^(*i).errsts2)&IE31200_ERRSTS_BITS!=0 {edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED,mci,1,0,0,0,-1,-1,-1,"UE overwrote CE","");(*i).errsts=(*i).errsts2;}}
    for ch in 0..nr_channels { let l=(*i).eccerrlog[ch as usize]; if l&c.reg_eccerrlog_ue_mask!=0 {edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED,mci,1,(*i).erraddr>>PAGE_SHIFT,0,0,field_get(c.reg_eccerrlog_rank_mask,l),ch,-1,"ie31200 UE","");} else if l&c.reg_eccerrlog_ce_mask!=0 {edac_mc_handle_error(HW_EVENT_ERR_CORRECTED,mci,1,(*i).erraddr>>PAGE_SHIFT,0,0,field_get(c.reg_eccerrlog_rank_mask,l),ch,-1,"ie31200 CE","");}}
}
unsafe fn __ie31200_check(mci:*mut mem_ctl_info,mce:*mut mce){let mut i=core::mem::zeroed::<ie31200_error_info>();i.erraddr=if mce.is_null(){0}else{(*mce).addr};ie31200_get_and_clear_error_info(mci,&mut i);ie31200_process_error_info(mci,&mut i)}
unsafe fn ie31200_check(mci:*mut mem_ctl_info){__ie31200_check(mci,core::ptr::null_mut())}

unsafe fn ie31200_map_mchbar(pdev:*mut pci_dev,cfg:*mut res_config,mc:i32)->*mut u8 { let mut lo=0u32;let mut hi=0u32;pci_read_config_dword(pdev,IE31200_MCHBAR_LOW,&mut lo);pci_read_config_dword(pdev,IE31200_MCHBAR_HIGH,&mut hi);let mut a=((hi as u64)<<32)|(lo as u64);a&=(*cfg).reg_mchbar_mask;a=a.wrapping_add((*cfg).reg_mchbar_window_size.wrapping_mul(mc as u64));ioremap(a,(*cfg).reg_mchbar_window_size) }
unsafe fn populate_dimm_info(d:*mut dimm_data,ad:u32,n:usize,c:*mut res_config){(*d).size=field_get((*c).reg_mad_dimm_size_mask[n] as u64,ad as u64)*(*c).reg_mad_dimm_size_granularity;(*d).ranks=(field_get((*c).reg_mad_dimm_rank_mask[n] as u64,ad as u64)+1)as u8;(*d).dtype=(field_get((*c).reg_mad_dimm_width_mask[n] as u64,ad as u64)as i32+DEV_X8)as dev_type;}
unsafe fn ie31200_get_dimm_config(mci:*mut mem_ctl_info,w:*mut u8,c:*mut res_config,_mc:i32){let mut d=core::mem::zeroed::<dimm_data>();for i in 0..IE31200_CHANNELS{let ad=readl(w.add((*c).reg_mad_dimm_offset[i] as usize));for j in 0..IE31200_DIMMS_PER_CHANNEL{populate_dimm_info(&mut d,ad,j,c);let mut pages=MiB_TO_PAGES(d.size>>20);if pages==0{continue;}pages/=d.ranks as u64;for k in 0..d.ranks{let x=edac_get_dimm(mci,(j*d.ranks as usize)+k as usize,i,0);(*x).nr_pages=pages;(*x).grain=8;(*x).mtype=(*c).mtype;(*x).dtype=d.dtype;(*x).edac_mode=EDAC_UNKNOWN;}}}}
unsafe fn ie31200_register_mci(pdev:*mut pci_dev,c:*mut res_config,mc:i32)->i32{nr_channels=how_many_channels(pdev);let w=ie31200_map_mchbar(pdev,c,mc);if w.is_null(){return -19;}let m=edac_mc_alloc(mc,2,core::ptr::null_mut(),core::mem::size_of::<ie31200_priv>());if m.is_null(){return -12;}let p=(*m).pvt_info as *mut ie31200_priv;(*p).window=w;(*p).cfg=c;(*p).mci=m;(*p).pdev=pdev;(*p).c0errlog=w.add((*c).reg_eccerrlog_offset[0]as usize);(*p).c1errlog=w.add((*c).reg_eccerrlog_offset[1]as usize);ie31200_get_dimm_config(m,w,c,mc);ie31200_clear_error_info(m);if edac_mc_add_mc(m)!=0{ iounmap(w);edac_mc_free(m);return -19;}ie31200_pvt.priv_[mc as usize]=p;0}
unsafe fn mce_check(x:*mut mce){for i in 0..IE31200_IMC_NUM{let p=ie31200_pvt.priv_[i];if !p.is_null(){__ie31200_check((*p).mci,x);}}}
unsafe fn ie31200_unregister_mcis(){for i in 0..IE31200_IMC_NUM{let p=ie31200_pvt.priv_[i];if !p.is_null(){edac_mc_del_mc((*p).mci);iounmap((*p).window);edac_mc_free((*p).mci);ie31200_pvt.priv_[i]=core::ptr::null_mut();}}}

static mut snb_cfg:res_config=res_config{mtype:MEM_DDR3,cmci:false,imc_num:1,reg_mchbar_mask:(1u64<<39)- (1u64<<15),reg_mchbar_window_size:1<<15,reg_eccerrlog_offset:[0x40c8,0x44c8],reg_eccerrlog_ce_mask:1,reg_eccerrlog_ce_ovfl_mask:0,reg_eccerrlog_ue_mask:2,reg_eccerrlog_ue_ovfl_mask:0,reg_eccerrlog_rank_mask:0x18000000,reg_eccerrlog_syndrome_mask:0xff0000,msr_clear_eccerrlog_offset:0,reg_mad_dimm_size_granularity:1<<28,reg_mad_dimm_offset:[0x5004,0x5008],reg_mad_dimm_size_mask:[0xff,0xff00],reg_mad_dimm_rank_mask:[1<<17,1<<18],reg_mad_dimm_width_mask:[1<<19,1<<20]};
static mut skl_cfg:res_config=res_config{mtype:MEM_DDR4,cmci:false,imc_num:1,reg_mchbar_mask:(1u64<<39)-(1u64<<15),reg_mchbar_window_size:1<<15,reg_eccerrlog_offset:[0x4048,0x4448],reg_eccerrlog_ce_mask:1,reg_eccerrlog_ce_ovfl_mask:0,reg_eccerrlog_ue_mask:2,reg_eccerrlog_ue_ovfl_mask:0,reg_eccerrlog_rank_mask:0x18000000,reg_eccerrlog_syndrome_mask:0xff0000,msr_clear_eccerrlog_offset:0,reg_mad_dimm_size_granularity:1<<30,reg_mad_dimm_offset:[0x500c,0x5010],reg_mad_dimm_size_mask:[0x3f,0x3f0000],reg_mad_dimm_rank_mask:[1<<10,1<<26],reg_mad_dimm_width_mask:[0x300,0x3000000]};
// Raptor/Alder/Bartlett configurations use DDR5, two IMCs, CMCI, and MSR 0x791.
extern "C" { fn ie31200_init() -> i32; fn ie31200_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
