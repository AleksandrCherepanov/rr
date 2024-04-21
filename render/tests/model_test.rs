use render::model::{Face, Model};
use render::vertex::Vertex;

#[test]
fn open() {
    let m = Model::open("tests/data/model/model.obj");

    let expected_faces = vec![
        Face { vertices: vec![0, 3, 1], textures: vec![0, 1, 2], normals: vec![0, 0, 0] },
        Face { vertices: vec![0, 2, 3], textures: vec![0, 3, 1], normals: vec![0, 0, 0] },
    ];

    let expected_textures = vec![
        Vertex::create(0.999900, 0.999900, 0.0),
        Vertex::create(0.000100, 0.000100, 0.0),
        Vertex::create(0.999900, 0.000100, 0.0),
        Vertex::create(0.000100, 0.999900, 0.0),
    ];

    let expected_vertices = vec![
        Vertex::create(1.000000, 1.000000, -0.000000),
        Vertex::create(1.000000, -1.000000, -0.000000),
        Vertex::create(-1.000000, 1.000000, 0.000000),
        Vertex::create(-1.000000, -1.000000, 0.000000),
    ];

    assert_eq!(expected_faces, m.faces);
    assert_eq!(expected_textures, m.textures);
    assert_eq!(expected_vertices, m.vertices);
}
