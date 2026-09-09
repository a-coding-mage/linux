// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 ****************************************************************************
 *
 *                   "DHRYSTONE" Benchmark Program
 *                   -----------------------------
 *
 *  Version:    C, Version 2.1
 *
 *  File:       dhry_1.c (part 2 of 3)
 *
 *  Date:       May 25, 1988
 *
 *  Author:     Reinhold P. Weicker
 *
 ****************************************************************************
 */

// Dependencies are supplied by the corresponding Dhrystone/kernel headers.

/* Global Variables: */

static mut Int_Glob: i32 = 0;
static mut Ch_1_Glob: core::ffi::c_char = 0;

static mut Ptr_Glob: Rec_Pointer = core::ptr::null_mut();
static mut Next_Ptr_Glob: Rec_Pointer = core::ptr::null_mut();
static mut Bool_Glob: Boolean = false;
static mut Ch_2_Glob: core::ffi::c_char = 0;
static mut Arr_1_Glob: [i32; 50] = [0; 50];
static mut Arr_2_Glob: [[i32; 50]; 50] = [[0; 50]; 50];

unsafe fn Proc_3(Ptr_Ref_Par: *mut Rec_Pointer) {
	if !Ptr_Glob.is_null() {
		*Ptr_Ref_Par = (*Ptr_Glob).Ptr_Comp;
	}
	Proc_7(10, Int_Glob, &mut (*(*Ptr_Glob)).variant.var_1.Int_Comp);
}

unsafe fn Proc_1(Ptr_Val_Par: Rec_Pointer) {
	let Next_Record: Rec_Pointer = (*Ptr_Val_Par).Ptr_Comp;

	*(*Ptr_Val_Par).Ptr_Comp = *Ptr_Glob;
	(*Ptr_Val_Par).variant.var_1.Int_Comp = 5;
	(*Next_Record).variant.var_1.Int_Comp = (*Ptr_Val_Par).variant.var_1.Int_Comp;
	(*Next_Record).Ptr_Comp = Ptr_Val_Par;
	Proc_3(&mut (*Next_Record).Ptr_Comp);
	if (*Next_Record).Discr == Ident_1 {
		(*Next_Record).variant.var_1.Int_Comp = 6;
		Proc_6((*Ptr_Val_Par).variant.var_1.Enum_Comp,
		       &mut (*Next_Record).variant.var_1.Enum_Comp);
		(*Next_Record).Ptr_Comp = (*Ptr_Glob).Ptr_Comp;
		Proc_7((*Next_Record).variant.var_1.Int_Comp, 10,
		       &mut (*Next_Record).variant.var_1.Int_Comp);
	} else {
		*Ptr_Val_Par = *(*Ptr_Val_Par).Ptr_Comp;
	}
}

unsafe fn Proc_2(Int_Par_Ref: *mut One_Fifty) {
	let mut Int_Loc: One_Fifty = *Int_Par_Ref + 10;
	let mut Enum_Loc: Enumeration;
	loop {
		if Ch_1_Glob == 'A' as core::ffi::c_char {
			Int_Loc -= 1;
			*Int_Par_Ref = Int_Loc - Int_Glob;
			Enum_Loc = Ident_1;
		}
		if Enum_Loc == Ident_1 { break; }
	}
}

unsafe fn Proc_4() {
	let Bool_Loc: Boolean = Ch_1_Glob == 'A' as core::ffi::c_char;
	Bool_Glob = Bool_Loc | Bool_Glob;
	Ch_2_Glob = 'B' as core::ffi::c_char;
}

unsafe fn Proc_5() {
	Ch_1_Glob = 'A' as core::ffi::c_char;
	Bool_Glob = false;
}

