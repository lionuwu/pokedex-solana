# pokedex-solana
Este es un programa on-chain desarrollado con Anchor Framework que permite a los usuarios crear su propia Pokédex personal en la blockchain de Solana. Cada entrenador puede registrar sus capturas, ver su equipo y gestionar su progreso de manera descentralizada.



🚀 ¿Qué hace este programa?
El contrato inteligente (Smart Contract) permite gestionar una base de datos persistente de Pokémon vinculada a la billetera (Wallet) del usuario mediante PDAs (Program Derived Addresses).

Funcionalidades principales:

Iniciar Pokédex: Crea una cuenta única para el entrenador utilizando su Pubkey como semilla.

Registrar Pokémon: Guarda un nuevo Pokémon con datos específicos: nombre, número de Pokédex, tipo, nivel y estado de captura.

Modificar Estado: Permite alternar el estado de un Pokémon (ej. marcarlo como "atrapado" o "visto").

Liberar Pokémon: Elimina un registro de la lista (ideal para cuando decides dejar ir a ese Rattata de nivel 2).

Consultar Datos: Envía mensajes de registro (logs) con la lista completa de Pokémon en formato legible.



🛠️ Estructura de Datos
El programa utiliza dos estructuras principales para optimizar el espacio en la cuenta:

1. Pokedex (Cuenta de Usuario)
Es la cuenta principal que vive en la blockchain.

owner: La llave pública del entrenador.

nombre_entrenador: Nombre personalizado (máx. 60 caracteres).

pokemones: Un vector (lista) que almacena hasta 20 criaturas.

2. Pokemon (Struct Interno)
Los atributos que definen a cada criatura:

nombre (String)

numero_pokedex (u16)

tipo (String)

nivel (u8)

atrapado (Boolean)
