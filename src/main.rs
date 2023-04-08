use std::{error};

use aws_lambda_events::s3::object_lambda::{GetObjectContext, S3ObjectLambdaEvent};
use aws_sdk_s3::Client;
use aws_smithy_http::byte_stream::ByteStream;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};


/**
This s3 object lambda handler
    * downloads the asked file
    * creates a PNG thumbnail from it
    * forward it to the browser
*/
pub(crate) async fn function_handler(
    event: LambdaEvent<S3ObjectLambdaEvent>,
    client: &Client,
) -> Result<(), Box<dyn error::Error>> {
    tracing::info!("handler starts");

    let context: GetObjectContext = event.payload.get_object_context.unwrap();

    let route = context.output_route;
    let token = context.output_token;

    tracing::info!("Route: {}", route);

    let bytes = ByteStream::from("Hello Rust".to_string().as_bytes().to_vec());

    let _write = client.write_get_object_response()
        .request_route(route)
        .request_token(token)
        .status_code(200)
        .body(bytes)
        .send().await;
 
    tracing::info!("handler ends");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let client_ref = &client;

    let func = service_fn(move |event| async move { function_handler(event, client_ref).await });

    let _ = run(func).await;

    Ok(())
}
