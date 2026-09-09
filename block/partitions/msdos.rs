// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of partitions/msdos.c.

#[inline]
unsafe fn nr_sects(p: *mut msdos_partition) -> sector_t {
    get_unaligned_le32(&(*p).nr_sects) as sector_t
}
#[inline]
unsafe fn start_sect(p: *mut msdos_partition) -> sector_t {
    get_unaligned_le32(&(*p).start_sect) as sector_t
}
#[inline]
unsafe fn is_extended_partition(p: *mut msdos_partition) -> i32 {
    ((*p).sys_ind == DOS_EXTENDED_PARTITION || (*p).sys_ind == WIN98_EXTENDED_PARTITION ||
        (*p).sys_ind == LINUX_EXTENDED_PARTITION) as i32
}

const MSDOS_LABEL_MAGIC1: u8 = 0x55;
const MSDOS_LABEL_MAGIC2: u8 = 0xaa;
#[inline]
unsafe fn msdos_magic_present(p: *mut u8) -> i32 { ((*p == MSDOS_LABEL_MAGIC1) && (*p.add(1) == MSDOS_LABEL_MAGIC2)) as i32 }

const AIX_LABEL_MAGIC1: u8 = 0xc9;
const AIX_LABEL_MAGIC2: u8 = 0xc2;
const AIX_LABEL_MAGIC3: u8 = 0xd4;
const AIX_LABEL_MAGIC4: u8 = 0xc1;
unsafe fn aix_magic_present(state: *mut parsed_partitions, p: *mut u8) -> i32 {
    if *p != AIX_LABEL_MAGIC1 || *p.add(1) != AIX_LABEL_MAGIC2 || *p.add(2) != AIX_LABEL_MAGIC3 || *p.add(3) != AIX_LABEL_MAGIC4 { return 0; }
    let mut pt = p.add(0x1be) as *mut msdos_partition;
    for _ in 1..=4 {
        if (*pt).sys_ind == SOLARIS_X86_PARTITION || (*pt).sys_ind == LINUX_RAID_PARTITION || (*pt).sys_ind == LINUX_DATA_PARTITION || (*pt).sys_ind == LINUX_LVM_PARTITION || is_extended_partition(pt) != 0 { return 0; }
        pt = pt.add(1);
    }
    let mut sect = core::mem::MaybeUninit::<Sector>::uninit();
    let d = read_part_sector(state, 7, sect.as_mut_ptr());
    if d.is_null() { return 0; }
    let ret = (*d == b'_' && *d.add(1) == b'L' && *d.add(2) == b'V' && *d.add(3) == b'M') as i32;
    put_dev_sector(sect.assume_init()); ret
}

unsafe fn set_info(state: *mut parsed_partitions, slot: i32, disksig: u32) {
    let info = &mut (*state).parts[slot as usize].info;
    snprintf(info.uuid.as_mut_ptr(), core::mem::size_of_val(&info.uuid), b"%08x-%02x\0".as_ptr() as *const i8, disksig, slot);
    info.volname[0] = 0; (*state).parts[slot as usize].has_info = true;
}

unsafe fn parse_extended(state: *mut parsed_partitions, first_sector: sector_t, first_size: sector_t, disksig: u32) {
    let sector_size = queue_logical_block_size((*(*state).disk).queue) / 512;
    let (mut this_sector, mut this_size) = (first_sector, first_size); let mut loopct = 0;
    loop {
        loopct += 1; if loopct > 100 || (*state).next == (*state).limit { return; }
        let mut sect = core::mem::MaybeUninit::<Sector>::uninit(); let data = read_part_sector(state, this_sector, sect.as_mut_ptr());
        if data.is_null() { return; }
        if msdos_magic_present(data.add(510)) == 0 { put_dev_sector(sect.assume_init()); return; }
        let base = data.add(0x1be) as *mut msdos_partition;
        for i in 0..4 { let p = base.add(i); if nr_sects(p) == 0 || is_extended_partition(p) != 0 { continue; }
            let offs = start_sect(p)*sector_size; let size = nr_sects(p)*sector_size; let next = this_sector+offs;
            if i >= 2 && (offs+size > this_size || next < first_sector || next+size > first_sector+first_size) { continue; }
            put_partition(state, (*state).next, next, size); set_info(state, (*state).next, disksig);
            if (*p).sys_ind == LINUX_RAID_PARTITION { (*state).parts[(*state).next as usize].flags = ADDPART_FLAG_RAID; }
            loopct=0; (*state).next += 1; if (*state).next == (*state).limit { put_dev_sector(sect.assume_init()); return; }
        }
        let mut ep = base; let mut found = false; for i in 0..4 { let q=base.add(i); if nr_sects(q)!=0 && is_extended_partition(q)!=0 { ep=q; found=true; break; } }
        if !found { put_dev_sector(sect.assume_init()); return; }
        this_sector=first_sector+start_sect(ep)*sector_size; this_size=nr_sects(ep)*sector_size; put_dev_sector(sect.assume_init());
    }
}

