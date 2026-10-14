use crate::harness::{PluginContext, ScreenPlugin};
use crate::loans::{demo, screens};
use std::pin::Pin;

// ---- income ----
pub struct IncomePlugin;
impl ScreenPlugin for IncomePlugin {
    fn id(&self) -> &'static str { "income" }
    fn tools(&self) -> &'static [&'static str] { &["get_income", "get_borrowers"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (income, name) = match pool {
                Some(p) => {
                    let income = crate::db::queries::get_income(&p, ctx.loan_id.unwrap()).await?;
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    (income, crate::loans::borrower_name(&bs))
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (demo::income(lid), crate::loans::borrower_name(&demo::borrowers(lid)))
                }
            };
            Ok(screens::income::build_context(&name, &income, "conventional"))
        })
    }
}

// ---- terms ----
pub struct TermsPlugin;
impl ScreenPlugin for TermsPlugin {
    fn id(&self) -> &'static str { "terms" }
    fn tools(&self) -> &'static [&'static str] { &["get_loan_terms", "get_borrowers"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (terms, name) = match pool {
                Some(p) => {
                    let terms = crate::db::queries::get_loan_terms(&p, ctx.loan_id.unwrap()).await?
                        .unwrap_or_else(|| demo::loan_terms(ctx.loan_id.unwrap()));
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    (terms, crate::loans::borrower_name(&bs))
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (demo::loan_terms(lid), crate::loans::borrower_name(&demo::borrowers(lid)))
                }
            };
            Ok(screens::terms::build_context(&name, &terms))
        })
    }
}

// ---- borrower ----
pub struct BorrowerPlugin;
impl ScreenPlugin for BorrowerPlugin {
    fn id(&self) -> &'static str { "borrower" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_dependents"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (borrower, dependents) = match pool {
                Some(p) => {
                    let mut bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    if bs.is_empty() { bs = demo::borrowers(ctx.loan_id.unwrap()); }
                    let b = bs.into_iter().find(|x| x.is_primary).unwrap_or_else(|| demo::borrowers(ctx.loan_id.unwrap())[0].clone());
                    let d = crate::db::queries::get_dependents(&p, ctx.loan_id.unwrap()).await?;
                    (b, d)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (demo::borrowers(lid)[0].clone(), demo::dependents(lid))
                }
            };
            Ok(screens::borrower::build_context(&borrower, &dependents))
        })
    }
}

// ---- employment ----
pub struct EmploymentPlugin;
impl ScreenPlugin for EmploymentPlugin {
    fn id(&self) -> &'static str { "employment" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_employments"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, employments) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let emps = crate::db::queries::get_employments(&p, ctx.loan_id.unwrap()).await?;
                    (name, emps)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::employments(lid))
                }
            };
            Ok(screens::employment::build_context(&name, &employments))
        })
    }
}

// ---- assets ----
pub struct AssetsPlugin;
impl ScreenPlugin for AssetsPlugin {
    fn id(&self) -> &'static str { "assets" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_assets"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, assets) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let a = crate::db::queries::get_assets(&p, ctx.loan_id.unwrap()).await?;
                    (name, a)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::assets(lid))
                }
            };
            Ok(screens::assets::build_context(&name, &assets))
        })
    }
}

// ---- property ----
pub struct PropertyPlugin;
impl ScreenPlugin for PropertyPlugin {
    fn id(&self) -> &'static str { "property" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_subject_property"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, property) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let prop = crate::db::queries::get_subject_property(&p, ctx.loan_id.unwrap()).await?
                        .unwrap_or_else(|| demo::subject_property(ctx.loan_id.unwrap()));
                    (name, prop)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::subject_property(lid))
                }
            };
            Ok(screens::property::build_context(&name, &property))
        })
    }
}

