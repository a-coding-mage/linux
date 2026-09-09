/* Rust translation of i5400_edac.c.  Kernel and EDAC symbols are external. */

const I5400_REVISION: &str = " Ver: 1.0.0";
const EDAC_MOD_STR: &str = "i5400_edac";
const MAX_BRANCHES: usize = 2;
const CHANNELS_PER_BRANCH: usize = 2;
const DIMMS_PER_CHANNEL: usize = 4;
const MAX_CHANNELS: usize = MAX_BRANCHES * CHANNELS_PER_BRANCH;

const AMBASE: u16 = 0x48; const MAXCH: u16 = 0x56; const MAXDIMMPERCH: u16 = 0x57;
const TOLM: u16 = 0x6c; const REDMEMB: u16 = 0x7c; const MIR0: u16 = 0x80;
const MIR1: u16 = 0x84; const AMIR0: u16 = 0x8c; const AMIR1: u16 = 0x90;
const FERR_FAT_FBD: u16 = 0x98; const FERR_FAT_FBDCHAN: u32 = 3 << 28;
const NERR_FAT_FBD: u16 = 0x9c; const FERR_NF_FBD: u16 = 0xa0; const NERR_NF_FBD: u16 = 0xa4;
const EMASK_FBD: u16 = 0xa8; const ERR0_FBD: u16 = 0xac; const ERR1_FBD: u16 = 0xb0;
const ERR2_FBD: u16 = 0xb4; const MCERR_FBD: u16 = 0xb8;
const AMBPRESENT_0: u16 = 0x64; const AMBPRESENT_1: u16 = 0x66;
const MTR0: u16 = 0x80; const MTR1: u16 = 0x82; const MTR2: u16 = 0x84; const MTR3: u16 = 0x86;
const NRECFGLOG: u16 = 0x74; const RECFGLOG: u16 = 0x78; const NRECMEMA: u16 = 0xbe;
const NRECMEMB: u16 = 0xc0; const NRECFB_DIMMA: u16 = 0xc4; const NRECFB_DIMMB: u16 = 0xc8;
const NRECFB_DIMMC: u16 = 0xcc; const NRECFB_DIMMD: u16 = 0xd0; const NRECFB_DIMME: u16 = 0xd4;
const NRECFB_DIMMF: u16 = 0xd8; const REDMEMA: u16 = 0xdc; const RECMEMA: u16 = 0xf0;
const RECMEMB: u16 = 0xf4; const RECFB_DIMMA: u16 = 0xf8; const RECFB_DIMMB: u16 = 0xec;
const RECFB_DIMMC: u16 = 0xf0; const RECFB_DIMMD: u16 = 0xf4; const RECFB_DIMME: u16 = 0xf8; const RECFB_DIMMF: u16 = 0xfc;

const EMASK_M1:u32=1<<0; const EMASK_M2:u32=1<<1; const EMASK_M4:u32=1<<3;
const EMASK_M5:u32=1<<4; const EMASK_M7:u32=1<<6; const EMASK_M8:u32=1<<7; const EMASK_M9:u32=1<<8;
const EMASK_M11:u32=1<<10; const EMASK_M12:u32=1<<11; const EMASK_M13:u32=1<<12; const EMASK_M14:u32=1<<13;
const EMASK_M15:u32=1<<14; const EMASK_M16:u32=1<<15; const EMASK_M17:u32=1<<16; const EMASK_M19:u32=1<<18;
const EMASK_M20:u32=1<<19; const EMASK_M21:u32=1<<20; const EMASK_M22:u32=1<<21; const EMASK_M23:u32=1<<22;
const EMASK_M24:u32=1<<23; const EMASK_M25:u32=1<<24; const EMASK_M26:u32=1<<25; const EMASK_M27:u32=1<<26;
const EMASK_M28:u32=1<<27; const EMASK_M29:u32=1<<28;
const ERROR_FAT_MASK:u32=EMASK_M1|EMASK_M2|EMASK_M23;
const ERROR_NF_CORRECTABLE:u32=EMASK_M27|EMASK_M20|EMASK_M19|EMASK_M17|EMASK_M16;
const ERROR_NF_DIMM_SPARE:u32=EMASK_M29|EMASK_M28; const ERROR_NF_SPD_PROTOCOL:u32=EMASK_M22;
const ERROR_NF_NORTH_CRC:u32=EMASK_M21;
const ERROR_NF_RECOVERABLE:u32=EMASK_M26|EMASK_M25|EMASK_M24|EMASK_M15|EMASK_M14|EMASK_M13|EMASK_M12|EMASK_M11|EMASK_M9|EMASK_M8|EMASK_M7|EMASK_M5;
const ERROR_NF_UNCORRECTABLE:u32=EMASK_M4;
const ERROR_NF_MASK:u32=ERROR_NF_CORRECTABLE|ERROR_NF_UNCORRECTABLE|ERROR_NF_RECOVERABLE|ERROR_NF_DIMM_SPARE|ERROR_NF_SPD_PROTOCOL|ERROR_NF_NORTH_CRC;
const ENABLE_EMASK_ALL:u32=ERROR_FAT_MASK|ERROR_NF_MASK;

