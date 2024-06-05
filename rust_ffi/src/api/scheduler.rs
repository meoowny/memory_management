use super::pages;
use std::collections;

#[derive(Debug)]
pub enum Fault {
    PageFault(usize),
}

pub trait Scheduler {
    // 检查是否缺页，如果缺页则返回缺页错误并包含应调出页的所在内存块的块号
    fn check(&mut self, new_page_id: usize, blocks: &Vec<Option<pages::Page>>, page_table: &Vec<pages::Page>) -> Result<usize, Fault>;
    fn reset(&mut self);
}

//
// FIFO 置换算法实现
//

pub struct FIFOScheduler {
    capacity: usize,
    history: collections::LinkedList<usize>,
}

impl FIFOScheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            history: collections::LinkedList::from([]),
        }
    }
}

impl Scheduler for FIFOScheduler {
    fn check(&mut self, new_page_id: usize, blocks: &Vec<Option<pages::Page>>, page_table: &Vec<pages::Page>) -> Result<usize, Fault> {
        // 查看是否缺页
        let mut it = blocks.iter();
        let result: Result<usize, Fault> = loop {
            match it.next() {
                Some(Some(page)) if page.page_id == new_page_id => break Ok(page.page_id.to_owned()),
                None => break Err(Fault::PageFault(0)),
                Some(_) => continue,
            }
        };

        // 内存块尚有空闲且缺页时
        if result.is_err() && blocks.iter().find(|&&x| x.is_none()).is_some() {
            self.history.push_front(new_page_id);
            for i in 0..self.capacity {
                if blocks[i].is_none() {
                    return Ok(i);
                }
            }
            panic!("Unexpected branch!");
        }

        // 内存无空闲，不缺页则不做任何事，缺页则调出页并调入请求页
        match result {
            Ok(page_id) => {
                let block_id = page_table[page_id].block_id.unwrap();
                Ok(block_id)
            },
            Err(Fault::PageFault(0)) => {
                let oldest_page = self.history.back().unwrap().to_owned();
                self.history.pop_back();
                self.history.push_front(new_page_id);
                Err(Fault::PageFault(page_table[oldest_page].block_id.unwrap()))
            },
            Err(_) => panic!("Unexpected branch"),
        }
    }

    fn reset(&mut self) {
        self.history.clear();
    }
}

//
// LRU 置换算法实现
//

pub struct LRUScheduler {
    capacity: usize,
    current_pages: Vec<usize>,
}

impl LRUScheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            current_pages: vec![],
        }
    }
}

impl Scheduler for LRUScheduler {
    fn reset(&mut self) {
        self.current_pages.clear();
    }

    fn check(&mut self, new_page_id: usize, blocks: &Vec<Option<pages::Page>>, page_table: &Vec<pages::Page>) -> Result<usize, Fault> {
        // 查看是否缺页
        let mut it = self.current_pages.iter();
        let result: Result<usize, Fault> = loop {
            match it.next() {
                Some(page_id) if *page_id == new_page_id => break Ok(page_id.to_owned()),
                None => break Err(Fault::PageFault(0)),
                Some(_) => continue,
            }
        };

        // 内存块尚有空闲且缺页时
        if result.is_err() && self.current_pages.len() < self.capacity {
            self.current_pages.push(new_page_id);
            for i in 0..self.capacity {
                if blocks[i].is_none() {
                    return Ok(i);
                }
            }
            panic!("Unexpected branch!");
        }

        // 内存无空闲，不缺页则更新栈，缺页则调出页并调入请求页
        match result {
            Ok(page_id) => {
                let block_id = page_table[page_id].block_id.unwrap();
                self.current_pages.retain(|page| *page != page_id);
                self.current_pages.push(page_id);
                Ok(block_id)
            },
            Err(Fault::PageFault(0)) => {
                let oldest_page = self.current_pages.last().unwrap().to_owned();
                self.current_pages.pop();
                self.current_pages.push(new_page_id);
                Err(Fault::PageFault(page_table[oldest_page].block_id.unwrap()))
            },
            Err(_) => panic!("Unexpected branch"),
        }
    }
}
