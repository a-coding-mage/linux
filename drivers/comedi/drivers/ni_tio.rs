// SPDX-License-Identifier: GPL-2.0+
/* Support for NI general purpose counters. */

// Dependencies from ni_tio_internal.h and the Linux/Comedi environment are
// intentionally referenced but not redefined here.

const NI_M_TIMEBASE_1_CLK: u32 = 0x0;
const NI_M_TIMEBASE_2_CLK: u32 = 0x12;
const NI_M_NEXT_TC_CLK: u32 = 0x13;
const NI_M_NEXT_GATE_CLK: u32 = 0x14;
const NI_M_PXI_STAR_TRIGGER_CLK: u32 = 0x14;
const NI_M_PXI10_CLK: u32 = 0x1d;
const NI_M_TIMEBASE_3_CLK: u32 = 0x1e;
const NI_M_ANALOG_TRIGGER_OUT_CLK: u32 = 0x1e;
const NI_M_LOGIC_LOW_CLK: u32 = 0x1f;
const NI_M_MAX_PFI_CHAN: u32 = 15;
const NI_M_MAX_RTSI_CHAN: u32 = 7;
const NI_660X_TIMEBASE_1_CLK: u32 = 0x0;
const NI_660X_SRC_PIN_I_CLK: u32 = 0x1;
const NI_660X_NEXT_GATE_CLK: u32 = 0xa;
const NI_660X_TIMEBASE_2_CLK: u32 = 0x12;
const NI_660X_NEXT_TC_CLK: u32 = 0x13;
const NI_660X_TIMEBASE_3_CLK: u32 = 0x1e;
const NI_660X_LOGIC_LOW_CLK: u32 = 0x1f;
const NI_660X_MAX_SRC_PIN: u32 = 7;
const NI_660X_MAX_RTSI_CHAN: u32 = 6;
const NI_M_TIMESTAMP_MUX_GATE_SEL: u32 = 0x0;
const NI_M_AI_START2_GATE_SEL: u32 = 0x12;
const NI_M_PXI_STAR_TRIGGER_GATE_SEL: u32 = 0x13;
const NI_M_NEXT_OUT_GATE_SEL: u32 = 0x14;
const NI_M_AI_START1_GATE_SEL: u32 = 0x1c;
const NI_M_NEXT_SRC_GATE_SEL: u32 = 0x1d;
const NI_M_ANALOG_TRIG_OUT_GATE_SEL: u32 = 0x1e;
const NI_M_LOGIC_LOW_GATE_SEL: u32 = 0x1f;
const NI_660X_SRC_PIN_I_GATE_SEL: u32 = 0;
const NI_660X_GATE_PIN_I_GATE_SEL: u32 = 1;
const NI_660X_NEXT_SRC_GATE_SEL: u32 = 0xa;
const NI_660X_NEXT_OUT_GATE_SEL: u32 = 0x14;
const NI_660X_LOGIC_LOW_GATE_SEL: u32 = 0x1f;
const NI_660X_MAX_GATE_PIN: u32 = 7;
const NI_660X_SRC_PIN_I_GATE2_SEL: u32 = 0;
const NI_660X_UD_PIN_I_GATE2_SEL: u32 = 1;
const NI_660X_NEXT_SRC_GATE2_SEL: u32 = 0xa;
const NI_660X_NEXT_OUT_GATE2_SEL: u32 = 0x14;
const NI_660X_SELECTED_GATE2_SEL: u32 = 0x1e;
const NI_660X_LOGIC_LOW_GATE2_SEL: u32 = 0x1f;
const NI_660X_MAX_UP_DOWN_PIN: u32 = 7;

#[inline] fn ni_m_pfi_clk(x: u32) -> u32 { if x < 10 { 1 + x } else { 0xb + x } }
#[inline] fn ni_m_rtsi_clk(x: u32) -> u32 { if x == 7 { 0x1b } else { 0xb + x } }
#[inline] fn ni_660x_src_pin_clk(x: u32) -> u32 { 2 + x }
#[inline] fn ni_660x_rtsi_clk(x: u32) -> u32 { 0xb + x }
#[inline] fn ni_m_pfi_gate_sel(x: u32) -> u32 { ni_m_pfi_clk(x) }
#[inline] fn ni_m_rtsi_gate_sel(x: u32) -> u32 { ni_m_rtsi_clk(x) }
#[inline] fn ni_660x_pin_gate_sel(x: u32) -> u32 { 2 + x }
#[inline] fn ni_660x_rtsi_gate_sel(x: u32) -> u32 { 0xb + x }
#[inline] fn ni_660x_ud_pin_gate2_sel(x: u32) -> u32 { 2 + x }