pub unsafe fn dhry(n: i32) -> i32 {
	let mut Int_1_Loc: One_Fifty;
	let mut Int_2_Loc: One_Fifty;
	let mut Int_3_Loc: One_Fifty;
	let mut Ch_Index: core::ffi::c_char;
	let mut Enum_Loc: Enumeration;
	let mut Str_1_Loc: Str_30;
	let mut Str_2_Loc: Str_30;
	let mut Run_Index: i32;
	let Number_Of_Runs: i32;
	let mut Begin_Time: ktime_t;
	let mut End_Time: ktime_t;
	let mut User_Time: u32;

	Next_Ptr_Glob = kzalloc_obj(Rec_Type, GFP_ATOMIC) as Rec_Pointer;
	if Next_Ptr_Glob.is_null() { return -ENOMEM; }
	Ptr_Glob = kzalloc_obj(Rec_Type, GFP_ATOMIC) as Rec_Pointer;
	if Ptr_Glob.is_null() { kfree(Next_Ptr_Glob as *mut core::ffi::c_void); return -ENOMEM; }

	(*Ptr_Glob).Ptr_Comp = Next_Ptr_Glob;
	(*Ptr_Glob).Discr = Ident_1;
	(*Ptr_Glob).variant.var_1.Enum_Comp = Ident_3;
	(*Ptr_Glob).variant.var_1.Int_Comp = 40;
	strcpy((*Ptr_Glob).variant.var_1.Str_Comp.as_mut_ptr(), b"DHRYSTONE PROGRAM, SOME STRING\0".as_ptr() as *const core::ffi::c_char);
	strcpy(Str_1_Loc.as_mut_ptr(), b"DHRYSTONE PROGRAM, 1'ST STRING\0".as_ptr() as *const core::ffi::c_char);

	Arr_2_Glob[8][7] = 10;
	pr_debug("Dhrystone Benchmark, Version 2.1 (Language: C)\n");
	Number_Of_Runs = n;
	pr_debug("Execution starts, %d runs through Dhrystone\n", Number_Of_Runs);
	Begin_Time = ktime_get();

	for Run_Index in 1..=Number_Of_Runs {
		Proc_5(); Proc_4();
		Int_1_Loc = 2; Int_2_Loc = 3;
		strcpy(Str_2_Loc.as_mut_ptr(), b"DHRYSTONE PROGRAM, 2'ND STRING\0".as_ptr() as *const core::ffi::c_char);
		Enum_Loc = Ident_2;
		Bool_Glob = !Func_2(Str_1_Loc.as_mut_ptr(), Str_2_Loc.as_mut_ptr());
		while Int_1_Loc < Int_2_Loc {
			Int_3_Loc = 5 * Int_1_Loc - Int_2_Loc;
			Proc_7(Int_1_Loc, Int_2_Loc, &mut Int_3_Loc);
			Int_1_Loc += 1;
		}
		Proc_8(Arr_1_Glob.as_mut_ptr(), Arr_2_Glob.as_mut_ptr() as *mut [i32; 50], Int_1_Loc, Int_3_Loc);
		Proc_1(Ptr_Glob);
		Ch_Index = 'A' as core::ffi::c_char;
		while Ch_Index <= Ch_2_Glob {
			if Enum_Loc == Func_1(Ch_Index, 'C' as core::ffi::c_char) {
				Proc_6(Ident_1, &mut Enum_Loc);
				strcpy(Str_2_Loc.as_mut_ptr(), b"DHRYSTONE PROGRAM, 3'RD STRING\0".as_ptr() as *const core::ffi::c_char);
				Int_2_Loc = Run_Index; Int_Glob = Run_Index;
			}
			Ch_Index += 1;
		}
		Int_2_Loc = Int_2_Loc * Int_1_Loc;
		Int_1_Loc = Int_2_Loc / Int_3_Loc;
		Int_2_Loc = 7 * (Int_2_Loc - Int_3_Loc) - Int_1_Loc;
		Proc_2(&mut Int_1_Loc);
	}

	End_Time = ktime_get();
	pr_debug("Execution ends\n");
	pr_debug("Final values of the variables used in the benchmark:\n");
	macro_rules! dhry_assert_int_eq { ($v:expr, $e:expr) => { if $v != $e { pr_err("value: %d (FAIL, expected %d)\n", $v, $e); } else { pr_debug("value: %d (OK)\n", $v); } }; }
	macro_rules! dhry_assert_char_eq { ($v:expr, $e:expr) => { if $v != $e { pr_err("value: %c (FAIL, expected %c)\n", $v, $e); } else { pr_debug("value: %c (OK)\n", $v); } }; }
	dhry_assert_int_eq!(Int_Glob, 5);
	dhry_assert_int_eq!(Bool_Glob, true);
	dhry_assert_char_eq!(Ch_1_Glob, 'A' as core::ffi::c_char);
	dhry_assert_char_eq!(Ch_2_Glob, 'B' as core::ffi::c_char);
	dhry_assert_int_eq!(Arr_1_Glob[8], 7);
	dhry_assert_int_eq!(Arr_2_Glob[8][7], Number_Of_Runs + 10);
	pr_debug("Ptr_Comp: %px\n", (*Ptr_Glob).Ptr_Comp);
	dhry_assert_int_eq!((*Ptr_Glob).Discr, 0);
	dhry_assert_int_eq!((*Ptr_Glob).variant.var_1.Enum_Comp, 2);
	dhry_assert_int_eq!((*Ptr_Glob).variant.var_1.Int_Comp, 17);
	dhry_assert_int_eq!(strcmp((*Ptr_Glob).variant.var_1.Str_Comp.as_ptr(), b"DHRYSTONE PROGRAM, SOME STRING\0".as_ptr() as *const core::ffi::c_char), 0);
	if (*Next_Ptr_Glob).Ptr_Comp != (*Ptr_Glob).Ptr_Comp { pr_err("Next_Ptr_Glob->Ptr_Comp differs\n"); } else { pr_debug("Next_Ptr_Glob->Ptr_Comp: %px\n", (*Next_Ptr_Glob).Ptr_Comp); }
	dhry_assert_int_eq!((*Next_Ptr_Glob).Discr, 0);
	dhry_assert_int_eq!((*Next_Ptr_Glob).variant.var_1.Enum_Comp, 1);
	dhry_assert_int_eq!((*Next_Ptr_Glob).variant.var_1.Int_Comp, 18);
	dhry_assert_int_eq!(strcmp((*Next_Ptr_Glob).variant.var_1.Str_Comp.as_ptr(), b"DHRYSTONE PROGRAM, SOME STRING\0".as_ptr() as *const core::ffi::c_char), 0);
	dhry_assert_int_eq!(Int_1_Loc, 5);
	dhry_assert_int_eq!(Int_2_Loc, 13);
	dhry_assert_int_eq!(Int_3_Loc, 7);
	dhry_assert_int_eq!(Enum_Loc, 1);
	dhry_assert_int_eq!(strcmp(Str_1_Loc.as_ptr(), b"DHRYSTONE PROGRAM, 1'ST STRING\0".as_ptr() as *const core::ffi::c_char), 0);
	dhry_assert_int_eq!(strcmp(Str_2_Loc.as_ptr(), b"DHRYSTONE PROGRAM, 2'ND STRING\0".as_ptr() as *const core::ffi::c_char), 0);
	User_Time = ktime_ms_delta(End_Time, Begin_Time);
		kfree(Ptr_Glob as *mut core::ffi::c_void);
		kfree(Next_Ptr_Glob as *mut core::ffi::c_void);
	if User_Time < 2 * MSEC_PER_SEC { return -EAGAIN; }
	div_u64(mul_u32_u32(MSEC_PER_SEC, Number_Of_Runs as u32), User_Time) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
