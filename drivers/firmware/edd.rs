// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/firmware/edd.c
 *  Copyright (C) 2002, 2003, 2004 Dell Inc.
 *  by Matt Domsch <Matt_Domsch@dell.com>
 *  disk signature by Matt Domsch, Andrew Wilks, and Sandeep K. Shandilya
 *  legacy CHS by Patrick J. LoPresti <patl@users.sourceforge.net>
 *
 * BIOS Enhanced Disk Drive Services (EDD)
 * conformant to T13 Committee www.t13.org
 *   projects 1572D, 1484D, 1386D, 1226DT
 *
 * This code takes information provided by BIOS EDD calls
 * fn41 - Check Extensions Present and
 * fn48 - Get Device Parameters with EDD extensions
 * made in setup.S, copied to safe structures in setup.c,
 * and presents it in sysfs.
 */

// External Linux kernel dependencies supplied by the surrounding repository.

pub const EDD_VERSION: &str = "0.16";
pub const EDD_DATE: &str = "2004-Jun-25";

#[repr(C)]
pub struct edd_device {
    pub index: c_uint,
    pub mbr_signature: c_uint,
    pub info: *mut edd_info,
    pub kobj: kobject,
}

#[repr(C)]
pub struct edd_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut edd_device, *mut c_char) -> ssize_t>,
    pub test: Option<unsafe extern "C" fn(*mut edd_device) -> c_int>,
}

static mut EDD_DEVICES: [*mut edd_device; EDD_MBR_SIG_MAX as usize] = [core::ptr::null_mut(); EDD_MBR_SIG_MAX as usize];
static mut edd_kset: *mut kset = core::ptr::null_mut();

unsafe fn edd_has_mbr_signature(edev: *mut edd_device) -> c_int {
    ((*edev).index < core::cmp::min(edd.mbr_signature_nr as u8, EDD_MBR_SIG_MAX as u8) as c_uint) as c_int
}

unsafe fn edd_has_edd_info(edev: *mut edd_device) -> c_int {
    ((*edev).index < core::cmp::min(edd.edd_info_nr as u8, EDDMAXNR as u8) as c_uint) as c_int
}

#[inline]
unsafe fn edd_dev_get_info(edev: *mut edd_device) -> *mut edd_info { (*edev).info }

#[inline]
unsafe fn edd_dev_set_info(edev: *mut edd_device, i: c_int) {
    (*edev).index = i as c_uint;
    if edd_has_mbr_signature(edev) != 0 { (*edev).mbr_signature = edd.mbr_signature[i as usize]; }
    if edd_has_edd_info(edev) != 0 { (*edev).info = &mut edd.edd_info[i as usize]; }
}

unsafe fn edd_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let dev = container_of!(kobj, edd_device, kobj);
    let edd_attr = container_of!(attr, edd_attribute, attr);
    let mut ret = -EIO as ssize_t;
    if let Some(show) = (*edd_attr).show { ret = show(dev, buf); }
    ret
}

static EDD_ATTR_OPS: sysfs_ops = sysfs_ops { show: Some(edd_attr_show) };

unsafe fn edd_show_host_bus(edev: *mut edd_device, buf: *mut c_char) -> ssize_t {
    if edev.is_null() || buf.is_null() { return -EINVAL as ssize_t; }
    let info = edd_dev_get_info(edev); if info.is_null() { return -EINVAL as ssize_t; }
    let mut p = buf; for i in 0..4 { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, if isprint((*info).params.host_bus_type[i] as c_int) != 0 { "%c" } else { " " }, (*info).params.host_bus_type[i] as c_int) as usize); }
    if !strncmp((*info).params.host_bus_type.as_ptr(), b"ISA\0".as_ptr() as *const c_char, 3).eq(&0) {
        p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tbase_address: %x\n", (*info).params.interface_path.isa.base_address) as usize);
    } else if !strncmp((*info).params.host_bus_type.as_ptr(), b"PCIX\0".as_ptr() as *const c_char, 4).eq(&0) || !strncmp((*info).params.host_bus_type.as_ptr(), b"PCI\0".as_ptr() as *const c_char, 3).eq(&0) || !strncmp((*info).params.host_bus_type.as_ptr(), b"XPRS\0".as_ptr() as *const c_char, 4).eq(&0) {
        p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\t%02x:%02x.%d  channel: %u\n", (*info).params.interface_path.pci.bus, (*info).params.interface_path.pci.slot, (*info).params.interface_path.pci.function, (*info).params.interface_path.pci.channel) as usize);
    } else if !strncmp((*info).params.host_bus_type.as_ptr(), b"IBND\0".as_ptr() as *const c_char, 4).eq(&0) || !strncmp((*info).params.host_bus_type.as_ptr(), b"HTPT\0".as_ptr() as *const c_char, 4).eq(&0) {
        p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tTBD: %llx\n", (*info).params.interface_path.ibnd.reserved) as usize);
    } else { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tunknown: %llx\n", (*info).params.interface_path.unknown.reserved) as usize); }
    p.offset_from(buf) as ssize_t
}

