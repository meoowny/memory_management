use super::scheduler;
use super::MemState;
use super::pages;
use super::scheduler::Fault;

pub enum Choice {
    FIFO,
    LRU,
}

pub struct MemoryManager
{
    capacity: usize,
    scheduler: Box<dyn scheduler::Scheduler>,
    blocks: Vec<Option<pages::Page>>,
    page_table: Vec<pages::Page>,
    page_size: usize,
    fault_counter: usize,
    instrument_counter: usize,
}

impl MemoryManager
{
    pub fn new<T, F>(sc: F, capacity: usize, blocks: &Vec<Option<pages::Page>>, page_table: Vec<pages::Page>, page_size: usize) -> MemoryManager
    where
        T: 'static + scheduler::Scheduler,
        F: Fn(usize) -> T
    {
        MemoryManager {
            blocks: blocks.to_owned(),
            ..MemoryManager::default(sc, capacity, page_table, page_size)
        }
    }

    pub fn default<T, F>(sc: F, capacity: usize, page_table: Vec<pages::Page>, page_size: usize) -> Self
    where
        T: 'static + scheduler::Scheduler,
        F: Fn(usize) -> T
    {
        MemoryManager {
            capacity,
            scheduler: Box::new(sc(capacity)),
            blocks: vec![None; capacity],
            page_table,
            page_size,
            instrument_counter: 0,
            fault_counter: 0,
        }
    }

    pub fn step(&mut self, instrument: &usize) -> MemState {
        self.instrument_counter += 1;
        let current_page = instrument / self.page_size;
        let (is_page_fault, past_page_id, block_id) =
            match self.scheduler.check(current_page, &self.blocks, &self.page_table) {
                Ok(block_id) => {
                    (false, None, block_id)
                },
                Err(Fault::PageFault(block_id)) => {
                    let past_page = self.blocks[block_id].unwrap();
                    self.page_table[past_page.page_id].swap_out();
                    (true,
                     Some(past_page.page_id),
                     block_id)
                }
            };
        self.page_table[current_page].swap_in(block_id);
        self.blocks[block_id] = Some(self.page_table[current_page].to_owned());

        let info = if is_page_fault {
            format!("发生缺页，置换内存块 {block_id} 中的 {past_page_id:?} 页为 {current_page} 页")
        }
        else {
            String::from("正常运行")
        };

        MemState {
            sequential: self.instrument_counter,
            instrument: instrument.to_owned(),
            frame:
                self.blocks
                .iter()
                .map(|x| match x {
                    Some(page) => Some(page.page_id),
                    None => None,
                })
            .collect(),
            info,
        }
    }

    // TODO: 
    pub fn reset(&mut self) {
        self.blocks
            .iter_mut()
            .for_each(|x| { *x = None; });
        self.instrument_counter = 0;
        self.fault_counter = 0;
    }

    pub fn total_fault(&self) -> usize {
        self.fault_counter
    }
}
