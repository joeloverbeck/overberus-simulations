extern crate file_system;
extern crate neural_networks;

use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
use self::file_system::read_file_to_string::read_file_to_string;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neuron::Neuron;
use constants::SETTLING_IN_A_COMPONENT_SURVIVALIST_FILENAME;
use constants::SETTLING_IN_A_COMPONENT_URBAN_FILENAME;
use constants::SETTLING_IN_A_COMPONENT_WILD_FILENAME;

use agents::agent_traits::AgentTraits;
use components::domain::components::Components;

fn load_json(filename: &str) -> Result<NeuralNetwork<Neuron>, String> {
    Ok(deserialize_json_from_string::<NeuralNetwork<Neuron>>(
        &read_file_to_string(filename)?,
    )?)
}

pub fn create_brain_component(traits: &[AgentTraits]) -> Result<Components, String> {
    let mut settling_in: Option<NeuralNetwork<Neuron>> = None;

    // Should go through each trait in the passed vector and figure out what stored
    // brain you should pick.
    for agent_trait in traits {
        match agent_trait {
            AgentTraits::Urban => {
                settling_in = Some(load_json(SETTLING_IN_A_COMPONENT_URBAN_FILENAME)?)
            }
            AgentTraits::Wild => {
                settling_in = Some(load_json(SETTLING_IN_A_COMPONENT_WILD_FILENAME)?)
            }
            AgentTraits::Survivalist => {
                settling_in = Some(load_json(SETTLING_IN_A_COMPONENT_SURVIVALIST_FILENAME)?)
            }
        }
    }

    Ok(Components::Brain {
        settling_in: settling_in.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_a_brain_for_given_traits() -> Result<(), String> {
        let brain_component = create_brain_component(&vec![AgentTraits::Urban])?;

        assert!(brain_component.is_brain());

        Ok(())
    }
}
