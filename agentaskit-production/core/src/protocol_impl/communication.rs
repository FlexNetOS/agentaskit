//! AgentCommunicationProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashSet;
use tokio::sync::RwLock;

use shared::protocols::AgentCommunicationProtocol;
use shared::data_models::{AgentMessage, AgentId};
use crate::communication::MessageBroker;

/// Concrete implementation of AgentCommunicationProtocol
pub struct AgentCommunicationService {
    message_broker: Arc<MessageBroker>,
    subscriptions: Arc<RwLock<HashSet<String>>>,
}

impl AgentCommunicationService {
    pub fn new(message_broker: Arc<MessageBroker>) -> Self {
        Self {
            message_broker,
            subscriptions: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

#[async_trait]
impl AgentCommunicationProtocol for AgentCommunicationService {
    async fn send_message(&self, message: AgentMessage) -> Result<()> {
        // Convert AgentMessage to internal Message format
        let internal_msg = crate::communication::Message::new(
            message.sender,
            Some(message.recipient),
            crate::communication::MessageType::Request,
            crate::communication::Priority::Normal,
            message.payload,
        );

        self.message_broker.send(internal_msg).await
    }

    async fn receive_messages(&self) -> Result<Vec<AgentMessage>> {
        // Get messages from the broker's queue
        let messages = self.message_broker.get_pending_messages().await?;

        Ok(messages.into_iter().map(|m| AgentMessage {
            id: m.id,
            sender: m.from,
            recipient: m.to.unwrap_or_default(),
            message_type: format!("{:?}", m.message_type),
            payload: m.payload,
            timestamp: m.timestamp,
            correlation_id: m.correlation_id,
        }).collect())
    }

    async fn broadcast_message(&self, message: AgentMessage, targets: Vec<AgentId>) -> Result<()> {
        for target in targets {
            let mut msg = message.clone();
            msg.recipient = target;
            self.send_message(msg).await?;
        }
        Ok(())
    }

    async fn subscribe(&self, message_types: Vec<String>) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        for msg_type in message_types {
            subs.insert(msg_type);
        }
        Ok(())
    }

    async fn unsubscribe(&self, message_types: Vec<String>) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        for msg_type in message_types {
            subs.remove(&msg_type);
        }
        Ok(())
    }
}
