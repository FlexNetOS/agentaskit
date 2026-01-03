//! Security Test Suite - TEST-003
//!
//! Comprehensive security tests including:
//! - Capability token validation
//! - ACL enforcement
//! - Audit logging verification
//! - Penetration test framework

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, Duration};
use anyhow::Result;

#[cfg(test)]
mod capability_token_tests {
    use super::*;
    use agentaskit_core::security::{SecurityManager, Capability, CapabilityToken};

    #[tokio::test]
    async fn test_token_issuance() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        let token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution, Capability::DataAccess])
            .await
            .unwrap();

        assert_eq!(token.agent_id, agent_id);
        assert!(token.is_valid());
        assert!(token.has_capability(&Capability::TaskExecution));
        assert!(token.has_capability(&Capability::DataAccess));
        assert!(!token.has_capability(&Capability::SystemAdmin));
    }

    #[tokio::test]
    async fn test_token_validation() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        let token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Validate the token
        let validated = manager.validate_token(token.id).await.unwrap();
        assert_eq!(validated.agent_id, agent_id);
    }

    #[tokio::test]
    async fn test_token_validation_not_found() {
        let manager = SecurityManager::new().await.unwrap();
        let fake_token_id = Uuid::new_v4();

        let result = manager.validate_token(fake_token_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_revocation() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        let token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Revoke the token
        manager.revoke_token(token.id).await.unwrap();

        // Token should no longer be valid
        let result = manager.validate_token(token.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_revocation_not_found() {
        let manager = SecurityManager::new().await.unwrap();
        let fake_token_id = Uuid::new_v4();

        let result = manager.revoke_token(fake_token_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_expiry_check() {
        let token = CapabilityToken {
            id: Uuid::new_v4(),
            agent_id: Uuid::new_v4(),
            issued_at: Utc::now() - Duration::hours(25), // Issued 25 hours ago
            expires_at: Utc::now() - Duration::hours(1), // Expired 1 hour ago
            capabilities: vec![Capability::TaskExecution],
            issuer: "test".to_string(),
            signature: "test_sig".to_string(),
        };

        assert!(!token.is_valid());
        assert!(token.time_until_expiry().is_none());
    }

    #[tokio::test]
    async fn test_token_capability_check() {
        let token = CapabilityToken {
            id: Uuid::new_v4(),
            agent_id: Uuid::new_v4(),
            issued_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
            capabilities: vec![
                Capability::TaskExecution,
                Capability::DataAccess,
                Capability::NetworkAccess,
            ],
            issuer: "test".to_string(),
            signature: "test_sig".to_string(),
        };

        assert!(token.has_capability(&Capability::TaskExecution));
        assert!(token.has_capability(&Capability::DataAccess));
        assert!(token.has_capability(&Capability::NetworkAccess));
        assert!(!token.has_capability(&Capability::SystemAdmin));
        assert!(!token.has_capability(&Capability::SecurityManagement));
    }

    #[tokio::test]
    async fn test_multiple_tokens_same_agent() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue multiple tokens for same agent
        let token1 = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        let token2 = manager
            .issue_token(agent_id, vec![Capability::DataAccess])
            .await
            .unwrap();

        // Both tokens should be valid
        assert!(manager.validate_token(token1.id).await.is_ok());
        assert!(manager.validate_token(token2.id).await.is_ok());

        // They should be different tokens
        assert_ne!(token1.id, token2.id);
    }

    #[tokio::test]
    async fn test_cleanup_expired_tokens() {
        let manager = SecurityManager::new().await.unwrap();

        // Issue a token (won't expire immediately, but test the cleanup function)
        let agent_id = Uuid::new_v4();
        let _token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Cleanup should not fail
        let result = manager.cleanup_expired_tokens().await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod access_control_tests {
    use super::*;
    use agentaskit_core::security::{SecurityManager, Capability};

    #[tokio::test]
    async fn test_access_check_with_valid_token() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue token with DataAccess capability
        let _token = manager
            .issue_token(agent_id, vec![Capability::DataAccess])
            .await
            .unwrap();

        // Check access - should succeed
        let has_access = manager
            .check_access(agent_id, "resource://database", &Capability::DataAccess)
            .await
            .unwrap();

        assert!(has_access);
    }

    #[tokio::test]
    async fn test_access_check_insufficient_capability() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue token with TaskExecution capability only
        let _token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Check access for SystemAdmin - should fail
        let has_access = manager
            .check_access(agent_id, "resource://admin", &Capability::SystemAdmin)
            .await
            .unwrap();

        assert!(!has_access);
    }

    #[tokio::test]
    async fn test_access_check_no_token() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Don't issue any token - check access
        let has_access = manager
            .check_access(agent_id, "resource://anything", &Capability::TaskExecution)
            .await
            .unwrap();

        assert!(!has_access);
    }

    #[tokio::test]
    async fn test_grant_access() {
        let manager = SecurityManager::new().await.unwrap();
        let target_agent = Uuid::new_v4();
        let admin_agent = Uuid::new_v4();

        let result = manager
            .grant_access(
                "resource://secure_data".to_string(),
                target_agent,
                vec![Capability::DataAccess],
                admin_agent,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_access() {
        let manager = SecurityManager::new().await.unwrap();
        let target_agent = Uuid::new_v4();
        let admin_agent = Uuid::new_v4();

        // First grant access
        manager
            .grant_access(
                "resource://secure_data".to_string(),
                target_agent,
                vec![Capability::DataAccess],
                admin_agent,
            )
            .await
            .unwrap();

        // Then revoke
        let result = manager
            .revoke_access("resource://secure_data", target_agent, admin_agent)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_access_not_found() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();

        // Try to revoke access for non-existent resource
        let result = manager
            .revoke_access("resource://nonexistent", agent_id, admin_id)
            .await;

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod audit_logging_tests {
    use super::*;
    use agentaskit_core::security::{SecurityManager, Capability};

    #[tokio::test]
    async fn test_audit_log_token_issuance() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue a token - this should create an audit log entry
        let _token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Check audit log
        let log = manager.get_audit_log(None).await;
        assert!(!log.is_empty());

        // Find the token_issued entry
        let issuance_entry = log.iter().find(|e| e.action == "token_issued");
        assert!(issuance_entry.is_some());

        let entry = issuance_entry.unwrap();
        assert_eq!(entry.agent_id, agent_id);
        assert!(entry.success);
    }

    #[tokio::test]
    async fn test_audit_log_access_check() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue token
        let _token = manager
            .issue_token(agent_id, vec![Capability::DataAccess])
            .await
            .unwrap();

        // Perform access check
        let _has_access = manager
            .check_access(agent_id, "resource://test", &Capability::DataAccess)
            .await
            .unwrap();

        // Check audit log
        let log = manager.get_audit_log(None).await;

        let access_entry = log.iter().find(|e| e.action == "access_check");
        assert!(access_entry.is_some());

        let entry = access_entry.unwrap();
        assert_eq!(entry.agent_id, agent_id);
        assert_eq!(entry.resource, Some("resource://test".to_string()));
    }

    #[tokio::test]
    async fn test_audit_log_with_limit() {
        let manager = SecurityManager::new().await.unwrap();

        // Create multiple audit entries
        for i in 0..10 {
            let agent_id = Uuid::new_v4();
            let _token = manager
                .issue_token(agent_id, vec![Capability::TaskExecution])
                .await
                .unwrap();
        }

        // Get limited log
        let log = manager.get_audit_log(Some(5)).await;
        assert_eq!(log.len(), 5);
    }

    #[tokio::test]
    async fn test_audit_log_failed_access() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Try access without token (should fail)
        let _has_access = manager
            .check_access(agent_id, "resource://secret", &Capability::SystemAdmin)
            .await
            .unwrap();

        // Check audit log for denied entry
        let log = manager.get_audit_log(None).await;

        let denied_entry = log.iter().find(|e| e.action == "access_denied");
        assert!(denied_entry.is_some());

        let entry = denied_entry.unwrap();
        assert!(!entry.success);
        assert!(entry.error_message.is_some());
    }
}

#[cfg(test)]
mod security_stats_tests {
    use super::*;
    use agentaskit_core::security::{SecurityManager, Capability};

    #[tokio::test]
    async fn test_security_stats_initial() {
        let manager = SecurityManager::new().await.unwrap();

        let stats = manager.get_security_stats().await;

        assert_eq!(stats.active_tokens, 0);
        assert_eq!(stats.total_audit_events, 0);
        assert_eq!(stats.protected_resources, 0);
    }

    #[tokio::test]
    async fn test_security_stats_after_operations() {
        let manager = SecurityManager::new().await.unwrap();

        // Issue some tokens
        for _ in 0..5 {
            let agent_id = Uuid::new_v4();
            let _token = manager
                .issue_token(agent_id, vec![Capability::TaskExecution])
                .await
                .unwrap();
        }

        // Grant access to a resource
        manager
            .grant_access(
                "resource://protected".to_string(),
                Uuid::new_v4(),
                vec![Capability::DataAccess],
                Uuid::new_v4(),
            )
            .await
            .unwrap();

        let stats = manager.get_security_stats().await;

        assert_eq!(stats.active_tokens, 5);
        assert!(stats.total_audit_events >= 6); // 5 token issuances + 1 access grant
        assert_eq!(stats.protected_resources, 1);
    }
}

#[cfg(test)]
mod penetration_test_framework {
    use super::*;
    use agentaskit_core::security::{SecurityManager, Capability};

    #[tokio::test]
    async fn test_privilege_escalation_attempt() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        // Issue low-privilege token
        let _token = manager
            .issue_token(agent_id, vec![Capability::TaskExecution])
            .await
            .unwrap();

        // Attempt to access admin resources
        let admin_access = manager
            .check_access(agent_id, "resource://admin_console", &Capability::SystemAdmin)
            .await
            .unwrap();

        // Should be denied
        assert!(!admin_access, "Privilege escalation should be blocked");
    }

    #[tokio::test]
    async fn test_token_replay_after_revocation() {
        let manager = SecurityManager::new().await.unwrap();
        let agent_id = Uuid::new_v4();

        let token = manager
            .issue_token(agent_id, vec![Capability::DataAccess])
            .await
            .unwrap();

        let token_id = token.id;

        // Revoke the token
        manager.revoke_token(token_id).await.unwrap();

        // Attempt to validate revoked token
        let result = manager.validate_token(token_id).await;

        assert!(result.is_err(), "Revoked token should not validate");
    }

    #[tokio::test]
    async fn test_capability_boundary_enforcement() {
        let manager = SecurityManager::new().await.unwrap();

        // Test each capability is enforced properly
        let test_cases = vec![
            (Capability::TaskExecution, Capability::SystemAdmin, false),
            (Capability::DataAccess, Capability::SecurityManagement, false),
            (Capability::TaskExecution, Capability::TaskExecution, true),
            (Capability::SystemAdmin, Capability::SystemAdmin, true),
        ];

        for (granted, requested, expected) in test_cases {
            let agent_id = Uuid::new_v4();

            let _token = manager
                .issue_token(agent_id, vec![granted.clone()])
                .await
                .unwrap();

            let has_access = manager
                .check_access(agent_id, "resource://test", &requested)
                .await
                .unwrap();

            assert_eq!(
                has_access, expected,
                "Granted {:?}, requested {:?}, expected {}",
                granted, requested, expected
            );
        }
    }

    #[tokio::test]
    async fn test_concurrent_token_operations() {
        let manager = Arc::new(SecurityManager::new().await.unwrap());

        let mut handles = Vec::new();

        // Spawn concurrent token operations
        for _ in 0..50 {
            let manager = Arc::clone(&manager);

            handles.push(tokio::spawn(async move {
                let agent_id = Uuid::new_v4();

                let token = manager
                    .issue_token(agent_id, vec![Capability::TaskExecution])
                    .await
                    .unwrap();

                // Immediately validate
                let valid = manager.validate_token(token.id).await.is_ok();

                // Then revoke
                let revoked = manager.revoke_token(token.id).await.is_ok();

                (valid, revoked)
            }));
        }

        for handle in handles {
            let (valid, revoked) = handle.await.unwrap();
            assert!(valid, "Token should be valid after issuance");
            assert!(revoked, "Token should be revokable");
        }
    }
}
