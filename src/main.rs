use learn_wgpu::run;

fn main() {
    tracing_subscriber::fmt::init();
    pollster::block_on(run());
}
