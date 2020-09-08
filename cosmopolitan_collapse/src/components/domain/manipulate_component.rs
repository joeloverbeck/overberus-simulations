use components::domain::components::Components;

pub fn manipulate_component<T: FnMut(&&mut Components) -> bool, U: Fn(&mut Components)>(
    components: &mut Vec<Components>,
    matching_condition: T,
    operation_to_perform_on_matching_component: U,
) -> Result<(), String> {
    let mut matching_components = components
        .iter_mut()
        .filter(matching_condition)
        .collect::<Vec<&mut Components>>();

    operation_to_perform_on_matching_component(matching_components[0]);

    Ok(())
}
