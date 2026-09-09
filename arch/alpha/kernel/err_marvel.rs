// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/arch/alpha/kernel/err_marvel.c. */

// Kernel and architecture declarations supplied by other translation units.
use core::ptr;

#[inline]
unsafe fn extract(v: u64, s: u32, m: u64) -> u64 { (v >> s) & m }
#[inline]
unsafe fn gen_mask(s: u32, m: u64) -> u64 { m << s }

unsafe fn marvel_print_680_frame(lf: *mut ev7_lf_subpackets) {
    #[cfg(CONFIG_VERBOSE_MCHECK)]
    {
        let packets: &[(i32, &[u8])] = &[
            (EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE, b"Ambient Temperature\0"),
            (EL_TYPE__PAL__ENV__AIRMOVER_FAN, b"AirMover / Fan\0"),
            (EL_TYPE__PAL__ENV__VOLTAGE, b"Voltage\0"),
            (EL_TYPE__PAL__ENV__INTRUSION, b"Intrusion\0"),
            (EL_TYPE__PAL__ENV__POWER_SUPPLY, b"Power Supply\0"),
            (EL_TYPE__PAL__ENV__LAN, b"LAN\0"),
            (EL_TYPE__PAL__ENV__HOT_PLUG, b"Hot Plug\0"),
        ];
        for &(kind, name) in packets {
            let env = (*lf).env[ev7_lf_env_index(kind) as usize];
            if env.is_null() { continue; }
            printk(b"%s**%s event (cabinet %d, drawer %d)\0".as_ptr() as *const i8,
                   err_print_prefix, name.as_ptr(), (*env).cabinet, (*env).drawer);
            printk(b"%s   Module Type: 0x%x - Unit ID 0x%x - Condition 0x%x\n\0".as_ptr() as *const i8,
                   err_print_prefix, (*env).module_type, (*env).unit_id, (*env).condition);
        }
    }
}

unsafe fn marvel_process_680_frame(lf: *mut ev7_lf_subpackets, print: i32) -> i32 {
    let mut status = MCHK_DISPOSITION_UNKNOWN_ERROR;
    let mut i = ev7_lf_env_index(EL_TYPE__PAL__ENV__AMBIENT_TEMPERATURE);
    while i <= ev7_lf_env_index(EL_TYPE__PAL__ENV__HOT_PLUG) {
        if !(*lf).env[i as usize].is_null() { status = MCHK_DISPOSITION_REPORT; }
        i += 1;
    }
    if print != 0 { marvel_print_680_frame(lf); }
    status
}

#[cfg(CONFIG_VERBOSE_MCHECK)]
unsafe fn marvel_print_err_cyc(v: u64) {
    let d: [&[u8]; 9] = [b"No Error\0",b"UNKNOWN\0",b"1 cycle (1 or 2 flit packet)\0",b"2 cycles (3 flit packet)\0",b"9 cycles (18 flit packet)\0",b"10 cycles (19 flit packet)\0",b"UNKNOWN\0",b"UNKNOWN\0",b"UNKNOWN\0"];
    printk(b"%s        Packet In Error: %s\n%s        Error in %s, cycle %lld%s%s\n\0".as_ptr() as *const i8,
        err_print_prefix, d[extract(v,6,7) as usize].as_ptr(), err_print_prefix,
        if v & (1<<5) != 0 { b"DATA\0".as_ptr() } else { b"HEADER\0".as_ptr() },
        extract(v,2,7), if v&1 != 0 { b" [ODD Flit]\0".as_ptr() } else { b"\0".as_ptr() },
        if v&2 != 0 { b" [Even Flit]\0".as_ptr() } else { b"\0".as_ptr() });
}

#[cfg(CONFIG_VERBOSE_MCHECK)]
unsafe fn marvel_print_po7_crrct_sym(v: u64) {
    printk(b"%s      Correctable Error Symptoms:\n%s        Syndrome: 0x%llx\n\0".as_ptr() as *const i8, err_print_prefix, err_print_prefix, extract(v,0,0x7f));
    marvel_print_err_cyc(extract(v,7,0x1ff));
}

#[cfg(CONFIG_VERBOSE_MCHECK)]
unsafe fn marvel_print_po7_ugbge_sym(v: u64) {
    if v & (1u64<<63) == 0 { return; }
    let op = extract(v,40,0xff);
    let s: &[u8] = match op { 0x51=>b"Wr32\0",0x50=>b"WrQW\0",0x54=>b"WrIPR\0",0xd8=>b"Victim\0",0xc5=>b"BlkIO\0",_=>b"UNKNOWN\0" };
    printk(b"%s      Up Hose Garbage Symptom:\n%s        Source Port: %lld - Dest PID: %lld - OpCode: %s\n\0".as_ptr() as *const i8, err_print_prefix,err_print_prefix,extract(v,48,0xf),extract(v,52,0x7ff),s.as_ptr());
    if op != 0xc5 { printk(b"%s        Packet Offset 0x%08llx\n\0".as_ptr() as *const i8,err_print_prefix,extract(v,6,0xffffffff)); }
}

