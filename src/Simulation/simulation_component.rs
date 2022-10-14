use std::collections::HashSet;

use crate::{ModelObjects::{component::{Channel, Component, Location}, self}, TransitionSystems::{TransitionSystemPtr, LocationTuple, TransitionSystem}};

#[derive(Clone)]
#[allow(dead_code)]
pub struct SimulationComponent {
    transition_system: TransitionSystemPtr,
    valid_actions: HashSet<String>,
    state: Vec<ModelObjects::component::State>
    // valid_transitions: Vec<&'a Edge>,
}

impl SimulationComponent {
    pub fn new(transitionSystem: TransitionSystemPtr) -> Self {
        Self {
            valid_actions: transitionSystem.get_actions(),
            transition_system: transitionSystem.clone(),
            state: vec![get_inital_state(transitionSystem)],
            }
        }

    pub fn use_transition(&self, action: String, State: ModelObjects::component::State) {
        
    }
}
    
    fn get_initial_location(transitionSystem: TransitionSystemPtr) -> LocationTuple {
        let initialLocation = match transitionSystem.get_initial_location() {
            None => panic!("no initial location found"),
            Some(x) => x.clone(),
        };
        initialLocation
    }

    fn get_inital_state(transitionSystem: TransitionSystemPtr) -> ModelObjects::component::State {
        let initialState = match transitionSystem.get_initial_state() {
            None => panic!("Initial state is empty"),
            Some(x) => x.clone(),
        };
        initialState
    }


    



pub fn continue_simulation(
    simulation_component: SimulationComponent,
    _action: Channel,
) -> SimulationComponent {
    // let start_location: Location = input_component.location;
    // let action_taken: Channel = action;

    // Check the allowed actions from start location to edges.
    // Can this be done with the intersection Casper explained?
    // let new_edges: Vec<SimulationComponent::component::Edge> = vec![];
    // let input_edges = simulation_component.get_next_edges(start_location, action_taken.to_string(), SimulationComponent:component::SyncType::Input);

    simulation_component
}

#[cfg(test)]
mod tests {
    use crate::DataReader::json_reader;
    use crate::ModelObjects::component::{Channel, Component, Location, State, Transition};
    use crate::Simulation::simulation_component;
    use crate::TransitionSystems::LocationTuple;

    #[test]
    fn Convert_GivenComponent_ReturnsSimulationComponent() {
        // Arrange
        let should_equal: Component = json_reader::read_json_component("samples/json/AG", "A");

        // Act
        let input_component: Component = json_reader::read_json_component("samples/json/AG", "A");
        let output = simulation_component::SimulationComponent::new(input_component);
        //let output: simulation_component::SimulationComponent = simulation_component::start_simulation("samples/json/AG", "A");

        // Assert
        assert_eq!(should_equal, output.component);
    }

    #[test]
    fn JsonObject_NotEqualTo_Simulation_Component() {
        // Arrange
        let should_equal: Component = json_reader::read_json_component("samples/json/AG", "A");

        // Act
        let input_component: Component = json_reader::read_json_component("samples/json/AG", "AA");
        let output = simulation_component::SimulationComponent::new(input_component);

        // Assert
        assert_ne!(should_equal, output.component);
    }

    #[test]
    fn Moved_To_New_Location() {
        // Arrange
        let test_component: Component = json_reader::read_json_component("samples/json/AG", "Imp");
        let should_equal: Location = test_component.get_location_by_name("L1").clone();
        let t_struct: Channel = Channel {
            name: String::from("t_struct"),
        };

        // Act
        let input_component: Component = json_reader::read_json_component("samples/json/AG", "Imp");
        let test_simulation_component =
            simulation_component::SimulationComponent::new(input_component);
        let output: simulation_component::SimulationComponent =
            simulation_component::continue_simulation(test_simulation_component, t_struct);

        // Assert
        assert_eq!(should_equal, output.location);
    }

    #[test]
    fn Take_TransitionFromComponent() {
        // Arrange
        let test_component: Component = json_reader::read_json_component("samples/json/AG", "Imp");
        let should_equal: Location = test_component.get_location_by_name("L1").clone();
        let equal_tuple: LocationTuple =
            LocationTuple::simple(&should_equal, &test_component.declarations, 1);

        // Act
        let location: &Location = match test_component.get_initial_location() {
            None => panic!("no initial location found"),
            Some(x) => x,
        };

        let test_loctuple: LocationTuple =
            LocationTuple::simple(location, &test_component.declarations, 1);

        let test_transition: Transition = Transition::new(&test_loctuple, 5);
        let test_transition_cloned: Transition = test_transition.clone();

        let test_state: &mut State =
            &mut State::create(test_loctuple, test_transition_cloned.guard_zone);

        test_transition.use_transition(test_state);

        println!(
            "!!!!!!!!!!!!!!!!!!!! === {:?} === !!!!!!!!!!!!!!!!!!!!!!!!!!!!",
            test_transition.target_locations
        );
        // Assert

        assert_eq!(equal_tuple, test_transition.target_locations)
    }
}

