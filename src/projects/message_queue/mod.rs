mod message;

use std::sync::OnceLock;

use log::error;
pub use message::Message;
pub use message::project_viewed::ProjectViewed;

use async_channel::Sender;
use async_channel::Receiver;
use message::Queueable;
use tokio::task::JoinHandle;
use tokio::spawn;

static MESSAGE_QUEUE: OnceLock<MessageQueue> = OnceLock::new();

pub struct MessageQueue {
    #[allow(dead_code)]
    consumer: MessageQueueConsumer,
    producer: MessageQueueProducer,
}

impl MessageQueue {
    pub fn new() -> Self {
        let (sender, reciever) = async_channel::unbounded();
        
        Self {
            consumer: MessageQueueConsumer::new(reciever),
            producer: MessageQueueProducer::new(sender),
        }
    }
    
    pub fn producer(&self) -> MessageQueueProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct MessageQueueConsumer(JoinHandle<()>);

impl MessageQueueConsumer {
    pub fn new(receiver: Receiver<Message>) -> Self {
        Self(spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(message) => if let Err(e) = message.handle().await {
                        error!("Error handling message: {}", e)
                    }
                    Err(e) => error!("Error receiving message: {}", e),
                };
            }
        }))
    }
}

#[derive(Clone)]
pub struct MessageQueueProducer(Sender<Message>);

impl MessageQueueProducer {
    pub fn new(producer: Sender<Message>) -> Self {
        Self(producer)
    }

    pub async fn send(&self, message: impl Into<Message>) {
        self.0
            .send(message.into())
            .await
            .expect("Error sending message");
    }
}

impl Default for MessageQueueProducer {
    fn default() -> Self {
        let message_queue = MESSAGE_QUEUE.get_or_init(MessageQueue::new);

        message_queue.producer()
    }
}