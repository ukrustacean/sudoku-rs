use rand::{distributions::Uniform, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

#[derive(Clone)]
pub struct Solver {
    sudoku: [[i32; 9]; 9],
    map: [[bool; 9]; 9],
    rng: StdRng,
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..9 {
            if y == 3 || y == 6 {
                writeln!(f, "---------------------")?;
            }

            for x in 0..9 {
                if x == 3 || x == 6 {
                    write!(f, "| ")?;
                }

                write!(f, "{} ", self.get(y, x))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for Solver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Solver {
            sudoku: Default::default(),
            map: Default::default(),
            // Seeding with the same number for reproducibility
            rng: rand::rngs::StdRng::seed_from_u64(0x9C80_D331_87F0_7473),
        };

        // Represnting sudoku in such a way so that squares 3x3 are rows
        for (i, c) in s.chars().filter(char::is_ascii_digit).enumerate() {
            let y = i / 9;
            let x = i % 9;
            *result.get_mut(y, x) = (c as i32) - ('0' as i32);
        }

        result.map = result.sudoku.map(|r| r.map(|n| n != 0));

        Ok(result)
    }
}

impl From<&[i32]> for Solver {
    fn from(value: &[i32]) -> Self {
        let mut result = Solver {
            sudoku: Default::default(),
            map: Default::default(),
            // Seeding with the same number for reproducibility
            rng: rand::rngs::StdRng::seed_from_u64(0x9C80_D331_87F0_7473),
        };

        // Represnting sudoku in such a way so that squares 3x3 are rows
        for (i, &v) in value.iter().enumerate() {
            let y = i / 9;
            let x = i % 9;
            *result.get_mut(y, x) = v as _;
        }

        result.map = result.sudoku.map(|r| r.map(|n| n != 0));

        result
    }
}

impl Solver {
    fn get(&self, y: usize, x: usize) -> i32 {
        let square = y / 3 * 3 + x / 3;
        let index = y % 3 * 3 + x % 3;

        self.sudoku[square][index]
    }

    fn get_mut(&mut self, y: usize, x: usize) -> &mut i32 {
        let square = y / 3 * 3 + x / 3;
        let index = y % 3 * 3 + x % 3;

        &mut self.sudoku[square][index]
    }

    fn get_error(&self) -> i32 {
        let mut error = 0;

        for y in 0..9 {
            let mut repetitions_row = [0; 10];
            for x in 0..9 {
                repetitions_row[self.get(y, x) as usize] += 1;
            }
            error += repetitions_row
                .iter()
                .filter(|&&x| x > 1)
                .map(|&x| x - 1)
                .sum::<i32>();
        }

        for x in 0..9 {
            let mut repetitions_col = [0; 10];
            for y in 0..9 {
                repetitions_col[self.get(y, x) as usize] += 1;
            }
            error += repetitions_col
                .iter()
                .filter(|&&x| x > 1)
                .map(|&x| x - 1)
                .sum::<i32>();
        }

        error
    }

    fn fill_random(&mut self) {
        for square in &mut self.sudoku {
            let mut avalaible_numbers = [true; 10];
            avalaible_numbers[0] = false;

            for i in square.iter() {
                avalaible_numbers[*i as usize] = false;
            }

            let mut nums = avalaible_numbers
                .into_iter()
                .enumerate()
                .filter_map(|(n, is_available)| if is_available { Some(n as i32) } else { None })
                .collect::<Vec<_>>();
            nums.shuffle(&mut self.rng);
            let mut nums = nums.into_iter();

            for x in square {
                if *x == 0 {
                    *x = nums.next().expect("There are enogh numbers to fill square");
                }
            }
        }
    }

    fn two_cells_from(&mut self, square: usize) -> (usize, usize) {
        let choices = (0..9).filter(|&n| !self.map[square][n]).collect::<Vec<_>>();

        let mut cells = (0, 0);
        let mut i = choices.choose_multiple(&mut self.rng, 2);

        cells.0 = i.next().copied().unwrap_or_default();
        cells.1 = i.next().copied().unwrap_or(cells.0);

        cells
    }

    fn swap_cells(&mut self, square: usize, cells: (usize, usize)) {
        self.sudoku[square].swap(cells.0, cells.1);
    }

    fn get_init_temp(&mut self) -> f64 {
        const N: f64 = 32.0;
        let mut population = vec![];

        for _ in 0..(N as usize) {
            population.push(self.try_new_state().2 as f64);
        }

        let mu = population.iter().sum::<f64>() / N;
        (population.iter().map(|&x| (x - mu).powi(2)).sum::<f64>() / N).sqrt()
    }

    fn get_num_of_iters(&self) -> i32 {
        self.map
            .iter()
            .flatten()
            .map(|&x| i32::from(!x))
            .sum::<i32>()
    }

    fn try_new_state(&mut self) -> (usize, (usize, usize), i32) {
        let square = rand_square(&mut self.rng);
        let cells = self.two_cells_from(square);

        self.swap_cells(square, cells);
        let error = self.get_error();
        self.swap_cells(square, cells);

        (square, cells, error)
    }

    #[cfg(not(feature = "error_cache"))]
    fn change_state(&mut self, temp: f64) -> i32 {
        let error = self.get_error();
        let (square, cells, new_error) = self.try_new_state();

        let diff = (error - new_error) as f64;

        let distr = Uniform::new_inclusive(0f64, 1.);

        if self.rng.sample(distr) < (diff / temp).exp() {
            self.swap_cells(square, cells);
            new_error
        } else {
            error
        }
    }

    #[cfg(feature = "error_cache")]
    fn change_state(&mut self, temp: f64, error: i32) -> i32 {
        let (square, cells, new_error) = self.try_new_state();

        let diff = (error - new_error) as f64;

        let distr = Uniform::new_inclusive(0f64, 1.);

        if self.rng.sample(distr) < (diff / temp).exp() {
            self.swap_cells(square, cells);
            new_error
        } else {
            error
        }
    }

    pub fn solve(&mut self) {
        let iterations = self.get_num_of_iters();
        let mut stuck = 0;
        self.fill_random();
        let mut temp = self.get_init_temp();
        let mut error = self.get_error();

        #[cfg(feature = "print_statistics")]
        let mut iteration_count = 0;
        #[cfg(feature = "print_statistics")]
        let mut reset_count = 0;

        if error <= 0 {
            println!("{self}");
            return;
        }

        'search: loop {
            let prev_err = error;
            // println!("{error}");

            for _ in 0..iterations {
                #[cfg(feature = "print_statistics")]
                {
                    iteration_count += 1;
                }

                #[cfg(not(feature = "error_cache"))]
                {
                    error = self.change_state(temp);
                }

                #[cfg(feature = "error_cache")]
                {
                    error = self.change_state(temp, error);
                }

                if error <= 0 {
                    break 'search;
                }
            }

            temp *= 0.99;
            stuck = if error >= prev_err { stuck + 1 } else { 0 };
            if stuck > 80 {
                temp += 2.;

                #[cfg(feature = "print_statistics")]
                {
                    reset_count += 1;
                }
            }
        }

        #[cfg(feature = "print_statistics")]
        {
            println!("Statistics");
            println!("Total iterations:  {iteration_count}");
            println!("Total resets:      {reset_count}");
            println!("Final temperature: {temp}");
            println!()
        }
    }
}

fn rand_square(rng: &mut impl Rng) -> usize {
    *[0usize, 1, 2, 3, 4, 5, 6, 7, 8].choose(rng).unwrap()
}
