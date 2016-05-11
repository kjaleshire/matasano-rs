pub mod challenge_9 {
    pub static ORIGINAL_STRING: &'static str = "YELLOW SUBMARINE";
    pub static PADDED_STRING: &'static str = "YELLOW SUBMARINE\x04\x04\x04\x04";
}

pub mod challenge_10 {
    pub static DECODED_FIRST_LINE: &'static str = "I'm back and I'm ringin' the bell";
    pub static FILE_PATH: &'static str = "fixtures/10.txt";
    pub static IV: [u8; 16] = [0; 16];
    pub static KEY: &'static str = "YELLOW SUBMARINE";
}

pub mod challenge_11 {
    pub static FILE_PATH: &'static str = "fixtures/7.txt";
    pub static KEY: &'static str = "YELLOW SUBMARINE";
    pub static DECODED_FIRST_LINE: &'static str = "I'm back and I'm ringin' the bell";
}

pub mod challenge_12 {
    pub static APPEND_STR: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    pub static EXPECTED_STR: &'static str = "Rollin\' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";
}