use std::process;
use tokio::runtime::Runtime;

#[test]
fn mqtt_test(){
    use std::time::Duration;
    use paho_mqtt as mqtt;
    use tokio::runtime;
    use futures_util::stream::StreamExt;

    // fn try_reconnect(cli: &mqtt::Client) -> bool {
    //     println!("Connection lost. Waiting to retry connection");
    //     for _ in 0..12 {
    //         tokio::time::sleep(Duration::from_millis(5000)).await?;
    //         if cli.reconnect().is_ok() {
    //             println!("Successfully reconnected");
    //             return true;
    //         }
    //     }
    //     println!("Unable to reconnect after several attempts.");
    //     false
    // }

    const TOPICS: &[&str] = &["test", "hello"];
    const QOS: &[i32] = &[1, 1];

    let runtime = Runtime::new().unwrap();

    let host = "tcp://mqtt.rastyletech.com:1883".to_string();

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_subscribe")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });


    if let Err(err) = runtime.block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .will_message(lwt)
            .user_name("ecoran")
            .password("Abcdef9*")
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        // Note that we're not providing a way to cleanly shut down and
        // disconnect. Therefore, when you kill this app (with a ^C or
        // whatever) the server will get an unexpected drop and then
        // should emit the LWT message.

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                
                println!("{}", msg.payload_str());
            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    println!("Error reconnecting: {:#?}", err);
                    // For tokio use: tokio::time::delay_for()
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        println!("Exiting");

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}