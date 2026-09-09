// SPDX-License-Identifier: GPL-2.0
/* EDAC driver for Intel(R) Xeon(R) Skylake processors */

// Dependencies supplied by the surrounding kernel/EDAC translation unit.
use core::ffi::{c_char, c_int, c_void};

const EDAC_MOD_STR: &[u8] = b"skx_edac\0";
const MASK26: u64 = 0x3ffffff;
const MASK29: u64 = 0x1fffffff;
const SKX_MAX_SAD: usize = 24;
const SKX_MAX_TAD: usize = 8;
const SKX_MAX_RIR: usize = 4;

static mut skx_edac_list: *mut list_head = core::ptr::null_mut();
static mut skx_tolm: u64 = 0;
static mut skx_tohm: u64 = 0;
static mut skx_num_sockets: c_int = 0;
static mut nvdimm_count: u32 = 0;

#[repr(C)]
struct munit { did: u16, devfn: [u16; 2], busidx: u8, per_socket: u8, mtype: munittype }
#[repr(C)]
#[derive(Copy, Clone)]
enum munittype { CHAN0, CHAN1, CHAN2, SAD_ALL, UTIL_ALL, SAD, ERRCHAN0, ERRCHAN1, ERRCHAN2 }

static mut skx_cfg: res_config = res_config { r#type: SKX, decs_did: 0x2016, busno_cfg_offset: 0xcc, ddr_imc_num: 2, ddr_chan_num: 3, ddr_dimm_num: 2 };
static skx_granularity: [c_int; 4] = [6, 8, 12, 30];
static skx_close_row: [u8; 18] = [15,16,17,18,20,21,22,28,10,11,12,13,29,30,31,32,33,34];
static skx_close_column: [u8; 10] = [3,4,5,14,19,23,24,25,26,27];
static skx_open_row: [u8; 18] = [14,15,16,20,28,21,22,23,24,25,26,27,29,30,31,32,33,34];
static skx_open_column: [u8; 10] = [3,4,5,6,7,8,9,10,11,12];
static skx_open_fine_column: [u8; 10] = [3,4,5,7,8,9,10,11,12,13];

#[inline] unsafe fn get_skx_dev(bus: *mut pci_bus, idx: u8) -> *mut skx_dev {
    let mut d: *mut skx_dev = core::ptr::null_mut();
    list_for_each_entry(&mut d, skx_edac_list);
    while !d.is_null() { if (*d).seg == pci_domain_nr(bus) && (*d).bus[idx as usize] == (*bus).number { return d; } d = (*d).next; }
    core::ptr::null_mut()
}

unsafe fn get_all_munits(m: *const munit) -> c_int {
    let mut prev: *mut pci_dev = core::ptr::null_mut(); let mut ndev = 0; let mut i = 0;
    loop {
        let pdev = pci_get_device(PCI_VENDOR_ID_INTEL, (*m).did, prev); if pdev.is_null() { break; } ndev += 1;
        let d = get_skx_dev((*pdev).bus, (*m).busidx); if d.is_null() { pci_dev_put(pdev); return -ENODEV; }
        if (*m).per_socket == skx_cfg.ddr_imc_num { i = 0; while i < skx_cfg.ddr_imc_num as usize && (*m).devfn[i] != (*pdev).devfn { i += 1; } if i == skx_cfg.ddr_imc_num as usize { pci_dev_put(pdev); return -ENODEV; } }
        if pci_enable_device(pdev) < 0 { pci_dev_put(pdev); return -ENODEV; }
        match (*m).mtype {
            munittype::CHAN0 | munittype::CHAN1 | munittype::CHAN2 => { pci_dev_get(pdev); (*d).imc[i].chan[(*m).mtype as usize].cdev = pdev; }
            munittype::ERRCHAN0 | munittype::ERRCHAN1 | munittype::ERRCHAN2 => { pci_dev_get(pdev); (*d).imc[i].chan[(*m).mtype as usize - 6].edev = pdev; }
            munittype::SAD_ALL => { pci_dev_get(pdev); (*d).sad_all = pdev; }
            munittype::UTIL_ALL => { pci_dev_get(pdev); (*d).util_all = pdev; }
            munittype::SAD => { let mut reg=0; pci_read_config_dword(pdev,0xb4,&mut reg); if reg != 0 { if (*d).mcroute == 0 { (*d).mcroute=reg; } else if (*d).mcroute != reg { pci_dev_put(pdev); return -ENODEV; } } ndev -= 1; }
        }
        prev = pdev;
    } ndev
}

unsafe fn skx_check_ecc(mcmtr: u32) -> bool { ((mcmtr >> 2) & 1) != 0 }

