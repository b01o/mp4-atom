use crate::*;

#[test]
fn esds() {
    const ENCODED: &[u8] = &[
        0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x36, 0x00, 0x00, 0x02,
        0x00, 0x69, 0x73, 0x6F, 0x36, 0x63, 0x6D, 0x66, 0x63, 0x6D, 0x70, 0x34, 0x31, 0x00, 0x00,
        0x04, 0xF0, 0x6D, 0x6F, 0x6F, 0x76, 0x00, 0x00, 0x00, 0x6C, 0x6D, 0x76, 0x68, 0x64, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xE8,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0xFE, 0x74, 0x72,
        0x61, 0x6B, 0x00, 0x00, 0x00, 0x5C, 0x74, 0x6B, 0x68, 0x64, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x02, 0xD0, 0x00, 0x00, 0x00, 0x00, 0x01, 0x9A, 0x6D, 0x64, 0x69, 0x61, 0x00, 0x00, 0x00,
        0x20, 0x6D, 0x64, 0x68, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0xC4, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x42, 0x68, 0x64, 0x6C, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x76, 0x69, 0x64, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x28, 0x43, 0x29, 0x20, 0x32, 0x30, 0x30, 0x37, 0x20, 0x47, 0x6F, 0x6F, 0x67, 0x6C,
        0x65, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x20, 0x76, 0x30, 0x38, 0x2E, 0x31, 0x33, 0x2E, 0x32,
        0x30, 0x30, 0x37, 0x2E, 0x00, 0x00, 0x00, 0x01, 0x30, 0x6D, 0x69, 0x6E, 0x66, 0x00, 0x00,
        0x00, 0x14, 0x76, 0x6D, 0x68, 0x64, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x64, 0x69, 0x6E, 0x66, 0x00, 0x00, 0x00, 0x1C,
        0x64, 0x72, 0x65, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x0C, 0x75, 0x72, 0x6C, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xF0, 0x73, 0x74,
        0x62, 0x6C, 0x00, 0x00, 0x00, 0xA4, 0x73, 0x74, 0x73, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x94, 0x61, 0x76, 0x63, 0x31, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0xD0, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x15, 0x4C, 0x61, 0x76, 0x63, 0x36, 0x30,
        0x2E, 0x33, 0x31, 0x2E, 0x31, 0x30, 0x32, 0x20, 0x6C, 0x69, 0x62, 0x78, 0x32, 0x36, 0x34,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0xFF, 0xFF, 0x00,
        0x00, 0x00, 0x2E, 0x61, 0x76, 0x63, 0x43, 0x01, 0x42, 0xC0, 0x1F, 0xFF, 0xE1, 0x00, 0x17,
        0x67, 0x42, 0xC0, 0x1F, 0xDA, 0x01, 0x40, 0x16, 0xEC, 0x04, 0x40, 0x00, 0x00, 0x03, 0x00,
        0x40, 0x00, 0x00, 0x0C, 0x23, 0xC6, 0x0C, 0xA8, 0x01, 0x00, 0x04, 0x68, 0xCE, 0x0F, 0xC8,
        0x00, 0x00, 0x00, 0x10, 0x70, 0x61, 0x73, 0x70, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x10, 0x73, 0x74, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x73, 0x74, 0x73, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x73, 0x74, 0x73, 0x7A, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x73, 0x74, 0x63,
        0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xD4, 0x74, 0x72,
        0x61, 0x6B, 0x00, 0x00, 0x00, 0x5C, 0x74, 0x6B, 0x68, 0x64, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x70, 0x6D, 0x64, 0x69, 0x61, 0x00, 0x00, 0x00,
        0x20, 0x6D, 0x64, 0x68, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x55, 0xC4, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x42, 0x68, 0x64, 0x6C, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x73, 0x6F, 0x75, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x28, 0x43, 0x29, 0x20, 0x32, 0x30, 0x30, 0x37, 0x20, 0x47, 0x6F, 0x6F, 0x67, 0x6C,
        0x65, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x20, 0x76, 0x30, 0x38, 0x2E, 0x31, 0x33, 0x2E, 0x32,
        0x30, 0x30, 0x37, 0x2E, 0x00, 0x00, 0x00, 0x01, 0x06, 0x6D, 0x69, 0x6E, 0x66, 0x00, 0x00,
        0x00, 0x10, 0x73, 0x6D, 0x68, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x24, 0x64, 0x69, 0x6E, 0x66, 0x00, 0x00, 0x00, 0x1C, 0x64, 0x72, 0x65, 0x66,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x75, 0x72, 0x6C,
        0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xCA, 0x73, 0x74, 0x62, 0x6C, 0x00, 0x00,
        0x00, 0x7E, 0x73, 0x74, 0x73, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x6E, 0x6D, 0x70, 0x34, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x00, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x65, 0x73, 0x64, 0x73, 0x00, 0x00,
        0x00, 0x00, 0x03, 0x80, 0x80, 0x80, 0x25, 0x00, 0x02, 0x00, 0x04, 0x80, 0x80, 0x80, 0x17,
        0x40, 0x15, 0x00, 0x00, 0x00, 0x00, 0x01, 0xF4, 0x00, 0x00, 0x01, 0xF4, 0x00, 0x05, 0x80,
        0x80, 0x80, 0x05, 0x12, 0x10, 0x56, 0xE5, 0x00, 0x06, 0x80, 0x80, 0x80, 0x01, 0x02, 0x00,
        0x00, 0x00, 0x14, 0x62, 0x74, 0x72, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xF4, 0x00,
        0x00, 0x01, 0xF4, 0x00, 0x00, 0x00, 0x00, 0x10, 0x73, 0x74, 0x74, 0x73, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x73, 0x74, 0x73, 0x63, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x73, 0x74, 0x73, 0x7A, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
        0x73, 0x74, 0x63, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0x6D, 0x76, 0x65, 0x78, 0x00, 0x00, 0x00, 0x20, 0x74, 0x72, 0x65, 0x78, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x74, 0x72, 0x65, 0x78,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x75, 0x64,
        0x74, 0x61, 0x00, 0x00, 0x00, 0x5A, 0x73, 0x6B, 0x69, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x6C, 0x6D, 0x6F, 0x6F, 0x66, 0x00, 0x00, 0x00, 0x10, 0x6D,
        0x66, 0x68, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x54,
        0x74, 0x72, 0x61, 0x66, 0x00, 0x00, 0x00, 0x20, 0x74, 0x66, 0x68, 0x64, 0x00, 0x02, 0x00,
        0x3A, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x0D, 0x31, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x74, 0x66, 0x64, 0x74, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18,
        0x74, 0x72, 0x75, 0x6E, 0x01, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x74, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x39,
    ];

    let mut buf = &mut std::io::Cursor::new(&ENCODED);

    let ftyp = Ftyp::decode(buf).expect("failed to decode ftyp");

    assert_eq!(
        ftyp,
        Ftyp {
            major_brand: b"iso6".into(),
            minor_version: 512,
            compatible_brands: vec![b"iso6".into(), b"cmfc".into(), b"mp41".into()],
        }
    );

    let moov = Moov::decode(buf).expect("failed to decode moov");
    assert_eq!(
        moov,
        Moov {
            mvhd: Mvhd {
                timescale: 1000,
                rate: 1.into(),
                volume: 1.into(),
                next_track_id: 2,
                ..Default::default()
            },
            mvex: Some(Mvex {
                trex: vec![
                    Trex {
                        track_id: 1,
                        default_sample_description_index: 1,
                        ..Default::default()
                    },
                    Trex {
                        track_id: 2,
                        default_sample_description_index: 1,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }),
            trak: vec![Trak {
                tkhd: Tkhd {
                    track_id: 1,
                    enabled: true,
                    width: 1280.into(),
                    height: 720.into(),
                    ..Default::default()
                },
                mdia: Mdia {
                    mdhd: Mdhd {
                        timescale: 12288,
                        language: "und".into(),
                        ..Default::default()
                    },
                    hdlr: Hdlr {
                        handler: b"vide".into(),
                        name: "(C) 2007 Google Inc. v08.13.2007.".into(),
                    },
                    minf: Minf {
                        smhd: None,
                        vmhd: Vmhd {
                            ..Default::default()
                        }
                        .into(),
                        dinf: Dinf {
                            dref: Dref {
                                urls: vec![Url {
                                    location: "".into(),
                                }],
                            },
                        },
                        stbl: Stbl {
                            stsd: Stsd {
                                avc1: Some(Avc1 {
                                    data_reference_index: 1,
                                    width: 1280,
                                    height: 720,
                                    horizresolution: 72.into(),
                                    vertresolution: 72.into(),
                                    frame_count: 1,
                                    compressor: "\u{15}Lavc60.31.102 libx264".into(),
                                    depth: 24,
                                    avcc: Avcc {
                                        configuration_version: 1,
                                        avc_profile_indication: 66,
                                        profile_compatibility: 192,
                                        avc_level_indication: 31,
                                        length_size: 4,
                                        sequence_parameter_sets: vec![b"gB\xc0\x1f\xda\x01@\x16\xec\x04@\0\0\x03\0@\0\0\x0c#\xc6\x0c\xa8".into()],
                                        picture_parameter_sets:  vec![b"h\xce\x0f\xc8".into()],
                                        ext: None,
                                    },
                                }),
                                ..Default::default()
                            },
                            stts: Stts {
                                ..Default::default()
                            },
                            stsc: Stsc {
                                ..Default::default()
                            },
                            stsz: Stsz {
                                ..Default::default()
                            },
                            stco: Some(Stco { ..Default::default() }),
                            ..Default::default()
                        },
                    },
                },
                ..Default::default()
            },
            Trak {
                tkhd: Tkhd {
                    track_id: 2,
                    alternate_group: 1,
                    enabled: true,
                    volume: 1.into(),
                    ..Default::default()
                },
                mdia: Mdia {
                    mdhd: Mdhd {
                        timescale: 44100,
                        language: "und".into(),
                        ..Default::default()
                    },
                    hdlr: Hdlr {
                        handler: b"soun".into(),
                        name: "(C) 2007 Google Inc. v08.13.2007.".into(),
                    },
                    minf: Minf {
                        smhd: Some(Smhd {
                            ..Default::default()
                        }),
                        dinf: Dinf {
                            dref: Dref {
                                urls: vec![Url {
                                    location: "".into(),
                                }],
                            },
                        },
                        stbl: Stbl {
                            stsd: Stsd {
                                mp4a: Some(Mp4a {
                                    data_reference_index: 1,
                                    channelcount: 2,
                                    samplesize: 16,
                                    samplerate: 44100.into(),
                                    esds: Some(Esds {
                                        es_desc: esds::EsDescriptor {
                                            es_id: 2,
                                            dec_config: esds::DecoderConfig{
                                                object_type_indication: 64,
                                                stream_type: 5,
                                                max_bitrate: 128000,
                                                avg_bitrate: 128000,
                                                dec_specific: esds::DecoderSpecific {
                                                    profile: 2,
                                                    freq_index: 4,
                                                    chan_conf: 2,
                                                },
                                                ..Default::default()
                                            },
                                            sl_config: esds::SLConfig{},
                                        },
                                    }),
                                }),
                                ..Default::default()
                            },
                            stts: Stts {
                                ..Default::default()
                            },
                            stsc: Stsc {
                                ..Default::default()
                            },
                            stsz: Stsz {
                                ..Default::default()
                            },
                            stco: Some(Stco { ..Default::default() }),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                ..Default::default()
            }],
            udta: Some(Udta {
                skip: Some(Skip {
                    zeroed: 82.into(),
                }),
                ..Default::default()
            }),

            ..Default::default()
        },
    );

    let moof = Moof::decode(&mut buf).expect("failed to decode moof");
    assert_eq!(
        moof,
        Moof {
            mfhd: Mfhd { sequence_number: 1 },
            traf: vec![Traf {
                tfhd: Tfhd {
                    track_id: 1,
                    sample_description_index: 1.into(),
                    default_sample_duration: 512.into(),
                    default_sample_flags: 0x1010000.into(),
                    default_sample_size: 3377.into(),
                    ..Default::default()
                },
                tfdt: Some(Tfdt {
                    ..Default::default()
                }),
                trun: Some(Trun {
                    data_offset: 116.into(),
                    entries: vec![TrunEntry {
                        flags: Some(33554432),
                        ..Default::default()
                    }],
                }),
            }],
        },
    );
}
