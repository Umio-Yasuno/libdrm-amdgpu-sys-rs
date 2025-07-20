use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DpmClockType {
    SCLK,
    MCLK,
    FCLK,
    VCLK,
    VCLK1,
    DCLK,
    DCLK1,
}

impl DpmClockType {
    const fn sysfs_name(&self) -> &str {
        match self {
            Self::SCLK => "pp_dpm_sclk",
            Self::MCLK => "pp_dpm_mclk",
            Self::FCLK => "pp_dpm_fclk",
            Self::VCLK => "pp_dpm_vclk",
            Self::VCLK1 => "pp_dpm_vclk1",
            Self::DCLK => "pp_dpm_dclk",
            Self::DCLK1 => "pp_dpm_dclk1",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DpmClockRange {
    pub clk_type: DpmClockType,
    pub current_mhz: u32,
    pub min_mhz: u32,
    pub max_mhz: u32,
}

impl DpmClockRange {
    fn parse_mhz(s: &str) -> Option<u32> {
        let mut chars = s.chars();
        let mhz_pos = chars.position(|c| c.is_ascii_alphabetic())?;
        let s = s.get(3..mhz_pos)?;
        s.parse::<u32>().ok()
    }

    fn parse_fine_grained_dpm(clk_type: DpmClockType, lines: Vec<&str>) -> Option<Self> {
        let mut clks = [0u32; 3];
        let mut cur_index = 0usize;

        for (i, (l, clk)) in lines.iter().zip(clks.iter_mut()).enumerate() {
            if l.ends_with("*") {
                cur_index = i;
            }
            let u = Self::parse_mhz(l)?;
            *clk = u;
        }

        let last_mhz = Self::parse_mhz(lines.last()?)?;

        Some(Self {
            clk_type,
            current_mhz: clks[cur_index],
            min_mhz: std::cmp::min(clks[0], last_mhz),
            max_mhz: std::cmp::max(clks[0], last_mhz),
        })
    }

    pub fn from_sysfs<P: Into<PathBuf>>(clk_type: DpmClockType, sysfs: P) -> Option<Self> {
        let sysfs = sysfs.into();
        let path = sysfs.join(clk_type.sysfs_name());
        let s = std::fs::read_to_string(&path).ok()?;
        let lines: Vec<&str> = s.lines().collect();
        let len = lines.len();

        if len == 2 || len == 3 {
            return Self::parse_fine_grained_dpm(clk_type, lines);
        }
        
        let [first, last] = [lines.first()?, lines.last()?].map(|s| Self::parse_mhz(s));
        let [first, last] = [first?, last?];

        let current_mhz = lines
            .iter()
            .find(|&s| s.ends_with("*"))
            .and_then(|s| Self::parse_mhz(s))
            .unwrap_or(last);

        Some(Self {
            clk_type,
            current_mhz,
            min_mhz: std::cmp::min(first, last),
            max_mhz: std::cmp::max(first, last),
        })
    }
}

pub(crate) fn get_min_max_from_dpm<
    T: std::cmp::Ord + std::marker::Copy,
    P: Into<PathBuf>
>(
    sysfs_path: P,
    parse: fn(&str) -> Option<T>,
) -> Option<[T; 2]> {
    let sysfs_path = sysfs_path.into();
    let s = std::fs::read_to_string(sysfs_path).ok()?;
    let mut lines = s.lines();

    let first = parse(lines.next()?)?;
    let last = match lines.last() {
        Some(last) => parse(last)?,
        None => return Some([first; 2]),
    };

    Some([
        std::cmp::min(first, last),
        std::cmp::max(first, last),
    ])
}
