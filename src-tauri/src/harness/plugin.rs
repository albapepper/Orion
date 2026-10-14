use crate::harness::context::{PluginContext, ScreenPlugin};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

pub struct PluginRegistry {
    plugins: HashMap<&'static str, Arc<dyn ScreenPlugin>>,
    fallback: Arc<dyn ScreenPlugin>,
}

impl PluginRegistry {
    pub fn new(fallback: Arc<dyn ScreenPlugin>) -> Self {
        Self {
            plugins: HashMap::new(),
            fallback,
        }
    }
    pub fn register(&mut self, p: Arc<dyn ScreenPlugin>) {
        self.plugins.insert(p.id(), p);
    }
    pub fn get(&self, screen: &str) -> Arc<dyn ScreenPlugin> {
        self.plugins
            .get(screen)
            .cloned()
            .unwrap_or_else(|| self.fallback.clone())
    }
    pub fn ids(&self) -> Vec<&'static str> {
        self.plugins.keys().copied().collect()
    }
}

pub struct GenericPlugin;
impl ScreenPlugin for GenericPlugin {
    fn id(&self) -> &'static str {
        "generic"
    }
    fn build_context<'a>(
        &'a self,
        ctx: &'a PluginContext<'a>,
    ) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            Ok(format!(
                "You are a concise, expert mortgage assistant inside the Orion \
                 loan origination platform. Current screen: {}. Answer \
                 questions relevant to this screen only.",
                ctx.screen
            ))
        })
    }
}
