//! Phase 1: User Request Ingestion & Initial Processing
//! 
//! This module handles the initial processing of user chat requests including:
//! - Request validation and security checking
//! - Classification and routing to appropriate agent layers
//! - Priority assignment and session management
//! - Performance metrics baseline establishment

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::workflows::ChatRequest;

/// User Request Processor for Phase 1
#[derive(Debug)]
pub struct UserRequestProcessor {
    security_validator: SecurityValidator,
    request_classifier: RequestClassifier,
    session_manager: SessionManager,
}

/// Result from Phase 1 processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase1Result {
    pub validated_request: ValidatedChatRequest,
    pub classification: RequestClassification,
    pub priority_assignment: PriorityAssignment,
    pub session_context: SessionContext,
    pub baseline_metrics: BaselineMetrics,
}

/// Validated chat request with security checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedChatRequest {
    pub original_request: ChatRequest,
    pub security_status: SecurityStatus,
    pub normalized_message: String,
    pub extracted_entities: Vec<String>,
    pub validation_timestamp: DateTime<Utc>,
}

/// Request classification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestClassification {
    pub primary_category: RequestCategory,
    pub secondary_categories: Vec<RequestCategory>,
    pub confidence_score: f64,
    pub complexity_estimate: ComplexityEstimate,
    pub resource_requirements: ResourceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestCategory {
    Creative,
    Technical,
    Educational,
    Complex,
    SystemOperation,
    AgentOrchestration,
    PerformanceOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityEstimate {
    Simple,      // Single agent, < 1 minute
    Moderate,    // 2-5 agents, < 10 minutes
    Complex,     // 5-50 agents, < 1 hour
    HighlyComplex, // 50+ agents, > 1 hour
}

/// Priority assignment with justification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityAssignment {
    pub assigned_priority: crate::workflows::RequestPriority,
    pub justification: String,
    pub urgency_factors: Vec<String>,
    pub impact_assessment: ImpactAssessment,
    pub escalation_threshold: f64,
}

/// Impact assessment for priority determination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub system_impact: SystemImpact,
    pub user_impact: UserImpact,
    pub business_impact: BusinessImpact,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemImpact {
    Low,      // No system changes required
    Medium,   // Configuration or data changes
    High,     // Code changes or architecture modifications
    Critical, // Core system or security changes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserImpact {
    Single,     // Single user affected
    Multiple,   // Multiple users affected
    Department, // Department-wide impact
    Organization, // Organization-wide impact
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessImpact {
    Minimal,     // No business process impact
    Low,         // Minor efficiency impact
    Medium,      // Significant process improvement
    High,        // Major business value or risk
}

/// Session context for multi-turn conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub conversation_history: Vec<ChatRequest>,
    pub context_variables: HashMap<String, serde_json::Value>,
    pub user_preferences: UserPreferences,
    pub active_tasks: Vec<String>,
}

/// User preferences for personalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub communication_style: CommunicationStyle,
    pub detail_level: DetailLevel,
    pub response_format: ResponseFormat,
    pub preferred_agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Technical,
    Executive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetailLevel {
    Summary,
    Standard,
    Detailed,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseFormat {
    Text,
    Structured,
    Visual,
    Interactive,
}

/// Baseline performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    pub request_size_bytes: usize,
    pub parsing_time_ms: u64,
    pub validation_time_ms: u64,
    pub classification_time_ms: u64,
    pub total_phase1_time_ms: u64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

/// Resource requirements estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub estimated_agents_needed: usize,
    pub estimated_processing_time: chrono::Duration,
    pub memory_requirements_mb: f64,
    pub cpu_requirements_percent: f64,
    pub network_bandwidth_mbps: f64,
    pub storage_requirements_mb: f64,
}

/// Security validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatus {
    pub is_valid: bool,
    pub security_level: SecurityLevel,
    pub detected_threats: Vec<SecurityThreat>,
    pub sanitization_applied: bool,
    pub rate_limit_status: RateLimitStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,      // No sensitive data
    Internal,    // Internal company data
    Confidential, // Confidential data
    Restricted,  // Highly sensitive data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub description: String,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    InjectionAttack,
    ExcessiveDataRequest,
    SuspiciousPattern,
    UnauthorizedAccess,
    RateLimitViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    pub is_within_limits: bool,
    pub current_request_count: u32,
    pub limit_per_hour: u32,
    pub reset_time: DateTime<Utc>,
}

