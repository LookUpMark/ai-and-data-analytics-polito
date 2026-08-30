use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::fmt;

/// Error returned when a circular buffer operation fails
#[derive(Debug, PartialEq)]
pub enum CircularBufferError {
    /// Error when trying to read from an empty buffer
    EmptyBuffer,
    /// Error when trying to write to a full buffer
    FullBuffer,
    /// Error when the buffer cannot be made contiguous
    NonContiguousBuffer,
}

impl fmt::Display for CircularBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircularBufferError::EmptyBuffer => write!(f, "Attempt to read from an empty buffer"),
            CircularBufferError::FullBuffer => write!(f, "Attempt to write to a full buffer"),
            CircularBufferError::NonContiguousBuffer => write!(f, "The buffer is not contiguous"),
        }
    }
}

/// Implementation of a generic circular buffer
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,  // read position
    tail: usize,  // write position
    size: usize,  // number of elements present
}

impl<T> CircularBuffer<T> {
    /// Creates a new circular buffer with the specified capacity
    pub fn new(capacity: usize) -> Self {
        // Create an empty vector with the specified capacity
        let buffer = Vec::with_capacity(capacity);
        // The vector should be initialized with null elements to access
        // the indices, but since T doesn't necessarily have a default value,
        // we leave the vector empty and fill it only when needed

        CircularBuffer {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    /// Returns the current size of the buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the total capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Checks if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Checks if the buffer is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Writes an element to the buffer
    /// Returns an error if the buffer is full
    pub fn write(&mut self, item: T) -> Result<(), CircularBufferError> {
        if self.is_full() {
            return Err(CircularBufferError::FullBuffer);
        }

        // If the buffer is empty, we need to initialize the vector
        if self.buffer.len() < self.capacity {
            self.buffer.push(item);
        } else {
            self.buffer[self.tail] = item;
        }

        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;

        Ok(())
    }

    /// Overwrites an element in the buffer even if it's full
    pub fn overwrite(&mut self, item: T) {
        if self.is_full() {
            // If the buffer is full, replace the oldest element (head)
            self.buffer[self.head] = item;
            self.head = (self.head + 1) % self.capacity;
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            // Otherwise, behave like write()
            if self.buffer.len() < self.capacity {
                self.buffer.push(item);
            } else {
                self.buffer[self.tail] = item;
            }
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;
        }
    }

    /// Reads an element from the buffer, removing it
    /// Returns an error if the buffer is empty
    pub fn read(&mut self) -> Result<T, CircularBufferError>
    where
        T: Clone,
    {
        if self.is_empty() {
            return Err(CircularBufferError::EmptyBuffer);
        }

        let item = self.buffer[self.head].clone();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;

        Ok(item)
    }

    /// Empties the buffer
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
        // We maintain the buffer capacity but empty it
        self.buffer.clear();
    }

    /// Makes the buffer contiguous by reorganizing the elements
    /// Returns true if the operation was successful
    pub fn make_contiguous(&mut self) -> bool
    where
        T: Clone,
    {        if self.is_empty() || self.is_contiguous() {
            // The buffer is already contiguous
            return true;
        }// Create a new temporary vector with the elements in order
        let mut temp = Vec::new();
        for i in 0..self.size {
            let idx = (self.head + i) % self.capacity;
            temp.push(self.buffer[idx].clone());
        }

        // Clear the buffer and rebuild it
        self.buffer.clear();
        
        // Add the contiguous elements
        for item in temp {
            self.buffer.push(item);
        }
        
        // Ensure the buffer has the right capacity by padding with clones of the last element
        // This is safe because we only access elements [0..size)
        if !self.buffer.is_empty() {
            let last_item = self.buffer[self.size - 1].clone();
            while self.buffer.len() < self.capacity {
                self.buffer.push(last_item.clone());
            }
        }

        self.head = 0;
        self.tail = self.size;

        true
    }
    
    /// Checks if the buffer is contiguous
    pub fn is_contiguous(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        
        // The buffer is contiguous in memory if:
        // 1. head < tail (normal case, elements in [head..tail))
        // 2. head == tail == 0 and buffer is full (special case after initialization)
        if self.head < self.tail {
            true
        } else if self.head == self.tail {
            // Only contiguous if we have a full buffer starting from position 0
            self.is_full() && self.head == 0
        } else {
            false
        }
    }
}

/// Implementation of Index to access elements via index
impl<T> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Index out of buffer bounds");
        }

        let real_index = (self.head + index) % self.capacity;
        &self.buffer[real_index]
    }
}

