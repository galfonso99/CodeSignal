/* N candles are placed in a row, some of them are initially lit. For each candle from the 1st to the Nth the following algorithm is applied: if the observed candle is lit then states of this candle and all candles before it are changed to the opposite. Which candles will remain lit after applying the algorithm to all candles in the order they are placed in the line? */

fn switchLights(a: Vec<i32>) -> Vec<i32> {
    a.iter().scan(Vec::new(), |v, x| {
        if x == &1 {
            v.push(1);
            v.iter_mut().for_each(|x| *x=if x==&1 {0} else {1});  //Flip lits and unlits
        }
        else {v.push(0);}
        Some(v.clone())
    }).last().unwrap().to_vec()
}
