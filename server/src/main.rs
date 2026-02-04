use chat::chat_server::{Chat, ChatServer};
use chat::{ChatEventType, JoinRequest, SendMessageReply, SendMessageRequest};
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::chat::ChatEvent;

pub mod chat {
    tonic::include_proto!("chat");
}

#[derive(Debug)]
pub struct MyChat {
    tx: broadcast::Sender<ChatEvent>,
}

#[tonic::async_trait]
impl Chat for MyChat {
    type JoinStream = Pin<Box<dyn Stream<Item = Result<chat::ChatEvent, Status>> + Send>>;

    /// Enter Room
    async fn join(
        &self,
        request: Request<JoinRequest>,
    ) -> Result<Response<Self::JoinStream>, Status> {
        let rx = self.tx.subscribe();

        let chat_event = ChatEvent {
            event_type: ChatEventType::Join.into(),
            content: "joined".into(),
            room_name: "global".into(),
            user_name: request.into_inner().user_name,
            timestamp: "now".into(),
        };

        println!("{:?}", chat_event);
        let _ = self.tx.send(chat_event);

        let stream =
            BroadcastStream::new(rx).map(|r| r.map_err(|e| Status::internal(e.to_string())));

        Ok(Response::new(Box::pin(stream)))
    }

    /// Send Message
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageReply>, Status> {
        let reply = SendMessageReply { ok: true };

        let msg = request.into_inner();
        let chat_event = ChatEvent {
            event_type: ChatEventType::Message.into(),
            content: msg.content,
            room_name: msg.room_name,
            user_name: msg.user_name,
            timestamp: "now".into(),
        };
        let _ = self.tx.send(chat_event);

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let (tx, _) = broadcast::channel(100);
    let chat = MyChat { tx };

    println!("Server started at {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
