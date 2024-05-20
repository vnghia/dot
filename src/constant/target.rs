mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const TARGET_TRIPLET: &str = built_info::TARGET;
