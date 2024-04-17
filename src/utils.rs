pub struct Utils {
    data: Vec<String>,
}
pub struct UtilsCount {
    pub count: i32,
    pub am_count: i32,
}
impl Utils {
    pub fn new(data: Vec<String>) -> Utils {
        Utils { data }
    }
    pub fn get_count(&self) -> UtilsCount {
        let mut index = 0;
        let mut index_2 = 0;
        for key in self.data.iter() {
            if key.contains("true") {
                index = index + 1;
            }
            if key.contains("上午") {
                index_2 = index_2 + 1;
            }
        }
        UtilsCount {
            count: index,
            am_count: index_2,
        }
    }
    pub fn get_average_speed(&self) -> f32 {
        let mut index = self.get_count().count;
        let mut sum = 0.0;
        for key in self.data.iter() {
            if key.contains("true") {
                if let Ok(temp_speed) = key.split("-").collect::<Vec<&str>>()[2]
                    .replace(" m/s", "")
                    .parse::<f32>()
                {
                    sum = sum + temp_speed;
                } else {
                    index = index - 1;
                }
            }
        }
        format!("{:2}", sum / index as f32)
    }
}
