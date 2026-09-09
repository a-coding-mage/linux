/*
 * Copyright 2017 Valve Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Andres Rodriguez <andresx7@gmail.com>
 */

unsafe fn amdgpu_sched_process_priority_override(
    _adev: *mut amdgpu_device,
    fd: i32,
    priority: i32,
) -> i32 {
    // Equivalent to CLASS(fd, f)(fd); the descriptor guard is supplied by the
    // surrounding kernel bindings.
    let f = unsafe { fdget(fd) };
    let mut fpriv: *mut amdgpu_fpriv = core::ptr::null_mut();
    let mut mgr: *mut amdgpu_ctx_mgr;
    let mut ctx: *mut amdgpu_ctx;
    let mut id: u64 = 0;
    let r: i32;

    if unsafe { fd_empty(f) } {
        return -EINVAL;
    }

    r = unsafe { amdgpu_file_to_fpriv(fd_file(f), &mut fpriv) };
    if r != 0 {
        return r;
    }

    mgr = unsafe { &mut (*fpriv).ctx_mgr };
    unsafe {
        xa_lock(&mut (*mgr).ctx_handles);
        xa_for_each(&mut (*mgr).ctx_handles, &mut id, &mut ctx, {
            amdgpu_ctx_priority_override(ctx, priority);
        });
        xa_unlock(&mut (*mgr).ctx_handles);
    }

    0
}

unsafe fn amdgpu_sched_context_priority_override(
    _adev: *mut amdgpu_device,
    fd: i32,
    ctx_id: u32,
    priority: i32,
) -> i32 {
    // Equivalent to CLASS(fd, f)(fd); the descriptor guard is supplied by the
    // surrounding kernel bindings.
    let f = unsafe { fdget(fd) };
    let mut fpriv: *mut amdgpu_fpriv = core::ptr::null_mut();
    let ctx: *mut amdgpu_ctx;
    let r: i32;

    if unsafe { fd_empty(f) } {
        return -EINVAL;
    }

    r = unsafe { amdgpu_file_to_fpriv(fd_file(f), &mut fpriv) };
    if r != 0 {
        return r;
    }

    ctx = unsafe { amdgpu_ctx_get(fpriv, ctx_id) };

    if ctx.is_null() {
        return -EINVAL;
    }

    unsafe {
        amdgpu_ctx_priority_override(ctx, priority);
        amdgpu_ctx_put(ctx);
    }
    0
}

pub unsafe fn amdgpu_sched_ioctl(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    _filp: *mut drm_file,
) -> i32 {
    let args = data as *mut drm_amdgpu_sched;
    let adev = unsafe { drm_to_adev(dev) };
    let r: i32;

    /* First check the op, then the op's argument.
     */
    match unsafe { (*args).input.op } {
        AMDGPU_SCHED_OP_PROCESS_PRIORITY_OVERRIDE
        | AMDGPU_SCHED_OP_CONTEXT_PRIORITY_OVERRIDE => {}
        _ => {
            unsafe {
                DRM_ERROR!("Invalid sched op specified: {}\n", (*args).input.op);
            }
            return -EINVAL;
        }
    }

    if !unsafe { amdgpu_ctx_priority_is_valid((*args).input.priority) } {
        return -EINVAL;
    }

    match unsafe { (*args).input.op } {
        AMDGPU_SCHED_OP_PROCESS_PRIORITY_OVERRIDE => {
            r = unsafe {
                amdgpu_sched_process_priority_override(
                    adev,
                    (*args).input.fd,
                    (*args).input.priority,
                )
            };
        }
        AMDGPU_SCHED_OP_CONTEXT_PRIORITY_OVERRIDE => {
            r = unsafe {
                amdgpu_sched_context_priority_override(
                    adev,
                    (*args).input.fd,
                    (*args).input.ctx_id,
                    (*args).input.priority,
                )
            };
        }
        _ => {
            /* Impossible.
             */
            r = -EINVAL;
        }
    }

    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
