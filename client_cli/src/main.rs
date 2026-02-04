use crate::chat::SendMessageRequest;
use chat::JoinRequest;
use chat::chat_client::ChatClient;
use tonic::Request;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

pub mod chat {
    tonic::include_proto!("chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[31m░▒▓████████▓▒░▒▓███████▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓██████████████▓▒░░▒▓█▓▒░ 
   ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
   ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
   ░▒▓█▓▒░   ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
   ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
   ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
   ░▒▓█▓▒░  ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░    ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░ 
                                                                                                                      
                                                                                                                      \x1b[0m");

    let mut recv_client = ChatClient::connect("http://[::1]:50051").await?;
    let mut send_client = ChatClient::connect("http://[::1]:50051").await?;

    let request = Request::new(JoinRequest {
        room_name: "global".into(),
        user_name: "Jezz".into(),
    });

    let response = recv_client.join(request).await?;

    let request = Request::new(SendMessageRequest {
        content: "oi".into(),
        room_name: "global".into(),
        user_name: "Jezz".into(),
    });

    send_client.send_message(request).await?;

    let mut stream = response.into_inner();

    tokio::spawn(async move {
        loop {
            match stream.message().await {
                Ok(Some(evt)) => {
                    
                    println!("{}: {}", evt.user_name, evt.content);
                    print!("> ");
                     let _ = io::stdout().flush();
                }
                Ok(None) => {
                    println!("stream acabou");
                    break;
                }
                Err(e) => {
                    println!("erro no stream: {}", e);
                    break;
                }
            }
        }
    });

    // while let Some(evt) = stream.message().await? {
    //     println!("{}: {}", evt.user_name, evt.content);
    // }

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim().to_string();

        if line.is_empty() {
            continue;
        }
        if line == "/quit" {
            break;
        }

        let request = Request::new(SendMessageRequest {
            content: line,
            room_name: "global".into(),
            user_name: "Jazz".into(),
        });

        send_client.send_message(request).await?;
    }

    Ok(())
}
