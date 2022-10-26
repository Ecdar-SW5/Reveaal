use crate::{
    component::{State, Transition},
    TransitionSystems::TransitionSystemPtr,
};

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct TransitionDecision {
    pub source: State,
    pub transitions: Vec<Transition>,
}

impl TransitionDecision {
    /// Constructs the initial TransitionDecision for a given TransitionSystemPtr
    pub fn initial(system: TransitionSystemPtr) -> Option<Self> {
        match system.get_initial_state() {
            Some(source) => Some(Self::from(system, source)),
            None => None,
        }
    }

    /// Constructs the TransitionDecision from a source State and a given TransitionSystemPtr
    pub fn from(system: TransitionSystemPtr, source: State) -> TransitionDecision {
        let mut transitions = vec![];
        let actions = system.get_actions();

        // get all transitions
        for action in actions {
            let transition = system.next_transitions_if_available(source.get_location(), &action);
            transitions.append(&mut transition.clone());
        }

        // prune transitions that can not be taken
        for (index, transition) in transitions.clone().iter().enumerate() {
            if !transition.use_transition(&mut source.clone()) {
                transitions.remove(index);
            }
        }

        TransitionDecision {
            source: source,
            transitions: transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransitionDecision;
    use crate::{
        DataReader::json_reader::read_json_component,
        TransitionSystems::{CompiledComponent, TransitionSystemPtr},
    };

    fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
        let mut component = read_json_component("samples/json/EcdarUniversity", "Machine");
        component.create_edge_io_split();//used to get input and output edges, if it is not called, the input and output edges will be empty
        CompiledComponent::from(vec![component], "Machine")
    }

    fn create_EcdarUniversity_Machine4_system() -> TransitionSystemPtr {
        let mut component = read_json_component("samples/json/EcdarUniversity", "Machine4");
        component.create_edge_io_split();
        CompiledComponent::from(vec![component], "Machine4")
    }

    #[test]
    fn initial__EcdarUniversity_Machine__return_correct_state() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual = format!(
            "{:?}",
            TransitionDecision::initial(system.clone()).unwrap().source
        );

        // Assert
        let expected = format!("{:?}", system.get_initial_state().unwrap());
        assert_eq!(actual, expected)
    }

    #[test]
    fn initial__EcdarUniversity_Machine__correct_transitions() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual: Vec<String> = TransitionDecision::initial(system.clone())
            .unwrap()
            .transitions
            .into_iter()
            .map(|x| format!("{:?}", x)) // shhhhhh, close your eyes, this is not logic
            .collect();

        // Assert
        let expected_len = 2;
        assert_eq!(actual.len(), expected_len);

        let expected_tea_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "tea")[0]
        );
        assert!(actual.contains(expected_tea_transition));

        let expected_coin_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "coin")[0]
        );
        assert!(actual.contains(expected_coin_transition));
    }

    #[test]
    fn initial__EcdarUniversity_Machine4__correct_transitions() {
        // Arrange
        let system = create_EcdarUniversity_Machine4_system();

        // Act
        let actual: Vec<String> = TransitionDecision::initial(system.clone())
            .unwrap()
            .transitions
            .into_iter()
            .map(|x| format!("{:?}", x)) // still no logic to be found here 
            .collect();

        // Assert
        let expected_len = 1;
        assert_eq!(actual.len(), expected_len);

        let expected_coin_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "coin")[0]
        );
        assert!(actual.contains(expected_coin_transition));
    }
}