unsafe fn edd_show_interface(edev: *mut edd_device, buf: *mut c_char) -> ssize_t {
    if edev.is_null() || buf.is_null() { return -EINVAL as ssize_t; }
    let info = edd_dev_get_info(edev); if info.is_null() { return -EINVAL as ssize_t; }
    let mut p = buf; for i in 0..8 { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, if isprint((*info).params.interface_type[i] as c_int) != 0 { "%c" } else { " " }, (*info).params.interface_type[i] as c_int) as usize); }
    let t = (*info).params.interface_type.as_ptr();
    if !strncmp(t, b"ATAPI\0".as_ptr() as *const c_char, 5).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tdevice: %u  lun: %u\n", (*info).params.device_path.atapi.device, (*info).params.device_path.atapi.lun) as usize); }
    else if !strncmp(t, b"ATA\0".as_ptr() as *const c_char, 3).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tdevice: %u\n", (*info).params.device_path.ata.device) as usize); }
    else if !strncmp(t, b"SCSI\0".as_ptr() as *const c_char, 4).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tid: %u  lun: %llu\n", (*info).params.device_path.scsi.id, (*info).params.device_path.scsi.lun) as usize); }
    else if !strncmp(t, b"USB\0".as_ptr() as *const c_char, 3).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tserial_number: %llx\n", (*info).params.device_path.usb.serial_number) as usize); }
    else if !strncmp(t, b"1394\0".as_ptr() as *const c_char, 4).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\teui: %llx\n", (*info).params.device_path.i1394.eui) as usize); }
    else if !strncmp(t, b"FIBRE\0".as_ptr() as *const c_char, 5).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\twwid: %llx lun: %llx\n", (*info).params.device_path.fibre.wwid, (*info).params.device_path.fibre.lun) as usize); }
    else if !strncmp(t, b"I2O\0".as_ptr() as *const c_char, 3).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tidentity_tag: %llx\n", (*info).params.device_path.i2o.identity_tag) as usize); }
    else if !strncmp(t, b"RAID\0".as_ptr() as *const c_char, 4).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tidentity_tag: %x\n", (*info).params.device_path.raid.array_number) as usize); }
    else if !strncmp(t, b"SATA\0".as_ptr() as *const c_char, 4).eq(&0) { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tdevice: %u\n", (*info).params.device_path.sata.device) as usize); }
    else { p = p.add(scnprintf(p, PAGE_SIZE - p.offset_from(buf) as usize - 1, "\tunknown: %llx %llx\n", (*info).params.device_path.unknown.reserved1, (*info).params.device_path.unknown.reserved2) as usize); }
    p.offset_from(buf) as ssize_t
}

unsafe fn edd_show_raw_data(edev: *mut edd_device, buf: *mut c_char) -> ssize_t {
    if edev.is_null() || buf.is_null() { return -EINVAL as ssize_t; }
    let info = edd_dev_get_info(edev); if info.is_null() { return -EINVAL as ssize_t; }
    let mut len = core::mem::size_of_val(&(*info).params);
    if (*info).params.key != 0xBEDD && (*info).params.key != 0xDDBE { len = (*info).params.length as usize; }
    if len > core::mem::size_of_val(&(*info).params) { len = core::mem::size_of_val(&(*info).params); }
    memcpy(buf as *mut c_void, &(*info).params as *const _ as *const c_void, len); len as ssize_t
}

