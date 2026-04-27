/*
 * tests/role.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2026 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
mod common;

use deepwell::services::role::{InternalCreateRoleInput, RoleService};
use std::sync::atomic::{AtomicU64, Ordering};
use str_macro::str;

use self::common::TestRunner;
use deepwell::constants::SYSTEM_USER_ID;
use deepwell::error::prelude::*;
use deepwell::license::License;
use deepwell::models::role::Model as RoleModel;
use deepwell::services::site::{CreateSite, SiteService};
use deepwell::services::user::{CreateUser, UserService};
use deepwell::types::UserType;
use serde_json::json;

static FIXTURE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn next_n() -> u64 {
    FIXTURE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

struct RoleFixture {
    site_id: i64,
    user_id: i64,
}

impl RoleFixture {
    async fn setup(runner: &TestRunner) -> Self {
        let ctx = runner.context();
        let n = next_n();

        let site = SiteService::create(
            ctx,
            CreateSite {
                slug: format!("role-test-{n}"),
                name: format!("Role test site {n}"),
                tagline: String::new(),
                description: format!("Role test site {n}"),
                default_page: None,
                layout: None,
                license: License::CcBySa40,
                locale: String::from("en"),
                ip_address: common::IP_ADDRESS,
            },
        )
        .await
        .expect("Failed to create test site");

        let user = UserService::create(
            ctx,
            CreateUser {
                user_type: UserType::Regular,
                name: format!("Role Test User {n}"),
                email: format!("role-test-{n}@email.com"),
                locales: vec![str!("en")],
                password: String::from("password"),
                bypass_filter: true,
                bypass_email_verification: true,
                override_user_id: None,
                ip_address: common::IP_ADDRESS,
            },
        )
        .await
        .expect("Failed to create test user");

        RoleFixture {
            site_id: site.site_id,
            user_id: user.user_id,
        }
    }
}

// Test helpers
async fn create_role(
    runner: &TestRunner,
    site_id: i64,
    name: &str,
    parent_role_id: Option<i64>,
) -> RoleModel {
    // Use API because endpoint validates parent ID, and creating a root role for most tests causes more trouble than it's worth.
    // Create endpoint will be tested explicitly in its test
    RoleService::create(
        runner.context(),
        InternalCreateRoleInput {
            site_id,
            name: str!(name),
            description: None,
            is_virtual: false,
            parent_role_id,
            creating_user_id: SYSTEM_USER_ID,
            ip_address: common::IP_ADDRESS,
        },
    )
    .await
    .expect("Failed to create role")
}

async fn set_role_perms(
    runner: &TestRunner,
    site_id: i64,
    role_id: i64,
    actions: &[&str],
    cascade: bool,
) {
    let permissions: Vec<_> = actions
        .iter()
        // Hardcoding Page resource type and null category for simplicity
        .map(|a| json!({ "resource_type": "page", "resource_category": null, "action": a }))
        .collect();
    run_endpoint!(
        runner,
        role_update_permissions,
        json!({
            "site_id": site_id,
            "role_reference": role_id,
            "new_permissions": permissions,
            "cascade_removals": cascade,
            "updating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );
}

async fn reparent_role(
    runner: &TestRunner,
    site_id: i64,
    role_id: i64,
    new_parent_id: i64,
) {
    run_endpoint!(
        runner,
        role_reparent,
        json!({
            "site_id": site_id,
            "role_id": role_id,
            "new_parent_id": new_parent_id,
            "reparenting_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );
}

#[tokio::test]
async fn role_create_and_list() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Create a role with an invalid parent, should error
    let err = run_endpoint_err!(
        runner,
        role_create,
        json!({
            "site_id": f.site_id,
            "name": "Moderator",
            "description": "A moderator role",
            "parent_role_id": -1,
            "creating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::RoleNotFound);

    // Create a root role to attach the new role to
    let root_role = create_role(&runner, f.site_id, "Root", None).await;

    // Retry creating a role with a valid parent
    let role = run_endpoint!(
        runner,
        role_create,
        json!({
            "site_id": f.site_id,
            "name": "Moderator",
            "description": "A moderator role",
            "parent_role_id": root_role.role_id,
            "creating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_eq!(role.name, "Moderator");
    assert_eq!(role.description, "A moderator role");
    assert!(!role.is_virtual);

    // Check that the role appears in the site role listing
    let roles = run_endpoint!(runner, list_site_roles, json!({ "site_id": f.site_id }),);
    let get = roles
        .iter()
        .find(|r| r.role_id == role.role_id)
        .expect("Created role not found in site role listing");
    assert_eq!(roles.len(), 2);
    assert_eq!(get.role_id, role.role_id);
    assert_eq!(get.name, "Moderator");
}

#[tokio::test]
async fn role_update_info() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    let role = create_role(&runner, f.site_id, "OldName", None).await;

    let updated = run_endpoint!(
        runner,
        role_update_info,
        json!({
            "site_id": f.site_id,
            "role_id": role.role_id,
            "name": "NewName",
            "description": "New description",
            "updating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_eq!(updated.role_id, role.role_id);
    assert_eq!(updated.name, "NewName");
    assert_eq!(updated.description, "New description");
}

#[tokio::test]
async fn role_update_permissions_respect_hierarchy() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Parent role: page:view + page:edit
    let parent = create_role(&runner, f.site_id, "Parent", None).await;
    set_role_perms(&runner, f.site_id, parent.role_id, &["view", "edit"], false).await;

    // Child role: page:view only (subset of parent)
    let child = create_role(&runner, f.site_id, "Child", Some(parent.role_id)).await;
    set_role_perms(&runner, f.site_id, child.role_id, &["view"], false).await;

    // Test: Cannot add permissions that are not in the parent role
    let err = run_endpoint_err!(
        runner,
        role_update_permissions,
        json!({
            "site_id": f.site_id,
            "role_reference": child.role_id,
            "new_permissions": [
                { "resource_type": "page", "resource_category": null, "action": "view" },
                { "resource_type": "page", "resource_category": null, "action": "create" }, // Not in parent
            ],
            "cascade_removals": false,
            "updating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::RoleHierarchyViolation { .. });

    // Same permissions as parent is allowed
    set_role_perms(&runner, f.site_id, child.role_id, &["view", "edit"], false).await;

    // Test: Removing permission from parent without cascading should cause a hierarchy violation for the child
    let err = run_endpoint_err!(
        runner,
        role_update_permissions,
        json!({
            "site_id": f.site_id,
            "role_reference": parent.role_id,
            "new_permissions": [
                { "resource_type": "page", "resource_category": null, "action": "edit" }, // Removed view
            ],
            "cascade_removals": false, // Not cascading removal to child
            "updating_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );
    assert_contains_error!(err, ErrorType::RoleHierarchyViolation { .. });

    // Now try with cascading removals, should succeed and remove the offending permission from the child role as well
    set_role_perms(&runner, f.site_id, parent.role_id, &["edit"], true).await;

    let perms = run_endpoint!(
        runner,
        get_role_permissions,
        json!({
            "site_id": f.site_id,
            "role_reference": child.role_id,
            "human_readable_categories": false,
        }),
    );
    assert_eq!(
        perms.len(),
        1,
        "Expected child role to have 1 permission after cascading removal"
    );
}

#[tokio::test]
async fn role_delete() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Set up a role hierarchy: Role A -> Role B -> Role C
    let role_a = create_role(&runner, f.site_id, "Role A", None).await;
    let role_b = create_role(&runner, f.site_id, "Role B", Some(role_a.role_id)).await;
    let _role_c = create_role(&runner, f.site_id, "Role C", Some(role_b.role_id)).await;

    // Deleting role B without reparenting should throw an error because it has a child role
    let err = run_endpoint_err!(
        runner,
        role_delete,
        json!({
            "site_id": f.site_id,
            "role_id": role_b.role_id,
            "deleting_user_id": SYSTEM_USER_ID,
            "reparent_children": false,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::DeleteRoleWithChildren);

    // Now delete role B with reparenting, which should succeed and reparent role C under role A
    let deleted = run_endpoint!(
        runner,
        role_delete,
        json!({
            "site_id": f.site_id,
            "role_id": role_b.role_id,
            "deleting_user_id": SYSTEM_USER_ID,
            "reparent_children": true,
            "ip_address": common::IP_ADDRESS,
        }),
    );
    assert_eq!(deleted.role_id, role_b.role_id);
    assert!(
        deleted.deleted_at.is_some(),
        "Deleted role should have deleted_at timestamp set"
    );

    let roles = run_endpoint!(runner, list_site_roles, json!({ "site_id": f.site_id }),);
    assert_eq!(
        roles.len(),
        2,
        "Expected 2 roles after deletion with reparenting"
    );
    assert!(
        roles.iter().all(|r| r.role_id != role_b.role_id),
        "Deleted role should not appear in site role listing"
    );

    // Verify role C is now a child of role A
    let roles = run_endpoint!(
        runner,
        role_get,
        json!({
            "site_id": f.site_id,
            "role_reference": "Role C",
        }),
    );

    assert_eq!(
        roles.parent_role_id,
        Some(role_a.role_id),
        "Role C should now be a child of Role A after reparenting"
    );

    // Cannot delete top-level role A even with reparenting because it has no parent to reparent its children to
    let err = run_endpoint_err!(
        runner,
        role_delete,
        json!({
            "site_id": f.site_id,
            "role_id": role_a.role_id,
            "deleting_user_id": SYSTEM_USER_ID,
            "reparent_children": true,
            "ip_address": common::IP_ADDRESS,
        }),
    );
    assert_contains_error!(err, ErrorType::DeleteRoleWithChildren);
}

#[tokio::test]
async fn grant_and_revoke_role() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    let role = create_role(&runner, f.site_id, "Member", None).await;

    // Grant the role
    let user_role = run_endpoint!(
        runner,
        grant_role_to_user,
        json!({
            "site_id": f.site_id,
            "user_id": f.user_id,
            "role_id": role.role_id,
            "assigning_user_id": SYSTEM_USER_ID,
            "expires_at": null,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_eq!(user_role.user_id, f.user_id);
    assert_eq!(user_role.role_id, role.role_id);

    // Verify that the user has the role
    let user_roles = run_endpoint!(
        runner,
        get_user_roles,
        json!({
            "site_id": f.site_id,
            "user_id": f.user_id,
        }),
    );
    assert_eq!(user_roles.len(), 1);
    assert_eq!(user_roles[0].role_id, role.role_id);

    // Revoke the role
    let revoked = run_endpoint!(
        runner,
        revoke_role_from_user,
        json!({
            "site_id": f.site_id,
            "user_id": f.user_id,
            "role_id": role.role_id,
            "revoking_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert!(
        revoked.deleted_at.is_some(),
        "UserRole assignment should be deleted after revoking"
    );

    // Verify that the user no longer has the role
    let user_roles = run_endpoint!(
        runner,
        get_user_roles,
        json!({
            "site_id": f.site_id,
            "user_id": f.user_id,
        }),
    );
    assert_eq!(
        user_roles.len(),
        0,
        "User should have no roles after revocation"
    );
}

#[tokio::test]
async fn role_reparent() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Parent role: page:view + page:edit
    let parent = create_role(&runner, f.site_id, "Parent", None).await;
    set_role_perms(&runner, f.site_id, parent.role_id, &["view", "edit"], false).await;

    // Child role: page:view only (subset of parent)
    let child = create_role(&runner, f.site_id, "Child", None).await;
    set_role_perms(&runner, f.site_id, child.role_id, &["view"], false).await;

    reparent_role(&runner, f.site_id, child.role_id, parent.role_id).await;

    // Verify the new parent in the site role listing
    let roles = run_endpoint!(runner, list_site_roles, json!({ "site_id": f.site_id }),);
    let child_record = roles
        .iter()
        .find(|r| r.role_id == child.role_id)
        .expect("Child role not found in listing");
    assert_eq!(
        child_record.parent_role_id,
        Some(parent.role_id),
        "Child should now have parent set"
    );
}

#[tokio::test]
async fn role_reparent_cycle_rejected() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Role A and Role B: same permissions so we hit the cycle violation before the hierarchy violation
    let role_a = create_role(&runner, f.site_id, "RoleA", None).await;
    set_role_perms(&runner, f.site_id, role_a.role_id, &["view"], false).await;

    let role_b = create_role(&runner, f.site_id, "RoleB", None).await;
    set_role_perms(&runner, f.site_id, role_b.role_id, &["view"], false).await;

    // Parent A under B
    reparent_role(&runner, f.site_id, role_a.role_id, role_b.role_id).await;

    // Self-parenting should also be rejected
    let err = run_endpoint_err!(
        runner,
        role_reparent,
        json!({
            "site_id": f.site_id,
            "role_id": role_a.role_id,
            "new_parent_id": role_a.role_id,
            "reparenting_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::CyclicRoleViolation { .. });

    // Now attempt to parent B under A, should fail with a cycle error
    let err = run_endpoint_err!(
        runner,
        role_reparent,
        json!({
            "site_id": f.site_id,
            "role_id": role_b.role_id,
            "new_parent_id": role_a.role_id,
            "reparenting_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::CyclicRoleViolation { .. });
}

#[tokio::test]
async fn role_reparent_subset_violation_rejected() {
    let runner = TestRunner::setup().await;
    let f = RoleFixture::setup(&runner).await;

    // Role with page:view only
    let smaller = create_role(&runner, f.site_id, "Parent", None).await;
    set_role_perms(&runner, f.site_id, smaller.role_id, &["view"], false).await;

    // Role with page:view + page:edit (superset of smaller)
    let bigger = create_role(&runner, f.site_id, "Child", None).await;
    set_role_perms(&runner, f.site_id, bigger.role_id, &["view", "edit"], false).await;

    // Reparenting a role with more permissions under one with fewer should be rejected
    let err = run_endpoint_err!(
        runner,
        role_reparent,
        json!({
            "site_id": f.site_id,
            "role_id": bigger.role_id,
            "new_parent_id": smaller.role_id,
            "reparenting_user_id": SYSTEM_USER_ID,
            "ip_address": common::IP_ADDRESS,
        }),
    );

    assert_contains_error!(err, ErrorType::RoleHierarchyViolation { .. });

    // Sanity check: reparenting the role with fewer permissions under the one with more IS valid
    reparent_role(&runner, f.site_id, smaller.role_id, bigger.role_id).await;
}
