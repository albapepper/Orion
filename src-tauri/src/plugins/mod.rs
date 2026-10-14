pub mod dashboard;
pub mod loan;
pub mod tasks;

use crate::harness::PluginRegistry;
use std::sync::Arc;

pub fn register(reg: &mut PluginRegistry) {
    reg.register(Arc::new(dashboard::DashboardPlugin));
    reg.register(Arc::new(tasks::TasksPlugin));
    reg.register(Arc::new(loan::IncomePlugin));
    reg.register(Arc::new(loan::TermsPlugin));
    reg.register(Arc::new(loan::BorrowerPlugin));
    reg.register(Arc::new(loan::EmploymentPlugin));
    reg.register(Arc::new(loan::AssetsPlugin));
    reg.register(Arc::new(loan::PropertyPlugin));
    reg.register(Arc::new(loan::DeclarationsPlugin));
    reg.register(Arc::new(loan::ReoPlugin));
    reg.register(Arc::new(loan::LiabilitiesPlugin));
    reg.register(Arc::new(loan::DocumentsPlugin));
    reg.register(Arc::new(loan::ConditionsPlugin));
    reg.register(Arc::new(loan::PricingPlugin));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::AppState;
    use crate::config::Config;
    use crate::harness::context::ScreenPlugin;
    use crate::harness::GenericPlugin;
    use crate::harness::PluginContext;
    use uuid::Uuid;

    #[tokio::test]
    async fn every_plugin_builds_nonempty_demo_context() {
        let state = AppState::new(Config::default());
        for id in state.plugins.ids() {
            let ctx = PluginContext {
                state: &state,
                loan_id: Some(Uuid::new_v4()),
                screen: id,
            };
            let out = state.plugins.get(id).build_context(&ctx).await.unwrap();
            assert!(!out.trim().is_empty(), "plugin {id} produced empty context");
        }
        // generic fallback also works for an unregistered screen
        let ctx = PluginContext {
            state: &state,
            loan_id: None,
            screen: "overview",
        };
        let out = GenericPlugin.build_context(&ctx).await.unwrap();
        assert!(!out.trim().is_empty());
    }
}