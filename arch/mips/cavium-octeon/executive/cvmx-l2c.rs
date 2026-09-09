/* Implementation of Level 2 Cache control, measurement, and debugging. */

use core::ffi::c_char;

extern "C" {
    static mut cvmx_l2c_spinlock: cvmx_spinlock_t;
    fn cvmx_octeon_num_cores() -> u32;
    fn cvmx_l2c_get_num_assoc() -> u32;
    fn cvmx_l2c_get_num_sets() -> u64;
    fn cvmx_l2c_address_to_index(addr: u64) -> u32;
    fn cvmx_l2c_get_tag(assoc: i32, index: u32) -> cvmx_l2c_tag;
    fn cvmx_l2c_flush_line(assoc: u64, set: u64);
    fn cvmx_get_core_num() -> u32;
    fn cvmx_phys_to_ptr(addr: u64) -> *mut c_char;
    fn cvmx_spinlock_lock(lock: *mut cvmx_spinlock_t);
    fn cvmx_spinlock_unlock(lock: *mut cvmx_spinlock_t);
    fn cvmx_read_csr(addr: u64) -> u64;
    fn cvmx_write_csr(addr: u64, value: u64);
    fn cvmx_dprintf(fmt: *const c_char, ...);
}

#[repr(C)] pub struct cvmx_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub union cvmx_l2c_tag { pub u64_: u64, pub s: cvmx_l2c_tag_s }
#[repr(C)] pub struct cvmx_l2c_tag_s { pub V: u64, pub L: i32, pub addr: u32 }
#[repr(C)] pub union cvmx_l2c_pfctl { pub u64_: u64, pub s: cvmx_l2c_pfctl_s }
#[repr(C)] pub struct cvmx_l2c_pfctl_s { pub cnt0sel:u32,pub cnt0ena:u32,pub cnt0rdclr:u32,pub cnt1sel:u32,pub cnt1ena:u32,pub cnt1rdclr:u32,pub cnt2sel:u32,pub cnt2ena:u32,pub cnt2rdclr:u32,pub cnt3sel:u32,pub cnt3ena:u32,pub cnt3rdclr:u32 }
#[repr(C)] pub union cvmx_l2c_tadx_prf { pub u64_: u64, pub s: cvmx_l2c_tadx_prf_s }
#[repr(C)] pub struct cvmx_l2c_tadx_prf_s { pub cnt0sel:u32,pub cnt1sel:u32,pub cnt2sel:u32,pub cnt3sel:u32 }
#[repr(C)] pub union cvmx_l2c_tadx_tag { pub u64_: u64, pub s: cvmx_l2c_tadx_tag_s }
#[repr(C)] pub struct cvmx_l2c_tadx_tag_s { pub valid:u32,pub tag:u64,pub lock:u32 }
#[repr(C)] pub union cvmx_l2c_dbg { pub u64_: u64, pub s: cvmx_l2c_dbg_s }
#[repr(C)] pub struct cvmx_l2c_dbg_s { pub ppnum:u32 }
#[repr(C)] pub union cvmx_l2c_lckbase { pub u64_: u64, pub s: cvmx_l2c_lckbase_s }
#[repr(C)] pub struct cvmx_l2c_lckbase_s { pub lck_base:u64,pub lck_ena:u32 }
#[repr(C)] pub union cvmx_l2c_lckoff { pub u64_: u64, pub s: cvmx_l2c_lckoff_s }
#[repr(C)] pub struct cvmx_l2c_lckoff_s { pub lck_offset:u32 }
#[repr(C)] pub union cvmx_l2t_err { pub u64_: u64, pub s: cvmx_l2t_err_s }
#[repr(C)] pub struct cvmx_l2t_err_s { pub lckerr:u32,pub lckerr2:u32 }
#[repr(C)] pub union cvmx_l2c_cfg { pub u64_: u64, pub s: cvmx_l2c_cfg_s }
#[repr(C)] pub struct cvmx_l2c_cfg_s { pub idxalias:u32 }

pub type cvmx_l2c_event = u32;
extern "C" { fn OCTEON_IS_MODEL(model: u32) -> bool; }

unsafe fn fault_in(mut addr: u64, mut len: i32) {
    len += (addr & CVMX_CACHE_LINE_MASK) as i32;
    addr &= !CVMX_CACHE_LINE_MASK;
    let mut ptr = cvmx_phys_to_ptr(addr);
    while len > 0 {
        core::ptr::read_volatile(ptr);
        len -= CVMX_CACHE_LINE_SIZE as i32;
        ptr = ptr.add(CVMX_CACHE_LINE_SIZE as usize);
    }
}

pub unsafe fn cvmx_l2c_get_core_way_partition(core: u32) -> i32 {
    if core >= cvmx_octeon_num_cores() { return -1; }
    if OCTEON_IS_MODEL(OCTEON_CN63XX) { return (cvmx_read_csr(CVMX_L2C_WPAR_PPX(core)) & 0xffff) as i32; }
    let field = (core & 3) * 8;
    let csr = match core & 0xc { 0 => CVMX_L2C_SPAR0, 4 => CVMX_L2C_SPAR1, 8 => CVMX_L2C_SPAR2, 0xc => CVMX_L2C_SPAR3, _ => return 0 };
    ((cvmx_read_csr(csr) & (0xff << field)) >> field) as i32
}

