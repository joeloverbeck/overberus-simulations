use agents::agent::Agent;
use agents::brain_trait::BrainTrait;
use game_definitions::aspects::Aspects;
use world::coordinate::Coordinate;
use world::space::Space;
use world::space_trait::SpaceTrait;

pub fn is_agent_associated_with_space_aspect<T: BrainTrait>(
    agent_id: u32,
    coordinate: Coordinate,
    aspect: Aspects,
    _agents: &[Agent<T>],
    spaces: &[Space],
) -> bool {
    // Should search first whether the associated space exists.
    if !spaces
        .iter()
        .any(|space| space.get_coordinate() == &coordinate)
    {
        panic!("There wasn't a space for the coordinate {:?}", coordinate);
    }

    // Now whether that space has the aspect passed.
    let corresponding_space = spaces
        .iter()
        .filter(|space| space.get_coordinate() == &coordinate)
        .collect::<Vec<&Space>>()[0];

    if !corresponding_space.has_aspect(&aspect) {
        return false;
    }

    // At this point the space has the aspect passed.
    corresponding_space.is_agent_associated_with_aspect(agent_id, &aspect)
}
