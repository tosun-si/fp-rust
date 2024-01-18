pub struct Expenses;

impl Expenses {
    pub fn get_transport_expenses(turnover: f32) -> f32 {
        turnover - 2400.0
    }

    pub fn get_operating_expenses(turnover: f32) -> f32 {
        turnover - 15000.0
    }

    pub fn get_deductible_taxes(turnover: f32) -> f32 {
        turnover - 3000.0
    }

    pub fn get_remuneration(turnover: f32) -> f32 {
        turnover - 45000.0
    }

    pub fn get_exceptional_expenses(turnover: f32) -> f32 {
        turnover - 2000.0
    }

    pub fn transform_str1(element: &str) -> String {
        format!("{element} toto")
    }

    pub fn transform_str2(element: &str) -> String {
        format!("{element} tata")
    }
}