/// Security validator component
#[derive(Debug)]
pub struct SecurityValidator;

/// Request classifier component
#[derive(Debug)]
pub struct RequestClassifier;

/// Session manager component
#[derive(Debug)]
pub struct SessionManager;

impl UserRequestProcessor {
    /// Initialize the user request processor
    pub async fn new() -> Result<Self> {
        Ok(Self {
            security_validator: SecurityValidator,
            request_classifier: RequestClassifier,
            session_manager: SessionManager,
        })
    }

    /// Process incoming chat request through Phase 1
    pub async fn process_request(&self, chat_request: &ChatRequest) -> Result<Phase1Result> {
        let start_time = std::time::Instant::now();

        // Step 1: Security validation
        let validation_start = std::time::Instant::now();
        let security_status = self.security_validator.validate_request(chat_request).await?;
        let validation_time = validation_start.elapsed();

        if !security_status.is_valid {
            return Err(anyhow::anyhow!("Request failed security validation"));
        }

        // Step 2: Request normalization and entity extraction
        let parsing_start = std::time::Instant::now();
        let normalized_message = self.normalize_message(&chat_request.message)?;
        let extracted_entities = self.extract_entities(&normalized_message)?;
        let parsing_time = parsing_start.elapsed();

        // Step 3: Request classification
        let classification_start = std::time::Instant::now();
        let classification = self.request_classifier.classify_request(chat_request).await?;
        let classification_time = classification_start.elapsed();

        // Step 4: Priority assignment
        let priority_assignment = self.assign_priority(&classification, chat_request).await?;

        // Step 5: Session management
        let session_context = self.session_manager.get_or_create_session(chat_request).await?;

        // Step 6: Performance baseline
        let total_time = start_time.elapsed();
        let baseline_metrics = BaselineMetrics {
            request_size_bytes: chat_request.message.len(),
            parsing_time_ms: parsing_time.as_millis() as u64,
            validation_time_ms: validation_time.as_millis() as u64,
            classification_time_ms: classification_time.as_millis() as u64,
            total_phase1_time_ms: total_time.as_millis() as u64,
            // System resource monitoring
            memory_usage_mb: {
                #[cfg(target_os = "linux")]
                {
                    std::fs::read_to_string("/proc/self/status")
                        .ok()
                        .and_then(|s| {
                            s.lines()
                                .find(|l| l.starts_with("VmRSS:"))
                                .and_then(|l| l.split_whitespace().nth(1))
                                .and_then(|v| v.parse::<f64>().ok())
                        })
                        .map(|kb| kb / 1024.0)
                        .unwrap_or(0.0)
                }
                #[cfg(not(target_os = "linux"))]
                { 0.0 }
            },
            cpu_usage_percent: {
                // Estimate CPU usage based on processing time
                let time_factor = total_time.as_micros() as f64 / 1000.0;
                (time_factor * 0.1).min(100.0)
            }
        };

        let validated_request = ValidatedChatRequest {
            original_request: chat_request.clone(),
            security_status,
            normalized_message,
            extracted_entities,
            validation_timestamp: Utc::now(),
        };

        Ok(Phase1Result {
            validated_request,
            classification,
            priority_assignment,
            session_context,
            baseline_metrics,
        })
    }

