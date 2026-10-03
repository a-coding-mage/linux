# Newly authored admission logic. No payload text is executed.
.repository.full_name == "a-coding-mage/linux" and
.repository.private == false and
.ref == "refs/heads/feat/rust-translation-lupos" and
.created == false and .deleted == false and .forced == false and
(.before | type == "string" and test("^[0-9a-f]{40}$") and . != "0000000000000000000000000000000000000000") and
(.after | type == "string" and test("^[0-9a-f]{40}$") and . != "0000000000000000000000000000000000000000") and
(.commits | type == "array" and length == 1) and
.commits[0].id == .after and .head_commit.id == .after
