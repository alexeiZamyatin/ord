use {
  super::*,
  ord::subcommand::wallet::{inscriptions::Output, receive},
};

#[test]
fn inscriptions() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  create_wallet(&rpc_server);
  rpc_server.mine_blocks(1);

  let Inscribe {
    reveal,
    inscription,
    ..
  } = inscribe(&rpc_server);

  let output = CommandBuilder::new("wallet inscriptions")
    .rpc_server(&rpc_server)
    .output::<Vec<Output>>();

  assert_eq!(output.len(), 1);
  assert_eq!(output[0].inscription, inscription.parse().unwrap());
  assert_eq!(output[0].location, format!("{reveal}:0:0").parse().unwrap());
  assert_eq!(
    output[0].explorer,
    format!("https://ordinals.com/inscription/{inscription}")
  );

  let address = CommandBuilder::new("wallet receive")
    .rpc_server(&rpc_server)
    .output::<receive::Output>()
    .address;

  let send = CommandBuilder::new(format!("wallet send {address} {inscription}"))
    .rpc_server(&rpc_server)
    .output::<ord::subcommand::wallet::send::Output>();

  rpc_server.mine_blocks(1);

  let inscriptions = CommandBuilder::new("wallet inscriptions")
    .rpc_server(&rpc_server)
    .output::<Vec<Output>>();

  assert_eq!(inscriptions.len(), 1);
  assert_eq!(inscriptions[0].inscription, inscription.parse().unwrap());
  assert_eq!(
    inscriptions[0].location,
    format!("{}:0:0", send.transaction).parse().unwrap()
  );
}

#[test]
fn inscriptions_includes_locked_utxos() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  create_wallet(&rpc_server);

  rpc_server.mine_blocks(1);

  let Inscribe {
    inscription,
    reveal,
    ..
  } = inscribe(&rpc_server);

  rpc_server.mine_blocks(1);

  rpc_server.lock(OutPoint {
    txid: reveal,
    vout: 0,
  });

  let output = CommandBuilder::new("wallet inscriptions")
    .rpc_server(&rpc_server)
    .output::<Vec<Output>>();

  assert_eq!(output.len(), 1);
  assert_eq!(output[0].inscription, inscription.parse().unwrap());
  assert_eq!(output[0].location, format!("{reveal}:0:0").parse().unwrap());
}
