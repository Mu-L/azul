use azul_core::styled_dom::StyledDom;
use azul_css::ColorU;

pub fn coloru_from_str(s: &str) -> ColorU {
    azul_css::parser::parse_css_color(s)
        .ok()
        .unwrap_or(ColorU::BLACK)
}

// extra functions that can't be implemented in azul_core
#[cfg(not(feature = "xml"))]
pub fn styled_dom_from_file(_: &str) -> StyledDom {
    Dom::body()
        .with_children(
            vec![Dom::text(format!(
                "library was not compiled with --feature=\"xml\""
            ))]
            .into(),
        )
        .style(&mut Css::empty())
}

#[cfg(feature = "xml")]
pub fn styled_dom_from_file(path: &str) -> StyledDom {
    use azul_layout::xml::XmlComponentMap;
    azul_layout::xml::domxml_from_file(path, &mut XmlComponentMap::default()).parsed_dom
}

#[cfg(not(feature = "xml"))]
pub fn styled_dom_from_str(_: &str) -> StyledDom {
    Dom::body()
        .with_children(
            vec![Dom::text(format!(
                "library was not compiled with --feature=\"xml\""
            ))]
            .into(),
        )
        .style(&mut Css::empty())
}

#[cfg(feature = "xml")]
pub fn styled_dom_from_str(s: &str) -> StyledDom {
    use azul_layout::xml::XmlComponentMap;
    azul_layout::xml::domxml_from_str(s, &mut XmlComponentMap::default()).parsed_dom
}
