// SPDX-License-Identifier: GPL-2.0-only
/* several functions that help interpret ARC instructions */

// Linux dependencies and configuration conditions are supplied by the surrounding kernel.

#[cfg(any(feature = "CONFIG_KGDB", feature = "CONFIG_ARC_EMUL_UNALIGNED", feature = "CONFIG_KPROBES"))]
pub unsafe fn disasm_instr(addr: usize, state: *mut disasm_state, userspace: i32,
                           regs: *mut pt_regs, cregs: *mut callee_regs) {
    let mut field_a: i32 = 0;
    let mut field_c: i32 = 0;
    let mut field_c_is_reg = 0;
    let mut word1: u16 = 0;
    let mut word0: u16 = 0;
    let mut subopcode: i32;
    let mut is_linked: i32;
    let mut op_format: i32;
    let mut ins_ptr: *mut u16;
    let mut ins_buf = [0u16; 4];
    let mut bytes_not_copied = 0i32;

    core::ptr::write_bytes(state, 0, 1);
    if userspace != 0 {
        bytes_not_copied = copy_from_user(ins_buf.as_mut_ptr() as *mut _, addr as *const _, 8) as i32;
        if bytes_not_copied > 6 { (*state).fault = 1; return; }
        ins_ptr = ins_buf.as_mut_ptr();
    } else { ins_ptr = addr as *mut u16; }
    let _ = ins_ptr;

    word1 = *(addr as *const u16);
    (*state).major_opcode = ((word1 >> 11) & 0x1f) as _;
    if (*state).major_opcode < 0x0b {
        if bytes_not_copied > 4 { (*state).fault = 1; return; }
        (*state).instr_len = 4;
        word0 = *((addr + 2) as *const u16);
        (*state).words[0] = ((word1 as u32) << 16) | word0 as u32;
    } else {
        (*state).instr_len = 2;
        (*state).words[0] = word1 as u32;
    }
    word1 = *((addr + (*state).instr_len as usize) as *const u16);
    word0 = *((addr + (*state).instr_len as usize + 2) as *const u16);
    (*state).words[1] = ((word1 as u32) << 16) | word0 as u32;

    match (*state).major_opcode {
        op_Bcc => {
            (*state).is_branch = 1;
            field_a = if IS_BIT((*state).words[0], 16) { FIELD_s25((*state).words[0]) } else { FIELD_s21((*state).words[0]) };
            (*state).delay_slot = IS_BIT((*state).words[0], 5);
            (*state).target = field_a as _ + (addr & !3) as _;
            (*state).flow = direct_jump;
        }
        op_BLcc => {
            if IS_BIT((*state).words[0], 16) {
                field_a = if IS_BIT((*state).words[0], 17) { FIELD_s25((*state).words[0]) & !3 } else { FIELD_s21((*state).words[0]) };
                (*state).flow = direct_call;
            } else { field_a = FIELD_s9((*state).words[0]) & !3; (*state).flow = direct_jump; }
            (*state).delay_slot = IS_BIT((*state).words[0], 5);
            (*state).target = field_a as _ + (addr & !3) as _; (*state).is_branch = 1;
        }
        op_LD => {
            (*state).write = 0; (*state).di = BITS((*state).words[0],11,11); if (*state).di != 0 { return; }
            (*state).x = BITS((*state).words[0],6,6); (*state).zz = BITS((*state).words[0],7,8); (*state).aa = BITS((*state).words[0],9,10);
            (*state).wb_reg = FIELD_B((*state).words[0]);
            if (*state).wb_reg == REG_LIMM { (*state).instr_len += 4; (*state).aa=0; (*state).src1=(*state).words[1] as _; } else { (*state).src1=get_reg((*state).wb_reg,regs,cregs); }
            (*state).src2=FIELD_s9((*state).words[0]); (*state).dest=FIELD_A((*state).words[0]); (*state).pref=((*state).dest==REG_LIMM) as _;
        }
        op_ST => {
            (*state).write=1; (*state).di=BITS((*state).words[0],5,5); if (*state).di != 0 { return; }
            (*state).aa=BITS((*state).words[0],3,4); (*state).zz=BITS((*state).words[0],1,2); (*state).src1=FIELD_C((*state).words[0]);
            if (*state).src1==REG_LIMM { (*state).instr_len+=4; (*state).src1=(*state).words[1] as _; } else { (*state).src1=get_reg((*state).src1,regs,cregs); }
            (*state).wb_reg=FIELD_B((*state).words[0]); if (*state).wb_reg==REG_LIMM { (*state).aa=0; (*state).instr_len+=4; (*state).src2=(*state).words[1] as _; } else { (*state).src2=get_reg((*state).wb_reg,regs,cregs); }
            (*state).src3=FIELD_s9((*state).words[0]);
        }
        op_MAJOR_4 => {
            subopcode=MINOR_OPCODE((*state).words[0]);
            match subopcode {
                32|33|34|35 => { is_linked=0; if subopcode==33||subopcode==35 {(*state).delay_slot=1;} if subopcode==34||subopcode==35 {is_linked=1;}
                    op_format=BITS((*state).words[0],22,23); field_c_is_reg=0;
                    if op_format==0 || (op_format==3 && !IS_BIT((*state).words[0],5)) { field_c=FIELD_C((*state).words[0]); if field_c==REG_LIMM {field_c=(*state).words[1] as _; (*state).instr_len+=4;} else {field_c_is_reg=1;} }
                    else if op_format==1 || (op_format==3 && IS_BIT((*state).words[0],5)) {field_c=FIELD_C((*state).words[0]);} else {field_c=FIELD_s12((*state).words[0]);}
                    (*state).target=if field_c_is_reg!=0 {get_reg(field_c,regs,cregs)} else {field_c as _}; (*state).flow=if is_linked!=0 {if field_c_is_reg!=0 {indirect_call} else {direct_call}} else {if field_c_is_reg!=0 {indirect_jump} else {direct_jump}}; (*state).is_branch=1;
                }
                40 => if BITS((*state).words[0],22,23)==3 {field_c=FIELD_C((*state).words[0]); field_c=(field_c<<1)+((addr&!3) as i32); (*state).is_branch=1; (*state).flow=direct_jump; (*state).target=field_c as _;}
                48..=55 => { (*state).di=BITS((*state).words[0],15,15); if (*state).di!=0{return;} (*state).x=BITS((*state).words[0],16,16); (*state).zz=BITS((*state).words[0],17,18); (*state).aa=BITS((*state).words[0],22,23); (*state).wb_reg=FIELD_B((*state).words[0]); if (*state).wb_reg==REG_LIMM {(*state).instr_len+=4;(*state).src1=(*state).words[1] as _;} else {(*state).src1=get_reg((*state).wb_reg,regs,cregs);} (*state).src2=FIELD_C((*state).words[0]); if (*state).src2==REG_LIMM {(*state).instr_len+=4;(*state).src2=(*state).words[1] as _;} else {(*state).src2=get_reg((*state).src2,regs,cregs);} (*state).dest=FIELD_A((*state).words[0]); (*state).pref=((*state).dest==REG_LIMM) as _; }
                10 => { let f=BITS((*state).words[0],22,23); if f==0 && FIELD_C((*state).words[0])==REG_LIMM {(*state).instr_len+=4;} if f==3 && !IS_BIT((*state).words[0],5) && FIELD_C((*state).words[0])==REG_LIMM {(*state).instr_len+=4;} }
                _ => { let f=BITS((*state).words[0],22,23); if f==0 && (FIELD_B((*state).words[0])==REG_LIMM||FIELD_C((*state).words[0])==REG_LIMM) {(*state).instr_len+=4;} if f==3 && !IS_BIT((*state).words[0],5) && (FIELD_B((*state).words[0])==REG_LIMM||FIELD_C((*state).words[0])==REG_LIMM) {(*state).instr_len+=4;} }
            }
        }
        op_LD_ADD => { (*state).zz=BITS((*state).words[0],3,4); (*state).src1=get_reg(FIELD_S_B((*state).words[0]),regs,cregs); (*state).src2=get_reg(FIELD_S_C((*state).words[0]),regs,cregs); (*state).dest=FIELD_S_A((*state).words[0]); }
        op_ADD_MOV_CMP => if BITS((*state).words[0],3,4)<3 && FIELD_S_H((*state).words[0])==REG_LIMM {(*state).instr_len+=4;}
        op_S => { subopcode=BITS((*state).words[0],5,7); match subopcode {0..=3=>{(*state).target=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).delay_slot=subopcode&1;(*state).flow=if subopcode>=2{direct_call}else{indirect_jump};},7=>match BITS((*state).words[0],8,10){4..=7=>{(*state).delay_slot=(subopcode==7) as _;(*state).flow=indirect_jump;(*state).target=get_reg(31,regs,cregs);},_=>{}},_=>{}} }
        op_LD_S => {(*state).src1=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src2=FIELD_S_u7((*state).words[0]);(*state).dest=FIELD_S_C((*state).words[0]);}
        op_LDB_S|op_STB_S => (*state).zz=1,
        op_LDWX_S => {(*state).x=1;(*state).zz=2;(*state).src1=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src2=FIELD_S_u6((*state).words[0]);(*state).dest=FIELD_S_C((*state).words[0]);}
        op_LDW_S => {(*state).zz=2;(*state).src1=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src2=FIELD_S_u6((*state).words[0]);(*state).dest=FIELD_S_C((*state).words[0]);}
        op_ST_S => {(*state).write=1;(*state).src1=get_reg(FIELD_S_C((*state).words[0]),regs,cregs);(*state).src2=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src3=FIELD_S_u7((*state).words[0]);}
        op_STW_S => {(*state).write=1;(*state).zz=2;(*state).src1=get_reg(FIELD_S_C((*state).words[0]),regs,cregs);(*state).src2=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src3=FIELD_S_u6((*state).words[0]);}
        op_SP => {(*state).write=BITS((*state).words[0],6,6);(*state).zz=BITS((*state).words[0],5,5);if (*state).zz!=0{return;} if (*state).write==0 {(*state).src1=get_reg(28,regs,cregs);(*state).src2=FIELD_S_u7((*state).words[0]);(*state).dest=FIELD_S_B((*state).words[0]);} else {(*state).src1=get_reg(FIELD_S_B((*state).words[0]),regs,cregs);(*state).src2=get_reg(28,regs,cregs);(*state).src3=FIELD_S_u7((*state).words[0]);}}
        op_GP => {(*state).zz=BITS((*state).words[0],9,10);(*state).src1=get_reg(26,regs,cregs);(*state).src2=if (*state).zz!=0 {FIELD_S_s10((*state).words[0])} else {FIELD_S_s11((*state).words[0])};(*state).dest=0;}
        op_Pcl => {(*state).src1=(*regs).ret & !3;(*state).src2=FIELD_S_u10((*state).words[0]);(*state).dest=FIELD_S_B((*state).words[0]);}
        op_BR_S => {(*state).target=FIELD_S_s8((*state).words[0])+(addr&!3) as _;(*state).flow=direct_jump;(*state).is_branch=1;}
        op_B_S => {field_a=if BITS((*state).words[0],9,10)==3{FIELD_S_s7((*state).words[0])}else{FIELD_S_s10((*state).words[0])};(*state).target=field_a as _+(addr&!3) as _;(*state).flow=direct_jump;(*state).is_branch=1;}
        op_BL_S => {(*state).target=FIELD_S_s13((*state).words[0])+(addr&!3) as _;(*state).flow=direct_call;(*state).is_branch=1;}
        _=>{}
    }
    if bytes_not_copied > 8-(*state).instr_len {(*state).fault=1;}
}

