use anyhow::Result;


// #[tokio::main]
// async fn main() -> Result<()>{
//     let collector = Collector::new();
//     collector.connect().await?;
//     Ok(())
// }



// #[tokio::main]
// async fn main() -> Result<()>{
//     let request_client = irelia::RequestClient::new();
//     let lcu_client = irelia::rest::LcuClient::new(false)?;

//     let champs:Vec<Champion> = lcu_client.get("/lol-champ-select/v1/all-grid-champions", &request_client).await?.unwrap();
    
//     let instant = Instant::now();

//     for champ in champs {
//         println!("{:?}", champ);
//     }
    
//     println!("{:?}", Instant::now().duration_since(instant));
//     Ok(())
// }

fn main() -> Result<()>{
    let authorize = lcu_api::get_authorize_info()?;
    println!("{:?}", authorize);
    Ok(())
}