// Integration test for SigmaOS-user-quota
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use user_quota as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
