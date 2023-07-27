use super::{person::Person, team::Team};

pub struct Staff {
    person: Person,
    function: Function,
    teams: Vec<Team>,
    //team
    //skills
}

pub enum Function {
    MANAGER,
    COACH,
    SCOUT,
    CHAIRMAN,
    ASSISTANT,
}