pub unsafe fn get_reg(reg: i32, regs: *mut pt_regs, cregs: *mut callee_regs) -> i64 {
    if reg <= 11 { return *(&(*regs).r0 as *const _ as *const i64).add(reg as usize); }
    if reg==12{return (*regs).r12;} if reg==30{return (*regs).r30;}
    if !cregs.is_null() && reg<=25{return *(&(*cregs).r13 as *const _ as *const i64).sub((reg-13) as usize);}
    match reg {26 => (*regs).r26,27 => (*regs).fp,28 => (*regs).sp,31 => (*regs).blink,_=>0}
}

pub unsafe fn set_reg(reg:i32,val:i64,regs:*mut pt_regs,cregs:*mut callee_regs){
    if reg<=11 {*(&mut (*regs).r0 as *mut _ as *mut i64).add(reg as usize)=val;} else if reg==12{(*regs).r12=val;} else if (13..=25).contains(&reg){if !cregs.is_null(){*(&mut (*cregs).r13 as *mut _ as *mut i64).sub((reg-13) as usize)=val;}} else {match reg{26 => (*regs).r26=val,27 => (*regs).fp=val,28 => (*regs).sp=val,30 => (*regs).r30=val,31 => (*regs).blink=val,_=>{}}}
}

pub unsafe fn disasm_next_pc(pc:usize,regs:*mut pt_regs,cregs:*mut callee_regs,next_pc:*mut usize,tgt_if_br:*mut usize)->i32{
    let mut instr=core::mem::MaybeUninit::<disasm_state>::zeroed().assume_init(); disasm_instr(pc,&mut instr,0,regs,cregs); *next_pc=pc+instr.instr_len as usize; if instr.is_branch!=0{*tgt_if_br=instr.target as usize;} if instr.delay_slot!=0{let mut d=core::mem::MaybeUninit::<disasm_state>::zeroed().assume_init();disasm_instr(*next_pc,&mut d,0,regs,cregs);*next_pc+=d.instr_len as usize;} if ((*regs).status32&STATUS32_L)==0&&*next_pc==(*regs).lp_end&&(*regs).lp_count>1{*next_pc=(*regs).lp_start;} instr.is_branch
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
