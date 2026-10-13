use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::{env, sync::Arc};
use subtle::ConstantTimeEq;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    token: Arc<str>,
}
#[derive(Deserialize)]
struct DeviceInput {
    manufacturer: String,
    model: String,
    #[serde(default)]
    notes: String,
}
#[derive(Deserialize)]
struct ResearchInput {
    device_id: Uuid,
    title: String,
    body: String,
}
#[derive(Deserialize)]
struct ExperimentInput {
    device_id: Uuid,
    title: String,
    hypothesis: String,
}
#[derive(Deserialize)]
struct ArtifactInput {
    device_id: Uuid,
    name: String,
    sha256: String,
    provenance: String,
}
#[derive(Serialize)]
struct ApiError {
    error: &'static str,
}
type ApiResult = Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)>;
fn fail(code: StatusCode, msg: &'static str) -> (StatusCode, Json<ApiError>) {
    (code, Json(ApiError { error: msg }))
}
fn authorized(h: &HeaderMap, s: &AppState) -> Result<(), (StatusCode, Json<ApiError>)> {
    let candidate = h
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let expected = format!("Bearer {}", s.token);
    if candidate.len() == expected.len()
        && bool::from(candidate.as_bytes().ct_eq(expected.as_bytes()))
    {
        Ok(())
    } else {
        Err(fail(StatusCode::UNAUTHORIZED, "unauthorized"))
    }
}
fn valid(s: &str, max: usize) -> bool {
    !s.trim().is_empty() && s.chars().count() <= max
}
fn server_error(_: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    fail(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
}
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","version":"0.1.0"}))
}
async fn home() -> Html<&'static str> {
    Html(include_str!("../web/index.html"))
}
async fn devices_get(State(s): State<AppState>, h: HeaderMap) -> ApiResult {
    authorized(&h, &s)?;
    let rows=sqlx::query("SELECT id,manufacturer,model,notes,created_at FROM devices ORDER BY created_at DESC LIMIT 100").fetch_all(&s.pool).await.map_err(server_error)?;
    Ok(Json(serde_json::Value::Array(rows.into_iter().map(|r|serde_json::json!({"id":r.get::<Uuid,_>("id"),"manufacturer":r.get::<String,_>("manufacturer"),"model":r.get::<String,_>("model"),"notes":r.get::<String,_>("notes"),"created_at":r.get::<chrono::DateTime<chrono::Utc>,_>("created_at")})).collect())))
}
async fn devices_post(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(v): Json<DeviceInput>,
) -> ApiResult {
    authorized(&h, &s)?;
    if !valid(&v.manufacturer, 100) || !valid(&v.model, 150) || v.notes.chars().count() > 2000 {
        return Err(fail(StatusCode::BAD_REQUEST, "invalid_device"));
    }
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO devices(id,manufacturer,model,notes) VALUES ($1,$2,$3,$4)")
        .bind(id)
        .bind(v.manufacturer.trim())
        .bind(v.model.trim())
        .bind(v.notes.trim())
        .execute(&s.pool)
        .await
        .map_err(server_error)?;
    Ok(Json(serde_json::json!({"id":id})))
}
async fn research_post(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(v): Json<ResearchInput>,
) -> ApiResult {
    authorized(&h, &s)?;
    if !valid(&v.title, 200) || !valid(&v.body, 10000) {
        return Err(fail(StatusCode::BAD_REQUEST, "invalid_research"));
    }
    let id = Uuid::new_v4();
    let result =
        sqlx::query("INSERT INTO research_entries(id,device_id,title,body) VALUES ($1,$2,$3,$4)")
            .bind(id)
            .bind(v.device_id)
            .bind(v.title.trim())
            .bind(v.body.trim())
            .execute(&s.pool)
            .await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({"id":id}))),
        Err(sqlx::Error::Database(e)) if e.is_foreign_key_violation() => {
            Err(fail(StatusCode::BAD_REQUEST, "unknown_device"))
        }
        Err(e) => Err(server_error(e)),
    }
}
async fn experiments_post(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(v): Json<ExperimentInput>,
) -> ApiResult {
    authorized(&h, &s)?;
    if !valid(&v.title, 200) || !valid(&v.hypothesis, 2000) {
        return Err(fail(StatusCode::BAD_REQUEST, "invalid_experiment"));
    }
    let id = Uuid::new_v4();
    let result =
        sqlx::query("INSERT INTO experiments(id,device_id,title,hypothesis) VALUES ($1,$2,$3,$4)")
            .bind(id)
            .bind(v.device_id)
            .bind(v.title.trim())
            .bind(v.hypothesis.trim())
            .execute(&s.pool)
            .await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({"id":id,"status":"proposed"}))),
        Err(sqlx::Error::Database(e)) if e.is_foreign_key_violation() => {
            Err(fail(StatusCode::BAD_REQUEST, "unknown_device"))
        }
        Err(e) => Err(server_error(e)),
    }
}
async fn artifacts_post(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(v): Json<ArtifactInput>,
) -> ApiResult {
    authorized(&h, &s)?;
    if !valid(&v.name, 200)
        || !valid(&v.provenance, 2000)
        || v.sha256.len() != 64
        || !v
            .sha256
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(fail(StatusCode::BAD_REQUEST, "invalid_artifact"));
    }
    let id = Uuid::new_v4();
    let result = sqlx::query(
        "INSERT INTO artifacts(id,device_id,name,sha256,provenance) VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(id)
    .bind(v.device_id)
    .bind(v.name.trim())
    .bind(v.sha256)
    .bind(v.provenance.trim())
    .execute(&s.pool)
    .await;
    match result {
        Ok(_) => Ok(Json(serde_json::json!({"id":id}))),
        Err(sqlx::Error::Database(e)) if e.is_foreign_key_violation() => {
            Err(fail(StatusCode::BAD_REQUEST, "unknown_device"))
        }
        Err(e) => Err(server_error(e)),
    }
}
async fn overview(State(s): State<AppState>, h: HeaderMap) -> ApiResult {
    authorized(&h, &s)?;
    let devices: i64 = sqlx::query_scalar("SELECT count(*) FROM devices")
        .fetch_one(&s.pool)
        .await
        .map_err(server_error)?;
    let research: i64 = sqlx::query_scalar("SELECT count(*) FROM research_entries")
        .fetch_one(&s.pool)
        .await
        .map_err(server_error)?;
    let experiments: i64 = sqlx::query_scalar("SELECT count(*) FROM experiments")
        .fetch_one(&s.pool)
        .await
        .map_err(server_error)?;
    let artifacts: i64 = sqlx::query_scalar("SELECT count(*) FROM artifacts")
        .fetch_one(&s.pool)
        .await
        .map_err(server_error)?;
    Ok(Json(
        serde_json::json!({"devices":devices,"research":research,"experiments":experiments,"artifacts":artifacts,"recovery_verified":false}),
    ))
}
async fn export(State(s): State<AppState>, h: HeaderMap) -> ApiResult {
    authorized(&h, &s)?;
    let devices =
        sqlx::query("SELECT row_to_json(d)::text AS payload FROM devices d ORDER BY created_at")
            .fetch_all(&s.pool)
            .await
            .map_err(server_error)?;
    let research = sqlx::query(
        "SELECT row_to_json(r)::text AS payload FROM research_entries r ORDER BY created_at",
    )
    .fetch_all(&s.pool)
    .await
    .map_err(server_error)?;
    let experiments = sqlx::query(
        "SELECT row_to_json(e)::text AS payload FROM experiments e ORDER BY created_at",
    )
    .fetch_all(&s.pool)
    .await
    .map_err(server_error)?;
    let artifacts =
        sqlx::query("SELECT row_to_json(a)::text AS payload FROM artifacts a ORDER BY created_at")
            .fetch_all(&s.pool)
            .await
            .map_err(server_error)?;
    let parse = |rows: Vec<sqlx::postgres::PgRow>| -> Vec<serde_json::Value> {
        rows.iter()
            .filter_map(|r| {
                serde_json::from_str::<serde_json::Value>(&r.get::<String, _>("payload")).ok()
            })
            .collect()
    };
    Ok(Json(
        serde_json::json!({"schema_version":1,"devices":parse(devices),"research":parse(research),"experiments":parse(experiments),"artifacts":parse(artifacts),"omissions":["credentials","firmware_binaries","sensitive_device_logs"]}),
    ))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("LAB_OWNER_TOKEN").expect("LAB_OWNER_TOKEN must be set");
    assert!(
        token.len() >= 32,
        "LAB_OWNER_TOKEN must have at least 32 characters"
    );
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let state = AppState {
        pool,
        token: Arc::from(token),
    };
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/api/v1/devices", get(devices_get).post(devices_post))
        .route("/api/v1/research", post(research_post))
        .route("/api/v1/experiments", post(experiments_post))
        .route("/api/v1/artifacts", post(artifacts_post))
        .route("/api/v1/overview", get(overview))
        .route("/api/v1/export", get(export))
        .with_state(state);
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("GMSL listening on {port}");
    axum::serve(listener, app).await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn required_text_rules() {
        assert!(valid("hello", 5));
        assert!(!valid("  ", 5));
        assert!(!valid("abcdef", 5));
    }
    #[tokio::test]
    async fn rejects_missing_owner_token() {
        let s = AppState {
            pool: PgPool::connect_lazy("postgres://x:y@localhost/db").unwrap(),
            token: Arc::from("a".repeat(32)),
        };
        assert!(authorized(&HeaderMap::new(), &s).is_err());
    }
}
