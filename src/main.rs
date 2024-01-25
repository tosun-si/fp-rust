use crate::expenses::Expenses;
use crate::factory::Factory;
use crate::fluent_decorator::FluentDecorator;
use crate::validator::Validator;

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub civility: String,
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
}

pub struct Psg {
    team: Team,
}

pub struct Real {
    team: Team,
}

mod validator;
mod fluent_decorator;
mod expenses;
mod factory;

fn main() {
    let person = Person {
        first_name: "Coucou Mazlum".into(),
        last_name: "".into(),
        civility: "Madrid".into(),
    };

    let result: Vec<&str> = Validator::of(&person)
        .validate(|p| &p.first_name, |first_name| !first_name.is_empty(), "First name should not empty")
        .validate(|p| &p.last_name, |last_name| !last_name.is_empty(), "Last name should not empty")
        .validate(|p| &p.civility, |civility| civility.eq("Paris"), "Civility not in Paris")
        .get_error_messages();

    let turnover: f32 = 100000.0;

    let profit: f32 = FluentDecorator::from(turnover, std::convert::identity)
        .with(Expenses::get_remuneration)
        .with(Expenses::get_deductible_taxes)
        .with(Expenses::get_operating_expenses)
        .with(Expenses::get_transport_expenses)
        .with(Expenses::get_exceptional_expenses)
        .calculate();

    let result_str = FluentDecorator::from(&person.first_name, std::convert::identity)
        .with(|s| Expenses::transform_str1(&s))
        .with(|s| Expenses::transform_str2(&s))
        .calculate();

    let case1 = "Coucou Mazlum".to_string();
    let case2 = "Coucou Mazizou".to_string();

    let team = Factory::from_type(&person.first_name)
        .register(&case1, || get_psg())
        .register(&case2, || get_real())
        .get();

    println!("{result:#?}");
    println!("{profit:#?}");
    println!("{result_str:#?}");
    println!("{team:#?}");
}

pub fn get_psg() -> Team {
    Psg { team: Team { name: "PSG".into() } }.team
}

pub fn get_real() -> Team {
    Real { team: Team { name: "Real".into() } }.team
}
