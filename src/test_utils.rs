#![cfg(test)]
#![allow(unused_imports)]

use std::fs;
use crate::state::State;
use ethereum_types::H256;
use crate::trie_nodes::Node;
use crate::nibble_utils::get_nibbles_from_bytes;
use crate::get_block::deserialize_block_json_to_block_struct;
use crate::rlp_codec::get_rlp_encoded_receipts_and_nibble_tuples;
use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
use crate::get_branch_from_trie::get_branch_from_trie_and_put_in_state;
use crate::trie::{
    Trie,
    put_in_trie_recursively
};
use crate::utils::{
    convert_hex_to_h256,
    convert_h256_to_prefixed_hex,
};
use crate::make_rpc_call::{
    deserialize_to_block_rpc_response,
    deserialize_to_receipt_rpc_response,
};
use crate::constants::{
    DOT_ENV_PATH,
    DEFAULT_ENDPOINT,
};
use crate::get_database::{
    put_thing_in_database,
};
use crate::types::{
    Log,
    Block,
    Bytes,
    Result,
    Receipt,
    Database,
};

pub const TX_INDEX: usize = 96;

pub const PROOF_1_INDEX: usize = 14;

pub const PROOF_2_INDEX: usize = 134;

pub const WORKING_ENDPOINT: &str = "http://localhost:8545";

pub const SAMPLE_BLOCK_JSON_PATH: &str = "./test_utils/sample_block_json";

pub const SAMPLE_RECEIPT_JSON_PATH: &str = "./test_utils/sample_receipt_json";

pub const SAMPLE_RECEIPT_JSONS_1_PATH: &str = "./test_utils/sample_receipt_jsons_1/";

pub const SAMPLE_RECEIPT_JSONS_2_PATH: &str = "./test_utils/sample_receipt_jsons_2/";

pub fn get_sample_tx_hashes_1() -> Vec<String> {
    vec![
        "0xee6b2afff6a61686199965dd64d56ec613213b48bb4620e71e0176a881d3b0dc"
            .to_string(),
        "0xf2df2d51c0b5187e32363ec5dbcfe2e0bb8b8cb70a6708ffc0095d9db53ffda9"
            .to_string(),
        "0xab8078c9aa8720c5f9206bd2673f25f359d8a01b62212da99ff3b53c1ca3d440"
            .to_string(),
        "0x0ab2a8d425c3a55855717ce37b0831f644ae8afe496b269b347690ab4f393e3e"
            .to_string(),
        "0x5af4923b95627fdc57c6573d16e6fa0df716a98063a1027d9733e3eed2cbc24b"
            .to_string(),
        "0x93c8c513ad5a3eed0150166861c76010254efedbe4951ccb4d02f81cc0f85369"
            .to_string(),
        "0xe35e3b404ccd568df46ed52ce421998b83063ee1ee1420b36a90288121d5dcc1"
            .to_string(),
        "0xcdc5a5c943c62a489a04045dbe0e10eda34e3a7162ca6fb0e618b6590ca72ae1"
            .to_string(),
        "0xe805f3c56e99d3dbbf3bc0fd93f440fd8c9dae1f7876153f96449da523ea21f0"
            .to_string(),
        "0x4250ff983d0907f560003873c6a916e319a85a111f26127fb2ad459a296e0ce8"
            .to_string(),
        "0x8cedbb955a7c090ea993591ea541adfe1383f3b2391b74526ef481729b32aa7f"
            .to_string(),
        "0x8bbcf4950d5924a739114ca0c2bc6f2be118651ccd0dc9028f74f500198ecc06"
            .to_string(),
        "0x5f023c49e60c14763f5fe72cf6df2666aa4d311e6897ce408301a7246dc17bda"
            .to_string(),
        "0xbbebd7bbb8797b8790e4f91a0ee49080c4456b8f95c27af8562f70dda40be67a"
            .to_string(),
        "0x640cb533d56a7e215c6a81aa1cf988c1e7ba479e70a571b974fa811ab2d41796"
            .to_string(),
        "0xa067162103a794e23234844ff4c8951853488cbafb3e138df2a8ce24968fd394"
            .to_string(),
        "0xf9ca12a74c3454fcf7e23f5287a057c3605e2aec13fee03a3e03b4774b5faf38"
            .to_string(),
        "0x20d2a35a89b01589489f142f4881acf8e419308f99c30c791a1bb1f3035b949e"
            .to_string(),
        "0x40a07797beb2b5247a832e62deff7b631f415a5e6c559eae621d40bc7c33e8bd"
            .to_string(),
        "0x852cce56dcd2d00c22fab9143d59e5e2a547f0d3390e500f351124b922e7903d"
            .to_string(),
        "0x164207a34902693be57ccc4b6c2860eb781db2aba1a6e2ed93473a9dd516a542"
            .to_string(),
        "0x9b8063fe52a38566d5279e8ee9fa3c23c17557b339ea55a7ea1100b44f436434"
            .to_string(),
        "0x5272da6bc5a763d93e2023a1cd80ad97a112d4a8af0e8e0629c5e7d6e5eddb9d"
            .to_string(),
        "0x4d2c712ffbc54f8970a4377c03cc7ca8b6d58f8af2181282954b9b16f860cda2"
            .to_string(),
        "0x49b980475527f989936ddc8afd1e045612cd567238bb567dbd99b48ad15860dc"
            .to_string(),
    ]
}

