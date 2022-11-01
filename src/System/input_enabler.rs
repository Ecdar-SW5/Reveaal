use edbm::zones::OwnedFederation;

use crate::EdgeEval::constraint_applyer;
use crate::ModelObjects::component;
use crate::ModelObjects::component::DeclarationProvider;
use crate::ModelObjects::representations::BoolExpression;

pub fn make_input_enabled(component: &mut component::Component, inputs: &[String]) {
    let dimension = component.declarations.get_clock_count() + 1;
    let mut new_edges: Vec<component::Edge> = vec![];

    for location in component.get_locations() {
        let mut location_inv_zone = OwnedFederation::universe(dimension);

        if let Some(invariant) = location.get_invariant() {
            location_inv_zone = constraint_applyer::apply_constraints_to_state(
                invariant,
                component.get_declarations(),
                location_inv_zone,
            )
            .unwrap();
        }

        // No constraints on any clocks
        let full_federation = location_inv_zone.clone();

        for input in inputs {
            let input_edges = component.get_next_edges(location, input, component::SyncType::Input);
            let mut zones_federation = OwnedFederation::empty(dimension);

            for edge in input_edges {
                let mut guard_zone = OwnedFederation::universe(dimension);
                if let Some(target_invariant) = component
                    .get_location_by_name(edge.get_target_location())
                    .get_invariant()
                {
                    guard_zone = constraint_applyer::apply_constraints_to_state(
                        target_invariant,
                        component.get_declarations(),
                        guard_zone,
                    )
                    .unwrap();
                }

                if let Some(updates) = edge.get_update() {
                    for update in updates {
                        let cu = update.compiled(component.get_declarations());
                        guard_zone = cu.apply_as_guard(guard_zone);
                        guard_zone = cu.apply_as_free(guard_zone);
                    }
                }

                if let Some(guard) = edge.get_guard() {
                    guard_zone = constraint_applyer::apply_constraints_to_state(
                        guard,
                        component.get_declarations(),
                        guard_zone,
                    )
                    .unwrap();
                }

                zones_federation += guard_zone.intersection(&location_inv_zone);
            }

            let result_federation = full_federation.clone().subtraction(&zones_federation);

            if result_federation.is_empty() {
                continue;
            }

            //for fed_zone in result_federation.iter_zones() {
            new_edges.push(component::Edge {
                id: "NotImplemented".to_string(), //NEEDS TO BE FIXED
                source_location: location.get_id().to_string(),
                target_location: location.get_id().to_string(),
                sync_type: component::SyncType::Input,
                guard: BoolExpression::from_disjunction(
                    &result_federation.minimal_constraints(),
                    component.get_declarations().get_clocks(),
                ),
                update: None,
                sync: input.to_string(),
            });
            //}
        }
    }

    component.add_input_edges(&mut new_edges);
}
