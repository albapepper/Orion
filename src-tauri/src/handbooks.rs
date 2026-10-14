pub fn lookup(product: &str, screen: &str) -> &'static str {
    match (product, screen) {
        ("30yr_fixed", "income") => "30yr fixed: stable W-2 preferred; 2-yr avg for bonus/commission/OT.",
        ("30yr_fixed", "terms") => "30yr fixed: standard amortization; check front/back DTI and LTV.",
        (_, "income") => "Income must be documented/verified; variable income needs 2-yr average.",
        (_, "assets") => "Reserves ~2 months PITI; source large deposits; gift needs donor+letter.",
        (_, "property") => "Occupancy drives LTV limits and pricing; condos may need review.",
        (_, "liabilities") => "Include monthly obligations; payoff at close can exclude payments.",
        _ => "No specific handbook entry; answer from IPAC snapshot + screen context.",
    }
}