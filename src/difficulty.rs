#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum Difficulty {
    Easy = 0, Medium = 1, Hard = 2, Extreme = 3, Reality = 4
}

impl Difficulty {
    pub fn to_string(self) -> String {
        match self {
            Difficulty::Easy    => "2014 (Easy)".to_string(),
            Difficulty::Medium  => "1989 (Medium)".to_string(),
            Difficulty::Hard    => "1917 (Hard)".to_string(),
            Difficulty::Extreme => "1922 (Extreme)".to_string(),
            Difficulty::Reality => "1944 (Reality)".to_string(),
        }
    }

    pub fn harder(self) -> Difficulty {
        match self {
            Difficulty::Easy    => Difficulty::Medium,
            Difficulty::Medium  => Difficulty::Hard,
            Difficulty::Hard    => Difficulty::Extreme,
            Difficulty::Extreme => Difficulty::Reality,
            Difficulty::Reality => Difficulty::Reality,
        }
    }

    pub fn easier(self) -> Difficulty {
        match self {
            Difficulty::Reality => Difficulty::Extreme,
            Difficulty::Extreme => Difficulty::Hard,
            Difficulty::Hard    => Difficulty::Medium,
            Difficulty::Medium  => Difficulty::Easy,
            Difficulty::Easy    => Difficulty::Easy,
        }
    }

    #[inline(always)]
    pub fn get_josef_police_rate(self) -> u16 {
        match self {
            Difficulty::Easy => 1000,
            Difficulty::Medium => 700,
            Difficulty::Hard => 400,
            Difficulty::Extreme => 200,
            Difficulty::Reality => 50
        }
    }

    #[inline(always)]
    pub fn get_josef_speed(self) -> u16 {
        match self {
            Difficulty::Easy => 60,
            Difficulty::Medium => 40,
            Difficulty::Hard => 30,
            Difficulty::Extreme => 20,
            Difficulty::Reality => 15
        }
    }

    #[inline(always)]
    pub fn get_police_speed(self) -> u16 {
        match self {
            Difficulty::Easy => 20,
            Difficulty::Medium => 15,
            Difficulty::Hard => 12,
            Difficulty::Extreme => 10,
            Difficulty::Reality => 7
        }
    }

    #[inline(always)]
    pub fn get_police_hurt_rate(self) -> u16 {
        match self {
            Difficulty::Easy => 7,
            Difficulty::Medium => 5,
            Difficulty::Hard => 4,
            Difficulty::Extreme => 3,
            Difficulty::Reality => 1
        }
    }

    #[inline(always)]
    pub fn get_josef_health(self) -> u16 {
        match self {
            Difficulty::Easy => 3,
            Difficulty::Medium => 5,
            Difficulty::Hard => 7,
            Difficulty::Extreme => 13,
            Difficulty::Reality => 17
        }
    }

    #[inline(always)]
    pub fn get_communism_drop_rate(self) -> f64 {
        match self {
            Difficulty::Easy => 5.,
            Difficulty::Medium => 4.,
            Difficulty::Hard => 3.,
            Difficulty::Extreme => 2.,
            Difficulty::Reality => 1.,
        }
    }

    #[inline(always)]
    pub fn get_start_health(self) -> u16 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 4,
            Difficulty::Hard => 3,
            Difficulty::Extreme => 2,
            Difficulty::Reality => 1,
        }
    }
}
