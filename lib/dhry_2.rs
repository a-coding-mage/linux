// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 ****************************************************************************
 *
 *                   "DHRYSTONE" Benchmark Program
 *                   -----------------------------
 *
 *  Version:    C, Version 2.1
 *
 *  File:       dhry_2.c (part 3 of 3)
 *
 *  Date:       May 25, 1988
 *
 *  Author:     Reinhold P. Weicker
 *
 ****************************************************************************
 */

// Dependency declarations are supplied by dhry.h and the platform string library.

extern "C" {
    static mut Int_Glob: One_Fifty;
    static mut Ch_1_Glob: Capital_Letter;
    fn strcmp(a: *const Capital_Letter, b: *const Capital_Letter) -> i32;
}

unsafe fn Func_3(Enum_Par_Val: Enumeration) -> Boolean {
    /* executed once */
    /* Enum_Par_Val == Ident_3 */
    let Enum_Loc = Enum_Par_Val;
    if Enum_Loc == Ident_3 {
        /* then, executed */
        true
    } else {
        /* not executed */
        false
    }
} /* Func_3 */

pub unsafe fn Proc_6(Enum_Val_Par: Enumeration, Enum_Ref_Par: *mut Enumeration) {
    /* executed once */
    /* Enum_Val_Par == Ident_3, Enum_Ref_Par becomes Ident_2 */
    *Enum_Ref_Par = Enum_Val_Par;
    if !Func_3(Enum_Val_Par) {
        /* then, not executed */
        *Enum_Ref_Par = Ident_4;
    }
    match Enum_Val_Par {
        Ident_1 => *Enum_Ref_Par = Ident_1,
        Ident_2 => {
            if Int_Glob > 100 {
                /* then */
                *Enum_Ref_Par = Ident_1;
            } else {
                *Enum_Ref_Par = Ident_4;
            }
        }
        Ident_3 => *Enum_Ref_Par = Ident_2, /* executed */
        Ident_4 => {}
        Ident_5 => *Enum_Ref_Par = Ident_3,
        _ => {}
    } /* switch */
} /* Proc_6 */

pub unsafe fn Proc_7(Int_1_Par_Val: One_Fifty, Int_2_Par_Val: One_Fifty,
                     Int_Par_Ref: *mut One_Fifty) {
    /* executed three times */
    let Int_Loc = Int_1_Par_Val + 2;
    *Int_Par_Ref = Int_2_Par_Val + Int_Loc;
} /* Proc_7 */

pub unsafe fn Proc_8(mut Arr_1_Par_Ref: Arr_1_Dim, mut Arr_2_Par_Ref: Arr_2_Dim,
                      Int_1_Par_Val: i32, Int_2_Par_Val: i32) {
    /* executed once */
    let Int_Loc = Int_1_Par_Val + 5;
    Arr_1_Par_Ref[Int_Loc as usize] = Int_2_Par_Val;
    Arr_1_Par_Ref[(Int_Loc + 1) as usize] = Arr_1_Par_Ref[Int_Loc as usize];
    Arr_1_Par_Ref[(Int_Loc + 30) as usize] = Int_Loc;
    let mut Int_Index = Int_Loc;
    while Int_Index <= Int_Loc + 1 {
        Arr_2_Par_Ref[Int_Loc as usize][Int_Index as usize] = Int_Loc;
        Int_Index += 1;
    }
    Arr_2_Par_Ref[Int_Loc as usize][(Int_Loc - 1) as usize] += 1;
    Arr_2_Par_Ref[(Int_Loc + 20) as usize][Int_Loc as usize] = Arr_1_Par_Ref[Int_Loc as usize];
    Int_Glob = 5;
} /* Proc_8 */

pub unsafe fn Func_1(Ch_1_Par_Val: Capital_Letter, Ch_2_Par_Val: Capital_Letter) -> Enumeration {
    let Ch_1_Loc = Ch_1_Par_Val;
    let Ch_2_Loc = Ch_1_Loc;
    if Ch_2_Loc != Ch_2_Par_Val {
        /* then, executed */
        Ident_1
    } else {
        /* not executed */
        Ch_1_Glob = Ch_1_Loc;
        Ident_2
    }
} /* Func_1 */

pub unsafe fn Func_2(Str_1_Par_Ref: Str_30, Str_2_Par_Ref: Str_30) -> Boolean {
    let mut Int_Loc: One_Thirty = 2;
    let mut Ch_Loc: Capital_Letter = 0 as Capital_Letter;
    while Int_Loc <= 2 {
        if Func_1(Str_1_Par_Ref[Int_Loc as usize], Str_2_Par_Ref[(Int_Loc + 1) as usize]) == Ident_1 {
            Ch_Loc = 'A' as Capital_Letter;
            Int_Loc += 1;
        }
    }
    if Ch_Loc >= 'W' as Capital_Letter && Ch_Loc < 'Z' as Capital_Letter {
        Int_Loc = 7;
    }
    if Ch_Loc == 'R' as Capital_Letter {
        true
    } else if strcmp(Str_1_Par_Ref.as_ptr(), Str_2_Par_Ref.as_ptr()) > 0 {
        Int_Loc += 7;
        Int_Glob = Int_Loc;
        true
    } else {
        false
    }
} /* Func_2 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
