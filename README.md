# Rust Hero Game

Este proyecto utiliza Rust para acceder a la API de [Hero Game](https://github.com/maxwellnewage/udemy-django-hero-game) desarrollada en Django y DRF.

## Dependencias utilizadas

- reqwest (v0.11.18)
- tokio (v1)
- serde_json (v1.0.96)
- serde (v1.0)

## Instalación

Asegurarse de tener cargo instalado:

> cargo --version

Instalar dependencias del proyecto:

> cargo build

## Estructura

En [api.rs](src/api.rs) se encuentra el código que llama a la API de Hero Game, previamente montada en un entorno local bajo el puerto 8001. Esto se puede cambiar modificando la constante BASE_URL:

> const BASE_URL: &str = "http://127.0.0.1:8001/api/";

La función _make_api_request_ se encarga de llamar endpoints GET, como es el caso de _get_all_players_.

Las respuestas con serializadas por serde_json utilizando las structs que se encuentran en [models.rs](src/models.rs). Este es un ejemplo con un vector de jugadores:

> serde_json::from_value::<Vec<Player>>(resp)

Las llamadas a la API son asincrónicas, por lo cual tanto _make_api_request_ como _get_all_players_ utilizan async/await. En el caso del main, se implementa el trait de tokio:

> #[tokio::main]

Para que pueda ser asincrónico y a la vez manejar la respuesta.