pub fn get_sample_tx_hashes_2() -> Vec<String> {
    vec![
        "0x07dd9b6591c8b5ee7dabd7729d5feb5c5df49fed8b6ec2cab6564e3f5870b8cf"
            .to_string(),
        "0xbc4b13682d42c1d9dfe516365081d0c65762a0411b3b6d533b7b9b2bb6fcf2b5"
            .to_string(),
        "0x4f31036dbf2dcd3e536e5ec7f6c5e994472685fada0ebd4318cf5ca99d58c4ab"
            .to_string(),
        "0x8b3827f52ad4b47c22ed03caaf67092f31ab4a9d8d12f984a3491a85b30fc20a"
            .to_string(),
        "0x1c853611ecacf3c7a4e64d13b604a8a02868cfa9f46a28416244d5c947b27b4c"
            .to_string(),
        "0xa0fa87a5533f59eb585ccbb7eac26dcbc1ff8875c0fd17c1fb0e0075ab5d6b68"
            .to_string(),
        "0x6edbbe5536423cb5032b614791f443d7335a78642d99b927e92c879c96e81c78"
            .to_string(),
        "0x92dce67f5d088c59bedd8d69c5d8364858fd4bddc93b853a64e4d1d70d388f14"
            .to_string(),
        "0xe35c8b0103f8f8c90779d123ee882f026e99c48d13a11256355d95bbf27bb531"
            .to_string(),
        "0x774e8e9509b7862b2a9db8f913a4b6cd67284129c349650dc50e2a6ec8a4ef79"
            .to_string(),
        "0xee4866f3953ca5acbe087c4309123b409dc218f6bf21e37a2462a98cb6f1da40"
            .to_string(),
        "0x7dfe344dd7ccdcb473f5819c260ab7021daaac686ee2a0a74a0ef036aa28685e"
            .to_string(),
        "0x2b2f28c732f6f72264d7826c78f88fb6ef966698bf666ccdd5e6da4324f27012"
            .to_string(),
        "0xa2593fede895bee85342683f2440be58fd710b39b9359c2c8b9a67bc50a2db97"
            .to_string(),
        "0x6774e92025938351b6f66d17f5ad56dd033b7f1d7270c41a4e6cc3fb5b87422f"
            .to_string(),
        "0x51bc3df733bca53c5b8863d6afcfdca3f606f563cb1645a93cb7e0527d6bd8a2"
            .to_string(),
        "0x8145ca6957c0422db5f979f2a5418ee9189ef7b20b8592dc56040b4cfd09f75a"
            .to_string(),
        "0x66f543fb0aaa1edbaccb9a7432636e2de0f4a5001c25f9a05fcebf74cdae92ae"
            .to_string(),
        "0x14e93355fbbb29a34540a7262b645435c80636f13fe81368d81fab2370cc6c0c"
            .to_string(),
        "0xcd96377e671686dc4d6668a5de26e1bb8336c14fd530c6017a4e98ad9f50d484"
            .to_string(),
        "0x83623ff3f2bd8785028d2f02ba0b6cca239f595bfdd510b9682a3f2b1bcdd217"
            .to_string(),
        "0x9a3a9ee79d1453e13d1b372b72f15897a5ac0570652acd30565c7e13d134142b"
            .to_string(),
        "0xd451c51a5469be5dcc8cb13ff1b3184d730bc9ef95a5a70ef138de4de3c60e91"
            .to_string(),
        "0xc930535b3572754433ed446de9f65f41a92369661ab062df17640a81933204d8"
            .to_string(),
        "0x99784377782aa84acf4858bf5e65c42d97cb59c9b9c071417d396eca73677209"
            .to_string(),
        "0x3bdcf84480dcedbeb474025794c9dc451a0b6c682e639903be1bd31bd61452d8"
            .to_string(),
        "0x471c1c0d98d3f83ac88630e1543041ae29768a25cc2b40af80c671e69652e23e"
            .to_string(),
        "0x24b0e5de9821a4778230eb6ff85d9780fa4c0de46e1910e0d4abf680d6e678f6"
            .to_string(),
        "0x844d6a69561de72505fb644d66b56b361e7ac892eba8905ef8db8a09e980a2fe"
            .to_string(),
        "0x8a73834fa1154ea60462bf7664c52a81d8fe7c1f40f635ca2f3774a35abc9f49"
            .to_string(),
        "0xe3dd6b1adc9ce0929dec0714b1c26574ae9bbff4769a1526598678a91f434f42"
            .to_string(),
        "0x63e045c76edf59ad01a2b908a01ce4d1d20ebfb28e00263c4a367967c171ac35"
            .to_string(),
        "0x3eab8e0f2815f311c64d3a97c5c9fd1b576d22164401ce2863b352a85cecc01b"
            .to_string(),
        "0x522ef7b2b6524faf7742ee3ae4200d15de1e5f043ef6d4f49c7c78c23bed9d64"
            .to_string(),
        "0x3154d752e1b8ac70bbd015643758235bc3092fe1e26aa24cec0d0e4aeda82abd"
            .to_string(),
        "0x8de66f29fa3e4f293c3098c7a6d9fb9704a81996572b1f90590d75a92bd0b0fa"
            .to_string(),
        "0x4dbe0546c8fb995df781b28b13a199a17681f80e4fc52ca00c22290344fdf0cf"
            .to_string(),
        "0x3e47fc34fa7239ed7d22d997aefa42c7b62b1eaf4384a6d4a90b09fb463bc943"
            .to_string(),
        "0x7146ebca597f00e7b56aef3370d99d0e32ed82a257a108397fcff53f7f5f25b2"
            .to_string(),
        "0xac11e8471a6f660633ece024604596a7c28c9b79b4c20c65e791c48fbca1d2ca"
            .to_string(),
        "0x68890c4a52e91298b2c3ddf5b3eb22b79f84f192beed2eda5ee7c8a710032446"
            .to_string(),
        "0x6f093b4bf7101bf118b5b5ddfd6a89f75b6a850c3058e33adcdedc2a26c903b5"
            .to_string(),
        "0x5946fa56a2c03d45cf7635763a7b721b652df0f4075527715b3e3c22ee078166"
            .to_string(),
        "0x234c40de2b80b13608df62a62d778efba112423b4102ad3d366e7d7cb63214bb"
            .to_string(),
        "0xdccb599d783a0f0644912be86e59227b9e2fd3acac40ae595a6f06b6aed39e47"
            .to_string(),
        "0x2bfadf455adee791b0270b5a10aafc4cf3ddc4e1842b1c2f4209de9c942e07a3"
            .to_string(),
        "0xeebbb299f904452dad59cd0a21812318129922705d895c65d856e7f89163cc36"
            .to_string(),
        "0xce53fd86e55430dca16d00339b8ee9fd9624f2ec4c5db3185c6c94304382e583"
            .to_string(),
        "0xca30e6b5221b0964b7599dc43dc7230ddd66b15145ab4d17a0c25d7193fcb143"
            .to_string(),
        "0xbc46ba92eb64d34590ff19d8e6af162aba56148ab97df87c602c6c2008a00362"
            .to_string(),
        "0xd86041d902e7d974bd102e105cef045866fffb1ddc7cd5f777a454780772a4ff"
            .to_string(),
        "0xe4828b3860e891e2f9cd735a859bf5fc853875668d1cae2ff1e3566911d69c50"
            .to_string(),
        "0x68ce3fc05c1e753290dcbe0d0c3472acbc89b9274e11652f02afb1cc3f8fe737"
            .to_string(),
        "0xf8d6e1beacd3136c7fbf05dcef893fcc917161a7106d98745bd797231a46ac70"
            .to_string(),
        "0x4411a4b9571e5ee36807bcb432eb55266af49fbff2687d4d27826a4d1b1fcf6e"
            .to_string(),
        "0x4ded32917cf04078813b4f9f1901cd2dce68b62f64c48fbbffbed5d4ba3c3452"
            .to_string(),
        "0x0fdd60f46ab0bd61610005cb8b3ade6adc900fbf1a6e4beea1e740a34b44e712"
            .to_string(),
        "0x3c9e7d9abcc62c4feab46c86fb74414d71dd3cf7c3e2acbbb4102d92f76b22b3"
            .to_string(),
        "0xb8a1184ba719f10af0f592d0f09bc6745787bac3b60be0e9c9df5439bcb0213b"
            .to_string(),
        "0xaa576afe990e664362261ee263565dfcc888bd889d4077c1d8f89a0827ca41cf"
            .to_string(),
        "0x57de6afed4275bd308630b282bd7b88bc727cc44ca4561dcefa6c951dc363c66"
            .to_string(),
        "0x7609a7189904614ad0bd046d6987fe70471d55b1c2fcf275673490a8dec8fba7"
            .to_string(),
        "0x291267a432a4eb446d4129defc9cd9070a49716d296caeddbe33292d0c047ce1"
            .to_string(),
        "0x69e5469acb42baa2e819c52aa6015a29b1273a70a5cca4d7e0c0e33e07db9a58"
            .to_string(),
        "0xcfff0319844efa9ba8de27840bd205c38e74b071bf4b1dee3cf24be0e1a27b8b"
            .to_string(),
        "0xf5e9d1746900055d7fd8dbbb0f77618bc8555dbdf5beed51bfb074079227fb74"
            .to_string(),
        "0x7db0646a3fc5d3cdd1a2df92c5e1e51f44e64912fa344d98dd3068c0c25d96ab"
            .to_string(),
        "0x42cb51e6ae55f231d3a2c374bd581c672c1c230de1690b1883865d74390548bf"
            .to_string(),
        "0xdb2a8609b8b7c69eda17561ec96fbefeb4b61c8899b081cebec5d12b8fad7994"
            .to_string(),
        "0xe85192a762f4564d35aad9a2ec8717e33664b8253fbd48abb34fd1ae27cda6fc"
            .to_string(),
        "0x0400b3fbe2d9ade065dcecb90cca119660c428156e00a39ea5234a6a9b8b7016"
            .to_string(),
        "0x9805ab92860ff05274dd2f601768208a665e4acb2dd049a2360fad672c54ead8"
            .to_string(),
        "0x315c2a5ec3bc8c6e32cda0dab163a842c4bbcae3a02e7f4659e74653250d802f"
            .to_string(),
        "0xccc72d00bf65ee7456e6dbddd52876642a1ca993f8ba234d2d8f2bc079a97f89"
            .to_string(),
        "0x6e83f92881a59682dba483e27213da26c3c91e84701cb7a5ae36088076f209b1"
            .to_string(),
        "0x3335f0981493cd09e3d2a24fdd2bda51f2200eebdbf8c95f4688cf7e81460a0f"
            .to_string(),
        "0x7601fda949aff63ca0a090207b509e40195def3972e2b8e2635986b6226a1cf9"
            .to_string(),
        "0xd86e9e436d029bbd724de9fb23a596a658c818b9388517308b0b8da10747c60e"
            .to_string(),
        "0xc2d0e0defe99411ab628e6718799bc1874593a8bd6465512b26be8b277f888de"
            .to_string(),
        "0xf7ea41d8c6e0846906a287fecad0e54eefdd44ef583c3a6113dafc5467f6bd09"
            .to_string(),
        "0x2133a100c26b73430213b6b391fd8d611b14d199e354aa37885b1a1248687a6d"
            .to_string(),
        "0x98ad164af7e5d1754d18b54d29f2abe760d4762e2c35f92c3e259fd93dda9507"
            .to_string(),
        "0x19b1c3682a8bea3f2d8b92f29d0762c55e9bdecd60649bf8a06382f8b06a89e1"
            .to_string(),
        "0x394e63e3b2a81972e459c91b2a85629cb843c8baacb757f67179a1cf4b86649e"
            .to_string(),
        "0xbaad0c878c9a3150fbcd92d693f75b6695477cebf15a7d8ec3e055fcc41875d9"
            .to_string(),
        "0x22eeb3c814d0c82e9ad82218db4f5275b201df6eb24cd05dc96991126d586936"
            .to_string(),
        "0x7c1910fcbd47121ce694a4f28619a1801780575b81b96c7f39dee88d504be5cc"
            .to_string(),
        "0xadfe653a3686ce7fc8a50abe92b27c6b93290c0b5f833eddf52091fa471f7b98"
            .to_string(),
        "0x10084c1501363efaf591084761b9e0b14523759ec1da027aad29e368ad35fa66"
            .to_string(),
        "0x427c1b90ef125f854731fb2152594247b18b34da9b7e68e57a1bab58fd5dd257"
            .to_string(),
        "0x33f9406514aeb539776ffc19a14a6c79a8c11f722ef51c38b38ede3f79a9e4c9"
            .to_string(),
        "0xe9cb5d1f00f221318bd266900ebdedd9cb73cd83c5c90736af64f80715baaade"
            .to_string(),
        "0x7970ce71ad08f5e437d9c896eefa3124ce5803a94d12dfa5f7832eac51890496"
            .to_string(),
        "0xbb9b05ee97017efa68861fa28d8cd3a78d547f68406bb913e28cce14ad759e2a"
            .to_string(),
        "0xf5fa79dfaa3ab52a0a85affe63e0a97a1625c1b40ca347b5bb481b9f4c736791"
            .to_string(),
        "0xae19f1aed8a203765e603a3254756e21ca86e668fcd1dc8e0b408d391aa9c3cc"
            .to_string(),
        "0x60b66e131af6ac59ae20cfa8b19282644f6a69d74515c342d6effda3d80a0766"
            .to_string(),
        "0xe666bf35814c261a7dd245ea3dd073b4a54cf5e31095a95d28520d072fdf6a6f"
            .to_string(),
        "0x004eb25bf13eb24db0e345bb9000ccaf7bd4de8c9179b901225cb030e5b5667c"
            .to_string(),
        "0xbd94b7ce955c2f5e045d092c97dcf5a85263cb3f62645b211c4e02511cc5d6f9"
            .to_string(),
        "0x5dcb405529fecb89fc2233d4f0fdf11acd6e27100aee1493a281798d4ab74e04"
            .to_string(),
        "0x02d5f22fd5b4a53e4add76559c2579da3f066a579528e649031c235239672ed9"
            .to_string(),
        "0xf9d577490509d0ba98d1713a4b5cfa030d6f93b128e3e85e56fed5c849970b5e"
            .to_string(),
        "0xd29f4b4b96ee3095e4b53316b7db1a517c005e9c1af9664f5f7c6ecfcf879529"
            .to_string(),
        "0xf1933b70ace2fec925e6997f9ed8e65cd4817eabbdb1dc32fb30724308b226e8"
            .to_string(),
        "0x95ac9aa452a25460fde405ef9e056020fa11dda54647d45b0f444c7df2acc1d9"
            .to_string(),
        "0xd287835f2429ec421ec432e7e2b6626894c781bf197a5d5f099920d005ae2e4b"
            .to_string(),
        "0x80bb8a63ee3b9b6fb6ad02a90b0c73106d79c42c6ac9e7f46ebd7f358df36ea4"
            .to_string(),
        "0xd1193c365e044db5d62316f1ac6bab5b9fc4d850092083f52bda1ea24acc7d2e"
            .to_string(),
        "0x07d37a7f42f66d1367f7e3e9d45a3a3610c7e6a25bb5a8cdeadcee35b61d7d71"
            .to_string(),
        "0x33874c4d2b1b9cd24ebde8b3e041fce6bda86a466b9d9b2baf2ed8e3a3123fb8"
            .to_string(),
        "0x2890d760a8f2fba5313daeeb6737582c481eaab7559aa7e03c91ffe020ef88cf"
            .to_string(),
        "0xa9750e6ab507ba6a9d5677c703d5b3803092f74fc8fe653afc331ad0537c6029"
            .to_string(),
        "0x65c4c1bb4dfdc694b8222d97893f6881906960e3a92ec36e73b3799ec7cf821d"
            .to_string(),
        "0x89c3f4f877fed580bc433d4e8b6ed5978a70967769f50fa15b3a18690a3b73ac"
            .to_string(),
        "0xcbed350f8291c7afa10bd6c55dc8a0860b3eaa963804a249d67d4e6e2987ae11"
            .to_string(),
        "0x3d50147105f69850f3ead900bc8b40eec4b84b5b468c4105041b7740b3f4b6b4"
            .to_string(),
        "0xc3b7243223bc0741394f973ee3d6e2a87185ccca0e3ce282afc81fc7bde80ae8"
            .to_string(),
        "0x2d44b97872d38390e729bf59c2da3c36a0fbdd21110ea0aed929ca125c394ab3"
            .to_string(),
        "0xc5688c24b08d52149c30c44779eb4f63d882937a2f2da2be0f1525d8a8ca3520"
            .to_string(),
        "0x0a6e680f653dd27541e8af5dcd665813d3df2f5990dd8b0e474697596ff61122"
            .to_string(),
        "0x68aa5051888871b00fe42349248288d5437e000680844acd60f4b4798f895f4d"
            .to_string(),
        "0xfd0bae2887ad4c1cb7404a1c193c444e5e1a40196cdd8843a4a9b0a59cf8dcaa"
            .to_string(),
        "0x0d7a9de2c204bbfb93bf6c9fd4d34f179a2c70ef53c6c83c1ec9d30333963184"
            .to_string(),
        "0xea9a63caef80c7b8dc9a7e3fc08b951868a116c7384775159a30ab2390026d07"
            .to_string(),
        "0xda4c3e0179537594ab1e7994bc168fc3f8abde4aaa8ec00195b3068aefd8555f"
            .to_string(),
        "0x0dd7bbbd5f801699c6abd77f92f5d780bb154bfc9c17f0c77c87b6c48124c386"
            .to_string(),
        "0x8d22b309557f434ce148c25f1cd792d30aedfaa62c8bf5210ce917b6a465b223"
            .to_string(),
        "0x7f55539a419a08151613e95967b1620eb9b84677624110024e15d82dc6811ed0"
            .to_string(),
        "0x6c8a3c8410e8d00e099acb1dc1b82f2597c6275a7abb9c0c1979a1067f483905"
            .to_string(),
        "0x78ba0bc5416c27fce1ec35a631a377e1e47f322ff510792d48ac05130d0cf02c"
            .to_string(),
        "0x4cbc518bfb7b7abdaa6ae47c8c323b8db918661e3e9a678ce937cea4ca63eb54"
            .to_string(),
        "0xd5d0cb5050ccfc00a56c42c814a4d0f784a969de0400467415df24bd306bdc6e"
            .to_string(),
        "0x551aeaab762b8bd0261204bb806ad3ae15627b039af86523e27ce09de522fa11"
            .to_string(),
        "0x05f6110dc812d0cf89356778b7bc34b9f50bbe8f6790e6b08377cbd9cac671ba"
            .to_string(),
        "0x8ad20589903f1989bf5c834b5dc7455f253258bcf1d88883cadf1a607ecf72cf"
            .to_string(),
        "0x5f9754d7386ea23a5df0cf2dcb118e214ed51b61a265a0c146dc8809a09c8d87"
            .to_string(),
        "0x989ec9e6cb54ec854ad261b5dead5a7c6e094264200f0294379ea6497fcc1698"
            .to_string(),
        "0x084dbc4156445b4d9b8b5ddcba1e0ad9b62e6c67037281f765c04a761e866273"
            .to_string(),
        "0xdcbde5886a8f26112ca879aeed59e9974afac8ed7ce86c64cff0f3e90043de6f"
            .to_string(),
        "0xaf7fc15eb07332b2a64120ba565b011813d94550ff62d16fb982cea50e75bf63"
            .to_string(),
        "0xb581869f92be82a656da5ab21c6c53563e6f99918d495dceaca66f1362a28617"
            .to_string(),
        "0x1c35b6f6fabb224c71afe81792d18e468527261461c0390c58bd0c01f13fc12b"
            .to_string(),
        "0x6f647e9d9da3959ae003095973b0d73cab7a16134f291c60444eb1123707f6b4"
            .to_string(),
        "0xb98ab8027e871a692732a2173ffa11503855a949e7dedbdac94eb5facab47af3"
            .to_string(),
        "0xdad4288d42a03f2f0cf740ec78e25ae1c81777e4ecf3ddba76dfadb7a26afa51"
            .to_string(),
        "0x058cc57d6a017e9b250c0f94516deedc861a4936ea19c4a1951fffee9aafd028"
            .to_string(),
        "0x92c49ca5bc0df07f127e2115b7cdeb8d7f2c64ae3a89640e9ee9803c75693c5d"
            .to_string(),
        "0x4b81949660b441eb1a884aeccb85b6c7dd1c1aedeb754cc7aad79c2d6d7b1e56"
            .to_string(),
        "0x8d80b181f8fe3d62b0eafc56248b11677d7902806685cbc01bb59cd62169b795"
            .to_string(),
        "0x71c8b037dbd77d572c98819ce23c36b9b66d172751ca27b82bc1245ff208a0db"
            .to_string(),
        "0x997af004bf4552bc1f56b06de9bf95c6818c48dec6e3cc69fdc10e5bed977bb1"
            .to_string(),
        "0x0639260b708a0da11225f37517bfad93bed6e1cb179503d4321b3135e1f63950"
            .to_string(),
        "0xce7fbe017bae99eb201ab5f385e45789e4469d6a65d56047ae92b7d3c544cae4"
            .to_string(),
        "0x65ac06dd833b3c72c2d80ab916a324354fdecbf46db876285adcb972b074126b"
            .to_string(),
        "0x8020cdbfda2ea5de1f9db7f3a1a45920fa556603ab978b4741c2f1a0849c4e35"
            .to_string(),
        "0xb8f2edb8bf8e0315d964d8dc9b65a413591bf70962f377a8605efe6f630ba175"
            .to_string(),
        "0x9dfd9a6073d9dc58e472b1f24e282699db118498cecdbf56cd25ba706a2856f4"
            .to_string(),
        "0x9d842351ec922103ec09f1fbaff28ae2bc3b1a681dff749ba795b874f233cffb"
            .to_string(),
        "0x17ff80a9512178cd88117f06e2f24aa7f8b5959fd92e359b2c459d2acfa0a21f"
            .to_string(),
        "0x075078ba6724414f6bb45c8c69c4aa374207d42e3e6b1dbec23a77aa1afee1ce"
            .to_string(),
        "0x3715be834c8b8b15ef98b671572fb58cb9a80e32c93e8a7a25808ccfa8f25743"
            .to_string(),
        "0xbfca1d0d493e3d67bcdc3417391f1c7ae7453427bba6fd7d06fe0689877d12e3"
            .to_string(),
        "0x22acd737c85a76fbd6c83bd2c7fbf1f56c2ab99e08a623da69013a616b11f3a5"
            .to_string(),
        "0x504cf9d0832857e98c9d420b4d2e4160f794f43252c60b693603d78701c0fabd"
            .to_string(),
        "0xbcca0fb551c0f8715d699e044cf863ecb8521f4a7e3c61e7c3d20d2394c9b994"
            .to_string(),
        "0x05a27ed81cac01454e9ceb2a492944a908ce4eed6886ee8a10e1f92089f23eb2"
            .to_string(),
        "0x2a792cf4e87a459052031eafe086eadc2ada640766cd8d42ca5f21131034ab4b"
            .to_string(),
        "0x55cf6839e8f10bde770ed6bf8102e873f05bebc7ce01de2d93b2356f3b45b0d5"
            .to_string(),
        "0xfde927d499d9f99f76d1641b0576c2d6026fee6346f7ac9a8996a0e3631e36fa"
            .to_string(),
        "0x6da583390c2610661bb68f7ca1a732ed9bd03fc2b5760b53a9064f6ae5aa647d"
            .to_string(),
        "0xbdb25a8bda3294c0a2f7ac73d0472c7d5ee37c69f4a6333df586eede858f5e9e"
            .to_string(),
        "0x774433e94167e0196b589f8249cee8c8a777ab578a73b05896736a6f39a0b940"
            .to_string(),
        "0x7c07f70412b332b68fd6c5550d49e7363286b601ade584794e359ff7aed19946"
            .to_string(),
        "0x996fda76f78382817c0b22645a62e15d201457b42bfc82a09f9007927d246735"
            .to_string(),
        "0x640bd536d51f6ca6c23abdc1332a67cb07967823f60c3f2fd22a6d4be7b85561"
            .to_string(),
        "0x955a583056d2269413fa8ac1c3f34d35d14cac31ded8df231868089ac5282f4b"
            .to_string(),
        "0x192683a668c683fd762ae70fde980a3c761e65d09e77c6fbf969feb676f7d2dc"
            .to_string(),
        "0x8af2449c0c77db3fd722f7a263d0e9ee356cbef87ca7b5e38a9e0032ee1d605b"
            .to_string(),
        "0x1c88da7d8a0dcff172d4c5485546bb2d7f1a98cf69f877c05bbcfe8a5476f147"
            .to_string(),
        "0xbfe243c11bb62d452ca97cbd9b4377c3fd03c0d84e33b9e48a77583f88e600e9"
            .to_string(),
        "0x3f2589725ae146b17e7e020c861ec597dc18018ac632df7222774791560c400a"
            .to_string(),
        "0xaa46a0b8a42f2f170587f69fcdaa760ad7c95ca0d482b7dc911b359e8d6faebd"
            .to_string(),
        "0x2b8b076125b602d37e53996882b6469925257786c48494c9f7b8847f866bb50a"
            .to_string(),
        "0xf3785f06c4982f832d2a0cb92c3ebf1c57d5e5ad39b2a8ecd8282165d37bc0bf"
            .to_string(),
        "0x6d43d41b6f07c616b0db42f24a24125c5ac28bf9cae158dc84e07decac5b9705"
            .to_string(),
        "0x0c8b8c91fb6ebe46b780bcba77e5466dbb32e30ef7632bac94014b948d08bb80"
            .to_string(),
    ]
}

