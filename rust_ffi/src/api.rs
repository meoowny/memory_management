mod memory_manager;
mod pages;
mod scheduler;

// 置换算法选择
pub enum AlgoChoice {
    FIFO,
    LRU,
}
// 指令序列生成方法选择
pub enum GenChoice {
    Random,
    Specific,
}

// 每条指令执行后内存状态
pub struct MemState {
    pub sequential: usize,
    pub instrument: usize,
    pub frame: Vec<Option<usize>>,
    pub info: String,
}

// 作业执行结果，含各指令执行后内存状态与缺页统计
pub struct ExecRecord {
    pub records: Vec<MemState>,
    pub total_instrument: usize,
    pub total_faults: usize,
}

// 模拟作业执行过程
pub fn generate_replacement_record(
    mem_capacity: usize,
    total_instrument: usize,
    page_size: usize,
    algo_choice: AlgoChoice,
    gen_choice: GenChoice,
) -> ExecRecord {
    // 初始化页表与指令执行顺序
    let mut page_table = Vec::new();
    for i in 0..total_instrument / page_size {
        page_table.push(pages::Page::default(i));
    }
    let instrument_order = match gen_choice {
        GenChoice::Random => pages::gen_random_order(total_instrument),
        GenChoice::Specific => pages::gen_specific_order(total_instrument),
    };

    let mut mem_manager = match algo_choice {
        AlgoChoice::FIFO => memory_manager::MemoryManager::default(
            scheduler::FIFOScheduler::new,
            mem_capacity,
            page_table,
            page_size,
        ),
        AlgoChoice::LRU => memory_manager::MemoryManager::default(
            scheduler::LRUScheduler::new,
            mem_capacity,
            page_table,
            page_size,
        ),
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
