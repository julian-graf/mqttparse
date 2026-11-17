fn main() {
    {
        let server_parser = mqttparse::v5::Parser::<mqttparse::v5::Client>::new();

        let packet = &[
            0x32, 0x14, 0x00, // Topic Name
            0x0A, //
            b't', //
            b'e', //
            b's', //
            b't', //
            b'/', //
            b't', //
            b'o', //
            b'p', //
            b'i', //
            b'c', // Topic Name
            0x17, // Packet identifier
            0x09, // Packet identifier
            0x00, // Property length
            b'h', // Payload
            b'e', //
            b'l', //
            b'l', //
            b'o', // Payload
        ];

        let header = server_parser.parse_header(packet).unwrap().unwrap();
        println!("{header:?}");

        let publish = server_parser
            .parse_publish::<_, Vec<_>, Vec<_>>(header, &packet[2..])
            .unwrap()
            .unwrap();

        println!("{:#?}", publish);

        println!("{}", core::str::from_utf8(publish.message).unwrap());

        // server_parser.parse_header(data)
    }

    {
        let client_parser = mqttparse::v5::Parser::<mqttparse::v5::Server>::new();

        #[rustfmt::skip]
        let packet = &[
            0x30, 0x4F,

            0x00, 0x04, b't', b'e', b's', b't', // Topic name "test"
            0x43, // Property length

            // Payload Format Indicator
            0x01, 0x01,

            // Message Expiry Interval
            0x02, 0x00, 0x00, 0x1C, 0x20,

            // Topic Alias
            0x23, 0x00, 0x0A,

            // Response Topic
            0x08, 0x00, 0x0E, b'r', b'e', b's', b'p', b'o', b'n', b's', b'e', b'/', b't', b'o', b'p', b'i', b'c',

            // Correlation Data
            0x09, 0x00, 0x08, b'c', b'o', b'r', b'r', b'_', b'i', b'd', b'1',

            // User Property
            0x26, 0x00, 0x04, b'n', b'a', b'm', b'e', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e',

            // Subscription Identifier
            0x0B, 0x2A,

            // Content Type
            0x03, 0x00, 0x0A, b't', b'e', b'x', b't', b'/', b'p', b'l', b'a', b'i', b'n',

            // Payload
            b'h', b'e', b'l', b'l', b'o',
        ];

        let header = client_parser.parse_header(packet).unwrap().unwrap();
        println!("{header:?}");

        let publish = client_parser
            .parse_publish::<_, Vec<_>, Vec<_>>(header, &packet[2..])
            .unwrap()
            .unwrap();

        println!("{:#?}", publish);

        println!("{}", core::str::from_utf8(publish.message).unwrap());
    }
}
