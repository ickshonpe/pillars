



pub type Color = [f32; 4];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.4, 0.0, 1.0];



#[repr(C, packed)]
pub struct CVertex2(pub Vertex2, pub Color);

#[repr(C, packed)]
pub struct TCVertex2(pub Vertex2, pub Vertex2, pub Color);

#[repr(C, packed)]
pub struct TVertex2(pub Vertex2, pub Vertex2);

pub type Matrix4 = [f32; 16];
pub type Vertex3 = [f32; 3];
pub type Vertex2 = [f32; 2];

pub fn calculate_orthogonal_projection_matrix(size: [f32; 2], position: [f32; 2]) -> [f32; 16] {
    let w = 2.0 / size[0];
    let h = 2.0 / size[1];
    let (tx, ty) = (position[0] * w, position[1] * h);
    [
        w, 0.0, 0.0, 0.0,
        0.0, h, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0 - tx, -1.0 - ty, 0.0, 1.0
    ]
}

pub const IDENTITY_MATRIX4: [f32; 16] = [
    1.,0.,0.,0.,
    0.,1.,0.,0.,
    0.,0.,1.,0.,
    0.,0.,0.,1.
];