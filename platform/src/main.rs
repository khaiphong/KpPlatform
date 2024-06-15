use container;

fn main() {
    println!("contalner is the platform minimum OCI with required namespaces and cgroups");

    let num = 10;
    println!("Platform Hello, world! {num} plus one is {}!", container::add_one(num));
}

