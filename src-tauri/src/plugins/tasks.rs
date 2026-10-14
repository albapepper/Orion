use crate::harness::{PluginContext, ScreenPlugin};
use crate::loans::screens::tasks as tasks_screen;
use crate::tasks::list_for_company;
use std::pin::Pin;

pub struct TasksPlugin;
impl ScreenPlugin for TasksPlugin {
    fn id(&self) -> &'static str {
        "tasks"
    }
    fn tools(&self) -> &'static [&'static str] {
        &["list_tasks"]
    }
    fn build_context<'a>(
        &'a self,
        ctx: &'a PluginContext<'a>,
    ) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let db = ctx.state.db.read().await.clone();
            let tasks_list = match db {
                Some(pool) => {
                    let company_id = ctx.state.company_id().await.map_err(|e| anyhow::anyhow!(e))?;
                    list_for_company(&pool, company_id).await?
                }
                None => crate::loans::demo::all_tasks(),
            };
            Ok(tasks_screen::build_context(&tasks_list, chrono::Utc::now()))
        })
    }
}