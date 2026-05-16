use core::slice;

use thiserror::Error;

const PAGE_SIZE: usize = 1024;
const SLOT_SIZE: usize = 12;
const PAGE_HEADER_SIZE: usize = 16;

#[derive(Debug, Error)]
#[error("An error occurred")]
struct SomeError;

#[repr(C)]
#[derive(Clone, Copy)]
struct Slot {
    id: i32,
    offset: i32,
    length: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PageHeader {
    number_of_pages: i32,
    data_offset: i32,
    slots_offset: i32,
    page_id: i32,
}

#[repr(align(8))]
struct Page {
    bytes: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new(id: i32) -> Page {
        let mut page = Page {
            bytes: [0; PAGE_SIZE]
        };

        let header = page.header_mut();
        header.data_offset = PAGE_SIZE as i32;
        header.page_id = id;
        page
    }

    pub fn header_mut(&mut self) -> &mut PageHeader {
        let p = self.bytes.as_mut_ptr() as *mut PageHeader;
        unsafe {
            &mut *p
        }
    }

    fn allocate_slot(&mut self) -> &mut Slot {
        let header = self.header_mut();
        let next = header.slots_offset as usize + PAGE_HEADER_SIZE;
        let next_id = header.slots_offset as usize / SLOT_SIZE;
        header.slots_offset += SLOT_SIZE as i32;
        let p = self.bytes[next..next + SLOT_SIZE].as_mut_ptr() as *mut Slot;
        unsafe {
            let slot = &mut *p;
            slot.id = next_id as i32;
            slot
        }
    }

    fn size(&self) -> i32 {
        let header = self.header();
        let tuple_size = PAGE_SIZE as i32 - header.data_offset;
        PAGE_SIZE as i32 - PAGE_HEADER_SIZE as i32 - header.slots_offset - tuple_size
    }

    pub fn read_slot(&self, slot: &Slot) -> &[u8] {
        let from = slot.offset as usize;
        let to = (slot.offset + slot.length) as usize;
        &self.bytes[from..to]
    }

    pub fn insert_record(&mut self, bytes: Vec<u8>) -> Result<(), SomeError> {
        if bytes.len() + SLOT_SIZE > self.size() as usize {
            return Err(SomeError);
        }

        let header = self.header_mut();
        let end_of_data = header.data_offset as usize;
        header.data_offset -= bytes.len() as i32;

        let slot = self.allocate_slot();
        slot.length = bytes.len() as i32;
        slot.offset = (end_of_data - bytes.len()) as i32;
        
        self.bytes[end_of_data - bytes.len()..end_of_data].copy_from_slice(&bytes);

        Ok(())
    }

    pub fn slots(&self) -> &[Slot] {
        let p = self.bytes[PAGE_HEADER_SIZE..].as_ptr() as *const Slot;
        let len = self.header().slots_offset as usize / SLOT_SIZE;
        unsafe {
            slice::from_raw_parts(p, len)
        }
    }

    pub fn header(&self) -> &PageHeader {
        let p = self.bytes.as_ptr() as *mut PageHeader;
        unsafe {
            &*p
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::page_as_pointer::{PAGE_SIZE, Page};

    #[test]
    fn should_init_page_header_correctly() {
        let page = Page::new(5);
        let header = page.header();

        assert_eq!(header.data_offset, PAGE_SIZE as i32);
        assert_eq!(header.page_id, 5);
    }

    #[test]
    fn should_write_and_read_slots() {
        let mut page = Page::new(1);
        page.insert_record(vec![1, 2, 3]).unwrap();
        page.insert_record(vec![4, 5, 6]).unwrap();

        let slots = page.slots();

        assert_eq!(slots.len(), 2);
        let bytes_slot_0 = page.read_slot(&slots[0]);
        let bytes_slot_1 = page.read_slot(&slots[1]);

        assert_eq!(bytes_slot_0, &[1, 2, 3]);
        assert_eq!(bytes_slot_1, &[4, 5, 6]);
    }

    #[test]
    fn should_create_slots_and_can_read_slice() {
        let mut page = Page::new(1);

        let slots = page.slots();
        assert_eq!(slots.len(), 0);

        let slot = page.allocate_slot();
        slot.offset = 10;
        slot.length = 11;

        let slot = page.allocate_slot();
        slot.offset = 20;
        slot.length = 21;

        let slots = page.slots();
        assert_eq!(slots.len(), 2);

        let slot1 = slots[0];
        assert_eq!(slot1.id, 0);
        assert_eq!(slot1.offset, 10);
        assert_eq!(slot1.length, 11);

        let slot2 = slots[1];
        assert_eq!(slot2.id, 1);
        assert_eq!(slot2.offset, 20);
        assert_eq!(slot2.length, 21);
    }

}