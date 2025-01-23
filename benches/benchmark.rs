use std::time::Duration;

use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration};
use rand::{rngs::StdRng, SeedableRng, RngCore};
use time_priority_order_algoritmh::*;


#[derive(Clone, Debug, PartialEq, Eq)]
struct Obj {
    start: i32,
    end: i32,
    priority: i32,
}

impl Timed<i32> for Obj {
    fn get_start(&self) -> i32 {
        self.start
    }
    fn get_end(&self) -> i32 {
        self.end
    }
    fn set_start(&mut self, time: i32) {
        self.start = time;
    }
    fn set_end(&mut self, time: i32) {
        self.end = time;
    }
}

impl PartialOrd for Obj {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

fn create_test_data(size: &usize) -> Vec<Obj> {
    let mut rng = StdRng::seed_from_u64(0);
    
    let mut data = Vec::with_capacity(*size);
    for _ in 0..*size {
        data.push(Obj {
            start: rng.next_u32() as i32,
            end: rng.next_u32() as i32,
            priority: rng.next_u32() as i32,
        });
    }
    data
}

const DEFAULT_TEST_SIZES: [usize; 5] = [
    10_usize,
    100_usize,
    1_000_usize,
    10_000_usize,
    100_000_usize
];


fn criterion_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("heavy workload");
    
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    group.measurement_time(Duration::new(30, 0));

    for test_size in DEFAULT_TEST_SIZES.iter() {
        group.bench_with_input(
            BenchmarkId::new("time_order_by_priority", test_size),
            test_size,
            |b, size| {
                b.iter_batched(
                    || {
                        create_test_data(size)
                    },
                    |vec| {
                        time_order_by_priority(vec);
                    }, criterion::BatchSize::SmallInput);
            },
        );
    }
    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
