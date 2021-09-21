pub const FRAGMENT_SIZE: usize= 1024;
pub const REED_SOLOMON_DE_EN_CODER_SIZE: usize = 10;
// the above two constants give us a maximum packet size of the largest packet (Cyphertext Link + Cyphertext NarrowWaistPacket)
// of 1469 bytes. 1472 is the maximum we can go (see https://gafferongames.com/post/packet_fragmentation_and_reassembly/)
// thus we have about 5 bytes of error correction capability and still have a nice roundish number of 1024 of data.

pub const DATA_SIZE_START: usize = FRAGMENT_SIZE-2;
pub const DATA_SIZE_END: usize = FRAGMENT_SIZE-1;
pub const DATA_SIZE: usize = FRAGMENT_SIZE-3; // 3rd bytes is a random u8 to add entropy

pub const BLOOM_FILTER_LENGTH: usize = u16::MAX as usize;
pub const BLOOM_FILTER_INDEX_ELEMENT_LENGTH: usize = 4;
pub const NONCE_SIZE: usize = 8;
pub const TAG_SIZE: usize = 16;
pub const ID_SIZE: usize = 32;
pub const CC_SIZE: usize = 32;
pub const SIG_SIZE: usize = 64;
pub const BFI_BYTE_SIZE: usize = BLOOM_FILTER_INDEX_ELEMENT_LENGTH * 2;
pub const BFI_COUNT: usize = 6; // RES, REQ, APP, MOD, FUN, ARG
pub const U64_SIZE: usize = 8;
pub const FRAME_SIZE: usize = U64_SIZE;

pub const REPLY_TO_INDEX_SIZE: usize = 2; //65535 different types of addressing should provide sufficient expansion
pub const REPLY_TO_MPSC_INDEX: u16 = 0;
pub const REPLY_TO_RF_INDEX: u16 = 1;
pub const REPLY_TO_UDPIPV4_INDEX: u16 = 2;
pub const REPLY_TO_UDPIPV6_INDEX: u16 = 3;

pub const REPLY_TO_MPSC_SIZE: usize = 0;
pub const REPLY_TO_RF_SIZE: usize = 4;
pub const REPLY_TO_UDPIPV4_SIZE: usize = 10;
pub const REPLY_TO_UDPIPV6_SIZE: usize = 22;
pub const REPLY_TO_SIZE: usize = REPLY_TO_UDPIPV6_SIZE; // select the biggest transport size, I assume IPv6 to be the largest addressing schema out there.

pub const REPLY_TO_INDEX_START: usize = 0; //
pub const REPLY_TO_INDEX_END: usize = 1;
pub const REPLY_TO_START: usize = 2;
pub const REPLY_TO_END: usize = REPLY_TO_START + REPLY_TO_SIZE + REPLY_TO_INDEX_SIZE;

pub const NARROW_WAIST_PACKET_INDEX_SIZE: usize = 1;
pub const CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_INDEX: u8 = 0;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_INDEX: u8 = 1;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_INDEX: u8 = 2;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_INDEX: u8 = 3;

pub const CYPHERTEXT_LINK_TX_PK_START: usize = 0;
pub const CYPHERTEXT_LINK_TX_PK_END: usize = CYPHERTEXT_LINK_TX_PK_START + ID_SIZE + CC_SIZE;
pub const CYPHERTEXT_LINK_NONCE_START: usize = CYPHERTEXT_LINK_TX_PK_END;
pub const CYPHERTEXT_LINK_NONCE_END: usize = CYPHERTEXT_LINK_NONCE_START + NONCE_SIZE;
pub const CYPHERTEXT_LINK_TAG_START: usize = CYPHERTEXT_LINK_NONCE_END;
pub const CYPHERTEXT_LINK_TAG_END: usize = CYPHERTEXT_LINK_TAG_START + TAG_SIZE;
pub const CYPHERTEXT_LINK_REPLY_TO_START: usize = CYPHERTEXT_LINK_TAG_END;
pub const CYPHERTEXT_LINK_REPLY_TO_END: usize = CYPHERTEXT_LINK_REPLY_TO_START + REPLY_TO_INDEX_SIZE + REPLY_TO_SIZE;
pub const CYPHERTEXT_LINK_NARROW_WAIST_PACKET_START: usize = CYPHERTEXT_LINK_REPLY_TO_END;

