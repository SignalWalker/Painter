// use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::str::FromStr;

// #[derive(Copy, Clone)]
// pub struct XMLColor(pub Color);

// impl From<Color> for XMLColor {
//     fn from(c: Color) -> Self {
//         XMLColor(c)
//     }
// }

// impl Into<Color> for XMLColor {
//     fn into(self) -> Color {
//         self.0
//     }
// }

// impl FromStr for XMLColor {
//     type Err = String;
//     fn from_str(hex: &str) -> Result<Self, Self::Err> {
//         if hex.len() != 6 && hex.len() != 8 {
//             return Err("Invalid Hex Length".into());
//         }
//         let r = match u8::from_str_radix(&hex[0..2], 16) {
//             Ok(n) => n,
//             Err(e) => return Err(format!("Failed to parse r: {}", e)),
//         };
//         let g = match u8::from_str_radix(&hex[2..4], 16) {
//             Ok(n) => n,
//             Err(e) => return Err(format!("Failed to parse g: {}", e)),
//         };
//         let b = match u8::from_str_radix(&hex[4..6], 16) {
//             Ok(n) => n,
//             Err(e) => return Err(format!("Failed to parse b: {}", e)),
//         };
//         let a = if hex.len() == 8 {
//             match u8::from_str_radix(&hex[6..8], 16) {
//                 Ok(n) => n,
//                 Err(e) => return Err(format!("Failed to parse a: {}", e)),
//             }
//         } else {
//             255
//         };
//         Ok(XMLColor([r, g, b, a]))
//     }
// }

// pub struct ColorMason {
//     pub colors: HashMap<String, Color>,
//     pub errors: Vec<StoneError>,
// }

// impl Default for ColorMason {
//     fn default() -> ColorMason {
//         ColorMason {
//             colors: HashMap::new(),
//             errors: Vec::new(),
//         }
//     }
// }

// impl ColorMason {
//     fn handle_color(&mut self, arch: &Architect, index: usize) {
//         let element = &arch.stones[index].value.as_el();
//         if !element.attr.contains_key("id") {
//             self.errors.push(StoneError::MissingAttr("id".into()));
//             return;
//         } else if !element.attr.contains_key("val") {
//             self.errors.push(StoneError::MissingAttr("val".into()));
//             return;
//         }
//         let id = String::from_attr(&element.attr["id"], &arch.stones, index).unwrap();
//         self.colors.insert(
//             id,
//             match XMLColor::from_attr(&element["val"], &arch.stones, index) {
//                 Ok(c) => c.into(),
//                 Err(e) => {
//                     self.errors
//                         .push(StoneError::InvalidAttr(format!("val: {}", e)));
//                     return;
//                 }
//             },
//         );
//     }
// }

// impl StoneMason for ColorMason {
//     fn handle_stones(
//         &mut self,
//         arch: &mut Architect,
//         map: &mut HashMap<String, Vec<usize>>,
//     ) -> HashSet<usize> {
//         let mut res = HashSet::new();
//         if !map.contains_key("color") {
//             return res;
//         }

//         for i in map.get("color").unwrap() {
//             self.handle_color(&arch, *i);
//             res.insert(*i);
//         }

//         res
//     }
// }
