impl super::STATUS {
    pub const fn to_sysfs_file_name(&self) -> [&str; 2] {
        match self {
            Self::Current => ["current_link_speed", "current_link_width"],
            Self::Max => ["max_link_speed", "max_link_width"],
        }
    }
}
