pub struct Team {
    pub id: str,
    pub name: str,
    pub shortname: str,
    pub color_1: Colors,
    pub color_2: Colors,
    pub stadium: Stadium,
    pub president: Staff,
    pub nation: str,
}

pub struct Stadium {
    pub id: str,
    pub name: str,
    pub capacity: i8,
    pub seat_capacity: i8,
}

pub enum Colors {
    WHITE,
    RED,
    BLUE,
    GREEN,
    ORANGE,
    YELLOW,
    PINK,
    BLACK,
}
