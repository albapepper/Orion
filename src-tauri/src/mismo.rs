//! P4-04: MISMO 3.4 serialization — a complete loan in, 1003 XML out,
//! per docs/1003-fields.md.
//!
//! The shape follows the reference model: MESSAGE → DEAL_SETS →
//! DEAL_SET → DEALS → DEAL, with the loan under LOANS/LOAN, people
//! under PARTIES (BORROWER hangs off ROLE), and the subject property
//! under COLLATERALS. Orion stores person-owned data in loan-scoped
//! tables keyed on borrower_id — that FK becomes the party grouping
//! here, which is what the xlink relationship would encode.
//!
//! Paths the field reference marks ⚠ (exact nesting varies between
//! model versions) are emitted in their most common GSE-accepted form
//! and carry a `<!-- ULAD: ... -->` comment in the output so the
//! pending ULAD confirmation (PT-02..04) shows up at the point of use.
//!
//! The serializer is pure and dependency-free: no XML crate, just a
//! small escaping writer. DU/LPA sandboxes (P4-05/06) submit whatever
//! this produces, so the money fields are the entered values — ratios
//! like LTV are computed here on the fly, never stored pre-rounded.

use crate::loans::{
    Asset, Borrower, BorrowerDependent, Declaration, Demographics, Employment, Liability,
    LoanIncome, LoanTerms, ReoProperty, SubjectProperty,
};

/// Everything the serializer needs. The export command assembles this
/// from a loan detail load, so real and demo data serialize identically.
pub struct MismoInput<'a> {
    pub loan_number: &'a str,
    pub terms: Option<&'a LoanTerms>,
    pub borrowers: &'a [Borrower],
    pub dependents: &'a [BorrowerDependent],
    pub employments: &'a [Employment],
    pub income: &'a [LoanIncome],
    pub assets: &'a [Asset],
    pub liabilities: &'a [Liability],
    pub property: Option<&'a SubjectProperty>,
    pub reo: &'a [ReoProperty],
    pub declarations: &'a [Declaration],
    pub demographics: &'a [Demographics],
}

// ── XML writer ───────────────────────────────────────────────────────

/// Escape text content and attribute values for XML.
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Indentation-aware XML builder. Elements are emitted explicitly
/// (open/leaf/close); None-valued fields are omitted entirely, which
/// is what MISMO consumers expect over empty tags.
struct Xml {
    buf: String,
    depth: usize,
}

impl Xml {
    fn new() -> Self {
        Self {
            buf: String::new(),
            depth: 0,
        }
    }

    fn pad(&mut self) {
        for _ in 0..self.depth {
            self.buf.push_str("  ");
        }
    }

    fn open(&mut self, tag: &str) {
        self.pad();
        self.buf.push('<');
        self.buf.push_str(tag);
        self.buf.push_str(">\n");
        self.depth += 1;
    }

    fn open_attr(&mut self, tag: &str, attr: &str, value: &str) {
        self.pad();
        self.buf.push('<');
        self.buf.push_str(tag);
        self.buf.push_str(&format!(" {attr}=\"{}\">\n", esc(value)));
        self.depth += 1;
    }

    fn comment(&mut self, text: &str) {
        self.pad();
        self.buf.push_str(&format!("<!-- {text} -->\n"));
    }

    fn leaf(&mut self, tag: &str, value: &str) {
        self.pad();
        self.buf.push_str(&format!("<{tag}>{}</{tag}>\n", esc(value)));
    }

    /// Leaf with a serialized number already formatted (no re-escaping
    /// concerns, but keep one path for safety).
    fn leaf_num(&mut self, tag: &str, value: impl std::fmt::Display) {
        self.leaf(tag, &value.to_string());
    }

    fn close(&mut self, tag: &str) {
        self.depth -= 1;
        self.pad();
        self.buf.push_str(&format!("</{tag}>\n"));
    }
}

// ── Orion value → MISMO enumeration maps ─────────────────────────────
// Orion stores snake_case display-agnostic values; the GSEs want the
// MISMO enumeration. Unknown values fall through to Other rather than
// being invented.

fn loan_purpose(v: &str) -> &'static str {
    match v {
        "purchase" => "Purchase",
        "refinance" => "Refinance",
        "cash_out" => "CashOutRefinance",
        "construction" => "ConstructionToPermanent",
        "home_improvement" => "HomeImprovement",
        _ => "Other",
    }
}