//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣴⡶⠶⠶⠶⠶⠶⠶⠶⠶⠤⠤⢤⣤⣤⣤⣤⣤⣄⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠟⠋⠀⠀⠀⠀⢀⣀⠤⠖⠚⢉⣉⣉⣉⣉⣀⠀⠀⠀⠀⠀⠀⠈⠉⠩⠛⠛⠛⠻⠷⣦⣄⡀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⣠⡿⠋⠀⠀⠀⣀⠤⠒⣉⠤⢒⣊⡉⠠⠤⠤⢤⡄⠈⠉⠉⠀⠂⠀⠀⠐⠂⠀⠉⠉⠉⠉⠂⠀⠙⠻⣶⣄⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⣰⡿⠁⠀⠀⡠⠊⢀⠔⣫⠔⠊⠁⠀⠀⠀⠀⠀⠀⠙⡄⠀⠀⠀⠀⠀⠘⣩⠋⠀⠀⠀⠉⠳⣄⠀⠀⠀⠈⢻⡇⠀⠀⠀
//⠀⠀⠀⠀⠀⣰⡿⠁⠀⠀⠀⠀⠀⠁⠜⠁⣀⣤⣴⣶⣶⣶⣤⣤⣀⠀⠃⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠈⠆⠀⠀⠀⠸⣧⡀⠀⠀
//⠀⠀⠀⣠⣾⣿⣥⠤⢄⡀⠀⢠⣤⠔⢠⣾⣿⣿⣿⣿⣿⣯⣄⡈⠙⢿⣦⠀⠀⠀⠀⡀⢀⣤⣶⣿⣿⣿⣿⣿⣦⠀⣀⣀⣀⣀⡙⢿⣦⡀
//⠀⣠⡾⣻⠋⢀⣠⣴⠶⠾⢶⣤⣄⡚⠉⠉⠉⠁⣠⣼⠏⠉⠙⠛⠷⡾⠛⠀⠀⠀⠘⠛⢿⡟⠛⠋⠉⠉⠉⠁⠀⠀⠀⠀⠀⠦⣝⠦⡙⣿
//⢰⡟⠁⡇⢠⣾⠋⠀⠀⣼⣄⠉⠙⠛⠷⠶⠶⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣇⠀⠀⠀⠠⣦⣄⣴⡾⢛⡛⠻⠷⠘⡄⢸⣿
//⢸⡇⠀⡇⢸⣇⢀⣤⣴⣿⠻⠷⣦⣄⣀⠀⠀⠀⢀⡀⠀⣀⠰⣤⡶⠶⠆⠀⠀⠀⠀⠀⠈⠛⢿⣦⣄⠀⠈⠉⠉⠁⢸⣇⠀⠀⣠⠃⢸⣿
//⠸⣿⡀⢇⠘⣿⡌⠁⠈⣿⣆⠀⠀⠉⢻⣿⣶⣦⣤⣀⡀⠀⠀⢻⣦⠰⡶⠿⠶⠄⠀⠀⠀⣠⣾⠿⠟⠓⠦⡄⠀⢀⣾⣿⡇⢈⠡⠔⣿⡟
//⠙⢿⣌⡑⠲⠄⠀⠀⠙⢿⣿⣶⣦⣼⣿⣄⠀⠈⠉⠛⠻⣿⣶⣯⣤⣀⣀⡀⠀⠘⠿⠾⠟⠁⠀⠀⢀⣀⣤⣾⣿⢿⣿⣇⠀⠀⣼⡟⠀
//⠀⠀⠀⠹⣿⣇⠀⠀⠀⠀⠈⢻⣦⠈⠙⣿⣿⣷⣶⣤⣄⣠⣿⠁⠀⠈⠉⠙⢻⡟⠛⠻⠿⣿⠿⠛⠛⢻⣿⠁⢈⣿⣨⣿⣿⠀⢰⡿⠀⠀
//⠀⠀⠀⠀⠈⢻⣇⠀⠀⠀⠀⠀⠙⢷⣶⡿⠀⠈⠙⠛⠿⣿⣿⣶⣶⣦⣤⣤⣼⣧⣤⣤⣤⣿⣦⣤⣤⣶⣿⣷⣾⣿⣿⣿⡟⠀⢸⡇⠀⠀
//⠀⠀⠀⠀⠀⠈⢿⣦⠀⠀⠀⠀⠀⠀⠙⢷⣦⡀⠀⠀⢀⣿⠁⠉⠙⠛⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⢸⣷⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠙⢷⣄⠀⢀⡀⠀⣀⡀⠈⠻⢷⣦⣾⡃⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⢹⡟⠉⠉⣿⠏⢡⣿⠃⣾⣷⡿⠁⠀⠘⣿⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢷⣤⣉⠒⠤⣉⠓⠦⣀⡈⠉⠛⠿⠶⢶⣤⣤⣾⣧⣀⣀⣀⣿⣄⣠⣼⣿⣤⣿⠷⠾⠟⠋⠀⠀⠀⠀⣿⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⣶⣄⡉⠒⠤⢌⣑⠲⠤⣀⡀⠀⠀⠀⠈⠍⠉⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⣠⠏⠀⢰⠀⠀⣿⡄⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠛⠿⢷⣦⣄⡉⠑⠒⠪⠭⢄⣀⣀⠀⠐⠒⠒⠒⠒⠀⠀⠐⠒⠊⠉⠀⢀⡠⠚⠀⠀⢸⡇⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢷⣦⣀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠓⠒⠒⠒⠊⠁⠀⠀⠀⢠⣿⠃⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠛⠛⠷⠶⣶⣦⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣴⠟⠁⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠛⠛⠷⠶⠶⠶⠶⠶⠾⠛⠛⠉⠀⠀⠀⠀⠀

