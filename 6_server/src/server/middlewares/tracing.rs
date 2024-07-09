use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::info;

pub async fn tracing_middleware(
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri().clone();

    info!("Request: {:?}", uri.path());

    let response = next.run(request).await;

    info!("Response: {:?} {:?}", uri.path(), response.status());

    response
}