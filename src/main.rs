use std::collections::BTreeMap;

enum EdgeShape
{
    Circle,
    Rect,
    Triangle,
    Trapeze
}

enum Orientation
{
    Top,
    Right,
    Bottom,
    Left
}

struct JigsawEdge
{
    shape : EdgeShape,
    is_inverted : bool,
    numbers : [u8; 2]
}

struct JigsawPiece
{
    edges : [JigsawEdge; 4],
    orientation : Orientation,
    position : u8
}



fn main() {
    let e = BTreeMap::new()
    let p = JigsawPiece {edges: [JigsawEdge{shape: }] }
    println!("Hello, world!");
}
