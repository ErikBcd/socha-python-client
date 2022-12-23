use pyo3::prelude::*;
use crate::plugin::penguin::Penguin;

pub struct ONE {}

impl ONE {
    const NAME: &'static str = "ONE";
}

pub struct TWO {}

impl TWO {
    const NAME: &'static str = "TWO";
}

#[pyclass]
#[derive(PartialEq, Copy, Eq, PartialOrd, Clone, Debug, Hash)]
pub enum TeamEnum {
    ONE,
    TWO,
}

impl std::fmt::Display for TeamEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                TeamEnum::ONE => write!(f, "{}", String::from(ONE::NAME))
                    .map_err(|_e| core::fmt::Error),
                TeamEnum::TWO => write!(f, "{}", String::from(TWO::NAME))
                    .map_err(|_e| core::fmt::Error),
            }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Team {
    #[pyo3(get, set)]
    pub name: TeamEnum,
    #[pyo3(get, set)]
    pub penguins: Vec<Penguin>,
    #[pyo3(get, set)]
    pub fish: i32,
}

#[pymethods]
impl Team {
    #[new]
    pub fn new(name: TeamEnum, penguins: Vec<Penguin>, fish: i32) -> Self {
            Team {
                name,
                penguins,
                fish,
            }
    }

    pub fn opponent(&self) -> Team {
        match self.name {
            TeamEnum::ONE => Team {
                name: TeamEnum::TWO,
                penguins: Vec::new(),
                fish: 0,
            },
            TeamEnum::TWO => Team {
                name: TeamEnum::ONE,
                penguins: Vec::new(),
                fish: 0,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("Team(name={}, penguins={:?}, fish={})", self.name, self.penguins, self.fish)
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Team(name={}, penguins={:?}, fish={})", self.name, self.penguins, self.fish)
        .map_err(|_e| core::fmt::Error)
    }
}