use crate::component::{Edge, State};

#[derive(Debug)]
pub struct Decision {
    source: State,
    decided: Edge,
}

impl Decision {
    pub fn new(source: State, decided: Edge) -> Self {
        Self { source, decided }
    }

    pub fn source(&self) -> &State {
        &self.source
    }

    pub fn decided(&self) -> &Edge {
        &self.decided
    }
}