// {                                , /
//                              *//******,******/*//*/*,
//                         ./********,/*,,************/////(*
//                     .***//***,***************,,,*//////*////*
//                  ******//*/**///**/*****///***,***//(((////**/,
//                *******/***,****/*///////****/((/(###%%%##(//**/.
//               *,***,,,,,****/*////((########%%%%%%%%%%%%%##(/**//#.
//              ,****,,,,,,**/(((((#######%%%%%%%%%%%%%%%%######///(((/
//              */*,,,,,****/(((###%%%%%%%%%%%%%%%%%%%%%%%%%####(((((//(
//             */*,,,,,****///((####%%%%%%%%%%%%%%%%%%%%%%%%%%###(/**/(/.
//             ,*,,,,,,,**///(((####%%%%%%%%%%%%%%%%%%%%%%%%%%%###//(**//
//            *.,,,,,,,,*/////(((####%%%%%%%%%%%%%%%%%%%%%%%%%%###(*//,*,*
//            *,,,*..,,,**//((((#####%%%%%%%%%%%%%%%%%%%%%%%%%%###//(//**//
//            *..**,.,,,,*/((((########%%%#%%%%%%%%########%%%%###//(**##((
//           **..,,,....,,*/////(/****//(##(%%%#(,//((######(((,*.,**,(#%##/
//            *,,..........,,/((#(**//***../(#//######/**//*/(#/###///((##%
//             ....  ....,*,,**///(((#(,..*##%%#,/##(((#%%###%*%%###//#%%%#
//             ,.........,*/****/((###/*,,,/#%%%%#*#%%%%#(*#%%%%%%##/(##%%
//             ,.........,*////****//##(*,,/(#%%%##%%%%%%%%%%%%%%%#((%##%/
//               ...  . ..,//((((######(,,*/(#%&%%%%###%%%%%%%%%%%###%%%%
//                ,.. ....,*///((##%%%/.,*,,*/(#(###%%%#(%%%%%%%%%##%%&%
//                 ,.......,,*/((##%#(((//*/(((#%%%%%%%%%##%%%%%%%###%*
//                   ,,,...,,,*((###((//(((##%##%%%%%%%%%%#(%%%%%###
//                     . ,..,,*//#%///////*////((((////(##%#%%#%###
//                        ....,**(%##(//**/**/#//(#####%%#%#%#####
//                         ....,**(#((/*,*//(######%###%#%######(
//                         ......,*///(/////((((#####%##%####(((.
//                         .......,,**//(((((###%%%%%%%####((/((
//                          ..........,*//((((####%######(///(((
//                          ,.,.,........*///(((((((((//((//((((
//                          ,*,****,...,..,,,**/////////((((###
//                    ...,*,,,*///////******///((//((##########
//                 ...,///*,,*//(((((((((((((((#%%%##%%%######(***
//              ,......(((/***/(((((((#(((((((###%%%%%#########*,,%&@@@@@@(**
//         ,........... .((**/(((((((#####((((((((##############.,,,,,,,,,,,,,*%##%
//     ................  ..**//((#((((######((##(((#############*.,,.,,..,,,..,,,,.
// ............. ........   ..*((########%%%#########%####(#####..,,.,....,....,,,,
// ............. ......... .   ...(#######%%%##################,.,...,.....,.....,,}