pub const RECEIPTS_ROOT_1 : &str = "0x937e08f03388b32d7c776e7a02371b930d71e3ec096d495230b6735e7f9b20ae";

pub const RECEIPTS_ROOT_2 : &str = "0x4c9bb7d6a6c74445c15e5915262c49c69cd14b3e19620302f2c10303fef1e392";

pub const SAMPLE_TX_HASH: &str = "0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6";

pub const SAMPLE_BLOCK_HASH: &str = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

pub fn get_sample_receipts(
    path: String,
    tx_hashes: Vec<String>
) -> Vec<Receipt> {
    tx_hashes
        .iter()
        .map(|hash_string| format!("{}{}", path, hash_string))
        .map(|path| fs::read_to_string(path).unwrap())
        .map(|rpc_string|
             deserialize_to_receipt_rpc_response(rpc_string)
                .unwrap()
        )
        .map(|receipt_json|
             deserialize_receipt_json_to_receipt_struct(receipt_json.result)
                .unwrap()
        )
        .collect::<Vec<Receipt>>()
}

pub fn get_sample_trie_with_sample_receipts(
    path: String,
    tx_hashes: Vec<String>
) -> Trie {
    let index = 0;
    let receipts = get_sample_receipts(path, tx_hashes);
    let trie = Trie::get_new_trie().unwrap();
    let key_value_tuples = get_rlp_encoded_receipts_and_nibble_tuples(
        &receipts
    ).unwrap();
    put_in_trie_recursively(
        trie,
        key_value_tuples,
        index
    ).unwrap()
}

