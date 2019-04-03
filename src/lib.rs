pub extern crate architect;
pub extern crate lightcycle;
extern crate pest;
extern crate rusttype;
#[macro_use]
extern crate pest_derive;

//pub mod color;
pub mod style;

use architect::birch::*;

use architect::*;

use lightcycle::na::*;

use lightcycle::*;

use std::str::FromStr;

type Line3<N> = [Point3<N>; 2];
type Line2<N> = [Point2<N>; 2];

// pub fn paint_tree<S: ::std::hash::BuildHasher>(
//     mut buf: Buffer<Color>,
//     colors: &HashMap<String, Color, S>,
//     xml: &Tree<Stone>,
//     clear: Option<Color>,
// ) {
//     if let Some(c) = clear {
//         lightcycle::clear(&mut buf, c)
//     }
//     let page = [
//         [0, 0].into(),
//         [buf.size[0] as isize - 1, buf.size[1] as isize - 1].into(),
//     ];
//     paint_stone(&mut buf, colors, xml, &page, &page, &Vec::new()[..], 0);
// }

pub enum Overflow {
    Scroll,
    Hide,
    Over,
    Scale,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Slide {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
    Float,
}

impl FromStr for Overflow {
    type Err = StoneError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Overflow::*;
        match &s.to_lowercase()[..] {
            "over" => Ok(Over),
            "scroll" => Ok(Scroll),
            "hide" => Ok(Hide),
            "scale" => Ok(Scale),
            _ => Err(StoneError::InvalidAttr("overflow".into())),
        }
    }
}

impl FromStr for Slide {
    type Err = StoneError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Slide::*;
        match &s.to_lowercase()[..] {
            "float" => Ok(Float),
            "up" => Ok(Up),
            "down" => Ok(Down),
            "left" => Ok(Left),
            "right" => Ok(Right),
            "in" => Ok(In),
            "out" => Ok(Out),
            _ => Err(StoneError::InvalidAttr("slide".into())),
        }
    }
}

// pub fn paint_stone<S: ::std::hash::BuildHasher>(
//     mut buf: &mut Buffer<Color>,
//     colors: &HashMap<String, Color, S>,
//     xml: &Tree<Stone>,
//     page: &Line2<isize>,
//     container: &Line2<isize>,
//     terrain: &[Line2<isize>],
//     index: usize,
// ) -> Option<Line2<isize>> {
//     use Overflow::*;
//     let node = &xml[index];
//     match &node.value {
//         Stone::Element(el) => {
//             let visible = el.parse_attr_or(xml, index, "visible", true);
//             let slide = {
//                 match node.branch() {
//                     None => Slide::Float,
//                     Some(b) => {
//                         let branch = &xml[b];
//                         if let Stone::Element(el) = &branch.value {
//                             el.parse_attr_or(xml, b, "slide", Slide::Float)
//                         } else {
//                             Slide::Float
//                         }
//                     }
//                 }
//             };
//             let mut rect = get_area(xml, index, page, &container, &terrain[..], slide);
//             if visible {
//                 let fill = colors[&el.parse_attr_or(xml, index, "fill", String::from("red"))];
//                 let overflow = {
//                     match node.branch() {
//                         None => Over,
//                         Some(b) => {
//                             let branch = &xml[b];
//                             if let Stone::Element(el) = &branch.value {
//                                 el.parse_attr_or(xml, b, "overflow", Over)
//                             } else {
//                                 Over
//                             }
//                         }
//                     }
//                 };
//                 match overflow {
//                     Scroll => panic!("Not Implemented"),
//                     Hide => panic!("Not Implemented"),
//                     Scale => {
//                         rect[1][0] = isize::min(rect[1][0], container[1][0]);
//                         rect[1][1] = isize::min(rect[1][1], container[1][1]);
//                     }
//                     Over => (),
//                 }
//                 lightcycle::plane::draw_rect(
//                     &mut buf,
//                     fill,
//                     [
//                         Point2::from(rect[0].coords.xy()),
//                         Point2::from(rect[1].coords.xy()),
//                     ],
//                 );
//             }
//             let mut terrain = Vec::new();
//             for leaf in node.leaves() {
//                 if let Some(r) = paint_stone(buf, colors, xml, page, &rect, &terrain[..], *leaf) {
//                     terrain.push(r)
//                 }
//             }
//             Some(rect)
//         }
//         Stone::Text(_t) => panic!("Not Implemented"),
//     }
// }

#[derive(Parser)]
#[grammar = "number.pest"]
pub struct NumParser;

pub fn measure_to_number(measure: &String, page: &Line2<isize>, container: &Line2<isize>) -> isize {
    use pest::Parser;
    fn delta(line: &Line2<isize>) -> Vector2<isize> {
        line[1] - line[0]
    }
    let mut pairs = NumParser::parse(Rule::Main, measure).unwrap();
    let rule_pair = pairs.next().unwrap();
    let rule = rule_pair.as_rule();
    let num: f64 = {
        let s = rule_pair.into_inner().next().unwrap().as_str();
        match s.parse() {
            Ok(n) => n,
            Err(e) => panic!("Error parsing float {}: {}", s, e),
        }
    };
    (match rule {
        Rule::CW => delta(&container)[0] as f64 * num,
        Rule::CH => delta(&container)[1] as f64 * num,
        //Rule::CD => delta(&container)[2] as f64 * num,
        Rule::PW => page[1][0] as f64 * num,
        Rule::PH => page[1][1] as f64 * num,
        //Rule::PD => page[1][2] as f64 * num,
        Rule::PX => num,
        _ => panic!("Not Implemented: {:?}", rule),
    }) as isize
}

