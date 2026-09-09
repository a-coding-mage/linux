// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.com, 1998-1999
    (c) Philip Blundell, 1998, 2001

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// Dependencies supplied by fpa11.h, softfloat.h, fpopcode.h, fpmodule.h,
// fpmodule.inl, and linux/uaccess.h remain external to this translation.

unsafe fn loadSingle(fn_: u32, p_mem: *const u32) {
    let fpa11 = GET_FPA11();
    (*fpa11).fType[fn_ as usize] = typeSingle;
    (*fpa11).fpreg[fn_ as usize].fSingle = p_mem.read_volatile();
}

unsafe fn loadDouble(fn_: u32, p_mem: *const u32) {
    let fpa11 = GET_FPA11();
    let p = &mut (*fpa11).fpreg[fn_ as usize].fDouble as *mut _ as *mut u32;
    (*fpa11).fType[fn_ as usize] = typeDouble;
    #[cfg(target_endian = "big")]
    {
        *p.add(0) = p_mem.add(0).read_volatile();
        *p.add(1) = p_mem.add(1).read_volatile();
    }
    #[cfg(target_endian = "little")]
    {
        *p.add(0) = p_mem.add(1).read_volatile();
        *p.add(1) = p_mem.add(0).read_volatile();
    }
}

#[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
unsafe fn loadExtended(fn_: u32, p_mem: *const u32) {
    let fpa11 = GET_FPA11();
    let p = &mut (*fpa11).fpreg[fn_ as usize].fExtended as *mut _ as *mut u32;
    (*fpa11).fType[fn_ as usize] = typeExtended;
    *p.add(0) = p_mem.add(0).read_volatile();
    #[cfg(target_endian = "big")]
    { *p.add(1) = p_mem.add(1).read_volatile(); *p.add(2) = p_mem.add(2).read_volatile(); }
    #[cfg(target_endian = "little")]
    { *p.add(1) = p_mem.add(2).read_volatile(); *p.add(2) = p_mem.add(1).read_volatile(); }
}

unsafe fn loadMultiple(fn_: u32, p_mem: *const u32) {
    let fpa11 = GET_FPA11();
    let p = &mut (*fpa11).fpreg[fn_ as usize] as *mut _ as *mut u32;
    let x = p_mem.read_volatile() as usize;
    (*fpa11).fType[fn_ as usize] = ((x >> 14) & 3) as _;
    match (*fpa11).fType[fn_ as usize] {
        typeSingle | typeDouble => { *p.add(0) = p_mem.add(2).read_volatile(); *p.add(1) = p_mem.add(1).read_volatile(); *p.add(2) = 0; }
        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        typeExtended => { *p.add(1) = p_mem.add(2).read_volatile(); *p.add(2) = p_mem.add(1).read_volatile(); *p.add(0) = (x as u32) & 0x80003fff; }
        _ => {}
    }
}

unsafe fn storeSingle(round_data: *mut roundingData, fn_: u32, p_mem: *mut u32) {
    let fpa11 = GET_FPA11();
    let mut val: u32;
    match (*fpa11).fType[fn_ as usize] {
        typeDouble => { val = float64_to_float32(round_data, (*fpa11).fpreg[fn_ as usize].fDouble); }
        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        typeExtended => { val = floatx80_to_float32(round_data, (*fpa11).fpreg[fn_ as usize].fExtended); }
        _ => { val = (*fpa11).fpreg[fn_ as usize].fSingle; }
    }
    p_mem.write_volatile(val);
}

unsafe fn storeDouble(round_data: *mut roundingData, fn_: u32, p_mem: *mut u32) {
    let fpa11 = GET_FPA11();
    let mut val = [0u32; 2];
    match (*fpa11).fType[fn_ as usize] {
        typeSingle => { val = float32_to_float64((*fpa11).fpreg[fn_ as usize].fSingle); }
        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        typeExtended => { val = floatx80_to_float64(round_data, (*fpa11).fpreg[fn_ as usize].fExtended); }
        _ => { val = (*fpa11).fpreg[fn_ as usize].fDouble; }
    }
    #[cfg(target_endian = "big")]
    { p_mem.add(0).write_volatile(val[0]); p_mem.add(1).write_volatile(val[1]); }
    #[cfg(target_endian = "little")]
    { p_mem.add(0).write_volatile(val[1]); p_mem.add(1).write_volatile(val[0]); }
}

// The extended store is conditional in the original CONFIG_FPE_NWFPE_XP build.
#[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
unsafe fn storeExtended(fn_: u32, p_mem: *mut u32) {
    let fpa11 = GET_FPA11();
    let mut val = [0u32; 3];
    match (*fpa11).fType[fn_ as usize] {
        typeSingle => { val = float32_to_floatx80((*fpa11).fpreg[fn_ as usize].fSingle); }
        typeDouble => { val = float64_to_floatx80((*fpa11).fpreg[fn_ as usize].fDouble); }
        _ => { val = (*fpa11).fpreg[fn_ as usize].fExtended; }
    }
    p_mem.write_volatile(val[0]);
    #[cfg(target_endian = "big")]
    { p_mem.add(1).write_volatile(val[1]); p_mem.add(2).write_volatile(val[2]); }
    #[cfg(target_endian = "little")]
    { p_mem.add(1).write_volatile(val[2]); p_mem.add(2).write_volatile(val[1]); }
}

