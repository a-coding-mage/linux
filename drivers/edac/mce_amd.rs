// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of mce_amd.c. Kernel-provided types, constants,
// macros, and functions are intentionally referenced as external dependencies.

static mut fam_ops: amd_decoder_ops = amd_decoder_ops { mc0_mce: None, mc1_mce: None, mc2_mce: None };
static mut xec_mask: u8 = 0xf;
static mut decode_dram_ecc: Option<unsafe extern "C" fn(i32, *mut mce)> = None;

#[repr(C)]
pub struct amd_decoder_ops {
    pub mc0_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
    pub mc1_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
    pub mc2_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
}

extern "C" {
    fn pr_cont(fmt: *const core::ffi::c_char, ...);
    fn pr_emerg(fmt: *const core::ffi::c_char, ...);
    fn pr_warn_once(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn x86_family(cpuid: u32) -> u32;
    fn x86_model(cpuid: u32) -> u32;
    fn x86_stepping(cpuid: u32) -> u32;
    fn topology_amd_node_id(cpu: i32) -> i32;
    fn smca_get_bank_type(cpu: i32, bank: i32) -> smca_bank_types;
    fn rdmsrq_safe(msr: u32, value: *mut u64) -> i32;
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn mce_register_decode_chain(nb: *mut notifier_block);
    fn mce_unregister_decode_chain(nb: *mut notifier_block);
    static mut boot_cpu_data: cpuinfo_x86;
}

#[repr(C)] pub struct cpuinfo_x86 { pub x86_vendor: u32, pub x86: u32, pub x86_model: u32 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32>, pub priority: i32 }
#[repr(C)] pub struct mce { pub status: u64, pub mcgstatus: u64, pub extcpu: i32, pub bank: i32, pub cpuid: u32, pub addr: u64, pub ppin: u64, pub ipid: u64, pub synd: u64, pub tsc: u64, pub kflags: u64 }
#[repr(C)] pub struct mce_hw_err { pub vendor: mce_vendor }
#[repr(C)] pub struct mce_vendor { pub amd: mce_amd_vendor }
#[repr(C)] pub struct mce_amd_vendor { pub synd1: u64, pub synd2: u64 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)] pub enum smca_bank_types { SMCA_CS, SMCA_DACC_BE, SMCA_DACC_FE, SMCA_DE, SMCA_EDDR5CMN, SMCA_EX, SMCA_FP, SMCA_GMI_PCS, SMCA_GMI_PHY, SMCA_IF, SMCA_L2_CACHE, SMCA_L3_CACHE, SMCA_LS, SMCA_MP5, SMCA_MPART, SMCA_MPASP, SMCA_MPDACC, SMCA_MPDMA, SMCA_MPM, SMCA_MPRAS, SMCA_NBIF, SMCA_NBIO, SMCA_PB, SMCA_PCIE, SMCA_PCIE_PL, SMCA_PIE, SMCA_PSP, SMCA_RESERVED, SMCA_SATA, SMCA_SHUB, SMCA_SMU, SMCA_SSBDCI, SMCA_UMC, SMCA_UMC_V2, SMCA_USB, SMCA_WAFL_PHY, SMCA_XGMI_PCS, SMCA_XGMI_PHY, N_SMCA_BANK_TYPES }

extern "C" { fn to_mce_hw_err(m: *mut mce) -> *mut mce_hw_err; }

static tt_msgs: [&[u8]; 4] = [b"INSN\0", b"DATA\0", b"GEN\0", b"RESV\0"];
static ll_msgs: [&[u8]; 4] = [b"RESV\0", b"L1\0", b"L2\0", b"L3/GEN\0"];
static rrrr_msgs: [&[u8]; 9] = [b"GEN\0",b"RD\0",b"WR\0",b"DRD\0",b"DWR\0",b"IRD\0",b"PRF\0",b"EV\0",b"SNP\0"];
#[no_mangle] pub static pp_msgs: [&[u8]; 4] = [b"SRC\0", b"RES\0", b"OBS\0", b"GEN\0"];
static to_msgs: [&[u8]; 2] = [b"no timeout\0", b"timed out\0"];
static ii_msgs: [&[u8]; 4] = [b"MEM\0",b"RESV\0",b"IO\0",b"GEN\0"];
static uu_msgs: [&[u8]; 4] = [b"RESV\0",b"RESV\0",b"HWA\0",b"RESV\0"];

static f15h_mc1_mce_desc: [&[u8]; 18] = [b"UC during a demand linefill from L2\0",b"Parity error during data load from IC\0",b"Parity error for IC valid bit\0",b"Main tag parity error\0",b"Parity error in prediction queue\0",b"PFB data/address parity error\0",b"Parity error in the branch status reg\0",b"PFB promotion address error\0",b"Tag error during probe/victimization\0",b"Parity error for IC probe tag valid bit\0",b"PFB non-cacheable bit parity error\0",b"PFB valid bit parity error\0",b"Microcode Patch Buffer\0",b"uop queue\0",b"insn buffer\0",b"predecode buffer\0",b"fetch address FIFO\0",b"dispatch uop queue\0"];
static f15h_mc2_mce_desc: [&[u8]; 14] = [b"Fill ECC error on data fills\0",b"Fill parity error on insn fills\0",b"Prefetcher request FIFO parity error\0",b"PRQ address parity error\0",b"PRQ data parity error\0",b"WCC Tag ECC error\0",b"WCC Data ECC error\0",b"WCB Data parity error\0",b"VB Data ECC or parity error\0",b"L2 Tag ECC error\0",b"Hard L2 Tag ECC error\0",b"Multiple hits on L2 tag\0",b"XAB parity error\0",b"PRB address parity error\0"];
static mc4_mce_desc: [&[u8]; 19] = [b"DRAM ECC error detected on the NB\0",b"CRC error detected on HT link\0",b"Link-defined sync error packets detected on HT link\0",b"HT Master abort\0",b"HT Target abort\0",b"Invalid GART PTE entry during GART table walk\0",b"Unsupported atomic RMW received from an IO link\0",b"Watchdog timeout due to lack of progress\0",b"DRAM ECC error detected on the NB\0",b"SVM DMA Exclusion Vector error\0",b"HT data error detected on link\0",b"Protocol error (link, L3, probe filter)\0",b"NB internal arrays parity error\0",b"DRAM addr/ctl signals parity error\0",b"IO link transmission error\0",b"L3 data cache ECC error\0",b"L3 cache tag error\0",b"L3 LRU parity bits error\0",b"ECC Error in the Probe Filter directory\0"];
static mc5_mce_desc: [&[u8]; 14] = [b"CPU Watchdog timer expire\0",b"Wakeup array dest tag\0",b"AG payload array\0",b"EX payload array\0",b"IDRF array\0",b"Retire dispatch queue\0",b"Mapper checkpoint array\0",b"Physical register file EX0 port\0",b"Physical register file EX1 port\0",b"Physical register file AG0 port\0",b"Physical register file AG1 port\0",b"Flag register file\0",b"DE error occurred\0",b"Retire status queue\0"];
static mc6_mce_desc: [&[u8]; 6] = [b"Hardware Assertion\0",b"Free List\0",b"Physical Register File\0",b"Retire Queue\0",b"Scheduler table\0",b"Status Register File\0"];

unsafe extern "C" fn f12h_mc0_mce(ec:u16,_xec:u8)->bool{if MEM_ERROR(ec){let ll=LL(ec);if ll==LL_L2{pr_cont(b"during L1 linefill from L2.\n\0".as_ptr() as _);true}else if ll==LL_L1{pr_cont(b"Data/Tag %s error.\n\0".as_ptr() as _,R4_MSG(ec));true}else{false}}else{false}}

// The remaining functions retain the C decoder's control flow and invoke the
// kernel MCA macros and logging primitives supplied by the surrounding build.
unsafe extern "C" fn f10h_mc0_mce(ec: u16, xec: u8) -> bool { if R4(ec)==R4_GEN && LL(ec)==LL_L1 { pr_cont(b"during data scrub.\0".as_ptr() as _); return true } f12h_mc0_mce(ec,xec) }
unsafe extern "C" fn k8_mc0_mce(ec:u16,xec:u8)->bool { if BUS_ERROR(ec) { pr_cont(b"during system linefill.\0".as_ptr() as _); true } else { f10h_mc0_mce(ec,xec) } }
unsafe extern "C" fn cat_mc0_mce(ec:u16,_xec:u8)->bool { let r4=R4(ec); if MEM_ERROR(ec) { if TT(ec)!=TT_DATA || LL(ec)!=LL_L1{return false}; match r4 { R4_DRD|R4_DWR=>pr_cont(b"Data/Tag parity error due to %s.\0".as_ptr() as _, if r4==R4_DRD {b"load/hw prf\0".as_ptr()}else{b"store\0".as_ptr()}), R4_EVICT=>pr_cont(b"Copyback parity error on a tag miss.\0".as_ptr() as _), R4_SNOOP=>pr_cont(b"Tag parity error during snoop.\0".as_ptr() as _), _=>return false}; true } else if BUS_ERROR(ec) { if (II(ec)!=II_MEM&&II(ec)!=II_IO)||LL(ec)!=LL_LG{return false}; pr_cont(b"System read data error on a \0".as_ptr() as _); match r4 {R4_RD=>pr_cont(b"TLB reload.\0".as_ptr() as _),R4_DWR=>pr_cont(b"store.\0".as_ptr() as _),R4_DRD=>pr_cont(b"load.\0".as_ptr() as _),_=>return false}; true } else {false} }
unsafe extern "C" fn f15h_mc0_mce(ec:u16,xec:u8)->bool { if MEM_ERROR(ec) { match xec {0=>pr_cont(b"Data Array access error.\0".as_ptr() as _),1=>pr_cont(b"UC error during a linefill from L2/NB.\0".as_ptr() as _),2|0x11=>pr_cont(b"STQ access error.\0".as_ptr() as _),3=>pr_cont(b"SCB access error.\0".as_ptr() as _),0x10=>pr_cont(b"Tag error.\0".as_ptr() as _),0x12=>pr_cont(b"LDQ access error.\0".as_ptr() as _),_=>return false}; true } else if BUS_ERROR(ec) { if xec==0 {pr_cont(b"System Read Data Error.\0".as_ptr() as _)} else {pr_cont(b" Internal error condition type %d.\0".as_ptr() as _,xec)}; true } else if INT_ERROR(ec) {if xec<=0x1f {pr_cont(b"Hardware Assert.\0".as_ptr() as _);true}else{false}} else {false} }

unsafe extern "C" fn decode_mc0_mce(m:*mut mce){let ec=EC((*m).status);let xec=XEC((*m).status,xec_mask);pr_emerg(b"MC0 Error: \0".as_ptr() as _);if TLB_ERROR(ec)&&TT(ec)==TT_DATA{pr_cont(b"%s TLB %s.\0".as_ptr() as _,LL_MSG(ec),if xec==2{b"locked miss\0".as_ptr()}else if xec!=0{b"multimatch\0".as_ptr()}else{b"parity\0".as_ptr()});return} if let Some(f)=fam_ops.mc0_mce {if !f(ec,xec){pr_emerg(b"Corrupted MC0 MCE info?\n\0".as_ptr() as _)}}else{pr_emerg(b"Corrupted MC0 MCE info?\n\0".as_ptr() as _)} }

unsafe extern "C" fn k8_mc1_mce(ec:u16,_xec:u8)->bool{if !MEM_ERROR(ec){return false};if LL(ec)==2{pr_cont(b"during a linefill from L2.\0".as_ptr() as _);true}else if LL(ec)==1{match R4(ec){R4_IRD=>pr_cont(b"Parity error during data load.\0".as_ptr() as _),R4_EVICT=>pr_cont(b"Copyback Parity/Victim error.\0".as_ptr() as _),R4_SNOOP=>pr_cont(b"Tag Snoop error.\0".as_ptr() as _),_=>return false};true}else{false}}
unsafe extern "C" fn cat_mc1_mce(ec:u16,xec:u8)->bool{if !MEM_ERROR(ec)||TT(ec)!=TT_INSTR{return false};if R4(ec)==R4_IRD{pr_cont(b"Data/tag array parity error for a tag hit.\0".as_ptr() as _)}else if R4(ec)==R4_SNOOP{pr_cont(b"Tag error during snoop/victimization.\0".as_ptr() as _)}else if xec==0{pr_cont(b"Tag parity error from victim castout.\0".as_ptr() as _)}else if xec==2{pr_cont(b"Microcode patch RAM parity error.\0".as_ptr() as _)}else{return false};true}
unsafe extern "C" fn f15h_mc1_mce(ec:u16,xec:u8)->bool{if !MEM_ERROR(ec){return false};let i=if xec<=0xa{xec as usize}else if xec==0xd{11}else if xec==0x10{12}else if xec>=0x11&&xec<=0x15{(xec-4)as usize}else{return false};if xec>=0x11&&xec<=0x15{pr_cont(b"Decoder %s parity error.\n\0".as_ptr() as _,f15h_mc1_mce_desc[i].as_ptr())}else{pr_cont(b"%s.\n\0".as_ptr() as _,f15h_mc1_mce_desc[i].as_ptr())};true}

unsafe extern "C" fn decode_mc1_mce(m:*mut mce){let ec=EC((*m).status);let xec=XEC((*m).status,xec_mask);pr_emerg(b"MC1 Error: \0".as_ptr() as _);if TLB_ERROR(ec){pr_cont(b"%s TLB %s.\n\0".as_ptr() as _,LL_MSG(ec),if xec!=0{b"multimatch\0".as_ptr()}else{b"parity error\0".as_ptr()})}else if BUS_ERROR(ec){pr_cont(b"during %s.\n\0".as_ptr() as _,if boot_cpu_data.x86==0xf&&((*m).status&(1u64<<58))!=0{b"system linefill\0".as_ptr()}else{b"NB data read\0".as_ptr()})}else if INT_ERROR(ec){if xec<=0x3f{pr_cont(b"Hardware Assert.\n\0".as_ptr() as _)}else{pr_emerg(b"Corrupted MC1 MCE info?\n\0".as_ptr() as _);return}}else if let Some(f)=fam_ops.mc1_mce{if !f(ec,xec){pr_emerg(b"Corrupted MC1 MCE info?\n\0".as_ptr() as _)}}else{pr_emerg(b"Corrupted MC1 MCE info?\n\0".as_ptr() as _)}}

unsafe extern "C" fn decode_mc2_mce(m:*mut mce){let ec=EC((*m).status);let xec=XEC((*m).status,xec_mask);pr_emerg(b"MC2 Error: \0".as_ptr() as _);if let Some(f)=fam_ops.mc2_mce{if !f(ec,xec){pr_cont(b"Corrupted MC2 MCE info?\n\0".as_ptr() as _)}}}
unsafe extern "C" fn k8_mc2_mce(ec:u16,xec:u8)->bool{if xec==1{pr_cont(b" in the write data buffers.\n\0".as_ptr() as _)}else if xec==3{pr_cont(b" in the victim data buffers.\n\0".as_ptr() as _)}else if xec==2&&MEM_ERROR(ec){pr_cont(b": %s error in the L2 cache tags.\n\0".as_ptr() as _,R4_MSG(ec))}else if xec==0{if TLB_ERROR(ec){pr_cont(b"%s error in a Page Descriptor Cache or Guest TLB.\n\0".as_ptr() as _,TT_MSG(ec))}else if BUS_ERROR(ec){pr_cont(b": %s/ECC error in data read from NB: %s.\n\0".as_ptr() as _,R4_MSG(ec),PP_MSG(ec))}else if MEM_ERROR(ec){if R4(ec)>=7{pr_cont(b": %s error during data copyback.\n\0".as_ptr() as _,R4_MSG(ec))}else if R4(ec)<=1{pr_cont(b": %s parity/ECC error during data access from L2.\n\0".as_ptr() as _,R4_MSG(ec))}else{return false}}else{return false}}else{return false};true}
unsafe extern "C" fn f15h_mc2_mce(ec:u16,xec:u8)->bool{if TLB_ERROR(ec){if xec==0{pr_cont(b"Data parity TLB read error.\n\0".as_ptr() as _)}else if xec==1{pr_cont(b"Poison data provided for TLB fill.\n\0".as_ptr() as _)}else{return false}}else if BUS_ERROR(ec){if xec>2{return false}pr_cont(b"Error during attempted NB data read.\n\0".as_ptr() as _)}else if MEM_ERROR(ec){let i=if xec>=4&&xec<=0xc{(xec-4)as usize}else if xec>=0x10&&xec<=0x14{(xec-7)as usize}else{return false};pr_cont(b"%s.\n\0".as_ptr() as _,f15h_mc2_mce_desc[i].as_ptr())}else if INT_ERROR(ec){if xec<=0x3f{pr_cont(b"Hardware Assert.\n\0".as_ptr() as _)}else{return false}};true}
unsafe extern "C" fn f16h_mc2_mce(ec:u16,xec:u8)->bool{if !MEM_ERROR(ec){return false};let r4=R4(ec);match xec{4|5=>pr_cont(b"%cBUFF parity error.\n\0".as_ptr() as _,if r4==R4_RD{b'I'}else{b'O'}),9..=11|13..=15=>pr_cont(b"ECC error in L2 tag (%s).\n\0".as_ptr() as _,if r4==R4_GEN{b"BankReq\0".as_ptr()}else if r4==R4_SNOOP{b"Prb\0".as_ptr()}else{b"Fill\0".as_ptr()}),0x10..=0x19|0x1b=>pr_cont(b"ECC error in L2 data array (%s).\n\0".as_ptr() as _,if r4==R4_RD&&xec&3==0{b"Hit\0".as_ptr()}else if r4==R4_GEN{b"Attr\0".as_ptr()}else if r4==R4_EVICT{b"Vict\0".as_ptr()}else{b"Fill\0".as_ptr()}),0x1c|0x1d|0x1f=>pr_cont(b"Parity error in L2 attribute bits (%s).\n\0".as_ptr() as _,if r4==R4_RD{b"Hit\0".as_ptr()}else if r4==R4_GEN{b"Attr\0".as_ptr()}else{b"Fill\0".as_ptr()}),_=>return false};true}
unsafe extern "C" fn decode_mc4_mce(m:*mut mce){let fam=x86_family((*m).cpuid);let n=topology_amd_node_id((*m).extcpu);let ec=EC((*m).status);let x=XEC((*m).status,0x1f);pr_emerg(b"MC4 Error (node %d): \0".as_ptr() as _,n);if x<=0xe{if (x==0||x==8)&&fam!=0x11{pr_cont(b"%s.\n\0".as_ptr() as _,mc4_mce_desc[x as usize].as_ptr());if let Some(f)=decode_dram_ecc{f(n,m)};return}}else if x==0xf{if TLB_ERROR(ec){pr_cont(b"GART Table Walk data error.\n\0".as_ptr() as _)}else if BUS_ERROR(ec){pr_cont(b"DMA Exclusion Vector Table Walk error.\n\0".as_ptr() as _)}else{pr_emerg(b"Corrupted MC4 MCE info?\n\0".as_ptr() as _)};return}else if x==0x19&&(fam==0x15||fam==0x16){pr_cont(b"Compute Unit Data Error.\n\0".as_ptr() as _);return}else if x>=0x1c&&x<=0x1f{pr_cont(b"%s.\n\0".as_ptr() as _,mc4_mce_desc[(x-13)as usize].as_ptr());return}pr_emerg(b"Corrupted MC4 MCE info?\n\0".as_ptr() as _)}
unsafe extern "C" fn decode_mc5_mce(m:*mut mce){let fam=x86_family((*m).cpuid);let ec=EC((*m).status);let x=XEC((*m).status,xec_mask);if fam==0xf||fam==0x11{pr_emerg(b"Corrupted MC5 MCE info?\n\0".as_ptr() as _);return}pr_emerg(b"MC5 Error: \0".as_ptr() as _);if INT_ERROR(ec)&&x<=0x1f{pr_cont(b"Hardware Assert.\n\0".as_ptr() as _);return}if x==0||x==0xc{pr_cont(b"%s.\n\0".as_ptr() as _,mc5_mce_desc[x as usize].as_ptr())}else if x<=0xd{pr_cont(b"%s parity error.\n\0".as_ptr() as _,mc5_mce_desc[x as usize].as_ptr())}else{pr_emerg(b"Corrupted MC5 MCE info?\n\0".as_ptr() as _)}}
unsafe extern "C" fn decode_mc6_mce(m:*mut mce){let x=XEC((*m).status,xec_mask);pr_emerg(b"MC6 Error: \0".as_ptr() as _);if x<=5{pr_cont(b"%s parity error.\n\0".as_ptr() as _,mc6_mce_desc[x as usize].as_ptr())}else{pr_emerg(b"Corrupted MC6 MCE info?\n\0".as_ptr() as _)} }

unsafe extern "C" fn decode_smca_error(m:*mut mce){let bank=smca_get_bank_type((*m).extcpu,(*m).bank);let x=XEC((*m).status,xec_mask);if bank==SMCA_RESERVED{pr_emerg(b"Bank %d is reserved.\n\0".as_ptr() as _,(*m).bank);return}if bank>=N_SMCA_BANK_TYPES{return}if (bank==SMCA_UMC||bank==SMCA_UMC_V2)&&x==0{if let Some(f)=decode_dram_ecc{f(topology_amd_node_id((*m).extcpu),m)}}}
unsafe extern "C" fn amd_decode_mce(_nb:*mut notifier_block,_val:usize,data:*mut core::ffi::c_void)->i32{let m=data as *mut mce;if (*m).kflags&MCE_HANDLED_CEC!=0{return NOTIFY_DONE} ;pr_emerg(b"%s\n\0".as_ptr() as _,decode_error_status(m));if boot_cpu_has(X86_FEATURE_SMCA){decode_smca_error(m)}else if fam_ops.mc0_mce.is_some(){match (*m).bank{0=>decode_mc0_mce(m),1=>decode_mc1_mce(m),2=>decode_mc2_mce(m),3=>decode_mc3_mce(m),4=>decode_mc4_mce(m),5=>decode_mc5_mce(m),6=>decode_mc6_mce(m),_=>{}}}amd_decode_err_code(EC((*m).status));(*m).kflags|=MCE_HANDLED_EDAC;NOTIFY_OK}
static mut amd_mce_dec_nb:notifier_block=notifier_block{notifier_call:Some(amd_decode_mce),priority:MCE_PRIO_EDAC};
#[no_mangle] pub unsafe extern "C" fn amd_register_ecc_decoder(f:Option<unsafe extern "C" fn(i32,*mut mce)>){decode_dram_ecc=f}
#[no_mangle] pub unsafe extern "C" fn amd_unregister_ecc_decoder(f:Option<unsafe extern "C" fn(i32,*mut mce)>){if decode_dram_ecc.is_some(){if decode_dram_ecc!=f{ }decode_dram_ecc=None}}
unsafe extern "C" fn decode_mc3_mce(m:*mut mce){let ec=EC((*m).status);let xec=XEC((*m).status,xec_mask);if boot_cpu_data.x86>=0x14{pr_emerg(b"You shouldn't be seeing MC3 MCE on this cpu family, please report on LKML.\n\0".as_ptr() as _);return}pr_emerg(b"MC3 Error\0".as_ptr() as _);if xec==0&&BUS_ERROR(ec)&&(R4(ec)==R4_DRD||R4(ec)==R4_DWR){pr_cont(b" during %s.\n\0".as_ptr() as _,R4_MSG(ec))}else{pr_emerg(b"Corrupted MC3 MCE info?\n\0".as_ptr() as _)} }

unsafe extern "C" fn amd_decode_err_code(ec:u16){if INT_ERROR(ec){pr_emerg(b"internal: %s\n\0".as_ptr() as _,UU_MSG(ec));return}pr_emerg(b"cache level: %s\0".as_ptr() as _,LL_MSG(ec));if BUS_ERROR(ec){pr_cont(b", mem/io: %s\0".as_ptr() as _,II_MSG(ec))}else{pr_cont(b", tx: %s\0".as_ptr() as _,TT_MSG(ec))}if MEM_ERROR(ec)||BUS_ERROR(ec){pr_cont(b", mem-tx: %s\0".as_ptr() as _,R4_MSG(ec));if BUS_ERROR(ec){pr_cont(b", part-proc: %s (%s)\0".as_ptr() as _,PP_MSG(ec),TO_MSG(ec))}}pr_cont(b"\n\0".as_ptr() as _)}

unsafe extern "C" fn decode_error_status(m:*mut mce)->*const u8{if (*m).status&MCI_STATUS_UC!=0{if (*m).status&MCI_STATUS_PCC!=0{b"System Fatal error.\0".as_ptr()}else if (*m).mcgstatus&MCG_STATUS_RIPV!=0{b"Uncorrected, software restartable error.\0".as_ptr()}else{b"Uncorrected, software containable error.\0".as_ptr()}}else if (*m).status&MCI_STATUS_DEFERRED!=0{b"Deferred error, no action required.\0".as_ptr()}else{b"Corrected error, no action required.\0".as_ptr()}}

// External MCA bitfield helpers and constants are supplied by the kernel port.
extern "C" { fn EC(x:u64)->u16; fn XEC(x:u64,m:u8)->u8; fn R4(x:u16)->u8; fn LL(x:u16)->u8; fn TT(x:u16)->u8; fn II(x:u16)->u8; fn R4_MSG(x:u16)->*const u8; fn LL_MSG(x:u16)->*const u8; fn TT_MSG(x:u16)->*const u8; fn II_MSG(x:u16)->*const u8; fn PP_MSG(x:u16)->*const u8; fn TO_MSG(x:u16)->*const u8; fn UU_MSG(x:u16)->*const u8; fn MEM_ERROR(x:u16)->bool; fn BUS_ERROR(x:u16)->bool; fn TLB_ERROR(x:u16)->bool; fn INT_ERROR(x:u16)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