unsafe fn edd_show_version(edev: *mut edd_device, buf: *mut c_char) -> ssize_t { if edev.is_null() || buf.is_null() || edd_dev_get_info(edev).is_null() { return -EINVAL as ssize_t; } let info=edd_dev_get_info(edev); scnprintf(buf, PAGE_SIZE-1, "0x%02x\n", (*info).version) as ssize_t }
unsafe fn edd_show_mbr_signature(edev: *mut edd_device, buf: *mut c_char) -> ssize_t { if edev.is_null() || buf.is_null() { return -EINVAL as ssize_t; } scnprintf(buf, PAGE_SIZE-1, "0x%08x\n", (*edev).mbr_signature) as ssize_t }

unsafe fn edd_show_extensions(edev: *mut edd_device, buf: *mut c_char) -> ssize_t {
    if edev.is_null() || buf.is_null() || edd_dev_get_info(edev).is_null() { return -EINVAL as ssize_t; }
    let info=edd_dev_get_info(edev); let mut p=buf;
    if (*info).interface_support & EDD_EXT_FIXED_DISK_ACCESS != 0 { p=p.add(scnprintf(p,PAGE_SIZE-p.offset_from(buf) as usize-1,"Fixed disk access\n") as usize); }
    if (*info).interface_support & EDD_EXT_DEVICE_LOCKING_AND_EJECTING != 0 { p=p.add(scnprintf(p,PAGE_SIZE-p.offset_from(buf) as usize-1,"Device locking and ejecting\n") as usize); }
    if (*info).interface_support & EDD_EXT_ENHANCED_DISK_DRIVE_SUPPORT != 0 { p=p.add(scnprintf(p,PAGE_SIZE-p.offset_from(buf) as usize-1,"Enhanced Disk Drive support\n") as usize); }
    if (*info).interface_support & EDD_EXT_64BIT_EXTENSIONS != 0 { p=p.add(scnprintf(p,PAGE_SIZE-p.offset_from(buf) as usize-1,"64-bit extensions\n") as usize); } p.offset_from(buf) as ssize_t
}

unsafe fn edd_show_info_flags(edev: *mut edd_device, buf: *mut c_char) -> ssize_t {
    if edev.is_null() || buf.is_null() || edd_dev_get_info(edev).is_null() { return -EINVAL as ssize_t; }
    let info=edd_dev_get_info(edev); let mut p=buf;
    macro_rules! flag { ($f:expr,$s:expr) => { if (*info).params.info_flags & $f != 0 { p=p.add(scnprintf(p,PAGE_SIZE-p.offset_from(buf) as usize-1,$s) as usize); } }; }
    flag!(EDD_INFO_DMA_BOUNDARY_ERROR_TRANSPARENT,"DMA boundary error transparent\n"); flag!(EDD_INFO_GEOMETRY_VALID,"geometry valid\n"); flag!(EDD_INFO_REMOVABLE,"removable\n"); flag!(EDD_INFO_WRITE_VERIFY,"write verify\n"); flag!(EDD_INFO_MEDIA_CHANGE_NOTIFICATION,"media change notification\n"); flag!(EDD_INFO_LOCKABLE,"lockable\n"); flag!(EDD_INFO_NO_MEDIA_PRESENT,"no media present\n"); flag!(EDD_INFO_USE_INT13_FN50,"use int13 fn50\n"); p.offset_from(buf) as ssize_t
}

macro_rules! simple_show { ($name:ident,$field:expr,$fmt:expr) => { unsafe fn $name(edev:*mut edd_device,buf:*mut c_char)->ssize_t { if edev.is_null()||buf.is_null()||edd_dev_get_info(edev).is_null(){return -EINVAL as ssize_t;} scnprintf(buf,PAGE_SIZE-1,$fmt,$field(edd_dev_get_info(edev))) as ssize_t } }; }
simple_show!(edd_show_legacy_max_cylinder, |i:*mut edd_info| (*i).legacy_max_cylinder, "%u\n");
simple_show!(edd_show_legacy_max_head, |i:*mut edd_info| (*i).legacy_max_head, "%u\n");
simple_show!(edd_show_legacy_sectors_per_track, |i:*mut edd_info| (*i).legacy_sectors_per_track, "%u\n");
simple_show!(edd_show_default_cylinders, |i:*mut edd_info| (*i).params.num_default_cylinders, "%u\n");
simple_show!(edd_show_default_heads, |i:*mut edd_info| (*i).params.num_default_heads, "%u\n");
simple_show!(edd_show_default_sectors_per_track, |i:*mut edd_info| (*i).params.sectors_per_track, "%u\n");
simple_show!(edd_show_sectors, |i:*mut edd_info| (*i).params.number_of_sectors, "%llu\n");

