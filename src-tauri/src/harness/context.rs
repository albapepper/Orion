use crate::commands::AppState;
use uuid::Uuid;

/// Runtime handle passed to every plugin. Plugins own DB-vs-demo branching here;
/// this only carries the inputs, so collection logic stays with the plugin.
pub struct PluginContext<'a> {
    pub state: &'a AppState,
    pub loan_id: Option<Uuid>,
    pub screen: &'a str,
}

impl PluginContext<'_> {
    pub async fn pool(&self) -> Option<sqlx::PgPool> {
        self.state.db.read().await.clone()
    }
}

pub trait ScreenPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn tools(&self) -> &'static [&'static str] {
        &[]
    }
    fn instructions(&self) -> Option<&'static str> {
        None
    }
    fn build_context<'a>(
        &'a self,
        ctx: &'a PluginContext<'a>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>>;
}

pub type PluginFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>>;
