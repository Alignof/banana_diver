use std::collections::HashMap;

#[derive(Debug)]
pub struct LabelManager {
    labels: HashMap<String, String>,
    phandles: HashMap<String, u32>,
    current_phandle: u32,
}

impl LabelManager {
    pub fn new() -> Self {
        LabelManager {
            labels: HashMap::new(),
            phandles: HashMap::new(),
            current_phandle: 0,
        }
    }

    pub fn regist_label(&mut self, label: &str, data: String) {
        self.labels.insert(label.trim_start().to_string(), data);
    }

    pub fn regist_phandle(&mut self, label: &str) -> u32 {
        *self
            .phandles
            .entry(label.trim_start().to_string())
            .or_insert_with(|| {
                self.current_phandle += 1;
                self.current_phandle
            })
    }

    pub fn lookup(&self, label: &str) -> Option<String> {
        self.labels.get(label).cloned()
    }

    pub fn is_phandle_needed(&self, label_name: &str) -> Option<u32> {
        self.lookup(label_name)
            .as_ref()
            .and_then(|label| self.phandles.get(label))
            .copied()
    }
}