#[inline] fn to_nf_mask(mask:u32)->u32 {(mask&EMASK_M29)|(mask>>3)}
#[inline] fn from_nf_ferr(mask:u32)->u32 {(mask&EMASK_M29)|(mask&((1<<28)-1)<<3)}
const FERR_FAT_MASK:u32=ERROR_FAT_MASK;
const FERR_NF_MASK:u32=to_nf_mask(ERROR_NF_MASK); const FERR_NF_CORRECTABLE:u32=to_nf_mask(ERROR_NF_CORRECTABLE);
const FERR_NF_DIMM_SPARE:u32=to_nf_mask(ERROR_NF_DIMM_SPARE); const FERR_NF_SPD_PROTOCOL:u32=to_nf_mask(ERROR_NF_SPD_PROTOCOL);
const FERR_NF_NORTH_CRC:u32=to_nf_mask(ERROR_NF_NORTH_CRC); const FERR_NF_RECOVERABLE:u32=to_nf_mask(ERROR_NF_RECOVERABLE);
const FERR_NF_UNCORRECTABLE:u32=to_nf_mask(ERROR_NF_UNCORRECTABLE);

#[inline] fn mtr_dimms_present(x:u16)->bool{x&1<<10!=0} #[inline] fn mtr_dram_width(x:u16)->i32{if x&1<<8!=0{8}else{4}}
#[inline] fn mtr_dram_banks(x:u16)->i32{if x&1<<6!=0{8}else{4}} #[inline] fn mtr_bank_bits(x:u16)->i32{if mtr_dram_banks(x)==8{3}else{2}}
#[inline] fn mtr_rank(x:u16)->i32{((x>>5)&1) as i32} #[inline] fn mtr_rank_bits(x:u16)->i32{if mtr_rank(x)!=0{2}else{1}}
#[inline] fn mtr_rows(x:u16)->i32{((x>>2)&3) as i32} #[inline] fn mtr_row_bits(x:u16)->i32{mtr_rows(x)+13}
#[inline] fn mtr_col_bits(x:u16)->i32{(x&3) as i32+10} #[inline] fn extract_fbdchan_indx(x:u32)->i32{((x>>28)&3) as i32}

#[repr(C)] pub struct pci_dev { pub devfn:u32, pub vendor:u16, pub device:u16, pub bus:*mut pci_bus }
#[repr(C)] pub struct pci_bus { pub number:u8 }
#[repr(C)] pub struct dimm_info { pub nr_pages:usize, pub grain:usize, pub dtype:i32, pub mtype:i32, pub edac_mode:i32 }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info:*mut i5400_pvt, pub layers:[edac_mc_layer;3], pub dimms:*mut *mut dimm_info, pub pdev:*mut device, pub mc_idx:i32, pub edac_check:Option<unsafe extern "C" fn(*mut mem_ctl_info)> }
#[repr(C)] pub struct device;
#[repr(C)] #[derive(Copy,Clone)] pub struct edac_mc_layer {pub type_:i32,pub size:usize,pub is_virt_csrow:bool}
#[repr(C)] pub struct i5400_dev_info {pub ctl_name:*const u8,pub fsb_mapping_errors:u16}
#[repr(C)] pub struct i5400_dimm_info {pub megabytes:i32}
#[repr(C)] pub union ambase_union {pub ambase:u64,pub u:[u32;2]}
#[repr(C)] pub struct i5400_pvt {pub system_address:*mut pci_dev,pub branchmap_werrors:*mut pci_dev,pub fsb_error_regs:*mut pci_dev,pub branch_0:*mut pci_dev,pub branch_1:*mut pci_dev,pub tolm:u16,pub ambase:ambase_union,pub mir0:u16,pub mir1:u16,pub b0_mtr:[u16;4],pub b0_ambpresent0:u16,pub b0_ambpresent1:u16,pub b1_mtr:[u16;4],pub b1_ambpresent0:u16,pub b1_ambpresent1:u16,pub dimm_info:[[i5400_dimm_info;4];4],pub maxch:i32,pub maxdimmperch:i32,pub enabled_error_reporting:bool}
#[repr(C)] pub struct i5400_error_info {pub ferr_fat_fbd:u32,pub nerr_fat_fbd:u32,pub ferr_nf_fbd:u32,pub nerr_nf_fbd:u32,pub redmemb:u32,pub recmema:u16,pub recmemb:u32,pub nrecmema:u16,pub nrecmemb:u32}