/// Implementation of IndexMut to modify elements via index
impl<T> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!("Index out of buffer bounds");
        }

        let real_index = (self.head + index) % self.capacity;
        &mut self.buffer[real_index]
    }
}

/// TryDeref trait as a safe version of Deref
pub trait TryDeref {
    type Target: ?Sized;
    type Error;

    fn try_deref(&self) -> Result<&Self::Target, Self::Error>;
}

/// Implementation of Deref to convert the buffer to a slice
impl<T> Deref for CircularBuffer<T> {
    type Target = [T];
    
    fn deref(&self) -> &Self::Target {
        if !self.is_contiguous() {
            panic!("Attempt to dereference a non-contiguous buffer");
        }

        if self.is_empty() {
            return &[];
        }

        // Handle special case where buffer is full and head == tail == 0
        if self.head == self.tail && self.is_full() && self.head == 0 {
            return &self.buffer[0..self.capacity];
        }

        &self.buffer[self.head..self.tail]
    }
}

/// Implementation of DerefMut to modify the buffer as a slice
impl<T> DerefMut for CircularBuffer<T> {    
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.is_contiguous() {
            panic!("Attempt to dereference a non-contiguous buffer");
        }

        if self.is_empty() {
            return &mut [];
        }

        // Handle special case where buffer is full and head == tail == 0
        if self.head == self.tail && self.is_full() && self.head == 0 {
            return &mut self.buffer[0..self.capacity];
        }

        &mut self.buffer[self.head..self.tail]
    }
}

/// Implementation of TryDeref for CircularBuffer
impl<T> TryDeref for CircularBuffer<T> {
    type Target = [T];
    type Error = CircularBufferError;
    
    fn try_deref(&self) -> Result<&Self::Target, Self::Error> {
        if !self.is_contiguous() {
            return Err(CircularBufferError::NonContiguousBuffer);
        }

        if self.is_empty() {
            return Ok(&[]);
        }

        // Handle special case where buffer is full and head == tail == 0
        if self.head == self.tail && self.is_full() && self.head == 0 {
            return Ok(&self.buffer[0..self.capacity]);
        }

        Ok(&self.buffer[self.head..self.tail])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_read() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        assert_eq!(buffer.size(), 0);
        assert_eq!(buffer.capacity(), 3);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());

        // Write elements
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();

        assert_eq!(buffer.size(), 2);
        assert!(!buffer.is_empty());
        assert!(!buffer.is_full());

        // Read elements
        assert_eq!(buffer.read().unwrap(), 1);
        assert_eq!(buffer.size(), 1);
        assert_eq!(buffer.read().unwrap(), 2);
        assert_eq!(buffer.size(), 0);
        assert!(buffer.is_empty());

