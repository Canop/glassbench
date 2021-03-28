use {
    lettersorter::sort,
    glassbench::*,
};

static SMALL_NUMBERS: &[&str] = &[
    "0.123456789",
    "42",
    "-6",
    "π/2",
    "e²",
];

static BIG_NUMBERS: &[&str] = &[
    "424568",
    "45865452*44574*778141*78999",
    "same but even bigger",
    "42!",
    "infinite",
];

fn bench_number_sorting(bench: &mut Bench) {
    bench.task("small numbers", |task| {
        task.iter(|| {
            for n in SMALL_NUMBERS {
                pretend_used(sort(n));
            }
        });
    });
    bench.task("big numbers", |task| {
        task.iter(|| {
            for n in BIG_NUMBERS {
                pretend_used(sort(n));
            }
        });
    });
}

glassbench!(
    "Number Sorting",
    bench_number_sorting,
);