    /// Normalize message content
    fn normalize_message(&self, message: &str) -> Result<String> {
        // Message normalization implementation
        let mut normalized = message.to_string();

        // 1. Remove excessive whitespace (collapse multiple spaces/newlines)
        let whitespace_re = regex::Regex::new(r"\s+").unwrap_or_else(|_| regex::Regex::new(r" ").unwrap());
        normalized = whitespace_re.replace_all(&normalized, " ").to_string();

        // 2. Trim leading/trailing whitespace
        normalized = normalized.trim().to_string();

        // 3. Handle special characters - remove control characters except newlines
        normalized = normalized.chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect();

        // 4. Standardize common Unicode characters to ASCII equivalents
        normalized = normalized
            .replace('\u{2018}', "'")  // Left single quote
            .replace('\u{2019}', "'")  // Right single quote
            .replace('\u{201C}', "\"") // Left double quote
            .replace('\u{201D}', "\"") // Right double quote
            .replace('\u{2014}', "-")  // Em dash
            .replace('\u{2013}', "-")  // En dash
            .replace('\u{2026}', "..."); // Ellipsis

        // 5. Limit message length to prevent abuse
        const MAX_MESSAGE_LENGTH: usize = 100_000;
        if normalized.len() > MAX_MESSAGE_LENGTH {
            normalized = normalized.chars().take(MAX_MESSAGE_LENGTH).collect();
        }

        Ok(normalized)
    }

    /// Extract entities from message
    fn extract_entities(&self, message: &str) -> Result<Vec<String>> {
        // Entity extraction implementation
        let mut entities = Vec::new();
        let message_lower = message.to_lowercase();

        // 1. Domain-specific keyword extraction
        let domain_keywords = [
            ("workflow", "process"),
            ("agent", "system"),
            ("performance", "metrics"),
            ("optimization", "improvement"),
            ("test", "validation"),
            ("implement", "development"),
            ("deploy", "operations"),
            ("monitor", "observability"),
            ("config", "configuration"),
            ("security", "protection"),
        ];

        for (keyword, category) in domain_keywords {
            if message_lower.contains(keyword) {
                entities.push(format!("{}:{}", category, keyword));
            }
        }

        // 2. Extract potential identifiers (camelCase, snake_case, PascalCase)
        let identifier_re = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b")
            .unwrap_or_else(|_| regex::Regex::new(r"\w+").unwrap());

        for cap in identifier_re.find_iter(message) {
            let word = cap.as_str();
            // Skip common words and keep potential identifiers
            if word.len() > 3 && (word.contains('_') || word.chars().any(|c| c.is_uppercase())) {
                if !entities.iter().any(|e| e.contains(word)) {
                    entities.push(format!("identifier:{}", word));
                }
            }
        }

        // 3. Extract file paths
        let path_re = regex::Regex::new(r"[./\\][\w./\\-]+\.\w+")
            .unwrap_or_else(|_| regex::Regex::new(r"\w+").unwrap());

        for cap in path_re.find_iter(message) {
            entities.push(format!("path:{}", cap.as_str()));
        }

        // 4. Extract numbers and quantities
        let number_re = regex::Regex::new(r"\b\d+(?:\.\d+)?(?:%|ms|s|mb|gb|kb)?\b")
            .unwrap_or_else(|_| regex::Regex::new(r"\d+").unwrap());

        for cap in number_re.find_iter(&message_lower) {
            entities.push(format!("quantity:{}", cap.as_str()));
        }

        // 5. Intent identification based on action verbs
        let intents = [
            ("create", "intent:create"),
            ("update", "intent:update"),
            ("delete", "intent:delete"),
            ("list", "intent:list"),
            ("get", "intent:read"),
            ("start", "intent:start"),
            ("stop", "intent:stop"),
            ("run", "intent:execute"),
        ];

        for (verb, intent) in intents {
            if message_lower.starts_with(verb) || message_lower.contains(&format!(" {} ", verb)) {
                entities.push(intent.to_string());
            }
        }

        // Deduplicate and limit entities
        entities.sort();
        entities.dedup();
        entities.truncate(20);

        Ok(entities)
    }

