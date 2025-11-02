use std::sync::Arc;
use std::time::Duration;
use crate::AppState;

pub async fn queue_data_execution(state: Arc<AppState>) {
    tracing::info!("Starting queue processor for all queues");
    let retries = std::env::var("RETRIES").unwrap_or("3".to_string()).parse::<u8>().unwrap();
    // Read all existing queue names (do this once at startup)
    let state_read = state.queues.read().await;
    let queue_names: Vec<String> = state_read.keys().cloned().collect();
    drop(state_read); //  Released read lock

    // Spawn a worker per queue
    for key in queue_names {
        let original_state = Arc::clone(&state);

        tokio::spawn(async move {
            tracing::info!("Spawned worker for queue: {}", key);

            loop {
                // taking the first message
                let maybe_message = {
                    let mut queues = original_state.queues.write().await;
                    if let Some(queue) = queues.get_mut(&key) {
                        queue.pop_front()
                    } else {
                        None
                    }
                }; // dropping write lock


                if maybe_message.is_none() {
                    tracing::debug!("No data for queue '{}'", key);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }

                let message = maybe_message.unwrap();


                tracing::info!(
                    "Executing lambda for message in queue '{}': {}",
                    key,
                    message.0
                );

                let result = lambda_execution(message.0.clone()).await;

                let success = result.is_ok();


                {
                    let mut queues = original_state.queues.write().await;
                    if let Some(queue) = queues.get_mut(&key) {
                        if !success {
                            // put the failed message back at end
                            tracing::warn!(
                                "Lambda failed for queue '{}', requesting message",
                                key
                            );
                            if message.1 < retries {
                                queue.push_back((message.0, message.1+1));
                            }
                        } else {
                            tracing::info!(
                                "Successfully processed message in '{}'",
                                key
                            );
                        }
                    }
                } // dropping write lock

                // Small delay to prevent tight loop
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        });
    }
}



async fn lambda_execution(message: String) -> Result<String, String> {
    Ok(String::from("Hello, World!"))
    // here the logic for the lambda can be written by the people whom are using this,
    // based on the message type they can execute which lambda need to be called or which api need to called
}

/*
* we are going to give choice to the user on choosing how much wait time to be mentioned when the queue is empty
* and also number of retries for each message fail
*/