use raylib::prelude::*;

pub unsafe fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, angle: f32, color: Color) {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    ffi::rlPushMatrix();
    
    ffi::rlTranslatef(position.x, position.y, position.z);
    ffi::rlRotatef(angle, 0.0, -1.0, 0.0);

    ffi::rlBegin(ffi::RL_LINES as i32);
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);

    // Front Face -----------------------------------------------------
    // Bottom Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right

    // Left Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right

    // Top Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    // Right Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left

    // Back Face ------------------------------------------------------
    // Bottom Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right

    // Left Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right

    // Top Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left

    // Right Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left

    // Top Face -------------------------------------------------------
    // Left Line
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Front
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Back

    // Right Line
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Front
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Back

    // Bottom Face  ---------------------------------------------------
    // Left Line
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Top Left Front
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left Back

    // Right Line
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Top Right Front
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right Back
    ffi::rlEnd();
    ffi::rlPopMatrix();
}
pub unsafe fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, angle: f32, color: Color) {
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    ffi::rlPushMatrix();

    // NOTE: Be careful! Function order matters (rotate -> scale -> translate)
    ffi::rlTranslatef(position.x, position.y, position.z);
    //rlScalef(2.0, 2.0, 2.0);
    ffi::rlRotatef(angle, 0.0, -1.0, 0.0);

    ffi::rlBegin(ffi::RL_TRIANGLES as i32);
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);

    // Front Face -----------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right

    // Back Face ------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left

    // Top Face -------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right

    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right

    // Bottom Face ----------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left

    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left

    // Right face -----------------------------------------------------
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left

    // Left Face ------------------------------------------------------
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right

    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right
    ffi::rlEnd();
    ffi::rlPopMatrix();
}
