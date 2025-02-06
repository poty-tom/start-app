use sqlx::{
    PgPool,
    FromRow,
};
use tauri::{
    Manager, 
    State,
    async_runtime::block_on,
};
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct Taurimon {
    pub tauri_id: String,
    pub tauri_value: String,
}

// get postgres connection pool
async fn pg_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(db_url).await?;
    Ok(pool)
}

// get taurimon records
async fn get_taurimon_rows(pool: &PgPool) -> Result<Vec<Taurimon>, sqlx::Error> {
    let taurimon_rows = sqlx::query_as::<_, Taurimon>(
        "select tauri_id, tauri_value from taurimon"
    ).fetch_all(pool).await?;
    Ok(taurimon_rows)
}

// greet command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// taurimon command
#[tauri::command]
async fn get_taurimons(pool: State<'_, sqlx::PgPool>) -> Result<Vec<Taurimon>, String> {
    let taurimons = get_taurimon_rows(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(taurimons)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool = block_on(pg_pool("postgresql://postgres:postgres@127.0.0.1:5432/postgres"))?;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_taurimons
        ])
        .setup(|app| {
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
