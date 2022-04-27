# $NTV Token NEP-141 Token Contract

## ¿Qué es el token $NTV?

* $NTV es el token de gobernanza de Nativo NFT el cual permitirá a sus tenedores el tomar decisiones sobre cuál tendría que ser el futuro de este mercado de NFT’s.

## Nativo DAO - Siendo parte de la gobernanza
Astro DAO mantendrá el Nativo DAO que permitirá a en una segunda fase a los tenedores de tokens $NTV tener peso de votación. Esto les permitirá:
* Votar en encuestas de la comunidad.
* Aprobar dispersión de fondos de la tesorería.
* Aprobar actualizaciones del contrato inteligente de Nativo NFT.

## Especificaciones
* Suministro máximo: 100,000,000 de $NTV.
* Token deflacionario
* Nombre del token: Nativo token
* Ticker: $NTV
* Estándar del token: NEP-141
* Tiempo de bloqueo: Los tokens no podrán ser transferidos dentro de los primeros 3 meses después de su lanzamiento.

# Comandos para utilizar el $NTV Token

ID=nativo_token.testnet
echo $ID

Initializes the contract with the given total supply owned by the given `owner_id`.
near call $ID new '{"owner_id": "nativo_token.testnet"}' --accountId $ID

Obtener propietario del contrato
near view $ID get_owner_id

Cambiar propietario del contrato
near call $ID set_owner_id '{"owner_id": "joehank.testnet"}' --accountId $ID
near call $ID set_owner_id '{"owner_id": "dokxo.testnet"}' --accountId $ID

Obtener lista de mineros
near view $ID get_minters

Agregar minero
near call $ID add_minter '{"account_id": "joehank.testnet"}' --accountId $ID --deposit 0.000000000000000000000001

Remover minero
near call $ID remove_minter '{"account_id": "joehank.testnet"}' --accountId $ID --deposit 0.000000000000000000000001

Minar
1.5 Token
near call $ID mint '{"account_id": "joehank.testnet", "amount" : "1500000000000000000000000"}' --accountId joehank.testnet --deposit 0.000000000000000000000001
100 Token
near call $ID mint '{"account_id": "dokxo.testnet", "amount" : "100000000000000000000000000"}' --accountId dokxo.testnet --deposit 0.000000000000000000000001

Obtener valance total
near view $ID ft_total_supply

Obtener balance de una cuenta
near view $ID ft_balance_of '{"account_id": "joehank.testnet"}'
near view $ID ft_balance_of '{"account_id": "dokxo.testnet"}'

Mostrar tokens en Wallet
near call $ID ft_transfer '{"receiver_id": "joehank.testnet", "amount":"0", "memo":""}' --accountId joehank.testnet --deposit 0.000000000000000000000001

Minar tokens y agregarlos al wallet
100 tokens
near call $ID reward_player '{"player_owner_id": "dokxo.testnet", "tokens_mint" : "100000000000000000000000000"}' --accountId $ID --deposit 0.000000000000000000000001

32.58
near call $ID reward_player '{"player_owner_id": "joehank.testnet", "tokens_mint" : "32580000000000000000000000"}' --accountId $ID --deposit 0.000000000000000000000001