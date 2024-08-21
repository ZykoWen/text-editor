use std::cmp;
use std::ops::Range;

pub struct Line {
    string: String,
}
impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }
    pub fn get(&self, range: Range<usize>) -> String {
        //如果line.get(start..end)的参数超过line的最有边界，依旧返回完整的line，但是如果直接对String数据类型的数据操作，会返回None
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