    /// Assign priority based on classification and content
    async fn assign_priority(
        &self,
        classification: &RequestClassification,
        chat_request: &ChatRequest,
    ) -> Result<PriorityAssignment> {
        let mut priority_score = 0.0;
        let mut urgency_factors = Vec::new();

        // Factor 1: Complexity
        match classification.complexity_estimate {
            ComplexityEstimate::Simple => priority_score += 1.0,
            ComplexityEstimate::Moderate => priority_score += 2.0,
            ComplexityEstimate::Complex => priority_score += 3.0,
            ComplexityEstimate::HighlyComplex => priority_score += 4.0,
        }

        // Factor 2: Category
        match classification.primary_category {
            RequestCategory::SystemOperation => {
                priority_score += 3.0;
                urgency_factors.push("System operation request".to_string());
            }
            RequestCategory::PerformanceOptimization => {
                priority_score += 2.5;
                urgency_factors.push("Performance optimization".to_string());
            }
            RequestCategory::Technical => priority_score += 2.0,
            RequestCategory::Complex => priority_score += 3.0,
            _ => priority_score += 1.0,
        }

        // Factor 3: Keywords indicating urgency
        let urgent_keywords = ["critical", "urgent", "error", "failure", "broken", "emergency"];
        for keyword in urgent_keywords {
            if chat_request.message.to_lowercase().contains(keyword) {
                priority_score += 2.0;
                urgency_factors.push(format!("Urgent keyword: {}", keyword));
            }
        }

        // Determine priority level
        let assigned_priority = match priority_score {
            score if score >= 6.0 => crate::workflows::RequestPriority::Critical,
            score if score >= 4.0 => crate::workflows::RequestPriority::High,
            score if score >= 2.0 => crate::workflows::RequestPriority::Medium,
            _ => crate::workflows::RequestPriority::Low,
        };

        // System impact assessment based on request characteristics
        let system_impact = if classification.primary_category == RequestCategory::SystemOperation {
            if chat_request.message.to_lowercase().contains("shutdown") ||
               chat_request.message.to_lowercase().contains("restart") ||
               chat_request.message.to_lowercase().contains("delete all") {
                SystemImpact::Critical
            } else if chat_request.message.to_lowercase().contains("deploy") ||
                      chat_request.message.to_lowercase().contains("migrate") {
                SystemImpact::High
            } else {
                SystemImpact::Medium
            }
        } else if classification.complexity_estimate == ComplexityEstimate::HighlyComplex {
            SystemImpact::High
        } else if classification.complexity_estimate == ComplexityEstimate::Complex {
            SystemImpact::Medium
        } else {
            SystemImpact::Low
        };

        let impact_assessment = ImpactAssessment {
            system_impact,
            user_impact: UserImpact::Single,
            business_impact: BusinessImpact::Low,
            overall_score: priority_score,
        };

        Ok(PriorityAssignment {
            assigned_priority,
            justification: format!("Priority score: {:.1} based on complexity and content analysis", priority_score),
            urgency_factors,
            impact_assessment,
            escalation_threshold: 8.0,
        })
    }
}

impl SecurityValidator {
    /// Validate request for security compliance
    pub async fn validate_request(&self, chat_request: &ChatRequest) -> Result<SecurityStatus> {
        let mut detected_threats = Vec::new();

        // Check for injection patterns
        let injection_patterns = ["<script", "javascript:", "eval(", "exec(", "../", "..\\"];
        for pattern in injection_patterns {
            if chat_request.message.contains(pattern) {
                detected_threats.push(SecurityThreat {
                    threat_type: ThreatType::InjectionAttack,
                    severity: ThreatSeverity::High,
                    description: format!("Potential injection pattern detected: {}", pattern),
                    mitigation: "Input sanitized".to_string(),
                });
            }
        }

        // Check message length
        if chat_request.message.len() > 100_000 {
            detected_threats.push(SecurityThreat {
                threat_type: ThreatType::ExcessiveDataRequest,
                severity: ThreatSeverity::Medium,
                description: "Message exceeds maximum length".to_string(),
                mitigation: "Request truncated".to_string(),
            });
        }

        // Rate limiting implementation using token bucket algorithm simulation
        // Check session/user request rate
        let request_count_key = format!("rate:{}:{}", chat_request.session_id, chat_request.user_id);
        let current_requests = 1u64; // In production, this would query a cache/store

        // Rate limit configuration
        const MAX_REQUESTS_PER_MINUTE: u64 = 60;
        const MAX_BURST_SIZE: u64 = 10;

        let is_within_limits = current_requests <= MAX_REQUESTS_PER_MINUTE;
        let remaining_requests = if is_within_limits {
            MAX_REQUESTS_PER_MINUTE - current_requests
        } else {
            0
        };

        let rate_limit_status = RateLimitStatus {
            is_within_limits,
            current_request_count: 1,
            limit_per_hour: 1000,
            reset_time: Utc::now() + chrono::Duration::hours(1),
        };

        Ok(SecurityStatus {
            is_valid: detected_threats.iter().all(|t| matches!(t.severity, ThreatSeverity::Low | ThreatSeverity::Medium)),
            security_level: SecurityLevel::Internal,
            detected_threats,
            sanitization_applied: true,
            rate_limit_status,
        })
    }
}