macro_rules! has_field { ($name:ident,$field:expr) => { unsafe fn $name(edev:*mut edd_device)->c_int { if edev.is_null()||edd_dev_get_info(edev).is_null(){0}else{($field(edd_dev_get_info(edev))>0) as c_int} } }; }
has_field!(edd_has_legacy_max_cylinder,|i:*mut edd_info|(*i).legacy_max_cylinder); has_field!(edd_has_legacy_max_head,|i:*mut edd_info|(*i).legacy_max_head); has_field!(edd_has_legacy_sectors_per_track,|i:*mut edd_info|(*i).legacy_sectors_per_track); has_field!(edd_has_default_cylinders,|i:*mut edd_info|(*i).params.num_default_cylinders); has_field!(edd_has_default_heads,|i:*mut edd_info|(*i).params.num_default_heads); has_field!(edd_has_default_sectors_per_track,|i:*mut edd_info|(*i).params.sectors_per_track);

unsafe fn edd_has_edd30(edev:*mut edd_device)->c_int { if edev.is_null()||edd_dev_get_info(edev).is_null(){return 0;} let i=edd_dev_get_info(edev); if (*i).params.key!=0xBEDD&&(*i).params.key!=0xDDBE{return 0;} if (*i).params.device_path_info_length!=44{return 0;} let mut c: u8=0; for n in 30..((*i).params.device_path_info_length as usize+30){c=c.wrapping_add(*((&(*i).params as *const _ as *const u8).add(n));} (c==0) as c_int }

unsafe fn edd_release(kobj:*mut kobject){ kfree(container_of!(kobj,edd_device,kobj) as *mut c_void); }
unsafe fn edd_dev_is_type(edev:*mut edd_device, typ:*const c_char)->c_int { if edev.is_null(){return 0;} let i=edd_dev_get_info(edev); if !typ.is_null()&&!i.is_null() && (strncmp((*i).params.host_bus_type.as_ptr(),typ,strlen(typ))==0 || strncmp((*i).params.interface_type.as_ptr(),typ,strlen(typ))==0){1}else{0} }

unsafe fn edd_get_pci_dev(edev:*mut edd_device)->*mut pci_dev { let i=edd_dev_get_info(edev); if edd_dev_is_type(edev,b"PCI\0".as_ptr() as *const c_char)!=0||edd_dev_is_type(edev,b"XPRS\0".as_ptr() as *const c_char)!=0 { pci_get_domain_bus_and_slot(0,(*i).params.interface_path.pci.bus,PCI_DEVFN((*i).params.interface_path.pci.slot,(*i).params.interface_path.pci.function)) } else {core::ptr::null_mut()} }
unsafe fn edd_create_symlink_to_pcidev(edev:*mut edd_device)->c_int { let p=edd_get_pci_dev(edev); if p.is_null(){return 1;} let r=sysfs_create_link(&mut (*edev).kobj,&mut (*p).dev.kobj,b"pci_dev\0".as_ptr() as *const c_char); pci_dev_put(p); r }
unsafe fn edd_device_unregister(edev:*mut edd_device){kobject_put(&mut (*edev).kobj);}
unsafe fn edd_populate_dir(edev:*mut edd_device){let mut i=0;let mut e=0;while !edd_attrs[i].is_null()&&e==0{let a=edd_attrs[i];if (*a).test.is_none()||((*a).test.unwrap())(edev)!=0{e=sysfs_create_file(&mut (*edev).kobj,&mut (*a).attr);}i+=1;}if e==0{edd_create_symlink_to_pcidev(edev);}}
unsafe fn edd_num_devices()->c_int{core::cmp::max(core::cmp::min(EDD_MBR_SIG_MAX as u8,edd.mbr_signature_nr as u8),core::cmp::min(EDDMAXNR as u8,edd.edd_info_nr as u8)) as c_int}
unsafe extern "C" fn edd_init()->c_int{if edd_num_devices()==0{return -ENODEV;} edd_kset=kset_create_and_add(b"edd\0".as_ptr() as *const c_char,core::ptr::null_mut(),firmware_kobj);if edd_kset.is_null(){return -ENOMEM;}for i in 0..edd_num_devices(){let d=kzalloc(core::mem::size_of::<edd_device>(),GFP_KERNEL) as *mut edd_device;if d.is_null(){return -ENOMEM;}edd_dev_set_info(d,i);(*d).kobj.kset=edd_kset;let r=kobject_init_and_add(&mut (*d).kobj,&EDD_KTYPE,core::ptr::null_mut(),b"int13_dev%02x\0".as_ptr() as *const c_char,0x80+i);if r!=0{kfree(d as *mut c_void);return r;}edd_populate_dir(d);kobject_uevent(&mut (*d).kobj,KOBJ_ADD);EDD_DEVICES[i as usize]=d;}0}
unsafe extern "C" fn edd_exit(){for i in 0..edd_num_devices(){let d=EDD_DEVICES[i as usize];if !d.is_null(){edd_device_unregister(d);}}kset_unregister(edd_kset);}

macro_rules! edd_device_attr { ($name:ident,$show:ident,$test:ident) => { static mut $name: edd_attribute = edd_attribute { attr: attribute { name: stringify!($name).as_ptr() as *const c_char, mode: 0o444 }, show: Some($show), test: Some($test) }; }; }
edd_device_attr!(edd_attr_raw_data, edd_show_raw_data, edd_has_edd_info);
edd_device_attr!(edd_attr_version, edd_show_version, edd_has_edd_info);
edd_device_attr!(edd_attr_extensions, edd_show_extensions, edd_has_edd_info);
edd_device_attr!(edd_attr_info_flags, edd_show_info_flags, edd_has_edd_info);
edd_device_attr!(edd_attr_sectors, edd_show_sectors, edd_has_edd_info);
edd_device_attr!(edd_attr_legacy_max_cylinder, edd_show_legacy_max_cylinder, edd_has_legacy_max_cylinder);
edd_device_attr!(edd_attr_legacy_max_head, edd_show_legacy_max_head, edd_has_legacy_max_head);
edd_device_attr!(edd_attr_legacy_sectors_per_track, edd_show_legacy_sectors_per_track, edd_has_legacy_sectors_per_track);
edd_device_attr!(edd_attr_default_cylinders, edd_show_default_cylinders, edd_has_default_cylinders);
edd_device_attr!(edd_attr_default_heads, edd_show_default_heads, edd_has_default_heads);
edd_device_attr!(edd_attr_default_sectors_per_track, edd_show_default_sectors_per_track, edd_has_default_sectors_per_track);
edd_device_attr!(edd_attr_interface, edd_show_interface, edd_has_edd30);
edd_device_attr!(edd_attr_host_bus, edd_show_host_bus, edd_has_edd30);
edd_device_attr!(edd_attr_mbr_signature, edd_show_mbr_signature, edd_has_mbr_signature);

static mut edd_attrs: [*mut edd_attribute; 15] = [
    &raw mut edd_attr_raw_data, &raw mut edd_attr_version, &raw mut edd_attr_extensions,
    &raw mut edd_attr_info_flags, &raw mut edd_attr_sectors, &raw mut edd_attr_legacy_max_cylinder,
    &raw mut edd_attr_legacy_max_head, &raw mut edd_attr_legacy_sectors_per_track,
    &raw mut edd_attr_default_cylinders, &raw mut edd_attr_default_heads,
    &raw mut edd_attr_default_sectors_per_track, &raw mut edd_attr_interface,
    &raw mut edd_attr_host_bus, &raw mut edd_attr_mbr_signature, core::ptr::null_mut()
];

static EDD_KTYPE:kobj_type=kobj_type{release:Some(edd_release),sysfs_ops:&EDD_ATTR_OPS};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
