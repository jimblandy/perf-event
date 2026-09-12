fn main() {
    let six = left::bindings6::perf_event_attr::default();
    let seven = right::bindings7::perf_event_attr::default();
    println!("{} {}", six.type_, seven.type_);
}
