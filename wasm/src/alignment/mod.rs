mod score;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Alignment {
    dnas: Vec<String>,
    score: score::Score,
}

impl Alignment {
    fn alignment (&self, x_dna: &str, y_dna: &str) -> (i128, Vec<i32>, Vec<i32>) {
        let mut res: Vec<Vec<i128>> = Vec::with_capacity(y_dna.len()+1);
        res.push(vec![0;x_dna.len()+1]);
        for i in 0..y_dna.len() {
            let mut result: Vec<i128> = Vec::with_capacity(x_dna.len()+1);
            result.push(0i128);
            let mut vector: Vec<i128>;
            for j in 0..x_dna.len() {
                vector = vec![res[i][j+1]+self.score.gapVal, res[i][j]+self.score.get(y_dna.chars().nth(i).unwrap(), x_dna.chars().nth(j).unwrap()), result[j]+self.score.gapVal];
                let val = vector.iter().max().unwrap();
                result.push(*val);
            }
        }

        let mut i = y_dna.len();
        let mut j = x_dna.len();

        let mut x_gaps: Vec<i32> = vec![0;x_dna.len()+1];
        let mut y_gaps: Vec<i32> = vec![0;y_dna.len()+1];

        while i > 0 || j > 0 {
            if i==0 {
                y_gaps[i] += 1;
                j -= 1;
                continue;
            }
            if j==0 {
                x_gaps[j] += 1;
                i -= 1;
                continue;
            }
            match res[i][j] {
                x if x == res[i][j-1] + self.score.gapVal => {
                    y_gaps[i] += 1;
                    j -= 1;
                    continue;
                },
                x if x == res[i-1][j-1]+self.score.get(y_dna.chars().nth(i).unwrap(), x_dna.chars().nth(j).unwrap()) => {
                    i -= 1;
                    j -= 1;
                    continue;
                },
                x if x ==  res[i][j-1]+self.score.gapVal => {
                    x_gaps[j] += 1;
                    continue;
                },
                _ => {}
            }
        }

        (res[i][j],x_gaps,y_gaps)
    }

    pub fn culc_star (&self) -> Vec<Vec<i128>> {
        let mut sims: Vec<Vec<(i128, Vec<i32>, Vec<i32>)>> = Vec::with_capacity(self.dnas.len());
        for i in 0..self.dnas.len() {
            let vector: Vec<(i128, Vec<i32>, Vec<i32>)> = vec![(0, vec![], vec![]);self.dnas.len()];
            sims.push(vector);
        }

        for i in 0..self.dnas.len() {
            for j in i..self.dnas.len() {
                if i == j {
                    sims[i][j] = (0, vec![], vec![]);
                } else {
                    let res_tuple = self.alignment( &self.dnas[i], &self.dnas[j]);
                    sims[i][j] = res_tuple;
                }
            }
        }

        let sums: Vec<i128> = sims.iter().map(|x| x.iter().map(|(x,_,_)| *x).sum()).collect();
        let (max_key,_) = sums.iter().enumerate().max_by(|(i_x,v_x),(i_y,v_y)| v_x.cmp(v_y) ).unwrap();

        let mut res: Vec<(Vec<i32>, Vec<i32>)> = Vec::with_capacity(self.dnas.len());
        for i in 0..self.dnas.len() {
            let mut x_gaps;
            let mut c_gaps;
            if i >= max_key {
                let val = sims.get(max_key).unwrap().get(i).unwrap();
                x_gaps = val.1.clone();
                c_gaps = val.2.clone();
            } else {
                let val = sims.get(i).unwrap().get(max_key).unwrap();
                x_gaps = val.2.clone();
                c_gaps = val.1.clone();
            }
            res.push((x_gaps, c_gaps));
        }

        let mut state: Vec<(usize,i32)> = vec![(0,0);self.dnas.len()];
        for i in 0..self.dnas[max_key].len()+1 {
            let mut c_gap_lens: Vec<i32> = Vec::with_capacity(self.dnas.len());
            for (_,c_gaps) in res.clone() {
                c_gap_lens.push(c_gaps[i])
            }
            let max_len = c_gap_lens.iter().max().unwrap();
            for j in 0..self.dnas.len() {
                res[j].0[state[j].0] += max_len - res[j].1[i];
                res[j].1[i] = *max_len;
                let mut diff = *max_len;
                loop {
                    if diff >= res[j].0[state[j].0] - state[j].1 {
                        diff -= res[j].0[state[j].0] - state[j].1;
                        state[j].0 += 1;
                        state[j].1 = 0;
                    } else {
                        state[j].1 += diff;
                    }
                }
            }
        }

        let mut sim_ress: Vec<Vec<i128>> = Vec::with_capacity(self.dnas.len());
        for i in 0..self.dnas.len() {
            let mut vector: Vec<i128> = vec![0;self.dnas.len()];
            sim_ress.push(vector);
        }
        for i in 0..self.dnas.len() {
            for j in i..self.dnas.len() {
                let mut x_i = 0;
                let mut x_j = 0;
                let mut y_i = 0;
                let mut y_j = 0;
                let mut score: i128 = 0;
                loop {
                    if x_i == res[i].0.len() {
                        break;
                    }
                    let mut x_val = '_';
                    let mut y_val = '_';
                    if x_j == res[i].0[x_i] {
                        x_val = self.dnas[i].chars().nth(x_i).unwrap();
                        x_i += 1;
                        x_j += 0;
                    } else {
                        x_val = '_';
                        x_j += 1;
                    }
                    if y_j == res[i].0[y_i] {
                        y_val = self.dnas[i].chars().nth(y_i).unwrap();
                        y_i += 1;
                        y_j += 0;
                    } else {
                        y_val = '_';
                        y_j += 1;
                    }

                    score += self.score.get(x_val,y_val);
                }
                sim_ress[i][j] = score;
                sim_ress[j][i] = score;
            }
        }

        sim_ress
    }
}