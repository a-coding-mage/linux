// SPDX-License-Identifier: GPL-2.0
/* Translated from linux/arch/alpha/kernel/err_titan.c. */

unsafe fn titan_parse_c_misc(c_misc: u64, print: i32) -> i32 {
    #[cfg(CONFIG_VERBOSE_MCHECK)]
    let mut src: *const i8;
    let mut status = MCHK_DISPOSITION_REPORT;
    const NXM: u64 = 1 << 28;
    const NXS_S: u32 = 29;
    const NXS_M: u64 = 0x7;
    if c_misc & NXM == 0 { return MCHK_DISPOSITION_UNKNOWN_ERROR; }
    #[cfg(CONFIG_VERBOSE_MCHECK)]
    {
        if print == 0 { return status; }
        let mut nxs = ((c_misc >> NXS_S) & NXS_M) as i32;
        match nxs {
            0..=3 => { src = b"CPU\0".as_ptr() as *const i8; }
            4..=5 => { src = b"Pchip\0".as_ptr() as *const i8; nxs -= 4; }
            _ => { src = b"Unknown, NXS =\0".as_ptr() as *const i8; }
        }
        printk(b"%s    Non-existent memory access from: %s %d\n\0".as_ptr() as *const i8, err_print_prefix, src, nxs);
    }
    status
}

unsafe fn titan_parse_p_serror(which: i32, serror: u64, print: i32) -> i32 {
    let status = MCHK_DISPOSITION_REPORT;
    const LOST_UECC: u64 = 1 << 0; const UECC: u64 = 1 << 1; const CRE: u64 = 1 << 2;
    const NXIO: u64 = 1 << 3; const LOST_CRE: u64 = 1 << 4;
    const ECCMASK: u64 = UECC | CRE;
    const ERRMASK: u64 = LOST_UECC | UECC | CRE | NXIO | LOST_CRE;
    if serror & ERRMASK == 0 { return MCHK_DISPOSITION_UNKNOWN_ERROR; }
    #[cfg(CONFIG_VERBOSE_MCHECK)]
    {
        if print == 0 { return status; }
        printk(b"%s  PChip %d SERROR: %016llx\n\0".as_ptr() as *const i8, err_print_prefix, which, serror);
        if serror & ECCMASK != 0 { printk(b"%s    ECC Error\n\0".as_ptr() as *const i8, err_print_prefix); }
        if serror & NXIO != 0 { printk(b"%s    Non Existent I/O Error\n\0".as_ptr() as *const i8, err_print_prefix); }
        if serror & LOST_UECC != 0 { printk(b"%s    Lost Uncorrectable ECC Error\n\0".as_ptr() as *const i8, err_print_prefix); }
        if serror & LOST_CRE != 0 { printk(b"%s    Lost Correctable ECC Error\n\0".as_ptr() as *const i8, err_print_prefix); }
    }
    status
}

