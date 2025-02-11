# Binance Candlestick Chart CLI

Este es un programa en Rust que se conecta a la API WebSocket de Binance para recibir datos de velas (candlestick) en tiempo real y mostrarlos en la terminal mediante `cli_candlestick_chart`.

## Requisitos

- Rust instalado en el sistema
- Dependencias especificadas en `Cargo.toml`

> [!IMPORTANT]  
> Si estás en los EE.UU., tendrás que cambiar la URL de Binance de binance.com a binance.us para que funcione correctamente.

## Uso

Ejecuta el programa con el símbolo de la criptomoneda como argumento:

```sh
cargo run -- btcusdt
```

En el caso que tenga el binario compilado puede usar

```
./charts btcusdt
```

Reemplaza `btcusdt` con el par de criptomonedas que deseas visualizar.

## Funcionalidad

- Se conecta al WebSocket de Binance para recibir datos en tiempo real.
- Muestra un gráfico de velas en la terminal.
- Refresca la vista con cada nueva vela recibida.

## Enlaces útiles

- [Documentación de la API de Binance](https://developers.binance.com/docs/derivatives/usds-margined-futures/websocket-market-streams/Kline-Candlestick-Streams)
- [Rust](https://www.rust-lang.org/)