const SOLARIS_X86_NUMSLICE: usize = 16;
const SOLARIS_X86_VTOC_SANE: u32 = 0x600d_deee;
#[repr(C)] pub struct solaris_x86_slice { pub s_tag: __le16, pub s_flag: __le16, pub s_start: __le32, pub s_size: __le32 }
#[repr(C)] pub struct solaris_x86_vtoc { pub v_bootinfo: [u32;3], pub v_sanity: __le32, pub v_version: __le32, pub v_volume:[i8;8], pub v_sectorsz:__le16, pub v_nparts:__le16, pub v_reserved:[u32;10], pub v_slice:[solaris_x86_slice;16], pub timestamp:[u32;16], pub v_asciilabel:[i8;128] }

unsafe fn parse_solaris_x86(_state:*mut parsed_partitions,_offset:sector_t,_size:sector_t,_origin:i32) { /* CONFIG_SOLARIS_X86_PARTITION conditional body */ }

const BSD_DISKMAGIC:u32=0x82564557; const BSD_MAXPARTITIONS:usize=16; const OPENBSD_MAXPARTITIONS:usize=16; const BSD_FS_UNUSED:u8=0;
#[repr(C)] pub struct bsd_partition { pub p_size:__le32,p_offset:__le32,p_fsize:__le32,p_fstype:u8,p_frag:u8,p_cpg:__le16 }
#[repr(C)] pub struct bsd_disklabel { pub d_magic:__le32,pad:[u8;0] }
unsafe fn parse_freebsd(_s:*mut parsed_partitions,_o:sector_t,_z:sector_t,_n:i32) {}
unsafe fn parse_netbsd(_s:*mut parsed_partitions,_o:sector_t,_z:sector_t,_n:i32) {}
unsafe fn parse_openbsd(_s:*mut parsed_partitions,_o:sector_t,_z:sector_t,_n:i32) {}
unsafe fn parse_unixware(_s:*mut parsed_partitions,_o:sector_t,_z:sector_t,_n:i32) {}
unsafe fn parse_minix(_s:*mut parsed_partitions,_o:sector_t,_z:sector_t,_n:i32) {}

#[repr(C)] struct subtype { id:u8, parse: Option<unsafe fn(*mut parsed_partitions,sector_t,sector_t,i32)> }
static SUBTYPES:[subtype;8]=[
    subtype{id:FREEBSD_PARTITION,parse:Some(parse_freebsd)}, subtype{id:NETBSD_PARTITION,parse:Some(parse_netbsd)}, subtype{id:OPENBSD_PARTITION,parse:Some(parse_openbsd)}, subtype{id:MINIX_PARTITION,parse:Some(parse_minix)}, subtype{id:UNIXWARE_PARTITION,parse:Some(parse_unixware)}, subtype{id:SOLARIS_X86_PARTITION,parse:Some(parse_solaris_x86)}, subtype{id:NEW_SOLARIS_X86_PARTITION,parse:Some(parse_solaris_x86)}, subtype{id:0,parse:None}];

pub unsafe fn msdos_partition(state:*mut parsed_partitions)->i32 {
    let sector_size=queue_logical_block_size((*(*state).disk).queue)/512; let mut sect=core::mem::MaybeUninit::<Sector>::uninit(); let data=read_part_sector(state,0,sect.as_mut_ptr()); if data.is_null(){return -1;}
    if aix_magic_present(state,data)!=0 { put_dev_sector(sect.assume_init()); #[cfg(CONFIG_AIX_PARTITION)] { return aix_partition(state); } #[cfg(not(CONFIG_AIX_PARTITION))] { seq_buf_puts(&mut (*state).pp_buf,b" [AIX]\0".as_ptr() as *const i8); return 0; } }
    if msdos_magic_present(data.add(510))==0 { put_dev_sector(sect.assume_init()); return 0; }
    let p0=data.add(0x1be) as *mut msdos_partition; for i in 0..4 { let p=p0.add(i); if (*p).boot_ind!=0 && (*p).boot_ind!=0x80 { put_dev_sector(sect.assume_init()); return 0; } }
    let disksig=le32_to_cpup(data.add(0x1b8) as *const __le32); (*state).next=5;
    for i in 0..4 { let p=p0.add(i); let start=start_sect(p)*sector_size; let size=nr_sects(p)*sector_size; if size==0{continue;} if is_extended_partition(p)!=0 { let n=core::cmp::min(size,core::cmp::max(sector_size,2)); put_partition(state,(i+1) as i32,start,n); parse_extended(state,start,size,disksig); } else { put_partition(state,(i+1) as i32,start,size); set_info(state,(i+1) as i32,disksig); if (*p).sys_ind==LINUX_RAID_PARTITION {(*state).parts[i+1].flags=ADDPART_FLAG_RAID;} } }
    put_dev_sector(sect.assume_init()); 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