pub fn get_sample_leaf_node() -> Node {
    let path_bytes = vec![0x12, 0x34, 0x56];
    let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
    let value = hex::decode("c0ffee".to_string()).unwrap();
    Node::get_new_leaf_node(path_nibbles, value)
        .unwrap()
}

pub fn get_sample_extension_node() -> Node {
    let path_bytes = vec![0xc0, 0xff, 0xee];
    let path_nibbles = get_nibbles_from_bytes(path_bytes);
    let value = hex::decode(
        "1d237c84432c78d82886cb7d6549c179ca51ebf3b324d2a3fa01af6a563a9377".to_string()
    ).unwrap();
    Node::get_new_extension_node(path_nibbles, value)
        .unwrap()
}

pub fn get_sample_branch_node() -> Node {
    let branch_value_1 = hex::decode(
        "4f81663d4c7aeb115e49625430e3fa114445dc0a9ed73a7598a31cd60808a758"
    ).unwrap();
    let branch_value_2 = hex::decode(
        "d55a192f93e0576f46019553e2b4c0ff4b8de57cd73020f751aed18958e9ecdb"
    ).unwrap();
    let index_1 = 1;
    let index_2 = 2;
    let value = None;
    Node::get_new_branch_node(value)
        .and_then(|node| node.update_branch_at_index(Some(branch_value_1), index_1))
        .and_then(|node| node.update_branch_at_index(Some(branch_value_2), index_2))
        .unwrap()
}

