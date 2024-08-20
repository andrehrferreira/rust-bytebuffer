use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::str;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone)]
pub struct ByteBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl ByteBuffer {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        ByteBuffer {
            buffer: data.unwrap_or_default(),
            position: 0,
        }
    }

    pub fn ensure_capacity(&mut self, required_bytes: usize) {
        let required_capacity = self.position + required_bytes;

        if required_capacity > self.buffer.len() {
            self.buffer.resize(required_capacity, 0);
        }
    }

    pub fn put_int32(&mut self, value: i32) -> &mut Self {
        self.ensure_capacity(4);
        let mut cursor = Cursor::new(&mut self.buffer[self.position..self.position + 4]);
        cursor.write_i32::<LittleEndian>(value).unwrap();
        self.position += 4;
        self
    }

    pub fn get_int32(&mut self) -> Result<i32, String> {
        if self.position + 4 > self.buffer.len() {
            return Err("Buffer underflow".to_string());
        }
        let mut cursor = Cursor::new(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(cursor.read_i32::<LittleEndian>().unwrap())
    }

    pub fn put_uint32(&mut self, value: u32) -> &mut Self {
        self.ensure_capacity(4);
        let mut cursor = Cursor::new(&mut self.buffer[self.position..self.position + 4]);
        cursor.write_u32::<LittleEndian>(value).unwrap();
        self.position += 4;
        self
    }

    pub fn get_uint32(&mut self) -> Result<u32, String> {
        if self.position + 4 > self.buffer.len() {
            return Err("Buffer underflow".to_string());
        }
        let mut cursor = Cursor::new(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(cursor.read_u32::<LittleEndian>().unwrap())
    }

    pub fn put_byte(&mut self, value: u8) -> &mut Self {
        self.ensure_capacity(1);
        self.buffer[self.position] = value;
        self.position += 1;
        self
    }

    pub fn get_byte(&mut self) -> Result<u8, String> {
        if self.position + 1 > self.buffer.len() {
            return Err("Buffer underflow".to_string());
        }
        let value = self.buffer[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn put_bool(&mut self, value: bool) -> &mut Self {
        self.put_byte(if value { 1 } else { 0 })
    }

    pub fn get_bool(&mut self) -> Result<bool, String> {
        Ok(self.get_byte()? != 0)
    }

    pub fn put_string(&mut self, value: &str) -> &mut Self {
        let bytes = value.as_bytes();
        self.put_int32(bytes.len() as i32);
        self.ensure_capacity(bytes.len());
        for &byte in bytes {
            self.put_byte(byte);
        }
        self
    }

    pub fn get_string(&mut self) -> Result<String, String> {
        let length = self.get_int32()? as usize;
        if self.position + length > self.buffer.len() {
            return Err("Buffer underflow".to_string());
        }
        let value = str::from_utf8(&self.buffer[self.position..self.position + length])
            .map_err(|_| "Invalid UTF-8 string".to_string())?
            .to_string();
        self.position += length;
        Ok(value)
    }

    pub fn put_float(&mut self, value: f32) -> &mut Self {
        self.ensure_capacity(4);
        let mut cursor = Cursor::new(&mut self.buffer[self.position..self.position + 4]);
        cursor.write_f32::<LittleEndian>(value).unwrap();
        self.position += 4;
        self
    }

    pub fn get_float(&mut self) -> Result<f32, String> {
        if self.position + 4 > self.buffer.len() {
            return Err("Buffer underflow".to_string());
        }
        let mut cursor = Cursor::new(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(cursor.read_f32::<LittleEndian>().unwrap())
    }

    pub fn put_vector(&mut self, vector: (f32, f32, f32)) -> &mut Self {
        self.put_float(vector.0);
        self.put_float(vector.1);
        self.put_float(vector.2);
        self
    }

    pub fn get_vector(&mut self) -> Result<(f32, f32, f32), String> {
        Ok((self.get_float()?, self.get_float()?, self.get_float()?))
    }

    pub fn put_rotator(&mut self, rotator: (f32, f32, f32)) -> &mut Self {
        self.put_float(rotator.0);
        self.put_float(rotator.1);
        self.put_float(rotator.2);
        self
    }

    pub fn get_rotator(&mut self) -> Result<(f32, f32, f32), String> {
        Ok((self.get_float()?, self.get_float()?, self.get_float()?))
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn to_hex(&self) -> String {
        self.buffer.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get_int32() {
        let mut buffer = ByteBuffer::new(None);
        buffer.put_int32(12345);
        buffer.position = 0; // Reset position for reading

        let value = buffer.get_int32().unwrap(); // Desembrulha o Result
        assert_eq!(value, 12345);
    }

    #[test]
    fn test_put_and_get_uint32() {
        let mut buffer = ByteBuffer::new(None);
        buffer.put_uint32(98765);
        buffer.position = 0;

        let value = buffer.get_uint32().unwrap(); // Desembrulha o Result
        assert_eq!(value, 98765);
    }

    #[test]
    fn test_put_and_get_bool() {
        let mut buffer = ByteBuffer::new(None);
        buffer.put_bool(true);
        buffer.put_bool(false);
        buffer.position = 0;

        let value1 = buffer.get_bool().unwrap();
        let value2 = buffer.get_bool().unwrap();
        assert_eq!(value1, true);
        assert_eq!(value2, false);
    }

    #[test]
    fn test_put_and_get_string() {
        let mut buffer = ByteBuffer::new(None);
        let test_string = String::from("Hello, Rust!");
        buffer.put_string(&test_string);
        buffer.position = 0;

        let value = buffer.get_string().unwrap();
        assert_eq!(value, test_string);
    }

    #[test]
    fn test_put_and_get_vector() {
        let mut buffer = ByteBuffer::new(None);
        let vector = (1.0, 2.0, 3.0);
        buffer.put_vector(vector);
        buffer.position = 0;

        let value = buffer.get_vector().unwrap();
        assert_eq!(value, vector);
    }

    #[test]
    fn test_put_and_get_rotator() {
        let mut buffer = ByteBuffer::new(None);
        let rotator = (45.0, 90.0, 180.0);
        buffer.put_rotator(rotator);
        buffer.position = 0;

        let value = buffer.get_rotator().unwrap();
        assert_eq!(value, rotator);
    }

    #[test]
    fn test_to_hex() {
        let mut buffer = ByteBuffer::new(None);
        buffer.put_int32(12345);
        let hex_string = buffer.to_hex();
        
        // Verificar se a string hexadecimal está correta
        assert_eq!(hex_string, "39300000");
    }
}
