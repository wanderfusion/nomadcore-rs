use axum::{
    extract::Request,
    response::Response,
    middleware::Next
};
use tracing::{span, Level};

pub async fn log_middleware(
    request: Request,
    next: Next,
) -> Response {
    // before
    let span = span!(Level::INFO, "my_span").entered();
    
    // run the function
    let response = next.run(request).await;

    // after

    // exit
    span.exit();
    response
}
