use core::slice;

const PAGE_SIZE: usize = 1024;
const SLOT_SIZE: usize = 8;
const PAGE_HEADER_SIZE: usize = 16;

#[repr(C)]
#[derive(Clone, Copy)]
struct Slot {
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

    pub fn allocate_slot(&mut self) -> &mut Slot {
        let header = self.header_mut();
        let next = header.slots_offset as usize + PAGE_HEADER_SIZE;
        header.slots_offset += SLOT_SIZE as i32;
        let p = self.bytes[next..next + SLOT_SIZE].as_mut_ptr() as *mut Slot;
        unsafe {
            &mut *p
        }
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
        assert_eq!(slot1.offset, 10);
        assert_eq!(slot1.length, 11);

        let slot2 = slots[1];
        assert_eq!(slot2.offset, 20);
        assert_eq!(slot2.length, 21);
    }

}