// ---- declarations ----
pub struct DeclarationsPlugin;
impl ScreenPlugin for DeclarationsPlugin {
    fn id(&self) -> &'static str { "declarations" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_declarations"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, decls) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let d = crate::db::queries::get_declarations(&p, ctx.loan_id.unwrap()).await?;
                    (name, d)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::declarations(lid))
                }
            };
            Ok(screens::declarations::build_context(&name, &decls))
        })
    }
}

// ---- reo ----
pub struct ReoPlugin;
impl ScreenPlugin for ReoPlugin {
    fn id(&self) -> &'static str { "reo" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_reo"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, reo) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let r = crate::db::queries::get_reo(&p, ctx.loan_id.unwrap()).await?;
                    (name, r)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::reo(lid))
                }
            };
            Ok(screens::reo::build_context(&name, &reo))
        })
    }
}

// ---- liabilities ----
pub struct LiabilitiesPlugin;
impl ScreenPlugin for LiabilitiesPlugin {
    fn id(&self) -> &'static str { "liabilities" }
    fn tools(&self) -> &'static [&'static str] { &["get_borrowers", "get_liabilities", "get_income"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, liabilities, income) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let liab = crate::db::queries::get_liabilities(&p, ctx.loan_id.unwrap()).await?;
                    let inc = crate::db::queries::get_income(&p, ctx.loan_id.unwrap()).await?;
                    (name, liab, inc)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), demo::liabilities(lid), demo::income(lid))
                }
            };
            let total_monthly_income: f64 = income.iter().map(|i| i.monthly_amount).sum();
            Ok(screens::liabilities::build_context(&name, &liabilities, total_monthly_income))
        })
    }
}

// ---- documents ----
pub struct DocumentsPlugin;
impl ScreenPlugin for DocumentsPlugin {
    fn id(&self) -> &'static str { "documents" }
    fn tools(&self) -> &'static [&'static str] { &["list_documents"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let documents = match pool {
                Some(p) => crate::documents::list_for_loan(&p, ctx.loan_id.unwrap()).await?,
                None => demo::documents(ctx.loan_id.unwrap()),
            };
            Ok(screens::documents::build_context(&documents))
        })
    }
}

// ---- conditions ----
pub struct ConditionsPlugin;
impl ScreenPlugin for ConditionsPlugin {
    fn id(&self) -> &'static str { "conditions" }
    fn tools(&self) -> &'static [&'static str] { &["list_conditions"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let conditions = match pool {
                Some(p) => crate::conditions::list_for_loan(&p, ctx.loan_id.unwrap()).await?,
                None => demo::conditions(ctx.loan_id.unwrap()),
            };
            Ok(screens::conditions::build_context(&conditions, chrono::Utc::now()))
        })
    }
}

// ---- pricing ----
pub struct PricingPlugin;
impl ScreenPlugin for PricingPlugin {
    fn id(&self) -> &'static str { "pricing" }
    fn tools(&self) -> &'static [&'static str] { &["get_loan_terms", "list_loan_options"] }
    fn build_context<'a>(&'a self, ctx: &'a PluginContext<'a>) -> Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let pool = ctx.state.db.read().await.clone();
            let (name, terms, options) = match pool {
                Some(p) => {
                    let bs = crate::db::queries::get_borrowers(&p, ctx.loan_id.unwrap()).await?;
                    let name = crate::loans::borrower_name(&bs);
                    let terms = crate::db::queries::get_loan_terms(&p, ctx.loan_id.unwrap()).await?
                        .unwrap_or_else(|| demo::loan_terms(ctx.loan_id.unwrap()));
                    let opts = crate::pricing::list_loan_options(&p, ctx.loan_id.unwrap()).await?;
                    (name, Some(terms), opts)
                }
                None => {
                    let lid = ctx.loan_id.unwrap();
                    (crate::loans::borrower_name(&demo::borrowers(lid)), Some(demo::loan_terms(lid)), demo::loan_pricing(lid))
                }
            };
            Ok(screens::pricing::build_context(&name, terms.as_ref(), &options))
        })
    }
}