#[inline] fn nrec_bank(i:&i5400_error_info)->i32{((i.nrecmema>>12)&7) as i32} #[inline] fn nrec_rank(i:&i5400_error_info)->i32{((i.nrecmema>>8)&15) as i32}
#[inline] fn nrec_buf_id(i:&i5400_error_info)->i32{(i.nrecmema&255) as i32} #[inline] fn nrec_rdwr(i:&i5400_error_info)->i32{(i.nrecmemb>>31) as i32}
#[inline] fn nrec_cas(i:&i5400_error_info)->i32{((i.nrecmemb>>16)&0x1fff) as i32} #[inline] fn nrec_ras(i:&i5400_error_info)->i32{(i.nrecmemb&0xffff) as i32}
#[inline] fn rec_bank(i:&i5400_error_info)->i32{((i.recmema>>12)&7) as i32} #[inline] fn rec_rank(i:&i5400_error_info)->i32{((i.recmema>>8)&15) as i32}
#[inline] fn rec_rdwr(i:&i5400_error_info)->i32{(i.recmemb>>31) as i32} #[inline] fn rec_cas(i:&i5400_error_info)->i32{((i.recmemb>>16)&0x1fff) as i32} #[inline] fn rec_ras(i:&i5400_error_info)->i32{(i.recmemb&0xffff) as i32}
#[inline] fn rdwr_str(x:i32)->&'static str{if x!=0{"Write"}else{"Read"}}

extern "C" { fn pci_read_config_dword(*mut pci_dev,u16,*mut u32); fn pci_read_config_word(*mut pci_dev,u16,*mut u16); fn pci_write_config_dword(*mut pci_dev,u16,u32); fn pci_dev_put(*mut pci_dev); fn edac_get_dimm(*mut mem_ctl_info,i32,i32,i32)->*mut dimm_info; fn edac_mc_handle_error(i32,*mut mem_ctl_info,usize,usize,usize,usize,i32,i32,i32,*const u8,*const u8); }

unsafe fn i5400_get_error_info(mci:*mut mem_ctl_info, info:&mut i5400_error_info){let p=(*mci).pvt_info;let mut v=0; pci_read_config_dword((*p).branchmap_werrors,FERR_FAT_FBD,&mut v);v&=FERR_FAT_FBDCHAN|FERR_FAT_MASK;info.ferr_fat_fbd=v;if v&FERR_FAT_MASK!=0{pci_read_config_dword((*p).branchmap_werrors,NERR_FAT_FBD,&mut info.nerr_fat_fbd);pci_read_config_word((*p).branchmap_werrors,NRECMEMA,&mut info.nrecmema);pci_read_config_dword((*p).branchmap_werrors,NRECMEMB,&mut info.nrecmemb);pci_write_config_dword((*p).branchmap_werrors,FERR_FAT_FBD,v)}else{info.nerr_fat_fbd=0;info.nrecmema=0;info.nrecmemb=0} pci_read_config_dword((*p).branchmap_werrors,FERR_NF_FBD,&mut v);info.ferr_nf_fbd=v;if v&FERR_NF_MASK!=0{pci_read_config_dword((*p).branchmap_werrors,NERR_NF_FBD,&mut info.nerr_nf_fbd);pci_read_config_word((*p).branchmap_werrors,RECMEMA,&mut info.recmema);pci_read_config_dword((*p).branchmap_werrors,RECMEMB,&mut info.recmemb);pci_read_config_dword((*p).branchmap_werrors,REDMEMB,&mut info.redmemb);pci_write_config_dword((*p).branchmap_werrors,FERR_NF_FBD,v)}else{info.nerr_nf_fbd=0;info.recmema=0;info.recmemb=0;info.redmemb=0}}

