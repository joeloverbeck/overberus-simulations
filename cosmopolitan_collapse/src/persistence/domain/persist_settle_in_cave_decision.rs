use components::controllers::components_controller::ComponentsController;
use components::domain::components::Components;

pub fn persist_settle_in_cave_decision(
    agent_id: u32,
    space_id: u32,
    components_controller: &mut ComponentsController,
) -> Result<(), String> {
    // Things that should happen when an agent has decided to settle in a cave:
    // Checks that there's actually a cave in the space.
    // Checks whether there is room there.
    // The agent shouldn't already be there.
    // Gets added to the inhabitants.
    if !components_controller
        .does_any_component_of_an_entity_check_a_condition(space_id, |component| {
            component.is_cave()
        })
    {
        panic!("Was going to persist the decision to settle in a cave, but there was no cave in space with id {:?}", space_id);
    }

    let matching_cave = components_controller
        .get_mut_component_of_entity(space_id, |component| component.is_cave())?;

    if let Components::Cave {
        ref mut inhabitants,
        ref room_limit,
    } = matching_cave
    {
        if inhabitants.len() >= *room_limit {
            panic!("During the decision to persist settling in a cave, turns out that the room limit {:?} wouldn't allowed due to {:?} inhabitants already present.", room_limit, inhabitants.len());
        }

        if inhabitants.iter().any(|inhabitant| inhabitant == &agent_id) {
            panic!("During the decision to persist settling in a cave, turns out that the agent was already living in that cave: {:?}", inhabitants);
        }

        inhabitants.push(agent_id);
    }

    todo!();

    Ok(())
}
