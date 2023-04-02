/// ### Quaternion Camera
pub struct Camera {
    projection: glam::Mat4,

    orientation: glam::Quat, // Rotation
    position: glam::Mat4,    // Translation

    pub pitch: f32,
    pub yaw: f32,
    //roll: f32
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, near_clip: f32, far_clip: f32) -> Self{
        let projection = glam::Mat4::perspective_rh(
            fov.to_radians(),
            aspect,
            near_clip,
            far_clip
        );
        let position = glam::Mat4::from_translation(glam::vec3(0.0, 0.0, 0.0));
        return Camera { 
            projection,
            orientation: glam::quat(0.0, 0.0, 0.0, 0.0),
            position,
            pitch: 0.0,
            yaw: 0.0 
        };
    }

    // Update Camera -------
    pub fn translate(&mut self, translate_vec: glam::Vec3){
        self.position *= glam::Mat4::from_translation(translate_vec);
    }

    pub fn update(&mut self){
        let pitch_quat = glam::Quat::from_rotation_x(self.pitch.to_radians());
        let yaw_quat = glam::Quat::from_rotation_y(self.yaw.to_radians());
        self.orientation = (pitch_quat*yaw_quat).normalize();
    }

    // Setters -------
    pub fn set_position(&mut self, new_position: glam::Mat4){
        self.position = new_position;
    }

    // Getters -------
    pub fn get_view_projection(&mut self) -> glam::Mat4{
        let view = glam::Mat4::from_quat(self.orientation) * self.position;
        return self.projection * view;
    }

    pub fn get_front(&self) -> glam::Vec3{
        return self.orientation.conjugate() * glam::vec3(0.0, 0.0, 1.0);
    }

    pub fn get_back(&self) -> glam::Vec3{
        return self.orientation.conjugate() * glam::vec3(0.0, 0.0, -1.0);
    }

    pub fn get_left(&self) -> glam::Vec3{
        return self.orientation.conjugate() * glam::vec3(1.0, 0.0, 0.0);
    }

    pub fn get_right(&self) -> glam::Vec3{
        return self.orientation.conjugate() * glam::vec3(-1.0, 0.0, 0.0);
    }
}