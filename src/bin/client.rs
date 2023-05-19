use rusty_redis::response::Response;
use rusty_redis::command::Command;
use rusty_redis::{client, RedisConnection};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Ok(mut conn) = client::connect("localhost:8080").await {
        println!("Connection Established");
        let _ = conn.write_command(Command::SET("1234".to_string(), b"{\"hello\":\"stream 1234!\"}".to_vec())).await?;
        println!("SET 1234");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::GET("1234".to_string())).await?;
        println!("GET 1234");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::SET("4444".to_string(), b"{\"hello\":\"stream 4444!\"}".to_vec())).await?;
        println!("SET 4444");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::SET("4321".to_string(), b"{\"hello\":\"stream 4321!\"}".to_vec())).await?;
        println!("SET 4321");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::KEYS).await?;
        println!("KEYS");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::DELETE("1234".to_string())).await?;
        println!("DEL 1234");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::GET("1234".to_string())).await?;
        println!("GET 1234");
        wait_and_read_response(&mut conn).await?;

        let _ = conn.write_command(Command::KEYS).await?;
        println!("KEYS");
        wait_and_read_response(&mut conn).await?;

    } else {
        eprintln!("Failed to connect to server");
    }
    Ok(())
}

async fn wait_and_read_response(conn: &mut RedisConnection<TcpStream>) -> Result<(), anyhow::Error> {
    match conn.read_response().await? {
        Response::Empty => println!("Empty Response"),
        Response::Error(s) => println!("{s}"),
        Response::String(s) => println!("{s}"),
        Response::Int(_) => todo!(),
        Response::Float(_) => todo!(),
        Response::Array(list) => list.iter().for_each(|s| println!("{s}")),
        Response::Data(data) => println!("{}", std::str::from_utf8(data.as_slice()).unwrap()),
    };
    Ok(())
}
