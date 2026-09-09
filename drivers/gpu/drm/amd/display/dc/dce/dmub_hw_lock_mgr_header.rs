/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependencies: dc_dmub_srv.h and core_types.h.

extern "C" {
    pub fn dmub_hw_lock_mgr_cmd(
        dmub_srv: *mut dc_dmub_srv,
        lock: bool,
        hw_locks: *mut dmub_hw_lock_flags,
        inst_flags: *mut dmub_hw_lock_inst_flags,
    );

    pub fn dmub_hw_lock_mgr_inbox0_cmd(
        dmub_srv: *mut dc_dmub_srv,
        hw_lock_cmd: dmub_inbox0_cmd_lock_hw,
    );

    /**
     * should_use_dmub_inbox1_lock() - Checks if the DMCUB hardware lock via inbox1 should be used.
     *
     * @dc: pointer to DC object
     * @link: optional pointer to the link object to check for enabled link features
     *
     * Return: true if the inbox1 lock should be used, false otherwise
     */
    pub fn should_use_dmub_inbox1_lock(dc: *const dc, link: *const dc_link) -> bool;

    /**
     * dmub_hw_lock_mgr_does_link_require_lock() - Returns true if the link has a feature that needs the HW lock.
     *
     * @dc: Pointer to DC object
     * @link: The link to check
     *
     * Return: true if the link has a feature that needs the HW lock, false otherwise
     */
    pub fn dmub_hw_lock_mgr_does_link_require_lock(dc: *const dc, link: *const dc_link) -> bool;

    /**
     * dmub_hw_lock_mgr_does_context_require_lock() - Returns true if the context has any stream that needs the HW lock.
     *
     * @dc: Pointer to DC object
     * @context: The context to check
     *
     * Return: true if the context has any stream that needs the HW lock, false otherwise
     */
    pub fn dmub_hw_lock_mgr_does_context_require_lock(
        dc: *const dc,
        context: *const dc_state,
    ) -> bool;

    /**
     * should_use_dmub_inbox0_lock_for_link() - Checks if the inbox0 interlock with DMU should be used.
     *
     * Is not functionally equivalent to inbox1 as DMUB will not own programming of the relevant locking
     * registers.
     *
     * @dc: pointer to DC object
     * @link: optional pointer to the link object to check for enabled link features
     *
     * Return: true if the inbox0 lock should be used, false otherwise
     */
    pub fn should_use_dmub_inbox0_lock_for_link(
        dc: *const dc,
        link: *const dc_link,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
