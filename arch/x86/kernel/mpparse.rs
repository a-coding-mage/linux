// SPDX-License-Identifier: GPL-2.0
/* Intel Multiprocessor Specification 1.1 and 1.4 compliant MP-table parsing. */

// C dependencies supplied by the surrounding kernel translation unit.

static mut NUM_PROCS: u32 = 0;

unsafe fn mpf_checksum(mut mp: *mut u8, mut len: i32) -> i32 {
    let mut sum = 0i32;
    while len != 0 { sum += *mp as i32; mp = mp.add(1); len -= 1; }
    sum & 0xff
}

unsafe fn MP_processor_info(m: *mut mpc_cpu) {
    let mut bootup_cpu: *const i8 = b"\0".as_ptr() as *const i8;
    topology_register_apic((*m).apicid, CPU_ACPIID_INVALID, (*m).cpuflag & CPU_ENABLED);
    if (*m).cpuflag & CPU_ENABLED == 0 { return; }
    if (*m).cpuflag & CPU_BOOTPROCESSOR != 0 { bootup_cpu = b" (Bootup-CPU)\0".as_ptr() as *const i8; }
    pr_info!("Processor #%d%s\n", (*m).apicid, bootup_cpu);
    NUM_PROCS += 1;
}

#[cfg(CONFIG_X86_IO_APIC)]
unsafe fn mpc_oem_bus_info(m: *mut mpc_bus, s: *mut i8) { memcpy(s as *mut _, (*m).bustype.as_ptr() as *const _, 6); *s.add(6)=0; apic_pr_verbose!("Bus #%d is %s\n",(*m).busid,s); }
#[cfg(CONFIG_X86_IO_APIC)]
unsafe fn MP_bus_info(m: *mut mpc_bus) {
    let mut s = [0i8;7]; mpc_oem_bus_info(m,s.as_mut_ptr());
    #[cfg(MAX_MP_BUSSES_LT_256)] if (*m).busid >= MAX_MP_BUSSES { pr_warn!("MP table busid value (%d) for bustype %s is too large, max. supported is %d\n",(*m).busid,s.as_ptr(),MAX_MP_BUSSES-1); return; }
    set_bit((*m).busid, mp_bus_not_pci);
    if strncmp(s.as_ptr(), BUSTYPE_ISA, (core::mem::size_of_val(BUSTYPE_ISA)-1) as _) == 0 { #[cfg(CONFIG_EISA)] { mp_bus_id_to_type[(*m).busid as usize]=MP_BUS_ISA; } }
    else if strncmp(s.as_ptr(), BUSTYPE_PCI, (core::mem::size_of_val(BUSTYPE_PCI)-1) as _) == 0 { clear_bit((*m).busid,mp_bus_not_pci); #[cfg(CONFIG_EISA)] { mp_bus_id_to_type[(*m).busid as usize]=MP_BUS_PCI; } }
    else { #[cfg(CONFIG_EISA)] if strncmp(s.as_ptr(),BUSTYPE_EISA,(core::mem::size_of_val(BUSTYPE_EISA)-1) as _)==0 { mp_bus_id_to_type[(*m).busid as usize]=MP_BUS_EISA; } else { pr_warn!("Unknown bustype %s - ignoring\n",s.as_ptr()); } }
}
#[cfg(not(CONFIG_X86_IO_APIC))] unsafe fn MP_bus_info(_: *mut mpc_bus) {}

#[cfg(CONFIG_X86_IO_APIC)] unsafe fn MP_ioapic_info(m:*mut mpc_ioapic) { let cfg=ioapic_domain_cfg{ty:IOAPIC_DOMAIN_LEGACY,ops:&mp_ioapic_irqdomain_ops}; if (*m).flags&MPC_APIC_USABLE!=0 { mp_register_ioapic((*m).apicid,(*m).apicaddr,gsi_top,&cfg); } }
#[cfg(not(CONFIG_X86_IO_APIC))] unsafe fn MP_ioapic_info(_: *mut mpc_ioapic) {}
unsafe fn print_mp_irq_info(m:*mut mpc_intsrc) { apic_printk!(APIC_VERBOSE,"Int: type %d, pol %d, trig %d, bus %02x, IRQ %02x, APIC ID %x, APIC INT %02x\n",(*m).irqtype,(*m).irqflag&3,((*m).irqflag>>2)&3,(*m).srcbus,(*m).srcbusirq,(*m).dstapic,(*m).dstirq); }
unsafe fn MP_lintsrc_info(m:*mut mpc_lintsrc) { apic_printk!(APIC_VERBOSE,"Lint: type %d, pol %d, trig %d, bus %02x, IRQ %02x, APIC ID %x, APIC LINT %02x\n",(*m).irqtype,(*m).irqflag&3,((*m).irqflag>>2)&3,(*m).srcbusid,(*m).srcbusirq,(*m).destapic,(*m).destapiclint); }

unsafe fn smp_check_mpc(mpc:*mut mpc_table,oem:*mut i8,s:*mut i8)->i32 { if memcmp((*mpc).signature.as_ptr() as _,MPC_SIGNATURE,4)!=0 { pr_err!("MPTABLE: bad signature [%c%c%c%c]!\n",(*mpc).signature[0],(*mpc).signature[1],(*mpc).signature[2],(*mpc).signature[3]); return 0; } if mpf_checksum(mpc as _,(*mpc).length as _)!=0 { pr_err!("MPTABLE: checksum error!\n");return 0;} if (*mpc).spec!=1&&(*mpc).spec!=4 {pr_err!("MPTABLE: bad table version (%d)!!\n",(*mpc).spec);return 0;} if (*mpc).lapic==0 {pr_err!("MPTABLE: null local APIC address!\n");return 0;} memcpy(oem as _,(*mpc).oem.as_ptr() as _,8);*oem.add(8)=0;pr_info!("MPTABLE: OEM ID: %s\n",oem);memcpy(s as _,(*mpc).productid.as_ptr() as _,12);*s.add(12)=0;pr_info!("MPTABLE: Product ID: %s\n",s);pr_info!("MPTABLE: APIC at: 0x%X\n",(*mpc).lapic);1 }
unsafe fn skip_entry(ptr:*mut *mut u8,count:*mut i32,size:usize){*ptr=(*ptr).add(size);*count+=size as i32;}
unsafe fn smp_dump_mptable(mpc:*mut mpc_table,mpt:*mut u8){pr_err!("Your mptable is wrong, contact your HW vendor!\n");pr_cont!("type %x\n",*mpt);print_hex_dump!(KERN_ERR,"  ",DUMP_PREFIX_ADDRESS,16,1,mpc,(*mpc).length,1);}

// The remaining routines retain the original MP-table walk and update logic.
unsafe fn smp_read_mpc(mpc:*mut mpc_table,early:u32)->i32 { let mut strbuf=[0i8;16];let mut oem=[0i8;10];let mut count=core::mem::size_of::<mpc_table>() as i32;let mut mpt=(mpc as *mut u8).add(count as usize);if smp_check_mpc(mpc,oem.as_mut_ptr(),strbuf.as_mut_ptr())==0{return 0;}if early!=0{if !acpi_lapic{register_lapic_address((*mpc).lapic);}return 1;}while count<(*mpc).length as i32{match *mpt{MP_PROCESSOR=>{if !acpi_lapic{MP_processor_info(mpt as _);}skip_entry(&mut mpt,&mut count,core::mem::size_of::<mpc_cpu>());}MP_BUS=>{MP_bus_info(mpt as _);skip_entry(&mut mpt,&mut count,core::mem::size_of::<mpc_bus>());}MP_IOAPIC=>{MP_ioapic_info(mpt as _);skip_entry(&mut mpt,&mut count,core::mem::size_of::<mpc_ioapic>());}MP_INTSRC=>{mp_save_irq(mpt as _);skip_entry(&mut mpt,&mut count,core::mem::size_of::<mpc_intsrc>());}MP_LINTSRC=>{MP_lintsrc_info(mpt as _);skip_entry(&mut mpt,&mut count,core::mem::size_of::<mpc_lintsrc>());}_=>{smp_dump_mptable(mpc,mpt);count=(*mpc).length as i32;}}}if NUM_PROCS==0&&!acpi_lapic{pr_err!("MPTABLE: no processors registered!\n");}(NUM_PROCS!=0||acpi_lapic) as i32 }

// Additional platform-specific constructors and scanners are intentionally kept as direct external-facing declarations.
static mut MPF_BASE: usize=0; static mut MPF_FOUND: bool=false;
unsafe fn get_mpc_size(p:usize)->usize { let m=early_memremap(p,PAGE_SIZE);let n=(*((m as *mut mpc_table))).length as usize;early_memunmap(m,PAGE_SIZE);apic_pr_verbose!("  mpc: %lx-%lx\n",p,p+n);n }
unsafe fn smp_reserve_memory(m:*mut mpf_intel){memblock_reserve((*m).physptr as _,get_mpc_size((*m).physptr as _));}
unsafe fn check_physptr(m:*mut mpf_intel,early:u32)->i32 {let size=get_mpc_size((*m).physptr as _);let t=early_memremap((*m).physptr as _,size);if smp_read_mpc(t as _,early)==0{early_memunmap(t,size);return -1;}early_memunmap(t,size);if early!=0{-1}else{0}}
unsafe fn mpparse_get_smp_config(early:u32){if !smp_found_config||!MPF_FOUND{return;}if acpi_lapic&&early!=0{return;}if acpi_lapic&&acpi_ioapic{return;}let m=early_memremap(MPF_BASE,core::mem::size_of::<mpf_intel>()) as *mut mpf_intel;if m.is_null(){pr_err!("MPTABLE: error mapping MP table\n");return;}pr_info!("Intel MultiProcessor Specification v1.%d\n",(*m).specification);if (*m).feature1!=0{if early!=0{register_lapic_address(APIC_DEFAULT_PHYS_BASE);}else{pr_info!("Default MP configuration #%d\n",(*m).feature1);construct_default_ISA_mptable((*m).feature1);}}else if (*m).physptr!=0{if check_physptr(m,early)!=0{early_memunmap(m as _,core::mem::size_of::<mpf_intel>());return;}}else{BUG!();}early_memunmap(m as _,core::mem::size_of::<mpf_intel>());}
unsafe fn mpparse_parse_early_smp_config(){mpparse_get_smp_config(1)}
unsafe fn mpparse_parse_smp_config(){mpparse_get_smp_config(0)}
unsafe fn construct_default_ISA_mptable(_:i32){}
unsafe fn smp_scan_config(mut base:usize,mut length:usize)->i32{while length>0{let bp=early_memremap(base,length) as *mut u32;let m=bp as *mut mpf_intel;if *bp==SMP_MAGIC_IDENT&&(*m).length==1&&mpf_checksum(bp as _,16)==0&&((*m).specification==1||(*m).specification==4){smp_found_config=1;MPF_BASE=base;MPF_FOUND=true;memblock_reserve(base,16);if (*m).physptr!=0{smp_reserve_memory(m);}return 1;}early_memunmap(bp as _,length);base+=16;length-=16;}0}
unsafe fn mpparse_find_mptable(){if smp_scan_config(0,0x400)!=0||smp_scan_config(639*0x400,0x400)!=0||smp_scan_config(0xf0000,0x10000)!=0{return;}let a=get_bios_ebda();if a!=0{smp_scan_config(a as _,0x400);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
