mod memory_manager;
mod scheduler;
mod pages;

pub use memory_manager::Choice;

pub struct MemState {
    pub sequential: usize,
    pub instrument: usize,
    pub frame: Vec<Option<usize>>,
    pub info: String,
}

pub struct ExecRecord {
    pub records: Vec<MemState>,
    pub total_instrument: usize,
    pub total_faults: usize,
}

pub fn generate_replacement_record(mem_capacity: usize, total_instrument: usize, choice: Choice, page_size: usize) -> ExecRecord {
    // 初始化页表与指令执行顺序
    let mut page_table = Vec::new();
    for i in 0..total_instrument/page_size {
        page_table.push(pages::Page::default(i));
    }
    let instrument_order = pages::gen_access_order(total_instrument);

    let mut mem_manager = match choice {
        Choice::FIFO => memory_manager::MemoryManager::default(scheduler::FIFOScheduler::new, mem_capacity, page_table, page_size),
        Choice::LRU => memory_manager::MemoryManager::default(scheduler::LRUScheduler::new, mem_capacity, page_table, page_size),
    };

    // 模拟请求调页，并将结果存入数组
    let mut records = Vec::new();
    println!("{instrument_order:?}");
    for instrument in instrument_order.iter() {
        records.push(mem_manager.step(instrument));
    }

    // 获取缺页数
    let total_faults = mem_manager.total_fault();

    // 返回执行情况
    ExecRecord {
        records,
        total_instrument,
        total_faults,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_works() {
        let mut page_table = Vec::new();
        for i in 0..32 {
            page_table.push(pages::Page::default(i));
        }
        let mut mm = memory_manager::MemoryManager::default(scheduler::FIFOScheduler::new, 4, page_table, 10);
        let result = mm.step(&2);
        assert_eq!(result.sequential, 1);
    }

    #[test]
    fn lru_works() {
        let mut page_table = Vec::new();
        for i in 0..32 {
            page_table.push(pages::Page::default(i));
        }
        let mut mm = memory_manager::MemoryManager::default(scheduler::LRUScheduler::new, 4, page_table, 10);
        let result = mm.step(&2);
        assert_eq!(result.sequential, 1);
    }
}
