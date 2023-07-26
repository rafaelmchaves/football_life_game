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
    MANAGER_ASSISTANT,
}
