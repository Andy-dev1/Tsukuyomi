use crate::chat::SendMessageRequest;
use chat::JoinRequest;
use chat::chat_client::ChatClient;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::{cursor, execute, terminal::{self, Clear, ClearType}};
use std::io::{self as stdio, Write};
use std::sync::{Arc, Mutex};
use tonic::Request;

pub mod chat {
    tonic::include_proto!("chat");
}

const PROMPT: &str = "> ";

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

    let input_buf = Arc::new(Mutex::new(String::new()));
    let input_buf_recv = Arc::clone(&input_buf);

    tokio::spawn(async move {
        loop {
            match stream.message().await {
                Ok(Some(evt)) => {
                    let current_input = input_buf_recv
                        .lock()
                        .map(|s| s.clone())
                        .unwrap_or_default();
                    let mut out = stdio::stdout();
                    let _ = execute!(out, cursor::MoveToColumn(0), Clear(ClearType::CurrentLine));
                    let _ = writeln!(out, "{}: {}", evt.user_name, evt.content);
                    let _ = write!(out, "{}{}", PROMPT, current_input);
                    let _ = out.flush();
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

    // let mut lines = BufReader::new(io::stdin()).lines();
    terminal::enable_raw_mode()?;

    let mut redraw_input = |input: &str| {
        let mut out = stdio::stdout();
        let _ = execute!(out, cursor::MoveToColumn(0), Clear(ClearType::CurrentLine));
        let _ = write!(out, "{}{}", PROMPT, input);
        let _ = out.flush();
    };

    redraw_input("");

    loop {
        // bloqueia até vir uma tecla
        match event::read()? {
            Event::Key(KeyEvent {
                code, modifiers, kind, ..
            }) => {
                if kind != KeyEventKind::Press {
                    continue;
                }
                // sair com Ctrl+C
                if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }

                match code {
                    KeyCode::Enter => {
                        let msg = {
                            let mut guard = input_buf.lock().unwrap();
                            let msg = guard.trim().to_string();
                            guard.clear();
                            msg
                        };

                        // pula vazio
                        if msg.is_empty() {
                            // reimprime prompt
                            redraw_input("");
                            continue;
                        }

                        if msg == "/quit" {
                            break;
                        }

                        // envia
                        let request = Request::new(SendMessageRequest {
                            content: msg,
                            room_name: "global".into(),
                            user_name: "Jazz".into(),
                        });
                        send_client.send_message(request).await?;

                        // NÃO imprime a mensagem enviada aqui
                        // só reimprime prompt limpo
                        redraw_input("");
                    }

                    KeyCode::Backspace => {
                        let current = {
                            let mut guard = input_buf.lock().unwrap();
                            if !guard.is_empty() {
                                guard.pop();
                            }
                            guard.clone()
                        };
                        redraw_input(&current);
                    }

                    KeyCode::Char(ch) => {
                        let current = {
                            let mut guard = input_buf.lock().unwrap();
                            guard.push(ch);
                            guard.clone()
                        };
                        redraw_input(&current);
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}
