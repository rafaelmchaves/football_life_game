use super::staff::Staff;

pub struct Team {
    pub id: String,
    pub name: String,
    pub shortname: String,
    pub color_1: Colors,
    pub color_2: Colors,
    pub stadium: Stadium,
    pub president: Staff,
    pub nation: String,
}

pub struct Stadium {
    pub id: String,
    pub name: String,
    pub capacity: i8,
    pub seat_capacity: i8,
    pub city: String,
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
