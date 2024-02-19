use termion::async_stdin;
use termion::raw::IntoRawMode;

use std::env::args;
use std::io::stdout;
use std::slice;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::net::{TcpListener, TcpStream};

use std::io::{Read, Write};

#[tokio::main]
async fn main() -> Result<()> {
    let cmdOpt = args().nth(1).unwrap();
    if cmdOpt == "s" {
        listener().await?
    } else if cmdOpt == "c" {
        client().await?
    }
    Ok(())
}

async fn listener() -> Result<()> {
    let l = TcpListener::bind("localhost:2727").await?;

    loop {
        let (socket, _) = l.accept().await?;
        server(socket).await?
    }
}

async fn client() -> Result<()> {
    let mut client = TcpStream::connect("localhost:2727").await?;
    let _terminalRestorer = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            if key == b'q' {
                break Ok(());
            }
            client.write_all(slice::from_ref(&key)).await?;
            client.flush().await?;
        }
    }
}

async fn server(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1];
    let mut out = std::io::stdout();
    loop {
        let _len = stream.read_exact(&mut buffer).await.unwrap();
        out.write_all(&buffer)?;
        out.flush()?;
    }
}