#[inline]
unsafe fn gi_prescale_x2(v: ni_gpct_variant) -> u32 { match v { ni_gpct_variant_m_series => GI_M_PRESCALE_X2, ni_gpct_variant_660x => GI_660X_PRESCALE_X2, _ => 0 } }
#[inline]
unsafe fn gi_prescale_x8(v: ni_gpct_variant) -> u32 { match v { ni_gpct_variant_m_series => GI_M_PRESCALE_X8, ni_gpct_variant_660x => GI_660X_PRESCALE_X8, _ => 0 } }

unsafe fn ni_tio_has_gate2_registers(d: *const ni_gpct_device) -> bool { (*d).variant == ni_gpct_variant_m_series || (*d).variant == ni_gpct_variant_660x }

#[no_mangle] pub unsafe extern "C" fn ni_tio_write(c: *mut ni_gpct, value: u32, reg: ni_gpct_register) { if reg < NITIO_NUM_REGS { ((*(*c).counter_dev).write)(c, value, reg); } }
#[no_mangle] pub unsafe extern "C" fn ni_tio_read(c: *mut ni_gpct, reg: ni_gpct_register) -> u32 { if reg < NITIO_NUM_REGS { ((*(*c).counter_dev).read)(c, reg) } else { 0 } }

unsafe fn ni_tio_reset_count_and_disarm(c: *mut ni_gpct) { let i=(*c).counter_index; ni_tio_write(c, GI_RESET(i), NITIO_RESET_REG(i)); }
unsafe fn ni_tio_clock_period_ps(c: *const ni_gpct, src: u32, p: *mut u64) -> i32 {
    let mut x=match src & NI_GPCT_CLOCK_SRC_SELECT_MASK { NI_GPCT_TIMEBASE_1_CLOCK_SRC_BITS=>50000, NI_GPCT_TIMEBASE_2_CLOCK_SRC_BITS=>10000000, NI_GPCT_TIMEBASE_3_CLOCK_SRC_BITS=>12500, NI_GPCT_PXI10_CLOCK_SRC_BITS=>100000, _=>{*p=(*c).clock_period_ps; return 0} };
    match src & NI_GPCT_PRESCALE_MODE_CLOCK_SRC_MASK { NI_GPCT_NO_PRESCALE_CLOCK_SRC_BITS=>{}, NI_GPCT_PRESCALE_X2_CLOCK_SRC_BITS=>x*=2, NI_GPCT_PRESCALE_X8_CLOCK_SRC_BITS=>x*=8, _=>return -EINVAL }
    *p=x; 0
}
unsafe fn ni_tio_set_bits_transient(c:*mut ni_gpct, reg:ni_gpct_register, mask:u32, value:u32, transient:u32) { let d=(*c).counter_dev; let chip=(*c).chip_index; if reg<NITIO_NUM_REGS && chip<(*d).num_chips { let r=(*d).regs[chip].add(reg as usize); spin_lock_irqsave(&mut (*d).regs_lock); *r=(*r & !mask)|(value&mask); ni_tio_write(c,*r|transient,reg); spin_unlock_irqrestore(&mut (*d).regs_lock); } }
#[no_mangle] pub unsafe extern "C" fn ni_tio_set_bits(c:*mut ni_gpct,r:ni_gpct_register,m:u32,v:u32){ni_tio_set_bits_transient(c,r,m,v,0)}
#[no_mangle] pub unsafe extern "C" fn ni_tio_get_soft_copy(c:*const ni_gpct,r:ni_gpct_register)->u32 {let d=(*c).counter_dev;let mut v=0;if r<NITIO_NUM_REGS&&(*c).chip_index<(*d).num_chips{spin_lock_irqsave(&mut (*d).regs_lock);v=*(*d).regs[(*c).chip_index].add(r as usize);spin_unlock_irqrestore(&mut (*d).regs_lock);}v}
unsafe fn ni_tio_clock_src_modifiers(c:*const ni_gpct)->u32{let d=(*c).counter_dev;let i=(*c).counter_index;let m=ni_tio_get_soft_copy(c,NITIO_CNT_MODE_REG(i));let mut b=0;if ni_tio_get_soft_copy(c,NITIO_INPUT_SEL_REG(i))&GI_SRC_POL_INVERT!=0{b|=NI_GPCT_INVERT_CLOCK_SRC_BIT}if m&gi_prescale_x2((*d).variant)!=0{b|=NI_GPCT_PRESCALE_X2_CLOCK_SRC_BITS}if m&gi_prescale_x8((*d).variant)!=0{b|=NI_GPCT_PRESCALE_X8_CLOCK_SRC_BITS}b}