unsafe fn skx_get_dimm_config(mci: *mut mem_ctl_info, cfg: *mut res_config) -> c_int {
    let pvt = (*mci).pvt_info as *mut skx_pvt; let imc = (*pvt).imc; let mut mcmtr=0;
    pci_read_config_dword((*imc).chan[0].cdev,0x87c,&mut mcmtr);
    for i in 0..(*cfg).ddr_chan_num { let mut ndimms=0; let (mut amap,mut mcddrtcfg)=(0,0); pci_read_config_dword((*imc).chan[i as usize].cdev,0x8c,&mut amap); pci_read_config_dword((*imc).chan[i as usize].cdev,0x400,&mut mcddrtcfg);
        for j in 0..(*cfg).ddr_dimm_num { let dimm=edac_get_dimm(mci,i,j,0); let mut mtr=0; pci_read_config_dword((*imc).chan[i as usize].cdev,0x80+4*j,&mut mtr); if IS_DIMM_PRESENT(mtr) { ndimms += skx_get_dimm_info(mtr,mcmtr,amap,dimm,imc,i,j,cfg); } else if IS_NVDIMM_PRESENT(mcddrtcfg,j) { ndimms += skx_get_nvdimm_info(dimm,imc,i,j,EDAC_MOD_STR.as_ptr() as *const c_char); nvdimm_count+=1; } }
        if ndimms != 0 && !skx_check_ecc(mcmtr) { return -ENODEV; }
    } 0
}

unsafe fn skx_do_interleave(mut addr:u64, shift:c_int, ways:u64, lowbits:u64)->u64 { addr >>= shift; addr /= ways; addr <<= shift; addr | (lowbits & ((1u64<<shift)-1)) }
unsafe fn skx_bits(addr:u64, nbits:c_int, bits:*const u8)->c_int { let mut r=0; for i in 0..nbits { r |= (((addr >> *bits.add(i as usize)) & 1) as c_int) << i; } r }
unsafe fn skx_bank_bits(addr:u64,b0:c_int,b1:c_int,xor_:c_int,x0:c_int,x1:c_int)->c_int { let mut r=((addr>>b0)&1) as c_int | ((((addr>>b1)&1) as c_int)<<1); if xor_!=0 { r ^= ((addr>>x0)&1) as c_int | ((((addr>>x1)&1) as c_int)<<1); } r }

// The following declarations and routines retain the kernel ABI and control flow.
// Their types and helpers are supplied by the surrounding translation unit.
extern "C" { fn skx_mce_check_error(_: *mut notifier_block, _: c_ulong, _: *mut c_void)->c_int; }

unsafe fn skx_mad_decode(r:*mut decoded_addr)->bool { let dimm=&(*(*(*r).dev).imc[(*r).imc].chan[(*r).channel].dimms[(*r).dimm]; let bg0=if dimm.fine_grain_bank!=0{6}else{13}; if dimm.close_pg!=0 { (*r).row=skx_bits((*r).rank_address,dimm.rowbits,skx_close_row.as_ptr()); (*r).column=skx_bits((*r).rank_address,dimm.colbits,skx_close_column.as_ptr())|0x400; (*r).bank_address=skx_bank_bits((*r).rank_address,8,9,dimm.bank_xor_enable,22,28); (*r).bank_group=skx_bank_bits((*r).rank_address,6,7,dimm.bank_xor_enable,20,21); } else { (*r).row=skx_bits((*r).rank_address,dimm.rowbits,skx_open_row.as_ptr()); (*r).column=skx_bits((*r).rank_address,dimm.colbits,if dimm.fine_grain_bank!=0{skx_open_fine_column.as_ptr()}else{skx_open_column.as_ptr()}); (*r).bank_address=skx_bank_bits((*r).rank_address,18,19,dimm.bank_xor_enable,22,23); (*r).bank_group=skx_bank_bits((*r).rank_address,bg0,17,dimm.bank_xor_enable,20,21); } (*r).row &= (1u32<<dimm.rowbits)-1; true }

unsafe fn skx_sad_decode(res:*mut decoded_addr)->bool {
    let mut d=list_first_entry(skx_edac_list); let addr=(*res).addr; if addr>=skx_tohm || (addr>=skx_tolm && addr<(1u64<<32)){return false}; let mut prev=0; let (mut sad,mut ilv)=(0,0); let mut i=0; let mut remote=0;
    'restart: loop { prev=0; for n in 0..SKX_MAX_SAD { pci_read_config_dword((*d).sad_all,0x60+8*n,&mut sad); let limit=(((sad>>7)&0xfffff) as u64<<26)|MASK26; if (sad&1)!=0 && addr>=prev && addr<=limit { i=n; break; } prev=limit+1; if n==SKX_MAX_SAD-1{return false;} } pci_read_config_dword((*d).sad_all,0x64+8*i,&mut ilv); let mode=(sad>>1)&3; let idx=match mode{0=>((addr>>6)&7) as usize,1=>((addr>>8)&7) as usize,2=>((addr>>12)&7) as usize,_=>((addr>>30)&7) as usize}; let tgt=(ilv>>(4*idx))&15; if tgt&8==0 { if remote!=0{return false} remote=1; list_for_each_entry(&mut d,skx_edac_list); while !d.is_null(){if (*d).imc[0].src_id==tgt as u8{continue 'restart} d=(*d).next;} return false; } let mut lchan=(tgt&7) as c_int; if ((sad>>27)&1)!=0 { let shift=match (sad>>30)&3{0=>6,1=>8,2=>12,_=>return false}; lchan=match (sad>>5)&3{0=>((addr>>shift)%3)as c_int,1=>((addr>>shift)%2)as c_int,2=>{let x=((addr>>shift)%2)as c_int;(x<<1)|(!x&1)},_=>(((addr>>shift)%2)as c_int)<<1}; lchan=(lchan<<1)|((tgt&1)as c_int); } (*res).dev=d; (*res).socket=(*d).imc[0].src_id; (*res).imc=((*d).mcroute>>(lchan*3)&7)as c_int; (*res).channel=((*d).mcroute>>(lchan*2+18)&3)as c_int; return true; }
}

