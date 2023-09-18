use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    number: usize,
    is_current: bool,
}

impl Page {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            is_current: false,
        }
    }

    pub fn pages() -> Vec<Self> {
        let mut pages = Vec::new();
        for p in 1..=10 {
            let page = Page::new(p);
            pages.push(page)
        }
        pages
    }
}

pub fn current_page(pages: &mut [Page], page_num: usize) {
    let mut page = pages[page_num - 1].clone();
    page.is_current = true;
    pages[page_num - 1] = page;
}
