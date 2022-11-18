use crate::components::consts::BUF_SIZE;


pub(crate) fn read_line<'b>(
    buffer: &'b [u8; BUF_SIZE],
    current_pos: &mut usize
) -> Option<&'b [u8]> {
    if *current_pos >= BUF_SIZE - 2 {return None}

    let sol = *current_pos;
    let mut eol = BUF_SIZE;

    for pos in *current_pos..BUF_SIZE {
        if buffer[pos]   == b'\r'  
        && buffer[pos+1] == b'\n' {
            *current_pos = pos + 2;
            eol = pos;
            break
        }
    }
    
    if eol == BUF_SIZE {None} else {Some(&buffer[sol..eol])}
}

// pub(crate) fn split_line_1(line: &[u8]) -> Option((&[u8], &[u8])) {
//     for byte in line {
//         
//     }
// }