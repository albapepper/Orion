pub mod ai;
pub mod audit;
pub mod aus;
pub mod commands;
pub mod conditions;
pub mod config;
pub mod db;
pub mod documents;
pub mod loans;
pub mod locks;
pub mod mismo;
pub mod pricing;
pub mod pricing_ai;
pub mod tasks;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = config::Config::load();
    let state = AppState::new(config.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(move |app| {
            // Connect to the buyer's database in the background if configured.
            // If unreachable (or first run), the app still opens — the frontend
            // falls back to demo data and the setup wizard.
            if !config.database.url.is_empty() {
                let handle = app.handle().clone();
                let url = config.database.url.clone();
                tauri::async_runtime::spawn(async move {
                    use tauri::Manager;
                    match db::connect(&url).await {
                        Ok(pool) => {
                            let state = handle.state::<AppState>();
                            *state.db.write().await = Some(pool);
                        }
                        Err(e) => eprintln!("[orion] database connection failed: {e}"),
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::config_commands::get_status,
            commands::config_commands::get_config,
            commands::config_commands::save_config,
            commands::config_commands::test_db_connection,
            commands::config_commands::deploy_schema,
            commands::config_commands::test_ai_connection,
            commands::config_commands::list_companies,
            commands::config_commands::create_company,
            commands::config_commands::set_company,
            commands::loan_commands::list_loans,
            commands::loan_commands::get_loan_detail,
            commands::loan_commands::save_income,
            commands::loan_commands::save_borrower,
            commands::loan_commands::save_dependents,
            commands::loan_commands::save_employment,
            commands::loan_commands::delete_employment,
            commands::loan_commands::get_loan_terms,
            commands::loan_commands::delete_income,
            commands::loan_commands::save_loan_terms,
            commands::loan_commands::save_asset,
            commands::loan_commands::delete_asset,
            commands::loan_commands::save_liability,
            commands::loan_commands::delete_liability,
            commands::loan_commands::save_subject_property,
            commands::loan_commands::save_reo,
            commands::loan_commands::delete_reo,
            commands::loan_commands::save_declaration,
            commands::loan_commands::save_demographics,
            commands::loan_commands::acquire_loan_lock,
            commands::loan_commands::release_loan_lock,
            commands::loan_commands::get_loan_lock,
            commands::loan_commands::save_uw_review,
            commands::ai_commands::ai_chat,
            commands::task_commands::list_tasks,
            commands::task_commands::list_all_tasks,
            commands::task_commands::create_task,
            commands::task_commands::set_task_status,
            commands::task_commands::assign_task,
            commands::document_commands::list_documents,
            commands::document_commands::add_document,
            commands::document_commands::remove_document,
            commands::condition_commands::list_conditions,
            commands::condition_commands::create_condition,
            commands::condition_commands::set_condition_status,
            commands::condition_commands::remove_condition,
            commands::pricing_commands::list_rate_sheets,
            commands::pricing_commands::list_rate_options,
            commands::pricing_commands::create_rate_sheet,
            commands::pricing_commands::set_rate_sheet_active,
            commands::pricing_commands::add_rate_option,
            commands::pricing_commands::remove_rate_option,
            commands::pricing_commands::add_loan_pricing,
            commands::pricing_commands::remove_loan_pricing,
            commands::pricing_commands::select_loan_pricing,
            commands::pricing_commands::pricing_suggestions,
            commands::export_commands::export_loan_mismo,
            commands::export_commands::save_mismo_file,
            commands::aus_commands::submit_du,
            commands::aus_commands::submit_lpa,
            commands::user_commands::list_users,
            commands::user_commands::whoami,
            commands::user_commands::sign_in_as,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