unsafe fn titan_parse_p_perror(which: i32, port: i32, perror: u64, print: i32) -> i32 {
    let mut status = MCHK_DISPOSITION_REPORT;
    const LOST:u64=1<<0; const SERR:u64=1<<1; const PERR:u64=1<<2; const DCRTO:u64=1<<3;
    const SGE:u64=1<<4; const APE:u64=1<<5; const TA:u64=1<<6; const DPE:u64=1<<7;
    const NDS:u64=1<<8; const IPTPR:u64=1<<9; const IPTPW:u64=1<<10;
    const ERRMASK:u64=LOST|SERR|PERR|DCRTO|SGE|APE|TA|DPE|NDS|IPTPR|IPTPW;
    const DAC:u64=1<<47; const MWIN:u64=1<<48;
    if perror & ERRMASK == 0 { return MCHK_DISPOSITION_UNKNOWN_ERROR; }
    let cmd = ((perror >> 52) & 0xf) as i32;
    let addr = ((perror >> 14) & 0x1ffffffff) << 2;
    if ((perror & NDS != 0) || (perror & ERRMASK == LOST)) && (((cmd & 0xe)==2 && addr<0x1000) || ((cmd&0xe)==6 && addr>=0xa0000 && addr<0x100000)) { status=MCHK_DISPOSITION_DISMISS; }
    #[cfg(CONFIG_VERBOSE_MCHECK)]
    {
        if print == 0 { return status; }
        printk(b"%s  PChip %d %cPERROR: %016llx\n\0".as_ptr() as *const i8, err_print_prefix, which, if port != 0 { b'A' } else { b'G' }, perror);
        if perror & IPTPW != 0 { printk(b"%s    Invalid Peer-to-Peer Write\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & IPTPR != 0 { printk(b"%s    Invalid Peer-to-Peer Read\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & NDS != 0 { printk(b"%s    No DEVSEL as PCI Master [Master Abort]\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & DPE != 0 { printk(b"%s    Data Parity Error\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & TA != 0 { printk(b"%s    Target Abort\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & APE != 0 { printk(b"%s    Address Parity Error\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & SGE != 0 { printk(b"%s    Scatter-Gather Error, Invalid PTE\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & DCRTO != 0 { printk(b"%s    Delayed-Completion Retry Timeout\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & PERR != 0 { printk(b"%s    PERR Asserted\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & SERR != 0 { printk(b"%s    SERR Asserted\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror & LOST != 0 { printk(b"%s    Lost Error\n\0".as_ptr() as *const i8,err_print_prefix); }
        printk(b"%s      Command: 0x%x\n      Address: 0x%lx\n\0".as_ptr() as *const i8,err_print_prefix,cmd,addr);
        if perror&DAC != 0 { printk(b"%s      Dual Address Cycle\n\0".as_ptr() as *const i8,err_print_prefix); }
        if perror&MWIN != 0 { printk(b"%s      Hit in Monster Window\n\0".as_ptr() as *const i8,err_print_prefix); }
    }
    status
}

unsafe fn titan_parse_p_agperror(which:i32, agperror:u64, print:i32)->i32 {
    let status=MCHK_DISPOSITION_REPORT; const LOST:u64=1; const ERRMASK:u64=0x7f; const DAC:u64=1<<48; const MWIN:u64=1<<49; const FENCE:u64=1<<59;
    if agperror&ERRMASK==0{return MCHK_DISPOSITION_UNKNOWN_ERROR;}
    #[cfg(CONFIG_VERBOSE_MCHECK)] { if print==0{return status;} printk(b"%s  PChip %d AGPERROR: %016llx\n\0".as_ptr() as *const i8,err_print_prefix,which,agperror); if agperror&LOST!=0{printk(b"%s    Lost Error\n\0".as_ptr() as *const i8,err_print_prefix);} if agperror&FENCE!=0{printk(b"%s      FENCE\n\0".as_ptr() as *const i8,err_print_prefix);} if agperror&DAC!=0{printk(b"%s      Dual Address Cycle\n\0".as_ptr() as *const i8,err_print_prefix);} if agperror&MWIN!=0{printk(b"%s      Hit in Monster Window\n\0".as_ptr() as *const i8,err_print_prefix);} }
    status
}

unsafe fn titan_parse_p_chip(which:i32,s:u64,g:u64,a:u64,ag:u64,print:i32)->i32 { MCHK_DISPOSITION_UNKNOWN_ERROR | titan_parse_p_serror(which,s,print) | titan_parse_p_perror(which,0,g,print) | titan_parse_p_perror(which,1,a,print) | titan_parse_p_agperror(which,ag,print) }

pub unsafe fn titan_process_logout_frame(h:*mut el_common,print:i32)->i32 { let t=(h as *mut u8).add((*h).sys_offset as usize) as *mut el_TITAN_sysdata_mcheck; MCHK_DISPOSITION_UNKNOWN_ERROR|titan_parse_c_misc((*t).c_misc,print)|titan_parse_p_chip(0,(*t).p0_serror,(*t).p0_gperror,(*t).p0_aperror,(*t).p0_agperror,print)|titan_parse_p_chip(1,(*t).p1_serror,(*t).p1_gperror,(*t).p1_aperror,(*t).p1_agperror,print) }

pub unsafe fn titan_machine_check(vector:usize,la_ptr:usize){ let h=la_ptr as *mut el_common; mb();draina(); if vector!=SCB_Q_SYSMCHK&&vector!=SCB_Q_SYSERR {ev6_machine_check(vector,la_ptr);return;} if titan_process_logout_frame(h,0)!=MCHK_DISPOSITION_DISMISS { let saved=err_print_prefix;err_print_prefix=KERN_CRIT; printk(b"%s*System Error (Vector 0x%x) reported on CPU %d:\n\0".as_ptr() as *const i8,err_print_prefix,vector,smp_processor_id()); #[cfg(CONFIG_VERBOSE_MCHECK)] { titan_process_logout_frame(h,alpha_verbose_mcheck); } err_print_prefix=saved; let t=(h as *mut u8).add((*h).sys_offset as usize) as *mut el_TITAN_sysdata_mcheck; titan_dispatch_irqs((*t).c_dirx&0xf800000000000000); } wrmces(7);mb(); }

unsafe fn el_process_regatta_subpacket(header:*mut el_subpacket)->*mut el_subpacket { if (*header).class!=EL_CLASS__REGATTA_FAMILY { printk(b"%s  ** Unexpected header CLASS %d TYPE %d, aborting\n\0".as_ptr() as *const i8,err_print_prefix,(*header).class,(*header).type_); return core::ptr::null_mut(); } el_annotate_subpacket(header); (header as *mut u8).add((*header).length as usize) as *mut el_subpacket }

static mut EL_TITAN_PCHIP0_EXTENDED_ANNOTATION: [*const i8; 36] = [b"Subpacket Header\0".as_ptr() as *const i8,b"P0_SCTL\0".as_ptr() as *const i8,b"P0_SERREN\0".as_ptr() as *const i8,b"P0_APCTL\0".as_ptr() as *const i8,b"P0_APERREN\0".as_ptr() as *const i8,b"P0_AGPERREN\0".as_ptr() as *const i8,b"P0_ASPRST\0".as_ptr() as *const i8,b"P0_AWSBA0\0".as_ptr() as *const i8,b"P0_AWSBA1\0".as_ptr() as *const i8,b"P0_AWSBA2\0".as_ptr() as *const i8,b"P0_AWSBA3\0".as_ptr() as *const i8,b"P0_AWSM0\0".as_ptr() as *const i8,b"P0_AWSM1\0".as_ptr() as *const i8,b"P0_AWSM2\0".as_ptr() as *const i8,b"P0_AWSM3\0".as_ptr() as *const i8,b"P0_ATBA0\0".as_ptr() as *const i8,b"P0_ATBA1\0".as_ptr() as *const i8,b"P0_ATBA2\0".as_ptr() as *const i8,b"P0_ATBA3\0".as_ptr() as *const i8,b"P0_GPCTL\0".as_ptr() as *const i8,b"P0_GPERREN\0".as_ptr() as *const i8,b"P0_GSPRST\0".as_ptr() as *const i8,b"P0_GWSBA0\0".as_ptr() as *const i8,b"P0_GWSBA1\0".as_ptr() as *const i8,b"P0_GWSBA2\0".as_ptr() as *const i8,b"P0_GWSBA3\0".as_ptr() as *const i8,b"P0_GWSM0\0".as_ptr() as *const i8,b"P0_GWSM1\0".as_ptr() as *const i8,b"P0_GWSM2\0".as_ptr() as *const i8,b"P0_GWSM3\0".as_ptr() as *const i8,b"P0_GTBA0\0".as_ptr() as *const i8,b"P0_GTBA1\0".as_ptr() as *const i8,b"P0_GTBA2\0".as_ptr() as *const i8,b"P0_GTBA3\0".as_ptr() as *const i8,core::ptr::null()];
static mut EL_TITAN_PCHIP1_EXTENDED_ANNOTATION: [*const i8; 1] = [core::ptr::null()];
static mut EL_TITAN_MEMORY_EXTENDED_ANNOTATION: [*const i8; 1] = [core::ptr::null()];
static mut EL_TITAN_ANNOTATIONS: [el_subpacket_annotation; 4] = [SUBPACKET_ANNOTATION!(EL_CLASS__REGATTA_FAMILY,EL_TYPE__REGATTA__TITAN_PCHIP0_EXTENDED,1,b"Titan PChip 0 Extended Frame\0".as_ptr() as *const i8,EL_TITAN_PCHIP0_EXTENDED_ANNOTATION.as_mut_ptr()),SUBPACKET_ANNOTATION!(EL_CLASS__REGATTA_FAMILY,EL_TYPE__REGATTA__TITAN_PCHIP1_EXTENDED,1,b"Titan PChip 1 Extended Frame\0".as_ptr() as *const i8,EL_TITAN_PCHIP1_EXTENDED_ANNOTATION.as_mut_ptr()),SUBPACKET_ANNOTATION!(EL_CLASS__REGATTA_FAMILY,EL_TYPE__REGATTA__TITAN_MEMORY_EXTENDED,1,b"Titan Memory Extended Frame\0".as_ptr() as *const i8,EL_TITAN_MEMORY_EXTENDED_ANNOTATION.as_mut_ptr()),SUBPACKET_ANNOTATION!(EL_CLASS__REGATTA_FAMILY,EL_TYPE__TERMINATION__TERMINATION,1,b"Termination Subpacket\0".as_ptr() as *const i8,core::ptr::null_mut())];
static mut TITAN_SUBPACKET_HANDLER: el_subpacket_handler = SUBPACKET_HANDLER_INIT!(EL_CLASS__REGATTA_FAMILY, el_process_regatta_subpacket);

pub unsafe fn titan_register_error_handlers(){ cdl_register_subpacket_handler(&mut TITAN_SUBPACKET_HANDLER); ev6_register_error_handlers(); }

unsafe fn privateer_process_680_frame(_h:*mut el_common,_print:i32)->i32 { MCHK_DISPOSITION_UNKNOWN_ERROR }
pub unsafe fn privateer_process_logout_frame(h:*mut el_common,print:i32)->i32 { let e=h as *mut el_common_EV6_mcheck; match (*e).MCHK_Code { 0x86|0x9e|0x8e|0x90|0x98|0xa0|0xa2=>ev6_process_logout_frame(h,print),0x204|0x202=>titan_process_logout_frame(h,print),0x206=>privateer_process_680_frame(h,print),_=>{if print!=0{printk(b"%s** Unknown Error, frame follows\n\0".as_ptr() as *const i8,err_print_prefix);mchk_dump_logout_frame(h);} MCHK_DISPOSITION_REPORT} } }

pub unsafe fn privateer_machine_check(vector:usize,la_ptr:usize){ let h=la_ptr as *mut el_common; mb();draina(); if vector!=SCB_Q_SYSEVENT{return titan_machine_check(vector,la_ptr);} let saved=err_print_prefix;err_print_prefix=KERN_CRIT;privateer_process_680_frame(h,1);err_print_prefix=saved;let t=(la_ptr+(*h).sys_offset as usize) as *mut el_TITAN_sysdata_mcheck;titan_dispatch_irqs((*t).c_dirx&0xe00);wrmces(7);mb(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