#[cfg(CONFIG_VERBOSE_MCHECK)]
unsafe fn marvel_print_pox_tlb_err(v: u64) {
    if v & (1u64<<63)==0 {return;}
    let e:[&[u8];4]=[b"No Error\0",b"North Port Signaled Error fetching TLB entry\0",b"PTE invalid or UCC or GBG error on this entry\0",b"Address did not hit any DMA window\0"];
    printk(b"%s      TLB Error on index 0x%llx:\n%s        - %s\n%s        - Addr: 0x%016llx\n\0".as_ptr() as *const i8,err_print_prefix,extract(v,3,7),err_print_prefix,e[extract(v,0,3) as usize].as_ptr(),err_print_prefix,extract(v,6,0x3ffffffffff)<<6);
}

unsafe fn marvel_find_io7_with_error(lf: *mut ev7_lf_subpackets) -> *mut ev7_pal_io_subpacket {
    let io=(*lf).io; if io.is_null(){return ptr::null_mut();} memset(io,0x55,core::mem::size_of::<ev7_pal_io_subpacket>());
    let mut io7: *mut io7=ptr::null_mut();
    loop { io7=marvel_next_io7(io7); if io7.is_null(){break;} let mut e=(*io7).csrs.PO7_ERROR_SUM.csr; for i in 0..IO7_NUM_PORTS { if (*io7).ports[i].enabled {e|=(*io7).ports[i].csrs.POx_ERR_SUM.csr;} } if e&(1u64<<63)!=0 {break;} }
    if io7.is_null(){return ptr::null_mut();}
    (*io).io_asic_rev=(*io7).csrs.IO_ASIC_REV.csr; (*io).io_sys_rev=(*io7).csrs.IO_SYS_REV.csr; (*io).io7_uph=(*io7).csrs.IO7_UPH.csr; (*io).hpi_ctl=(*io7).csrs.HPI_CTL.csr; (*io).crd_ctl=(*io7).csrs.CRD_CTL.csr; (*io).hei_ctl=(*io7).csrs.HEI_CTL.csr; (*io).po7_error_sum=(*io7).csrs.PO7_ERROR_SUM.csr; (*io).po7_uncrr_sym=(*io7).csrs.PO7_UNCRR_SYM.csr; (*io).po7_crrct_sym=(*io7).csrs.PO7_CRRCT_SYM.csr; (*io).po7_ugbge_sym=(*io7).csrs.PO7_UGBGE_SYM.csr; (*io).po7_err_pkt0=(*io7).csrs.PO7_ERR_PKT[0].csr; (*io).po7_err_pkt1=(*io7).csrs.PO7_ERR_PKT[1].csr;
    (*lf).io_pid=(*io7).pe; io
}

unsafe fn marvel_process_io_error(lf:*mut ev7_lf_subpackets, _print:i32)->i32 { if (*lf).logout.is_null()||(*lf).io.is_null(){return MCHK_DISPOSITION_UNKNOWN_ERROR;} if (*lf).io->po7_error_sum&(1u64<<63)!=0 { let _=marvel_find_io7_with_error(lf); } MCHK_DISPOSITION_REPORT }
unsafe fn marvel_process_logout_frame(lf:*mut ev7_lf_subpackets, print:i32)->i32 { let mut s=MCHK_DISPOSITION_UNKNOWN_ERROR; if !(*lf).logout.is_null()&&(*lf).logout->rbox_int&0x20000400!=0{s=marvel_process_io_error(lf,print);} if !(*lf).ev7.is_null()&&(*lf).ev7->c_stat==0x14&&(*lf).ev7->c_sts&8==0&&((*lf).ev7->c_addr&0x400ff000000)==0x400fe000000{s=MCHK_DISPOSITION_DISMISS;} s }

pub unsafe fn marvel_machine_check(vector: usize, la_ptr: usize) { mb(); draina(); let mut p: Option<unsafe fn(*mut ev7_lf_subpackets,i32)->i32>=None; let mut typ=b"\0"; match vector { SCB_Q_SYSEVENT=>{p=Some(marvel_process_680_frame);typ=b"System Event\0";},SCB_Q_SYSMCHK=>{p=Some(marvel_process_logout_frame);typ=b"System Uncorrectable Error\0";},SCB_Q_SYSERR=>{p=Some(marvel_process_logout_frame);typ=b"System Correctable Error\0";},_=>{ev7_machine_check(vector,la_ptr);return;} } let mut col=ev7_lf_subpackets::default(); let mut scratch=ev7_pal_io_subpacket::default(); let lf=ev7_collect_logout_frame_subpackets(la_ptr as *mut el_subpacket,&mut col); if let Some(f)=p { if !lf.is_null()&&!(*lf).logout.is_null(){if (*lf).io.is_null(){(*lf).io=&mut scratch;}(*lf).io_pid=(*lf).logout->whami;let d=f(lf,0);if d==MCHK_DISPOSITION_REPORT{printk(b"%s*%s\n\0".as_ptr() as *const i8,err_print_prefix,typ.as_ptr());f(lf,1);}}} wrmces(7);mb(); }
pub unsafe fn marvel_register_error_handlers(){ev7_register_error_handlers();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
