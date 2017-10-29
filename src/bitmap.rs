
mod bitmap {

    #[derive(Debug)]
    struct BitmapFileHeader {
        header: [i8; 2], // = {'B', 'M'},
        file_size: i32,
        reserved: i32,
        data_offset: i32
    }
}