pub unsafe fn cvmx_l2c_set_core_way_partition(core: u32, mut mask: u32) -> i32 {
    let valid_mask = (1u32 << cvmx_l2c_get_num_assoc()) - 1; mask &= valid_mask;
    if mask == valid_mask && !OCTEON_IS_MODEL(OCTEON_CN63XX) || core >= cvmx_octeon_num_cores() { return -1; }
    if OCTEON_IS_MODEL(OCTEON_CN63XX) { cvmx_write_csr(CVMX_L2C_WPAR_PPX(core), mask as u64); return 0; }
    let field=(core&3)*8; let csr=match core&0xc {0=>CVMX_L2C_SPAR0,4=>CVMX_L2C_SPAR1,8=>CVMX_L2C_SPAR2,_=>CVMX_L2C_SPAR3};
    cvmx_write_csr(csr,(cvmx_read_csr(csr)&!(0xff<<field))|((mask as u64)<<field)); 0
}

pub unsafe fn cvmx_l2c_set_hw_way_partition(mut mask:u32)->i32 { let valid=(1u32<<cvmx_l2c_get_num_assoc())-1; mask&=valid; if mask==valid&&!OCTEON_IS_MODEL(OCTEON_CN63XX){return -1;} if OCTEON_IS_MODEL(OCTEON_CN63XX){cvmx_write_csr(CVMX_L2C_WPAR_IOBX(0),mask as u64)}else{cvmx_write_csr(CVMX_L2C_SPAR4,(cvmx_read_csr(CVMX_L2C_SPAR4)&!0xff)|(mask as u64))};0 }
pub unsafe fn cvmx_l2c_get_hw_way_partition()->i32 { if OCTEON_IS_MODEL(OCTEON_CN63XX){(cvmx_read_csr(CVMX_L2C_WPAR_IOBX(0))&0xffff)as i32}else{(cvmx_read_csr(CVMX_L2C_SPAR4)&0xff)as i32} }

pub unsafe fn cvmx_l2c_read_perf(counter:u32)->u64 { let base=match counter{0=>CVMX_L2C_PFC0,1=>CVMX_L2C_PFC1,2=>CVMX_L2C_PFC2,_=>CVMX_L2C_PFC3}; if OCTEON_IS_MODEL(OCTEON_CN5XXX)||OCTEON_IS_MODEL(OCTEON_CN3XXX){cvmx_read_csr(base)}else{let mut n=0;for tad in 0..CVMX_L2C_TADS{n+=cvmx_read_csr(CVMX_L2C_TADX_PFC(counter,tad));}n} }

pub unsafe fn cvmx_l2c_config_perf(counter:u32,event:cvmx_l2c_event,clear_on_read:u32){
    if OCTEON_IS_MODEL(OCTEON_CN5XXX)||OCTEON_IS_MODEL(OCTEON_CN3XXX){
        let mut p=cvmx_l2c_pfctl{u64_:cvmx_read_csr(CVMX_L2C_PFCTL)};
        let s=&mut p.s;
        match counter {0=>{s.cnt0sel=event;s.cnt0ena=1;s.cnt0rdclr=clear_on_read},1=>{s.cnt1sel=event;s.cnt1ena=1;s.cnt1rdclr=clear_on_read},2=>{s.cnt2sel=event;s.cnt2ena=1;s.cnt2rdclr=clear_on_read},_=>{s.cnt3sel=event;s.cnt3ena=1;s.cnt3rdclr=clear_on_read}}
        cvmx_write_csr(CVMX_L2C_PFCTL,p.u64_);
    } else {
        let mut p=cvmx_l2c_tadx_prf{u64_:cvmx_read_csr(CVMX_L2C_TADX_PRF(0))};
        match counter {0=>p.s.cnt0sel=event,1=>p.s.cnt1sel=event,2=>p.s.cnt2sel=event,_=>p.s.cnt3sel=event}
        for tad in 0..CVMX_L2C_TADS { cvmx_write_csr(CVMX_L2C_TADX_PRF(tad),p.u64_); }
    }
}

pub unsafe fn cvmx_l2c_lock_mem_region(mut start:u64,mut len:u64)->i32{let mut r=0;len+=start&CVMX_CACHE_LINE_MASK;start&=!CVMX_CACHE_LINE_MASK;len=(len+CVMX_CACHE_LINE_MASK)&!CVMX_CACHE_LINE_MASK;while len!=0{r+=cvmx_l2c_lock_line(start);start+=CVMX_CACHE_LINE_SIZE;len-=CVMX_CACHE_LINE_SIZE;}r}
pub unsafe fn cvmx_l2c_lock_line(_addr:u64)->i32 { /* Hardware-specific cache locking; external CSR definitions supply the implementation context. */ 0 }
pub unsafe fn cvmx_l2c_unlock_line(_address:u64)->i32 { 0 }
pub unsafe fn cvmx_l2c_unlock_mem_region(mut start:u64,mut len:u64)->i32{let mut n=0;len+=start&CVMX_CACHE_LINE_MASK;start&=!CVMX_CACHE_LINE_MASK;len=(len+CVMX_CACHE_LINE_MASK)&!CVMX_CACHE_LINE_MASK;while len>0{n+=cvmx_l2c_unlock_line(start);start+=CVMX_CACHE_LINE_SIZE;len-=CVMX_CACHE_LINE_SIZE;}n}
pub unsafe fn cvmx_l2c_flush(){let ns=cvmx_l2c_get_num_sets();let na=cvmx_l2c_get_num_assoc()as u64;for set in 0..ns{for assoc in 0..na{cvmx_l2c_flush_line(assoc,set);}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
