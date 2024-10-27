const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];

pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut min_cashes = vec![0; amount as usize + 1];
    // 动态收集从 1 到 amount 的最小找零纸币数
    // TODO: 这里写逻辑
    todo!()
}