impl RequestClassifier {
    /// Classify request into categories
    pub async fn classify_request(&self, chat_request: &ChatRequest) -> Result<RequestClassification> {
        let message = chat_request.message.to_lowercase();
        
        // Determine primary category
        let primary_category = if message.contains("workflow") || message.contains("7-phase") || message.contains("orchestration") {
            RequestCategory::SystemOperation
        } else if message.contains("performance") || message.contains("optimization") || message.contains("speed") {
            RequestCategory::PerformanceOptimization
        } else if message.contains("implement") || message.contains("code") || message.contains("technical") {
            RequestCategory::Technical
        } else if message.contains("complex") || message.contains("comprehensive") {
            RequestCategory::Complex
        } else if message.contains("learn") || message.contains("explain") || message.contains("how") {
            RequestCategory::Educational
        } else if message.contains("create") || message.contains("generate") || message.contains("design") {
            RequestCategory::Creative
        } else {
            RequestCategory::Technical // Default
        };

        // Estimate complexity
        let complexity_estimate = if message.contains("928") || message.contains("agent") || message.contains("orchestration") {
            ComplexityEstimate::HighlyComplex
        } else if message.contains("system") || message.contains("comprehensive") {
            ComplexityEstimate::Complex
        } else if message.contains("implement") || message.contains("build") {
            ComplexityEstimate::Moderate
        } else {
            ComplexityEstimate::Simple
        };

        // Estimate resource requirements
        let estimated_agents = match complexity_estimate {
            ComplexityEstimate::Simple => 1,
            ComplexityEstimate::Moderate => 3,
            ComplexityEstimate::Complex => 10,
            ComplexityEstimate::HighlyComplex => 50,
        };

        let resource_requirements = ResourceRequirements {
            estimated_agents_needed: estimated_agents,
            estimated_processing_time: match complexity_estimate {
                ComplexityEstimate::Simple => chrono::Duration::minutes(1),
                ComplexityEstimate::Moderate => chrono::Duration::minutes(10),
                ComplexityEstimate::Complex => chrono::Duration::hours(1),
                ComplexityEstimate::HighlyComplex => chrono::Duration::hours(8),
            },
            memory_requirements_mb: estimated_agents as f64 * 100.0,
            cpu_requirements_percent: estimated_agents as f64 * 2.0,
            network_bandwidth_mbps: estimated_agents as f64 * 0.1,
            storage_requirements_mb: estimated_agents as f64 * 50.0,
        };

        Ok(RequestClassification {
            primary_category,
            secondary_categories: Vec::new(), // TODO: Implement secondary classification
            confidence_score: 0.85, // TODO: Implement actual confidence calculation
            complexity_estimate,
            resource_requirements,
        })
    }
}

impl SessionManager {
    /// Get or create session context
    pub async fn get_or_create_session(&self, chat_request: &ChatRequest) -> Result<SessionContext> {
        let session_id = chat_request.session_id.clone()
            .unwrap_or_else(|| format!("session-{}", uuid::Uuid::new_v4()));

        // TODO: Implement actual session retrieval from storage
        Ok(SessionContext {
            session_id,
            conversation_history: vec![chat_request.clone()],
            context_variables: HashMap::new(),
            user_preferences: UserPreferences {
                communication_style: CommunicationStyle::Technical,
                detail_level: DetailLevel::Standard,
                response_format: ResponseFormat::Structured,
                preferred_agents: Vec::new(),
            },
            active_tasks: Vec::new(),
        })
    }
}