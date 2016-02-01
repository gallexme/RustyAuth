@0xdbb9ad1f14bf0b36;  # unique file ID, generated by `capnp id`


struct TPacketGCPhase {     
	header @0 :UInt8 = 253;
	phase  @1 :Phase;
	enum Phase {
		close @0;
		handshake @1;
		login @2;
		select @3;
		loading @4;
		game @5;
		dead @6;
		dbclientConnecting @7;
		dbclient @8;
		p2p @9;
		auth @10;

	}
}