pub fn get_area(
    xml: &Tree<Stone>,
    branch: usize,
    page: &Line2<isize>,
    container: &Line2<isize>,
    terrain: &[Line2<isize>],
    sl: Slide,
) -> Line2<isize> {
    fn reorder(mut line: Line2<isize>) -> Line2<isize> {
        if line[0][0] > line[1][0] {
            let temp = line[0][0];
            line[0][0] = line[1][0];
            line[1][0] = temp;
        }
        if line[0][1] > line[1][1] {
            let temp = line[0][1];
            line[0][1] = line[1][1];
            line[1][1] = temp;
        }
        line
    }
    let el = xml[branch].value.as_el();
    let width = measure_to_number(
        &el.parse_attr_or(xml, branch, "width", "1cw".into()),
        page,
        container,
    );
    let height = measure_to_number(
        &el.parse_attr_or(xml, branch, "height", "1ch".into()),
        page,
        container,
    );
    //let depth = measure_to_number(
    //    &el.parse_attr_or(xml, branch, "depth", "1cd".into()),
    //    container,
    //    page,
    //);
    let x = measure_to_number(
        &el.parse_attr_or(xml, branch, "x", "0cw".to_string()),
        page,
        container,
    );
    let y = measure_to_number(
        &el.parse_attr_or(xml, branch, "y", "0ch".to_string()),
        page,
        container,
    );
    //let z = measure_to_number(
    //    &el.parse_attr_or(xml, branch, "z", "1cd".to_string()),
    //    container,
    //    page,
    //);
    let p1 = Point2::new(x, y);
    let p2 = p1 + Vector2::new(width, height);
    slide(container, terrain, sl, reorder([p1, p2]))
}

fn slide(
    container: &Line2<isize>,
    terrain: &[Line2<isize>],
    slide: Slide,
    block: Line2<isize>,
) -> Line2<isize> {
    use Slide::*;
    let mut clip = match slide {
        Float => {
            return [
                block[0] + container[0].coords,
                block[1] + container[0].coords,
            ]
        }
        Up | Down => [
            [block[0][0] as f64, 0.0].into(),
            [block[1][0] as f64, container[1][1] as f64].into(),
        ],
        _ => panic!("Not Implemented: {:?}", slide),
    };
    //println!("Clip: {}", clip);
    for bound in terrain {
        let bound = [
            [bound[0].x as f64, bound[0].y as f64].into(),
            [bound[1].x as f64, bound[1].y as f64].into(),
        ];
        //println!("Clipping with {:?}", bound);
        for int in lightcycle::plane::aabb_collision(&clip, &bound) {
            match int {
                Either::A(p) => match slide {
                    Up => {
                        if clip[1].y > p.y {
                            clip[1].y = p.y
                        }
                    }
                    _ => panic!("Not Implemented: {:?}", slide),
                },
                _ => panic!("Not Implemented"),
            }
        }
    }
    //println!("Clip: {}", clip);
    match slide {
        Up => [
            [block[0].x, clip[1].y as isize - (block[1].y - block[0].y)].into(),
            [block[1].x, clip[1].y as isize].into(),
        ],
        _ => panic!("Not Implemented: {:?}", slide),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::iterators::Pair;
    use pest::Parser;

    const TEST_CW: &str = "12.34cw";

    #[test]
    fn make_pairs() {
        println!("Testing number parsing for: {}", TEST_CW);
        let pairs = match NumParser::parse(Rule::Main, TEST_CW) {
            Ok(p) => p,
            // Err(ParsingError {
            //     positives,
            //     negatives,
            //     pos,
            // }) => {
            //     println!("[ERROR]");
            //     println!("{}", selector);
            //     for _i in 0..pos.pos() {
            //         print!(" ")
            //     }
            //     println!("^");
            //     println!("+ :: {:?}", positives);
            //     println!("- :: {:?}", negatives);
            //     return;
            // }
            Err(e) => panic!("{}", e),
        };
        for pair in pairs {
            print_pair(&pair, 0)
        }
    }

    fn print_pair(pair: &Pair<Rule>, depth: u32) {
        fn tab(depth: u32) -> String {
            let mut res = String::new();
            for _i in 0..depth {
                res.push_str("|   ");
            }
            res
        }
        println!(
            "{}{:?} :: {}",
            tab(depth),
            pair.as_rule(),
            pair.clone().into_span().as_str()
        );
        //println!("{}Span: {:?}", tab(depth, false), pair.clone().into_span());
        for pair in pair.clone().into_inner() {
            print_pair(&pair, depth + 1);
        }
    }
}
