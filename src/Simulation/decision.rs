use crate::component::{Edge, State};

/// Represent a decision in a any composition of components, that has been taken: In the current `source` state I have `decided` to use this [`Edge`].
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
