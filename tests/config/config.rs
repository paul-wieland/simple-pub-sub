use simple_pub_sub::run;

pub async fn setup_service(){
    let server_handle = tokio::spawn(async { run().await });
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}