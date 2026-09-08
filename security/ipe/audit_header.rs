// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.

// Declarations depend on types from policy.h:
// - struct ipe_eval_ctx
// - enum ipe_match
// - enum ipe_action_type
// - struct ipe_rule
// - struct ipe_policy

extern "C" {
    pub fn ipe_audit_match(
        ctx: *const ipe_eval_ctx,
        match_type: ipe_match,
        act: ipe_action_type,
        r: *const ipe_rule,
    );

    pub fn ipe_audit_policy_load(p: *const ipe_policy);

    pub fn ipe_audit_policy_activation(
        op: *const ipe_policy,
        np: *const ipe_policy,
    );

    pub fn ipe_audit_enforce(new_enforce: bool, old_enforce: bool);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
