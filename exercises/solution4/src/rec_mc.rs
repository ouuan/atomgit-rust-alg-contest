const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];

pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut min_cashes = vec![0; amount as usize + 1];

    // 动态收集从 1 到 amount 的最小找零纸币数
    for denm in 1..=amount {
        // 此 min_cashe_num 等于全用 1 元纸币找零的纸币数
        let mut min_cashe_num = denm;
        for c in CASHES.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            // 加 1 是因为当前最小找零数等于上一最小找零数加 1 张 c 面额纸币
            let cashe_num = min_cashes[index] + 1;
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
            }
        }

        min_cashes[denm as usize] = min_cashe_num;
    }

    // 因为收集了所有的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}
