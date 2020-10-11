use core::cmp::min;

pub fn u8_str_equal(a: &[u8], b: &[u8]) -> bool {
    let len = min(a.len(), b.len());

    for i in 0..len {
        if a[i] != b[i] {
            return false;
        }

        if a[i] == b'\0' {
            return true;
        }
    }

    return false;
}
