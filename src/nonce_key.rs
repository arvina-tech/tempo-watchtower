const GROUP_NONCE_MAGIC: [u8; 4] = *b"NKG1";
const GROUP_NONCE_VERSION: u8 = 0x01;
const GROUP_NONCE_FLAG_MASK: u16 = 0x003F;

pub fn is_group_nonce_key(bytes: &[u8]) -> bool {
    if bytes.len() != 32 {
        return false;
    }
    if bytes[..4] != GROUP_NONCE_MAGIC {
        return false;
    }
    if bytes[4] != GROUP_NONCE_VERSION {
        return false;
    }
    let flags = u16::from_be_bytes([bytes[6], bytes[7]]);
    if flags & !GROUP_NONCE_FLAG_MASK != 0 {
        return false;
    }
    let scope_encoding = flags & 0b11;
    let group_encoding = (flags >> 2) & 0b11;
    let memo_encoding = (flags >> 4) & 0b11;
    if scope_encoding > 1 || group_encoding > 1 || memo_encoding > 1 {
        return false;
    }
    if scope_encoding == 1 && !is_ascii_field(&bytes[8..16]) {
        return false;
    }
    if group_encoding == 1 && !is_ascii_field(&bytes[16..20]) {
        return false;
    }
    if memo_encoding == 1 && !is_ascii_field(&bytes[20..32]) {
        return false;
    }
    true
}

fn is_ascii_field(bytes: &[u8]) -> bool {
    let mut zero_seen = false;
    for &byte in bytes {
        if byte == 0 {
            zero_seen = true;
            continue;
        }
        if zero_seen {
            return false;
        }
        if !(0x20..=0x7E).contains(&byte) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{GROUP_NONCE_MAGIC, GROUP_NONCE_VERSION, is_group_nonce_key};

    fn build_key(flags: u16, scope: [u8; 8], group: [u8; 4], memo: [u8; 12]) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[..4].copy_from_slice(&GROUP_NONCE_MAGIC);
        bytes[4] = GROUP_NONCE_VERSION;
        bytes[6..8].copy_from_slice(&flags.to_be_bytes());
        bytes[8..16].copy_from_slice(&scope);
        bytes[16..20].copy_from_slice(&group);
        bytes[20..32].copy_from_slice(&memo);
        bytes
    }

    fn padded_ascii<const N: usize>(value: &str) -> [u8; N] {
        let bytes = value.as_bytes();
        assert!(bytes.len() <= N);
        let mut out = [0u8; N];
        out[..bytes.len()].copy_from_slice(bytes);
        out
    }

    #[test]
    fn accepts_numeric_format() {
        let key = build_key(0, [0u8; 8], [0u8; 4], [0u8; 12]);
        assert!(is_group_nonce_key(&key));
    }

    #[test]
    fn accepts_ascii_format() {
        let flags = 0b01 | (0b01 << 2) | (0b01 << 4);
        let key = build_key(
            flags,
            padded_ascii("PAYROLL"),
            padded_ascii("G1"),
            padded_ascii("JAN-2026"),
        );
        assert!(is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_wrong_length() {
        let bytes = [0u8; 31];
        assert!(!is_group_nonce_key(&bytes));
    }

    #[test]
    fn rejects_wrong_magic() {
        let mut key = build_key(0, [0u8; 8], [0u8; 4], [0u8; 12]);
        key[0] = 0x00;
        assert!(!is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_wrong_version() {
        let mut key = build_key(0, [0u8; 8], [0u8; 4], [0u8; 12]);
        key[4] = 0x02;
        assert!(!is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_reserved_bits() {
        let key = build_key(0x0040, [0u8; 8], [0u8; 4], [0u8; 12]);
        assert!(!is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_reserved_encodings() {
        let flags = 0b10 | (0b11 << 2) | (0b10 << 4);
        let key = build_key(flags, [0u8; 8], [0u8; 4], [0u8; 12]);
        assert!(!is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_non_printable_ascii() {
        let flags = 0b01 | (0b01 << 2) | (0b01 << 4);
        let mut memo = [0u8; 12];
        memo[0] = b'H';
        memo[1] = 0x19;
        let key = build_key(flags, padded_ascii("SCOPE"), padded_ascii("G1"), memo);
        assert!(!is_group_nonce_key(&key));
    }

    #[test]
    fn rejects_ascii_with_embedded_zero() {
        let flags = 0b01;
        let mut scope = [0u8; 8];
        scope[0] = b'A';
        scope[1] = 0;
        scope[2] = b'B';
        let key = build_key(flags, scope, [0u8; 4], [0u8; 12]);
        assert!(!is_group_nonce_key(&key));
    }
}
