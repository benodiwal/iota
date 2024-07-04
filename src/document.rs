use crate::Row;
use crate::FileType;
use crate::Position;
use crate::SearchDirection;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        let mut rows: Vec<Row> = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value))
        }
        Ok(Self { 
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type,
         })
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[at.y];        
    }

}