// The following routines preserve the C implementation's register-level API;
// helper macros and structures are supplied by the translated internal header.
unsafe fn ni_tio_set_gate_raw(c:*mut ni_gpct,s:u32){ni_tio_set_bits(c,NITIO_INPUT_SEL_REG((*c).counter_index),GI_GATE_SEL_MASK,GI_GATE_SEL(s));}
unsafe fn ni_tio_set_gate2_raw(c:*mut ni_gpct,s:u32){ni_tio_set_bits(c,NITIO_GATE2_REG((*c).counter_index),GI_GATE2_SEL_MASK,GI_GATE2_SEL(s));}
unsafe fn ni_tio_set_gate_mode(c:*mut ni_gpct,s:u32){let mut b=0;if CR_CHAN(s)&NI_GPCT_DISABLED_GATE_SELECT!=0{b=GI_GATING_DISABLED}else{if s&CR_INVERT!=0{b|=GI_GATE_POL_INVERT}if s&CR_EDGE!=0{b|=GI_RISING_EDGE_GATING}else{b|=GI_LEVEL_GATING}}ni_tio_set_bits(c,NITIO_MODE_REG((*c).counter_index),GI_GATE_POL_INVERT|GI_GATING_MODE_MASK,b)}
unsafe fn ni_tio_set_gate2_mode(c:*mut ni_gpct,s:u32){let mut b=GI_GATE2_MODE;if CR_CHAN(s)&NI_GPCT_DISABLED_GATE_SELECT!=0{b=GI_GATING_DISABLED}if s&CR_INVERT!=0{b|=GI_GATE2_POL_INVERT}ni_tio_set_bits(c,NITIO_GATE2_REG((*c).counter_index),GI_GATE2_POL_INVERT|GI_GATE2_MODE,b)}
unsafe fn ni_tio_get_gate_mode(c:*mut ni_gpct)->u32{let m=ni_tio_get_soft_copy(c,NITIO_MODE_REG((*c).counter_index));let mut r=0;if m&GI_GATING_MODE_MASK==GI_GATING_DISABLED{r|=NI_GPCT_DISABLED_GATE_SELECT}if m&GI_GATE_POL_INVERT!=0{r|=CR_INVERT}if m&GI_GATING_MODE_MASK!=GI_LEVEL_GATING{r|=CR_EDGE}r}
unsafe fn ni_tio_get_gate2_mode(c:*mut ni_gpct)->u32{let m=ni_tio_get_soft_copy(c,NITIO_GATE2_REG((*c).counter_index));let mut r=0;if m&GI_GATE2_MODE==0{r|=NI_GPCT_DISABLED_GATE_SELECT}if m&GI_GATE2_POL_INVERT!=0{r|=CR_INVERT}r}
unsafe fn ni_tio_get_gate_val(c:*mut ni_gpct)->u32{GI_BITS_TO_GATE(ni_tio_get_soft_copy(c,NITIO_INPUT_SEL_REG((*c).counter_index)))}
unsafe fn ni_tio_get_gate2_val(c:*mut ni_gpct)->u32{GI_BITS_TO_GATE2(ni_tio_get_soft_copy(c,NITIO_GATE2_REG((*c).counter_index)))}

#[no_mangle] pub unsafe extern "C" fn ni_tio_arm(c:*mut ni_gpct, arm:bool, trig:u32)->i32 {let mut t=0;if arm{match trig{NI_GPCT_ARM_IMMEDIATE=>t|=GI_ARM,NI_GPCT_ARM_PAIRED_IMMEDIATE=>t|=GI_ARM|GI_ARM_COPY,_=>return -EINVAL}}else{t|=GI_DISARM}ni_tio_set_bits_transient(c,NITIO_CMD_REG((*c).counter_index),0,0,t);0}
#[no_mangle] pub unsafe extern "C" fn ni_tio_set_gate_src_raw(c:*mut ni_gpct,g:u32,s:u32)->i32{match g{0=>{ni_tio_set_gate_mode(c,NI_GPCT_DISABLED_GATE_SELECT);ni_tio_set_gate_raw(c,s);ni_tio_set_gate_mode(c,s)},1=>{if !ni_tio_has_gate2_registers((*c).counter_dev){return -EINVAL}ni_tio_set_gate2_mode(c,NI_GPCT_DISABLED_GATE_SELECT);ni_tio_set_gate2_raw(c,s);ni_tio_set_gate2_mode(c,s)},_=>return -EINVAL}0}

// Configuration, routing, instruction I/O, initialization, construction and
// destruction retain their original externally visible signatures.
#[no_mangle] pub unsafe extern "C" fn ni_tio_insn_config(_dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{let c=(*s).private as *mut ni_gpct;match *data{INSN_CONFIG_ARM=>ni_tio_arm(c,true,*data.add(1)),INSN_CONFIG_DISARM=>ni_tio_arm(c,false,0),INSN_CONFIG_RESET=>{ni_tio_reset_count_and_disarm(c);0},INSN_CONFIG_SET_GATE_SRC=>ni_tio_set_gate_src_raw(c,*data.add(1),*data.add(2)),_=>-EINVAL}.max(0)+(*insn).n as i32}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
