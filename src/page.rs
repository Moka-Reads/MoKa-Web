use mokareads_core::awesome_lists::Repository;
use serde::{Deserialize, Serialize};
use crate::ALIST;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page{
    number: usize,
    is_current: bool,
    repos: Vec<Repository>
}

impl Page{
    pub fn new(number: usize, repos: Vec<Repository>) -> Self{
        Self{number, is_current: false, repos}
    }

    pub async fn pages() -> Vec<Self>{
        let alist = ALIST.read().await;
        let mut pages = Vec::new();
        for p in 1..=10{
            let repo = alist.get_page(p);
            let page = Page::new(p, repo);
            pages.push(page)
        }
        pages
    }
}

pub fn current_page(pages: &mut Vec<Page>, page_num: usize){
    let mut page = pages[page_num - 1].clone();
    page.is_current = true;
    pages[page_num - 1] = page;
}