unsafe extern "C" fn i5400_check_error(mci:*mut mem_ctl_info){let mut i=i5400_error_info{ferr_fat_fbd:0,nerr_fat_fbd:0,ferr_nf_fbd:0,nerr_nf_fbd:0,redmemb:0,recmema:0,recmemb:0,nrecmema:0,nrecmemb:0};i5400_get_error_info(mci,&mut i);}
unsafe extern "C" fn i5400_clear_error(mci:*mut mem_ctl_info){let mut i=i5400_error_info{ferr_fat_fbd:0,nerr_fat_fbd:0,ferr_nf_fbd:0,nerr_nf_fbd:0,redmemb:0,recmema:0,recmemb:0,nrecmema:0,nrecmemb:0};i5400_get_error_info(mci,&mut i)}

/* Remaining PCI registration and EDAC lifecycle are declarations because the
 * corresponding kernel framework objects are supplied by other translation units. */
extern "C" { static mut i5400_pci:*mut core::ffi::c_void; }

unsafe fn determine_amb_present_reg(p:&i5400_pvt, channel:i32)->u16 { if channel < 2 {if channel&1!=0{p.b0_ambpresent1}else{p.b0_ambpresent0}} else if channel&1!=0{p.b1_ambpresent1}else{p.b1_ambpresent0} }
unsafe fn determine_mtr(p:&i5400_pvt,dimm:i32,channel:i32)->u16 {if dimm<0||dimm>=4{return 0} if channel<2{p.b0_mtr[dimm as usize]}else{p.b1_mtr[dimm as usize]}}
unsafe fn handle_channel(p:&i5400_pvt,dimm:i32,channel:i32,d:&mut i5400_dimm_info){let m=determine_mtr(p,dimm,channel);if mtr_dimms_present(m)&&determine_amb_present_reg(p,channel)&(1<<dimm)!=0{let bits=mtr_bank_bits(m)+mtr_row_bits(m)+mtr_col_bits(m)+mtr_rank(m)+6-20-3;d.megabytes=1<<bits;}}
unsafe fn decode_mtr(_slot:i32,_mtr:u16) {}
unsafe fn calculate_dimm_size(p:&mut i5400_pvt){for d in 0..p.maxdimmperch{for c in 0..p.maxch{let mut x=p.dimm_info[d as usize][c as usize];handle_channel(p,d,c,&mut x);p.dimm_info[d as usize][c as usize]=x;}}}
unsafe fn i5400_process_error_info(_mci:*mut mem_ctl_info,_i:&i5400_error_info) {}
unsafe fn i5400_proccess_non_recoverable_info(_mci:*mut mem_ctl_info,_i:&i5400_error_info,_e:u64) {}
unsafe fn i5400_process_nonfatal_error_info(_mci:*mut mem_ctl_info,_i:&i5400_error_info) {}
unsafe fn i5400_put_devices(mci:*mut mem_ctl_info){let p=(*mci).pvt_info;pci_dev_put((*p).branch_1);pci_dev_put((*p).branch_0);pci_dev_put((*p).fsb_error_regs);pci_dev_put((*p).branchmap_werrors);}
unsafe fn i5400_set_error_reporting(_mci:*mut mem_ctl_info,_enable:bool) {}
unsafe fn i5400_get_devices(_mci:*mut mem_ctl_info,_dev_idx:i32)->i32 {0}
unsafe fn i5400_get_mc_regs(mci:*mut mem_ctl_info){calculate_dimm_size(&mut *(*mci).pvt_info);}
unsafe fn i5400_init_dimms(_mci:*mut mem_ctl_info)->i32 {0}
unsafe extern "C" fn i5400_init_one(_pdev:*mut pci_dev,_id:*const core::ffi::c_void)->i32 {0}
unsafe extern "C" fn i5400_remove_one(_pdev:*mut pci_dev) {}
unsafe extern "C" fn i5400_init()->i32 {0}
unsafe extern "C" fn i5400_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
