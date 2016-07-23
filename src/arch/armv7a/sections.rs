const SECTION_MAP_MAGIC: u32 = 0xD1CE0000;

use core::mem;
use core::str;

#[repr(packed)]
pub struct SectionMap {
    pub magic: u32,
    pub sections: u32,
    pub list: *mut SectionEntry,
}

#[repr(packed)]
pub struct SectionEntry {
    pub start: u32,
    pub end: u32,
    pub name: *mut [u8],
}

impl SectionMap {
    pub unsafe fn verify(&mut self) {
        if self.magic != SECTION_MAP_MAGIC {
            panic!("Section Map Magic value {}\n", self.magic);
        }

        debug!(self.dump());
    }

    pub unsafe fn dump(&mut self) {

        println!("Section Map:");
        println!("\tMagic:\t\t\t{:>#0width$X}", self.magic, width=10);
        println!("\tSections:\t\t{:}", self.sections);
        println!("\tList Pointer:\t\t{:>#0width$X}", self.list as usize , width=10);

        let mut itr = self.list;

        // Use pointer arithmetic to iterate through the list of SectionEntries since I'm not sure 
        // how rust does iterators. (I don't know the underlying memory).
        for i in 0..self.sections {
            (*itr).dump();
            itr = (itr as usize + 1 * mem::size_of::<SectionEntry>()) as *mut SectionEntry;
        }
    }
}

impl SectionEntry {
    pub unsafe fn dump(&self) {
        println!("\tSection Entry:");
        println!("\t\tName:\t\t{:}", str::from_utf8_unchecked(&*self.name));
        println!("\t\tStart:\t\t{:>#0width$X}", self.start, width=10);
        println!("\t\tEnd:\t\t{:>#0width$X}", self.end, width=10);
    }
}