pub fn get_thing_to_put_in_database() -> Bytes {
    "Provable".as_bytes().to_owned()
}

pub fn get_expected_key_of_thing_in_database() -> H256 {
    H256::zero()
}

pub fn get_valid_tx_hash_hex() -> String {
    SAMPLE_TX_HASH.to_string()
}

pub fn get_valid_block_hash_hex() -> String {
    SAMPLE_BLOCK_HASH.to_string()
}

pub fn get_valid_block_hash_h256() -> Result<H256> {
    convert_hex_to_h256(get_valid_block_hash_hex())
}

pub fn get_valid_tx_hash_h256() -> Result<H256> {
     convert_hex_to_h256(get_valid_tx_hash_hex())
}

pub fn get_valid_initial_state() -> Result<State> {
    State::init(
        get_valid_tx_hash_h256()?,
        get_valid_tx_hash_hex(),
        true
    )
}

pub fn get_valid_state_with_endpoint() -> Result<State> {
    get_valid_initial_state()
        .and_then(|state|
            State::set_endpoint_in_state(state, WORKING_ENDPOINT.to_string())
        )
}

pub fn get_valid_state_with_receipts_trie_and_index(
    path: String,
    tx_hashes: Vec<String>
) -> Result<State> {
    get_valid_initial_state()
        .and_then(|state| state.set_index_in_state(14))
        .and_then(|state|
            state.set_receipts_trie_in_state(
                get_sample_trie_with_sample_receipts(path, tx_hashes)
            )
        )
}