        // Read from empty buffer
        assert_eq!(buffer.read(), Err(CircularBufferError::EmptyBuffer));
    }

    #[test]
    fn test_overwrite() {
        let mut buffer = CircularBuffer::<i32>::new(2);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        assert!(buffer.is_full());

        // Attempt to write to a full buffer
        assert_eq!(buffer.write(3), Err(CircularBufferError::FullBuffer));

        // Overwrite
        buffer.overwrite(3);
        assert_eq!(buffer.read().unwrap(), 2);
        assert_eq!(buffer.read().unwrap(), 3);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_circular_behavior() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        assert!(buffer.is_full());

        // Read an element to make space
        assert_eq!(buffer.read().unwrap(), 1);

        // Write a new element (should go in circular position)
        buffer.write(4).unwrap();

        // Read all remaining elements
        assert_eq!(buffer.read().unwrap(), 2);
        assert_eq!(buffer.read().unwrap(), 3);
        assert_eq!(buffer.read().unwrap(), 4);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        assert_eq!(buffer.size(), 2);

        buffer.clear();
        assert_eq!(buffer.size(), 0);
        assert!(buffer.is_empty());
        assert_eq!(buffer.read(), Err(CircularBufferError::EmptyBuffer));
    }

    #[test]
    fn test_make_contiguous() {
        let mut buffer = CircularBuffer::<i32>::new(5);

        // Fill the buffer
        for i in 1..=5 {
            buffer.write(i).unwrap();
        }

        // Read some elements
        assert_eq!(buffer.read().unwrap(), 1);
        assert_eq!(buffer.read().unwrap(), 2);

        // Write more elements (the buffer is now non-contiguous)
        buffer.write(6).unwrap();
        buffer.write(7).unwrap();

        // Make the buffer contiguous
        assert!(buffer.make_contiguous());

        // Verify that the elements are in the correct order
        assert_eq!(buffer.read().unwrap(), 3);
        assert_eq!(buffer.read().unwrap(), 4);
        assert_eq!(buffer.read().unwrap(), 5);
        assert_eq!(buffer.read().unwrap(), 6);
        assert_eq!(buffer.read().unwrap(), 7);
    }

    #[test]
    fn test_index() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(10).unwrap();
        buffer.write(20).unwrap();
        buffer.write(30).unwrap();

        assert_eq!(buffer[0], 10);
        assert_eq!(buffer[1], 20);
        assert_eq!(buffer[2], 30);

        // Modify an element using IndexMut
        buffer[1] = 25;

        assert_eq!(buffer[1], 25);
    }

    #[test]
    fn test_deref() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(10).unwrap();
        buffer.write(20).unwrap();

        // Test Deref
        let slice: &[i32] = &buffer;
        assert_eq!(slice, &[10, 20]);

        // Test DerefMut
        let slice_mut: &mut [i32] = &mut buffer;
        slice_mut[0] = 15;

        assert_eq!(buffer[0], 15);
    }

    #[test]
    fn test_try_deref() {
        let mut buffer = CircularBuffer::<i32>::new(5);

        // Fill the buffer
        for i in 1..=5 {
            buffer.write(i).unwrap();
        }

        // The buffer is contiguous
        assert!(buffer.is_contiguous());
        assert!(buffer.try_deref().is_ok());

        // Read some elements
        assert_eq!(buffer.read().unwrap(), 1);
        assert_eq!(buffer.read().unwrap(), 2);

        // Write more elements (the buffer is now non-contiguous)
        buffer.write(6).unwrap();
        buffer.write(7).unwrap();

        // The buffer is no longer contiguous
        assert!(!buffer.is_contiguous());
        assert_eq!(buffer.try_deref(), Err(CircularBufferError::NonContiguousBuffer));    }

    #[test]
    fn test_borrow_checker_protection() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(10).unwrap();
        buffer.write(20).unwrap();

        // Test that we can't modify the buffer while it's dereferenced
        // This demonstrates the borrow checker's protection
        {
            let slice: &[i32] = &buffer;
            assert_eq!(slice, &[10, 20]);
            
            // The following line would cause a compile error if uncommented:
            // buffer.write(30).unwrap(); // Error: cannot borrow `buffer` as mutable
        }
        
        // After the immutable borrow ends, we can modify again
        buffer.write(30).unwrap();
        assert_eq!(buffer.size(), 3);
    }

    #[test]
    fn test_deref_mut_protection() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        buffer.write(10).unwrap();
        buffer.write(20).unwrap();

        // Test mutable dereferencing
        {
            let slice: &mut [i32] = &mut buffer;
            slice[0] = 15;
            
            // The following would cause compile errors if uncommented:
            // buffer.write(30).unwrap(); // Error: cannot borrow `buffer` as mutable
            // let other_slice = &buffer; // Error: cannot borrow as immutable
        }
        
        assert_eq!(buffer[0], 15);
    }

    #[test]
    fn test_non_contiguous_deref_panic() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        // Fill and then make non-contiguous
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        
        // Read one element (makes it non-contiguous)
        buffer.read().unwrap();
        
        // Write one more element 
        buffer.write(4).unwrap();
        
        // Now the buffer is non-contiguous, deref should panic
        assert!(!buffer.is_contiguous());
        
        // This would panic if called:
        // let _slice: &[i32] = &buffer; // Would panic!
        
        // But try_deref should return an error
        assert_eq!(buffer.try_deref(), Err(CircularBufferError::NonContiguousBuffer));
    }

    #[test]
    fn test_comprehensive_circular_buffer() {
        let mut buffer = CircularBuffer::<String>::new(4);

        // Test with String type to ensure it works with non-Copy types
        buffer.write("first".to_string()).unwrap();
        buffer.write("second".to_string()).unwrap();
        assert_eq!(buffer.size(), 2);
        assert!(buffer.is_contiguous());

        // Test Index trait
        assert_eq!(buffer[0], "first");
        assert_eq!(buffer[1], "second");

        // Test IndexMut trait
        buffer[1] = "modified".to_string();
        assert_eq!(buffer[1], "modified");

        // Test Deref trait
        let slice: &[String] = &buffer;
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], "first");
        assert_eq!(slice[1], "modified");

        // Fill the buffer to capacity
        buffer.write("third".to_string()).unwrap();
        buffer.write("fourth".to_string()).unwrap();
        assert!(buffer.is_full());
        assert!(buffer.is_contiguous());

        // Test reading to create a wrap-around situation
        assert_eq!(buffer.read().unwrap(), "first");
        assert_eq!(buffer.read().unwrap(), "modified");
        assert!(!buffer.is_contiguous()); // Now it's non-contiguous

        // Test TryDeref on non-contiguous buffer
        assert_eq!(buffer.try_deref(), Err(CircularBufferError::NonContiguousBuffer));

        // Write more elements to wrap around
        buffer.write("fifth".to_string()).unwrap();
        buffer.write("sixth".to_string()).unwrap();
        assert!(buffer.is_full());
        assert!(!buffer.is_contiguous()); // Still non-contiguous        // Test make_contiguous
        assert!(buffer.make_contiguous());
        assert!(buffer.is_contiguous());

        // Now TryDeref should work
        let result = buffer.try_deref().unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "third");
        assert_eq!(result[1], "fourth");
        assert_eq!(result[2], "fifth");
        assert_eq!(result[3], "sixth");

        // Test overwrite functionality
        buffer.overwrite("seventh".to_string());
        assert_eq!(buffer.read().unwrap(), "fourth"); // "third" was overwritten

        // Test clear
        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.size(), 0);
        assert!(buffer.is_contiguous()); // Empty buffer is contiguous
    }

    #[test]
    #[should_panic(expected = "Attempt to dereference a non-contiguous buffer")]
    fn test_deref_panic_on_non_contiguous() {
        let mut buffer = CircularBuffer::<i32>::new(3);

        // Fill the buffer
        buffer.write(1).unwrap();
        buffer.write(2).unwrap(); 
        buffer.write(3).unwrap();

        // Read one element to make it non-contiguous
        buffer.read().unwrap();

        // Write one more element 
        buffer.write(4).unwrap();

        // This should panic because the buffer is non-contiguous
        let _slice: &[i32] = &buffer;
    }
}
