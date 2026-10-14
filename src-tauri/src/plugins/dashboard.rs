use crate::db::queries;
use crate::harness::{PluginContext, ScreenPlugin};
use crate::loans::screens::dashboard;
use std::pin::Pin;

pub struct DashboardPlugin;
impl ScreenPlugin for DashboardPlugin {
    fn id(&self) -> &'static str {
        "dashboard"
    }
    fn tools(&self) -> &'static [&'static str] {
        &["list_loans"]
    }
    fn build_context<'a>(
        &'a self,
        ctx: &'a PluginContext<'a>,
    ) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let db = ctx.state.db.read().await.clone();
            let loans = match db {
                Some(pool) => {
                    let company_id = ctx.state.company_id().await.map_err(|e| anyhow::anyhow!(e))?;
                    queries::list_loans(&pool, company_id).await?
                }
                None => crate::loans::demo::loan_summaries(),
            };
            Ok(dashboard::build_context(&loans, chrono::Utc::now()))
        })
    }
}
