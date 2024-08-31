# `rust-ls`

`rust-ls` es una herramienta de línea de comandos escrita en Rust que imita la funcionalidad básica del comando `ls` en sistemas Unix. Esta herramienta permite listar archivos y directorios de manera ordenada y personalizada. 

## Características

- **Listado de Archivos y Directorios**: Muestra los archivos y directorios en las rutas especificadas.
- **Soporte para Múltiples Rutas**: Puede aceptar varias rutas como argumentos y listar el contenido de cada una.
- **Archivos Ocultos**: La bandera `-a` permite mostrar archivos y directorios ocultos.
- **Ordenación de Archivos**: La bandera `--sort` permite ordenar los archivos por nombre en orden alfabético.

## Uso

### Instalación

Para compilar e instalar `rust-ls`, sigue estos pasos:

1. **Clona el Repositorio**:

    ```bash
    git clone https://github.com/tu_usuario/rust-ls.git
    cd rust-ls
    ```

2. **Compila el Proyecto**:

    ```bash
    cargo build --release
    ```

   El binario compilado se encontrará en `target/release/rust-ls`.

### Ejemplos de Uso

- **Listar Archivos y Directorios en el Directorio Actual**:

    ```bash
    ./rust-ls
    ```

- **Listar Archivos y Directorios en Rutas Específicas**:

    ```bash
    ./rust-ls /ruta/al/directorio1 /ruta/al/directorio2
    ```

- **Mostrar Archivos Ocultos**:

    ```bash
    ./rust-ls -a /ruta/al/directorio
    ```

- **Ordenar Archivos por Nombre**:

    ```bash
    ./rust-ls --sort /ruta/al/directorio
    ```

- **Combinar Opciones**:

    ```bash
    ./rust-ls -a --sort /ruta/al/directorio
    ```

## Bandera `-a`

La bandera `-a` permite que `rust-ls` muestre archivos y directorios ocultos. En sistemas Unix, los archivos ocultos son aquellos cuyos nombres comienzan con un punto (`.`). En Windows, se consideran ocultos los archivos con el atributo `FILE_ATTRIBUTE_HIDDEN`.

## Bandera `--sort`

La bandera `--sort` permite ordenar los archivos y directorios por su nombre en orden alfabético. Si esta bandera no se proporciona, los archivos se mostrarán en el orden en que se leen del directorio.

## Contribuciones

Las contribuciones son bienvenidas. Si deseas colaborar, por favor abre un `issue` o envía un `pull request` con tus mejoras.

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para obtener más detalles.

## Contacto

Para más información, puedes contactarme a través de [edwarsthat@gmail.com](edwarsthat@gmail.com).
