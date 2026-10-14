use crate::commands::AppState;
use crate::loans::demo;
use uuid::Uuid;

pub async fn build_summary(state: &AppState, loan_id: Option<Uuid>) -> String {
    let Some(lid) = loan_id else {
        return "IPAC SNAPSHOT: no loan open.".to_string();
    };
    let pool = state.db.read().await.clone();
    let (income_total, assets_total, property_value, liabilities_monthly) = match pool {
        Some(p) => {
            let income = crate::db::queries::get_income(&p, lid).await.unwrap_or_default();
            let assets = crate::db::queries::get_assets(&p, lid).await.unwrap_or_default();
            let property = crate::db::queries::get_subject_property(&p, lid).await.unwrap_or(None);
            let liabilities = crate::db::queries::get_liabilities(&p, lid).await.unwrap_or_default();
            (
                income.iter().map(|i| i.monthly_amount).sum::<f64>(),
                assets.iter().map(|a| a.balance).sum::<f64>(),
                property.and_then(|pr| pr.estimated_value).unwrap_or(0.0),
                liabilities.iter().filter_map(|l| l.monthly_payment).sum::<f64>(),
            )
        }
        None => {
            let lid = lid;
            (
                demo::income(lid).iter().map(|i| i.monthly_amount).sum(),
                demo::assets(lid).iter().map(|a| a.balance).sum(),
                demo::subject_property(lid).estimated_value.unwrap_or(0.0),
                demo::liabilities(lid).iter().filter_map(|l| l.monthly_payment).sum(),
            )
        }
    };
    // ponytail: credit not tracked yet; revisit when credit pull lands.
    format!(
        "IPAC SNAPSHOT: income ${income_total:.0}/mo, assets ${assets_total:.0}, \
         property ${property_value:.0}, liabilities ${liabilities_monthly:.0}/mo, \
         credit: not pulled"
    )
}