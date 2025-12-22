use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use hpair::{calculate_quantum_resistance, create_group, destroy_group, send_encrypted_message, GroupId, HPairError};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize)]
struct CreateGroupRequest {
    participants: Vec<String>,
}

#[derive(Serialize)]
struct CreateGroupResponse {
    group_id: GroupId,
}

#[derive(Deserialize)]
struct SendMessageRequest {
    sender: String,
    message: String,
}

#[derive(Serialize)]
struct SendMessageResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct QuantumResistanceRequest {
    key: Vec<u8>,
}

#[derive(Serialize)]
struct QuantumResistanceResponse {
    quantum_resistance_bits: u32,
    key_length_bytes: usize,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn create_group_handler(
    State(_state): State<Arc<()>>,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<Json<CreateGroupResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("📨 Received create_group request with {} participants", payload.participants.len());

    match create_group(payload.participants) {
        Ok(group_id) => {
            println!("✅ Group created with ID: {}", group_id);
            Ok(Json(CreateGroupResponse { group_id }))
        }
        Err(e) => {
            println!("❌ Failed to create group: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

async fn send_message_handler(
    State(_state): State<Arc<()>>,
    Path(group_id): Path<GroupId>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<SendMessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("📨 Received send_message request for group {} from {}", group_id, payload.sender);

    match send_encrypted_message(group_id, &payload.sender, &payload.message) {
        Ok(_) => {
            println!("✅ Message sent successfully from {}", payload.sender);
            Ok(Json(SendMessageResponse {
                status: "success".to_string(),
                message: format!("Message from {} sent successfully", payload.sender),
            }))
        }
        Err(e) => {
            println!("❌ Failed to send message: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

async fn destroy_group_handler(
    State(_state): State<Arc<()>>,
    Path(group_id): Path<GroupId>,
) -> Result<Json<SendMessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("📨 Received destroy_group request for group {}", group_id);

    match destroy_group(group_id) {
        Ok(_) => {
            println!("✅ Group {} destroyed successfully", group_id);
            Ok(Json(SendMessageResponse {
                status: "success".to_string(),
                message: format!("Group {} destroyed successfully", group_id),
            }))
        }
        Err(e) => {
            println!("❌ Failed to destroy group: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

async fn quantum_resistance_handler(
    State(_state): State<Arc<()>>,
    Json(payload): Json<QuantumResistanceRequest>,
) -> Result<Json<QuantumResistanceResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("📨 Received quantum_resistance request for {}-byte key", payload.key.len());

    let resistance = calculate_quantum_resistance(&payload.key);
    println!("🔐 Calculated quantum resistance: {} bits", resistance);

    Ok(Json(QuantumResistanceResponse {
        quantum_resistance_bits: resistance,
        key_length_bytes: payload.key.len(),
    }))
}

async fn health_check() -> &'static str {
    "HPair API Server is running! 🔒"
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting HPair REST API Server...");
    println!("🔒 Secure Multi-Linear Group Encryption with Quantum Resistance");
    println!("📡 Server will be available at http://localhost:3000\n");

    let app_state = Arc::new(());

    let app = Router::new()
        .route("/", get(health_check))
        .route("/groups", post(create_group_handler))
        .route("/groups/:group_id/messages", post(send_message_handler))
        .route("/groups/:group_id", delete(destroy_group_handler))
        .route("/quantum-resistance", post(quantum_resistance_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🌐 Listening on {}", addr);
    println!("📚 API Documentation:");
    println!("  POST /groups - Create a new group");
    println!("  POST /groups/:id/messages - Send encrypted message");
    println!("  DELETE /groups/:id - Destroy group");
    println!("  POST /quantum-resistance - Calculate quantum resistance");
    println!();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
