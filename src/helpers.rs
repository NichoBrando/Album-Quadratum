use bevy::prelude::*;

pub fn get_axis_value (
    keyboard_input: &Res<Input<KeyCode>>, 
    inverse_key: KeyCode, 
    forward_key: KeyCode
) -> f32 
{
    if keyboard_input.pressed(inverse_key)
    {
        return -1.
    }
    else if keyboard_input.pressed(forward_key)
    {
        return 1.
    }
    return 0.
}