pub fn get_valid_state_with_receipts_trie_index_and_branch(
    path: String,
    tx_hashes: Vec<String>
) -> Result<State> {
    get_valid_state_with_receipts_trie_and_index(path, tx_hashes)
        .and_then(get_branch_from_trie_and_put_in_state)
}

pub fn get_expected_block() -> Block {
    let string = fs::read_to_string(SAMPLE_BLOCK_JSON_PATH).unwrap();
    let res = deserialize_to_block_rpc_response(string).unwrap();
    deserialize_block_json_to_block_struct(res.result).unwrap()
}

pub fn get_expected_receipt() -> Receipt {
    let string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH).unwrap();
    let res = deserialize_to_receipt_rpc_response(string).unwrap();
    deserialize_receipt_json_to_receipt_struct(res.result).unwrap()
}

pub fn get_expected_log() -> Log {
    // TODO: Implement == for logs!
    get_expected_receipt().logs[0].clone()
}

pub fn assert_log_is_correct(log: Log) {
    let sample_log = get_expected_log();
    assert!(sample_log.address == log.address);
    assert!(sample_log.topics.len() == log.topics.len());
    assert!(sample_log.data == log.data);
}

pub fn assert_block_is_correct(block: Block) {
    // TODO: Implement == for blocks!
    let sample_block = get_expected_block();
    assert!(block.author == sample_block.author);
    assert!(block.difficulty == sample_block.difficulty);
    assert!(block.extra_data == sample_block.extra_data);
    assert!(block.gas_limit == sample_block.gas_limit);
    assert!(block.gas_used == sample_block.gas_used);
    assert!(block.hash == sample_block.hash);
    assert!(block.miner == sample_block.miner);
    assert!(block.mix_hash == sample_block.mix_hash);
    assert!(block.nonce == sample_block.nonce);
    assert!(block.number == sample_block.number);
    assert!(block.parent_hash == sample_block.parent_hash);
    assert!(block.receipts_root == sample_block.receipts_root);
    assert!(block.seal_fields.0 == sample_block.seal_fields.0);
    assert!(block.seal_fields.1 == sample_block.seal_fields.1);
    assert!(block.sha3_uncles == sample_block.sha3_uncles);
    assert!(block.size == sample_block.size);
    assert!(block.state_root == sample_block.state_root);
    assert!(block.timestamp == sample_block.timestamp);
    assert!(block.total_difficulty == sample_block.total_difficulty);
    assert!(block.transactions.len() == sample_block.transactions.len());
    assert!(block.transactions_root == sample_block.transactions_root);
    assert!(block.uncles.len() == sample_block.uncles.len());
}

pub fn assert_receipt_is_correct(receipt: Receipt) {
    // TODO: Implement == for receipts
    let sample_receipt = get_expected_receipt();
    assert!(receipt.to == sample_receipt.to);
    assert!(receipt.from == sample_receipt.from);
    assert!(receipt.status == sample_receipt.status);
    assert!(receipt.block_hash == sample_receipt.block_hash);
    assert!(receipt.transaction_hash == sample_receipt.transaction_hash);
    assert!(receipt.cumulative_gas_used == sample_receipt.cumulative_gas_used);
    assert!(receipt.block_number == sample_receipt.block_number);
    assert!(receipt.transaction_index == sample_receipt.transaction_index);
    assert!(receipt.contract_address == sample_receipt.contract_address);
    assert!(receipt.root == sample_receipt.root);
    assert!(receipt.logs.len() == sample_receipt.logs.len());
}

