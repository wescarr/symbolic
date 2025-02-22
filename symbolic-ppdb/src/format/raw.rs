use watto::Pod;

/// Signature for physical metadata as specified by ECMA-335.
pub const METADATA_SIGNATURE: u32 = 0x424A_5342;

/// First part of the metadata header, as specified in the ECMA-335 spec, II.24.2.1.
///
/// This includes everything before the version string.
#[repr(C)]
#[derive(Debug)]
pub struct Header {
    /// The metadata signature.
    ///
    /// The value of this should be [`METADATA_SIGNATURE`].
    pub signature: u32,
    /// Major version, 1 (ignore on read).
    pub major_version: u16,
    /// Minor version, 1 (ignore on read).
    pub minor_version: u16,
    /// Reserved, always 0.
    pub _reserved: u32,
    /// Number of bytes allocated to hold version string.
    ///
    /// This is the actual length of the version string, including the
    /// null terminator, rounded up to a multiple of 4.
    pub version_length: u32,
}

/// Second part of the metadata header, as specified in the ECMA-335 spec, II.24.2.1.
///
/// This includes everything after the version string.
#[repr(C)]
#[derive(Debug)]
pub struct HeaderPart2 {
    /// Reserved, always 0.
    pub flags: u16,
    /// Number of streams.
    pub streams: u16,
}

/// A stream header, as specified in the ECMA-335 spec, II.24.2.2.
///
/// Does not contain the stream's name due to its variable length.
#[repr(C)]
#[derive(Debug)]
pub struct StreamHeader {
    /// Memory offset to start of this stream form start of the metadata root.
    pub offset: u32,
    /// Size of this stream in bytes.
    ///
    /// This should always be a multiple of 4.
    pub size: u32,
}

#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy)]
pub struct PdbStreamHeader {
    pub id: [u8; 20],
    pub entry_point: u32,
    pub referenced_tables: u64,
}

#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy)]
pub struct MetadataStreamHeader {
    pub _reserved: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub heap_sizes: u8,
    pub _reserved2: u8,
    pub valid_tables: u64,
    pub sorted_tables: u64,
}

unsafe impl Pod for Header {}
unsafe impl Pod for HeaderPart2 {}
unsafe impl Pod for StreamHeader {}
unsafe impl Pod for PdbStreamHeader {}
unsafe impl Pod for MetadataStreamHeader {}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_sizeof() {
        assert_eq!(mem::size_of::<Header>(), 16);
        assert_eq!(mem::align_of::<Header>(), 4);

        assert_eq!(mem::size_of::<HeaderPart2>(), 4);
        assert_eq!(mem::align_of::<HeaderPart2>(), 2);

        assert_eq!(mem::size_of::<StreamHeader>(), 8);
        assert_eq!(mem::align_of::<StreamHeader>(), 4);

        assert_eq!(mem::size_of::<PdbStreamHeader>(), 32);
        assert_eq!(mem::align_of::<PdbStreamHeader>(), 4);

        assert_eq!(mem::size_of::<MetadataStreamHeader>(), 24);
        assert_eq!(mem::align_of::<MetadataStreamHeader>(), 4);
    }
}
