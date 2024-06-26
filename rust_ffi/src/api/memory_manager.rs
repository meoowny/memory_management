use super::pages;
use super::scheduler;
use super::scheduler::Fault;
use super::MemState;

pub struct MemoryManager {
    // capacity: usize,
    scheduler: Box<dyn scheduler::Scheduler>,
    blocks: Vec<Option<pages::Page>>,
    page_table: Vec<pages::Page>,
    page_size: usize,
    fault_counter: usize,
    instrument_counter: usize,
}

impl MemoryManager {
    pub fn new<T, F>(
        sc: F,
        capacity: usize,
        blocks: &Vec<Option<pages::Page>>,
        page_table: Vec<pages::Page>,
        page_size: usize,
    ) -> MemoryManager
    where
        T: 'static + scheduler::Scheduler,
        F: Fn(usize) -> T,
    {
        MemoryManager {
            // capacity,
            scheduler: Box::new(sc(capacity)),
            blocks: blocks.to_owned(),
            page_table,
            page_size,
            instrument_counter: 0,
            fault_counter: 0,
        }
    }

    pub fn default<T, F>(
        sc: F,
        capacity: usize,
        page_table: Vec<pages::Page>,
        page_size: usize,
    ) -> Self
    where
        T: 'static + scheduler::Scheduler,
        F: Fn(usize) -> T,
    {
        Self::new(sc, capacity, &vec![None; capacity], page_table, page_size)
    }

    pub fn step(&mut self, instrument: &usize) -> MemState {
        // 指令计数器加一
        self.instrument_counter += 1;
        // 计算当前指令所在页号
        let current_page = instrument / self.page_size;
        // 通过 scheduler 成员的 check 查看是否缺页，如果缺页需要调出的是哪一页
        let (is_page_fault, past_page_id, block_id) =
            match self
                .scheduler
                .check(current_page, &self.blocks, &self.page_table)
            {
                Ok(block_id) => (false, None, block_id),
                Err(Fault::PageFault(block_id)) => {
                    // 缺页则将缺页计数加一
                    self.fault_counter += 1;

                    (
                        true,
                        self.blocks[block_id].map_or(None, |page| {
                            self.page_table[page.page_id].swap_out();
                            Some(page.page_id)
                        }),
                        block_id,
                    )
                }
            };

        let info = if is_page_fault {
            self.page_table[current_page].swap_in(block_id);
            // 缺页则更新页表和内存块
            self.blocks[block_id] = Some(self.page_table[current_page].to_owned());

            match past_page_id {
                Some(id) => {
                    format!("发生缺页，置换内存块 {block_id} 中的第 {id} 页为第 {current_page} 页")
                }
                None => format!("发生缺页，内存块 {block_id} 调入第 {current_page} 页"),
            }
        } else {
            format!("指令 {instrument} 已在内存中，正常运行中")
        };

        MemState {
            sequential: self.instrument_counter,
            instrument: instrument.to_owned(),
            frame: self
                .blocks
                .iter()
                .map(|x| match x {
                    Some(page) => Some(page.page_id),
                    None => None,
                })
                .collect(),
            info,
        }
    }

    pub fn total_fault(&self) -> usize {
        self.fault_counter
    }
}
