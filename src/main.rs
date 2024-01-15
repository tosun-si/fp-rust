use crate::validator::Validator;

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub civility: String,
}

mod validator;

fn main() {
    let person = Person {
        first_name: "".into(),
        last_name: "".into(),
        civility: "Madrid".into(),
    };

    let result: Vec<&str> = Validator::of(&person)
        .validate(|p| &p.first_name, |first_name| !first_name.is_empty(), "First name should not empty")
        .validate(|p| &p.last_name, |last_name| !last_name.is_empty(), "Last name should not empty")
        .validate(|p| &p.civility, |civility| civility.eq("Paris"), "Civility not in Paris")
        .get_error_messages();

    println!("{result:#?}");
}
