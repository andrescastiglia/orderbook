# Trading engine. Evaluación técnica.

## Backend Rust

### Introduccion

Un *trading engine* se encarga de procesar las órdenes que los usuarios van creando en un exchange. El trading-engine tiene un componente principal llamado orderbook. Un orderbook se encarga de almacenar las órdenes hasta que matchean o sean canceladas por el usuario

Un orderbook almacena las órdenes en dos grandes grupos, órdenes de venta y órdenes de compra. La órdenes de venta se acceden siempre desde la más barata a la más cara y la ordenes de compra de la más cara a la más barata.
La órdenes pueden ser de compra o de venta
Cada vez que una orden nueva ingresa se evalúa contra el lado opuesto del orderbook si tiene posibilidad de hacer match

### Aclaracion
Una orden de compra matchea con todas las órdenes de venta con precio igual o menor. Una orden de venta matchea con todas las órdenes de compra con precio igual o mayor.
Si hay match, se retira la orden que matcheo con la nueva orden del orderbook
Si no hay match la orden debe ser almacenada en su lado correspondiente 

Cada match es llamado trade y normalmente se almacena la información de las órdenes involucradas en una estructura llamada trades.

### Requerimientos

- Implementar en Rust un trading engine
- El algoritmo tiene que procesar las órdenes desde el archivo `orders.json`, que se adjunto en el mail
- Como resultado de la ejecución el algoritmo de matching tiene que generar 2 archivos, uno llamado `orderbook.json` y otro `trades.json`
- Tiene que haber por lo menos un test que evalúe un trade simple
- Se valora cualquier plus que puedas ofrecer y que ponga de manifiesto tus conocimientos y habilidades técnicas.
- La solución se tiene que compartir a través de _Github_ o ~Bitbucket~.

### Información adicional
https://www.investopedia.com/terms/o/order-book.asp
