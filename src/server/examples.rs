pub const SETTLE_BODY: &str = "{\n\"signature\": \"\",\n\"tx\":\"\"\n}";

pub const STANDARD_TX_BODY : &str = "{\"wallet_type\":\"eternl\",\"used_addresses\":[\"00167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d4246458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"0039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"0042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"60167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e81\",\"60507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f2\",\"601a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e50\",\"60d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d42464\",\"60fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e8\",\"60e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e\",\"60210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f\",\"609bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b79\",\"60742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f5\",\"60683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac26945582\",\"603202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e\",\"60772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd5\",\"602a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed0\",\"60c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e\",\"6039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb0654917\",\"60893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c63\",\"6042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb5\"],\"unused_addresses\":[\"0011d461e926a7b78335b9c4034105a29609651effa4fea1afc1abc55058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00e5b1f1f8be0ce783c4794b6567e21d690bfbc8ea6c297a8603b3ad0858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"003c7591060f3dda4462f6f5298d69a40f0c69ac46a3211dfc2006983858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"005935b6f3d40ad7945e527d4ead87bf4e0e9abaed1690e13ec055276f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00072ed3eabb7f443482f199b2d4f2479982e5afe082a39008667a673b58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"0045b87ca04905fb96afa949d5253221fb665f63be35d13718db49354958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00c9529b1a3341d175008b9accf1b331d8d2a7979a05065b8a082fb8fe58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00d29624d7f27c5e46ab3f92017b17ca1fdcb688090082a25c31f15a4058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"002d7911e68dfbe2210c9b4072d55e582088b0d9e8ca224ad51b4e470a58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00dba492b4669def7588a05cd68c84bbc4b12fcd461f93a642c31bc60458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00f5a8006e9857e3793446ef23f34c0682de583991f052a17c6cf67b1b58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"006c10db2ecb99a43027d227f5151a759584fc14c3bd5301180047f67e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"007010eb3bb98e2a3ffe36869181980ff89b81e65d4a02c61197d2051258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00eb2a2e76ab56c63f3ba0a3e7d695e7d2dfeaf49384903e8d2730558758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00d6b59b8392ea7656b4debf5d506c94d32370b71d74228c808286371e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00cfd73f62ed11267c87ba3422cc450259b377f96de3c51fa93616f2c058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00e65f8e7a8296c581382166e0c7831ca7b4d93f9dddbf6cd8e4aa8bb158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"008fa3d4545de2521a9192e1663d65d1086f230e9d9f78c2b67b6da55d58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"0071d9f45b08966235ea48c77aacd6b1fcae80a8038ba21587f1f1c76e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"00f062c80a8e3a4a6a487a0aecfc0f60722addd67d2d9bb6c15cb5497058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\"],\"stake_address\":[\"e058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\"],\"change_address\":\"00f71fe6f6718cbdfbf86c96885c788823379e7e9383ac4a7d138d870158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934\",\"utxos\":[\"8282582005e27d72124d535a80439710c77cb55cf406962b0a92c6c928f9f635b0fa46d700825839001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200a0be2f66def8f29e6f9dba71e3165c3b6b3b00dd995b4a0f3b327c2a3262b9b008258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200a4784e70f6d6c05527926f2ca5a0b698429cfaf445889d01e5c4af344c65de50082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200ca67284834d0714809151fc8b8e8c7747e119786e5a734949f46391b024122400825839001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a001226b8a1581c868f75968b696aa86be159ea63f31221df4cd4b7a48159fb632968b3a24774656e4e465433014774656e4e46543501\",\"828258200d3693f2cdc02e9b35ae15c6131ab5b994bd47a095a1897121fce0d37684e23e0082583900772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200dd6f208795c34776fb8ec3bab7200f5685c894cc8d3f7af1e0858095e97f2f000825839003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200f5cba1f6634346d6f5af39ba8c115ce6fadd7323888106f7bfd6431005bf78f00825839003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258200f743681dfdb0015a9b7e200f9db08a852020ec5a105f5eb5966d05e54c881f8008258390039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820194a5330e3109763820ef0ef743177846660ccf58ea5de5d602bb8b60c76a4c40082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a0014851ea1581c868f75968b696aa86be159ea63f31221df4cd4b7a48159fb632968b3a14774656e4e46543101\",\"82825820197682d57314c26c5d39c8df022d3e220bbdf3a45e0bf7f95ffac6d99c00cb8b00825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258201a026d241e601f3181eef346c0aa037988d32a8272309bc643bf616c452abc9800825839002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258201b14f36df5f17fdb543246d2ad019431473e6fdd39be1599b52f68605d0fd0c40082583900e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258201b7e5e25701e56bd29804f2c7297f128ec15c3c086bc08fe8d7a044fe4670b9100825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258201c591b815b5862b89a24c576face3f201d2d537cf40e748de05942d1e164679a008258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258201cc79a5431bebaa07b31fba3fce059ccef5b5947cce24270d82d16d9b19cae690082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a00989680\",\"828258202495b5c3853609d54cf59d4676ad932e966519d1f8a95916c667a459fa02398f02825839003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a23446f4c\",\"828258203182c2a0a4d98cf4fe8e491cbf9068e43b100842eec7cbe3319b9f4b16e8fa820082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820319c27fe057a165741473771587e6acf0799d027cd1ae965241e07fa8d17dc6d0082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a000f422a\",\"82825820338ce96bf1ca1493c8b1611d8a8448f35e536bc0a4529e5a700a1793b62955a80082583900742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258203e0223880363b174fcf85887b7e20cf1e2c6cf6851bbcb1fbc77fb18e5f9d09a0082583900210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a001215e2a1581c75b3d8e650c8f7fdb83c1816d38c23aafd0b513ce50c5e6eeccc99e9a14f4c6f616457617272696f7230342d3501\",\"8282582044b6f488071f1e105f709a2f0df98b465331b73e8374be56f33af703589ccc0f0082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582045c6b0ddcf6b58d91ae99671fc8efca89abbb60826871db3eb33c97f9684bff80082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a004949b3a1581cd25b21f3694dfe8b614bad38b149281735a59a04c76353bc72fbbfbfa14474464c5a19096d\",\"8282582046f701e8bf5d699a9aed7aa1b6a41d3e4777277189aebdd05ecb7d6d490ef4cf008258390039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582046f8f9a4a5e39df190d9083bdf6f3e4383cd49fb60ed6ecc546f01eed7b2078c0082583900210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820493a91ee7bb676434215a17355e51f9b3aebd8b94dc58d957e1ac632f2c36b510082583900772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258204ba5508daf0df2c4b2df543abe6c57c2e0cc8daabb9d7b70ef3893f75c5a9ade0082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258204dde9003d57183d23ff19cfd4275dc8136e695703c9d74d9015d8d634bc6a02d0082583900210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582054f16809c7c94af95af7550c5d720d0342474b2da1a73a753f690727f633be8c0082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820591d2920eaafc6c2a49440b155cb43e367063acfc8824904fc0ce2abd95f520200825839002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258206302edf59e9a56e32c83078c72954038b0ca31a994bdf46bac9a236645090514008258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582064c5228857a32aea68fc1ba7867e74313b398ca62764307a502179e627c3dc99018258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a468c5559\",\"8282582064c5228857a32aea68fc1ba7867e74313b398ca62764307a502179e627c3dc99028258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a234793d2\",\"8282582064c5228857a32aea68fc1ba7867e74313b398ca62764307a502179e627c3dc99038258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a234793d1\",\"82825820677fdca9cd9601693b0168498b17f1f733c0f53c3e5325923bdd8bedcc0a57ab008258390039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582069e58d6f31b769268f5c4d111b8a60a1a34884f8d5479f5ce0e9f43e035179f60082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258206be7d565e9844c813f71e76fa0daa99be67d5d558eeb7d6cb5483f7cb843b1310082583900d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d4246458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258206c4daafc5b5bea7492306f8a20f07340360c02d89b4fdb7d918da1af13a285bf008258390042dce29f524571ab82acc73c181010deb46caf057bcde5ac87c2ebb558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258206e9f2e13208798cc686c062af8ad34ee854c6a08b49290bfbf5900bfed3b154c00825839002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a0014851ea1581c868f75968b696aa86be159ea63f31221df4cd4b7a48159fb632968b3a14774656e4e46543201\",\"828258206e9f2e13208798cc686c062af8ad34ee854c6a08b49290bfbf5900bfed3b154c018258390039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a46834532\",\"828258206ef58e3560d3e490eef8da9e32bc0e2ae6cf7f8f213e3ae3d97ebe795be4989d0082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820720156c42d31d1a56a7f97c8bfb2582f9c43d54b8a102f8fa72114c6630cc7ba00825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258207368961cd24e534df7bf54bf85b8c93bea75d528ea3cf9aa0db451c70f44bb530082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820794f8994ee76807135be7971b98ad0e5301afbe6cd17a62b067fc12faa2d715f0082583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258207d32b686b1b6a493f2f4660b1624d8af79342b08f94d5789502c4c96544b64180082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258207d66e974eb810de3dcca6ef580bd8dc3741e5ff535b49705853129094faf5ea30082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a000f422a\",\"828258207e0fd50e46b874e9754d520c4097b93ec9af7e152b19b032567ee30bd555f4970082583900e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258207f7daa628f573cedcd7b36c98b1298407123e4a20da5313b8de925f6763dc8200082583900683d2c61a629ebd0fb536660ed5cd567e0fa26e41664b9ac2694558258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258208034fbad2044370921e3748d6d372eea3d9e5dda104a1f582bd52cc3389cd4bf0082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a0019c812a1581c55e9a9737f5ee3f3a04a80b5bc9419c87724187318bd7ac376141e10a54774656e4e465431014774656e4e465432014774656e4e465433014774656e4e465434014774656e4e46543501\",\"8282582083b9534fec81d2fc9d5c6002706e12d33cd03a95221647afea3d3f519a7e2f060082583900c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258208ae4fa04190022b870725f075eadd01c55ffa3a4d67f21356ae0310ded03f81f0082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258208babc379e99195449a55443d72e1bdff879f388eb5de6e6bccc6cdb118a191a300825839003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258208d750a532b3e4f0ded65170f737cddef4da449e2f793c01bca86c77adc8f794e0082583900742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258208d7d59e3e5705a5b4c5cf0e3dc50ffb2cf30c7c2ce30fb2e232459bf5c755d0a0082583900e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582092b4c6c8d6962a1ba215f21528f3b39409d5deed994a9e436d33a8dded9edd3b0082583900742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820940e7bdb43b3e2242918e79afe478ba7d45a6467cf8388d90de36b17f8834a2b0082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"828258209907fcf5847e61f3724aa2c3165b42eb37912620bd6e3da6f119fae4f3418e710082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"8282582099c66e979975ec4ae94487bcfca40372738d0879b79e2d6a9beba41deabff0e50082583900e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a0011d28aa1581c0e11a695e4fd6e28dbe13609e59989c2e3fd73b8d17dcd6638ded4dca14b64697363324e465431303001\",\"82825820a3422ffbd9cf30ec6f733da807336cf6452c331f247d714799261d47192119f60082583900772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820a88521ee676f876cd99f7ebab1b3bb485bf95d269844e0aaa54201e8dec94f460082583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820a930b3692e2f2bd2b09116505f7b736c412ad9b0a2ffe781f8fdf6c7e73a3b170182583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a845375ad\",\"82825820a930b3692e2f2bd2b09116505f7b736c412ad9b0a2ffe781f8fdf6c7e73a3b170282583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a422b23fb\",\"82825820a930b3692e2f2bd2b09116505f7b736c412ad9b0a2ffe781f8fdf6c7e73a3b170382583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a422b23fb\",\"82825820a9749a9d51654fe2682771e2cc47e95d8c104ea1a879935cd3d2c50b05ca2adb0082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820aa509c893698b8f304e34d5a20c21dbbb414dce840182236aa8772c6772a53d100825839002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820ad0c17b595fa6af24f5968ba6acb727e37d7a74aab50844a76e0675eef3efe0600825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820aeaab7142e18485b35b473122499913eafa3613b353cdb5a013f1875ce3884d50082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820b03faef58ddc7eb05e368dd1290fda5d0d3bbeace7621cd39d838ccdf8dfe03f0082583900c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820b1d7a1f46e22b4fb90f9f3790ddb84b2b17a3ebdcb87bfcd58ef830ffb9c189e0082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a232e79b0a4581c3f1d9bd2f8c3d7d8144b789433261370eaf25cdc83fe8a745ef880c1a146744452415341190af0581cc693a41d2b4f241c992b88c7238131d92202206ffc92f5eae090d0eea1457454657374192b39581cd25b21f3694dfe8b614bad38b149281735a59a04c76353bc72fbbfbfa14474464c5a190122581cdfd18a815a25339777dcc80bce9c438ad632272d95f334a111711ac9a1447441726b1901cd\",\"82825820b44f51a043903a9e6c6b4ac119b16a555fc0c05114e24e75b6f7a97fe30a5bf20082583900c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820b5914e370c8cc215c8a55b67f686a850295465c06601819518cc3445b1c7b3f500825839003202260b544e5e47811a3de58c7786bfcbaff8835821f5b5b074e95e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820b5e4f2ad6e0a9a602a77518415647944b9fc2a7bbd07d77a46797e2f8360068604825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a004c4b40\",\"82825820b8c961575efae37c1eadeee54a5f8f14f62c673a5be1ac2ba54461aa352fa2d7008258390039c12fca1943e6610c6540de28cf84f45f64b55fbc2bda5cb065491758c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820bbe7b119def7d46e891262e0c8b118414747a8951a6d2971dae5053d2177e98400825839002a4713eaaf225793c9eee369c5a22f78a7ed124ab420657044823ed058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820c22795d4c0a4c423451cc9d669640daab6d9b5e0aaafaa54142a343dd66e59330082583900210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820c25bb18afe488cff815644a91e283e870c2d12212b484a8b8038a1db237f12d80082583900210186f0ce81b096efb439462770314ab4fd5fc6877611500b68e19f58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820c29470125cc1663361a1867494a1c9f39a4ae267b1aa55f7ff87ef0468aeee4c0082583900507cdd889144619a85cafca24a6a0bd4674371f9cb2748fc9625a8f258c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820ce6ba788dedb591d190a41af985bb313c57f043d0885fe1941ce1a33125064520082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a0036f957a1581cc693a41d2b4f241c992b88c7238131d92202206ffc92f5eae090d0eea14574546573741935ef\",\"82825820d1340a824fea8e3f30146c35e96cb38d18b9f8bbe234ca5adbd4dbee9329d54c0082583900c55a3c745800e4e1a5605acfabccb87cf617cfed7f07bd9d5f61e09e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820d3d37ca9cfae02e39961748f128a81bc08bde819133800d6b025cb89b14687ad0082583900772e10b3cd63f89481e7d88d9d443c78182cebddfe368f2d937febd558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820dd3e1186ebd365226daa691b106874a3ce565b0bf0a18671b3e6620b71b4f8e70082583900742e1c11f15a069d375cfead648168ae5927c21a52fdf340094772f558c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820e037166b40a9fbcf51003ddee74c7f71d25f0e30e55992a12bc792692f2585e100825839001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820e178ea9c00d9009476b321e6de55aca918c2130c965475471cd45c937f64fff40082583900e18e0e41515503315586432f22e0a91b6b66c2e8a91139307627dd7e58c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820e1ef7f173c1281a76b6fb4698240cc298670656a522907a0f55fe1468c2c9ac30082583900167d64052f9408816b9684b964befffb0b2c49132fb6e30097c12e8158c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00402093a1581c3f1d9bd2f8c3d7d8144b789433261370eaf25cdc83fe8a745ef880c1a146744452415341190dac\",\"82825820e4e03a601d70a57aaf87f0b2222ab388783238c10a93516533f4814fd1ec14110082583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820e8d6059bd8627cd69f31445bc03d0c5221d9130fed7e47cea674e61c29302c8a0082583900d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d4246458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820ea6ebb93e208716cb3cb0cac47066f2a47a4e472274483fdc18901228596754900825839001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820f0d8fe5ab94aacd7901cdbca325b5003d4767bef5093441abbcdfe3e1f345c1b0082583900d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d4246458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820f67ac37cf0a91615fc2d8415835921650a9217c36df62e1321d1271519d2194500825839001a18e064a330a4265f9616de6a275b2d8024d57a65c9dd65b8f63e5058c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820fc93c4536809a1ff8a63e29578cc2ff6f9d9e5978ffd6153ad4950d48ed57e630082583900893de54671444dba681fe60cb5e717ed3a6a8b7157c358c5462a3c6358c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\",\"82825820ff79939f8235e2b176b5673fb0ecbdbf3299870f80ac075248ceff8bdeeabc040082583900d5b73cefe484fe4f61997e6a2569e65d8d5d641e991ebe0ce7d4246458c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce251934821a00118f32a1581cd35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070a1456d544f534901\"],\"collateral\":[\"82825820b5e4f2ad6e0a9a602a77518415647944b9fc2a7bbd07d77a46797e2f8360068604825839009bd6e44028349755604b3922ebb38dbeebb7d9de582945c47b217b7958c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a004c4b40\",\"828258201cc79a5431bebaa07b31fba3fce059ccef5b5947cce24270d82d16d9b19cae690082583900fcb05fcb822bfe2e32669704bbecaeabbd5a72938afe20823f0481e858c8e2bf54937b76730263f0d6ebd8181861b0ddd84bf7fdce2519341a00989680\"],\"excludes\":[],\"network\":0,\"operation\": {
    \"config\": {
      \"validator_address\":\"ayavalcons1j4ynx8yhxfnzww4p8tkwf25w04ll70hc3fnly6\",
      \"operator_address\":\"aya1ax7ttvj2s5lxc22a9630p4c95yx24rqfwu0etn\",
      \"moniker\":\"MyValidator\"
    },
    \"ennft_assetname\":\"asset1r8veaawla724n7yt5hsfkqzyhlxz652q5ka8nw\"
  }}";