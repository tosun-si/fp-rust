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

pub struct Juve {
    team: Team,
}

pub struct Bayern {
    team: Team,
}

pub struct DefaultTeam {
    team: Team,
}

mod validator;
mod fluent_decorator;
mod expenses;
mod factory;

fn main() {
    let person = Person {
        first_name: "PSG".into(),
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

    let case1 = "PSG".to_string();
    let case2 = "Bayern".to_string();
    let case3 = "Juve".to_string();
    let case4 = "Bayern".to_string();

    let team = Factory::from_type(|| get_default_team())
        .register(&case1, || get_psg())
        .register(&case2, || get_real())
        .register(&case3, || get_juve())
        .register(&case4, || get_bayern())
        .get(&person.first_name);

    let factory = Factory::from_type(|| get_default_team())
        .register(&case1, || get_psg())
        .register(&case2, || get_real())
        .register(&case3, || get_juve())
        .register(&case4, || get_bayern());

    let team_psg = factory.get(&"PSG".to_string());
    let team_juve = factory.get(&"Juve".to_string());
    let team_bayern = factory.get(&"Bayern".to_string());
    let default_team = factory.get(&"Unknown team".to_string());

    println!("{result:#?}");
    println!("{profit:#?}");
    println!("{result_str:#?}");
    println!("{team:#?}");
    println!("{team_psg:#?}");
    println!("{team_juve:#?}");
    println!("{team_bayern:#?}");
    println!("{default_team:#?}");
}

pub fn get_psg() -> Team {
    Psg { team: Team { name: "PSG".into() } }.team
}

pub fn get_real() -> Team {
    Real { team: Team { name: "Real".into() } }.team
}

pub fn get_juve() -> Team {
    Juve { team: Team { name: "Juve".into() } }.team
}

pub fn get_bayern() -> Team {
    Bayern { team: Team { name: "Bayern".into() } }.team
}

pub fn get_default_team() -> Team {
    DefaultTeam { team: Team { name: "Default".into() } }.team
}
