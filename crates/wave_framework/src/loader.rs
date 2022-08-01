//! Tileset loader.
#![allow(dead_code)] // remove when implementation is done
use std::{collections::HashMap, fmt, fs, io, path::Path};
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    reader::{EventReader, XmlEvent},
};

/// Load a tileset from the given XML file.
// TODO: return Result
pub fn load_tileset_file<P>(filepath: P) -> Option<Tileset>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filepath).unwrap();
    let reader = io::BufReader::new(file);
    match XmlParser::new(reader).parse() {
        Ok(tileset) => Some(tileset),
        Err(err) => {
            log::error!("tileset parse error: {}", err);
            None
        }
    }
}

const XML_NODE_ROOT: &str = "set";
const XML_NODE_TILES: &str = "tiles";
const XML_NODE_TILE: &str = "tile";
const XML_NODE_NEIGHBOURS: &str = "neighbors";
const XML_NODE_NEIGHBOUR: &str = "neighbor";
const XML_ATTR_NAME: &str = "name";
const XML_ATTR_SYMMERTY: &str = "symmetry";
const XML_ATTR_WEIGHT: &str = "weight";
const XML_ATTR_LEFT: &str = "left";
const XML_ATTR_RIGHT: &str = "right";

const SYM_X: &str = "X";
const SYM_T: &str = "T";
const SYM_I: &str = "I";
const SYM_L: &str = "L";
const SYM_F: &str = "F";
const SYM_DIAG: &str = "\\";

/// Number of possible directions around a tile.
/// Up, down, left, right.
const DIRECTION_N: usize = 4;

/// Maximum number of possible tile orientation cases.
/// Tile can be rotated to every direction, and flipped (mirrored).
const CASE_N: usize = DIRECTION_N * 2;

#[derive(Default)]
pub struct Tileset {
    tiles: Vec<Tile>,
}

pub struct Tile {}

/// Current state of the parser as it traverses the XML tree.
#[derive(Clone, Copy)]
enum XmlState {
    Off,
    Root,
    Tiles,
    Neighbours,
}

enum NeighKind<'a> {
    Tile(&'a str),
    Complex { tile: &'a str, num: &'a str },
}

impl<'a> NeighKind<'a> {
    fn tile(&self) -> &str {
        match self {
            &NeighKind::Tile(tile) => tile,
            &NeighKind::Complex { tile, .. } => tile,
        }
    }

    fn num(&self) -> usize {
        match self {
            // Implicitly when no number is specified the index is 0
            &NeighKind::Tile(_) => 0,
            &NeighKind::Complex { num, .. } => {
                num.parse::<usize>().expect("failed parsing tile number")
            }
        }
    }
}

impl<'a> fmt::Display for NeighKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Mimic attribute from original XML input.
        match self {
            &NeighKind::Tile(tile) => write!(f, "{}", tile),
            &NeighKind::Complex { tile, num } => write!(f, "{} {}", tile, num),
        }
    }
}

struct XmlParser<R: io::Read> {
    reader: EventReader<R>,
    stack: Vec<XmlState>,
    /// Current depth in the XML tree.
    depth: i32,
    action: Vec<[i32; 8]>,
    /// Mapping of tile names to their offset into
    /// the `action` vector. This can be used to
    /// lookup a tile's actions.
    ///
    /// Called `firstOccurrence` in reference codebase.
    offsets: HashMap<String, usize>,
    /// Resulting tileset that's being built up by the parser.
    tileset: Tileset,
}

impl<R: io::Read> XmlParser<R> {
    fn new(reader: R) -> Self {
        XmlParser {
            reader: EventReader::new(reader),
            stack: vec![XmlState::Off],
            depth: 0,
            action: vec![],
            offsets: HashMap::new(),
            tileset: Tileset::default(),
        }
    }

    fn state(&self) -> XmlState {
        self.stack.last().cloned().unwrap_or(XmlState::Off)
    }

    fn push(&mut self, state: XmlState) {
        self.stack.push(state)
    }

    fn pop(&mut self) -> XmlState {
        self.stack.pop().unwrap_or(XmlState::Off)
    }

