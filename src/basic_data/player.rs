pub struct Player {
    person: Person,
    position: Position,
    team: Team,
    //skills
}

pub enum Position {
    GoalKeeper,
    RightBack,
    LeftBack,
    CentreBack,
    DefensiveMidfield,
    Midfield,
    AttackingMidfield,
    RightAttackingMidfield,
    LeftAttackingMidfield,
    Forward,
    Striker,
    LeftWinger,
    RightWinger,
}