pub fn read_env_file() -> Result<String> {
    Ok(fs::read_to_string(&DOT_ENV_PATH)?)
}

pub fn write_env_file(endpoint_url: Option<&str>) -> Result<()> {
    let url = endpoint_url.unwrap_or(DEFAULT_ENDPOINT);
    let data = format!("ENDPOINT=\"{}\"", url);
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn delete_env_file() -> Result<()> {
    Ok(fs::remove_file(&DOT_ENV_PATH)?)
}

pub fn restore_env_file(data: String) -> Result<()> {
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn get_database_with_thing_in_it() -> Result<Database> {
    let mut database: Database = std::collections::HashMap::new();
    database.insert(
        get_expected_key_of_thing_in_database(),
        "Provable".as_bytes().to_owned()
    );
    Ok(database)
}

pub fn get_sample_proof_1() -> String {
    "f91626f871a0fb5e0d429924a0287196102cda8544cfbbb0949d7ae9e6a2ebbdfe4f6e3c94eba0441d343ee56af21fb1a8e12802f9b91f415cb8dd5dcb47f880c4171846e11d69808080808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080f901f180a0d9673d2d9fc051cd0c137eb6e8e6fa792ca02465188c3408c86625d76f6396a0a068a36b8233852762f709f47f3bf8f49f7617b2768f432c4c53901158466f32cba0a2cb63340b3750a6f8e423d841f0f6ecf1ee905e2c5429215f90abab8cf981c4a0d5400a85346120207d40c40c706329212cad6256d6ac3c98b0a608382b94e49ba0086498ee145780c9a471de9095b478ca1555985afb5af85e9417240c42d1761ca0f919c75704da25ee06c61ec00a09b45e96233400846e210f1556e05e69b4026fa0ff852e16453c79cd5cb47453cc4b813f67ceeec2fba52f8493f34ce4e11f36dfa0e28a542ee13340426eae878242c5f6ae00d32d8428520d0e4d43c941c8501b5ca0690514b2df03293f0a4af4c5c1f7f4e14f597498fcafaef0294f7d9275fafb93a0fe6a59f583d64752de4922fa78290d52214d53dfdb8398ea622458045d1bf790a0b8c0730f4a260ecaff702f95300731b3b0bde28caf3591eb56c376e173655179a0c1fc1ea14cf024f0c58ae16906be6f3db05cee425ebfaded1402b248e979cecaa047b8d8cd77fbba9406f8be39009cd3de8681294ac06321d81ecefcabe2a50f5fa015cabab29394775f3844179f7200ac6f368a89a819ca370b29ef4e01fe1bd5f5a0fbfb53995b5a638a7f32d344fc04d26db19c1e0953d2b3128da43f9343c1e23080f913bc20b913b8f913b50183717d1db90100000000000000000000100000020000042000328000000002000000800000000000000000000100000002200008800000120000880804000040008081002000000000000000000000480004084008000080000404004000400000000880420000000200000220040000000000000809200000000000810400000000100000000000000000401000000000000040020400002000010100000000000000000000000200000000002000000000001040000000000104000000000002200000200110000500028902000000000000000020000000000000020008000800020000200000302000000a2000088002000000000100040000000000400400200040100000f912aaf87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a0e1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109ca0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da000000000000000000000000000000000000000000000000000be1571569ebfdaf89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a07fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df87a949ae49c0d7f8f9ef4b864e004fe86ac8294e20950f842a075f33ed68675112c77094e7c5b073890598be1d23e27cd7f6907b4a7d98ac619a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df8fb9457f8160e1c59d16c01bbe181fd94db4e56b60495f842a0ea9415385bae08fe9f6dc457b02577166790cde83bb18cc340aac6cb81b824dea00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950b8a0000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000001ee7fa454bdf5b6d000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d0000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950f89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8fa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a000000000000000000000000000000000000000000000000000000000172b812cf8fb9463825c174ab367968ec60f061753d3bbd36a0d8ff842a0ea9415385bae08fe9f6dc457b02577166790cde83bb18cc340aac6cb81b824dea00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950b8a0000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000000000000000000000000000000000000172b812c0000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950f89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da000000000000000000000000000000000000000000000000000000000172b812cf8799452166528fcc12681af996e409ee3a421a4e128a3e1a0f838f6ddc89706878e3c3e698e9b5cbfbf2c0e3d3dcd0bd2e00f1ccf313e0185b84000000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8f0000000000000000000000000000000000000000000000006261b3899e23cbe6f9019c949ae49c0d7f8f9ef4b864e004fe86ac8294e20950f842a0d30ca399cb43507ecec6a629a35cf45eb98cda550c27696dcb0d8c4a3873ce6ca00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56db90140000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000000000000000000000000000000000000172b812c0000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56d0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b6049500000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8f00000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000000f8db94818e6fecd516ecc3849daf6845e3ec868087b755f842a01849bd6a030a1bca28b83437fd3de96f3d27a5d172fa7e9c78e7b61468928a39a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56db880000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000000000000000000000000000000000000172b812cf89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000000be1571569ebfdaf89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca000000000000000000000000000000000000000000000000000000000172b812cf89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000011927d1af6564000f87994f55186cc537e7067ea616f2aae007b4427a120c8e1a09c2c6ec1cb8ee2fe8d5549d7d071a1a8f76ec3cc057d7c46f118247b0e5e8572b840000000000000000000000000d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91c00000000000000000000000000000000000000000000000011927d1af6564000f9015c9473df03b5436c84cf9d5a758fb756928dceaf19d7f842a0c7fce5271a7dcbf20bd48128dcbf6f2df01bceda67919e43870de3be7f1b0690a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392bb90100000000000000000000000000d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91c00000000000000000000000000000000000000000000000011927d1af6564000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca000000000000000000000000000000000000000000000000011927d1af6541593f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba0000000000000000000000000882d80d3a191859d64477eb78cca46599307ec1ca0fffffffffffffffffffffffffffffffffffffffffffffff67d19c841ccf2e46cf89b949ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000005be139fa43fdc0e583ac0e4fab48e5e451fa6575a00000000000000000000000000000000000000000000000001460fce85296abc0f87994f55186cc537e7067ea616f2aae007b4427a120c8e1a09c2c6ec1cb8ee2fe8d5549d7d071a1a8f76ec3cc057d7c46f118247b0e5e8572b8400000000000000000000000009ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951d0000000000000000000000000000000000000000000000001460fce85296abc0f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca00000000000000000000000000000000000000000000000000000000000022a6df89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba0000000000000000000000000882d80d3a191859d64477eb78cca46599307ec1ca0fffffffffffffffffffffffffffffffffffffffffffffff67d19c841ccf0b9fff87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a07fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000000be1571569ebfdaf8dc94d4240987d6f92b06c8b5068b1e4006a97c47392bf863a000293d5012632fad25e327fa894460c60bef74241d2f04c42802f4b2212f66aaa00000000000000000000000009ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951da00000000000000000000000005be139fa43fdc0e583ac0e4fab48e5e451fa6575b860000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000001460fce85296abc000000000000000000000000000000000000000000000000000be1571569ebfda".to_string()
}

pub fn get_sample_proof_2() -> String {
    "f905dff90131a0d02cd140be36772a3f2625d6555048db04d497ef93597c869f6120e3702f3fe1a0c5bd323412a3b70bf1404191a2a6f4f6d495aa3b06d4e98e0e0c7b876e3b90dba0cfe6b3f0576ed08d3aa1de057380b86387e97d256a9f6eb7f98f42304dc42cefa055aa466e458379e87e1e8f417122b1d8ce1487697601e24c804b20ad2ceb2008a00a454257e8826bc555f13e2610d09cb2bb5b93c61dc468c13619b2346e4d0f34a0294e8f7d67169822f67feea8f87103ffb463113cf658c452b1e7f176b9e3d2c2a0392ff864c6c538a951f66c8eb0b7953bcc01df7d64cf7ac8ca34dcdf7bdb31daa0b46ef2b2d6bfd406775e7e9689d9fec350f90ccafafe415df1ccbcc0cf4a0fc9a0e5516a81d7b05620820dec62757160e20a983bfacb90aabe02f9975ae89f43518080808080808080f851a0747db39fb51628381cc6d2a560570da0f91bcfa781c691542e84eec9f35661baa079076420da046b598254d6396c4b79b5594202c7eadf166e9ee6a7ccd35c19c2808080808080808080808080808080f8918080808080808080a0664372997bac3a2edfd8af5f4bdecbc8372662795a239c041be431be86c210b6a056cc8b139adb64a9315157318fe768c9943c6bb5547592c3a95724ac2c5bdc69a07692b0a375db5a16b3146695a0713749f167421af35b74517057fa33fe6aad92a0d5fc06d27773e9bb61c2b0af38ca8777db11984b6bb80932027238812e58f96c8080808080f90211a00784bcc7304124860945acee21f5d3e1c0f22c4e92f4862844488e248a00ca63a0caf1f87971b250c69e6ae82a0de31da470816275c9a76b6b664d4449f7e419dfa0a943dba7d889d750fc7ea1df3da9adde37a8504a0a4e666e9b7cf4657cec3a5aa0c431d9e786fb4d2a848fb3ab136dffd9dcc5fddcbf06cff2569e0e4d2020f90fa0c8d0503cecdf9fa895b33e90c10c3f92a3631a22d9ded83b6d0f70e7d783a4eca03b06a864bb8bdcf4dfa03b0091de96c402d926b3419b06eb52bcb320900a6078a0a3b9eaa76caa76d0f5109d0070943ba511ca5de6e90c81e3e7d9424e139c9ea2a0bd201ba74d14629643fcff259e13d1ed3b97931e2281de7d9645a5aa0f447ee6a08bd9b8440aae70c9e5e4ea823b9032821c83ea279850eca09c9b56ecdd45c8bda0c51456b3501d56795eecbbdc02f3daf3df4c1c14fcb707c24436295d36918de2a0ef9bfcf2238f8e727b4d0b011c32f0d8a4487f89acc72e7463578d13293c50ada0e176430d0660a4721470d1d20f00874c4181dcf52437bd9acca6f30c562a2d0ca0187bc38a520124d273c9349f2eb769825705f25f05d291725af21d1c238538b7a09d12823107a14133fd0b9ac1c69a31df0c02e8c1e500a678d9da2c0a4f5bfe77a033a1d714ac6c19648e43b4d9cca44045f25d5efa98cdc2b0f8a488f66d1218e9a046af5a1b7116c8d8f29eb9ed7bb3a4cf8405da10d1336e8a999c6b3fe410c02c80f901ae20b901aaf901a701835dd373b9010000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000010010000000000000000000000000000000000000000000000000000000008040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000002000000000000000100000000000000000000000000080000000000000000000000000000000000000000000000402000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f89df89b94dac17f958d2ee523a2206206994597c13d831ec7f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000be40f004e5581cae4e8acc2193e5522106aca308a0000000000000000000000000288bf776b5f1d6d659c9bda07f4c4c78192600a2a00000000000000000000000000000000000000000000000000000000011d4f8d0".to_string()
}
mod tests {
    use hex;
    use std::fs;
    use super::*;
    use crate::state::State;
    use crate::errors::AppError;
    use crate::validate_tx_hash::validate_tx_hash;
    use crate::utils::{
        dot_env_file_exists,
        get_not_in_state_err,
    };

    #[test]
    fn should_get_expected_block_correctly() {
        let result = get_expected_block();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_expected_log_correctly() {
        let result = get_expected_log();
        assert_log_is_correct(result);
    }

    #[test]
    fn should_get_expected_receipt_correctly() {
        let result = get_expected_receipt();
        assert_receipt_is_correct(result);
    }

    #[test]
    fn should_get_valid_tx_hash_as_hex() {
        let result = get_valid_tx_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex tx hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_block_hash_as_hex() {
        let result = get_valid_block_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex block hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_tx_hash_as_h256() {
        let result = get_valid_tx_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_tx_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_block_hash_as_h256() {
        let result = get_valid_block_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_block_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_intial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_initial_state()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        match State::get_endpoint_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("endpoint")),
            _ => panic!("Intial state should not have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }

    #[test]
    fn should_get_valid_state_with_endpoint_correctly() {
        let expected_verbosity = true;
        let expected_endpoint = WORKING_ENDPOINT;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_state_with_endpoint()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        match State::get_endpoint_from_state(&result) {
            Ok(endpoint) => assert!(endpoint == expected_endpoint),
            _ => panic!("Intial w/ endpoint should have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }

    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            assert!(file == original_file);
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_read_existing_env_file_correctly() {
        if dot_env_file_exists() {
            let file = read_env_file().unwrap();
            assert!(file.contains("ENDPOINT"))
        }
    }


    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            restore_env_file(original_file.clone()).unwrap();
            let file = read_env_file().unwrap();
            assert!(file == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == file);
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }
}
