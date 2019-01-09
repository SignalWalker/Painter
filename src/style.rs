use architect::birch::Tree;
use architect::pest;
use architect::pest_derive;
use architect::select::*;
use architect::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Style(Selector, Tree<Stone>);

impl Default for Style {
    fn default() -> Style {
        Style(
            match "$*".parse() {
                Ok(s) => s,
                Err(e) => panic!("Error parsing $*: {:?}", e),
            },
            Tree::with_root(Element::new("style".into()).into()),
        )
    }
}

impl PartialEq<Selector> for Style {
    fn eq(&self, other: &Selector) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Style> for Style {
    fn eq(&self, other: &Style) -> bool {
        self.0 == other.0
    }
}

pub struct StyleMason {
    style_tree: Tree<Style>,
    errors: Vec<StoneError>,
}

impl Default for StyleMason {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleMason {
    pub fn new() -> StyleMason {
        StyleMason {
            style_tree: Tree::with_root(Style::default()),
            errors: Vec::new(),
        }
    }
}

impl StyleMason {
    pub fn handle_style(&mut self, arch: &mut Architect, style: usize) -> Result<(), StoneError> {
        let select = {
            let attr = match arch.stones[style].value {
                Stone::Element(ref mut el) => el,
                _ => return Err(StoneError::InvalidRoot),
            }
            .attr
            .remove("select")
            .unwrap();
            match format!(
                "${}",
                match attr {
                    // this is probably a little confusing but whatever
                    // (if you put a $ at the beginning of a style selector, it will be
                    // interpreted as selecting some other string, rather than being the selector
                    // for a style)
                    Attribute::Select(s) => s.select(&arch.stones, 0, style).1[0].into(),
                    Attribute::String(t) => t,
                }
            )
            .parse()
            {
                Ok(s) => s,
                Err(e) => return Err(StoneError::from(e)),
            }
        };

        let style = Style(select, arch.stones.remove(style));

        let leaf = self
            .style_tree
            .first_nearest(&style, |a, b| a.0.subsumes(&b.0));

        if self.style_tree[leaf].value == style {
            combine_tree(&mut self.style_tree[leaf].value.1, style.1, 0);
        } else {
            self.style_tree.push(leaf, style)
        }

        Ok(())
    }
}

impl StoneMason for StyleMason {
    fn handle_stones(
        &mut self,
        arch: &mut Architect,
        map: &mut HashMap<String, Vec<usize>>,
    ) -> HashSet<usize> {
        if !map.contains_key("style") {
            return HashSet::new();
        }
        for i in map.get("style").unwrap() {
            if let Err(e) = self.handle_style(arch, *i) {
                arch.errors.push(e)
            }
        }
        HashSet::new()
    }
}

pub fn combine_tree(a: &mut Tree<Stone>, mut b: Tree<Stone>, branch: usize) -> &mut Tree<Stone> {
    match a[branch].value {
        Stone::Element(ref mut el) => *el = el.clone() & b[0].value.as_el().clone(),
        _ => panic!("Root of a is not element"),
    }
    for leaf in b[0].leaves().to_vec() {
        let leaf = b.remove(leaf);
        a.push_tree(branch, leaf);
    }
    a
}