    fn parse(mut self) -> xml::reader::Result<Tileset> {
        loop {
            match self.reader.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    self.depth += 1;

                    match self.state() {
                        XmlState::Off => {
                            if name.local_name == XML_NODE_ROOT {
                                self.stack.push(XmlState::Root);
                            }
                        }
                        XmlState::Root => {
                            match name.local_name.as_str() {
                                XML_NODE_TILES => {
                                    self.stack.push(XmlState::Tiles);
                                }
                                XML_NODE_NEIGHBOURS => {
                                    self.stack.push(XmlState::Neighbours);
                                }
                                _ => { /* consume unknown nodes */ }
                            }
                        }
                        XmlState::Tiles => {
                            if name.local_name == XML_NODE_TILE {
                                self.handle_tile(name, &attributes);
                            }
                        }
                        XmlState::Neighbours => {
                            if name.local_name == XML_NODE_NEIGHBOUR {
                                self.handle_neighbour(name, &attributes);
                            }
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    self.depth -= 1;

                    match self.state() {
                        XmlState::Tiles => {
                            if name.local_name == XML_NODE_TILES {
                                self.pop();
                            }
                        }
                        _ => { /* do nothing  */ }
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(err) if err.kind() == &xml::reader::ErrorKind::UnexpectedEof => {
                    log::warn!("unexpected xml eof");
                    break;
                }
                Err(err) => return Err(err),
                _ => { /* do nothing */ }
            }
        }

        let Self { tileset, .. } = self;

        Ok(tileset)
    }

    fn handle_tile(&mut self, name: OwnedName, attributes: &[OwnedAttribute]) {
        if name.local_name == XML_NODE_TILE {
            // Extract tile name and symmetry
            let mut tilename: Option<&str> = None;
            let mut symmetry: Option<&str> = None;

            for attr in attributes {
                match attr.name.local_name.as_str() {
                    XML_ATTR_NAME => tilename = Some(attr.value.as_str()),
                    XML_ATTR_SYMMERTY => symmetry = Some(attr.value.as_str()),
                    attr_name => {
                        log::warn!("unknown attribute in tile node: {}", attr_name);
                    }
                }
            }

            // Both must be present for a valid tile
            if let (Some(t), Some(s)) = (tilename, symmetry) {
                // TODO: Extract weight from tile node
                self.create_tile(t, s, "1.0");
            } else {
                log::warn!(
                    "incomplete tile node: name=\"{:?}\" symmetry=\"{:?}\"",
                    tilename,
                    symmetry
                );
            }
        }
    }

    /// Adds a tile into the set.
    #[rustfmt::skip]
    fn create_tile(&mut self, tilename: &str, symmetry: &str, weight: &str) {
        log::info!("tile: {} {} {}", tilename, symmetry, weight);

        // TODO: Return float parse error
        let _weight = weight.parse::<f32>().unwrap();

        let cardinality: i32;
        let a: &dyn Fn(i32) -> i32;
        let b: &dyn Fn(i32) -> i32;

        match symmetry {
            SYM_L => {
                cardinality = 4;
                a = &|i| (i + 1) % 4;
                b = &|i| if i % 2 == 0 { i + 1 } else { i - 1 };
            }
            SYM_T => {
                cardinality = 4;
                a = &|i| (i + 1) % 4;
                b = &|i| if i % 2 == 0 { i } else { 4 - i };
            }
            SYM_I => {
                cardinality = 2;
                a = &|i| 1 - i;
                b = &|i| i;
            }
            SYM_DIAG => {
                cardinality = 2;
                a = &|i| 1 - i;
                b = &|i| 1 - i;
            }
            SYM_F => {
                cardinality = 8;
                a = &|i| if i < 4 { (i + 1) % 4 } else { 4 + (i - 1) % 4 };
                b = &|i| if i < 4 { i + 4 } else { i - 4 };
            }
            SYM_X | _ => {
                cardinality = 1;
                a = &|i| i;
                b = &|i| i;
            }
        }

        let offset = self.action.len(); // called T in original
        self.offsets.insert(tilename.to_string(), offset);

        for t in 0..cardinality {
            let mut map = [0; CASE_N];

            map[0] = t;
            map[1] = a(t);
            map[2] = a(a(t));
            map[3] = a(a(a(t)));
            map[4] = b(t);
            map[5] = b(a(t));
            map[6] = b(a(a(t)));
            map[7] = b(a(a(a(t))));

            for s in &mut map {
                *s += offset as i32;
            }

            self.action.push(map);
        }

        // TODO: Load bitmap
    }

    fn handle_neighbour(&mut self, _name: OwnedName, attributes: &[OwnedAttribute]) {
        // Text in the left and right attributes encode two values:
        // a tile name and a case number (related to 'cardinality')
        let left = match attributes
            .iter()
            .find(|attr| attr.name.local_name == XML_ATTR_LEFT)
            .and_then(extract_neighbour)
        {
            Some(neigh) => neigh,
            None => {
                log::warn!("neighbour left not found");
                return;
            }
        };

        let right = match attributes
            .iter()
            .find(|attr| attr.name.local_name == XML_ATTR_RIGHT)
            .and_then(extract_neighbour)
        {
            Some(neigh) => neigh,
            None => {
                log::warn!("neighbour right not found");
                return;
            }
        };

        log::info!("neighbours: {}, {}", left, right);

        // TODO: Lookup firstOccurence by tilename
        // TODO: If neighbour kind is complex, parse num to usize
        // TODO: use num to lookup action
    }
}

/// Extract tile and case number from neighbour attribute,
/// either the `left` or `right` value.
///
/// The text encodes tile information like the following:
///
///     "{tile} {num}"
///
/// The first `tile` part is required. The second `num` part is optional.
///
/// If the `tile` part is not present, `None` is returned.
#[inline]
fn extract_neighbour(attr: &OwnedAttribute) -> Option<NeighKind> {
    let mut parts = attr.value.split_whitespace().filter(|s| !s.is_empty());

    let tile = parts.next();
    let num = parts.next();

    match (tile, num) {
        (Some(tile), Some(num)) => Some(NeighKind::Complex { tile, num }),
        (Some(tile), None) => Some(NeighKind::Tile(tile)),
        (None, None) | _ => None,
    }
}