fn mortgage_type(v: &str) -> &'static str {
    match v {
        "conventional" => "Conventional",
        "fha" => "FHA",
        "va" => "VA",
        "usda" => "USDA",
        _ => "Other",
    }
}

fn amortization_type(v: &str) -> &'static str {
    match v {
        "fixed" => "Fixed",
        "arm" => "AdjustableRate",
        _ => "Other",
    }
}

fn lien_priority(v: &str) -> &'static str {
    match v {
        "first" => "FirstLien",
        "second" => "SecondLien",
        _ => "Other",
    }
}

fn marital_status(v: &str) -> &'static str {
    match v {
        "married" => "Married",
        "unmarried" => "Unmarried",
        "separated" => "Separated",
        _ => "Unknown",
    }
}

fn citizenship_status(v: &str) -> &'static str {
    match v {
        "us_citizen" => "USCitizen",
        "permanent_resident" => "PermanentResidentAlien",
        "non_resident" => "NonResidentAlien",
        _ => "Unknown",
    }
}

fn account_type(v: &str) -> &'static str {
    match v {
        "checking" => "CheckingAsset",
        "savings" => "SavingsAsset",
        "money_market" => "MoneyMarket",
        "cd" => "CertificateOfDeposit",
        "retirement" => "RetirementFund",
        "mutual_fund" => "MutualFund",
        "gift" => "GiftFunds",
        "bridge" => "BridgeLoanProceeds",
        _ => "Other",
    }
}

fn liability_type(v: &str) -> &'static str {
    match v {
        "mortgage" => "MortgageLoan",
        "installment" => "Installment",
        "revolving" => "Revolving",
        "open" => "Open",
        "lease" => "Lease",
        _ => "Other",
    }
}

fn property_type(v: &str) -> &'static str {
    match v {
        "detached" => "Detached",
        "condo" => "Condominium",
        "pud" => "PUD",
        "townhouse" => "Townhouse",
        "manufactured" => "ManufacturedHousing",
        "multi_family" => "MultiFamily",
        "cooperative" => "Cooperative",
        _ => "Other",
    }
}

fn occupancy(v: &str) -> &'static str {
    match v {
        "primary" => "PrimaryResidence",
        "second" => "SecondHome",
        "investment" => "Investment",
        _ => "Other",
    }
}

fn income_type(v: &str) -> &'static str {
    match v {
        "base" => "Base",
        "overtime" => "Overtime",
        "bonus" => "Bonus",
        "commission" => "Commissions",
        "self_employed" => "SelfEmployed",
        "retirement" => "Retirement",
        "social_security" => "SocialSecurity",
        "military_benefits" => "MilitaryBenefit",
        "rental" => "RentalIncome",
        _ => "OtherIncome",
    }
}

fn disposition(v: &str) -> &'static str {
    match v {
        "pending_sale" => "PendingSale",
        "retain" => "Retain",
        "sold" => "Sold",
        "rental" => "Rental",
        _ => "Other",
    }
}

fn sex(v: &str) -> &'static str {
    match v {
        "male" => "Male",
        "female" => "Female",
        _ => "NotProvided",
    }
}

fn ethnicity_type(v: &str) -> &'static str {
    match v {
        "hispanic_or_latino" => "HispanicOrLatino",
        "not_hispanic_or_latino" => "NotHispanicOrLatino",
        _ => "InformationNotProvided",
    }
}

fn race_type(v: &str) -> &'static str {
    match v {
        "american_indian_or_alaska_native" => "AmericanIndianOrAlaskaNative",
        "asian" => "Asian",
        "black_or_african_american" => "BlackOrAfricanAmerican",
        "native_hawaiian_or_pacific_islander" => "NativeHawaiianOrOtherPacificIslander",
        "white" => "White",
        _ => "InformationNotProvided",
    }
}

// ── Number formatting ────────────────────────────────────────────────

/// Amounts carry cents, no grouping: "560000.00".
fn amount(v: f64) -> String {
    format!("{v:.2}")
}

