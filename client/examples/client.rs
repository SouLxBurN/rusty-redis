use rusty_redis_client::Response;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Ok(mut conn) = rusty_redis_client::connect("localhost:8080").await {
        println!("Connection Established");

        let response = conn.set("1234".to_string(), b"{\"hello\":\"stream 1234!\"}".to_vec(), 10000).await?;
        println!("SET 1234 10000");
        print_response(response)?;

        let response = conn.get("1234".to_string()).await?;
        println!("GET 1234");
        print_response(response)?;

        let response = conn.set("4444".to_string(), b"{\"hello\":\"stream 4444!\"}".to_vec(), 15000).await?;
        println!("SET 4444 15000");
        print_response(response)?;

        let response = conn.set("4321".to_string(), b"{\"hello\":\"stream 4321!\"}".to_vec(), 0).await?;
        println!("SET 4321 0");
        print_response(response)?;

        let response = conn.keys().await?;
        println!("KEYS");
        print_response(response)?;

        let response = conn.delete("1234".to_string()).await?;
        println!("DEL 1234");
        print_response(response)?;

        let response = conn.get("1234".to_string()).await?;
        println!("GET 1234");
        print_response(response)?;

        let response = conn.keys().await?;
        println!("KEYS");
        print_response(response)?;

        let response = conn.get("4321".to_string()).await?;
        println!("GET 4321");
        print_response(response)?;

    } else {
        eprintln!("Failed to connect to server");
    }
    Ok(())
}

fn print_response(response: Response) -> Result<(), anyhow::Error> {
    match response {
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

