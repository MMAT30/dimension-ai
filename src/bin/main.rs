fn main() {
    let inputs = vec![
        vec![1.0 , 2.0 , 3.0, 2.5],
        vec![2.0, 5.0, -1.0, 2.0],
        vec![-1.5, 2.7, 3.3, -0.8]
    ];
    // let weights = vec![
    //     vec![0.2 , 0.8, -0.5, 1.0],
    //     vec![0.5, -0.91, 0.26, -0.5],
    //     vec![-0.26, -0.27, 0.17, 0.87]
    // ];

    // let bias = vec![ 2.0, 3.0, 0.5 ];


    let n_inputs = inputs.t();
         println!("{:?}",inputs);
        println!("{:?}", n_inputs);
    // let mut output: Vec<f64> = Vec::new();
    
    // for (ws, bs) in weights.iter().zip(bias.iter()) {
    //     let mut nueron_output = 0.0;
    //     println!("{:?} {:?}", ws, bs);
    //     for (is, sws) in inputs.iter().zip(ws.iter()) {
    //         println!("{:?} {:?}", is, sws);
    //         nueron_output += is * sws;
    //     }
    //     output.push(nueron_output + bs);
    // }

    // println!("{:?}", output);
}




trait Transpose {
    fn t(&self) -> Vec<Vec<f64>>;
}

impl Transpose for Vec<Vec<f64>> {
    fn t(&self) -> Vec<Vec<f64>> {
        
        let cols = self[0].len();
        let rows = self.len();

        println!("{:?}", cols);
        println!("{:?}", rows);

        let mut transposed: Vec<Vec<f64>> = Vec::new();

        for x in 0..rows {
            // println!("{:?}", self[x]);
            let mut temp_vec: Vec<f64> = Vec::new();
            for y in 0..cols {
                println!("{:?}", self[x][y]);

                temp_vec.push(self[x][y]);
            }

            transposed.push(temp_vec);
        }
        transposed
    }
}