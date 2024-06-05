use super::pages;
use std::collections::VecDeque;

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
    oldest_block_id: usize,
}

impl FIFOScheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            oldest_block_id: 0,
        }
    }
}

impl Scheduler for FIFOScheduler {
    fn check(&mut self, new_page_id: usize, blocks: &Vec<Option<pages::Page>>, page_table: &Vec<pages::Page>) -> Result<usize, Fault> {
        // 不缺页则不做任何事，缺页则调出页并调入请求页
        match page_table[new_page_id].block_id {
            Some(block_id) => Ok(block_id),
            None => {
                let oldest_page_id = self.oldest_block_id;
                self.oldest_block_id = (self.oldest_block_id + 1) % self.capacity;
                Err(Fault::PageFault(oldest_page_id))
            },
        }
    }

    fn reset(&mut self) {
        self.oldest_block_id = 0;
    }
}

//
// LRU 置换算法实现
//

pub struct LRUScheduler {
    capacity: usize,
    current_pages: VecDeque<usize>,
}

impl LRUScheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            current_pages: VecDeque::new(),
        }
    }
}

impl Scheduler for LRUScheduler {
    fn reset(&mut self) {
        self.current_pages.clear();
    }

    fn check(&mut self, new_page_id: usize, blocks: &Vec<Option<pages::Page>>, page_table: &Vec<pages::Page>) -> Result<usize, Fault> {
        match page_table[new_page_id].block_id {
            Some(block_id) => {
                self.current_pages.retain(|page| *page != new_page_id);
                self.current_pages.push_back(new_page_id);
                Ok(block_id)
            },
            None if self.current_pages.len() < self.capacity => {
                self.current_pages.push_back(new_page_id);
                for i in 0..self.capacity {
                    if blocks[i].is_none() {
                        return Err(Fault::PageFault(i));
                    }
                }
                panic!("Unexpected branch in LRU check: {new_page_id} -- {}/{}", self.current_pages.len(), self.capacity);
            },
            None => {
                let oldest_page = self.current_pages.front().unwrap().to_owned();
                self.current_pages.push_back(new_page_id);
                self.current_pages.pop_front();
                Err(Fault::PageFault(page_table[oldest_page].block_id.unwrap()))
            },
        }
    }
    // 查看是否缺页
//    let result = match page_table[new_page_id].block_id {
//        Some(_) => Ok(new_page_id),
//        None => Err(Fault::PageFault(0)),
//    };
//
//    // 内存块尚有空闲且缺页时
//    if result.is_err() && self.current_pages.len() < self.capacity {
//        self.current_pages.push_back(new_page_id);
//        for i in 0..self.capacity {
//            if blocks[i].is_none() {
//                return Err(Fault::PageFault(i));
//            }
//        }
//        panic!("Unexpected branch!");
//    }
//
//    // 内存无空闲，不缺页则更新栈，缺页则调出页并调入请求页
//    match result {
//        Ok(page_id) => {
//            let block_id = page_table[page_id].block_id.unwrap();
//            self.current_pages.retain(|page| *page != page_id);
//            self.current_pages.push_back(page_id);
//            Ok(block_id)
//        },
//        Err(Fault::PageFault(_)) => {
//            let oldest_page = self.current_pages.back().unwrap().to_owned();
//            self.current_pages.pop_front();
//            self.current_pages.push_back(new_page_id);
//            Err(Fault::PageFault(page_table[oldest_page].block_id.unwrap()))
//        },
//        Err(_) => panic!("Unexpected branch"),
//    }
//}
}
