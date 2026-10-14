use crate::commands::AppState;
use uuid::Uuid;

pub async fn build(
    state: &AppState,
    loan_id: Option<Uuid>,
    screen: &str,
    base_context: &str,
) -> String {
    let ipac = crate::ipac::build_summary(state, loan_id).await;
    let product = match loan_id {
        Some(lid) => selected_product(state, lid).await,
        None => "n/a".to_string(),
    };

    // ponytail: keep prompt assembly in one place so tone/rules stay consistent.
    format!(
        "Tone: friendly, knowledgeable, helpful, and non-combative. \
         Explain plainly, avoid blame, and offer next steps.\n\
         Never fabricate: answer only from the supplied screen context, \
         IPAC snapshot, or handbook; if unknown, say so and suggest \
         where to verify.\n\n\
         SELECTED PRODUCT: {product}\n\
         CURRENT SCREEN: {screen}\n\
         {ipac}\n\n\
         {base_context}\n\n\
         You may call get_handbook(product, screen) for domain rules."
    )
}

async fn selected_product(state: &AppState, loan_id: Uuid) -> String {
    let pool = state.db.read().await.clone();
    if let Some(p) = pool {
        if let Ok(opts) = crate::pricing::list_loan_options(&p, loan_id).await {
            if let Some(sel) = opts.into_iter().find(|o| o.selected) {
                return sel.product;
            }
        }
        if let Ok(Some(terms)) = crate::db::queries::get_loan_terms(&p, loan_id).await {
            return terms.mortgage_type;
        }
        "30yr_fixed".to_string()
    } else {
        crate::loans::demo::loan_terms(loan_id).mortgage_type
    }
}