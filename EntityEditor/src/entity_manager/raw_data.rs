use std::collections::HashMap;

pub struct RawData {
    data : HashMap<String, String>,
}

pub struct RawDataBox
{
    datas : Vec<RawData>,
}

impl RawDataBox {
    pub fn new() -> Self {
        RawDataBox { datas: Vec::new() }
    }

    pub fn add_data(&mut self, data: RawData) {
        self.datas.push(data);
    }

    pub fn get_data(&self, index: usize) -> Option<&RawData> {
        self.datas.get(index)
    }
}