unsafe fn storeMultiple(fn_: u32, p_mem: *mut u32) {
    let fpa11 = GET_FPA11();
    let p = &(*fpa11).fpreg[fn_ as usize] as *const _ as *const u32;
    let n_type = (*fpa11).fType[fn_ as usize];
    match n_type {
        typeSingle | typeDouble => { p_mem.add(2).write_volatile(p.read()); p_mem.add(1).write_volatile(p.add(1).read()); p_mem.write_volatile((n_type as u32) << 14); }
        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        typeExtended => { p_mem.add(1).write_volatile(p.add(2).read()); p_mem.add(2).write_volatile(p.add(1).read()); p_mem.write_volatile((p.read() & 0x80003fff) | ((n_type as u32) << 14)); }
        _ => {}
    }
}

pub unsafe fn PerformLDF(opcode: u32) -> u32 {
    let mut p_base = readRegister(getRn(opcode)) as *mut u32;
    let mut write_back = WRITE_BACK(opcode);
    if REG_PC == getRn(opcode) { p_base = p_base.add(2); write_back = 0; }
    let p_final = if BIT_UP_SET(opcode) { p_base.add(getOffset(opcode) as usize) } else { p_base.sub(getOffset(opcode) as usize) };
    let p_address = if PREINDEXED(opcode) { p_final } else { p_base };
    let mut rc = 1;
    match opcode & MASK_TRANSFER_LENGTH { TRANSFER_SINGLE => loadSingle(getFd(opcode), p_address), TRANSFER_DOUBLE => loadDouble(getFd(opcode), p_address), #[cfg(feature = "CONFIG_FPE_NWFPE_XP")] TRANSFER_EXTENDED => loadExtended(getFd(opcode), p_address), _ => rc = 0 }
    if write_back != 0 { writeRegister(getRn(opcode), p_final as usize as _); } rc
}

pub unsafe fn PerformSTF(opcode: u32) -> u32 {
    let mut rd = roundingData { mode: SetRoundingMode(opcode), precision: SetRoundingPrecision(opcode), exception: 0 };
    let mut p_base = readRegister(getRn(opcode)) as *mut u32; let mut wb = WRITE_BACK(opcode);
    if REG_PC == getRn(opcode) { p_base = p_base.add(2); wb = 0; }
    let p_final = if BIT_UP_SET(opcode) { p_base.add(getOffset(opcode) as usize) } else { p_base.sub(getOffset(opcode) as usize) }; let p = if PREINDEXED(opcode) { p_final } else { p_base }; let mut rc=1;
    match opcode & MASK_TRANSFER_LENGTH { TRANSFER_SINGLE=>storeSingle(&mut rd,getFd(opcode),p), TRANSFER_DOUBLE=>storeDouble(&mut rd,getFd(opcode),p), #[cfg(feature="CONFIG_FPE_NWFPE_XP")] TRANSFER_EXTENDED=>storeExtended(getFd(opcode),p), _=>rc=0 }
    if rd.exception != 0 { float_raise(rd.exception); } if wb != 0 { writeRegister(getRn(opcode),p_final as usize as _); } rc
}

unsafe fn perform_multiple(opcode: u32, store: bool) -> u32 {
    let mut base=readRegister(getRn(opcode)) as *mut u32; let mut wb=WRITE_BACK(opcode); if REG_PC==getRn(opcode){base=base.add(2);wb=0;}
    let final_=if BIT_UP_SET(opcode){base.add(getOffset(opcode) as usize)}else{base.sub(getOffset(opcode) as usize)}; let mut p=if PREINDEXED(opcode){final_}else{base}; let mut fd=getFd(opcode); let mut i=getRegisterCount(opcode);
    while i>0 { if store {storeMultiple(fd,p)} else {loadMultiple(fd,p)} p=p.add(3); fd+=1; if fd==8{fd=0;} i-=1; } if wb!=0{writeRegister(getRn(opcode),final_ as usize as _);} 1
}
pub unsafe fn PerformLFM(opcode:u32)->u32{perform_multiple(opcode,false)}
pub unsafe fn PerformSFM(opcode:u32)->u32{perform_multiple(opcode,true)}
pub unsafe fn EmulateCPDT(opcode:u32)->u32 { if LDF_OP(opcode){PerformLDF(opcode)}else if LFM_OP(opcode){PerformLFM(opcode)}else if STF_OP(opcode){PerformSTF(opcode)}else if SFM_OP(opcode){PerformSFM(opcode)}else{0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
