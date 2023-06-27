use std::collections::{BTreeMap, HashMap};
use std::{fs, mem};
use serde::{Deserialize, Serialize};
//use serde::de::Unexpected::Option;
use serde_json::de::IoRead;
use serde_with::serde_as;
use serde_json::{Number, Result};
use core::option::Option;

#[derive(Serialize, Deserialize)]
enum EdgeShape
{
    Circle,
    Rect,
    Triangle,
    Trapeze,
}


#[derive(Serialize, Deserialize, Copy, Clone)]
struct Vector
{
    x: i8,
    y: i8,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Rotation
{
    matrix: [[i8; 2]; 2],
}

impl Rotation
{
    fn new() -> Rotation
    {
        Rotation { matrix: [[1, 0], [0, 1]] }
    }

    fn rotate(&self) -> Rotation
    {
        let mut result = *self;
        result.matrix[0][0] = self.matrix[0][1];
        result.matrix[0][1] = -self.matrix[0][0];
        result.matrix[1][0] = self.matrix[1][1];
        result.matrix[1][1] = -self.matrix[1][0];
        result
    }

    fn rotate_n(&self, count: usize) -> Self
    {
        let mut result = *self;
        for _ in 0..count
        {
            result = result.rotate();
        }
        result
    }
}

impl Eq for Rotation
{}

#[test]
fn test_rotate()
{
    let mut r = Rotation::new();
    assert_eq!(Rotation::new(), r.rotate().rotate().rotate().rotate());
    assert_eq!(Rotation::new(), r.rotate_n(4));
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Orientation
{
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Orientation
{
    fn rotate(&self) -> Orientation
    {
        match &self
        {
            Orientation::Top => Orientation::Right,
            Orientation::Right => Orientation::Bottom,
            Orientation::Bottom => Orientation::Left,
            Orientation::Left => Orientation::Top
        }
    }
}

impl Eq for Orientation
{}


impl From<Orientation> for Vector
{
    fn from(orientation: Orientation) -> Self
    {
        match orientation {
            Orientation::Top => Vector { x: 0, y: 1 },
            Orientation::Right => Vector { x: 1, y: 0 },
            Orientation::Bottom => Vector { x: 0, y: -1 },
            Orientation::Left => Vector { x: -1, y: 0 }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct JigsawEdge
{
    shape: EdgeShape,
    is_inverted: bool,
    numbers: [String; 2],
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct JigsawPiece
{
    #[serde_as(as = "Vec<(_, _)>")]
    edges: HashMap<Orientation, JigsawEdge>,
    id: u8,
}

impl PartialEq for JigsawPiece
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Copy)]
struct Place<'a>
{
    rotation: Rotation,
    place: Vector,
    piece: Option<&'a JigsawPiece>,
}

struct Field<'a>
{
    pieces: &'a Vec<JigsawPiece>,
    places: Vec<Place<'a>>,
}

impl<'a> Field<'a> {
    fn next_move(&self) -> Vec<Self> {
        let laid_pieces: Vec<_> = self.places.iter().map(|p| p.piece).filter(|p| p.is_some()).map(|p| p.unwrap()).collect();
        let available_pieces: Vec<_> = self.pieces.iter().filter(|p| !laid_pieces.contains(p)).collect();
        let available_count = available_pieces.len();
        available_pieces.iter()
            .map(|p| (0..4).map(|i|
                Place {
                    rotation: Rotation::new().rotate_n(i),
                    place: Field::position_to_vector(laid_pieces.len() as i8),
                    piece: Some(p),
                }).collect::<Vec<_>>())
            .flatten()
            .map(|p|
                {
                    let mut places = self.places.clone();
                    places.push(p);
                    Field{pieces: self.pieces, places}
                }).collect()
    }
}

impl<'a> Field<'a>
{
    fn set(&mut self, position: i8, piece: &'a JigsawPiece)
    {
        self.places[position as usize] = Place { rotation: Rotation::new(), place: Field::position_to_vector(position), piece: Option::from(piece) };
    }

    fn position_to_vector(position: i8) -> Vector
    {
        Vector { x: position * 2 % 6 + 1, y: (position / 3) * 2 + 1 }
    }
}

fn main() {
    let data = fs::read_to_string("puzzle.json").unwrap();
    let pieces: Vec<JigsawPiece> = serde_json::from_str(&data).unwrap();
    //let mut p= [Place{rotation: Rotation::new(),place: Vector{x:1, y:1},piece: None} ;9];
    let places: Vec<_> = (0..9).into_iter().map(|i| Place { rotation: Rotation::new(), place: Field::position_to_vector(i), piece: None }).collect();
    let mut field = Field { places, pieces: &pieces };
    let f = field.next_move();
    println!("Hello, world!");
}
