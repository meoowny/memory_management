use rand::Rng;
use rand::distributions::{Distribution, Uniform};

pub fn gen_access_order(total_instrument: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0.0..1.0);

    let mut results: Vec<usize> = Vec::new();

    for _ in 0..total_instrument {
        match die.sample(&mut rng) {
            x if x < 0.5 => {
                results.push(
                    match results.last() {
                        Some(x) if x + 1 < total_instrument => x + 1,
                        _ => 0
                    })
            },
            x if x < 0.75 => results.push(rng.gen_range(0..(results.last().unwrap_or(&(total_instrument - 1)) + 1))),
            _ => results.push(rng.gen_range(results.last().unwrap_or(&0).clone()..total_instrument)),
        }
    }
    results
}

pub struct Page {
    pub page_id: usize,           // 页号
    pub block_id: Option<usize>,  // 内存块号（含中断位信息，由 Option 体现）
}

impl Page {
    pub fn default(page_id: usize) -> Self {
        Page {
            page_id,
            block_id: None,
        }
    }

    pub fn swap_in(&mut self, block_id: usize) {
        self.block_id = Some(block_id);
    }

    pub fn swap_out(&mut self) {
        self.block_id = None;
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        Page {
            page_id: self.page_id,
            block_id: self.block_id,
        }
    }
}

impl Copy for Page { }