pub const CLEARTEXT_LINK_TX_PK_START: usize = 0;
pub const CLEARTEXT_LINK_TX_PK_END: usize = CLEARTEXT_LINK_TX_PK_START + ID_SIZE + CC_SIZE;
pub const CLEARTEXT_LINK_REPLY_TO_START: usize = CLEARTEXT_LINK_TX_PK_END;
pub const CLEARTEXT_LINK_REPLY_TO_END: usize = CLEARTEXT_LINK_REPLY_TO_START + REPLY_TO_INDEX_SIZE + REPLY_TO_SIZE;
pub const CLEARTEXT_LINK_NARROW_WAIST_PACKET_START: usize = CLEARTEXT_LINK_REPLY_TO_END;

pub const CYPHERTEXT_HBFI_SIZE: usize = ((BLOOM_FILTER_INDEX_ELEMENT_LENGTH * 2) * BFI_COUNT) + FRAME_SIZE + (ID_SIZE * 2) + (CC_SIZE * 2);
pub const CYPHERTEXT_RESPONSE_DATA_SIZE: usize = FRAGMENT_SIZE + TAG_SIZE;

pub const CLEARTEXT_HBFI_SIZE: usize = ((BLOOM_FILTER_INDEX_ELEMENT_LENGTH * 2) * BFI_COUNT) + FRAME_SIZE + ID_SIZE + CC_SIZE;
pub const CLEARTEXT_RESPONSE_DATA_SIZE: usize = FRAGMENT_SIZE;

pub const CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_START: usize =   1;
pub const CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END: usize =     CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_START   + NONCE_SIZE;
pub const CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_HBFI_START: usize =    CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END;
pub const CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_HBFI_END: usize =      CLEARTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END     + CLEARTEXT_HBFI_SIZE;

pub const CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_START: usize =  1;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END: usize =    CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_START  + NONCE_SIZE;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_HBFI_START: usize =   CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_HBFI_END: usize =     CYPHERTEXT_NARROW_WAIST_PACKET_REQUEST_NONCE_END    + CYPHERTEXT_HBFI_SIZE;

pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_START: usize =    1;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_END: usize =      CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_START    + SIG_SIZE;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_START: usize =  CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_END;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_END: usize =    CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_END      + NONCE_SIZE;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_START: usize =   CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_END;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_END: usize =     CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_START   + CLEARTEXT_HBFI_SIZE;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_START: usize =   CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_END;
pub const CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_END: usize =     CLEARTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_START   + CLEARTEXT_RESPONSE_DATA_SIZE;

pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_START: usize =   1;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_END: usize =     CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_START   + SIG_SIZE;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_START: usize = CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_SIG_END;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_END: usize =   CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_START + NONCE_SIZE;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_START: usize =  CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_NONCE_END;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_END: usize =    CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_START  + CYPHERTEXT_HBFI_SIZE;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_START: usize =  CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_HBFI_END;
pub const CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_END: usize =    CYPHERTEXT_NARROW_WAIST_PACKET_RESPONSE_DATA_START  + CYPHERTEXT_RESPONSE_DATA_SIZE;

pub const HBFI_FRAME_START: usize = BFI_BYTE_SIZE * BFI_COUNT;
pub const HBFI_FRAME_END: usize = HBFI_FRAME_START + FRAME_SIZE;
pub const HBFI_RESPONSE_KEY_START: usize = HBFI_FRAME_END;
pub const HBFI_RESPONSE_KEY_END: usize = HBFI_RESPONSE_KEY_START + ID_SIZE + CC_SIZE;
pub const HBFI_REQUEST_KEY_START: usize = HBFI_RESPONSE_KEY_END;
pub const HBFI_REQUEST_KEY_END: usize = HBFI_REQUEST_KEY_START + ID_SIZE + CC_SIZE;

pub const BOUNDED_BUFFER_SIZE: usize = 25;
pub const RESPONSE_STORE_SIZE: usize = 250;
pub const PACKET_PHEROMONE_TRAIL_COUNT: usize = 500;

pub const LABEL_SIZE: usize = 100;
