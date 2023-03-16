use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection,Frame};

#[tokio::main]
async fn main(){
    let listener=TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket,_)=listener.accept().await.unwrap();
        // process(socket).await;
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
           process(socket).await;
        });
    }
}

async fn process(socket:TcpStream){
    // let mut connection=Connection::new(socket);
    // if let Some(frame)=connection.read_frame().await.unwrap(){
    //     println!("GOT:{:?}",frame);
    //     let response=Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

    use mini_redis::Command::{self,Get,Set};
    use std::collections::HashMap;

    let mut db=HashMap::new();
    let mut connection=Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(); {

            
        };
    }
}