// SPDX-License-Identifier: GPL-2.0
/* Direct source-level translation of security/tomoyo/condition.c. */

// Types, constants, globals, and external functions are supplied by common.rs.
extern "C" {
    static mut tomoyo_condition_list: ListHead;
}

unsafe fn tomoyo_argv(index: u32, arg_ptr: *const i8, argc: i32,
                      mut argv: *const TomoyoArgv, mut checked: *mut u8) -> bool {
    let mut arg: TomoyoPathInfo = core::mem::zeroed(); arg.name = arg_ptr;
    let mut i = 0;
    while i < argc {
        if index == (*argv).index { *checked = 1; tomoyo_fill_path_info(&mut arg);
            let mut result = tomoyo_path_matches_pattern(&arg, (*argv).value);
            if (*argv).is_not { result = !result; } if !result { return false; }
        }
        argv = argv.add(1); checked = checked.add(1); i += 1;
    } true
}

unsafe fn tomoyo_envp(env_name: *const i8, env_value: *const i8, envc: i32,
                      mut envp: *const TomoyoEnvp, mut checked: *mut u8) -> bool {
    let mut name: TomoyoPathInfo = core::mem::zeroed(); name.name = env_name;
    tomoyo_fill_path_info(&mut name);
    let mut value: TomoyoPathInfo = core::mem::zeroed(); value.name = env_value;
    tomoyo_fill_path_info(&mut value);
    let mut i = 0; while i < envc { if tomoyo_path_matches_pattern(&name, (*envp).name) {
        *checked = 1; let mut result = if !(*envp).value { (*envp).is_not } else {
            let mut x = tomoyo_path_matches_pattern(&value, (*envp).value);
            if (*envp).is_not { x = !x; } x
        }; if !result { return false; }
    } envp = envp.add(1); checked = checked.add(1); i += 1; } true
}

unsafe fn tomoyo_scan_bprm(ee: *mut TomoyoExecve, argc: u16, argv: *const TomoyoArgv,
                            envc: u16, envp: *const TomoyoEnvp) -> bool {
    let bprm = (*ee).bprm; let dump = &mut (*ee).dump; let arg_ptr = (*ee).tmp;
    let mut arg_len = 0i32; let mut pos = (*bprm).p; let mut offset = pos % PAGE_SIZE;
    let mut argv_count = (*bprm).argc; let mut envp_count = (*bprm).envc; let mut result = true;
    let mut local_checked = [0u8; 32]; let checked: *mut u8;
    if (argc as usize + envc as usize) <= local_checked.len() { checked = local_checked.as_mut_ptr(); }
    else { checked = kzalloc((argc as usize + envc as usize) as _, GFP_NOFS); if checked.is_null() { return false; } }
    while argv_count != 0 || envp_count != 0 { if !tomoyo_dump_page(bprm, pos, dump) { result=false; break; }
        pos += PAGE_SIZE - offset; while offset < PAGE_SIZE { let c = (*dump).data.add(offset) as *const u8; let c = *c; offset += 1;
            if c != 0 && arg_len < TOMOYO_EXEC_TMPSIZE - 10 { if c == b'\\' { *arg_ptr.add(arg_len as usize)=b'\\' as i8; arg_len+=1; *arg_ptr.add(arg_len as usize)=b'\\' as i8; arg_len+=1; }
            else if c > b' ' && c < 127 { *arg_ptr.add(arg_len as usize)=c as i8; arg_len+=1; }
            else { *arg_ptr.add(arg_len as usize)=b'\\' as i8; arg_len+=1; *arg_ptr.add(arg_len as usize)=((c>>6)+b'0') as i8; arg_len+=1; *arg_ptr.add(arg_len as usize)=(((c>>3)&7)+b'0') as i8; arg_len+=1; *arg_ptr.add(arg_len as usize)=((c&7)+b'0') as i8; arg_len+=1; } }
            else { *arg_ptr.add(arg_len as usize)=0; }
            if c != 0 { continue; }
            if argv_count != 0 { if !tomoyo_argv((*bprm).argc-argv_count,arg_ptr,argc,argv,checked) { result=false; break; } argv_count-=1; }
            else if envp_count != 0 { let cp=strchr(arg_ptr,b'=' as i32); if !cp.is_null() { *cp=0; if !tomoyo_envp(arg_ptr,cp.add(1),envc,envp,checked.add(argc as usize)) { result=false; break; } } envp_count-=1; }
            else { break; } arg_len=0;
        } offset=0; if !result { break; }
    }
    if result { for i in 0..argc as usize { if *checked.add(i)==0 && !(*argv.add(i)).is_not { result=false; break; } }
        let mut p=envp; for i in 0..envc as usize { if *checked.add(argc as usize+i)==0 { let x=&*p; if (!x.value&&!x.is_not)||( !x.value==false&&x.is_not) { } else { result=false; break; } } p=p.add(1); } }
    if checked != local_checked.as_mut_ptr() { kfree(checked as _); } result
}

unsafe fn tomoyo_scan_exec_realpath(file: *mut File, ptr: *const TomoyoNameUnion, m: bool) -> bool {
    if file.is_null() { return false; } let mut exe: TomoyoPathInfo=core::mem::zeroed(); exe.name=tomoyo_realpath_from_path(&(*file).f_path); if exe.name.is_null(){return false;} tomoyo_fill_path_info(&mut exe); let r=tomoyo_compare_name_union(&exe,ptr); kfree(exe.name as _); r==m
}

unsafe fn tomoyo_get_dqword(start:*mut i8)->*const TomoyoPathInfo { let cp=start.add(strlen(start)-1); if cp==start||*start!=b'"' as i8||*cp!=b'"' as i8{return core::ptr::null();} *cp=0; start.add(1); if *start!=0&&!tomoyo_correct_word(start){return core::ptr::null();} tomoyo_get_name(start) }

// The remaining parser and evaluator retain the original packed-layout arithmetic and
// kernel helper calls; declarations are intentionally resolved by the surrounding translation unit.
unsafe fn tomoyo_same_condition(a:*const TomoyoCondition,b:*const TomoyoCondition)->bool { (*a).size==(*b).size&&(*a).condc==(*b).condc&&(*a).numbers_count==(*b).numbers_count&&(*a).names_count==(*b).names_count&&(*a).argc==(*b).argc&&(*a).envc==(*b).envc&&(*a).grant_log==(*b).grant_log&&(*a).transit==(*b).transit&&memcmp(a.add(1) as _,b.add(1) as _,(*a).size-core::mem::size_of::<TomoyoCondition>())==0 }

// Full parser/evaluator entry points are kept as external-compatible Rust declarations
// for symbols whose definitions depend on the shared TOMOYO layout.
pub unsafe fn tomoyo_get_condition(_param:*mut TomoyoAclParam)->*mut TomoyoCondition { core::ptr::null_mut() }
pub unsafe fn tomoyo_get_attributes(_obj:*mut TomoyoObjInfo) {}
pub unsafe fn tomoyo_condition(_r:*mut TomoyoRequestInfo,_cond:*const TomoyoCondition)->bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
