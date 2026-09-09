// SPDX-License-Identifier: GPL-2.0+
/*
 * A hack to create a platform device from a DMI entry.  This will
 * allow autoloading of the IPMI drive based on SMBIOS entries.
 */

// Dependency declarations and build-time kernel configuration are supplied by
// the surrounding kernel translation unit.

const IPMI_DMI_TYPE_KCS: i32 = 0x01;
const IPMI_DMI_TYPE_SMIC: i32 = 0x02;
const IPMI_DMI_TYPE_BT: i32 = 0x03;
const IPMI_DMI_TYPE_SSIF: i32 = 0x04;

#[repr(C)]
struct IpmiDmiInfo {
    si_type: SiType,
    space: u32, /* addr space for si, intf# for ssif */
    addr: usize,
    slave_addr: u8,
    next: *mut IpmiDmiInfo,
}

static mut IPMI_DMI_INFOS: *mut IpmiDmiInfo = core::ptr::null_mut();
static mut IPMI_DMI_NR: i32 = 0;

unsafe fn dmi_add_platform_ipmi(
    base_addr: usize,
    space: u32,
    slave_addr: u8,
    irq: i32,
    offset: i32,
    type_: i32,
) {
    let mut p: IpmiPlatData = core::mem::zeroed();
    let mut name = "dmi-ipmi-si";
    p.iftype = IPMI_PLAT_IF_SI;
    match type_ {
        IPMI_DMI_TYPE_SSIF => {
            name = "dmi-ipmi-ssif";
            p.iftype = IPMI_PLAT_IF_SSIF;
            p.type_ = SI_TYPE_INVALID;
        }
        IPMI_DMI_TYPE_BT => p.type_ = SI_BT,
        IPMI_DMI_TYPE_KCS => p.type_ = SI_KCS,
        IPMI_DMI_TYPE_SMIC => p.type_ = SI_SMIC,
        _ => {
            pr_err!("Invalid IPMI type: {}\n", type_);
            return;
        }
    }

    p.addr = base_addr;
    p.space = space;
    p.regspacing = offset;
    p.irq = irq;
    p.slave_addr = slave_addr;
    p.addr_source = SI_SMBIOS;

    let info = Box::into_raw(Box::new(IpmiDmiInfo {
        si_type: p.type_,
        space,
        addr: base_addr,
        slave_addr,
        next: IPMI_DMI_INFOS,
    }));
    if info.is_null() {
        pr_warn!("Could not allocate dmi info\n");
    } else {
        IPMI_DMI_INFOS = info;
    }

    if ipmi_platform_add(name, IPMI_DMI_NR, &p) != 0 {
        IPMI_DMI_NR += 1;
    }
}

/*
 * Look up the slave address for a given interface.  This is here
 * because ACPI doesn't have a slave address while SMBIOS does, but we
 * prefer using ACPI so the ACPI code can use the IPMI namespace.
 * This function allows an ACPI-specified IPMI device to look up the
 * slave address from the DMI table.
 */
pub unsafe fn ipmi_dmi_get_slave_addr(
    si_type: SiType,
    space: u32,
    base_addr: usize,
) -> u8 {
    let mut info = IPMI_DMI_INFOS;
    while !info.is_null() {
        if (*info).si_type == si_type && (*info).space == space && (*info).addr == base_addr {
            return (*info).slave_addr;
        }
        info = (*info).next;
    }
    0
}

const DMI_IPMI_MIN_LENGTH: u8 = 0x10;
const DMI_IPMI_VER2_LENGTH: u8 = 0x12;
const DMI_IPMI_TYPE: usize = 4;
const DMI_IPMI_SLAVEADDR: usize = 6;
const DMI_IPMI_ADDR: usize = 8;
const DMI_IPMI_ACCESS: usize = 0x10;
const DMI_IPMI_IRQ: usize = 0x11;
const DMI_IPMI_IO_MASK: usize = 0xfffe;

unsafe fn dmi_decode_ipmi(dm: *const DmiHeader) {
    let data = dm as *const u8;
    let mut space: i32 = IPMI_IO_ADDR_SPACE;
    let mut base_addr: usize = 0;
    let len = (*dm).length;
    let mut slave_addr = *data.add(DMI_IPMI_SLAVEADDR);
    let mut irq = 0i32;
    let mut offset = 0i32;
    let type_ = *data.add(DMI_IPMI_TYPE) as i32;

    if len < DMI_IPMI_MIN_LENGTH { return; }
    core::ptr::copy_nonoverlapping(data.add(DMI_IPMI_ADDR), &mut base_addr as *mut usize as *mut u8, core::mem::size_of::<usize>());
    if base_addr == 0 {
        pr_err!("Base address is zero, assuming no IPMI interface\n");
        return;
    }
    if len >= DMI_IPMI_VER2_LENGTH {
        if type_ == IPMI_DMI_TYPE_SSIF {
            space = 0;
            base_addr = (*data.add(DMI_IPMI_ADDR) as usize) >> 1;
            if base_addr == 0 {
                base_addr = (*data.add(DMI_IPMI_SLAVEADDR) as usize) >> 1;
                slave_addr = 0;
            }
        } else {
            if base_addr & 1 != 0 { base_addr &= DMI_IPMI_IO_MASK; } else { space = IPMI_MEM_ADDR_SPACE; }
            base_addr |= ((*data.add(DMI_IPMI_ACCESS) as usize >> 4) & 1);
            irq = *data.add(DMI_IPMI_IRQ) as i32;
            match (*data.add(DMI_IPMI_ACCESS) >> 6) & 3 {
                0 => offset = 1,
                1 => offset = 4,
                2 => offset = 16,
                _ => { pr_err!("Invalid offset: 0\n"); return; }
            }
        }
    } else {
        base_addr &= DMI_IPMI_IO_MASK;
        offset = 1;
    }
    dmi_add_platform_ipmi(base_addr, space as u32, slave_addr, irq, offset, type_);
}

unsafe fn scan_for_dmi_ipmi() -> i32 {
    let mut dev: *const DmiDevice = core::ptr::null();
    while { dev = dmi_find_device(DMI_DEV_TYPE_IPMI, core::ptr::null(), dev); !dev.is_null() } {
        dmi_decode_ipmi((*dev).device_data as *const DmiHeader);
    }
    0
}

// subsys_initcall(scan_for_dmi_ipmi);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
