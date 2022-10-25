use crate::ModelObjects::component::{
    Component, Declarations, Edge, Location, LocationType, SyncType,
};
use crate::ModelObjects::state::State;

pub struct SerializedDecisionPoint {}

impl SerializedDecisionPoint {}

#[cfg(test)]
mod tests {
    struct setup {
        testSource: State,
        testEdges: Vec<Edge>,
    }

    impl Setup {
        fn new() -> Self {
            testSource = State::create();
            testEdges = vec![]
        }
    }

    #[test]
    fn given_state_return_serialized_state()
    {
        assert_eq!(false);
    }
}