unsafe fn skx_tad_decode(res:*mut decoded_addr)->bool { let mut base=0; let mut way=0; let mut off=0; let mut i=0; for n in 0..SKX_MAX_TAD { i=n; pci_read_config_dword((*(*res).dev).imc[(*res).imc].chan[0].cdev,0x850+4*n,&mut base); pci_read_config_dword((*(*res).dev).imc[(*res).imc].chan[0].cdev,0x880+4*n,&mut way); let b=((base>>12)&0xfffff)as u64<<26; let l=((way>>12)&0xfffff)as u64<<26|MASK26; if b<=(*res).addr&&(*res).addr<=l{break} if n==SKX_MAX_TAD-1{return false;} } (*res).sktways=1<<((way>>10)&3); (*res).chanways=((way>>8)&3)+1; let s=skx_granularity[((base>>4)&3)as usize]; let c=skx_granularity[((base>>6)&3)as usize]; pci_read_config_dword((*(*res).dev).imc[(*res).imc].chan[(*res).channel].cdev,0x90+4*i,&mut off); let mut a=(*res).addr-(((off>>4)&0xfffff)as u64<<26); if (*res).chanways==3&&s>c {a=skx_do_interleave(a,c,(*res).chanways as u64,a);a=skx_do_interleave(a,s,(*res).sktways as u64,a)}else{a=skx_do_interleave(a,s,(*res).sktways as u64,(*res).addr);a=skx_do_interleave(a,c,(*res).chanways as u64,(*res).addr)} (*res).chan_addr=a; true }

unsafe fn skx_rir_decode(res:*mut decoded_addr)->bool { let shift=if (*(*(*res).dev).imc[(*res).imc].chan[(*res).channel].dimms[0]).close_pg!=0{6}else{13}; let mut w=0; let mut i=0; for n in 0..SKX_MAX_RIR {i=n;pci_read_config_dword((*(*res).dev).imc[(*res).imc].chan[(*res).channel].cdev,0x108+4*n,&mut w);if (w>>31)!=0&&(*res).chan_addr<=(((w>>1)&0x7ff)as u64<<29|MASK29){break}if n==SKX_MAX_RIR-1{return false}} let ways=1<<((w>>28)&3);let mut a=((*res).chan_addr>>shift)/ways<<shift|((*res).chan_addr&((1u64<<shift)-1));let idx=(((*res).chan_addr>>shift)%ways)as usize;let mut lv=0;pci_read_config_dword((*(*res).dev).imc[(*res).imc].chan[(*res).channel].cdev,0x120+16*idx+4*i,&mut lv);a-=((lv>>2)&0x3fff)as u64<<26;(*res).rank_address=a;(*res).channel_rank=((lv>>16)&15)as c_int;(*res).dimm=(*res).channel_rank/4;(*res).rank=(*res).channel_rank%4;true}

unsafe fn skx_init()->c_int { if ghes_get_devices()!=0{return -EBUSY}; if cpu_feature_enabled(X86_FEATURE_HYPERVISOR)!=0{return -ENODEV}; let mut off=[0xd0,0xd4,0xd8]; let rc=skx_get_hi_lo(0x2034,off.as_mut_ptr(),&mut skx_tolm,&mut skx_tohm); if rc!=0{return rc}; skx_set_res_cfg(&mut skx_cfg); rc }
unsafe fn skx_exit(){skx_remove();}
unsafe fn skx_decode(res:*mut decoded_addr)->bool{skx_sad_decode(res)&&skx_tad_decode(res)&&skx_rir_decode(res)&&skx_mad_decode(res)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
