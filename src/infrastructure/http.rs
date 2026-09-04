use std::time::Duration;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::{
    catch_panic::CatchPanicLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    application::{OrderBookService, OrderCommand, OrderCommandResult, ServiceError},
    domain::{BookError, NewOrder, OrderId, OrderValidationError, Side, TimeInForce},
};

const DEFAULT_DEPTH: usize = 20;
const MAX_DEPTH: usize = 100;
const MAX_REQUEST_BYTES: usize = 64 * 1024;

#[derive(Clone)]
struct AppState {
    service: OrderBookService,
}

pub fn router(service: OrderBookService) -> Router {
    let state = AppState { service };

    Router::new()
        .route("/health", get(health))
        .route("/v1/orderbook", get(snapshot))
        .route("/v1/orders", post(submit_order))
        .route("/v1/orders/{order_id}", delete(cancel_order))
        .with_state(state)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BYTES))
        .layer(CatchPanicLayer::new())
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, ApiError> {
    state.service.snapshot(0).map_err(ApiError::from)?;
    Ok(Json(HealthResponse {
        status: "ok",
        service: "rustilleus",
        version: env!("CARGO_PKG_VERSION"),
    }))
}

#[derive(Debug, Deserialize)]
struct DepthQuery {
    depth: Option<usize>,
}

async fn snapshot(
    State(state): State<AppState>,
    query: Result<Query<DepthQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query.map_err(|error| {
        ApiError::new(StatusCode::BAD_REQUEST, "invalid_query", error.body_text())
    })?;
    let depth = query.depth.unwrap_or(DEFAULT_DEPTH);
    if !(1..=MAX_DEPTH).contains(&depth) {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_depth",
            format!("depth must be between 1 and {MAX_DEPTH}"),
        ));
    }

    let snapshot = state.service.snapshot(depth).map_err(ApiError::from)?;
    Ok(Json(snapshot))
}

#[derive(Debug, Deserialize)]
struct SubmitOrderRequest {
    order_id: u64,
    side: Side,
    price: u64,
    quantity: u64,
    #[serde(default)]
    time_in_force: TimeInForce,
    #[serde(default)]
    post_only: bool,
}

impl TryFrom<SubmitOrderRequest> for NewOrder {
    type Error = OrderValidationError;

    fn try_from(request: SubmitOrderRequest) -> Result<Self, Self::Error> {
        NewOrder::new(
            request.order_id,
            request.side,
            request.price,
            request.quantity,
            request.time_in_force,
            request.post_only,
        )
    }
}

async fn submit_order(
    State(state): State<AppState>,
    request: Result<Json<SubmitOrderRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = request.map_err(|error| {
        ApiError::new(StatusCode::BAD_REQUEST, "invalid_json", error.body_text())
    })?;
    let order = NewOrder::try_from(request).map_err(ApiError::from)?;
    let result = state
        .service
        .execute(OrderCommand::Submit(order))
        .map_err(ApiError::from)?;

    match result {
        OrderCommandResult::Submitted(report) => Ok((StatusCode::CREATED, Json(report))),
        OrderCommandResult::Cancelled(_) => unreachable!("submit command returned cancel result"),
    }
}

async fn cancel_order(
    State(state): State<AppState>,
    order_id: Result<Path<u64>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(raw_order_id) = order_id.map_err(|error| {
        ApiError::new(StatusCode::BAD_REQUEST, "invalid_path", error.body_text())
    })?;
    let order_id = OrderId::new(raw_order_id).map_err(ApiError::from)?;
    let result = state
        .service
        .execute(OrderCommand::Cancel(order_id))
        .map_err(ApiError::from)?;

    match result {
        OrderCommandResult::Cancelled(report) => Ok(Json(report)),
        OrderCommandResult::Submitted(_) => unreachable!("cancel command returned submit result"),
    }
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    body: ErrorBody,
}

impl ApiError {
    fn new(status: StatusCode, code: &'static str, message: String) -> Self {
        Self {
            status,
            body: ErrorBody { code, message },
        }
    }
}

impl From<OrderValidationError> for ApiError {
    fn from(error: OrderValidationError) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_order",
            error.to_string(),
        )
    }
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::Book(BookError::DuplicateOrder(_)) => {
                Self::new(StatusCode::CONFLICT, "duplicate_order", error.to_string())
            }
            ServiceError::Book(BookError::OrderNotFound(_)) => {
                Self::new(StatusCode::NOT_FOUND, "order_not_found", error.to_string())
            }
            ServiceError::Book(BookError::PostOnlyWouldCross(_)) => Self::new(
                StatusCode::CONFLICT,
                "post_only_would_cross",
                error.to_string(),
            ),
            ServiceError::Book(BookError::InvariantViolation) | ServiceError::EngineUnavailable => {
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "the matching engine could not process the request".to_owned(),
                )
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::{Body, to_bytes},
        http::{Request, header::CONTENT_TYPE},
    };
    use serde_json::Value;
    use tower::ServiceExt;

    use super::*;
    use crate::{domain::OrderBook, infrastructure::events::TracingEventSink};

    fn test_app() -> Router {
        let service = OrderBookService::new(
            Box::new(OrderBook::new("TEST-USD")),
            Arc::new(TracingEventSink),
        );
        router(service)
    }

    async fn json_body(response: Response) -> Value {
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_endpoint_reports_version() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = json_body(response).await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn submit_then_snapshot_exposes_resting_order() {
        let app = test_app();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/orders")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"order_id":1,"side":"buy","price":100,"quantity":5}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/orderbook?depth=10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = json_body(response).await;

        assert_eq!(body["symbol"], "TEST-USD");
        assert_eq!(body["bids"][0]["price"], 100);
        assert_eq!(body["bids"][0]["total_quantity"], 5);
    }

    #[tokio::test]
    async fn invalid_order_returns_machine_readable_error() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/orders")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"order_id":1,"side":"buy","price":0,"quantity":5}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(json_body(response).await["code"], "invalid_order");
    }

    #[tokio::test]
    async fn malformed_json_returns_machine_readable_error() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/orders")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"order_id":1,"side":not-json}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(json_body(response).await["code"], "invalid_json");
    }

    #[tokio::test]
    async fn response_contains_request_id() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.headers().contains_key("x-request-id"));
    }
}
