const PAGE_SIZE: usize = 1024;

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

}