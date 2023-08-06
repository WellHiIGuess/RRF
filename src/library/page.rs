use super::element::Element;

pub struct Page {
    html: String,
    // style_sheet:,
}

impl Page {
    pub fn new(html: String) -> Self {
        Self {
            html,
        }
    }
    pub fn serve(self) -> String {
        self.html
    }
}

pub fn page(elements: Vec<&dyn Element>) -> Page {
    let output: String = elements
        .iter()
        .map(|&m| m.get_html())
        .collect();

    Page::new(output)
}