/// Rates/percent points keep their entered precision: "6.375".
fn percent(v: f64) -> String {
    let s = format!("{v:.3}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

// ── The serializer ───────────────────────────────────────────────────

pub fn serialize(input: &MismoInput) -> String {
    let mut x = Xml::new();
    x.buf.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    x.buf.push('\n');
    // Namespace per the MISMO 3.4 reference model.
    x.open_attr(
        "MESSAGE",
        "xmlns",
        "http://www.mismo.org/residential/2009/schemas",
    );
    x.open("DEAL_SETS");
    x.open("DEAL_SET");
    x.open("DEALS");
    x.open_attr("DEAL", "ID", "Deal-1");

    serialize_loan(&mut x, input);
    serialize_parties(&mut x, input);
    serialize_collateral(&mut x, input);

    x.close("DEAL");
    x.close("DEALS");
    x.close("DEAL_SET");
    x.close("DEAL_SETS");
    x.close("MESSAGE");
    x.buf
}

// ── LOANS/LOAN — the deal shape (field reference §1) ─────────────────

fn serialize_loan(x: &mut Xml, input: &MismoInput) {
    x.open("LOANS");
    x.open_attr("LOAN", "ID", "Loan-1");

    if let Some(t) = input.terms {
        x.open("TERMS_OF_LOAN");
        if let Some(a) = t.loan_amount {
            x.leaf_num("NoteAmount", amount(a));
        }
        if let Some(r) = t.interest_rate {
            x.leaf_num("NoteRatePercent", percent(r));
        }
        if let Some(m) = t.term_months {
            x.leaf_num("LoanAmortizationTermMonths", m);
        }
        x.leaf("LienPriorityType", lien_priority(&t.lien_priority));
        x.close("TERMS_OF_LOAN");

        x.open("AMORTIZATION");
        x.open("AMORTIZATION_DETAIL");
        x.leaf("AmortizationType", amortization_type(&t.amortization_type));
        x.close("AMORTIZATION_DETAIL");
        x.close("AMORTIZATION");

        x.open("LOAN_DETAIL");
        x.leaf("MortgageType", mortgage_type(&t.mortgage_type));
        // LTV is computed from entered values on read — never stored.
        if let (Some(a), Some(v)) = (t.loan_amount, t.property_value) {
            if v > 0.0 {
                x.leaf_num("LTVValue", percent(a / v * 100.0));
            }
        }
        x.close("LOAN_DETAIL");

        x.open("APPLICATION");
        if let Some(d) = t.application_date {
            x.leaf("ApplicationDate", &d.to_string());
        }
        x.leaf("LoanPurposeType", loan_purpose(&t.loan_purpose));
        x.close("APPLICATION");
    }

    x.close("LOAN");
    x.close("LOANS");
}

// ── PARTIES — one PARTY per borrower (field reference §2–§10) ────────

fn serialize_parties(x: &mut Xml, input: &MismoInput) {
    if input.borrowers.is_empty() {
        return;
    }
    x.open("PARTIES");
    for (i, b) in input.borrowers.iter().enumerate() {
        x.open_attr("PARTY", "ID", &format!("Party-{}", i + 1));

        // Contact points live on the PARTY, not inside BORROWER.
        if b.email.is_some() || b.phone.is_some() {
            x.open("CONTACT_POINTS");
            if let Some(email) = &b.email {
                x.open("CONTACT_POINT");
                x.leaf("ContactPointType", "Email");
                x.open("CONTACT_DETAIL");
                x.leaf("Email", email);
                x.close("CONTACT_DETAIL");
                x.close("CONTACT_POINT");
            }
            if let Some(phone) = &b.phone {
                x.open("CONTACT_POINT");
                x.leaf("ContactPointType", "Home");
                x.open("CONTACT_DETAIL");
                x.leaf("Telephone", phone);
                x.close("CONTACT_DETAIL");
                x.close("CONTACT_POINT");
            }
            x.close("CONTACT_POINTS");
        }

        x.open("ROLES");
        x.open("ROLE");
        x.open("ROLE_DETAIL");
        x.leaf("RoleType", "Borrower");
        x.close("ROLE_DETAIL");

        x.open("BORROWER");
        serialize_borrower_core(x, b, i, input);
        x.close("BORROWER");

        x.close("ROLE");
        x.close("ROLES");
        x.close("PARTY");
    }
    x.close("PARTIES");
}

fn serialize_borrower_core(x: &mut Xml, b: &Borrower, idx: usize, input: &MismoInput) {
    x.open("BORROWER_DETAIL");
    x.leaf("FirstName", &b.first_name);
    if let Some(m) = &b.middle_name {
        x.leaf("MiddleName", m);
    }
    x.leaf("LastName", &b.last_name);
    if let Some(s) = &b.suffix_name {
        x.leaf("SuffixName", s);
    }
    // Full SSN belongs in the export payload (DU requires it) — the
    // never-logged rule is about audit and AI context, not the 1003.
    if let Some(ssn) = &b.ssn {
        x.leaf("SSN", ssn);
    }
    if let Some(d) = b.birth_date {
        x.leaf("BirthDate", &d.to_string());
    }
    if let Some(m) = &b.marital_status {
        x.leaf("MaritalStatusType", marital_status(m));
    }
    if let Some(c) = &b.citizenship_status {
        x.leaf("CitizenshipStatusType", citizenship_status(c));
    }
    x.close("BORROWER_DETAIL");

    serialize_dependents(x, b, input);
    serialize_employers(x, b, input);
    serialize_income(x, b, input);
    serialize_assets(x, b, input);
    serialize_liabilities(x, b, input);
    serialize_reo(x, b, input);
    serialize_declarations(x, b, input);
    serialize_demographics(x, idx, input);
}

fn serialize_dependents(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .dependents
        .iter()
        .filter(|d| d.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("DEPENDENTS");
    x.leaf_num("DependentCount", mine.len());
    // MISMO carries a count plus an age list, not identified children.
    let ages: Vec<String> = mine
        .iter()
        .filter_map(|d| d.age.map(|a| a.to_string()))
        .collect();
    if !ages.is_empty() {
        x.leaf("DependentAges", &ages.join(","));
    }
    x.close("DEPENDENTS");
}

fn serialize_employers(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .employments
        .iter()
        .filter(|e| e.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("EMPLOYERS");
    for e in mine {
        x.open("EMPLOYER");
        x.open("EMPLOYER_DETAIL");
        if let Some(name) = &e.employer_name {
            x.leaf("EmployerName", name);
        }
        if let Some(p) = &e.position {
            x.leaf("PositionDescription", p);
        }
        if let Some(p) = &e.business_phone {
            x.leaf("BusinessPhone", p);
        }
        x.close("EMPLOYER_DETAIL");
        x.open("CURRENT_EMPLOYMENT");
        x.leaf(
            "EmploymentClassificationType",
            if e.self_employed {
                "SelfEmployed"
            } else {
                "SalariedOrWageEarner"
            },
        );
        x.leaf("CurrentIndicator", if e.is_current { "true" } else { "false" });
        if let Some(m) = e.monthly_income {
            x.leaf_num("MonthlyIncomeAmount", amount(m));
        }
        if let Some(d) = e.start_date {
            x.leaf("EmploymentStartDate", &d.to_string());
        }
        if let Some(d) = e.end_date {
            x.leaf("EmploymentEndDate", &d.to_string());
        }
        if let Some(m) = e.months_in_line_of_work {
            x.leaf_num("TimeInLineOfWorkMonths", m);
        }
        x.close("CURRENT_EMPLOYMENT");
        x.close("EMPLOYER");
    }
    x.close("EMPLOYERS");
}

fn serialize_income(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .income
        .iter()
        .filter(|i| i.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("CURRENT_INCOMES");
    for i in mine {
        x.open("CURRENT_INCOME");
        x.leaf("CurrentIncomeType", income_type(&i.income_type));
        x.leaf_num("CurrentIncomeMonthlyTotalAmount", amount(i.monthly_amount));
        // MISMO carries monthly only — the 2-year average stays an
        // Orion-computed column and doesn't serialize.
        let mut desc = Vec::new();
        if let Some(e) = &i.employer_name {
            desc.push(format!("Employer: {e}"));
        }
        if let Some(n) = &i.notes {
            desc.push(n.clone());
        }
        if !desc.is_empty() {
            x.leaf("CurrentIncomeDescription", &desc.join(" — "));
        }
        x.close("CURRENT_INCOME");
    }
    x.close("CURRENT_INCOMES");
}

fn serialize_assets(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .assets
        .iter()
        .filter(|a| a.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("ASSETS");
    for a in mine {
        x.open("ACCOUNT");
        x.open("ACCOUNT_DETAIL");
        x.leaf("AccountType", account_type(&a.account_type));
        if let Some(inst) = &a.institution {
            x.leaf("AccountIssuerName", inst);
        }
        x.close("ACCOUNT_DETAIL");
        x.leaf_num("BookValue", amount(a.balance));
        if a.account_type == "gift" {
            // Gift sources sit in a dedicated container in 3.4; Orion
            // stores donor + relationship, serialized as captured.
            x.open("GIFT_GRANT");
            x.open("GIFT_GRANT_DETAIL");
            if let Some(d) = &a.gift_donor {
                x.leaf("DonorName", d);
            }
            if let Some(r) = &a.gift_relationship {
                x.leaf("DonorRelationshipDescription", r);
            }
            x.close("GIFT_GRANT_DETAIL");
            x.close("GIFT_GRANT");
        }
        if let Some(n) = &a.notes {
            x.leaf("Description", n);
        }
        x.close("ACCOUNT");
    }
    x.close("ASSETS");
}

fn serialize_liabilities(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .liabilities
        .iter()
        .filter(|l| l.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("LIABILITIES");
    for l in mine {
        x.open("LIABILITY");
        x.leaf("LiabilityType", liability_type(&l.liability_type));
        if let Some(c) = &l.creditor_name {
            x.leaf("CreditorName", c);
        }
        if let Some(p) = l.monthly_payment {
            x.leaf_num("MonthlyPaymentAmount", amount(p));
        }
        if let Some(u) = l.unpaid_balance {
            x.leaf_num("UnpaidBalanceAmount", amount(u));
        }
        x.leaf(
            "WillBePaidOffIndicator",
            if l.payoff_at_close { "true" } else { "false" },
        );
        x.close("LIABILITY");
    }
    x.close("LIABILITIES");
}

fn serialize_reo(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .reo
        .iter()
        .filter(|r| r.borrower_id == b.id)
        .collect();
    if mine.is_empty() {
        return;
    }
    // ULAD: REO is xlink-linked rather than nested in some model
    // versions — confirm before GSE certification.
    x.comment("ULAD: confirm REO path (xlink vs nested) before certification");
    x.open("REO_PROPERTIES");
    for r in mine {
        x.open("REO_PROPERTY");
        x.open("ADDRESS");
        if let Some(a) = &r.address_line {
            x.leaf("AddressLineText", a);
        }
        if let Some(c) = &r.city {
            x.leaf("CityName", c);
        }
        if let Some(s) = &r.state {
            x.leaf("StateCode", s);
        }
        if let Some(z) = &r.postal_code {
            x.leaf("PostalCode", z);
        }
        x.close("ADDRESS");
        x.open("PROPERTY_DETAIL");
        x.leaf("PropertyType", property_type(&r.property_type));
        x.close("PROPERTY_DETAIL");
        if let Some(v) = r.market_value {
            x.leaf_num("MarketValueAmount", amount(v));
        }
        if let Some(p) = r.mortgage_payment {
            x.leaf_num("HousingPaymentAmount", amount(p));
        }
        if let Some(r_i) = r.rental_income {
            x.leaf_num("GrossRentalIncomeAmount", amount(r_i));
        }
        x.leaf("DispositionStatusType", disposition(&r.disposition));
        x.close("REO_PROPERTY");
    }
    x.close("REO_PROPERTIES");
}

fn serialize_declarations(x: &mut Xml, b: &Borrower, input: &MismoInput) {
    let mine: Vec<_> = input
        .declarations
        .iter()
        .filter(|d| d.borrower_id == b.id && d.answer.is_some())
        .collect();
    if mine.is_empty() {
        return;
    }
    x.open("DECLARATIONS");
    for d in mine {
        x.open("DECLARATION");
        // ULAD: the URLA §5 question ↔ DeclarationType enum mapping is
        // pending ULAD; the Orion question key rides through until then.
        x.comment("ULAD: map question key to DeclarationType enum");
        x.leaf("DeclarationType", &d.question_key);
        x.leaf(
            "AnswerIndicator",
            if d.answer == Some(true) { "true" } else { "false" },
        );
        if let Some(e) = &d.explanation {
            x.leaf("ExplanationDescription", e);
        }
        x.close("DECLARATION");
    }
    x.close("DECLARATIONS");
}

fn serialize_demographics(x: &mut Xml, idx: usize, input: &MismoInput) {
    let Some(d) = input.demographics.iter().find(|d| {
        d.borrower_id == input.borrowers[idx].id
    }) else {
        return;
    };
    x.open("GOVERNMENT_MONITORING");
    // One container per selection — MISMO/HMDA multi-select (P3-15).
    // "not_provided" serializes as InformationNotProvided, the
    // decline-to-answer form.
    for e in &d.ethnicity_choices {
        x.open("ETHNICITY");
        x.leaf("EthnicityType", ethnicity_type(e));
        x.close("ETHNICITY");
    }
    for r in &d.race_choices {
        x.open("RACE");
        x.leaf("RaceType", race_type(r));
        x.close("RACE");
    }
    if let Some(s) = &d.sex {
        x.open("SEX");
        x.leaf("SexOfApplicantOrBorrowerType", sex(s));
        x.close("SEX");
    }
    if let Some(c) = &d.collection_basis {
        // Face-to-face vs not — the collection-basis indicator.
        x.leaf(
            "CollectedFaceToFaceIndicator",
            if c == "face_to_face" { "true" } else { "false" },
        );
    }
    x.close("GOVERNMENT_MONITORING");
}

// Demographics pass through mostly raw (they're compliance capture;
// Orion must not reinterpret them) — the XML writer escapes content.

// ── COLLATERALS — the subject property (field reference §8) ──────────

fn serialize_collateral(x: &mut Xml, input: &MismoInput) {
    let Some(p) = input.property else {
        return;
    };
    x.open("COLLATERALS");
    x.open_attr("COLLATERAL", "ID", "Collateral-1");
    x.open("SUBJECT_PROPERTY");
    x.open("ADDRESS");
    if let Some(a) = &p.address_line {
        x.leaf("AddressLineText", a);
    }
    if let Some(c) = &p.city {
        x.leaf("CityName", c);
    }
    if let Some(s) = &p.state {
        x.leaf("StateCode", s);
    }
    if let Some(z) = &p.postal_code {
        x.leaf("PostalCode", z);
    }
    x.close("ADDRESS");
    x.open("PROPERTY_DETAIL");
    x.leaf("PropertyType", property_type(&p.property_type));
    x.leaf("PropertyUsageType", occupancy(&p.occupancy));
    if let Some(y) = p.year_built {
        x.leaf_num("PropertyStructureBuiltYear", y);
    }
    x.leaf_num("PropertyUnitCount", p.units);
    x.close("PROPERTY_DETAIL");
    // Estimated value rides in a valuation container in 3.4; Orion
    // has a single entered estimate, serialized as one valuation.
    if let Some(v) = p.estimated_value {
        x.open("PROPERTY_VALUATIONS");
        x.open("PROPERTY_VALUATION");
        x.leaf_num("ValueAmount", amount(v));
        x.close("PROPERTY_VALUATION");
        x.close("PROPERTY_VALUATIONS");
    }
    x.close("SUBJECT_PROPERTY");
    x.close("COLLATERAL");
    x.close("COLLATERALS");
}

// ── Completeness — what DU will bounce, listed before it does ────────

/// The gaps between this loan and a submittable 1003. The export still
/// serializes whatever exists — this is advice, not a gate — but the
/// UI surfaces it next to the XML so the LO can act on it.
pub fn completeness(input: &MismoInput) -> Vec<String> {
    let mut missing = Vec::new();

    let Some(t) = input.terms else {
        missing.push("Loan terms — nothing entered".into());
        return missing;
    };
    if t.loan_amount.is_none() {
        missing.push("Loan amount".into());
    }
    if t.interest_rate.is_none() {
        missing.push("Note rate".into());
    }
    if t.term_months.is_none() {
        missing.push("Amortization term".into());
    }

    match input.borrowers.is_empty() {
        true => {
            missing.push("Borrower — no borrowers on the loan".into());
            return missing;
        }
        false => {
            let primary = input
                .borrowers
                .iter()
                .find(|b| b.is_primary)
                .unwrap_or(&input.borrowers[0]);
            if primary.ssn.is_none() {
                missing.push("Primary borrower SSN".into());
            }
            if primary.birth_date.is_none() {
                missing.push("Primary borrower date of birth".into());
            }
            if primary.email.is_none() && primary.phone.is_none() {
                missing.push("Borrower phone or email".into());
            }
        }
    }

    if input.employments.is_empty() {
        missing.push("Employment history".into());
    }
    if input.income.is_empty() {
        missing.push("Income sources".into());
    }
    if input.assets.is_empty() {
        missing.push("Assets".into());
    }
    match &input.property {
        None => missing.push("Subject property".into()),
        Some(p) if p.address_line.is_none() => {
            missing.push("Subject property address".into())
        }
        Some(_) => {}
    }
    if !input.declarations.iter().any(|d| d.answer.is_some()) {
        missing.push("Declarations — none answered".into());
    }
    if input
        .demographics
        .iter()
        .all(|d| {
            d.ethnicity_choices.is_empty()
                && d.race_choices.is_empty()
                && d.sex.is_none()
        })
    {
        missing.push("Government monitoring (HMDA demographics)".into());
    }

    missing
}

